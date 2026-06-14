/// Causal model types.
pub mod types;
/// Causal criteria (backdoor, frontdoor).
pub mod criteria;
/// Causal interventions and counterfactuals.
pub mod intervention;
/// Causal effect identification algorithms.
pub mod identification;

pub use types::{CausalModel, IdentificationResult, IdentificationFormula};
