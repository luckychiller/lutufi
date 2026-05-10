//! Unified Inference Engine
//!
//! The InferenceEngine serves as a facade that delegates to specific
//! inference strategies. This design separates algorithm selection logic
//! from algorithm implementation, following the Strategy Pattern and
//! adhering to the Single Responsibility Principle.

use crate::core::{
    assignment::Assignment,
    error::LutufiResult,
    models::bayesian_network::BayesianNetwork,
};

pub use super::strategy::{Algorithm, Diagnostics, InferenceResult, InferenceStrategy};
use super::{config::InferenceConfig, strategy_factory::InferenceStrategyFactory};
use std::sync::OnceLock;

/// Global strategy factory (lazy-initialized singleton for efficiency).
static STRATEGY_FACTORY: OnceLock<InferenceStrategyFactory> = OnceLock::new();

/// Facade for running probabilistic inference.
///
/// The InferenceEngine abstracts away algorithm selection and delegates
/// to specific strategy implementations. This maintains a clean separation
/// of concerns and makes it easy to add new algorithms without modifying
/// the engine.
///
/// Configuration thresholds (like `exact_max_nodes`, `ve_max_nodes`) are
/// user-configurable via [`InferenceConfig`] rather than hardcoded,
/// following the Open/Closed Principle.
pub struct InferenceEngine;

impl InferenceEngine {
    /// Get or initialize the global strategy factory.
    fn factory() -> &'static InferenceStrategyFactory {
        STRATEGY_FACTORY.get_or_init(InferenceStrategyFactory::new)
    }

    /// Run a query using the specified algorithm.
    ///
    /// # Arguments
    /// * `model` - The Bayesian Network to query
    /// * `variables` - Variables to compute marginals for
    /// * `evidence` - Observed variable assignments
    /// * `algorithm` - Algorithm to use (Auto selects automatically)
    ///
    /// # Returns
    /// An InferenceResult with marginals and diagnostics
    ///
    /// # Example
    /// ```
    /// # use lutufi_core::core::{models::bayesian_network::BayesianNetwork, assignment::Assignment, domain::Domain, inference::{Algorithm, InferenceEngine}};
    /// # let mut model = BayesianNetwork::new();
    /// # model.add_variable("Disease", Domain::binary()).unwrap();
    /// # let evidence = Assignment::new();
    /// let result = InferenceEngine::query(
    ///     &model,
    ///     &["Disease"],
    ///     &evidence,
    ///     Algorithm::Auto
    /// ).unwrap();
    /// ```
    pub fn query(
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
        algorithm: Algorithm,
    ) -> LutufiResult<InferenceResult> {
        let config = InferenceConfig::default();
        Self::query_with_config(model, variables, evidence, algorithm, &config)
    }

    /// Run a query with a custom configuration for algorithm selection.
    ///
    /// This allows fine-grained control over when exact vs approximate
    /// inference is preferred, which thresholds trigger warnings, etc.
    ///
    /// # Example
    /// ```
    /// # use lutufi_core::core::{models::bayesian_network::BayesianNetwork, assignment::Assignment, domain::Domain, inference::{Algorithm, InferenceEngine, InferenceConfig}};
    /// # let mut model = BayesianNetwork::new();
    /// # model.add_variable("Disease", Domain::binary()).unwrap();
    /// # let evidence = Assignment::new();
    /// let config = InferenceConfig::new()
    ///     .with_exact_max_nodes(25);
    /// let result = InferenceEngine::query_with_config(
    ///     &model, &["Disease"], &evidence,
    ///     Algorithm::Auto, &config
    /// ).unwrap();
    /// ```
    pub fn query_with_config(
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
        algorithm: Algorithm,
        config: &InferenceConfig,
    ) -> LutufiResult<InferenceResult> {
        let factory = Self::factory();

        // Select strategy based on algorithm choice and config
        let strategy = if algorithm == Algorithm::Auto {
            factory.select_best_strategy(model, config)?
        } else {
            factory.get_strategy(algorithm)?
        };

        // Delegate to the strategy
        strategy.infer(model, variables, evidence)
    }

    /// Run a query with automatic algorithm selection.
    ///
    /// This is a convenience method that automatically selects the best
    /// algorithm for your model based on size and structure.
    pub fn query_auto(
        model: &BayesianNetwork,
        variables: &[&str],
        evidence: &Assignment,
    ) -> LutufiResult<InferenceResult> {
        Self::query(model, variables, evidence, Algorithm::Auto)
    }

    /// List all available inference algorithms.
    pub fn available_algorithms() -> Vec<Algorithm> {
        Self::factory().list_strategies()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_algorithms() {
        let algorithms = InferenceEngine::available_algorithms();
        assert!(!algorithms.is_empty());
        assert!(algorithms.contains(&Algorithm::VariableElimination));
        assert!(algorithms.contains(&Algorithm::LBP));
    }
}
