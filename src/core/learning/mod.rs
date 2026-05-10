//! Learning module
//!
//! Provides learning algorithms:
//! - Parameter learning (MLE, Bayesian, EM)
//! - Structure learning (constraint-based, score-based)
//! - Expectation-Maximization for incomplete data

/// Parameter learning implementation.
pub mod parameter;
/// Structure learning implementation.
pub mod structure;
/// Common data processing utilities.
pub mod data_processor;

pub use parameter::{
    ParameterEstimator, ParameterLearner, ParameterLearningMethod, 
    ParameterLearningOptions, LegacyParameterLearningOptions, SmoothingMethod
};
pub use data_processor::DataProcessor;
pub use structure::{
    score_based::{ScoreBasedLearner, ScoreType, ScoreBasedOptions},
    constraint_based::{
        ConstraintBasedLearner, IndependenceTestType, ConstraintBasedOptions,
        SkeletonDiscovery, VStructureOrientator, MeeksRuleApplier, ConstrainedNetworkBuilder,
        EdgeOrientation, SkeletonResult, VStructureResult,
        FciResult, PagGraph, PagEdgeMark,
    },
};

/// Trait for learning algorithms (Base trait)
pub trait LearningAlgorithm {
    /// Type for learned model
    type Model;
    /// Type for training data
    type Data;
    
    /// Fit the model to data
    fn fit(&mut self, data: &Self::Data) -> crate::core::error::LutufiResult<Self::Model>;
}

/// Generic learning options (for shared configurations)
#[derive(Debug, Clone, Copy)]
pub struct LearningOptions {
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Convergence threshold
    pub tolerance: f64,
    /// Random seed
    pub seed: Option<u64>,
}

impl Default for LearningOptions {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-6,
            seed: None,
        }
    }
}

/// Learning result (Diagnostic summary)
#[derive(Debug, Clone)]
pub struct LearningResult {
    /// Number of iterations performed
    pub iterations: usize,
    /// Final log-likelihood (or score)
    pub score: f64,
    /// Whether the algorithm converged
    pub converged: bool,
}
