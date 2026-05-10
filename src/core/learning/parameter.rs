//! Parameter learning algorithms.
//!
//! Provides MLE, Bayesian estimation (Dirichlet), and EM algorithm.

use std::collections::HashMap;
use crate::core::{
    error::{LutufiError, LutufiResult},
    models::bayesian_network::BayesianNetwork,
    variable::VariableId,
};

/// Smoothing strategy for parameter learning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmoothingMethod {
    /// MLE with Laplace smoothing.
    Laplace,
    /// Bayesian estimation with Dirichlet priors (equivalent sample size).
    Bayesian,
}

/// Method for parameter learning (for backward compatibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterLearningMethod {
    /// Maximum Likelihood Estimation.
    MLE,
    /// Bayesian Parameter Estimation with Dirichlet priors.
    Bayesian,
}

/// Options for parameter learning.
#[derive(Debug, Clone)]
pub struct ParameterLearningOptions {
    /// Smoothing method (Laplace or Bayesian).
    pub smoothing: SmoothingMethod,
    /// Pseudocount or alpha for smoothing.
    pub alpha: f64,
    /// Maximum iterations for EM algorithm.
    pub max_iterations: usize,
    /// Convergence threshold for EM log-likelihood.
    pub tolerance: f64,
}

impl Default for ParameterLearningOptions {
    fn default() -> Self {
        Self {
            smoothing: SmoothingMethod::Laplace,
            alpha: 0.5,
            max_iterations: 100,
            tolerance: 1e-4,
        }
    }
}

/// Compatibility layer for legacy options.
#[derive(Debug, Clone)]
pub struct LegacyParameterLearningOptions {
    /// Learning method (MLE or Bayesian).
    pub method: ParameterLearningMethod,
    /// Pseudocount for Laplace smoothing (MLE) or equivalent sample size (Bayesian).
    pub alpha: f64,
    /// Maximum iterations for EM algorithm.
    pub max_iterations: usize,
    /// Convergence tolerance for EM log-likelihood.
    pub tolerance: f64,
}

impl Default for LegacyParameterLearningOptions {
    fn default() -> Self {
        Self {
            method: ParameterLearningMethod::MLE,
            alpha: 0.5,
            max_iterations: 100,
            tolerance: 1e-4,
        }
    }
}

/// Common data collection logic.
struct DataCollector;

impl DataCollector {
    /// Collect counts of variable assignments from data.
    fn collect_counts(
        model: &BayesianNetwork,
        node_name: &str,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<(Vec<f64>, crate::core::factor::Scope, VariableId)> {
        let child_id = model.id_of(node_name)?;
        let parents = model.graph.parents(&child_id);
        let child_var = model.registry().variable(&child_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: format!("{:?}", child_id),
            available: format!("{:?}", model.nodes()),
        })?;

        let mut scope_vars = parents
            .iter()
            .map(|id| model.registry().variable(id).ok_or_else(|| LutufiError::VariableNotFound {
                name: format!("{:?}", id),
                available: format!("{:?}", model.nodes()),
            }))
            .collect::<LutufiResult<Vec<_>>>()?;
        scope_vars.push(child_var);
        let scope = crate::core::factor::Scope::new(scope_vars);

        let mut counts = vec![0.0; scope.num_entries()];

        for row in data {
            let mut assignment = crate::core::assignment::Assignment::new();
            let mut missing = false;

            if let Some(val) = row.get(node_name) {
                let idx = child_var.domain().index_of(val).ok_or_else(|| LutufiError::ValueNotInDomain {
                    value: val.to_string(),
                    variable: node_name.to_string(),
                    valid_values: format!("{:?}", child_var.domain()),
                })?;
                assignment.set_discrete(child_id, idx)?;
            } else {
                missing = true;
            }

            for &p_id in &parents {
                let p_var = model.registry().variable(&p_id).ok_or_else(|| LutufiError::VariableNotFound {
                    name: format!("{:?}", p_id),
                    available: format!("{:?}", model.nodes()),
                })?;
                if let Some(val) = row.get(p_var.name()) {
                    let idx = p_var
                        .domain()
                        .index_of(val)
                        .ok_or_else(|| LutufiError::ValueNotInDomain {
                            value: val.to_string(),
                            variable: p_var.name().to_string(),
                            valid_values: format!("{:?}", p_var.domain()),
                        })?;
                    assignment.set_discrete(p_id, idx)?;
                } else {
                    missing = true;
                }
            }

            if !missing {
                let mut flat_idx = 0;
                let mut stride = 1;
                for i in (0..scope.len()).rev() {
                    let var_id = scope.variable_ids()[i];
                    let val_idx = assignment.get_discrete(&var_id)?;
                    flat_idx += val_idx * stride;
                    stride *= scope.sizes()[i];
                }
                counts[flat_idx] += 1.0;
            }
        }

        Ok((counts, scope, child_id))
    }

