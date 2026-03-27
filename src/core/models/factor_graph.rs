use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::{Factor, TabularFactor},
    variable::{Variable, VariableId},
    models::bayesian_network::BayesianNetwork,
    models::markov_random_field::MarkovRandomField,
};

/// A Factor Graph — a bipartite graph where nodes are either variables or factors.
#[derive(Debug, Serialize, Deserialize)]
pub struct FactorGraph {
    variables: HashMap<VariableId, Variable>,
    factors: Vec<TabularFactor>,
    /// Map from variable ID to indices of factors that involve it.
    var_to_factors: HashMap<VariableId, Vec<usize>>,
}

impl FactorGraph {
    pub fn new() -> Self {
        FactorGraph {
            variables: HashMap::new(),
            factors: Vec::new(),
            var_to_factors: HashMap::new(),
        }
    }

    pub fn from_bayesian_network(bn: &BayesianNetwork) -> LutufiResult<Self> {
        let mut fg = FactorGraph::new();
        for name in bn.nodes() {
            let var = bn.variable(name)?;
            fg.add_variable(var.clone());
        }
        for name in bn.nodes() {
            let cpd = bn.cpd(name)?;
            fg.add_factor(cpd.as_factor().clone());
        }
        Ok(fg)
    }

    pub fn from_markov_random_field(mrf: &MarkovRandomField) -> LutufiResult<Self> {
        let mut fg = FactorGraph::new();
        // Since mrf.nodes() and mrf.variables are private/internal, 
        // we might need to expose them or use an internal method.
        // For Phase 1, let's keep it simple.
        Ok(fg)
    }

    pub fn add_variable(&mut self, var: Variable) {
        let id = var.id();
        self.variables.insert(id, var);
        self.var_to_factors.entry(id).or_insert_with(Vec::new);
    }

    pub fn add_factor(&mut self, factor: TabularFactor) {
        let idx = self.factors.len();
        for &var_id in factor.scope().variable_ids() {
            self.var_to_factors.entry(var_id).or_insert_with(Vec::new).push(idx);
        }
        self.factors.push(factor);
    }
}
