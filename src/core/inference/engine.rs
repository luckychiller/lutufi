use std::collections::HashMap;
use std::time::Duration;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::TabularFactor,
    models::bayesian_network::BayesianNetwork,
    models::factor_graph::FactorGraph,
};
use super::{lbp, mcmc, variational, junction_tree};

/// Choice of inference algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    /// Automatically select the best algorithm.
    Auto,
    /// Exact inference using Junction Tree.
    Exact,
    /// Loopy Belief Propagation.
    LBP,
    /// Markov Chain Monte Carlo (Gibbs sampling).
    MCMC,
    /// Mean Field Variational Inference.
    Variational,
}

/// Unified result structure for all inference algorithms.
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// The variables queried.
    pub variables: Vec<String>,
    /// Estimated marginal distributions for each variable.
    pub distributions: HashMap<String, TabularFactor>,
    /// Which algorithm was actually used.
    pub algorithm_used: Algorithm,
    /// How long the computation took.
    pub computation_time: Duration,
    /// Algorithm-specific diagnostics.
    pub diagnostics: Diagnostics,
}

/// Algorithm-specific diagnostic information.
#[derive(Debug, Clone)]
pub enum Diagnostics {
    /// Exact inference diagnostic (treewidth).
    JunctionTree { 
        /// Estimated treewidth of the model.
        treewidth: usize 
    },
    /// LBP diagnostics (convergence status, iterations).
    LBP { 
        /// Whether the algorithm converged within tolerance.
        converged: bool, 
        /// Number of iterations performed.
        iterations: usize, 
        /// Final maximum message change.
        residual: f64 
    },
    /// MCMC diagnostics (sample count).
    MCMC { 
        /// Number of samples collected per chain.
        n_samples: usize 
    },
    /// Variational diagnostics (ELBO, convergence).
    Variational { 
        /// Final Evidence Lower Bound (higher is better).
        elbo: f64, 
        /// Whether CAVI converged.
        converged: bool, 
        /// Total iterations performed.
        iterations: usize 
    },
    /// No diagnostics available.
    None,
}

/// Factory for creating and running inference engines.
pub struct InferenceEngine;

impl InferenceEngine {
    /// Run a query using the specified algorithm.
    pub fn query(
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
        algorithm: Algorithm,
    ) -> LutufiResult<InferenceResult> {
        let start_time = std::time::Instant::now();
        
        let actual_algorithm = if algorithm == Algorithm::Auto {
            Self::select_algorithm(model)
        } else {
            algorithm
        };

        match actual_algorithm {
            Algorithm::Exact => {
                let jt = junction_tree::JunctionTreeEngine::new(model)?;
                let mut distributions = HashMap::new();
                for &var in variables {
                    distributions.insert(var.to_string(), jt.query(&[var], evidence)?);
                }
                Ok(InferenceResult {
                    variables: variables.iter().map(|&s| s.to_string()).collect(),
                    distributions,
                    algorithm_used: Algorithm::Exact,
                    computation_time: start_time.elapsed(),
                    diagnostics: Diagnostics::JunctionTree { treewidth: jt.treewidth() },
                })
            }
            Algorithm::LBP => {
                let fg = FactorGraph::from_bayesian_network(model)?;
                let mut engine = lbp::LBPEngine::new(fg, lbp::LBPOptions::default());
                let res = engine.run(evidence)?;
                
                let mut distributions = HashMap::new();
                for &var_name in variables {
                    let var_id = model.id_of(var_name)?;
                    let belief = res.beliefs.get(&var_id).ok_or_else(|| LutufiError::VariableNotFound { 
                        name: var_name.to_string(), 
                        available: "".to_string() 
                    })?.clone();
                    distributions.insert(var_name.to_string(), belief);
                }

                Ok(InferenceResult {
                    variables: variables.iter().map(|&s| s.to_string()).collect(),
                    distributions,
                    algorithm_used: Algorithm::LBP,
                    computation_time: start_time.elapsed(),
                    diagnostics: Diagnostics::LBP {
                        converged: res.converged,
                        iterations: res.iterations,
                        residual: res.residual,
                    },
                })
            }
            Algorithm::MCMC => {
                let fg = FactorGraph::from_bayesian_network(model)?;
                let engine = mcmc::MCMCEngine::new(fg, mcmc::MCMCOptions::default());
                let res = engine.run(evidence)?;
                
                let mut distributions = HashMap::new();
                for &var_name in variables {
                    let var_id = model.id_of(var_name)?;
                    let marginal = res.marginals.get(&var_id).ok_or_else(|| LutufiError::InternalError { 
                        message: format!("MCMC failed to produce marginal for {}", var_name) 
                    })?;
                    distributions.insert(var_name.to_string(), marginal.clone());
                }

                Ok(InferenceResult {
                    variables: variables.iter().map(|&s| s.to_string()).collect(),
                    distributions,
                    algorithm_used: Algorithm::MCMC,
                    computation_time: start_time.elapsed(),
                    diagnostics: Diagnostics::MCMC { n_samples: res.n_samples },
                })
            }
            Algorithm::Variational => {
                let fg = FactorGraph::from_bayesian_network(model)?;
                let engine = variational::VariationalEngine::new(fg, variational::VariationalOptions::default());
                let res = engine.run_with_evidence(evidence)?;
                
                let mut distributions = HashMap::new();
                for &var_name in variables {
                    let var_id = model.id_of(var_name)?;
                    let marginal = res.marginals.get(&var_id).ok_or_else(|| LutufiError::InternalError { 
                        message: format!("Variational inference failed to produce marginal for {}", var_name) 
                    })?;
                    distributions.insert(var_name.to_string(), marginal.clone());
                }

                Ok(InferenceResult {
                    variables: variables.iter().map(|&s| s.to_string()).collect(),
                    distributions,
                    algorithm_used: Algorithm::Variational,
                    computation_time: start_time.elapsed(),
                    diagnostics: Diagnostics::Variational {
                        elbo: res.elbo,
                        converged: res.converged,
                        iterations: res.elbo_history.len(),
                    },
                })
            }
            Algorithm::Auto => unreachable!(),
        }
    }

    fn select_algorithm(model: &BayesianNetwork) -> Algorithm {
        if model.variables.len() <= 20 {
            Algorithm::Exact
        } else {
            Algorithm::LBP
        }
    }
}
