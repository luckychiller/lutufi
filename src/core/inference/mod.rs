//! Probabilistic inference module
//!
//! Provides inference algorithms:
//! - Belief propagation (sum-product)
//! - Loopy belief propagation
//! - Gibbs sampling
//! - Variational inference
//! - Exact inference for small networks

/// Trait for inference algorithms
pub trait InferenceAlgorithm {
    /// Type for inference results
    type Result;
    
    /// Run inference
    fn infer(&mut self) -> crate::Result<Self::Result>;
    
    /// Set evidence for inference
    fn set_evidence(&mut self, node: usize, value: usize) -> crate::Result<()>;
    
    /// Clear all evidence
    fn clear_evidence(&mut self);
}

/// Inference options
#[derive(Debug, Clone, Copy)]
pub struct InferenceOptions {
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Convergence threshold
    pub tolerance: f64,
    /// Random seed for stochastic algorithms
    pub seed: Option<u64>,
}

impl Default for InferenceOptions {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            tolerance: 1e-6,
            seed: None,
        }
    }
}

/// Inference result
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// Marginal probabilities for each node
    pub marginals: Vec<Vec<f64>>,
    /// Number of iterations performed
    pub iterations: usize,
    /// Whether the algorithm converged
    pub converged: bool,
    /// Log-likelihood (if available)
    pub log_likelihood: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_options_default() {
        let opts = InferenceOptions::default();
        assert_eq!(opts.max_iterations, 1000);
        assert_eq!(opts.tolerance, 1e-6);
        assert!(opts.seed.is_none());
    }
}