use std::sync::{Arc, OnceLock};
use crate::core::{
    error::LutufiResult,
    factor::{Scope, TabularFactor},
    variable::{Variable, VariableId},
};

/// A lazily-computed factor wrapper.
///
/// The actual factor values are not computed until accessed,
/// enabling deferred computation and automatic caching.
#[derive(Clone)]
pub struct LazyFactor {
    /// Variable IDs in this factor's scope.
    var_ids: Vec<VariableId>,
    /// Domain sizes for each variable.
    sizes: Vec<usize>,
    /// Cached computed factor (wrapped in Result for fallible init).
    cached: Arc<OnceLock<LutufiResult<TabularFactor>>>,
    /// Closure (as Arc trait object) that produces the factor.
    computer: Arc<dyn Fn() -> LutufiResult<TabularFactor> + Send + Sync>,
}

impl std::fmt::Debug for LazyFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyFactor")
            .field("var_ids", &self.var_ids)
            .field("sizes", &self.sizes)
            .field("cached", &self.cached.get().map(|_| "Some"))
            .field("computer", &"<closure>")
            .finish()
    }
}

impl LazyFactor {
    pub fn new<F>(var_ids: Vec<VariableId>, sizes: Vec<usize>, computer: F) -> Self
    where
        F: Fn() -> LutufiResult<TabularFactor> + Send + Sync + 'static,
    {
        LazyFactor {
            var_ids,
            sizes,
            cached: Arc::new(OnceLock::new()),
            computer: Arc::new(computer),
        }
    }

    pub fn compute(&self) -> LutufiResult<&TabularFactor> {
        let result = self.cached.get_or_init(|| (self.computer)());
        match result {
            Ok(factor) => Ok(factor),
            Err(e) => Err(e.clone()),
        }
    }

    pub fn scope(&self) -> Scope {
        Scope::from_ids_and_sizes(self.var_ids.clone(), self.sizes.clone())
    }

    pub fn is_computed(&self) -> bool {
        self.cached.get().is_some()
    }

    pub fn num_entries(&self) -> usize {
        self.sizes.iter().product()
    }
}

/// A factor graph that uses lazy evaluation for its factors.
///
/// Factors are only computed when needed, reducing overhead
/// for queries that only access a subset of the network.
#[derive(Debug, Clone)]
pub struct LazyFactorGraph {
    variables: Vec<Variable>,
    factors: Vec<LazyFactor>,
}

impl LazyFactorGraph {
    pub fn new(variables: Vec<Variable>) -> Self {
        LazyFactorGraph {
            variables,
            factors: Vec::new(),
        }
    }

    pub fn add_factor(&mut self, factor: LazyFactor) {
        self.factors.push(factor);
    }

    pub fn factors(&self) -> &[LazyFactor] { &self.factors }

    pub fn variables(&self) -> &[Variable] { &self.variables }

    pub fn compute_all(&self) -> LutufiResult<Vec<&TabularFactor>> {
        self.factors.iter().map(|f| f.compute()).collect()
    }

    pub fn compute_for_variable(&self, var_id: &VariableId) -> LutufiResult<Vec<TabularFactor>> {
        let mut results = Vec::new();
        for lazy in &self.factors {
            if lazy.var_ids.contains(var_id) {
                results.push(lazy.compute()?.clone());
            }
        }
        Ok(results)
    }
}

/// Build a lazy factor graph from a Bayesian network.
pub fn build_lazy_factor_graph(
    bn: &crate::core::models::bayesian_network::BayesianNetwork,
) -> LutufiResult<LazyFactorGraph> {
    let variables: Vec<Variable> = bn.variables().values().cloned().collect();
    let mut lazy_fg = LazyFactorGraph::new(variables);

    for name in bn.nodes() {
        let cpd = bn.cpd(name)?;
        let var_ids = cpd.as_factor().scope().variable_ids().to_vec();
        let sizes = cpd.as_factor().scope().sizes().to_vec();
        let cpd_clone = cpd.as_factor().clone();

        let lazy = LazyFactor::new(var_ids, sizes, move || {
            Ok(cpd_clone.clone())
        });
        lazy_fg.add_factor(lazy);
    }

    Ok(lazy_fg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::variable::Variable;
    use crate::core::domain::Domain;

    #[test]
    fn test_lazy_factor_not_computed_until_requested() {
        let v = Variable::new("X", Domain::binary());
        let var_ids = vec![v.id()];
        let sizes = vec![2];

        let lazy = LazyFactor::new(var_ids.clone(), sizes.clone(), move || {
            let scope = Scope::from_ids_and_sizes(var_ids.clone(), sizes.clone());
            TabularFactor::from_values(scope, vec![0.5, 0.5])
        });

        assert!(!lazy.is_computed());
        let _ = lazy.compute();
        assert!(lazy.is_computed());
    }

    #[test]
    fn test_lazy_factor_graph_creation() {
        let v = Variable::new("Y", Domain::binary());
        let fg = LazyFactorGraph::new(vec![v]);
        assert_eq!(fg.variables().len(), 1);
        assert!(fg.factors().is_empty());
    }
}
