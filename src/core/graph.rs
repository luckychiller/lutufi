use petgraph::stable_graph::{StableDiGraph, StableUnGraph, NodeIndex};
use petgraph::algo::{is_cyclic_directed, toposort};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::{
    error::{LutufiError, LutufiResult},
    variable::VariableId,
};

/// A directed graph where nodes are identified by VariableId.
/// Wraps petgraph::StableDiGraph and maintains a mapping from VariableId to NodeIndex.
#[derive(Debug, Clone, Default)]
pub struct DirectedVariableGraph {
    graph: StableDiGraph<VariableId, ()>,
    node_index: HashMap<VariableId, NodeIndex>,
    id_at_index: HashMap<NodeIndex, VariableId>,
}

impl DirectedVariableGraph {
    /// Create a new, empty directed graph.
    pub fn new() -> Self {
        DirectedVariableGraph {
            graph: StableDiGraph::new(),
            node_index: HashMap::new(),
            id_at_index: HashMap::new(),
        }
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, var_id: VariableId) {
        if !self.node_index.contains_key(&var_id) {
            let idx = self.graph.add_node(var_id);
            self.node_index.insert(var_id, idx);
            self.id_at_index.insert(idx, var_id);
        }
    }

    /// Remove a node and its incident edges.
    pub fn remove_node(&mut self, var_id: &VariableId) {
        if let Some(idx) = self.node_index.remove(var_id) {
            self.graph.remove_node(idx);
            self.id_at_index.remove(&idx);
        }
    }

    /// Add a directed edge from `from` to `to`.
    /// 
    /// # Errors
    /// - `LutufiError::EdgeToMissingNode` if either node is missing.
    /// - `LutufiError::CyclicGraph` if the edge creates a cycle.
    pub fn add_edge(&mut self, from: &VariableId, to: &VariableId, from_name: &str, to_name: &str) -> LutufiResult<()> {
        let from_idx = self.node_index.get(from).copied()
            .ok_or_else(|| LutufiError::EdgeToMissingNode { from: from_name.to_string(), to: to_name.to_string(), missing: from_name.to_string() })?;
        let to_idx = self.node_index.get(to).copied()
            .ok_or_else(|| LutufiError::EdgeToMissingNode { from: from_name.to_string(), to: to_name.to_string(), missing: to_name.to_string() })?;

        let edge = self.graph.add_edge(from_idx, to_idx, ());
        if is_cyclic_directed(&self.graph) {
            self.graph.remove_edge(edge);
            return Err(LutufiError::CyclicGraph { 
                from: from_name.to_string(), 
                to: to_name.to_string(), 
                cycle: format!("{} -> {}. If you need temporal cycles, use a DynamicBayesianNetwork (DBN).", from_name, to_name) 
            });
        }
        Ok(())
    }

    /// Remove a directed edge.
    pub fn remove_edge(&mut self, from: &VariableId, to: &VariableId) {
        if let (Some(&f), Some(&t)) = (self.node_index.get(from), self.node_index.get(to)) {
            if let Some(e) = self.graph.find_edge(f, t) {
                self.graph.remove_edge(e);
            }
        }
    }

    /// Get all parent IDs of a node.
    pub fn parents(&self, var_id: &VariableId) -> Vec<VariableId> {
        if let Some(&idx) = self.node_index.get(var_id) {
            self.graph.neighbors_directed(idx, petgraph::Direction::Incoming).map(|i| self.id_at_index[&i]).collect()
        } else { Vec::new() }
    }

    /// Get all child IDs of a node.
    pub fn children(&self, var_id: &VariableId) -> Vec<VariableId> {
        if let Some(&idx) = self.node_index.get(var_id) {
            self.graph.neighbors_directed(idx, petgraph::Direction::Outgoing).map(|i| self.id_at_index[&i]).collect()
        } else { Vec::new() }
    }

    /// Get the Markov blanket (parents, children, and parents of children).
    pub fn markov_blanket(&self, var_id: &VariableId) -> Vec<VariableId> {
        let mut blanket = std::collections::HashSet::new();
        for p in self.parents(var_id) { blanket.insert(p); }
        for c in self.children(var_id) {
            blanket.insert(c);
            for cp in self.parents(&c) {
                if cp != *var_id { blanket.insert(cp); }
            }
        }
        blanket.into_iter().collect()
    }

    /// Returns nodes in topological order.
    pub fn topological_order(&self) -> LutufiResult<Vec<VariableId>> {
        toposort(&self.graph, None).map(|ids| ids.iter().filter_map(|idx| self.id_at_index.get(idx).copied()).collect())
            .map_err(|_| LutufiError::InternalError { message: "Topo sort failed".to_string() })
    }

