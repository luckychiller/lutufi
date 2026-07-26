//! Inference Strategy Pattern
//!
//! This module defines the Strategy pattern for inference algorithms.
//! Each algorithm implements the InferenceStrategy trait, allowing them
//! to be swapped at runtime without modifying the InferenceEngine.

use crate::core::{
    assignment::Assignment,
    error::LutufiResult,
    factor::TabularFactor,
    models::bayesian_network::BayesianNetwork,
};
use std::collections::HashMap;
use std::time::Duration;

/// Metadata about an inference algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    /// Automatically select the best algorithm.
    Auto,
    /// Exact inference using Variable Elimination.
    VariableElimination,
    /// Exact inference using Junction Tree.
    Exact,
    /// Loopy Belief Propagation.
    LBP,
    /// Markov Chain Monte Carlo (Gibbs sampling).
    MCMC,
    /// Mean Field Variational Inference.
    Variational,
}

impl Algorithm {
    /// Get a human-readable name for the algorithm.
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::Auto => "Auto",
            Algorithm::VariableElimination => "Variable Elimination",
            Algorithm::Exact => "Junction Tree",
            Algorithm::LBP => "Loopy Belief Propagation",
            Algorithm::MCMC => "Markov Chain Monte Carlo",
            Algorithm::Variational => "Variational Inference",
        }
    }
}

/// Algorithm-specific diagnostic information.
#[derive(Debug, Clone)]
pub enum Diagnostics {
    /// Exact inference diagnostic (treewidth).
    JunctionTree {
        /// Estimated treewidth of the model.
        treewidth: usize,
    },
    /// LBP diagnostics (convergence status, iterations).
    LBP {
        /// Whether the algorithm converged within tolerance.
        converged: bool,
        /// Number of iterations performed.
        iterations: usize,
        /// Final maximum message change.
        residual: f64,
    },
    /// MCMC diagnostics (sample count).
    MCMC {
        /// Number of samples collected per chain.
        n_samples: usize,
    },
    /// Variational diagnostics (ELBO, convergence).
    Variational {
        /// Final Evidence Lower Bound (higher is better).
        elbo: f64,
        /// Whether CAVI converged.
        converged: bool,
        /// Total iterations performed.
        iterations: usize,
    },
    /// No diagnostics available.
    None,
}

/// Unified result structure for all inference algorithms.
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// The variables queried.
    pub variables: Vec<String>,
    /// Estimated marginal distributions for each variable.
    pub distributions: HashMap<String, TabularFactor>,
    /// The joint factor for the query variables (if requested/available).
    pub joint_factor: Option<TabularFactor>,
    /// Log of the partition function (log normalization constant).
    pub log_z: f64,
    /// Which algorithm was actually used.
    pub algorithm_used: Algorithm,
    /// How long the computation took.
    pub computation_time: Duration,
    /// Algorithm-specific diagnostics.
    pub diagnostics: Diagnostics,
    /// State names per variable, in index order, when the caller supplied them.
    ///
    /// A `TabularFactor` carries only variable IDs and domain *sizes*, so a
    /// result is not self-describing on its own: index 0 of a distribution is
    /// just index 0. Anything rendering a result for humans — CSV export, plots,
    /// `marginal_prob` lookups by name — needs the labels, and inventing them
    /// (as the CSV writer previously did, assuming `false`/`true` for binary
    /// variables) silently corrupts output for every model that names its states
    /// otherwise.
    ///
    /// Empty when the producing engine did not have access to the model's
    /// domains; consumers must fall back to indices rather than guess.
    #[doc(alias = "labels")]
    pub state_names: HashMap<String, Vec<String>>,
}

/// Collect the state labels of `variables` from the model, in index order.
///
/// `Domain::Binary` reports no explicit states, so it is rendered with the
/// canonical `false`/`true` labels — the same convention the rest of the library
/// uses for that domain type, and correct here because `Binary` really does mean
/// those two states. Named binary domains are `Domain::Discrete` and keep their
/// own labels.
pub fn collect_state_names(
    model: &crate::core::models::bayesian_network::BayesianNetwork,
    variables: &[&str],
) -> HashMap<String, Vec<String>> {
    let mut names = HashMap::new();
    for &var_name in variables {
        let Ok(id) = model.id_of(var_name) else { continue };
        let Some(var) = model.variables().get(&id) else { continue };
        let labels = match var.domain() {
            crate::core::domain::Domain::Binary => {
                vec!["false".to_string(), "true".to_string()]
            }
            domain => match domain.states() {
                Some(states) => states.to_vec(),
                None => continue,
            },
        };
        names.insert(var_name.to_string(), labels);
    }
    names
}

