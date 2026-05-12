use std::collections::HashMap;

use crate::core::{
    error::LutufiWarning,
    factor::TabularFactor,
    variable::VariableId,
};

/// Options for DBN inference.
#[derive(Debug, Clone)]
pub struct DBNInferenceOptions {
    /// Maximum number of time steps to process in temporal inference.
    pub max_time_steps: usize,
    /// Convergence tolerance threshold for iterative inference algorithms.
    pub tolerance: f64,
}

impl Default for DBNInferenceOptions {
    fn default() -> Self {
        DBNInferenceOptions { max_time_steps: 1000, tolerance: 1e-8 }
    }
}

/// Result of DBN inference operations.
#[derive(Debug, Clone)]
pub struct DBNInferenceResult {
    /// Time step indices for which marginals were computed.
    pub time_steps: Vec<usize>,
    /// Marginal distributions for each variable at each time step.
    pub marginals: Vec<HashMap<VariableId, TabularFactor>>,
    /// Log probability (evidence) of the observation sequence.
    pub log_evidence: f64,
    /// Name of the algorithm used for inference.
    pub algorithm: String,
    /// Time spent performing inference computation.
    pub computation_time: std::time::Duration,
    /// Warnings generated during inference (e.g., convergence issues).
    pub warnings: Vec<LutufiWarning>,
}
