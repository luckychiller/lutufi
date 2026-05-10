//! Concrete implementations of inference strategies.
//!
//! This module provides strategy implementations for each inference algorithm,
//! allowing the InferenceEngine to use them polymorphically without hardcoding
//! algorithm-specific logic.

use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    models::{bayesian_network::BayesianNetwork, factor_graph::FactorGraph},
};
use std::collections::HashMap;
use std::time::Instant;

use super::strategy::{Algorithm, Diagnostics, InferenceResult, InferenceStrategy};
use super::{junction_tree, lbp, mcmc, variational, variable_elimination};

/// Variable Elimination Strategy - Exact inference for general DAGs.
pub struct VariableEliminationStrategy;

impl InferenceStrategy for VariableEliminationStrategy {
    fn algorithm(&self) -> Algorithm {
        Algorithm::VariableElimination
    }

    fn infer(
        &self,
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<InferenceResult> {
        let start_time = Instant::now();

        let ve = variable_elimination::VariableEliminationEngine::new(model);
        let joint_raw =
            ve.query_raw(variables, evidence, variable_elimination::EliminationHeuristic::MinFill)?;

        let mut log_z = f64::NEG_INFINITY;
        for i in 0..joint_raw.scope().num_entries() {
            log_z = crate::core::factor::log_sum_exp(log_z, joint_raw.log_value_at(i));
        }

        let mut distributions = HashMap::new();
        let mut joint = joint_raw.clone();
        joint.normalize();

        for &var in variables {
            let var_id = model.id_of(var)?;
            let vars_to_sum: Vec<crate::core::variable::VariableId> = joint
                .scope()
                .variable_ids()
                .iter()
                .filter(|&&id| id != var_id)
                .copied()
                .collect();
            let mut marginal = joint.marginalize(&vars_to_sum)?;
            marginal.normalize();
            distributions.insert(var.to_string(), marginal);
        }

        Ok(InferenceResult {
            variables: variables.iter().map(|&s| s.to_string()).collect(),
            distributions,
            joint_factor: Some(joint),
            log_z,
            algorithm_used: Algorithm::VariableElimination,
            computation_time: start_time.elapsed(),
            diagnostics: Diagnostics::None,
        })
    }

    fn estimated_cost(&self, model: &BayesianNetwork) -> usize {
        // Cost grows with number of nodes and edges
        model.nodes().len() * 100 + model.edges().len() * 50
    }

    fn can_handle(&self, model: &BayesianNetwork) -> bool {
        // VE works for any size DAG, but becomes impractical for large models
        model.nodes().len() < 30
    }
}

/// Junction Tree Strategy - Exact inference using compiled networks.
pub struct JunctionTreeStrategy;

impl InferenceStrategy for JunctionTreeStrategy {
    fn algorithm(&self) -> Algorithm {
        Algorithm::Exact
    }

    fn infer(
        &self,
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<InferenceResult> {
        let start_time = Instant::now();

        let jt = junction_tree::JunctionTreeEngine::new(model)?;
        let treewidth = jt.treewidth();

        let mut distributions = HashMap::new();
        for &var in variables {
            distributions.insert(var.to_string(), jt.query(&[var], evidence)?);
        }

        Ok(InferenceResult {
            variables: variables.iter().map(|&s| s.to_string()).collect(),
            distributions,
            joint_factor: None,
            log_z: 0.0,
            algorithm_used: Algorithm::Exact,
            computation_time: start_time.elapsed(),
            diagnostics: Diagnostics::JunctionTree { treewidth },
        })
    }

    fn estimated_cost(&self, model: &BayesianNetwork) -> usize {
        // Compilation cost is higher upfront, but queries are fast
        let base = model.nodes().len() * 200;
        let structural_cost = model.edges().len() * 100;
        base + structural_cost
    }

    fn can_handle(&self, _model: &BayesianNetwork) -> bool {
        true
    }
}

/// Loopy Belief Propagation Strategy - Approximate inference.
pub struct LBPStrategy;

impl InferenceStrategy for LBPStrategy {
    fn algorithm(&self) -> Algorithm {
        Algorithm::LBP
    }

