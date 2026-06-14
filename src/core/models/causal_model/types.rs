use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::core::error::{LutufiError, LutufiResult};
use crate::core::models::bayesian_network::BayesianNetwork;
use crate::core::variable::VariableId;

/// A causal graphical model consisting of a Bayesian network augmented with bidirected edges
/// representing unobserved confounders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalModel {
    /// The underlying Bayesian network structure and parameters.
    pub network: BayesianNetwork,
    bidirected_edges: HashSet<(VariableId, VariableId)>,
}

impl CausalModel {
    /// Creates a new causal model by marking the given Bayesian network as causal.
    pub fn new(mut network: BayesianNetwork) -> Self {
        network.mark_as_causal();
        CausalModel {
            network,
            bidirected_edges: HashSet::new(),
        }
    }

    /// Ensures the model is causal, returning a `NonCausalModel` error otherwise.
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

    /// Marks a hidden confounder between two variables by adding a bidirected edge.
    pub fn mark_hidden_confounder(&mut self, var1: &str, var2: &str) -> LutufiResult<()> {
        let id1 = self.network.id_of(var1)?;
        let id2 = self.network.id_of(var2)?;
        let pair = if id1 < id2 { (id1, id2) } else { (id2, id1) };
        self.bidirected_edges.insert(pair);
        Ok(())
    }

    /// Returns the set of bidirected edges representing hidden confounders.
    pub fn bidirected_edges(&self) -> &HashSet<(VariableId, VariableId)> {
        &self.bidirected_edges
    }
}

/// The result of an identification query.
#[derive(Debug, Clone)]
pub enum IdentificationResult {
    /// The causal effect is identifiable and has a formula.
    Identifiable(IdentificationFormula),
    /// The causal effect is not identifiable, with a reason string.
    NotIdentifiable(String),
}

/// A formula representing an identified causal effect.
#[derive(Debug, Clone)]
pub struct IdentificationFormula {
    /// The string representation of the identification formula.
    pub formula: String,
    /// The target variables of the causal query.
    pub targets: Vec<String>,
    /// The intervention variables of the causal query.
    pub interventions: Vec<String>,
}

impl IdentificationFormula {
    /// Evaluates the identification formula on the given model, returning marginal probabilities
    /// for each target variable given the specified outcome value.
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
