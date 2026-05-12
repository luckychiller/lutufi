use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rayon::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::slice::ParallelSlice;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    models::factor_graph::FactorGraph,
    variable::VariableId,
    inference::lbp::{LBPOptions, LBPResult, ConvergenceMonitor},
};

/// Parallel Loopy Belief Propagation engine.
///
/// Uses rayon for parallel message updates across the factor graph.
/// Messages on different edges are independent and can be updated
/// concurrently in the asynchronous schedule.
pub struct ParallelLBPEngine {
    graph: Arc<FactorGraph>,
    options: LBPOptions,
    var_to_factor_msgs: Arc<RwLock<HashMap<(VariableId, usize), TabularFactor>>>,
    factor_to_var_msgs: Arc<RwLock<HashMap<(usize, VariableId), TabularFactor>>>,
}

impl ParallelLBPEngine {
    pub fn new(graph: FactorGraph, options: LBPOptions) -> Self {
        ParallelLBPEngine {
            graph: Arc::new(graph),
            options,
            var_to_factor_msgs: Arc::new(RwLock::new(HashMap::new())),
            factor_to_var_msgs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init_messages(&self) -> LutufiResult<()> {
        let graph = self.graph.as_ref();
        for (&var_id, var) in &graph.variables {
            let factor_indices = graph.var_to_factors.get(&var_id)
                .ok_or_else(|| LutufiError::InternalError {
                    message: format!("Variable {} not mapped in FactorGraph", var_id)
                })?;
            let scope = Scope::new(vec![var]);
            for &f_idx in factor_indices {
                let msg = TabularFactor::identity(scope.clone())?;
                {
                    let mut v2f = self.var_to_factor_msgs.write().map_err(|_| {
                        LutufiError::InternalError { message: "Lock poisoned during init".to_string() }
                    })?;
                    v2f.insert((var_id, f_idx), msg.clone());
                }
                {
                    let mut f2v = self.factor_to_var_msgs.write().map_err(|_| {
                        LutufiError::InternalError { message: "Lock poisoned during init".to_string() }
                    })?;
                    f2v.insert((f_idx, var_id), msg);
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self, evidence: &Assignment) -> LutufiResult<LBPResult> {
        let graph = self.graph.as_ref();

        let reduced_factors: Vec<TabularFactor> = graph.factors.par_iter()
            .map(|f: &TabularFactor| f.reduce(evidence))
            .collect::<LutufiResult<Vec<_>>>()?;

        self.init_messages()?;

        let mut iteration = 0;
        let mut max_residual = f64::INFINITY;
        let monitor = ConvergenceMonitor::new(self.options.tolerance, self.options.max_iterations);

        while !monitor.is_converged(max_residual, iteration) {
            iteration += 1;
            max_residual = 0.0;
            let var_ids: Vec<VariableId> = graph.variables.keys().copied().collect();

            let results: Vec<LutufiResult<f64>> = var_ids.par_iter().map(|&var_id| {
                let factor_indices = graph.var_to_factors.get(&var_id)
                    .ok_or_else(|| LutufiError::InternalError {
                        message: "Variable not in factor mapping".to_string()
                    })?.clone();

                let mut local_max_residual = 0.0_f64;

                for f_idx in factor_indices {
                    let new_v2f = self.compute_v2f_message(var_id, f_idx)?;
                    let old_v2f = {
                        let msgs = self.var_to_factor_msgs.read().map_err(|_| {
                            LutufiError::InternalError { message: "Lock poisoned".to_string() }
                        })?;
                        msgs.get(&(var_id, f_idx))
                            .ok_or_else(|| LutufiError::InternalError {
                                message: "V2F message missing".to_string()
                            })?
                            .clone()
                    };
                    local_max_residual = local_max_residual.max(
                        self.compute_residual(&old_v2f, &new_v2f)
                    );
                    let damped_v2f = self.apply_damping(&old_v2f, &new_v2f)?;
                    {
                        let mut msgs = self.var_to_factor_msgs.write().map_err(|_| {
                            LutufiError::InternalError { message: "Lock poisoned".to_string() }
                        })?;
                        msgs.insert((var_id, f_idx), damped_v2f);
                    }

                    let new_f2v = self.compute_f2v_message(f_idx, var_id, &reduced_factors[f_idx])?;
                    let old_f2v = {
                        let msgs = self.factor_to_var_msgs.read().map_err(|_| {
                            LutufiError::InternalError { message: "Lock poisoned".to_string() }
                        })?;
                        msgs.get(&(f_idx, var_id))
                            .ok_or_else(|| LutufiError::InternalError {
                                message: "F2V message missing".to_string()
                            })?
                            .clone()
                    };
                    local_max_residual = local_max_residual.max(
                        self.compute_residual(&old_f2v, &new_f2v)
                    );
                    let damped_f2v = self.apply_damping(&old_f2v, &new_f2v)?;
                    {
                        let mut msgs = self.factor_to_var_msgs.write().map_err(|_| {
                            LutufiError::InternalError { message: "Lock poisoned".to_string() }
                        })?;
                        msgs.insert((f_idx, var_id), damped_f2v);
                    }
                }

                Ok(local_max_residual)
            }).collect();

            for result in &results {
                match result {
                    Ok(residual) => max_residual = max_residual.max(*residual),
                    Err(e) => return Err(LutufiError::InternalError {
                        message: format!("Parallel LBP error: {}", e)
                    }),
                }
            }
        }

        let mut beliefs = HashMap::new();
        for (&var_id, var) in &graph.variables {
            let factor_indices = graph.var_to_factors.get(&var_id)
                .ok_or_else(|| LutufiError::InternalError {
                    message: "Variable missing from factor mapping".to_string()
                })?;
            let mut belief = TabularFactor::identity(Scope::new(vec![var]))?;
            let msgs = self.factor_to_var_msgs.read().map_err(|_| {
                LutufiError::InternalError { message: "Lock poisoned".to_string() }
            })?;
            for &f_idx in factor_indices {
                if let Some(msg) = msgs.get(&(f_idx, var_id)) {
                    belief = belief.multiply(msg)?;
                }
            }
            belief.normalize();
            beliefs.insert(var_id, belief);
        }

        let converged = max_residual < self.options.tolerance;
        let mut warnings = Vec::new();
        if !converged {
            warnings.push(crate::core::error::LutufiWarning::ConvergenceWarning {
                algorithm: "Parallel Loopy Belief Propagation".to_string(),
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
        let graph = self.graph.as_ref();
        let factor_indices = graph.var_to_factors.get(&var_id)
            .ok_or_else(|| LutufiError::InternalError {
                message: "Variable missing from factor mapping".to_string()
            })?;
        let var = graph.variables.get(&var_id)
            .ok_or_else(|| LutufiError::VariableNotFound {
                name: var_id.to_string(),
                available: "".to_string()
            })?;
        let mut msg = TabularFactor::identity(Scope::new(vec![var]))?;
        let msgs = self.factor_to_var_msgs.read().map_err(|_| {
            LutufiError::InternalError { message: "Lock poisoned".to_string() }
        })?;
        for &other_f_idx in factor_indices {
            if other_f_idx == f_idx { continue; }
            if let Some(other_msg) = msgs.get(&(other_f_idx, var_id)) {
                msg = msg.multiply(other_msg)?;
            }
        }
        msg.normalize();
        Ok(msg)
    }

    fn compute_f2v_message(&self, f_idx: usize, var_id: VariableId, factor: &TabularFactor) -> LutufiResult<TabularFactor> {
        let mut product = factor.clone();
        let msgs = self.var_to_factor_msgs.read().map_err(|_| {
            LutufiError::InternalError { message: "Lock poisoned".to_string() }
        })?;
        for &other_var_id in factor.scope().variable_ids() {
            if other_var_id == var_id { continue; }
            if let Some(msg) = msgs.get(&(other_var_id, f_idx)) {
                product = product.multiply(msg)?;
            }
        }
        let vars_to_sum_out: Vec<VariableId> = product.scope().variable_ids()
            .iter().filter(|&&id| id != var_id).copied().collect();
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

/// Parallel factor product computation using rayon.
pub fn parallel_factor_product(factors: &[TabularFactor]) -> LutufiResult<TabularFactor> {
    if factors.is_empty() {
        return TabularFactor::identity(Scope::from_ids_and_sizes(vec![], vec![]));
    }
    if factors.len() == 1 {
        return Ok(factors[0].clone());
    }

    let chunk_size = (factors.len() + rayon::current_num_threads() - 1) / rayon::current_num_threads();
    let products: Vec<LutufiResult<TabularFactor>> = factors.par_chunks(chunk_size)
        .map(|chunk: &[TabularFactor]| {
            let mut p_iter = chunk.iter();
            let first = p_iter.next().ok_or_else(|| LutufiError::InternalError {
                message: "Empty chunk in parallel factor product".to_string()
            })?;
            let mut product = first.clone();
            for factor in p_iter {
                product = product.multiply(factor)?;
            }
            Ok(product)
        })
        .collect();

    let mut iter = products.into_iter();
    let first = iter.next()
        .ok_or_else(|| LutufiError::InternalError {
            message: "No products from parallel factor computation".to_string()
        })??;
    let mut result = first;
    for product in iter {
        result = result.multiply(&product?)?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_factor_product_empty() {
        let result = parallel_factor_product(&[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parallel_factor_product_single() {
        let v = crate::core::variable::Variable::new("X", crate::core::domain::Domain::binary());
        let scope = Scope::new(vec![&v]);
        let factor = TabularFactor::from_values(scope, vec![0.5, 0.5]).unwrap();
        let result = parallel_factor_product(&[factor]).unwrap();
        assert_eq!(result.scope().num_entries(), 2);
    }
}
