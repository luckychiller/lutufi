//! Network models module
//!
//! Provides implementations for various network models including:
//! - Bayesian Networks
//! - Markov Random Fields
//! - Dynamic Bayesian Networks
//! - Factor Graphs

//! Probabilistic Graphical Models.
//!
//! This module provides the different model types supported by Lutufi:
//! - Bayesian Networks (Directed Acyclic Graphs)
//! - Markov Random Fields (Undirected Graphs)
//! - Dynamic Bayesian Networks (Temporal Models)
//! - Factor Graphs (Internal Representation)

/// Bayesian Network implementation.
pub mod bayesian_network;
/// Causal Model implementation.
pub mod causal_model;
/// Markov Random Field implementation.
pub mod markov_random_field;
/// Dynamic Bayesian Network implementation.
pub mod dynamic_bayesian_network;
/// Factor Graph implementation.
pub mod factor_graph;
/// Network registry implementation.
pub mod registry;
/// Sampling engine for Bayesian Networks.
pub mod sampler;
/// Builder pattern for Bayesian Networks.
pub mod bayesian_network_builder;
/// Factor store for CPT management.
pub mod factor_store;

pub use bayesian_network::{BayesianNetwork, StructureLearningMethod};
pub use bayesian_network_builder::BayesianNetworkBuilder;
pub use causal_model::CausalModel;
pub use markov_random_field::MarkovRandomField;
pub use dynamic_bayesian_network::DynamicBayesianNetwork;
pub use factor_graph::FactorGraph;
pub use factor_store::{FactorStore, InMemoryFactorStore};

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
