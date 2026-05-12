use std::path::Path;
use crate::core::{
    domain::Domain,
    error::{LutufiError, LutufiResult},
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    variable::Variable,
};

/// Reader and writer for the Bayesian Interchange Format (BIF).
pub struct BifFormat;

impl BifFormat {
    /// Exports a Bayesian network to a BIF-formatted string.
    pub fn export(network: &BayesianNetwork) -> LutufiResult<String> {
        let mut out = String::new();
        out.push_str("// BIF format exported by Lutufi\n");
        out.push_str("// https://github.com/luckychiller/lutufi\n\n");

        let name = "lutufi_model";
        out.push_str(&format!("network {} {{\n}}\n\n", name));

        let variables = network.variables();
        let mut node_names: Vec<&str> = network.nodes();
        node_names.sort();

        for var_name in &node_names {
            let var = network.variable(var_name).map_err(|_| {
                LutufiError::SerializationError {
                    reason: format!("Variable '{}' not found", var_name),
                }
            })?;
            let domain = var.domain();
            match domain {
                Domain::Binary => {
                    out.push_str(&format!(
                        "variable {} {{\n  type discrete [2] {{ \"false\", \"true\" }};\n}}\n\n",
                        var_name
                    ));
                }
                Domain::Discrete { states } => {
                    let state_list: Vec<String> = states
                        .iter()
                        .map(|s| format!("\"{}\"", s))
                        .collect();
                    out.push_str(&format!(
                        "variable {} {{\n  type discrete [{}] {{ {} }};\n}}\n\n",
                        var_name,
                        states.len(),
                        state_list.join(", ")
                    ));
                }
                Domain::Continuous { .. } => {
                    out.push_str(&format!(
                        "variable {} {{\n  type continuous;\n}}\n\n",
                        var_name
                    ));
                }
            }
        }

        let order = network.topological_order().unwrap_or_else(|_| {
            node_names.iter().map(|s| *s).collect()
        });
        for var_name in &order {
            let cpt = match network.cpd(var_name) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let parent_ids = cpt.parent_ids();
            let parent_names: Vec<String> = parent_ids
                .iter()
                .filter_map(|pid| variables.get(pid).map(|v| v.name().to_string()))
                .collect();

            let factor = cpt.as_factor();
            let scope = factor.scope();
            let n = scope.num_entries();
            let values: Vec<f64> = (0..n).map(|i| factor.value_at(i)).collect();

            if parent_names.is_empty() {
                let row: Vec<String> = values.iter().map(|v| format!("{:.10}", v)).collect();
                out.push_str(&format!(
                    "probability ( {} ) {{\n  table {};\n}}\n\n",
                    var_name,
                    row.join(", ")
                ));
            } else {
                out.push_str(&format!(
                    "probability ( {} | {} ) {{\n",
                    var_name,
                    parent_names.join(", ")
                ));
                let child_size = scope.size_of(&network.id_of(var_name).unwrap()).unwrap_or(1);
                let parent_card: usize = parent_names
                    .iter()
                    .filter_map(|pn| {
                        variables.get(&network.id_of(pn).ok()?).map(|v| {
                            v.domain().size().unwrap_or(1)
                        })
                    })
                    .product();
                let parent_card = parent_card.max(1);
                for pc in 0..parent_card {
                    let start = pc * child_size;
                    let end = std::cmp::min(start + child_size, values.len());
                    let row_values: Vec<String> = values[start..end]
                        .iter()
                        .map(|v| format!("{:.10}", v))
                        .collect();
                    out.push_str(&format!("  table {};\n", row_values.join(", ")));
                }
                out.push_str("}\n\n");
            }
        }

        Ok(out)
    }

    /// Exports a Bayesian network to a BIF file at the given path.
    pub fn export_to_file<P: AsRef<Path>>(network: &BayesianNetwork, path: P) -> LutufiResult<()> {
        let content = Self::export(network)?;
        std::fs::write(path.as_ref(), &content).map_err(|e| {
            LutufiError::SerializationError {
                reason: format!("Failed to write BIF file: {}", e),
            }
        })?;
        Ok(())
    }