    /// Build frequency matrix from counts.
    fn build_matrix(
        model: &BayesianNetwork,
        child_id: VariableId,
        counts: &[f64],
        scope: &crate::core::factor::Scope,
    ) -> LutufiResult<Vec<Vec<f64>>> {
        let child_var = model.registry().variable(&child_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: format!("{:?}", child_id),
            available: format!("{:?}", model.nodes()),
        })?;
        let parents = model.graph.parents(&child_id);
        let child_domain_size = child_var.domain().size().ok_or_else(|| LutufiError::InternalError {
            message: format!("Variable {:?} has no discrete domain size", child_id),
        })?;
        let child_pos = scope
            .variable_ids()
            .iter()
            .position(|&id| id == child_id)
            .ok_or_else(|| LutufiError::InternalError {
                message: format!("Child variable {:?} not found in its own scope", child_id),
            })?;

        let num_parent_configs = scope.num_entries() / child_domain_size;
        if num_parent_configs == 0 || child_domain_size == 0 {
            return Ok(vec![vec![]; child_domain_size]);
        }
        let mut matrix = vec![vec![0.0; num_parent_configs]; child_domain_size];
        let parent_sizes: Vec<usize> = parents
            .iter()
            .map(|id| {
                model.registry().variable(id)
                    .ok_or_else(|| LutufiError::VariableNotFound {
                        name: format!("{:?}", id),
                        available: format!("{:?}", model.nodes()),
                    })
                    .and_then(|v| v.domain().size().ok_or_else(|| LutufiError::InternalError {
                        message: format!("Parent variable {:?} has no discrete domain size", id),
                    }))
            })
            .collect::<LutufiResult<Vec<_>>>()?;

        for i in 0..scope.num_entries() {
            let sorted_indices = crate::core::factor::multi_index_from_flat(i, scope.sizes());
            let child_state = sorted_indices[child_pos];
            let parent_config = crate::core::factor::project_indices(
                &sorted_indices,
                scope.variable_ids(),
                &parents,
                &parent_sizes,
            );
            matrix[child_state][parent_config] = counts[i];
        }

        Ok(matrix)
    }

    /// Normalize matrix with specified smoothing method.
    fn normalize_matrix(
        matrix: &mut Vec<Vec<f64>>,
        smoothing: SmoothingMethod,
        alpha: f64,
        num_total_configs: usize,
    ) {
        let child_domain_size = matrix.len();
        if child_domain_size == 0 { return; }
        let num_parent_configs = matrix[0].len();

        match smoothing {
            SmoothingMethod::Laplace => {
                for j in 0..num_parent_configs {
                    let mut col_sum = 0.0;
                    for i in 0..child_domain_size {
                        col_sum += matrix[i][j] + alpha;
                    }
                    for i in 0..child_domain_size {
                        matrix[i][j] = (matrix[i][j] + alpha) / col_sum;
                    }
                }
            }
            SmoothingMethod::Bayesian => {
                let pseudo_count = alpha / (num_total_configs as f64);
                for j in 0..num_parent_configs {
                    let mut col_sum = 0.0;
                    for i in 0..child_domain_size {
                        col_sum += matrix[i][j] + pseudo_count;
                    }
                    for i in 0..child_domain_size {
                        matrix[i][j] = (matrix[i][j] + pseudo_count) / col_sum;
                    }
                }
            }
        }
    }
}

/// Unified parameter learning engine.
pub struct ParameterLearner;

fn get_var<'a>(model: &'a BayesianNetwork, id: &VariableId) -> LutufiResult<&'a crate::core::variable::Variable> {
    model.registry().variable(id).ok_or_else(|| LutufiError::VariableNotFound {
        name: format!("{:?}", id),
        available: format!("{:?}", model.nodes()),
    })
}

