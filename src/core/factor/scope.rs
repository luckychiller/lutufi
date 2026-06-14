use serde::{Deserialize, Serialize};
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    variable::{Variable, VariableId},
};
use super::utils::multi_index_from_flat;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Represents the scope of a factor — a set of variables with their domain sizes.
pub struct Scope {
    pub(crate) variables: Vec<VariableId>,
    pub(crate) sizes: Vec<usize>,
}

impl Scope {
    /// Creates a new `Scope` from a slice of `Variable` references.
    pub fn new(variables: Vec<&Variable>) -> Self {
        let vars: Vec<VariableId> = variables.iter().map(|v| v.id()).collect();
        let sizes: Vec<usize> = variables.iter().map(|v| v.domain().size().unwrap_or(0)).collect();
        Scope { variables: vars, sizes }
    }

    /// Creates a `Scope` from variable IDs and their corresponding domain sizes.
    pub fn from_ids_and_sizes(variables: Vec<VariableId>, sizes: Vec<usize>) -> Self {
        Scope { variables, sizes }
    }

    /// Returns the number of variables in the scope.
    pub fn len(&self) -> usize { self.variables.len() }
    /// Returns `true` if the scope contains no variables.
    pub fn is_empty(&self) -> bool { self.variables.is_empty() }
    /// Returns the total number of entries in the scope's table (product of sizes).
    pub fn num_entries(&self) -> usize { self.sizes.iter().product() }
    /// Returns a slice of the variable IDs in this scope.
    pub fn variable_ids(&self) -> &[VariableId] { &self.variables }
    /// Returns a slice of the domain sizes in this scope.
    pub fn sizes(&self) -> &[usize] { &self.sizes }

    /// Returns `true` if the given variable ID is present in this scope.
    pub fn contains(&self, var_id: &VariableId) -> bool {
        self.variables.contains(var_id)
    }

    /// Returns `true` if all variables in the set are present in this scope.
    pub fn contains_all(&self, vars: &std::collections::HashSet<VariableId>) -> bool {
        vars.iter().all(|id| self.contains(id))
    }

    /// Returns the domain size of the given variable, or `None` if not in scope.
    pub fn size_of(&self, var_id: &VariableId) -> Option<usize> {
        self.variables.iter().position(|id| id == var_id).map(|idx| self.sizes[idx])
    }

    /// Converts a flat index into an `Assignment` over the scope's variables.
    pub fn assignment_from_flat(&self, index: usize) -> LutufiResult<Assignment> {
        if index >= self.num_entries() {
            return Err(LutufiError::InternalError { message: "Index out of bounds for scope".to_string() });
        }

        let mut assignment = Assignment::new();
        let indices = multi_index_from_flat(index, &self.sizes);
        for (i, &var_id) in self.variables.iter().enumerate() {
            assignment.set_discrete(var_id, indices[i])?;
        }
        Ok(assignment)
    }
}