    /// Imports a Bayesian network from a BIF-formatted string.
    pub fn import(content: &str) -> LutufiResult<BayesianNetwork> {
        let mut network = BayesianNetwork::new();
        let mut in_network = false;
        let mut in_variable = false;
        let mut current_var_name = String::new();
        let mut current_var_states: Vec<String> = Vec::new();
        let mut current_var_continuous = false;
        let mut current_child = String::new();
        let mut current_parents: Vec<String> = Vec::new();
        let mut current_table: Vec<f64> = Vec::new();
        let mut conditional_tables: Vec<(String, Vec<String>, Vec<f64>)> = Vec::new();
        let mut variable_info: Vec<(String, Vec<String>, bool)> = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if line.starts_with("network ") && line.ends_with('{') {
                in_network = true;
            } else if !in_network {
                continue;
            }

            if line.starts_with("variable ") {
                if let Some(name_start) = line.find(' ') {
                    let rest = &line[name_start + 1..];
                    if let Some(name_end) = rest.find('{') {
                        current_var_name = rest[..name_end].trim().to_string();
                    } else {
                        current_var_name = rest.trim_end_matches('{').trim().to_string();
                    }
                }
                in_variable = true;
                current_var_states.clear();
                current_var_continuous = false;
            } else if in_variable {
                if line.contains("type discrete") {
                    if let Some(bracket_start) = line.find('[') {
                        if let Some(bracket_end) = line[bracket_start + 1..].find(']') {
                            let after_bracket = &line[bracket_start + 1 + bracket_end..];
                            let brace_start = after_bracket.find('{');
                            if let Some(bs) = brace_start {
                                let content_inside = &after_bracket[bs + 1..];
                                let brace_end = content_inside.find('}');
                                let raw = if let Some(be) = brace_end {
                                    &content_inside[..be]
                                } else {
                                    content_inside
                                };
                                current_var_states = raw
                                    .split(',')
                                    .map(|s| {
                                        s.trim()
                                            .trim_matches('"')
                                            .trim_matches('{')
                                            .trim_matches('}')
                                            .to_string()
                                    })
                                    .filter(|s| !s.is_empty())
                                    .collect();
                            }
                        }
                    }
                } else if line.contains("type continuous") {
                    current_var_continuous = true;
                } else if line == "}" || line == "};" {
                    variable_info.push((
                        current_var_name.clone(),
                        current_var_states.clone(),
                        current_var_continuous,
                    ));
                    in_variable = false;
                }
            }

            if line.starts_with("probability ") {
                in_variable = false; // close any open variable
                current_parents.clear();
                current_table.clear();
                let paren_open = line.find('(');
                let paren_close = line.find(')');
                if let (Some(po), Some(pc)) = (paren_open, paren_close) {
                    let inside = &line[po + 1..pc];
                    if let Some(pipe) = inside.find('|') {
                        current_child = inside[..pipe].trim().to_string();
                        current_parents = inside[pipe + 1..]
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    } else {
                        current_child = inside.trim().to_string();
                    }
                }
                if let Some(semi) = line.find(';') {
                    let before_semi = &line[..semi];
                    if let Some(table_keyword) = before_semi.find("table") {
                        let numbers_str = &before_semi[table_keyword + 5..];
                        current_table = numbers_str
                            .split(',')
                            .filter_map(|s| {
                                let trimmed = s.trim().trim_matches(';');
                                if trimmed.is_empty() {
                                    None
                                } else {
                                    trimmed.parse::<f64>().ok()
                                }
                            })
                            .collect();
                    }
                }
            } else if line.starts_with('(') && !current_child.is_empty() {
                let paren_close = line.find(')');
                let numbers_part = if let Some(pc) = paren_close {
                    &line[pc + 1..]
                } else {
                    line
                };
                let nums: Vec<f64> = numbers_part
                    .split(|c: char| c == ',' || c == ';')
                    .filter_map(|s| {
                        let trimmed = s.trim();
                        if trimmed.is_empty() {
                            None
                        } else {
                            trimmed.parse::<f64>().ok()
                        }
                    })
                    .collect();
                current_table.extend(nums);
            } else if line.starts_with("table") && !current_child.is_empty() {
                let numbers_str = &line[5..];
                let nums: Vec<f64> = numbers_str
                    .split(',')
                    .filter_map(|s| {
                        let trimmed = s.trim().trim_matches(';');
                        if trimmed.is_empty() {
                            None
                        } else {
                            trimmed.parse::<f64>().ok()
                        }
                    })
                    .collect();
                current_table.extend(nums);
            } else if line == "}" || line == "};" {
                if !current_child.is_empty() && !current_table.is_empty() {
                    conditional_tables.push((
                        current_child.clone(),
                        current_parents.clone(),
                        current_table.clone(),
                    ));
                }
                current_child.clear();
                current_table.clear();
                current_parents.clear();
            }
        }

        for (var_name, states, is_cont) in &variable_info {
            if *is_cont {
                network.add_variable(var_name, Domain::continuous(None, None))?;
            } else if states.len() == 2
                && states.iter().any(|s| s == "false" || s == "0")
                && states.iter().any(|s| s == "true" || s == "1")
            {
                network.add_variable(var_name, Domain::binary())?;
            } else {
                network.add_variable(var_name, Domain::discrete(states.clone())?)?;
            }
        }

