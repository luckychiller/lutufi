use std::collections::HashMap;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult, LutufiWarning},
    factor::{Scope, TabularFactor},
    models::factor_graph::FactorGraph,
    variable::VariableId,
};

/// Options for Loopy Belief Propagation.
#[derive(Debug, Clone)]
pub struct LBPOptions {
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Convergence threshold (max message change).
    pub tolerance: f64,
    /// Damping factor (0.0 means no damping, 1.0 means no updates).
    pub damping: f64,
}

impl Default for LBPOptions {
    fn default() -> Self {
        LBPOptions {
            max_iterations: 1000,
            tolerance: 1e-6,
            damping: 0.5,
        }
    }
}

/// Result of Loopy Belief Propagation.
#[derive(Debug, Clone)]
pub struct LBPResult {
    /// Whether the algorithm converged.
    pub converged: bool,
    /// Number of iterations performed.
    pub iterations: usize,
    /// The final residual (max message change).
    pub residual: f64,
    /// Marginal probabilities for each variable.
    pub beliefs: HashMap<VariableId, TabularFactor>,
    /// Warnings generated during inference.
    pub warnings: Vec<LutufiWarning>,
}

/// A monitor for convergence in LBP.
pub struct ConvergenceMonitor {
    tolerance: f64,
    max_iterations: usize,
}

impl ConvergenceMonitor {
    /// Create a new ConvergenceMonitor.
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        ConvergenceMonitor { tolerance, max_iterations }
    }

    /// Check if the algorithm has converged.
    pub fn is_converged(&self, residual: f64, iteration: usize) -> bool {
        residual < self.tolerance || iteration >= self.max_iterations
    }
}

/// Loopy Belief Propagation (LBP) engine.
pub struct LBPEngine {
    graph: FactorGraph,
    options: LBPOptions,
    /// Messages from variable to factor. Key: (VariableId, FactorIndex).
    var_to_factor_msgs: HashMap<(VariableId, usize), TabularFactor>,
    /// Messages from factor to variable. Key: (FactorIndex, VariableId).
    factor_to_var_msgs: HashMap<(usize, VariableId), TabularFactor>,
}

impl LBPEngine {
    /// Create a new LBPEngine.
    pub fn new(graph: FactorGraph, options: LBPOptions) -> Self {
        LBPEngine {
            graph,
            options,
            var_to_factor_msgs: HashMap::new(),
            factor_to_var_msgs: HashMap::new(),
        }
    }

    /// Initialize messages with identity factors (all ones in probability space, zeros in log-space).
    fn init_messages(&mut self) -> LutufiResult<()> {
        for (&var_id, var) in &self.graph.variables {
            let factor_indices = self.graph.var_to_factors.get(&var_id).ok_or_else(|| LutufiError::InternalError {
                message: format!("Variable {} not mapped in FactorGraph", var_id)
            })?;
            let scope = Scope::new(vec![var]);
            
            for &f_idx in factor_indices {
                self.var_to_factor_msgs.insert((var_id, f_idx), TabularFactor::identity(scope.clone())?);
                self.factor_to_var_msgs.insert((f_idx, var_id), TabularFactor::identity(scope.clone())?);
            }
        }
        Ok(())
    }

    /// Run Loopy Belief Propagation.
    pub fn run(&mut self, evidence: &Assignment) -> LutufiResult<LBPResult> {
        // 1. Reduce factors by evidence
        let mut reduced_factors: Vec<TabularFactor> = Vec::with_capacity(self.graph.factors.len());
        for factor in &self.graph.factors {
            reduced_factors.push(factor.reduce(evidence)?);
        }

        // 2. Initialize messages
        self.init_messages()?;

        let mut iteration = 0;
        let mut max_residual = f64::INFINITY;
        let monitor = ConvergenceMonitor::new(self.options.tolerance, self.options.max_iterations);

        while !monitor.is_converged(max_residual, iteration) {
            iteration += 1;
            max_residual = 0.0;

            let var_ids: Vec<VariableId> = self.graph.variables.keys().copied().collect();
            for var_id in var_ids {
                let factor_indices = self.graph.var_to_factors.get(&var_id).ok_or_else(|| LutufiError::InternalError {
                    message: "Variable not in factor mapping".to_string()
                })?.clone();

                for f_idx in factor_indices {
                    // Update Variable to Factor message
                    let new_v2f = self.compute_v2f_message(var_id, f_idx)?;
                    let old_v2f = self.var_to_factor_msgs.get(&(var_id, f_idx)).ok_or_else(|| LutufiError::InternalError {
                        message: "V2F message missing during update".to_string()
                    })?;
                    let residual_v2f = self.compute_residual(old_v2f, &new_v2f);
                    max_residual = max_residual.max(residual_v2f);
                    
                    let damped_v2f = self.apply_damping(old_v2f, &new_v2f)?;
                    self.var_to_factor_msgs.insert((var_id, f_idx), damped_v2f);

                    // Update Factor to Variable message
                    let new_f2v = self.compute_f2v_message(f_idx, var_id, &reduced_factors[f_idx])?;
                    let old_f2v = self.factor_to_var_msgs.get(&(f_idx, var_id)).ok_or_else(|| LutufiError::InternalError {
                        message: "F2V message missing during update".to_string()
                    })?;
                    let residual_f2v = self.compute_residual(old_f2v, &new_f2v);
                    max_residual = max_residual.max(residual_f2v);

                    let damped_f2v = self.apply_damping(old_f2v, &new_f2v)?;
                    self.factor_to_var_msgs.insert((f_idx, var_id), damped_f2v);
                }
            }
        }

        // 3. Compute final beliefs (marginals)
        let mut beliefs = HashMap::new();
        for (&var_id, var) in &self.graph.variables {
            let factor_indices = self.graph.var_to_factors.get(&var_id).ok_or_else(|| LutufiError::InternalError {
                message: "Variable missing from factor mapping".to_string()
            })?;
            let mut belief = TabularFactor::identity(Scope::new(vec![var]))?;
            
            for &f_idx in factor_indices {
                let msg = self.factor_to_var_msgs.get(&(f_idx, var_id)).ok_or_else(|| LutufiError::InternalError {
                    message: "Final belief message missing".to_string()
                })?;
                belief = belief.multiply(msg)?;
            }
            belief.normalize();
            beliefs.insert(var_id, belief);
        }

        let converged = max_residual < self.options.tolerance;
        let mut warnings = Vec::new();
        if !converged {
            warnings.push(LutufiWarning::ConvergenceWarning {
                algorithm: "Loopy Belief Propagation".to_string(),
                max_iterations: self.options.max_iterations,
                residual: max_residual,
            });
        }

        Ok(LBPResult {
            converged,
            iterations: iteration,
            residual: max_residual,
            beliefs,
            warnings,
        })
    }

