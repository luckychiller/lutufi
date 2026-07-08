use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::domain::Domain;
use crate::core::error::{LutufiError, LutufiResult};

/// The current version string for the LMF format.
pub const LMF_CURRENT_VERSION: &str = "1.0.0";

/// The type of probabilistic graphical model stored in an LMF document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    /// A directed acyclic graph model.
    BayesianNetwork,
    /// An undirected graphical model.
    MarkovRandomField,
    /// A Bayesian network extended over time slices.
    DynamicBayesianNetwork,
    /// A factor graph representation.
    FactorGraph,
}

/// Metadata describing the LMF document's origin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfMetadata {
    /// The human-readable name of the model.
    pub name: String,
    /// The version of lutufi that produced this file.
    pub lutufi_version: String,
    /// ISO-8601 timestamp when the document was created.
    pub created_at: String,
    /// Optional author of the model.
    pub author: Option<String>,
    /// Optional free-text description of the model.
    pub description: Option<String>,
}

/// The graph structure of the model, containing its variables and edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfGraph {
    /// All variables in the graph.
    pub variables: Vec<LmfVariable>,
    /// Directed edges stored as `[parent, child]` pairs.
    pub edges: Vec<[String; 2]>,
    /// Whether the edges represent causal mechanisms (set via
    /// `BayesianNetwork::mark_as_causal()`), gating `do()`/identification
    /// queries. Defaults to `false` so documents written before this field
    /// existed still deserialize correctly.
    #[serde(default)]
    pub is_causal: bool,
}

/// A named variable with an associated domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfVariable {
    /// The unique name of the variable.
    pub name: String,
    /// The domain (state space) of the variable.
    pub domain: LmfDomain,
}

/// The domain (state space) of a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LmfDomain {
    /// A finite set of named states.
    Discrete {
        /// The list of named states for this discrete variable.
        states: Vec<String>,
    },
    /// A binary domain with states "true" and "false".
    Binary,
    /// A continuous real-valued interval, optionally bounded.
    Continuous {
        /// Optional lower bound of the continuous interval.
        lower: Option<f64>,
        /// Optional upper bound of the continuous interval.
        upper: Option<f64>,
    },
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

/// The parameters of the model, consisting of conditional probability tables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfParameters {
    /// All conditional probability distributions in the model.
    pub cpds: Vec<LmfCpd>,
}

/// A single conditional probability distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfCpd {
    /// The name of the child variable.
    pub child: String,
    /// Names of the parent variables (empty for a marginal distribution).
    pub parents: Vec<String>,
    /// Flattened probability table (row-major order).
    pub table: Vec<f64>,
    /// Whether the table values are stored as log-probabilities.
    pub log_space: bool,
}

/// Hard evidence assignments for observed variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfEvidence {
    /// Mapping from variable name to its observed state.
    pub assignments: HashMap<String, String>,
}

/// Settings that control how inference is performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfInferenceSettings {
    /// The inference algorithm to use (e.g., "variable_elimination", "junction_tree").
    pub algorithm: String,
    /// Maximum number of iterations for iterative algorithms.
    pub max_iterations: Option<usize>,
    /// Convergence threshold for iterative algorithms.
    pub tolerance: Option<f64>,
}

/// The result of an inference query on the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfResults {
    /// Variables that were queried during inference.
    pub query_variables: Vec<String>,
    /// The algorithm that produced these results.
    pub algorithm_used: String,
    /// Wall-clock time spent performing inference, in milliseconds.
    pub computation_time_ms: u64,
    /// Log-likelihood of the evidence under the model.
    pub log_likelihood: f64,
    /// Marginal probabilities for each queried variable (maps name → probabilities).
    pub marginals: HashMap<String, Vec<f64>>,
}

/// A complete LMF (lutufi model format) document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LmfDocument {
    /// The LMF format version used by this document.
    pub format_version: String,
    /// Metadata about the document.
    pub metadata: LmfMetadata,
    /// The type of probabilistic model.
    pub model_type: ModelType,
    /// The graph structure of the model.
    pub graph: LmfGraph,
    /// The parameters (CPDs) of the model.
    pub parameters: LmfParameters,
    /// Optional evidence observations.
    pub evidence: Option<LmfEvidence>,
    /// Optional inference settings.
    pub inference_settings: Option<LmfInferenceSettings>,
    /// Optional inference results.
    pub results: Option<LmfResults>,
}

/// The result of verifying an LMF document against the format spec.
#[derive(Debug, Clone)]
pub struct VerifyReport {
    /// Whether all checks passed.
    pub passed: bool,
    /// Individual verification checks.
    pub checks: Vec<VerifyCheck>,
}

/// A single verification check performed during validation.
#[derive(Debug, Clone)]
pub struct VerifyCheck {
    /// The name of the check.
    pub name: String,
    /// Whether this check passed.
    pub passed: bool,
    /// A human-readable description of the check outcome.
    pub detail: String,
}

/// Reshape a flat probability table into a 2-D matrix (child-state × parent-config).
///
/// The flat table is stored child-innermost (idx = parent_config * child_size
/// + child_state), and `ConditionalProbabilityTable::from_values` expects one
/// row per child state and one column per parent configuration.
pub fn flatten_to_2d(values: &[f64], child_size: usize, parent_configs: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0f64; parent_configs.max(1)]; child_size];
    for pc in 0..parent_configs.max(1) {
        for cs in 0..child_size {
            let idx = pc * child_size + cs;
            if idx < values.len() {
                matrix[cs][pc] = values[idx];
            }
        }
    }
    matrix
}
