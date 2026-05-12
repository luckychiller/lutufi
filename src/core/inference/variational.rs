use std::collections::HashMap;
use rand::prelude::*;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult, LutufiWarning},
    factor::{TabularFactor, Scope, multi_index_from_flat},
    models::factor_graph::FactorGraph,
    variable::{VariableId},
};

/// Options for Variational Inference.
#[derive(Debug, Clone)]
pub struct VariationalOptions {
    /// Maximum number of iterations for CAVI.
    pub max_iterations: usize,
    /// Convergence threshold for ELBO change.
    pub tolerance: f64,
    /// Number of random restarts.
    pub n_restarts: usize,
    /// Optional random seed.
    pub seed: Option<u64>,
}

impl Default for VariationalOptions {
    fn default() -> Self {
        VariationalOptions {
            max_iterations: 1000,
            tolerance: 1e-6,
            n_restarts: 5,
            seed: None,
        }
    }
}

/// Result of Variational Inference.
#[derive(Debug, Clone)]
pub struct VariationalResult {
    /// Approximate marginal distributions Q_i(X_i).
    pub marginals: HashMap<VariableId, TabularFactor>,
    /// Final Evidence Lower Bound (ELBO).
    pub elbo: f64,
    /// Whether the ELBO converged.
    pub converged: bool,
    /// ELBO value at each iteration (for the best restart).
    pub elbo_history: Vec<f64>,
    /// Warnings generated during inference.
    pub warnings: Vec<LutufiWarning>,
}

/// Mean Field Variational Inference engine.
pub struct VariationalEngine {
    graph: FactorGraph,
    options: VariationalOptions,
}

impl VariationalEngine {
    /// Create a new VariationalEngine.
    pub fn new(graph: FactorGraph, options: VariationalOptions) -> Self {
        VariationalEngine { graph, options }
    }

    /// Run Mean Field Variational Inference using CAVI.
    pub fn run(&self) -> LutufiResult<VariationalResult> {
        self.run_with_evidence(&Assignment::new())
    }

    /// Run Mean Field Variational Inference using CAVI with evidence.
    pub fn run_with_evidence(&self, evidence: &Assignment) -> LutufiResult<VariationalResult> {
        let mut best_result: Option<VariationalResult> = None;
        let mut rng = match self.options.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        let reduced_factors: Vec<TabularFactor> = self.graph.factors.iter()
            .map(|f| f.reduce(evidence))
            .collect::<LutufiResult<Vec<_>>>()?;

        for _ in 0..self.options.n_restarts {
            let result = self.run_cavi(&reduced_factors, evidence, &mut rng)?;
            if best_result.is_none() || result.elbo > best_result.as_ref().as_ref().map(|r| r.elbo).unwrap_or(f64::NEG_INFINITY) {
                best_result = Some(result);
            }
        }

        best_result.ok_or_else(|| LutufiError::InternalError { 
            message: "Variational inference failed to produce any result".to_string() 
        })
    }

    fn run_cavi(
        &self, 
        factors: &[TabularFactor], 
        evidence: &Assignment, 
        rng: &mut StdRng
    ) -> LutufiResult<VariationalResult> {
        let mut qs: HashMap<VariableId, TabularFactor> = HashMap::new();
        let mut elbo_history = Vec::new();
        let mut warnings = Vec::new();

        // 1. Initialize Q distributions randomly and fix evidence variables deterministically.
        for (&id, var) in &self.graph.variables {
            let size = var.domain().size().ok_or_else(|| LutufiError::InternalError { 
                message: "Variational inference only supports discrete variables for now".to_string() 
            })?;

            if evidence.contains(&id) {
                let observed = evidence.get_discrete(&id)?;
                let mut log_values = vec![f64::NEG_INFINITY; size];
                if observed < size {
                    log_values[observed] = 0.0;
                }
                qs.insert(id, TabularFactor::from_log_values(Scope::new(vec![var]), log_values)?);
                continue;
            }

            let mut values: Vec<f64> = (0..size).map(|_| rng.gen::<f64>()).collect();
            let sum: f64 = values.iter().sum();
            for v in values.iter_mut() { *v /= sum; }

            qs.insert(id, TabularFactor::from_values(Scope::new(vec![var]), values)?);
        }

        let mut prev_elbo = f64::NEG_INFINITY;
        let mut converged = false;

        for iteration in 0..self.options.max_iterations {
            // 2. Coordinate Ascent Updates
            let var_ids: Vec<VariableId> = qs.keys().copied().collect();
            for &var_id in &var_ids {
                let new_q = self.update_q(var_id, factors, &qs)?;
                qs.insert(var_id, new_q);
            }

            // 3. Compute ELBO
            let current_elbo = self.compute_elbo(factors, &qs)?;
            elbo_history.push(current_elbo);

            if iteration > 0 && (current_elbo - prev_elbo).abs() < self.options.tolerance {
                converged = true;
                break;
            }
            
            if iteration > 0 && current_elbo < prev_elbo - 1e-10 {
                warnings.push(LutufiWarning::InternalWarning {
                    message: format!("ELBO decreased at iteration {}: {} -> {}", iteration, prev_elbo, current_elbo),
                });
            }

            prev_elbo = current_elbo;
        }

        if !converged && self.options.max_iterations > 0 {
            warnings.push(LutufiWarning::ConvergenceWarning {
                algorithm: "Mean Field Variational Inference".to_string(),
                max_iterations: self.options.max_iterations,
                residual: (elbo_history.last().copied().unwrap_or(0.0) - prev_elbo).abs(),
            });
        }

        Ok(VariationalResult {
            marginals: qs,
            elbo: prev_elbo,
            converged,
            elbo_history,
            warnings,
        })
    }

