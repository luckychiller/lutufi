/// Module for factor scope representation.
pub mod scope;
/// Module for tabular factor types (dense and sparse).
pub mod tabular;
/// Module for conditional probability tables.
pub mod cpt;
/// Module with utility functions for factor operations.
pub mod utils;

pub use scope::Scope;
pub use tabular::{Factor, TabularFactor, PotentialFunction};
pub use cpt::ConditionalProbabilityTable;
pub use utils::{log_sum_exp, multi_index_from_flat, project_indices};
