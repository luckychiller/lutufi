//! Core module for network analysis and probabilistic reasoning
//!
//! This module contains submodules for:
//! - `models`: Network models (Bayesian networks, Markov networks, etc.)
//! - `representation`: Graph and network representations
//! - `inference`: Probabilistic inference algorithms
//! - `learning`: Parameter and structure learning
//! - `io`: Input/Output operations for various formats

/// Network models module (Bayesian networks, MRFs, etc.)
pub mod models;
/// Module for graph and network representations
pub mod representation;
/// Module for probabilistic inference algorithms
pub mod inference;
/// Module for parameter and structure learning
pub mod learning;
/// Module for input/output operations
pub mod io;
/// Module for sets of possible values a random variable can take
pub mod domain;
/// Module for error handling and LutufiError type
pub mod error;
/// Module for variables with unique IDs and names
pub mod variable;
/// Module for (possibly partial) assignments of values to variables
pub mod assignment;
/// Module for probabilistic factors and CPTs
pub mod factor;
/// Module for graph structures (directed and undirected)
pub mod graph;
/// Module for missing data handling (Phase 7)
pub mod missing_data;
/// Module for numerical utilities and resource budgets (Phase 10)
pub mod numerics;
/// Module for scalability features (Phase 10)
pub mod scalability;

/// Re-export commonly used types
pub use models::*;
pub use representation::*;
pub use inference::*;
pub use learning::*;
pub use io::*;