    fn update_q(
        &self, 
        var_id: VariableId, 
        factors: &[TabularFactor], 
        qs: &HashMap<VariableId, TabularFactor>
    ) -> LutufiResult<TabularFactor> {
        if let Some(current_q) = qs.get(&var_id) {
            let size = current_q.scope().num_entries();
            let nonzero = (0..size)
                .filter(|&i| current_q.value_at(i) > 1e-12)
                .count();
            if nonzero == 1 {
                return Ok(current_q.clone());
            }
        }

        let var = self.graph.variables.get(&var_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: var_id.to_string(),
            available: "".to_string()
        })?;
        let size = var.domain().size().ok_or_else(|| LutufiError::InternalError { 
            message: "Variable domain size not available".to_string() 
        })?;
        let mut new_log_q = vec![0.0; size];

        if let Some(factor_indices) = self.graph.var_to_factors.get(&var_id) {
            for &f_idx in factor_indices {
                let factor = &factors[f_idx];
                for x_i in 0..size {
                    new_log_q[x_i] += self.compute_expectation(factor, var_id, x_i, qs)?;
                }
            }
        }

        // Normalize in log-space (softmax)
        let mut max_log = f64::NEG_INFINITY;
        for &v in &new_log_q { if v > max_log { max_log = v; } }
        let log_sum = if max_log.is_infinite() && max_log.is_sign_negative() { 
            0.0 
        } else {
            max_log + new_log_q.iter().map(|&v| (v - max_log).exp()).sum::<f64>().ln()
        };
        for v in &mut new_log_q { *v -= log_sum; }

        TabularFactor::from_log_values(Scope::new(vec![var]), new_log_q)
    }

    fn compute_expectation(
        &self, 
        factor: &TabularFactor, 
        var_id: VariableId, 
        var_val: usize,
        qs: &HashMap<VariableId, TabularFactor>
    ) -> LutufiResult<f64> {
        let scope = factor.scope();
        let var_pos = scope.variable_ids().iter().position(|&id| id == var_id).ok_or_else(|| {
            LutufiError::InternalError { message: "Variable not in factor scope".to_string() }
        })?;
        
        let mut expectation = 0.0;
        let num_entries = scope.num_entries();
        
        for i in 0..num_entries {
            let multi_idx = multi_index_from_flat(i, scope.sizes());
            if multi_idx[var_pos] != var_val { continue; }
            
            let log_f = factor.log_value_at(i);
            if log_f.is_infinite() && log_f.is_sign_negative() { continue; }
            
            let mut q_prob = 1.0;
            for (j, &other_id) in scope.variable_ids().iter().enumerate() {
                if other_id == var_id { continue; }
                if let Some(q_var) = qs.get(&other_id) {
                    q_prob *= q_var.value_at(multi_idx[j]);
                }
            }
            
            expectation += q_prob * log_f;
        }
        
        Ok(expectation)
    }

    fn compute_elbo(
        &self, 
        factors: &[TabularFactor], 
        qs: &HashMap<VariableId, TabularFactor>
    ) -> LutufiResult<f64> {
        let mut energy = 0.0; 
        let mut entropy = 0.0; 

        for factor in factors {
            energy += self.compute_factor_expectation(factor, qs)?;
        }

        for q in qs.values() {
            let n = q.scope().num_entries();
            for i in 0..n {
                let log_q = q.log_value_at(i);
                if !log_q.is_infinite() || !log_q.is_sign_negative() {
                    entropy -= q.value_at(i) * log_q;
                }
            }
        }

        Ok(energy + entropy)
    }

    fn compute_factor_expectation(
        &self, 
        factor: &TabularFactor, 
        qs: &HashMap<VariableId, TabularFactor>
    ) -> LutufiResult<f64> {
        let scope = factor.scope();
        let mut expectation = 0.0;
        let n = scope.num_entries();
        
        for i in 0..n {
            let log_f = factor.log_value_at(i);
            if log_f.is_infinite() && log_f.is_sign_negative() { continue; }
            
            let multi_idx = multi_index_from_flat(i, scope.sizes());
            let mut q_prob = 1.0;
            for (j, &var_id) in scope.variable_ids().iter().enumerate() {
                if let Some(q_var) = qs.get(&var_id) {
                    q_prob *= q_var.value_at(multi_idx[j]);
                }
            }
            expectation += q_prob * log_f;
        }
        Ok(expectation)
    }
}
