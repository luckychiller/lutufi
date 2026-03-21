//! Lutufi - High-performance network analysis library with probabilistic reasoning
//!
//! This library provides core functionality for:
//! - Network/graph representation and manipulation
//! - Probabilistic inference on network structures
//! - Learning network parameters from data
//! - I/O operations for network formats

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod core;

#[cfg(feature = "python")]
#[path = "../bindings/ffi/mod.rs"]
pub mod ffi;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::*;
}

use thiserror::Error;

/// Main error type for Lutufi operations
#[derive(Error, Debug)]
pub enum LutufiError {
    /// Error in model operations
    #[error("Model error: {0}")]
    Model(String),
    
    /// Error in inference operations
    #[error("Inference error: {0}")]
    Inference(String),
    
    /// Error in learning operations
    #[error("Learning error: {0}")]
    Learning(String),
    
    /// Error in I/O operations
    #[error("I/O error: {0}")]
    Io(String),
    
    /// Error in representation operations
    #[error("Representation error: {0}")]
    Representation(String),
}

/// Result type alias for Lutufi operations
pub type Result<T> = std::result::Result<T, LutufiError>;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }

    #[test]
    fn test_error_display() {
        let err = LutufiError::Model("test error".to_string());
        assert_eq!(err.to_string(), "Model error: test error");
    }
}