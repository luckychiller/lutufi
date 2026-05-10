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

pub mod types;
pub mod engine;
pub mod hmm;
pub mod kalman;
pub mod temporal_query;
pub mod evidence_manager;
pub mod incremental;

pub use types::{DBNInferenceOptions, DBNInferenceResult};
pub use engine::{DBNInference, DBNInferenceEngine};
pub use hmm::HMMEngine;
pub use kalman::{KalmanFilter, KalmanFilterResult};
pub use temporal_query::{TemporalQueryEngine, TemporalMode};
pub use evidence_manager::EvidenceManager;
pub use incremental::{IncrementalInferenceContext, IncrementalUpdateOptions, effective_sample_size, particle_filter_weights};
