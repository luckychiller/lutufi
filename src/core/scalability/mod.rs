//! Scalability module (Phase 10)
//!
//! Provides:
//! - Sparse CPT storage (COO/CSR format)
//! - Noisy-OR factor parameterization
//! - Context-Specific Independence (CSI)
//! - Parallel inference (LBP message passing, factor products)
//! - Lazy evaluation infrastructure
//! - Memory-mapped factor storage (out-of-core)
//! - Chunked variable elimination
//! - Streaming DBN inference

pub mod sparse_cpt;
pub mod noisy_or;
pub mod csi;
pub mod parallel_inference;
pub mod lazy_evaluation;
pub mod memory_mapped;
pub mod chunked_ve;
pub mod streaming;

pub use sparse_cpt::SparseCptStorage;
pub use noisy_or::NoisyOrFactor;
pub use csi::ContextSpecificIndependence;
pub use parallel_inference::ParallelLBPEngine;
pub use lazy_evaluation::{LazyFactor, LazyFactorGraph};
pub use memory_mapped::MemoryMappedFactorStore;
pub use chunked_ve::ChunkedVariableElimination;
pub use streaming::StreamingDBNEngine;
