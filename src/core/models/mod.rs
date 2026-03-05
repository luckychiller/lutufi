//! Network models module
//!
//! Provides implementations for various network models including:
//! - Bayesian Networks
//! - Markov Random Fields
//! - Exponential Random Graph Models (ERGMs)
//! - Temporal network models

use serde::{Deserialize, Serialize};

/// Trait for all network models
pub trait NetworkModel {
    /// Get the number of nodes in the model
    fn node_count(&self) -> usize;
    
    /// Get the number of edges in the model
    fn edge_count(&self) -> usize;
    
    /// Validate the model structure
    fn validate(&self) -> crate::Result<()>;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_metadata_default() {
        let meta = ModelMetadata::default();
        assert_eq!(meta.name, "unnamed");
        assert_eq!(meta.version, "0.1.0");
    }
}