    fn infer(
        &self,
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<InferenceResult> {
        let start_time = Instant::now();

        let fg = FactorGraph::from_bayesian_network(model)?;
        let mut engine = lbp::LBPEngine::new(fg, lbp::LBPOptions::default());
        let res = engine.run(evidence)?;

        let mut distributions = HashMap::new();
        for &var_name in variables {
            let var_id = model.id_of(var_name)?;
            let belief = res
                .beliefs
                .get(&var_id)
                .ok_or_else(|| LutufiError::VariableNotFound {
                    name: var_name.to_string(),
                    available: "".to_string(),
                })?
                .clone();
            distributions.insert(var_name.to_string(), belief);
        }

        Ok(InferenceResult {
            variables: variables.iter().map(|&s| s.to_string()).collect(),
            distributions,
            joint_factor: None,
            log_z: 0.0,
            algorithm_used: Algorithm::LBP,
            computation_time: start_time.elapsed(),
            diagnostics: Diagnostics::LBP {
                converged: res.converged,
                iterations: res.iterations,
                residual: res.residual,
            },
        })
    }

    fn estimated_cost(&self, model: &BayesianNetwork) -> usize {
        // LBP is cheap for loopy graphs
        model.nodes().len() * 10
    }

    fn can_handle(&self, _model: &BayesianNetwork) -> bool {
        true
    }
}

/// MCMC Strategy - Approximate inference via sampling.
pub struct MCMCStrategy;

impl InferenceStrategy for MCMCStrategy {
    fn algorithm(&self) -> Algorithm {
        Algorithm::MCMC
    }

    fn infer(
        &self,
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<InferenceResult> {
        let start_time = Instant::now();

        let fg = FactorGraph::from_bayesian_network(model)?;
        let engine = mcmc::MCMCEngine::new(fg, mcmc::MCMCOptions::default());
        let res = engine.run(evidence)?;

        let mut distributions = HashMap::new();
        for &var_name in variables {
            let var_id = model.id_of(var_name)?;
            let marginal = res.marginals.get(&var_id).ok_or_else(|| LutufiError::InternalError {
                message: format!("MCMC failed to produce marginal for {}", var_name),
            })?;
            distributions.insert(var_name.to_string(), marginal.clone());
        }

        Ok(InferenceResult {
            variables: variables.iter().map(|&s| s.to_string()).collect(),
            distributions,
            joint_factor: None,
            log_z: 0.0,
            algorithm_used: Algorithm::MCMC,
            computation_time: start_time.elapsed(),
            diagnostics: Diagnostics::MCMC {
                n_samples: res.n_samples,
            },
        })
    }

    fn estimated_cost(&self, _model: &BayesianNetwork) -> usize {
        // MCMC has moderate cost for sampling
        1000
    }

    fn can_handle(&self, _model: &BayesianNetwork) -> bool {
        true
    }
}

/// Variational Inference Strategy - Approximate inference via optimization.
pub struct VariationalStrategy;

impl InferenceStrategy for VariationalStrategy {
    fn algorithm(&self) -> Algorithm {
        Algorithm::Variational
    }

    fn infer(
        &self,
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<InferenceResult> {
        let start_time = Instant::now();

        let fg = FactorGraph::from_bayesian_network(model)?;
        let engine = variational::VariationalEngine::new(fg, variational::VariationalOptions::default());
        let res = engine.run_with_evidence(evidence)?;

        let mut distributions = HashMap::new();
        for &var_name in variables {
            let var_id = model.id_of(var_name)?;
            let marginal =
                res.marginals
                    .get(&var_id)
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Variational inference failed to produce marginal for {}", var_name),
                    })?;
            distributions.insert(var_name.to_string(), marginal.clone());
        }

        Ok(InferenceResult {
            variables: variables.iter().map(|&s| s.to_string()).collect(),
            distributions,
            joint_factor: None,
            log_z: 0.0,
            algorithm_used: Algorithm::Variational,
            computation_time: start_time.elapsed(),
            diagnostics: Diagnostics::Variational {
                elbo: res.elbo,
                converged: res.converged,
                iterations: res.elbo_history.len(),
            },
        })
    }

    fn estimated_cost(&self, model: &BayesianNetwork) -> usize {
        // Variational is generally faster than MCMC
        model.nodes().len() * 50
    }

    fn can_handle(&self, _model: &BayesianNetwork) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_identification() {
        let ve_strategy = VariableEliminationStrategy;
        assert_eq!(ve_strategy.algorithm(), Algorithm::VariableElimination);

        let jt_strategy = JunctionTreeStrategy;
        assert_eq!(jt_strategy.algorithm(), Algorithm::Exact);

        let lbp_strategy = LBPStrategy;
        assert_eq!(lbp_strategy.algorithm(), Algorithm::LBP);
    }

    #[test]
    fn test_cost_estimation() {
        let ve_strategy = VariableEliminationStrategy;
        let mut bn = BayesianNetwork::new();
        bn.add_variable("A", crate::core::domain::Domain::binary()).unwrap();
        assert!(ve_strategy.estimated_cost(&bn) > 0);
    }
}
