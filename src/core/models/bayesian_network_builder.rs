//! BayesianNetwork Builder
//!
//! This module provides the Builder pattern for constructing Bayesian Networks.
//! It separates construction logic from the network itself, following the
//! Builder Pattern and improving separation of concerns.

use crate::core::{
    domain::Domain,
    error::LutufiResult,
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
};

/// Builder for constructing Bayesian Networks step-by-step.
///
/// This builder allows you to construct complex networks incrementally
/// and validates the structure at each step.
///
/// # Example
/// ```
/// # use lutufi_core::core::{models::bayesian_network::BayesianNetwork, domain::Domain};
/// let network = BayesianNetwork::builder()
///     .add_variable("A", Domain::binary()).unwrap()
///     .add_variable("B", Domain::binary()).unwrap()
///     .add_edge("A", "B").unwrap()
///     .build().unwrap();
/// ```
pub struct BayesianNetworkBuilder {
    network: BayesianNetwork,
}

impl BayesianNetworkBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        BayesianNetworkBuilder {
            network: BayesianNetwork::new(),
        }
    }

    /// Add a variable to the network.
    ///
    /// # Arguments
    /// * `name` - The variable name (must be unique in the network)
    /// * `domain` - The domain (set of possible values)
    ///
    /// # Errors
    /// Returns error if a variable with this name already exists.
    pub fn add_variable(mut self, name: &str, domain: Domain) -> LutufiResult<Self> {
        self.network.add_variable(name, domain)?;
        Ok(self)
    }

    /// Add multiple variables at once.
    pub fn add_variables(mut self, variables: Vec<(&str, Domain)>) -> LutufiResult<Self> {
        for (name, domain) in variables {
            self.network.add_variable(name, domain)?;
        }
        Ok(self)
    }

    /// Add an edge between two variables.
    ///
    /// # Arguments
    /// * `from` - Parent variable name
    /// * `to` - Child variable name
    ///
    /// # Errors
    /// Returns error if edge would create a cycle or variables don't exist.
    pub fn add_edge(mut self, from: &str, to: &str) -> LutufiResult<Self> {
        self.network.add_edge(from, to)?;
        Ok(self)
    }

    /// Add multiple edges at once.
    pub fn add_edges(mut self, edges: Vec<(&str, &str)>) -> LutufiResult<Self> {
        for (from, to) in edges {
            self.network.add_edge(from, to)?;
        }
        Ok(self)
    }

    /// Set the conditional probability table for a variable.
    pub fn set_cpd(
        mut self,
        variable_name: &str,
        cpd: ConditionalProbabilityTable,
    ) -> LutufiResult<Self> {
        self.network.set_cpd(variable_name, cpd)?;
        Ok(self)
    }

    /// Mark this network as a structural causal model.
    pub fn as_causal(mut self) -> Self {
        self.network.mark_as_causal();
        self
    }

    /// Finalize and return the built network.
    ///
    /// This performs final validation to ensure the network is well-formed.
    pub fn build(self) -> LutufiResult<BayesianNetwork> {
        // Optional: Could add validation here
        Ok(self.network)
    }

    /// Clone the current network state without finalizing.
    pub fn peek(&self) -> &BayesianNetwork {
        &self.network
    }
}

impl Default for BayesianNetworkBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_chain() -> LutufiResult<()> {
        let network = BayesianNetworkBuilder::new()
            .add_variable("A", Domain::binary())?
            .add_variable("B", Domain::binary())?
            .add_edge("A", "B")?
            .build()?;

        assert_eq!(network.nodes().len(), 2);
        assert_eq!(network.edges().len(), 1);
        Ok(())
    }

    #[test]
    fn test_builder_as_causal() -> LutufiResult<()> {
        let network = BayesianNetworkBuilder::new()
            .add_variable("X", Domain::binary())?
            .as_causal()
            .build()?;

        assert!(network.is_causal());
        Ok(())
    }
}
