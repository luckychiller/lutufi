use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    variable::{Variable, VariableId},
};

/// Manages variables and their identifiers within a network.
/// Handles the mapping between human-readable names and unique UUIDs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkRegistry {
    /// Map from variable ID to the Variable object.
    variables: HashMap<VariableId, Variable>,
    /// Map from variable name to its unique ID.
    name_to_id: HashMap<String, VariableId>,
}

impl NetworkRegistry {
    /// Create a new, empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new variable, returning a reference to it.
    pub fn add_variable(&mut self, name: &str, domain: crate::core::domain::Domain) -> LutufiResult<&Variable> {
        if self.name_to_id.contains_key(name) {
            return Err(LutufiError::VariableAlreadyExists {
                name: name.to_string(),
            });
        }

        let var = Variable::new(name, domain);
        let id = var.id();
        self.name_to_id.insert(name.to_string(), id);
        self.variables.insert(id, var);

        self.variables.get(&id).ok_or_else(|| LutufiError::InternalError {
            message: format!("Registry inconsistency: variable '{}' was just inserted but not found", name),
        })
    }

    /// Remove a variable by name, returning its ID.
    pub fn remove_variable(&mut self, name: &str) -> LutufiResult<VariableId> {
        let id = self.id_of(name)?;
        self.variables.remove(&id);
        self.name_to_id.remove(name);
        Ok(id)
    }

    /// Look up a variable's unique ID by its human-readable name.
    pub fn id_of(&self, name: &str) -> LutufiResult<VariableId> {
        self.name_to_id.get(name).copied().ok_or_else(|| {
            let available = self.name_to_id.keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            LutufiError::VariableNotFound {
                name: name.to_string(),
                available,
            }
        })
    }

    /// Get a variable by its unique ID, or `None` if not found.
    pub fn variable(&self, id: &VariableId) -> Option<&Variable> {
        self.variables.get(id)
    }

    /// Get a variable by its human-readable name.
    pub fn variable_by_name(&self, name: &str) -> LutufiResult<&Variable> {
        let id = self.id_of(name)?;
        self.variables.get(&id).ok_or_else(|| LutufiError::InternalError {
            message: format!("Registry inconsistency: variable '{}' not found after id lookup", name),
        })
    }

    /// Get all variable names in the registry.
    pub fn nodes(&self) -> Vec<&str> {
        self.variables.values().map(|v| v.name()).collect()
    }

    /// Get a reference to the internal variable map.
    pub fn variables(&self) -> &HashMap<VariableId, Variable> {
        &self.variables
    }

    /// The number of variables in the registry.
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Check whether a variable with the given ID exists.
    pub fn contains_id(&self, id: &VariableId) -> bool {
        self.variables.contains_key(id)
    }
}
