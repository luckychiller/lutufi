use serde::{Deserialize, Serialize};
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    variable::{Variable, VariableId},
};
use super::utils::multi_index_from_flat;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scope {
    pub(crate) variables: Vec<VariableId>,
    pub(crate) sizes: Vec<usize>,
}

impl Scope {
    pub fn new(variables: Vec<&Variable>) -> Self {
        let mut sorted_vars: Vec<&Variable> = variables.to_vec();
        sorted_vars.sort_by_key(|v| v.id());

        let vars: Vec<VariableId> = sorted_vars.iter().map(|v| v.id()).collect();
        let sizes = sorted_vars.iter().map(|v| v.domain().size().unwrap_or(0)).collect();
        Scope { variables: vars, sizes }
    }

    pub fn from_ids_and_sizes(variables: Vec<VariableId>, sizes: Vec<usize>) -> Self {
        let mut vars_with_sizes: Vec<(VariableId, usize)> = variables.into_iter().zip(sizes.into_iter()).collect();
        vars_with_sizes.sort_by_key(|(var_id, _)| var_id.to_string());
        let (vars, sizes): (Vec<_>, Vec<_>) = vars_with_sizes.into_iter().unzip();
        Scope { variables: vars, sizes }
    }

    pub fn len(&self) -> usize { self.variables.len() }
    pub fn is_empty(&self) -> bool { self.variables.is_empty() }
    pub fn num_entries(&self) -> usize { self.sizes.iter().product() }
    pub fn variable_ids(&self) -> &[VariableId] { &self.variables }
    pub fn sizes(&self) -> &[usize] { &self.sizes }

    pub fn contains(&self, var_id: &VariableId) -> bool {
        self.variables.binary_search(var_id).is_ok()
    }

    pub fn contains_all(&self, vars: &std::collections::HashSet<VariableId>) -> bool {
        vars.iter().all(|id| self.contains(id))
    }

    pub fn size_of(&self, var_id: &VariableId) -> Option<usize> {
        self.variables.binary_search(var_id).ok().map(|idx| self.sizes[idx])
    }

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
