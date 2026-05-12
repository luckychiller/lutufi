use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::core::error::{LutufiError, LutufiResult};
use crate::core::models::bayesian_network::BayesianNetwork;
use crate::core::variable::VariableId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalModel {
    pub network: BayesianNetwork,
    bidirected_edges: HashSet<(VariableId, VariableId)>,
}

impl CausalModel {
    pub fn new(mut network: BayesianNetwork) -> Self {
        network.mark_as_causal();
        CausalModel {
            network,
            bidirected_edges: HashSet::new(),
        }
    }

    pub fn ensure_causal(&self, operation: &str) -> LutufiResult<()> {
        if !self.network.is_causal() {
            Err(LutufiError::NonCausalModel {
                variable: operation.to_string(),
                value: String::new(),
            })
        } else {
            Ok(())
        }
    }

    pub fn mark_hidden_confounder(&mut self, var1: &str, var2: &str) -> LutufiResult<()> {
        let id1 = self.network.id_of(var1)?;
        let id2 = self.network.id_of(var2)?;
        let pair = if id1 < id2 { (id1, id2) } else { (id2, id1) };
        self.bidirected_edges.insert(pair);
        Ok(())
    }

    pub fn bidirected_edges(&self) -> &HashSet<(VariableId, VariableId)> {
        &self.bidirected_edges
    }
}

#[derive(Debug, Clone)]
pub enum IdentificationResult {
    Identifiable(IdentificationFormula),
    NotIdentifiable(String),
}

#[derive(Debug, Clone)]
pub struct IdentificationFormula {
    pub formula: String,
    pub targets: Vec<String>,
    pub interventions: Vec<String>,
}

impl IdentificationFormula {
    pub fn evaluate(
        &self,
        model: &CausalModel,
        outcome_value: &str,
    ) -> LutufiResult<std::collections::HashMap<String, f64>> {
        let mut results = std::collections::HashMap::new();

        let mut intervention_assign = crate::core::assignment::Assignment::new();
        for intervention in &self.interventions {
            let var = model.network.variable(intervention)?;
            let var_id = var.id();
            if var.domain().contains(outcome_value) {
                intervention_assign.set(var_id, outcome_value);
            } else {
                let states = match var.domain() {
                    crate::core::domain::Domain::Discrete { states } => states.clone(),
                    crate::core::domain::Domain::Binary => vec!["false".to_string(), "true".to_string()],
                    crate::core::domain::Domain::Continuous { .. } => return Err(LutufiError::InternalError {
                        message: "Cannot evaluate with continuous domains".to_string(),
                    }),
                };
                if let Some(first) = states.first() {
                    intervention_assign.set(var_id, first.clone());
                }
            }
        }

        let mutilated = model.do_operator(&intervention_assign)?;
        let query_vars: Vec<&str> = self.targets.iter().map(|s| s.as_str()).collect();

        for target in &self.targets {
            let result = mutilated.query(
                &query_vars,
                &crate::core::assignment::Assignment::new(),
                crate::core::inference::Algorithm::Auto,
            )?;
            results.insert(
                target.clone(),
                result.marginal_prob(target, outcome_value).unwrap_or(0.0),
            );
        }

        Ok(results)
    }
}

pub(crate) enum IdResult {
    Formula(String),
    Fail(String),
}
