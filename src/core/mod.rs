//! Core module for network analysis and probabilistic reasoning
//!
//! This module contains submodules for:
//! - `models`: Network models (Bayesian networks, Markov networks, etc.)
//! - `representation`: Graph and network representations
//! - `inference`: Probabilistic inference algorithms
//! - `learning`: Parameter and structure learning
//! - `io`: Input/Output operations for various formats

pub mod models;
pub mod representation;
pub mod inference;
pub mod learning;
pub mod io;
pub mod domain;
pub mod error;
pub mod variable;
pub mod assignment;

/// Re-export commonly used types
pub use models::*;
pub use representation::*;
pub use inference::*;
pub use learning::*;
pub use io::*;