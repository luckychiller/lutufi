use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    variable::{Variable, VariableId},
};
use super::scope::Scope;
use super::tabular::TabularFactor;
use super::utils::{multi_index_from_flat, project_indices};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalProbabilityTable {
    child_id: VariableId,
    parent_ids: Vec<VariableId>,
    factor: TabularFactor,
}

impl ConditionalProbabilityTable {
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

        for i in 0..num_rows {
            let mut sum = 0.0;
            for j in 0..num_cols {
                sum += values[i][j];
            }
            if (sum - 1.0).abs() > 1e-6 {
                return Err(LutufiError::CptDoesNotNormalize {
                    variable: child.name().to_string(),
                    parent_config: format!("index {}", i),
                    actual_sum: sum,
                });
            }
        }

        let mut flat_values = Vec::with_capacity(num_rows * num_cols);
        for i in 0..num_rows {
            for j in 0..num_cols {
                flat_values.push(values[i][j]);
            }
        }

        let original_vars: Vec<VariableId> = scope_vars.iter().map(|v| v.id()).collect();
        let original_sizes: Vec<usize> = scope_vars.iter().map(|v| v.domain().size().unwrap_or(0)).collect();
        let original_scope = Scope { variables: original_vars, sizes: original_sizes };

        let reordered_values = if original_scope.variable_ids() == scope.variable_ids() {
            flat_values
        } else {
            let mut reordered = vec![0.0; flat_values.len()];
            for i in 0..scope.num_entries() {
                let sorted_indices = multi_index_from_flat(i, scope.sizes());
                let original_index = project_indices(
                    &sorted_indices,
                    scope.variable_ids(),
                    original_scope.variable_ids(),
                    original_scope.sizes(),
                )?;
                reordered[i] = flat_values[original_index];
            }
            reordered
        };

        let factor = TabularFactor::from_values(scope, reordered_values)?;

        Ok(ConditionalProbabilityTable {
            child_id: child.id(),
            parent_ids: parents.iter().map(|v| v.id()).collect(),
            factor,
        })
    }

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

    pub fn child_id(&self) -> VariableId { self.child_id }
    pub fn parent_ids(&self) -> &[VariableId] { &self.parent_ids }
    pub fn as_factor(&self) -> &TabularFactor { &self.factor }

    pub fn validate_cpt(&self) -> LutufiResult<()> {
        Ok(())
    }
}
