//! Inference Strategy Factory
//!
//! The factory manages registered inference strategies and provides
//! intelligent algorithm selection. This follows the Factory Pattern and
//! allows InferenceEngine to remain closed for modification while new
//! strategies can be added through the factory.

use crate::core::{
    error::LutufiResult,
    models::bayesian_network::BayesianNetwork,
};
use std::sync::Arc;

use super::config::InferenceConfig;
use super::strategy::{Algorithm, InferenceStrategy};
use super::strategies::{
    JunctionTreeStrategy, LBPStrategy, MCMCStrategy, VariableEliminationStrategy, VariationalStrategy,
};

/// Factory for creating inference strategies.
pub struct InferenceStrategyFactory {
    strategies: Vec<Arc<dyn InferenceStrategy>>,
}

impl InferenceStrategyFactory {
    /// Create a new factory with all built-in strategies registered.
    pub fn new() -> Self {
        let strategies: Vec<Arc<dyn InferenceStrategy>> = vec![
            Arc::new(VariableEliminationStrategy),
            Arc::new(JunctionTreeStrategy),
            Arc::new(LBPStrategy),
            Arc::new(MCMCStrategy),
            Arc::new(VariationalStrategy),
        ];

        InferenceStrategyFactory { strategies }
    }

    /// Get a specific strategy by algorithm type.
    pub fn get_strategy(&self, algorithm: Algorithm) -> LutufiResult<Arc<dyn InferenceStrategy>> {
        self.strategies
            .iter()
            .find(|s| s.algorithm() == algorithm)
            .cloned()
            .ok_or_else(|| crate::core::error::LutufiError::InternalError {
                message: format!("Strategy for {:?} not found", algorithm),
            })
    }

    /// Automatically select the best strategy for a given model.
    ///
    /// Selection criteria:
    /// 1. Prefer Junction Tree for small models (config.exact_max_nodes)
    /// 2. Filter strategies by config-based thresholds
    /// 3. Use cost-based selection among viable strategies
    pub fn select_best_strategy(
        &self,
        model: &BayesianNetwork,
        config: &InferenceConfig,
    ) -> LutufiResult<Arc<dyn InferenceStrategy>> {
        let node_count = model.nodes().len();

        // Prefer exact inference (Junction Tree) for small models
        if node_count <= config.exact_max_nodes {
            if let Ok(strategy) = self.get_strategy(Algorithm::Exact) {
                return Ok(strategy);
            }
        }

        // Filter strategies by config-based viability
        let viable_strategies: Vec<_> = self
            .strategies
            .iter()
            .filter(|s| {
                if !s.can_handle(model) {
                    return false;
                }
                // Apply VE-specific node limit from config
                if s.algorithm() == Algorithm::VariableElimination
                    && node_count > config.ve_max_nodes
                {
                    return false;
                }
                true
            })
            .collect();

        if viable_strategies.is_empty() {
            return Err(crate::core::error::LutufiError::InternalError {
                message: "No inference strategy available for this model".to_string(),
            });
        }

        // Sort by estimated cost and pick the cheapest
        let best = viable_strategies
            .iter()
            .min_by_key(|s| s.estimated_cost(model))
            .ok_or_else(|| crate::core::error::LutufiError::InternalError {
                message: "Failed to select best strategy".to_string(),
            })?;

        Ok((*best).clone())
    }

    /// Register a custom strategy.
    ///
    /// This allows users to add their own inference algorithms without
    /// modifying the core library, supporting the Open/Closed Principle.
    pub fn register_strategy(&mut self, strategy: Arc<dyn InferenceStrategy>) {
        // Check if strategy already exists and replace it
        if let Some(pos) = self
            .strategies
            .iter()
            .position(|s| s.algorithm() == strategy.algorithm())
        {
            self.strategies[pos] = strategy;
        } else {
            self.strategies.push(strategy);
        }
    }

    /// Get all registered strategies.
    pub fn list_strategies(&self) -> Vec<Algorithm> {
        self.strategies.iter().map(|s| s.algorithm()).collect()
    }
}

impl Default for InferenceStrategyFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = InferenceStrategyFactory::new();
        let strategies = factory.list_strategies();
        assert_eq!(strategies.len(), 5); // All 5 built-in strategies
    }

    #[test]
    fn test_get_specific_strategy() {
        let factory = InferenceStrategyFactory::new();
        let strategy = factory.get_strategy(Algorithm::VariableElimination);
        assert!(strategy.is_ok());
    }

    #[test]
    fn test_unknown_strategy() {
        let _factory = InferenceStrategyFactory::new();
        // This would require adding a custom algorithm type, so we skip for now
    }
}
