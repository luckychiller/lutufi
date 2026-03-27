//! The set of possible values a random variable can take.
//!
//! # Variants
//!
//! - `Discrete`: A finite set of named states. The order of states is significant
//!   and determines the indexing of CPT entries.
//! - `Binary`: A performance-optimized special case of Discrete with exactly
//!   two states, always named `["false", "true"]` internally.
//! - `Continuous`: A real-valued domain with optional lower and upper bounds.
//!   Used for Gaussian nodes and continuous factor potentials.


use serde::{Deserialize, Serialize};
use crate::core::error::{LutufiError, LutufiResult};

/// The set of possible values a random variable can take.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Domain {
    /// A finite set of named states.
    Discrete { 
        /// The list of state names in order.
        states: Vec<String> 
    }, 
    /// A performance-optimized special case of Discrete with states ["false", "true"].
    Binary,
    /// A real-valued domain with optional lower and upper bounds.
    Continuous { 
        /// Optional lower bound (inclusive).
        lower: Option<f64>, 
        /// Optional upper bound (inclusive).
        upper: Option<f64> 
    },
}

impl Domain {
    /// Create a new discrete domain from a list of state names.
    ///
    /// # Errors
    /// Returns `LutufiError::EmptyDomain` if `states` is empty.
    ///
    pub fn discrete(states: Vec<impl Into<String>>) -> LutufiResult<Self> {
        let states: Vec<String> = states.into_iter().map(|s| s.into()).collect();
        if states.is_empty() {
            return Err(LutufiError::EmptyDomain {
                name: "unknown".to_string(), // Caller provides name in context
            });
        }
        Ok(Domain::Discrete { states })
    }

    /// Create a binary domain. States are always ["false", "true"].
    pub fn binary() -> Self {
        Domain::Binary
    }

    /// Create a continuous domain with optional bounds.
    pub fn continuous(lower: Option<f64>, upper: Option<f64>) -> Self {
        Domain::Continuous { lower, upper }
    }

    /// The number of states in a discrete domain.
    /// Returns `None` for continuous domains (uncountably infinite).
    pub fn size(&self) -> Option<usize> {
        match self {
            Domain::Discrete { states } => Some(states.len()),
            Domain::Binary => Some(2),
            Domain::Continuous { .. } => None,
        }
    }

    /// Check whether a string value is valid for this domain.
    ///
    /// For discrete domains, checks exact membership.
    /// For binary domains, checks "false" and "true".
    /// For continuous domains, checks whether the string parses as f64
    /// and falls within the optional bounds.
    pub fn contains(&self, value: &str) -> bool {
        match self {
            Domain::Discrete { states } => states.iter().any(|s| s == value),
            Domain::Binary => value == "false" || value == "true",
            Domain::Continuous { lower, upper } => {
                if let Ok(v) = value.parse::<f64>() {
                    let above_lower = lower.map_or(true, |l| v >= l);
                    let below_upper = upper.map_or(true, |u| v <= u);
                    above_lower && below_upper
                } else {
                    false
                }
            }
        }
    }

    /// The index of a state in a discrete domain.
    /// Used to index into CPT tables.
    ///
    /// # Returns
    /// `Some(index)` if the state exists, `None` if not found or domain is continuous.
    pub fn index_of(&self, state: &str) -> Option<usize> {
        match self {
            Domain::Discrete { states } => states.iter().position(|s| s == state),
            Domain::Binary => match state {
                "false" => Some(0),
                "true" => Some(1),
                _ => None,
            },
            Domain::Continuous { .. } => None,
        }
    }

    /// All states of a discrete domain, in order.
    /// Returns `None` for continuous domains.
    pub fn states(&self) -> Option<&[String]> {
        match self {
            Domain::Discrete { states } => Some(states),
            Domain::Binary => None, // Special case handled by caller
            Domain::Continuous { .. } => None,
        }
    }

    /// Whether this is a discrete (countable) domain.
    pub fn is_discrete(&self) -> bool {
        matches!(self, Domain::Discrete { .. } | Domain::Binary)
    }
}

/// The canonical state list for binary domains.
pub const BINARY_STATES: [&str; 2] = ["false", "true"];


