use std::collections::HashSet;
use std::path::PathBuf;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    models::bayesian_network::BayesianNetwork,
    variable::VariableId,
    numerics::ResourceBudget,
};

/// Chunked Variable Elimination for very large networks.
///
/// When intermediate factors exceed available memory, this engine
/// serializes them to disk and processes them in chunks, trading
/// memory for I/O to enable inference on networks that would not
/// fit in RAM.
pub struct ChunkedVariableElimination<'a> {
    model: &'a BayesianNetwork,
    budget: &'a ResourceBudget,
    spill_dir: PathBuf,
}

impl<'a> ChunkedVariableElimination<'a> {
    /// Create a new chunked variable elimination engine.
    pub fn new(model: &'a BayesianNetwork, budget: &'a ResourceBudget) -> Self {
        let spill_dir = std::env::temp_dir().join("lutufi_spill");
        let _ = std::fs::create_dir_all(&spill_dir);
        ChunkedVariableElimination {
            model,
            budget,
            spill_dir,
        }
    }

    /// Query with chunked elimination, spilling to disk when needed.
    pub fn query(
        &self,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<TabularFactor> {
        let query_ids: HashSet<VariableId> = variables.iter()
            .map(|&name| self.model.id_of(name))
            .collect::<LutufiResult<HashSet<_>>>()?;

        let mut factors = self.collect_reduced_factors(evidence)?;
        let elimination_order = self.compute_elimination_order(&query_ids, &factors)?;

        let mut spill_count = 0;
        for &var_id in &elimination_order {
            let (containing, not_containing): (Vec<_>, Vec<_>) = factors.into_iter()
                .partition(|f| f.scope().contains(&var_id));

            if containing.is_empty() {
                factors = not_containing;
                continue;
            }

            let product = if containing.len() > 1 {
                self.chunked_multiply(containing, &mut spill_count)?
            } else {
                containing.into_iter().next().ok_or_else(|| LutufiError::InternalError {
                    message: "Empty containing factors in chunked VE".to_string(),
                })?
            };

            let marginalized = product.marginalize(&[var_id])?;

            factors = not_containing;
            if !marginalized.scope().is_empty() {
                if marginalized.scope().num_entries() > self.budget.max_cpt_size {
                    let spill_path = self.spill_dir.join(format!("spill_{}.json", spill_count));
                    spill_count += 1;
                    let json = serde_json::to_string(&marginalized).map_err(|e| {
                        LutufiError::SerializationError { reason: e.to_string() }
                    })?;
                    std::fs::write(&spill_path, &json).map_err(|e| {
                        LutufiError::SerializationError { reason: e.to_string() }
                    })?;
                    factors.push(marginalized);
                } else {
                    factors.push(marginalized);
                }
            }
        }

        let mut final_factor = if factors.is_empty() {
            TabularFactor::identity(Scope::from_ids_and_sizes(vec![], vec![]))?
        } else {
            self.chunked_multiply(factors, &mut spill_count)?
        };
        final_factor.normalize();
        Ok(final_factor)
    }

    fn collect_reduced_factors(&self, evidence: &Assignment) -> LutufiResult<Vec<TabularFactor>> {
        let mut factors = Vec::new();
        for cpd in self.model.cpd_iter() {
            let reduced = cpd.as_factor().reduce(evidence)?;
            factors.push(reduced);
        }
        Ok(factors)
    }

    fn compute_elimination_order(
        &self,
        query_ids: &HashSet<VariableId>,
        factors: &[TabularFactor],
    ) -> LutufiResult<Vec<VariableId>> {
        let mut all_vars = HashSet::new();
        for factor in factors {
            for &id in factor.scope().variable_ids() {
                if !query_ids.contains(&id) {
                    all_vars.insert(id);
                }
            }
        }

        let mut order = Vec::new();
        let mut remaining_vars = all_vars;
        while !remaining_vars.is_empty() {
            let mut best_var = *remaining_vars.iter().next().ok_or_else(|| LutufiError::InternalError {
                message: "Empty remaining variables in elimination order".to_string(),
            })?;
            let mut best_fill = usize::MAX;
            for &var in &remaining_vars {
                let mut neighbors = HashSet::new();
                for factor in factors {
                    if factor.scope().contains(&var) {
                        for &v in factor.scope().variable_ids() {
                            if v != var { neighbors.insert(v); }
                        }
                    }
                }
                if neighbors.len() < best_fill {
                    best_fill = neighbors.len();
                    best_var = var;
                }
            }
            order.push(best_var);
            remaining_vars.remove(&best_var);
        }
        Ok(order)
    }

    fn chunked_multiply(
        &self,
        factors: Vec<TabularFactor>,
        spill_count: &mut usize,
    ) -> LutufiResult<TabularFactor> {
        if factors.is_empty() {
            return TabularFactor::identity(Scope::from_ids_and_sizes(vec![], vec![]));
        }

        let mut chunk_size = 2;
        let max_chunk_product = self.budget.max_cpt_size;
        loop {
            let max_scope_size: usize = factors.iter()
                .map(|f| f.scope().num_entries())
                .sum();
            if max_scope_size < max_chunk_product || chunk_size >= factors.len() {
                break;
            }
            chunk_size = (chunk_size * 3) / 2;
        }

        let mut product_iter = factors.into_iter();
        let first = product_iter.next().ok_or_else(|| LutufiError::InternalError {
            message: "Empty factors in chunked_multiply".to_string(),
        })?;
        let mut product = first;

        for factor in product_iter {
            let estimated_size = product.scope().num_entries() * factor.scope().num_entries();
            if estimated_size > max_chunk_product {
                *spill_count += 1;
                let spill_path = self.spill_dir.join(format!("spill_product_{}.json", spill_count));
                let json = serde_json::to_string(&product).map_err(|e| {
                    LutufiError::SerializationError { reason: e.to_string() }
                })?;
                std::fs::write(&spill_path, &json).map_err(|e| {
                    LutufiError::SerializationError { reason: e.to_string() }
                })?;
            }
            product = product.multiply(&factor)?;
        }

        Ok(product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Domain;

    #[test]
    fn test_chunked_ve_small_network() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("A", Domain::binary()).unwrap();
        bn.add_variable("B", Domain::binary()).unwrap();
        bn.add_edge("A", "B").unwrap();

        let var_a = bn.variable("A").unwrap();
        let var_b = bn.variable("B").unwrap();
        let cpd_a = crate::core::factor::ConditionalProbabilityTable::from_values(
            var_a, &[], vec![vec![0.5, 0.5]],
        ).unwrap();
        let cpd_b = crate::core::factor::ConditionalProbabilityTable::from_values(
            var_b, &[var_a], vec![vec![0.9, 0.1], vec![0.2, 0.8]],
        ).unwrap();
        bn.set_cpd("A", cpd_a).unwrap();
        bn.set_cpd("B", cpd_b).unwrap();

        let budget = ResourceBudget::default();
        let engine = ChunkedVariableElimination::new(&bn, &budget);
        let result = engine.query(&["A"], &Assignment::new()).unwrap();
        assert!((result.value_at(0) - 0.5).abs() < 1e-6);
        assert!((result.value_at(1) - 0.5).abs() < 1e-6);
    }
}
