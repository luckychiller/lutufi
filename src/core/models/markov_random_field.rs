use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::PotentialFunction,
    graph::UndirectedVariableGraph,
    variable::{Variable, VariableId},
};

/// A Markov Random Field (MRF) — an undirected graphical model.
///
/// In an MRF, dependencies are represented by undirected edges,
/// and the joint distribution is factorized into potential functions
/// defined over cliques of the graph.
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkovRandomField {
    /// The underlying undirected graph structure.
    graph: UndirectedVariableGraph,
    /// Map from variable ID to the Variable object.
    variables: HashMap<VariableId, Variable>,
    /// Map from variable name to its unique ID.
    name_to_id: HashMap<String, VariableId>,
    /// List of potential functions (factors) defined over variables in the network.
    factors: Vec<PotentialFunction>,
}

impl MarkovRandomField {
    /// Create a new, empty Markov Random Field.
    pub fn new() -> Self {
        MarkovRandomField {
            graph: UndirectedVariableGraph::new(),
            variables: HashMap::new(),
            name_to_id: HashMap::new(),
            factors: Vec::new(),
        }
    }

    /// Add a new variable to the network.
    ///
    /// # Errors
    /// Returns `LutufiError::VariableAlreadyExists` if a variable with this name already exists.
    pub fn add_variable(&mut self, name: &str, domain: crate::core::domain::Domain) -> LutufiResult<&Variable> {
        if self.name_to_id.contains_key(name) {
            return Err(LutufiError::VariableAlreadyExists { name: name.to_string() });
        }
        let var = Variable::new(name, domain);
        let id = var.id();
        self.graph.add_node(id);
        self.name_to_id.insert(name.to_string(), id);
        self.variables.insert(id, var);
        Ok(self.variables.get(&id).unwrap())
    }

    /// Add an undirected edge between two variables.
    ///
    /// # Errors
    /// Returns `LutufiError::VariableNotFound` if either variable doesn't exist.
    pub fn add_edge(&mut self, name1: &str, name2: &str) -> LutufiResult<()> {
        let id1 = self.id_of(name1)?;
        let id2 = self.id_of(name2)?;
        self.graph.add_edge(&id1, &id2);
        Ok(())
    }

    /// Add a potential function (factor) to the network.
    pub fn add_factor(&mut self, factor: PotentialFunction) {
        self.factors.push(factor);
    }

    /// Get the Markov blanket of a variable (its neighbors in the undirected graph).
    ///
    /// # Errors
    /// Returns `LutufiError::VariableNotFound` if the variable doesn't exist.
    pub fn markov_blanket(&self, name: &str) -> LutufiResult<Vec<&str>> {
        let id = self.id_of(name)?;
        Ok(self.graph.neighbors(&id).iter()
            .filter_map(|id| self.variables.get(id))
            .map(|v| v.name())
            .collect())
    }

    /// Get all variable names in this network.
    pub fn nodes(&self) -> Vec<&str> {
        self.variables.values().map(|v| v.name()).collect()
    }

    /// Get all edges as (name1, name2) pairs.
    pub fn edges(&self) -> Vec<(&str, &str)> {
        self.graph.edges().iter()
            .filter_map(|(id1, id2)| {
                let v1 = self.variables.get(id1)?.name();
                let v2 = self.variables.get(id2)?.name();
                Some((v1, v2))
            })
            .collect()
    }

    /// Convert this MRF to a Factor Graph representation.
    ///
    /// # Errors
    /// Returns `LutufiError` if conversion fails.
    pub fn to_factor_graph(&self) -> LutufiResult<crate::core::models::factor_graph::FactorGraph> {
        crate::core::models::factor_graph::FactorGraph::from_markov_random_field(self)
    }

    fn id_of(&self, name: &str) -> LutufiResult<VariableId> {
        self.name_to_id.get(name).copied().ok_or_else(|| LutufiError::VariableNotFound {
            name: name.to_string(),
            available: self.name_to_id.keys().cloned().collect::<Vec<_>>().join(", "),
        })
    }
}
