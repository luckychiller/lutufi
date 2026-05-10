//! Factor Store for CPT Management
//!
//! This module separates Conditional Probability Table (CPT) management
//! into its own responsibility, following the Single Responsibility Principle.

use crate::core::{
    error::LutufiResult,
    factor::ConditionalProbabilityTable,
    variable::VariableId,
};
use std::collections::HashMap;

/// Trait for managing conditional probability tables.
///
/// This abstraction allows different storage strategies for CPTs
/// (e.g., in-memory, sparse, cached) without affecting the network.
pub trait FactorStore {
    /// Store a CPT for a variable.
    fn set_cpd(&mut self, var_id: VariableId, cpd: ConditionalProbabilityTable) -> LutufiResult<()>;

    /// Retrieve a CPT for a variable.
    fn get_cpd(&self, var_id: VariableId) -> Option<&ConditionalProbabilityTable>;

    /// Remove a CPT for a variable.
    fn remove_cpd(&mut self, var_id: VariableId) -> Option<ConditionalProbabilityTable>;

    /// Get all stored CPDs.
    fn all_cpds(&self) -> &HashMap<VariableId, ConditionalProbabilityTable>;

    /// Check if a CPD exists for a variable.
    fn has_cpd(&self, var_id: VariableId) -> bool;

    /// Count of stored CPDs.
    fn count(&self) -> usize;
}

/// Default in-memory implementation of FactorStore.
pub struct InMemoryFactorStore {
    cpds: HashMap<VariableId, ConditionalProbabilityTable>,
}

impl InMemoryFactorStore {
    /// Create a new empty factor store.
    pub fn new() -> Self {
        InMemoryFactorStore {
            cpds: HashMap::new(),
        }
    }
}

impl Default for InMemoryFactorStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FactorStore for InMemoryFactorStore {
    fn set_cpd(&mut self, var_id: VariableId, cpd: ConditionalProbabilityTable) -> LutufiResult<()> {
        self.cpds.insert(var_id, cpd);
        Ok(())
    }

    fn get_cpd(&self, var_id: VariableId) -> Option<&ConditionalProbabilityTable> {
        self.cpds.get(&var_id)
    }

    fn remove_cpd(&mut self, var_id: VariableId) -> Option<ConditionalProbabilityTable> {
        self.cpds.remove(&var_id)
    }

    fn all_cpds(&self) -> &HashMap<VariableId, ConditionalProbabilityTable> {
        &self.cpds
    }

    fn has_cpd(&self, var_id: VariableId) -> bool {
        self.cpds.contains_key(&var_id)
    }

    fn count(&self) -> usize {
        self.cpds.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor_store_operations() {
        let store = InMemoryFactorStore::new();
        let var_id = VariableId::new();

        assert!(!store.has_cpd(var_id));
        assert_eq!(store.count(), 0);
    }
}
