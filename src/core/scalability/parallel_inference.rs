use std::collections::HashMap;
use std::sync::Arc;
use rayon::prelude::*;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    models::factor_graph::FactorGraph,
    variable::VariableId,
    inference::lbp::{LBPOptions, LBPResult, ConvergenceMonitor},
};

/// One recomputed message: which edge it belongs to, its damped value, and how
/// far it moved from the previous iteration.
type MessageUpdate<K> = LutufiResult<(K, TabularFactor, f64)>;

/// Parallel Loopy Belief Propagation engine.
///
/// # Schedule and determinism
///
/// This engine uses a **synchronous (Jacobi) schedule**: every message in an
/// iteration is computed from the previous iteration's messages, held immutably,
/// and written into a fresh buffer that replaces the old one only once the whole
/// iteration completes.
///
/// That choice is about correctness, not just speed. The earlier implementation
/// took a write lock per message inside the parallel loop, so a thread computing
/// a message could observe a neighbour's message from either the current or the
/// previous iteration depending on thread interleaving. Results therefore varied
/// between runs and between thread counts — incompatible with the reproducibility
/// guarantees Lutufi makes about inference, and a poor foundation for a library
/// whose users publish their numbers. The Jacobi schedule removes the shared
/// mutable state entirely: no locks in the hot loop, and identical output
/// regardless of thread count.
///
/// The trade-off is real: Gauss-Seidel (asynchronous) schedules often converge in
/// fewer iterations. The right way to buy that back is a deterministic graph
/// colouring — updating provably independent sets in a fixed order — not letting
/// the thread scheduler decide which messages are fresh.
pub struct ParallelLBPEngine {
    graph: Arc<FactorGraph>,
    options: LBPOptions,
    var_to_factor_msgs: HashMap<(VariableId, usize), TabularFactor>,
    factor_to_var_msgs: HashMap<(usize, VariableId), TabularFactor>,
}

impl ParallelLBPEngine {
    /// Create a new parallel LBP engine for the given factor graph and options.
    pub fn new(graph: FactorGraph, options: LBPOptions) -> Self {
        ParallelLBPEngine {
            graph: Arc::new(graph),
            options,
            var_to_factor_msgs: HashMap::new(),
            factor_to_var_msgs: HashMap::new(),
        }
    }

    /// Messages from the last [`ParallelLBPEngine::run`], variable to factor.
    pub fn var_to_factor_messages(&self) -> &HashMap<(VariableId, usize), TabularFactor> {
        &self.var_to_factor_msgs
    }

    /// Messages from the last [`ParallelLBPEngine::run`], factor to variable.
    pub fn factor_to_var_messages(&self) -> &HashMap<(usize, VariableId), TabularFactor> {
        &self.factor_to_var_msgs
    }

    /// Build the initial (uniform) message sets.
    #[allow(clippy::type_complexity)]
    fn init_messages(
        &self,
    ) -> LutufiResult<(
        HashMap<(VariableId, usize), TabularFactor>,
        HashMap<(usize, VariableId), TabularFactor>,
    )> {
        let graph = self.graph.as_ref();
        let mut v2f = HashMap::new();
        let mut f2v = HashMap::new();

        for (&var_id, var) in &graph.variables {
            let factor_indices = graph.var_to_factors.get(&var_id)
                .ok_or_else(|| LutufiError::InternalError {
                    message: format!("Variable {} not mapped in FactorGraph", var_id)
                })?;
            let scope = Scope::new(vec![var]);
            for &f_idx in factor_indices {
                let msg = TabularFactor::identity(scope.clone())?;
                v2f.insert((var_id, f_idx), msg.clone());
                f2v.insert((f_idx, var_id), msg);
            }
        }
        Ok((v2f, f2v))
    }

    /// A stable, sorted list of (variable, factor) edges.
    ///
    /// Iterating the `HashMap` directly would give an arbitrary order. A Jacobi
    /// sweep is order-independent in its results, but fixing the order keeps
    /// residual reporting and any future early-exit deterministic as well.
    fn sorted_edges(&self) -> Vec<(VariableId, usize)> {
        let graph = self.graph.as_ref();
        let mut var_ids: Vec<VariableId> = graph.variables.keys().copied().collect();
        var_ids.sort();

        let mut edges = Vec::new();
        for var_id in var_ids {
            if let Some(factor_indices) = graph.var_to_factors.get(&var_id) {
                let mut indices = factor_indices.clone();
                indices.sort_unstable();
                for f_idx in indices {
                    edges.push((var_id, f_idx));
                }
            }
        }
        edges
    }

