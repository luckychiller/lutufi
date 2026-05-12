pub mod scope;
pub mod tabular;
pub mod cpt;
pub mod utils;

pub use scope::Scope;
pub use tabular::{Factor, TabularFactor, PotentialFunction};
pub use cpt::ConditionalProbabilityTable;
pub use utils::{log_sum_exp, multi_index_from_flat, project_indices};
