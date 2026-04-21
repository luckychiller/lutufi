//! A (possibly partial) assignment of values to variables.
//!
//! Assignments are used throughout Lutufi to represent:
//! - **Evidence**: observed values provided to inference
//! - **States**: the current configuration when evaluating a factor
//! - **Queries**: partial assignments used to index factor tables
//!
//! An assignment is partial — not every variable needs to be assigned.
//! Unassigned variables are treated as latent during inference.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::variable::VariableId;

/// A mapping from variable IDs to their assigned values.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Assignment {
    values: HashMap<VariableId, String>,
}

impl Assignment {
    /// Create an empty assignment.
    pub fn new() -> Self {
        Assignment {
            values: HashMap::new(),
        }
    }

    /// Create an assignment from a list of (variable_id, value) pairs.
    pub fn from_pairs(pairs: impl IntoIterator<Item = (VariableId, String)>) -> Self {
        Assignment {
            values: pairs.into_iter().collect(),
        }
    }

    /// Set the value for a variable.
    ///
    /// If the variable was already assigned, the previous value is overwritten.
    pub fn set(&mut self, variable: VariableId, value: impl Into<String>) -> &mut Self {
        self.values.insert(variable, value.into());
        self
    }

    /// Get the assigned value for a variable.
    ///
    /// Returns `None` if the variable is not assigned.
    pub fn get(&self, variable: &VariableId) -> Option<&str> {
        self.values.get(variable).map(|s| s.as_str())
    }

    /// Get the discrete index of the assigned value for a variable.
    ///
    /// # Errors
    /// - `LutufiError::VariableNotFound` if the variable is not assigned.
    /// - `LutufiError::InternalError` if the value is not a valid integer index.
    pub fn get_discrete(&self, variable: &VariableId) -> crate::core::error::LutufiResult<usize> {
        self.get(variable)
            .ok_or_else(|| crate::core::error::LutufiError::VariableNotFound {
                name: variable.to_string(),
                available: self.values.keys().map(|id| id.to_string()).collect::<Vec<_>>().join(", "),
            })?
            .parse::<usize>()
            .map_err(|e| crate::core::error::LutufiError::InternalError {
                message: format!("Assignment value for {} is not a valid discrete index: {}", variable, e),
            })
    }

    /// Check whether a variable has been assigned.
    pub fn contains(&self, variable: &VariableId) -> bool {
        self.values.contains_key(variable)
    }

    /// Check whether a variable has been assigned.
    pub fn is_assigned(&self, variable: &VariableId) -> bool {
        self.values.contains_key(variable)
    }

    /// Set the discrete value index for a variable.
    pub fn set_discrete(&mut self, variable: VariableId, value: usize) -> crate::core::error::LutufiResult<&mut Self> {
        self.values.insert(variable, value.to_string());
        Ok(self)
    }

    /// Remove the assignment for a variable.
    /// Returns the previous value if it existed.
    pub fn unset(&mut self, variable: &VariableId) -> Option<String> {
        self.values.remove(variable)
    }

    /// The number of assigned variables.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the assignment is empty (no variables assigned).
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Iterate over all (variable_id, value) pairs in this assignment.
    pub fn iter(&self) -> impl Iterator<Item = (&VariableId, &String)> {
        self.values.iter()
    }

    /// Merge another assignment into this one.
    /// Values from `other` overwrite values in `self` for conflicting variables.
    pub fn merge(&mut self, other: &Assignment) -> &mut Self {
        for (id, value) in &other.values {
            self.values.insert(*id, value.clone());
        }
        self
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pairs: Vec<String> = self.values
            .iter()
            .map(|(id, val)| format!("{}: {}", id, val))
            .collect();
        write!(f, "Assignment{{{}}}", pairs.join(", "))
    }
}
