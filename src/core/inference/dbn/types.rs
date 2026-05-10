use std::collections::HashMap;

use crate::core::{
    error::LutufiWarning,
    factor::TabularFactor,
    variable::VariableId,
};

/// Options for DBN inference.
#[derive(Debug, Clone)]
pub struct DBNInferenceOptions {
    pub max_time_steps: usize,
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
    pub time_steps: Vec<usize>,
    pub marginals: Vec<HashMap<VariableId, TabularFactor>>,
    pub log_evidence: f64,
    pub algorithm: String,
    pub computation_time: std::time::Duration,
    pub warnings: Vec<LutufiWarning>,
}
