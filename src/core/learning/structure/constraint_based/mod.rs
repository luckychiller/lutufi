/// Shared types for constraint-based learning (options, results, graph types).
pub mod types;
/// Skeleton discovery via conditional independence tests.
pub mod skeleton;
/// V-structure orientation for orienting colliders.
pub mod orientation;
/// Meeks' rule application for additional edge orientation.
pub mod meeks;
/// Network builder that assembles the final graph from oriented edges.
pub mod network;
/// Constraint-based structure learner (PC algorithm).
pub mod learner;

pub use types::{
    ConstraintBasedOptions, EdgeOrientation, FciResult, IndependenceTestType,
    PagEdgeMark, PagGraph, SkeletonResult, VStructureResult,
};
pub use skeleton::SkeletonDiscovery;
pub use orientation::VStructureOrientator;
pub use meeks::MeeksRuleApplier;
pub use network::ConstrainedNetworkBuilder;
pub use learner::ConstraintBasedLearner;
