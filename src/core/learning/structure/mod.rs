//! Structure learning algorithms.
//!
//! Provides score-based and constraint-based algorithms for learning network structure.

pub mod score_based;
/// Constraint-based structure learning algorithms (PC, FCI).
pub mod constraint_based;

pub use score_based::{ScoreBasedLearner, ScoreType};
pub use constraint_based::{
    ConstraintBasedLearner, IndependenceTestType, ConstraintBasedOptions,
    SkeletonDiscovery, VStructureOrientator, MeeksRuleApplier, ConstrainedNetworkBuilder,
    EdgeOrientation, SkeletonResult, VStructureResult,
    FciResult, PagGraph, PagEdgeMark,
};
