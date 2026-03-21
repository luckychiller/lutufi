//! The master error type for all Lutufi operations.
//! Every public function returns Result<T, LutufiError>.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LutufiError {

    // Variable and Domain Errors 

    #[error(
        "Variable '{name}' already exists in this model.\n\
         Hint: Each variable must have a unique name. \
         If you want to update a variable, remove it first with remove_node()."
    )]
    VariableAlreadyExists { name: String },

    #[error(
        "Variable '{name}' was not found in this model.\n\
         Hint: Check that the variable name is spelled correctly. \
         Available variables: {available}"
    )]
    VariableNotFound { name: String, available: String },

    #[error(
        "Value '{value}' is not in the domain of variable '{variable}'.\n\
         Valid values are: {valid_values}\n\
         Hint: Domain values are case-sensitive."
    )]
    ValueNotInDomain {
        value: String,
        variable: String,
        valid_values: String,
    },

    #[error(
        "Domain cannot be empty. Variable '{name}' was given an empty domain.\n\
         Hint: A variable must have at least one possible state."
    )]
    EmptyDomain { name: String },

    // Graph and Structure Errors 

    #[error(
        "Adding edge '{from}' → '{to}' would create a cycle in this Bayesian Network.\n\
         Detected cycle: {cycle}\n\
         Hint: Bayesian Networks must be acyclic (DAGs). \
         If your domain has feedback loops, consider using a \
         DynamicBayesianNetwork (DBN) which models cycles across time steps, \
         or a MarkovRandomField for symmetric undirected relationships."
    )]
    CyclicGraph {
        from: String,
        to: String,
        cycle: String,
    },

    #[error(
        "Cannot add edge from '{from}' to '{to}': node '{missing}' does not exist.\n\
         Hint: Add all nodes before adding edges between them."
    )]
    EdgeToMissingNode { from: String, to: String, missing: String },

    // CPT and Factor Errors 

    #[error(
        "CPT for variable '{variable}' does not sum to 1 for parent \
         configuration [{parent_config}].\n\
         Expected sum: 1.0, Got: {actual_sum:.6}\n\
         Hint: Each column of a CPT must sum to exactly 1.0 (within 1e-6 tolerance). \
         Check your probability values."
    )]
    CptDoesNotNormalize {
        variable: String,
        parent_config: String,
        actual_sum: f64,
    },

    #[error(
        "CPT for variable '{variable}' has wrong shape.\n\
         Expected: {expected_shape} (based on parent domains × child domain)\n\
         Got: {actual_shape}\n\
         Hint: CPT rows correspond to child states, columns to parent configurations."
    )]
    CptWrongShape {
        variable: String,
        expected_shape: String,
        actual_shape: String,
    },

    #[error(
        "CPT for variable '{variable}' references parents {cpt_parents:?},\n\
         but the graph shows parents {graph_parents:?}.\n\
         Hint: The CPT parent order must match the graph edge order exactly."
    )]
    CptParentMismatch {
        variable: String,
        cpt_parents: Vec<String>,
        graph_parents: Vec<String>,
    },

    #[error(
        "Variable '{variable}' has no CPT set.\n\
         Hint: Call set_cpd('{variable}', ...) before running inference. \
         All variables in the network must have CPTs before inference can proceed."
    )]
    MissingCpt { variable: String },

    // Inference and Causal Errors 

    #[error(
        "Cannot run causal inference on a non-causal model.\n\
         You called do({variable} = {value}) but this model has not been \
         marked as a structural causal model.\n\
         Hint: If your edges represent direct causal mechanisms (not just \
         statistical dependencies), call model.mark_as_causal() first. \
         If you are unsure, do not mark the model as causal — correlation \
         does not imply causation."
    )]
    NonCausalModel { variable: String, value: String },

    #[error(
        "Cannot run inference: the model has not been validated.\n\
         Validation errors:\n{errors}\n\
         Hint: Call model.validate() to see all issues, \
         or model.is_valid() to check without an error."
    )]
    InvalidModel { errors: String },

    // Numerical Errors

    #[error(
        "Numerical underflow detected during inference.\n\
         This should not happen because Lutufi uses log-space arithmetic.\n\
         If you see this error, please file a bug report with your model."
    )]
    NumericalUnderflow,

    #[error(
        "Factor product produced a result with infinite log-probability.\n\
         This usually means a probability of exactly 0 was set somewhere \
         and then evidence was set that requires it to be non-zero.\n\
         Hint: Check for zero entries in your CPTs where evidence requires a non-zero value."
    )]
    InfiniteLogProbability,

    // I/O Errors 

    #[error("Failed to serialize model: {reason}")]
    SerializationError { reason: String },

    #[error("Failed to deserialize model: {reason}\nHint: Check that the file is a valid Lutufi model file (.lmf).")]
    DeserializationError { reason: String },

    // Internal Errors (should never reach users) 

    #[error("Internal error: {message}\nThis is a bug in Lutufi. Please file a bug report.")]
    InternalError { message: String },
}

/// Convenience alias used throughout the codebase.
pub type LutufiResult<T> = Result<T, LutufiError>;