    fn compute_v2f_message(&self, var_id: VariableId, f_idx: usize) -> LutufiResult<TabularFactor> {
        let factor_indices = self.graph.var_to_factors.get(&var_id).ok_or_else(|| LutufiError::InternalError {
            message: "Variable missing from factor mapping".to_string()
        })?;
        let var = self.graph.variables.get(&var_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: var_id.to_string(),
            available: "".to_string()
        })?;
        let mut msg = TabularFactor::identity(Scope::new(vec![var]))?;

        for &other_f_idx in factor_indices {
            if other_f_idx == f_idx { continue; }
            let other_msg = self.factor_to_var_msgs.get(&(other_f_idx, var_id)).ok_or_else(|| LutufiError::InternalError {
                message: "Message missing for V2F computation".to_string()
            })?;
            msg = msg.multiply(other_msg)?;
        }
        
        msg.normalize();
        Ok(msg)
    }

    fn compute_f2v_message(&self, f_idx: usize, var_id: VariableId, factor: &TabularFactor) -> LutufiResult<TabularFactor> {
        let mut product = factor.clone();
        
        for &other_var_id in factor.scope().variable_ids() {
            if other_var_id == var_id { continue; }
            let msg = self.var_to_factor_msgs.get(&(other_var_id, f_idx)).ok_or_else(|| LutufiError::InternalError {
                message: "Message missing for F2V computation".to_string()
            })?;
            product = product.multiply(msg)?;
        }

        let vars_to_sum_out: Vec<VariableId> = product.scope().variable_ids()
            .iter()
            .filter(|&&id| id != var_id)
            .copied()
            .collect();

        let mut result = product.marginalize(&vars_to_sum_out)?;
        result.normalize();
        Ok(result)
    }

    fn compute_residual(&self, old: &TabularFactor, new: &TabularFactor) -> f64 {
        let mut max_diff = 0.0;
        let n = old.scope().num_entries();
        for i in 0..n {
            let diff = (old.log_value_at(i) - new.log_value_at(i)).abs();
            if diff.is_finite() && diff > max_diff {
                max_diff = diff;
            }
        }
        max_diff
    }

    fn apply_damping(&self, old: &TabularFactor, new: &TabularFactor) -> LutufiResult<TabularFactor> {
        let d = self.options.damping;
        if d.abs() < 1e-12 { return Ok(new.clone()); }
        
        let scope = old.scope().clone();
        let n = scope.num_entries();
        let mut log_values = Vec::with_capacity(n);
        
        for i in 0..n {
            let p_new = new.log_value_at(i).exp();
            let p_old = old.log_value_at(i).exp();
            let p_damped = (1.0 - d) * p_new + d * p_old;
            log_values.push(if p_damped < 1e-300 { f64::NEG_INFINITY } else { p_damped.ln() });
        }
        
        TabularFactor::from_log_values(scope, log_values)
    }
}

/// Perform Loopy Belief Propagation on a factor graph.
pub fn lbp_query(
    graph: FactorGraph,
    variables: &[&str],
    evidence: &Assignment,
    options: LBPOptions,
) -> LutufiResult<HashMap<String, TabularFactor>> {
    let mut engine = LBPEngine::new(graph, options);
    let result = engine.run(evidence)?;
    
    let mut final_results = HashMap::new();
    for &name in variables {
        let var_id = engine.graph.variables.values()
            .find(|v| v.name() == name)
            .map(|v| v.id())
            .ok_or_else(|| LutufiError::VariableNotFound { 
                name: name.to_string(), 
                available: engine.graph.variables.values().map(|v| v.name()).collect::<Vec<_>>().join(", ") 
            })?;
        
        let belief = result.beliefs.get(&var_id).ok_or_else(|| LutufiError::InternalError {
            message: "Result missing belief for variable".to_string()
        })?;
        final_results.insert(name.to_string(), belief.clone());
    }
    
    Ok(final_results)
}
