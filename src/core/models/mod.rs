//! Network models module
//!
//! Provides implementations for various network models including:
//! - Bayesian Networks
//! - Markov Random Fields
//! - Dynamic Bayesian Networks
//! - Factor Graphs

pub mod bayesian_network;
pub mod markov_random_field;
pub mod dynamic_bayesian_network;
pub mod factor_graph;

use serde::{Deserialize, Serialize};

/// Trait for all network models
pub trait NetworkModel {
    /// Get the number of nodes in the model
    fn node_count(&self) -> usize;
    
    /// Get the number of edges in the model
    fn edge_count(&self) -> usize;
}

/// Model metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model description
    pub description: Option<String>,
    /// Creation timestamp
    pub created_at: Option<String>,
}

impl Default for ModelMetadata {
    fn default() -> Self {
        Self {
            name: "unnamed".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            created_at: None,
        }
    }
}
