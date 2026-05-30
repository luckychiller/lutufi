use std::collections::HashMap;
use std::path::Path;
use crate::core::{
    domain::Domain,
    error::{LutufiError, LutufiResult},
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    variable::Variable,
};

/// Reader and writer for the UAI Competition Format.
pub struct UaiFormat;

impl UaiFormat {
    /// Exports a Bayesian network to a UAI-formatted string.
    pub fn export(network: &BayesianNetwork) -> LutufiResult<String> {
        let mut out = String::new();
        out.push_str("BAYES\n");

        let variables = network.variables();
        let node_names: Vec<&str> = network.nodes();
        let n = node_names.len();

        let domain_sizes: Vec<usize> = node_names
            .iter()
            .filter_map(|name| network.variable(name).ok().and_then(|v| v.domain().size()))
            .collect();

        out.push_str(&format!("{}\n", n));
        out.push_str(
            &domain_sizes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
        out.push('\n');

        let order =
            network.topological_order().unwrap_or_else(|_| node_names.iter().map(|s| *s).collect());
        let mut scopes: Vec<Vec<usize>> = Vec::new();
        let mut tables: Vec<Vec<f64>> = Vec::new();
        let mut name_to_idx: HashMap<String, usize> = HashMap::new();
        for (i, name) in node_names.iter().enumerate() {
            name_to_idx.insert(name.to_string(), i);
        }

        for var_name in &order {
            let cpt = match network.cpd(var_name) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let parent_ids = cpt.parent_ids();
            let parent_idxs: Vec<usize> = parent_ids
                .iter()
                .filter_map(|pid| {
                    variables
                        .get(pid)
                        .and_then(|v| name_to_idx.get(v.name()))
                        .copied()
                })
                .collect();
            let child_idx = name_to_idx.get(*var_name).copied().unwrap_or(0);

            let mut scope = parent_idxs.clone();
            scope.push(child_idx);

            let factor = cpt.as_factor();
            let n_entries = factor.scope().num_entries();
            let mut table = Vec::with_capacity(n_entries);
            for i in 0..n_entries {
                table.push(factor.value_at(i));
            }

            scopes.push(scope);
            tables.push(table);
        }

        out.push_str(&format!("{}\n", scopes.len()));
        for scope in &scopes {
            out.push_str(&format!("{} ", scope.len()));
            for idx in scope {
                out.push_str(&format!("{} ", idx));
            }
            out.push('\n');
        }

        for table in &tables {
            out.push('\n');
            for chunk in table.chunks(10) {
                let line: Vec<String> = chunk.iter().map(|v| format!("{:.10}", v)).collect();
                out.push_str(&line.join(" "));
                out.push('\n');
            }
        }

        Ok(out)
    }

    /// Exports a Bayesian network to a UAI file at the given path.
    pub fn export_to_file<P: AsRef<Path>>(network: &BayesianNetwork, path: P) -> LutufiResult<()> {
        let content = Self::export(network)?;
        std::fs::write(path.as_ref(), &content).map_err(|e| {
            LutufiError::SerializationError {
                reason: format!("Failed to write UAI file: {}", e),
            }
        })?;
        Ok(())
    }

    /// Imports a Bayesian network from a UAI-formatted string.
    pub fn import(content: &str) -> LutufiResult<BayesianNetwork> {
        let lines: Vec<&str> = content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        if lines.is_empty() {
            return Err(LutufiError::DeserializationError {
                reason: "Empty UAI file".to_string(),
            });
        }

        let format_type = lines[0].trim().to_uppercase();
        if format_type != "BAYES" && format_type != "MARKOV" {
            return Err(LutufiError::DeserializationError {
                reason: format!(
                    "Unsupported UAI format type: {}. Only BAYES and MARKOV are supported.",
                    format_type
                ),
            });
        }

        let mut idx = 1;
        if idx >= lines.len() {
            return Err(LutufiError::DeserializationError {
                reason: "UAI file truncated: expected variable count".to_string(),
            });
        }
        let n: usize = lines[idx].parse().map_err(|_| {
            LutufiError::DeserializationError {
                reason: format!("Invalid variable count: {}", lines[idx]),
            }
        })?;
        idx += 1;

        if idx >= lines.len() {
            return Err(LutufiError::DeserializationError {
                reason: "UAI file truncated: expected domain sizes".to_string(),
            });
        }
        let domain_sizes: Vec<usize> = lines[idx]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        idx += 1;

        if domain_sizes.len() != n {
            return Err(LutufiError::DeserializationError {
                reason: format!(
                    "Expected {} domain sizes, got {}",
                    n,
                    domain_sizes.len()
                ),
            });
        }

        if idx >= lines.len() {
            return Err(LutufiError::DeserializationError {
                reason: "UAI file truncated: expected function count".to_string(),
            });
        }
        let m: usize = lines[idx].parse().map_err(|_| {
            LutufiError::DeserializationError {
                reason: format!("Invalid function count: {}", lines[idx]),
            }
        })?;
        idx += 1;

        let mut scopes: Vec<Vec<usize>> = Vec::new();
        for _ in 0..m {
            if idx >= lines.len() {
                return Err(LutufiError::DeserializationError {
                    reason: "UAI file truncated: expected scope line".to_string(),
                });
            }
            let parts: Vec<usize> = lines[idx]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.is_empty() {
                continue;
            }
            let scope_size = parts[0];
            let scope: Vec<usize> = parts[1..1 + scope_size.min(parts.len() - 1)]
                .iter()
                .map(|v| *v)
                .collect();
            scopes.push(scope);
            idx += 1;
        }

        let mut tables: Vec<Vec<f64>> = Vec::new();
        let mut current_table = Vec::new();
        while idx < lines.len() && tables.len() < m {
            let line = lines[idx];
            let numbers: Vec<f64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            current_table.extend(numbers);
            if tables.len() < scopes.len() {
                let expected_size: usize = scopes[tables.len()]
                    .iter()
                    .map(|&si| domain_sizes.get(si).copied().unwrap_or(2))
                    .product();
                if expected_size > 0 && current_table.len() >= expected_size {
                    tables.push(current_table[..expected_size].to_vec());
                    current_table = current_table[expected_size..].to_vec();
                }
            }
            idx += 1;
        }

        if !current_table.is_empty() && tables.len() < m {
            tables.push(current_table);
        }

        let mut network = BayesianNetwork::new();
        for i in 0..n {
            let name = format!("X{}", i);
            let size = domain_sizes.get(i).copied().unwrap_or(2);
            if size == 2 {
                network.add_variable(&name, Domain::binary())?;
            } else {
                let states: Vec<String> = (0..size).map(|j| j.to_string()).collect();
                network.add_variable(&name, Domain::discrete(states)?)?;
            }
        }

        for (scope_idx, scope) in scopes.iter().enumerate() {
            if scope.is_empty() {
                continue;
            }
            let child_idx = scope[scope.len() - 1];
            let parent_idxs: Vec<usize> = scope[..scope.len() - 1].to_vec();

            let child_name = format!("X{}", child_idx);
            if network.id_of(&child_name).is_err() {
                continue;
            }

            for &pidx in &parent_idxs {
                let pname = format!("X{}", pidx);
                if network.id_of(&pname).is_ok() && network.id_of(&child_name).is_ok() {
                    if !network
                        .edges()
                        .iter()
                        .any(|(f, t)| *f == pname && *t == child_name)
                    {
                        let _ = network.add_edge(&pname, &child_name);
                    }
                }
            }

            let child_var = network.variable(&child_name)?.clone();
            let parent_vars: Vec<&Variable> = parent_idxs
                .iter()
                .filter_map(|&pidx| {
                    let pname = format!("X{}", pidx);
                    network.variable(&pname).ok()
                })
                .collect();

            if scope_idx < tables.len() {
                let table = &tables[scope_idx];
                if !table.is_empty() {
                    let child_domain_size = child_var.domain().size().unwrap_or(1);
                    let parent_card: usize = parent_vars
                        .iter()
                        .map(|v| v.domain().size().unwrap_or(1))
                        .product();
                    let parent_card = parent_card.max(1);

                    let mut matrix = vec![vec![0.0f64; child_domain_size]; parent_card];
                    for pc in 0..parent_card {
                        for cs in 0..child_domain_size {
                            let idx_flat = pc * child_domain_size + cs;
                            if idx_flat < table.len() {
                                matrix[pc][cs] = table[idx_flat];
                            }
                        }
                    }

                    if let Ok(cpt) =
                        ConditionalProbabilityTable::from_values(&child_var, &parent_vars, matrix)
                    {
                        let _ = network.set_cpd(&child_name, cpt);
                    }
                }
            }
        }

        Ok(network)
    }

    /// Imports a Bayesian network from a UAI file at the given path.
    pub fn import_from_file<P: AsRef<Path>>(path: P) -> LutufiResult<BayesianNetwork> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            LutufiError::DeserializationError {
                reason: format!("Failed to read UAI file: {}", e),
            }
        })?;
        Self::import(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Domain;

    fn create_simple_network() -> BayesianNetwork {
        let mut net = BayesianNetwork::new();
        net.add_variable("X0", Domain::binary());
        net.add_variable("X1", Domain::binary());
        net.add_edge("X0", "X1").unwrap();
        let x0 = net.variable("X0").unwrap().clone();
        let x1 = net.variable("X1").unwrap().clone();
        let cpt0 = ConditionalProbabilityTable::from_values(
            &x0,
            &[] as &[&Variable],
            vec![vec![0.5, 0.5]],
        )
        .unwrap();
        net.set_cpd("X0", cpt0).unwrap();
        let cpt1 = ConditionalProbabilityTable::from_values(
            &x1,
            &[&x0],
            vec![vec![0.9, 0.1], vec![0.2, 0.8]],
        )
        .unwrap();
        net.set_cpd("X1", cpt1).unwrap();
        net
    }

    #[test]
    fn test_uai_export_contains_header() {
        let net = create_simple_network();
        let uai = UaiFormat::export(&net).unwrap();
        assert!(uai.starts_with("BAYES\n"));
        assert!(uai.contains("2\n2 2"));
    }

    #[test]
    fn test_uai_roundtrip() {
        let original = create_simple_network();
        let uai = UaiFormat::export(&original).unwrap();
        let imported = UaiFormat::import(&uai).unwrap();
        assert_eq!(original.nodes().len(), imported.nodes().len());
        assert_eq!(original.edges().len(), imported.edges().len());
        for name in original.nodes() {
            if let (Ok(oc), Ok(ic)) = (original.cpd(name), imported.cpd(name)) {
                let of = oc.as_factor();
                let impf = ic.as_factor();
                let n = of.scope().num_entries();
                let mut orig_vals: Vec<f64> = (0..n).map(|i| of.value_at(i)).collect();
                let mut imp_vals: Vec<f64> = (0..n).map(|i| impf.value_at(i)).collect();
                orig_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
                imp_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
                for i in 0..n {
                    let diff = (orig_vals[i] - imp_vals[i]).abs();
                    assert!(
                        diff < 1e-6,
                        "Mismatch at {}[{}]: {} vs {}",
                        name,
                        i,
                        orig_vals[i],
                        imp_vals[i]
                    );
                }
            }
        }
    }

    #[test]
    fn test_uai_export_file() {
        let net = create_simple_network();
        let tmp = std::env::temp_dir().join("test_export.uai");
        UaiFormat::export_to_file(&net, &tmp).unwrap();
        let imported = UaiFormat::import_from_file(&tmp).unwrap();
        let _ = std::fs::remove_file(&tmp);
        assert_eq!(imported.nodes().len(), 2);
    }
}
