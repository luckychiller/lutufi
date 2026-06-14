use std::collections::{HashMap, HashSet};
use crate::core::error::LutufiResult;
use crate::core::models::bayesian_network::BayesianNetwork;
use super::types::EdgeOrientation;

/// Builds a [`BayesianNetwork`] from oriented edges produced by constraint-based
/// structure learning.
pub struct ConstrainedNetworkBuilder;

impl ConstrainedNetworkBuilder {
    /// Constructs a [`BayesianNetwork`] by adding variables with their domain
    /// states from the data, then adding edges according to the given orientations.
    pub fn build(
        node_names: &[String],
        orientations: &HashMap<(String, String), EdgeOrientation>,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<BayesianNetwork> {
        let mut model = BayesianNetwork::new();

        for name in node_names {
            let mut states = HashSet::new();
            for row in data {
                if let Some(val) = row.get(name) {
                    states.insert(val.clone());
                }
            }
            let mut sorted_states: Vec<_> = states.into_iter().collect();
            sorted_states.sort();
            model.add_variable(
                name,
                crate::core::domain::Domain::discrete(sorted_states)?,
            )?;
        }

        for ((u, v), orientation) in orientations {
            match orientation {
                EdgeOrientation::Directed => {
                    model.add_edge(u, v)?;
                }
                EdgeOrientation::ReverseDirected => {
                    model.add_edge(v, u)?;
                }
                EdgeOrientation::Undirected => {
                    if model.add_edge(u, v).is_err() {
                        model.add_edge(v, u)?;
                    }
                }
            }
        }

        Ok(model)
    }
}
