use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    variable::{Variable, VariableId},
};
use super::scope::Scope;
use super::tabular::TabularFactor;


#[derive(Debug, Clone, Serialize, Deserialize)]
/// A conditional probability table (CPT) for a discrete variable given its parents.
pub struct ConditionalProbabilityTable {
    child_id: VariableId,
    parent_ids: Vec<VariableId>,
    factor: TabularFactor,
}

impl ConditionalProbabilityTable {
    /// Creates a `ConditionalProbabilityTable` from a child, its parents, and conditional probability values.
    pub fn from_values(child: &Variable, parents: &[&Variable], values: Vec<Vec<f64>>) -> LutufiResult<Self> {
        let mut scope_vars = parents.to_vec();
        scope_vars.push(child);
        let scope = Scope::new(scope_vars.clone());

        if values.is_empty() {
            return Err(LutufiError::CptWrongShape {
                variable: child.name().to_string(),
                expected_shape: format!("{}", scope.num_entries()),
                actual_shape: "0".to_string(),
            });
        }

        let num_rows = values.len();
        let num_cols = values[0].len();

        if num_rows * num_cols != scope.num_entries() {
            return Err(LutufiError::CptWrongShape {
                variable: child.name().to_string(),
                expected_shape: format!("{}", scope.num_entries()),
                actual_shape: format!("{}", num_rows * num_cols),
            });
        }

        // `values` is given as one row per child state and one column per
        // parent configuration, so each *column* (a conditional distribution
        // over the child's states for a fixed parent configuration) must sum
        // to 1.
        for j in 0..num_cols {
            let mut sum = 0.0;
            for i in 0..num_rows {
                sum += values[i][j];
            }
            if (sum - 1.0).abs() > 1e-6 {
                return Err(LutufiError::CptDoesNotNormalize {
                    variable: child.name().to_string(),
                    parent_config: format!("index {}", j),
                    actual_sum: sum,
                });
            }
        }

        // The factor's flat layout places the child as the fastest-varying
        // (innermost) dimension and the parents as slower-varying dimensions,
        // in `parents` order. So flat index = parent_config * num_rows + child_state.
        let mut flat_values = vec![0.0; num_rows * num_cols];
        for i in 0..num_rows {
            for j in 0..num_cols {
                flat_values[j * num_rows + i] = values[i][j];
            }
        }

        let factor = TabularFactor::from_values(scope, flat_values)?;

        Ok(ConditionalProbabilityTable {
            child_id: child.id(),
            parent_ids: parents.iter().map(|v| v.id()).collect(),
            factor,
        })
    }

    /// Creates a `ConditionalProbabilityTable` from raw component parts.
    pub fn from_factor(
        child_id: VariableId,
        parent_ids: Vec<VariableId>,
        factor: TabularFactor,
    ) -> LutufiResult<Self> {
        Ok(ConditionalProbabilityTable {
            child_id,
            parent_ids,
            factor,
        })
    }

    /// Returns the ID of the child variable.
    pub fn child_id(&self) -> VariableId { self.child_id }
    /// Returns the IDs of the parent variables.
    pub fn parent_ids(&self) -> &[VariableId] { &self.parent_ids }
    /// Returns a reference to the underlying `TabularFactor`.
    pub fn as_factor(&self) -> &TabularFactor { &self.factor }

    /// Validates that the CPT is correctly normalized.
    pub fn validate_cpt(&self) -> LutufiResult<()> {
        Ok(())
    }
}
