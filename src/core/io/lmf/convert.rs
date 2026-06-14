use crate::core::{
    domain::Domain,
    error::{LutufiError, LutufiResult},
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    variable::VariableId,
};
use super::types::{
    LmfCpd, LmfDocument, LmfDomain, LmfGraph, LmfMetadata, LmfParameters,
    LmfVariable, ModelType, LMF_CURRENT_VERSION, flatten_to_2d,
};

impl LmfDocument {
    /// Converts a [`BayesianNetwork`] into an [`LmfDocument`] by extracting
    /// variables, edges, and conditional probability tables.
    pub fn from_bayesian_network(network: &BayesianNetwork) -> LutufiResult<Self> {
        let variables = network.variables();
        let node_names: Vec<&str> = network.nodes();

        let lmf_vars: Vec<LmfVariable> = node_names
            .iter()
            .filter_map(|name| {
                let var = network.variable(name).ok()?;
                Some(LmfVariable {
                    name: name.to_string(),
                    domain: LmfDomain::from(var.domain()),
                })
            })
            .collect();

        let edges: Vec<(&str, &str)> = network.edges();
        let lmf_edges: Vec<[String; 2]> = edges
            .into_iter()
            .map(|(from, to)| [from.to_string(), to.to_string()])
            .collect();

        let mut cpds = Vec::new();
        for (var_id, cpt) in network.cpds() {
            let var_name = match variables.get(var_id) {
                Some(v) => v.name().to_string(),
                None => continue,
            };
            let parent_ids = cpt.parent_ids();
            let parent_names: Vec<String> = parent_ids
                .iter()
                .filter_map(|pid| variables.get(pid).map(|v| v.name().to_string()))
                .collect();
            let factor = cpt.as_factor();
            let scope = factor.scope();
            let n = scope.num_entries();
            let mut table_uuidsorted = Vec::with_capacity(n);
            for i in 0..n {
                table_uuidsorted.push(factor.value_at(i));
            }
            let canonical_vars: Vec<VariableId> = parent_ids
                .iter()
                .chain(std::iter::once(var_id))
                .copied()
                .collect();
            let canonical_sizes: Vec<usize> = canonical_vars
                .iter()
                .filter_map(|id| variables.get(id))
                .map(|v| v.domain().size().unwrap_or(1))
                .collect();
            let uuid_vars = scope.variable_ids();
            let uuid_sizes = scope.sizes();
            let mut table = Vec::with_capacity(n);
            for i in 0..n {
                let indices = crate::core::factor::multi_index_from_flat(i, &canonical_sizes);
                let orig_idx = crate::core::factor::project_indices(
                    &indices,
                    &canonical_vars,
                    uuid_vars,
                    uuid_sizes,
                )?;
                table.push(table_uuidsorted[orig_idx]);
            }
            cpds.push(LmfCpd {
                child: var_name,
                parents: parent_names,
                table,
                log_space: false,
            });
        }

        let metadata = LmfMetadata {
            name: "Untitled".to_string(),
            lutufi_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            author: None,
            description: None,
        };

        Ok(LmfDocument {
            format_version: LMF_CURRENT_VERSION.to_string(),
            metadata,
            model_type: ModelType::BayesianNetwork,
            graph: LmfGraph {
                variables: lmf_vars,
                edges: lmf_edges,
            },
            parameters: LmfParameters { cpds },
            evidence: None,
            inference_settings: None,
            results: None,
        })
    }

    /// Reconstructs a [`BayesianNetwork`] from this [`LmfDocument`] by
    /// re-creating variables, edges, and CPDs from the LMF representation.
    pub fn to_bayesian_network(&self) -> LutufiResult<BayesianNetwork> {
        let mut network = BayesianNetwork::new();

        for var in &self.graph.variables {
            let domain = Domain::try_from(&var.domain)?;
            network.add_variable(&var.name, domain)?;
        }

        for edge in &self.graph.edges {
            if edge.len() != 2 {
                return Err(LutufiError::DeserializationError {
                    reason: format!("Invalid edge format: {:?}", edge),
                });
            }
            network.add_edge(&edge[0], &edge[1])?;
        }

        for cpd in &self.parameters.cpds {
            let child_id = network.id_of(&cpd.child)?;
            let parent_ids: Vec<VariableId> = cpd
                .parents
                .iter()
                .filter_map(|name| network.id_of(name).ok())
                .collect();

            let child_var = network.variables().get(&child_id).ok_or_else(|| {
                LutufiError::DeserializationError {
                    reason: format!("Child variable '{}' not found after creation", cpd.child),
                }
            })?;
            let child_domain_size = child_var.domain().size().unwrap_or(1);

            let mut parent_sizes = Vec::new();
            for pid in &parent_ids {
                let pv = network.variables().get(pid).ok_or_else(|| {
                    LutufiError::DeserializationError {
                        reason: "Parent variable not found after creation".to_string(),
                    }
                })?;
                parent_sizes.push(pv.domain().size().unwrap_or(1));
            }

            let parent_card: usize = parent_sizes.iter().product();
            let expected_len = child_domain_size * parent_card.max(1);

            let prob_values: Vec<f64> = if cpd.log_space {
                if cpd.table.len() != expected_len {
                    return Err(LutufiError::DeserializationError {
                        reason: format!(
                            "CPD for '{}' has {} values, expected {}",
                            cpd.child,
                            cpd.table.len(),
                            expected_len
                        ),
                    });
                }
                cpd.table.iter().map(|v| v.exp()).collect()
            } else {
                if cpd.table.len() != expected_len {
                    return Err(LutufiError::DeserializationError {
                        reason: format!(
                            "CPD for '{}' has {} values, expected {}",
                            cpd.child,
                            cpd.table.len(),
                            expected_len
                        ),
                    });
                }
                cpd.table.clone()
            };

            let matrix = flatten_to_2d(&prob_values, child_domain_size, parent_card);

            let cpt = ConditionalProbabilityTable::from_values(
                child_var,
                &parent_ids
                    .iter()
                    .filter_map(|pid| network.variables().get(pid))
                    .collect::<Vec<_>>(),
                matrix,
            )?;

            network.set_cpd(&cpd.child, cpt)?;
        }

        Ok(network)
    }
}