impl ParameterLearner {
    /// Estimate parameters for a single node.
    pub fn estimate_node(
        model: &mut BayesianNetwork,
        node_name: &str,
        data: &[HashMap<String, String>],
        options: ParameterLearningOptions,
    ) -> LutufiResult<()> {
        let (counts, scope, child_id) = DataCollector::collect_counts(model, node_name, data)?;
        let mut matrix = DataCollector::build_matrix(model, child_id, &counts, &scope)?;
        DataCollector::normalize_matrix(&mut matrix, options.smoothing, options.alpha, scope.num_entries());

        let child_var = get_var(model, &child_id)?;
        let parents = model.graph.parents(&child_id);
        let parent_vars: Vec<&crate::core::variable::Variable> = parents
            .iter()
            .map(|id| get_var(model, id))
            .collect::<LutufiResult<Vec<_>>>()?;

        let cpd = crate::core::factor::ConditionalProbabilityTable::from_values(child_var, &parent_vars, matrix)?;
        model.set_cpd(node_name, cpd)?;
        Ok(())
    }

    /// Estimate parameters for all nodes.
    pub fn estimate_all(
        model: &mut BayesianNetwork,
        data: &[HashMap<String, String>],
        options: ParameterLearningOptions,
    ) -> LutufiResult<()> {
        let is_complete = data.iter().all(|row| row.len() == model.nodes().len());
        if is_complete {
            let node_names: Vec<String> = model.nodes().iter().map(|&s| s.to_string()).collect();
            for node_name in node_names {
                Self::estimate_node(model, &node_name, data, options.clone())?;
            }
            Ok(())
        } else {
            Self::estimate_em(model, data, options)
        }
    }

    fn build_scope(model: &BayesianNetwork, child_id: &VariableId) -> LutufiResult<crate::core::factor::Scope> {
        let child_var = get_var(model, child_id)?;
        let parents = model.graph.parents(child_id);
        let mut scope_vars: Vec<&crate::core::variable::Variable> = parents
            .iter()
            .map(|id| get_var(model, id))
            .collect::<LutufiResult<Vec<_>>>()?;
        scope_vars.push(child_var);
        Ok(crate::core::factor::Scope::new(scope_vars))
    }

