//! Network representation module
//!
//! Provides graph and network representations:
//! - Adjacency lists and matrices
//! - Sparse matrix representations
//! - Edge lists
//! - Node and edge attributes

use ndarray::Array2;
use sprs::CsMat;

/// Trait for graph representations
pub trait GraphRepresentation {
    /// Type for node identifiers
    type NodeId;
    /// Type for edge weights
    type EdgeWeight;
    
    /// Check if the graph is directed
    fn is_directed(&self) -> bool;
    
    /// Get the number of nodes
    fn num_nodes(&self) -> usize;
    
    /// Get the number of edges
    fn num_edges(&self) -> usize;
}

/// Sparse adjacency matrix representation
pub struct SparseAdjacencyMatrix {
    /// Compressed sparse row matrix
    pub matrix: CsMat<f64>,
    /// Node mapping
    pub node_ids: Vec<String>,
}

/// Dense adjacency matrix representation
pub struct DenseAdjacencyMatrix {
    /// Dense matrix
    pub matrix: Array2<f64>,
    /// Node mapping
    pub node_ids: Vec<String>,
}

/// Edge list representation
#[derive(Debug, Clone)]
pub struct EdgeList {
    /// Source nodes
    pub sources: Vec<usize>,
    /// Target nodes
    pub targets: Vec<usize>,
    /// Edge weights (optional)
    pub weights: Option<Vec<f64>>,
}

impl EdgeList {
    /// Create a new empty edge list
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            targets: Vec::new(),
            weights: None,
        }
    }
}

impl Default for EdgeList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_list_new() {
        let edge_list = EdgeList::new();
        assert!(edge_list.sources.is_empty());
        assert!(edge_list.targets.is_empty());
        assert!(edge_list.weights.is_none());
    }
}