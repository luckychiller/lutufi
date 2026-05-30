use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::domain::Domain;
use crate::core::error::{LutufiError, LutufiResult};

pub const LMF_CURRENT_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    BayesianNetwork,
    MarkovRandomField,
    DynamicBayesianNetwork,
    FactorGraph,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfMetadata {
    pub name: String,
    pub lutufi_version: String,
    pub created_at: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfGraph {
    pub variables: Vec<LmfVariable>,
    pub edges: Vec<[String; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfVariable {
    pub name: String,
    pub domain: LmfDomain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LmfDomain {
    Discrete { states: Vec<String> },
    Binary,
    Continuous { lower: Option<f64>, upper: Option<f64> },
}

impl From<&Domain> for LmfDomain {
    fn from(d: &Domain) -> Self {
        match d {
            Domain::Discrete { states } => LmfDomain::Discrete {
                states: states.clone(),
            },
            Domain::Binary => LmfDomain::Binary,
            Domain::Continuous { lower, upper } => LmfDomain::Continuous {
                lower: *lower,
                upper: *upper,
            },
        }
    }
}

impl TryFrom<&LmfDomain> for Domain {
    type Error = LutufiError;
    fn try_from(d: &LmfDomain) -> LutufiResult<Self> {
        match d {
            LmfDomain::Discrete { states } => {
                if states.is_empty() {
                    return Err(LutufiError::DeserializationError {
                        reason: "LMF domain has no states".to_string(),
                    });
                }
                Ok(Domain::Discrete {
                    states: states.clone(),
                })
            }
            LmfDomain::Binary => Ok(Domain::Binary),
            LmfDomain::Continuous { lower, upper } => Ok(Domain::Continuous {
                lower: *lower,
                upper: *upper,
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfParameters {
    pub cpds: Vec<LmfCpd>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfCpd {
    pub child: String,
    pub parents: Vec<String>,
    pub table: Vec<f64>,
    pub log_space: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfEvidence {
    pub assignments: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfInferenceSettings {
    pub algorithm: String,
    pub max_iterations: Option<usize>,
    pub tolerance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfResults {
    pub query_variables: Vec<String>,
    pub algorithm_used: String,
    pub computation_time_ms: u64,
    pub log_likelihood: f64,
    pub marginals: HashMap<String, Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfDocument {
    pub format_version: String,
    pub metadata: LmfMetadata,
    pub model_type: ModelType,
    pub graph: LmfGraph,
    pub parameters: LmfParameters,
    pub evidence: Option<LmfEvidence>,
    pub inference_settings: Option<LmfInferenceSettings>,
    pub results: Option<LmfResults>,
}

#[derive(Debug, Clone)]
pub struct VerifyReport {
    pub passed: bool,
    pub checks: Vec<VerifyCheck>,
}

#[derive(Debug, Clone)]
pub struct VerifyCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

pub fn flatten_to_2d(values: &[f64], child_size: usize, parent_configs: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0f64; child_size]; parent_configs.max(1)];
    for pc in 0..parent_configs.max(1) {
        for cs in 0..child_size {
            let idx = pc * child_size + cs;
            if idx < values.len() {
                matrix[pc][cs] = values[idx];
            }
        }
    }
    matrix
}