    /// Estimate parameters using the EM algorithm for incomplete data.
    pub fn estimate_em(
        model: &mut BayesianNetwork,
        data: &[HashMap<String, String>],
        options: ParameterLearningOptions,
    ) -> LutufiResult<()> {
        let mut last_log_likelihood = f64::NEG_INFINITY;
        let nodes = model.nodes().iter().map(|&s| s.to_string()).collect::<Vec<_>>();

        for node in &nodes {
            if model.cpd(node).is_err() {
                let var = model.variable(node)?;
                let child_id = var.id();
                let parents = model.graph.parents(&child_id);
                let child_var = get_var(model, &child_id)?;
                let mut scope_vars: Vec<&crate::core::variable::Variable> = parents
                    .iter()
                    .map(|id| get_var(model, id))
                    .collect::<LutufiResult<Vec<_>>>()?;
                scope_vars.push(child_var);
                let scope = crate::core::factor::Scope::new(scope_vars);
                let domain_size = var.domain().size().ok_or_else(|| LutufiError::InternalError {
                    message: format!("Variable '{}' has no discrete domain size", node),
                })?;
                let uniform_val = 1.0 / (domain_size as f64);
                let values = vec![uniform_val; scope.num_entries()];
                let factor = crate::core::factor::TabularFactor::from_values(scope, values)?;
                let cpd = crate::core::factor::ConditionalProbabilityTable::from_factor(child_id, parents, factor)?;
                model.set_cpd(node, cpd)?;
            }
        }

        for _iteration in 0..options.max_iterations {
            let mut expected_counts: HashMap<String, Vec<f64>> = HashMap::new();
            let mut total_log_likelihood = 0.0;

            for row in data {
                let mut evidence = crate::core::assignment::Assignment::new();
                for (name, val) in row {
                    if let Ok(id) = model.id_of(name) {
                        let var = get_var(model, &id)?;
                        let idx = var.domain().index_of(val).ok_or_else(|| LutufiError::ValueNotInDomain {
                            value: val.to_string(),
                            variable: name.clone(),
                            valid_values: format!("{:?}", var.domain()),
                        })?;
                        evidence.set_discrete(id, idx)?;
                    }
                }

                for node_name in &nodes {
                    let child_id = model.id_of(node_name)?;
                    let parents = model.graph.parents(&child_id);
                    let mut vars_to_query = vec![node_name.as_str()];
                    for &p_id in &parents {
                        let p_var = get_var(model, &p_id)?;
                        vars_to_query.push(p_var.name());
                    }

                    let all_observed = vars_to_query.iter().all(|&name| row.contains_key(name));
                    let expected_scope = Self::build_scope(model, &child_id)?;
                    let node_expected_counts = expected_counts.entry(node_name.clone()).or_insert_with(|| {
                        vec![0.0; expected_scope.num_entries()]
                    });

                    if all_observed {
                        let mut assignment = crate::core::assignment::Assignment::new();
                        for &name in &vars_to_query {
                            let val = row.get(name).ok_or_else(|| LutufiError::InternalError {
                                message: format!("Expected value for '{}' in all-observed row", name),
                            })?;
                            let id = model.id_of(name)?;
                            let var = get_var(model, &id)?;
                            let idx = var.domain().index_of(val).ok_or_else(|| LutufiError::ValueNotInDomain {
                                value: val.to_string(),
                                variable: name.to_string(),
                                valid_values: format!("{:?}", var.domain()),
                            })?;
                            assignment.set_discrete(id, idx)?;
                        }

                        let mut flat_idx = 0;
                        let mut stride = 1;
                        for i in (0..expected_scope.len()).rev() {
                            let var_id = expected_scope.variable_ids()[i];
                            let val_idx = assignment.get_discrete(&var_id)?;
                            flat_idx += val_idx * stride;
                            stride *= expected_scope.sizes()[i];
                        }
                        node_expected_counts[flat_idx] += 1.0;
                    } else {
                        let result = model.query(&vars_to_query, &evidence, crate::core::inference::Algorithm::Auto)?;
                        if let Some(factor) = result.joint_factor {
                            for i in 0..expected_scope.num_entries() {
                                let assignment = expected_scope.assignment_from_flat(i)?;
                                let mut consistent = true;
                                for (ev_id, ev_val) in evidence.iter() {
                                    if expected_scope.contains(&ev_id) {
                                        let ev_var = get_var(model, &ev_id)?;
                                        let val_idx = ev_var.domain().index_of(ev_val).ok_or_else(|| {
                                            LutufiError::ValueNotInDomain {
                                                value: ev_val.to_string(),
                                                variable: format!("{:?}", ev_id),
                                                valid_values: format!("{:?}", ev_var.domain()),
                                            }
                                        })?;
                                        if assignment.get_discrete(&ev_id)? != val_idx {
                                            consistent = false;
                                            break;
                                        }
                                    }
                                }

                                if consistent {
                                    node_expected_counts[i] += factor.log_value_at_assignment(&assignment).unwrap_or(f64::NEG_INFINITY).exp();
                                }
                            }
                        }
                    }
                }
                let result = model.query(&[], &evidence, crate::core::inference::Algorithm::Auto)?;
                total_log_likelihood += result.log_z;
            }

            for (node_name, counts) in expected_counts {
                let child_id = model.id_of(&node_name)?;
                let child_var = get_var(model, &child_id)?;
                let parents = model.graph.parents(&child_id);
                let scope = Self::build_scope(model, &child_id)?;

                let mut matrix = DataCollector::build_matrix(model, child_id, &counts, &scope)?;
                DataCollector::normalize_matrix(&mut matrix, options.smoothing, options.alpha, counts.len());

                let parent_vars: Vec<&crate::core::variable::Variable> = parents
                    .iter()
                    .map(|id| get_var(model, id))
                    .collect::<LutufiResult<Vec<_>>>()?;
                let cpd = crate::core::factor::ConditionalProbabilityTable::from_values(child_var, &parent_vars, matrix)?;
                model.set_cpd(&node_name, cpd)?;
            }

            if (total_log_likelihood - last_log_likelihood).abs() < options.tolerance { break; }
            if total_log_likelihood < last_log_likelihood && (total_log_likelihood - last_log_likelihood).abs() > 1e-6 {
                return Err(LutufiError::InternalError { message: format!("EM likelihood decrease at iter {}", _iteration) });
            }
            last_log_likelihood = total_log_likelihood;
        }
        Ok(())
    }
}

/// Legacy parameter estimation engine for backward compatibility.
pub struct ParameterEstimator {
    options: LegacyParameterLearningOptions,
}

impl ParameterEstimator {
    /// Create a new legacy ParameterEstimator.
    pub fn new(options: LegacyParameterLearningOptions) -> Self {
        Self { options }
    }

    /// Fit parameters to data using the refactored engine.
    pub fn fit(&self, model: &mut BayesianNetwork, data: &[HashMap<String, String>]) -> LutufiResult<()> {
        let smoothing = match self.options.method {
            ParameterLearningMethod::MLE => SmoothingMethod::Laplace,
            ParameterLearningMethod::Bayesian => SmoothingMethod::Bayesian,
        };

        let refactored_options = ParameterLearningOptions {
            smoothing,
            alpha: self.options.alpha,
            max_iterations: self.options.max_iterations,
            tolerance: self.options.tolerance,
        };

        ParameterLearner::estimate_all(model, data, refactored_options)
    }
}
