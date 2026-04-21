//! Probabilistic inference module
//!
//! Provides inference algorithms:
//! - Belief propagation (sum-product)
//! - Loopy belief propagation
//! - Gibbs sampling
//! - Variational inference
//! - Exact inference for small networks

/// Exact inference using variable elimination.
pub mod variable_elimination;
/// Exact inference using junction trees.
pub mod junction_tree;
/// Approximate inference using Loopy Belief Propagation.
pub mod lbp;
/// Approximate inference using Markov Chain Monte Carlo.
pub mod mcmc;
/// Approximate inference using Variational Inference.
pub mod variational;
/// Unified inference engine.
pub mod engine;

pub use engine::{Algorithm, InferenceResult, InferenceEngine, Diagnostics};

/// Inference result (Legacy, for backward compatibility during Phase 3)
#[derive(Debug, Clone)]
pub struct LegacyInferenceResult {
    /// Marginal probabilities.
    pub marginals: Vec<Vec<f64>>,
    /// Number of iterations.
    pub iterations: usize,
    /// Whether the algorithm converged.
    pub converged: bool,
    /// Final log-likelihood.
    pub log_likelihood: Option<f64>,
}
