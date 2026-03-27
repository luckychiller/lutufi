use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::core::domain::Domain;

/// A unique identifier for a variable within a model.
/// Wraps a UUID to guarantee uniqueness across all models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct VariableId(Uuid);

impl VariableId {
    /// Generate a new unique variable ID.
    pub fn new() -> Self {
        VariableId(Uuid::new_v4())
    }

    /// Get the underlying UUID value.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for VariableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for VariableId {
    fn default() -> Self {
        Self::new()
    }
}

/// A named random variable with a domain of possible values.
///
/// Variables are the fundamental building blocks of probabilistic graphical models.
/// Each variable has a unique ID, a human-readable name, and a domain that defines
/// what values the variable can take.
///
/// # Design note
/// The ID and name are separate by design. The name is for human readability;
/// the ID is for programmatic identity. Two variables can share a name in
/// different models, but their IDs are always unique.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    /// Globally unique identifier. Never changes after creation.
    pub(crate) id: VariableId,
    /// Human-readable name. Must be unique within a model.
    pub(crate) name: String,
    /// The set of values this variable can take.
    pub(crate) domain: Domain,
}

impl Variable {
    /// Create a new variable with a given name and domain.
    ///
    /// The ID is generated automatically and guaranteed to be unique.
    ///
    pub fn new(name: impl Into<String>, domain: Domain) -> Self {
        Variable {
            id: VariableId::new(),
            name: name.into(),
            domain,
        }
    }

    /// The unique identifier of this variable.
    pub fn id(&self) -> VariableId {
        self.id
    }

    /// The human-readable name of this variable.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The domain of this variable.
    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    /// Validate that a value is within this variable's domain.
    ///
    /// # Errors
    /// Returns `LutufiError::ValueNotInDomain` if the value is invalid.
    pub fn validate_value(&self, value: &str) -> crate::core::error::LutufiResult<()> {
        if self.domain.contains(value) {
            Ok(())
        } else {
            let valid_values = match &self.domain {
                Domain::Discrete { states } => format!("{:?}", states),
                Domain::Binary => "['false', 'true']".to_string(),
                Domain::Continuous { lower, upper } => format!("range [{:?}, {:?}]", lower, upper),
            };
            Err(crate::core::error::LutufiError::ValueNotInDomain {
                value: value.to_string(),
                variable: self.name.clone(),
                valid_values,
            })
        }
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable({}, domain={:?})", self.name, self.domain)
    }
}