    /// Get all node IDs in the graph.
    pub fn node_ids(&self) -> Vec<VariableId> { self.node_index.keys().copied().collect() }
    /// Get all edges as (from, to) ID pairs.
    pub fn edges(&self) -> Vec<(VariableId, VariableId)> {
        self.graph.edge_references().filter_map(|e| Some((*self.id_at_index.get(&e.source())?, *self.id_at_index.get(&e.target())?))).collect()
    }
    /// The number of nodes in the graph.
    pub fn node_count(&self) -> usize { self.graph.node_count() }
}

impl Serialize for DirectedVariableGraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let nodes: Vec<VariableId> = self.node_ids();
        let edges: Vec<(VariableId, VariableId)> = self.edges();
        (nodes, edges).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DirectedVariableGraph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let (nodes, edges): (Vec<VariableId>, Vec<(VariableId, VariableId)>) = Deserialize::deserialize(deserializer)?;
        let mut g = DirectedVariableGraph::new();
        for n in nodes { g.add_node(n); }
        for (f, t) in edges { 
            // In deserialization we don't have names, but we assume the graph was valid when serialized.
            let f_idx = g.node_index[&f];
            let t_idx = g.node_index[&t];
            g.graph.add_edge(f_idx, t_idx, ());
        }
        Ok(g)
    }
}

/// An undirected graph where nodes are identified by VariableId.
#[derive(Debug, Clone, Default)]
pub struct UndirectedVariableGraph {
    graph: StableUnGraph<VariableId, ()>,
    node_index: HashMap<VariableId, NodeIndex>,
    id_at_index: HashMap<NodeIndex, VariableId>,
}

impl UndirectedVariableGraph {
    /// Create a new, empty undirected graph.
    pub fn new() -> Self {
        UndirectedVariableGraph {
            graph: StableUnGraph::default(),
            node_index: HashMap::new(),
            id_at_index: HashMap::new(),
        }
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, var_id: VariableId) {
        if !self.node_index.contains_key(&var_id) {
            let idx = self.graph.add_node(var_id);
            self.node_index.insert(var_id, idx);
            self.id_at_index.insert(idx, var_id);
        }
    }

    /// Remove a node and its incident edges.
    pub fn remove_node(&mut self, var_id: &VariableId) {
        if let Some(idx) = self.node_index.remove(var_id) {
            self.graph.remove_node(idx);
            self.id_at_index.remove(&idx);
        }
    }

    /// Add an undirected edge between two nodes.
    pub fn add_edge(&mut self, v1: &VariableId, v2: &VariableId) {
        if let (Some(&i1), Some(&i2)) = (self.node_index.get(v1), self.node_index.get(v2)) {
            self.graph.update_edge(i1, i2, ());
        }
    }

    /// Remove an undirected edge.
    pub fn remove_edge(&mut self, v1: &VariableId, v2: &VariableId) {
        if let (Some(&i1), Some(&i2)) = (self.node_index.get(v1), self.node_index.get(v2)) {
            if let Some(e) = self.graph.find_edge(i1, i2) {
                self.graph.remove_edge(e);
            }
        }
    }

    /// Get all neighbor IDs of a node.
    pub fn neighbors(&self, var_id: &VariableId) -> Vec<VariableId> {
        if let Some(&idx) = self.node_index.get(var_id) {
            self.graph.neighbors(idx).map(|i| self.id_at_index[&i]).collect()
        } else { Vec::new() }
    }

    /// Get all edges as pairs of node IDs.
    pub fn edges(&self) -> Vec<(VariableId, VariableId)> {
        self.graph.edge_references().filter_map(|e| Some((*self.id_at_index.get(&e.source())?, *self.id_at_index.get(&e.target())?))).collect()
    }
}

impl Serialize for UndirectedVariableGraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let nodes: Vec<VariableId> = self.node_ids();
        let edges: Vec<(VariableId, VariableId)> = self.edges();
        (nodes, edges).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UndirectedVariableGraph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let (nodes, edges): (Vec<VariableId>, Vec<(VariableId, VariableId)>) = Deserialize::deserialize(deserializer)?;
        let mut g = UndirectedVariableGraph::new();
        for n in nodes { g.add_node(n); }
        for (v1, v2) in edges { g.add_edge(&v1, &v2); }
        Ok(g)
    }
}

impl UndirectedVariableGraph {
    /// Get all node IDs in the graph.
    pub fn node_ids(&self) -> Vec<VariableId> { self.node_index.keys().copied().collect() }
}
