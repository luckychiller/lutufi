//! Dynamic Bayesian Network inference algorithms.
//!
//! This module provides:
//! - `types`: Shared types for DBN inference
//! - `engine`: DBN filtering, smoothing, and prediction
//! - `hmm`: Hidden Markov Model inference
//! - `kalman`: Kalman Filter for linear Gaussian DBNs
//! - `temporal_query`: Temporal query API
//! - `evidence_manager`: Efficient evidence management
//! - `incremental`: Incremental inference for evolving networks

/// Shared types for DBN inference configuration and results.
pub mod types;
/// Core DBN inference engine implementing filtering, smoothing, and prediction.
pub mod engine;
/// Hidden Markov Model inference algorithms (Forward, Forward-Backward, Viterbi, Baum-Welch).
pub mod hmm;
/// Kalman Filter and Kalman Smoother for linear Gaussian DBNs.
pub mod kalman;
/// Unified temporal query API for convenient DBN inference.
pub mod temporal_query;
/// Efficient evidence caching and management for incremental inference.
pub mod evidence_manager;
/// Incremental inference for networks with evolving structure.
pub mod incremental;

pub use types::{DBNInferenceOptions, DBNInferenceResult};
pub use engine::{DBNInference, DBNInferenceEngine};
pub use hmm::HMMEngine;
pub use kalman::{KalmanFilter, KalmanFilterResult};
pub use temporal_query::{TemporalQueryEngine, TemporalMode};
pub use evidence_manager::EvidenceManager;
pub use incremental::{IncrementalInferenceContext, IncrementalUpdateOptions, effective_sample_size, particle_filter_weights};
