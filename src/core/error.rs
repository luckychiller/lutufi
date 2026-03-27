//! The master error type for all Lutufi operations.
//! Every public function returns Result<T, LutufiError>.

use thiserror::Error;

/// The master error type for all Lutufi operations.
#[derive(Debug, Error)]
pub enum LutufiError {

    // Variable and Domain Errors 

    /// Raised when a variable with the same name already exists in the model.
    #[error("Variable with name '{name}' already exists.")]
    VariableAlreadyExists { 
        /// The name of the existing variable.
        name: String 
    },

    /// Raised when a variable name is referenced but not found in the model.
    #[error("Variable '{name}' not found. Available variables: {available}")]
    VariableNotFound { 
        /// The name that was searched for.
        name: String, 
        /// List of variable names that actually exist in the model.
        available: String 
    },

    /// Raised when an assignment or CPT uses a value not present in the variable's domain.
    #[error("Value '{value}' is not in the domain of variable '{variable}'. Valid values: {valid_values}")]
    ValueNotInDomain {
        /// The invalid value.
        value: String,
        /// The variable it was assigned to.
        variable: String,
        /// The correct values for this variable.
        valid_values: String,
    },

    /// Raised when a domain is created with zero states.
    #[error("Domain for variable '{name}' cannot be empty.")]
    EmptyDomain { 
        /// The variable name.
        name: String 
    },

    // Graph and Structure Errors

    /// Raised when adding an edge would create a cycle in a Directed Acyclic Graph (DAG).
    #[error("Adding edge {from} -> {to} would create a cycle: {cycle}")]
    CyclicGraph {
        /// Start node of the edge.
        from: String,
        /// End node of the edge.
        to: String,
        /// Path forming the cycle.
        cycle: String,
    },

    /// Raised when an edge refers to a node that has not been added to the graph.
    #[error("Cannot add edge {from} -> {to}: node '{missing}' is missing from the graph.")]
    EdgeToMissingNode { 
        /// Source node.
        from: String, 
        /// Target node.
        to: String, 
        /// The node that is missing.
        missing: String 
    },

    // Factor and CPT Errors

    /// Raised when a Conditional Probability Table (CPT) does not sum to 1.0.
    #[error("CPT for variable '{variable}' for parent configuration {parent_config} sums to {actual_sum}, not 1.0.")]
    CptDoesNotNormalize {
        /// The child variable.
        variable: String,
        /// The parent configuration string.
        parent_config: String,
        /// The actual sum of the row.
        actual_sum: f64,
    },

    /// Raised when the provided values for a CPT do not match the expected table shape.
    #[error("CPT for variable '{variable}' has wrong shape. Expected {expected_shape}, got {actual_shape}.")]
    CptWrongShape {
        /// The child variable.
        variable: String,
        /// The required shape.
        expected_shape: String,
        /// The shape provided.
        actual_shape: String,
    },

    /// Raised when a CPT's parent set does not match the graph's parent set for that node.
    #[error("CPT for variable '{variable}' references parents {cpt_parents:?}, but the graph shows parents {graph_parents:?}.")]
    CptParentMismatch {
        /// The child variable.
        variable: String,
        /// Parents listed in the CPT.
        cpt_parents: Vec<String>,
        /// Parents present in the graph.
        graph_parents: Vec<String>,
    },

    /// Raised when inference is called but a node is missing its CPT.
    #[error("Variable '{variable}' has no CPT. Call set_cpd() before running inference.")]
    MissingCpt { 
        /// The variable missing its table.
        variable: String 
    },

    // Causal Inference Errors

    /// Raised when a causal operation (like the do-operator) is called on a non-causal model.
    #[error("Cannot perform causal operation on variable '{variable}' with value '{value}': model is not marked as causal.")]
    NonCausalModel { 
        /// The variable.
        variable: String, 
        /// The value.
        value: String 
    },

    // Validation Errors

    /// Raised when the model is in an inconsistent state.
    #[error("Model is invalid:\n{errors}")]
    InvalidModel { 
        /// Summary of all validation failures.
        errors: String 
    },

    // Computational Errors

    /// Raised when a probability calculation results in zero due to precision limits.
    #[error("Numerical underflow in probability calculation.")]
    NumericalUnderflow,

    /// Raised when taking the log of zero.
    #[error("Attempted to compute log-probability of an event with zero probability.")]
    InfiniteLogProbability,

    // I/O and Serialization Errors

    /// Raised when serializing a model to a file or string fails.
    #[error("Failed to serialize model: {reason}")]
    SerializationError { 
        /// The cause of the failure.
        reason: String 
    },

    /// Raised when loading a model from a file or string fails.
    #[error("Failed to deserialize model: {reason}")]
    DeserializationError { 
        /// The cause of the failure.
        reason: String 
    },

    // System Errors

    /// Raised when an internal invariant is violated. Indicates a bug in Lutufi.
    #[error("Internal error: {message}\nThis is a bug in Lutufi. Please file a bug report.")]
    InternalError { 
        /// The error message.
        message: String 
    },
}

/// Convenience alias used throughout the codebase.
pub type LutufiResult<T> = Result<T, LutufiError>;
