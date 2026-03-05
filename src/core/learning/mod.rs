//! Learning module
//!
//! Provides learning algorithms:
//! - Parameter learning (MLE, Bayesian)
//! - Structure learning (constraint-based, score-based)
//! - Expectation-Maximization for incomplete data
//! - Online learning algorithms

/// Trait for learning algorithms
pub trait LearningAlgorithm {
    /// Type for learned model
    type Model;
    /// Type for training data
    type Data;
    
    /// Fit the model to data
    fn fit(&mut self, data: &Self::Data) -> crate::Result<Self::Model>;
    
    /// Perform one iteration of online learning
    fn update(&mut self, sample: &Self::Data) -> crate::Result<()>;
}

/// Learning options
#[derive(Debug, Clone, Copy)]
pub struct LearningOptions {
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Regularization parameter
    pub regularization: f64,
    /// Convergence threshold
    pub tolerance: f64,
}

impl Default for LearningOptions {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            learning_rate: 0.01,
            regularization: 0.001,
            tolerance: 1e-6,
        }
    }
}

/// Learning result
#[derive(Debug, Clone)]
pub struct LearningResult {
    /// Number of iterations performed
    pub iterations: usize,
    /// Final log-likelihood
    pub log_likelihood: f64,
    /// Whether the algorithm converged
    pub converged: bool,
    /// Training time in seconds
    pub training_time_secs: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_options_default() {
        let opts = LearningOptions::default();
        assert_eq!(opts.max_iterations, 100);
        assert_eq!(opts.learning_rate, 0.01);
        assert_eq!(opts.regularization, 0.001);
    }
}