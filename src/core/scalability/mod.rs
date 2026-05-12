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

/// Sparse conditional probability table storage using COO/CSR format.
pub mod sparse_cpt;
/// Noisy-OR factor parameterization for binary variables with many parents.
pub mod noisy_or;
/// Context-specific independence (CSI) detection and CPT reduction.
pub mod csi;
/// Parallel Loopy Belief Propagation inference engine using rayon.
pub mod parallel_inference;
/// Lazy evaluation infrastructure for factor graphs.
pub mod lazy_evaluation;
/// GPU-accelerated compute backend via wgpu shaders.
pub mod wgpu_backend;
/// Memory-mapped (out-of-core) factor storage for large networks.
pub mod memory_mapped;
/// Chunked variable elimination with disk spilling for very large networks.
pub mod chunked_ve;
/// Streaming DBN inference engine for online/real-time processing.
pub mod streaming;

pub use sparse_cpt::SparseCptStorage;
pub use noisy_or::NoisyOrFactor;
pub use csi::ContextSpecificIndependence;
pub use parallel_inference::ParallelLBPEngine;
pub use lazy_evaluation::{LazyFactor, LazyFactorGraph};
pub use memory_mapped::MemoryMappedFactorStore;
pub use wgpu_backend::WgpuBackend;
pub use chunked_ve::ChunkedVariableElimination;
pub use streaming::StreamingDBNEngine;