        for (child, parents, table) in &conditional_tables {
            for parent in parents {
                if network.id_of(parent).is_ok() && network.id_of(child).is_ok() {
                    if !network
                        .edges()
                        .iter()
                        .any(|(f, t)| f == parent && t == child)
                    {
                        let _ = network.add_edge(parent, child);
                    }
                }
            }

            if network.id_of(child).is_err() {
                continue;
            }
            let child_var = network.variable(child)?.clone();
            let parent_vars: Vec<&Variable> = parents
                .iter()
                .filter_map(|pn| network.variable(pn).ok())
                .collect();
            if table.is_empty() {
                continue;
            }

            let child_domain_size = child_var.domain().size().unwrap_or(1);
            let parent_card: usize = parent_vars
                .iter()
                .map(|v| v.domain().size().unwrap_or(1))
                .product();
            let parent_card = parent_card.max(1);

            let mut matrix = vec![vec![0.0f64; parent_card]; child_domain_size];
            for pc in 0..parent_card {
                for cs in 0..child_domain_size {
                    let idx = pc * child_domain_size + cs;
                    if idx < table.len() {
                        matrix[cs][pc] = table[idx];
                    }
                }
            }
            eprintln!("DEBUG BIF matrix for {}: {:?}, parent_card={}, child_domain_size={}, parent_vars={:?}",
                child, matrix, parent_card, child_domain_size,
                parent_vars.iter().map(|v| v.name()).collect::<Vec<_>>());

            match ConditionalProbabilityTable::from_values(&child_var, &parent_vars, matrix) {
                Ok(cpt) => {
                    if let Err(e) = network.set_cpd(child, cpt) {
                        eprintln!("DEBUG BIF: set_cpd({}) failed: {:?}", child, e);
                    }
                }
                Err(e) => eprintln!("DEBUG BIF: from_values({}) failed: {:?}", child, e),
            }
        }

        Ok(network)
    }

    /// Imports a Bayesian network from a BIF file at the given path.
    pub fn import_from_file<P: AsRef<Path>>(path: P) -> LutufiResult<BayesianNetwork> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            LutufiError::DeserializationError {
                reason: format!("Failed to read BIF file: {}", e),
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
        net.add_variable("X", Domain::binary());
        net.add_variable("Y", Domain::binary());
        net.add_edge("X", "Y").unwrap();
        let x_var = net.variable("X").unwrap().clone();
        let y_var = net.variable("Y").unwrap().clone();
        let cpt_x = ConditionalProbabilityTable::from_values(
            &x_var,
            &[] as &[&Variable],
            vec![vec![0.5], vec![0.5]],
        )
        .unwrap();
        net.set_cpd("X", cpt_x).unwrap();
        let cpt_y = ConditionalProbabilityTable::from_values(
            &y_var,
            &[&x_var],
            vec![vec![0.9, 0.2], vec![0.1, 0.8]],
        )
        .unwrap();
        net.set_cpd("Y", cpt_y).unwrap();
        net
    }

    #[test]
    fn test_bif_export_contains_variables() {
        let net = create_simple_network();
        let bif = BifFormat::export(&net).unwrap();
        assert!(bif.contains("variable X"));
        assert!(bif.contains("variable Y"));
        assert!(bif.contains("probability ( X )"));
        assert!(bif.contains("probability ( Y | X )"));
    }

    #[test]
    fn test_bif_roundtrip() {
        let original = create_simple_network();
        let bif = BifFormat::export(&original).unwrap();
        let imported = BifFormat::import(&bif).unwrap();
        eprintln!("original nodes={} edges={}, imported nodes={} edges={}",
            original.nodes().len(), original.edges().len(),
            imported.nodes().len(), imported.edges().len());
        assert_eq!(original.nodes().len(), imported.nodes().len());
        assert_eq!(original.edges().len(), imported.edges().len());
        for name in original.nodes() {
            let orig_cpd = original.cpd(name).unwrap();
            let imp_cpd = imported.cpd(name).unwrap();
            let orig_factor = orig_cpd.as_factor();
            let imp_factor = imp_cpd.as_factor();
            let n = orig_factor.scope().num_entries();
            let mut orig_vals: Vec<f64> = (0..n).map(|i| orig_factor.value_at(i)).collect();
            let mut imp_vals: Vec<f64> = (0..n).map(|i| imp_factor.value_at(i)).collect();
            orig_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            imp_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for i in 0..n {
                let diff = (orig_vals[i] - imp_vals[i]).abs();
                assert!(
                    diff < 1e-6,
                    "Mismatch for '{}' at sorted index {}: {} vs {}",
                    name,
                    i,
                    orig_vals[i],
                    imp_vals[i]
                );
            }
        }
    }

    #[test]
    fn test_bif_export_file() {
        let net = create_simple_network();
        let tmp = std::env::temp_dir().join("test_export.bif");
        BifFormat::export_to_file(&net, &tmp).unwrap();
        let imported = BifFormat::import_from_file(&tmp).unwrap();
        let _ = std::fs::remove_file(&tmp);
        assert_eq!(imported.nodes().len(), 2);
    }
}