impl InferenceResult {
    /// State labels for `variable`, in index order, if they are known.
    pub fn states_of(&self, variable: &str) -> Option<&[String]> {
        self.state_names.get(variable).map(|v| v.as_slice())
    }

    /// The label for state `index` of `variable`, falling back to the index
    /// rendered as a string when labels are unavailable.
    ///
    /// Never guesses: an unlabelled binary variable renders as `"0"`/`"1"`, not
    /// as `"false"`/`"true"`, because those are only correct by coincidence.
    pub fn state_label(&self, variable: &str, index: usize) -> String {
        self.state_names
            .get(variable)
            .and_then(|names| names.get(index))
            .cloned()
            .unwrap_or_else(|| index.to_string())
    }

    /// Get the marginal probability of a specific value for a variable.
    /// For a univariate factor, the i-th index corresponds to the i-th state.
    /// Supports lookup by state name or by position index ("0", "1").
    pub fn marginal_prob(&self, variable: &str, value: &str) -> LutufiResult<f64> {
        let factor = self.distributions.get(variable).ok_or_else(|| {
            crate::core::error::LutufiError::VariableNotFound {
                name: variable.to_string(),
                available: format!("{:?}", self.variables),
            }
        })?;
        let scope = factor.scope();

        // Prefer the model's own labels when the engine recorded them. The
        // `false`/`true` fallback below is a guess that happens to be right for
        // some binary encodings and wrong for others ("no"/"yes", "F"/"T",
        // "absent"/"present"), so it must never take precedence over real labels.
        if let Some(names) = self.state_names.get(variable) {
            if let Some(idx) = names.iter().position(|n| n == value) {
                return Ok(factor.value_at(idx));
            }
        }

        let idx = match value {
            "false" | "False" => 0,
            "true" | "True" => 1,
            _ => {
                // Try parsing as numeric index
                if let Ok(n) = value.parse::<usize>() {
                    n
                } else {
                    // Try matching by name via assignment (for Discrete domains with custom names)
                    let var_id = scope.variable_ids()[0];
                    let mut found = None;
                    for i in 0..scope.num_entries() {
                        let assignment = scope.assignment_from_flat(i)?;
                        if assignment.get(&var_id) == Some(value) {
                            found = Some(i);
                            break;
                        }
                    }
                    match found {
                        Some(i) => i,
                        None => return Err(crate::core::error::LutufiError::ValueNotInDomain {
                            value: value.to_string(),
                            variable: variable.to_string(),
                            valid_values: "unknown".to_string(),
                        }),
                    }
                }
            }
        };

        if idx < scope.num_entries() {
            Ok(factor.value_at(idx))
        } else {
            Err(crate::core::error::LutufiError::ValueNotInDomain {
                value: value.to_string(),
                variable: variable.to_string(),
                valid_values: "unknown".to_string(),
            })
        }
    }
}

/// Strategy trait for inference algorithms.
///
/// Each inference algorithm implements this trait. This allows the
/// InferenceEngine to support new algorithms without modification,
/// following the Open/Closed Principle.
pub trait InferenceStrategy: Send + Sync {
    /// Get the algorithm identifier.
    fn algorithm(&self) -> Algorithm;

    /// Run inference on the given model with evidence.
    fn infer(
        &self,
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<InferenceResult>;

    /// Estimate the computational cost for this model.
    /// Returns a rough cost estimate (higher = more expensive).
    fn estimated_cost(&self, model: &BayesianNetwork) -> usize {
        // Default: estimate based on model size
        model.nodes().len() * 100
    }

    /// Check if this algorithm can handle this model.
    /// Returns true if the algorithm is suitable.
    fn can_handle(&self, _model: &BayesianNetwork) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_name() {
        assert_eq!(Algorithm::VariableElimination.name(), "Variable Elimination");
        assert_eq!(Algorithm::LBP.name(), "Loopy Belief Propagation");
        assert_eq!(Algorithm::MCMC.name(), "Markov Chain Monte Carlo");
    }
}