    /// Run parallel loopy belief propagation with the given evidence.
    pub fn run(&mut self, evidence: &Assignment) -> LutufiResult<LBPResult> {
        let graph = Arc::clone(&self.graph);

        let reduced_factors: Vec<TabularFactor> = graph.factors.par_iter()
            .map(|f: &TabularFactor| f.reduce(evidence))
            .collect::<LutufiResult<Vec<_>>>()?;

        let (mut v2f, mut f2v) = self.init_messages()?;
        let edges = self.sorted_edges();
        let damping = self.options.damping;

        let monitor = ConvergenceMonitor::new(self.options.tolerance, self.options.max_iterations);
        let mut iteration = 0;
        let mut max_residual = f64::INFINITY;

        while !monitor.is_converged(max_residual, iteration) {
            iteration += 1;
            let mut round_residual = 0.0f64;

            // --- Phase 1: variable -> factor, read only from the previous f2v ---
            let updates: Vec<MessageUpdate<(VariableId, usize)>> = edges
                .par_iter()
                .map(|&(var_id, f_idx)| {
                    let fresh = Self::compute_v2f_message(&graph, &f2v, var_id, f_idx)?;
                    let previous = v2f.get(&(var_id, f_idx))
                        .ok_or_else(|| LutufiError::InternalError {
                            message: "V2F message missing".to_string()
                        })?;
                    let residual = Self::residual(previous, &fresh);
                    let damped = Self::apply_damping(damping, previous, &fresh)?;
                    Ok(((var_id, f_idx), damped, residual))
                })
                .collect();

            let mut next_v2f = HashMap::with_capacity(v2f.len());
            for update in updates {
                let (key, msg, residual) = update?;
                round_residual = round_residual.max(residual);
                next_v2f.insert(key, msg);
            }
            v2f = next_v2f;

            // --- Phase 2: factor -> variable, read only from the new v2f ---
            let updates: Vec<MessageUpdate<(usize, VariableId)>> = edges
                .par_iter()
                .map(|&(var_id, f_idx)| {
                    let fresh = Self::compute_f2v_message(
                        &v2f, f_idx, var_id, &reduced_factors[f_idx])?;
                    let previous = f2v.get(&(f_idx, var_id))
                        .ok_or_else(|| LutufiError::InternalError {
                            message: "F2V message missing".to_string()
                        })?;
                    let residual = Self::residual(previous, &fresh);
                    let damped = Self::apply_damping(damping, previous, &fresh)?;
                    Ok(((f_idx, var_id), damped, residual))
                })
                .collect();

            let mut next_f2v = HashMap::with_capacity(f2v.len());
            for update in updates {
                let (key, msg, residual) = update?;
                round_residual = round_residual.max(residual);
                next_f2v.insert(key, msg);
            }
            f2v = next_f2v;

            max_residual = round_residual;
        }

        // --- Beliefs ---
        let mut beliefs = HashMap::new();
        let mut var_ids: Vec<VariableId> = graph.variables.keys().copied().collect();
        var_ids.sort();

        for var_id in var_ids {
            let var = graph.variables.get(&var_id)
                .ok_or_else(|| LutufiError::VariableNotFound {
                    name: var_id.to_string(),
                    available: String::new(),
                })?;
            let factor_indices = graph.var_to_factors.get(&var_id)
                .ok_or_else(|| LutufiError::InternalError {
                    message: "Variable missing from factor mapping".to_string()
                })?;
            let mut belief = TabularFactor::identity(Scope::new(vec![var]))?;
            let mut indices = factor_indices.clone();
            indices.sort_unstable();
            for f_idx in indices {
                if let Some(msg) = f2v.get(&(f_idx, var_id)) {
                    belief = belief.multiply(msg)?;
                }
            }
            belief.normalize();
            beliefs.insert(var_id, belief);
        }

        self.var_to_factor_msgs = v2f;
        self.factor_to_var_msgs = f2v;

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

    /// Message from `var_id` to factor `f_idx`: the product of everything the
    /// variable hears from its *other* factors.
    fn compute_v2f_message(
        graph: &FactorGraph,
        f2v: &HashMap<(usize, VariableId), TabularFactor>,
        var_id: VariableId,
        f_idx: usize,
    ) -> LutufiResult<TabularFactor> {
        let factor_indices = graph.var_to_factors.get(&var_id)
            .ok_or_else(|| LutufiError::InternalError {
                message: "Variable missing from factor mapping".to_string()
            })?;
        let var = graph.variables.get(&var_id)
            .ok_or_else(|| LutufiError::VariableNotFound {
                name: var_id.to_string(),
                available: String::new(),
            })?;
        let mut msg = TabularFactor::identity(Scope::new(vec![var]))?;
        for &other_f_idx in factor_indices {
            if other_f_idx == f_idx { continue; }
            if let Some(other_msg) = f2v.get(&(other_f_idx, var_id)) {
                msg = msg.multiply(other_msg)?;
            }
        }
        msg.normalize();
        Ok(msg)
    }

    /// Message from factor `f_idx` to `var_id`: the factor times everything it
    /// hears from its *other* variables, with those variables summed out.
    fn compute_f2v_message(
        v2f: &HashMap<(VariableId, usize), TabularFactor>,
        f_idx: usize,
        var_id: VariableId,
        factor: &TabularFactor,
    ) -> LutufiResult<TabularFactor> {
        let mut product = factor.clone();
        for &other_var_id in factor.scope().variable_ids() {
            if other_var_id == var_id { continue; }
            if let Some(msg) = v2f.get(&(other_var_id, f_idx)) {
                product = product.multiply(msg)?;
            }
        }
        let vars_to_sum_out: Vec<VariableId> = product.scope().variable_ids()
            .iter().filter(|&&id| id != var_id).copied().collect();
        let mut result = product.marginalize(&vars_to_sum_out)?;
        result.normalize();
        Ok(result)
    }

    fn residual(old: &TabularFactor, new: &TabularFactor) -> f64 {
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

    fn apply_damping(
        damping: f64,
        old: &TabularFactor,
        new: &TabularFactor,
    ) -> LutufiResult<TabularFactor> {
        if damping.abs() < 1e-12 { return Ok(new.clone()); }
        let scope = old.scope().clone();
        let n = scope.num_entries();
        let mut log_values = Vec::with_capacity(n);
        for i in 0..n {
            let p_new = new.log_value_at(i).exp();
            let p_old = old.log_value_at(i).exp();
            let p_damped = (1.0 - damping) * p_new + damping * p_old;
            log_values.push(if p_damped < 1e-300 { f64::NEG_INFINITY } else { p_damped.ln() });
        }
        TabularFactor::from_log_values(scope, log_values)
    }
}

/// Parallel factor product computation using rayon.
///
/// Reduction order is fixed by chunk index rather than completion order.
/// Floating-point addition is not associative, so a completion-ordered reduction
/// would give answers differing in the last bits from run to run.
pub fn parallel_factor_product(factors: &[TabularFactor]) -> LutufiResult<TabularFactor> {
    if factors.is_empty() {
        return TabularFactor::identity(Scope::from_ids_and_sizes(vec![], vec![]));
    }
    if factors.len() == 1 {
        return Ok(factors[0].clone());
    }

    let chunk_size = factors.len().div_ceil(rayon::current_num_threads()).max(1);

    // `par_chunks(...).collect()` preserves index order, so the fold below is
    // deterministic even though the chunks are computed concurrently.
    let partials: Vec<LutufiResult<TabularFactor>> = factors.par_chunks(chunk_size)
        .map(|chunk: &[TabularFactor]| {
            let mut iter = chunk.iter();
            let first = iter.next().ok_or_else(|| LutufiError::InternalError {
                message: "Empty chunk in parallel factor product".to_string()
            })?;
            let mut product = first.clone();
            for factor in iter {
                product = product.multiply(factor)?;
            }
            Ok(product)
        })
        .collect();

    let mut iter = partials.into_iter();
    let mut result = iter.next()
        .ok_or_else(|| LutufiError::InternalError {
            message: "No products from parallel factor computation".to_string()
        })??;
    for partial in iter {
        result = result.multiply(&partial?)?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{domain::Domain, variable::Variable};

    #[test]
    fn test_parallel_factor_product_empty() {
        let result = parallel_factor_product(&[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parallel_factor_product_single() {
        let v = Variable::new("X", Domain::binary());
        let scope = Scope::new(vec![&v]);
        let factor = TabularFactor::from_values(scope, vec![0.5, 0.5]).unwrap();
        let result = parallel_factor_product(&[factor]).unwrap();
        assert_eq!(result.scope().num_entries(), 2);
    }

    /// The property the previous lock-based implementation violated: the answer
    /// must not depend on how many threads computed it.
    #[test]
    fn parallel_factor_product_is_thread_count_independent() {
        let vars: Vec<Variable> = (0..6)
            .map(|i| Variable::new(format!("V{i}"), Domain::binary()))
            .collect();
        let factors: Vec<TabularFactor> = (0..6)
            .map(|i| {
                let scope = Scope::new(vec![&vars[i], &vars[(i + 1) % 6]]);
                TabularFactor::from_values(scope, vec![0.1, 0.2, 0.3, 0.4]).unwrap()
            })
            .collect();

        let run_with = |threads: usize| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| parallel_factor_product(&factors).unwrap())
        };

        let reference = run_with(1);
        for threads in [2usize, 3, 4, 8] {
            let result = run_with(threads);
            assert_eq!(
                reference.scope().variable_ids(),
                result.scope().variable_ids(),
                "scope differs at {threads} threads"
            );
            for i in 0..reference.scope().num_entries() {
                let (a, b) = (reference.log_value_at(i), result.log_value_at(i));
                assert!(
                    (a - b).abs() < 1e-12 || (a.is_infinite() && b.is_infinite()),
                    "entry {i} differs at {threads} threads: {a} vs {b}"
                );
            }
        }
    }
}
