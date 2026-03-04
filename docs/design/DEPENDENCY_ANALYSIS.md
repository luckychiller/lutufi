# Dependency Analysis Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Dependency Philosophy](#dependency-philosophy)
3. [Core Dependencies](#core-dependencies)
4. [Linear Algebra Libraries](#linear-algebra-libraries)
5. [Sparse Matrix Libraries](#sparse-matrix-libraries)
6. [Random Number Generation](#random-number-generation)
7. [Serialization Libraries](#serialization-libraries)
8. [Compression Libraries](#compression-libraries)
9. [Build Dependencies](#build-dependencies)
10. [Testing Dependencies](#testing-dependencies)
11. [Documentation Dependencies](#documentation-dependencies)
12. [Visualization Dependencies](#visualization-dependencies)
13. [Dependency Risk Assessment](#dependency-risk-assessment)
14. [Version Pinning Strategy](#version-pinning-strategy)
15. [Supply Chain Security](#supply-chain-security)
16. [Vendoring Strategy](#vendoring-strategy)
17. [Future Dependency Evolution](#future-dependency-evolution)
18. [How Lutufi Manages Dependencies](#how-lutufi-manages-dependencies)
19. [Key References](#key-references)

---

## Executive Summary

This document provides a comprehensive analysis of the dependencies for Lutufi, a library that unifies Bayesian networks with social and economic network analysis, authored by Wasswa Lutufi Sebbanja and licensed under Apache 2.0. Lutufi is implemented primarily in Rust with Python bindings, and its dependency strategy reflects the dual requirements of high-performance scientific computing and ease of deployment in diverse environments.

The dependency philosophy centers on minimalism: the core library depends only on essential libraries required for its primary functionality. Optional dependencies provide extended capabilities (advanced linear algebra, specialized algorithms, visualization) without burdening users who don't need them. This approach minimizes binary size, compilation time, security surface area, and supply chain risk while maintaining flexibility for users with diverse requirements.

The analysis covers all categories of dependencies: core Rust crates for data structures and algorithms, linear algebra libraries for numerical computation, sparse matrix libraries for efficient network representations, random number generators for probabilistic algorithms, serialization libraries for data persistence, compression libraries for storage efficiency, build tools, testing frameworks, documentation generators, and visualization libraries. Each dependency is assessed for maturity, maintenance status, license compatibility, and supply chain risk.

Lutufi employs a sophisticated strategy for managing dependency risk, including automated vulnerability scanning, reproducible builds, selective vendoring of critical dependencies, and careful version pinning. The document outlines specific procedures for monitoring security advisories, responding to vulnerabilities, and evolving the dependency graph as better alternatives emerge.

---

## Dependency Philosophy

### Minimal Dependencies Principle

Lutufi's dependency philosophy is grounded in the principle of minimalism. Every dependency added to the project carries costs: increased compile times, larger binary sizes, expanded security surface area, licensing complexity, and maintenance burden when dependencies evolve or become unmaintained. Therefore, each dependency must justify its inclusion through clear value proposition and necessity.

**Justification Requirements:** Before adding a dependency, the following questions must be answered affirmatively:
- Does this dependency provide functionality that cannot be reasonably implemented within Lutufi?
- Is the functionality essential for core library operations or a widely-used optional feature?
- Does the dependency's maintenance status indicate long-term viability?
- Is the dependency's license compatible with Lutufi's Apache 2.0 license?
- Does the benefit outweigh the costs in terms of binary size, compile time, and complexity?

**Dependency Categories:** Dependencies are categorized by necessity:
- **Core Dependencies:** Required for basic library functionality. These are included unconditionally.
- **Optional Dependencies:** Provide extended functionality. Users enable these via feature flags.
- **Development Dependencies:** Only needed for building, testing, and documentation. Not included in releases.

### Optional Dependencies for Extensions

Not all users need all features. A researcher using Lutufi for basic Bayesian network inference on small networks has different requirements than a financial institution running large-scale contagion simulations. Optional dependencies allow users to customize their installation:

**Feature Flags:** Rust's feature flag system enables conditional compilation of dependencies:

```toml
[features]
default = ["std", "serialize"]
std = []
serialize = ["serde", "serde_json"]
parallel = ["rayon"]
plotting = ["plotters"]
sparse = ["sprs"]
```

Users can customize their dependency footprint:
```toml
# Minimal installation
lutufi = { version = "0.5", default-features = false, features = ["std"] }

# Full-featured installation
lutufi = { version = "0.5", features = ["full"] }
```

**Benefits of Optional Dependencies:**
- Reduced binary size for users with minimal requirements
- Faster compilation when unused features are disabled
- Fewer transitive dependencies for security-conscious deployments
- Ability to exclude dependencies with complex system requirements (e.g., CUDA for GPU acceleration)

### Vendoring vs External Dependencies

Vendoring (including dependency source code directly in the project) is considered for specific situations:

**When to Vendor:**
- Critical dependencies with maintenance concerns (high bus factor, unclear future)
- Small, stable libraries where the vendored code is unlikely to need updates
- Dependencies with incompatible licensing that require license file modification
- Dependencies that need platform-specific patches for Lutufi's target environments

**When to Use External Dependencies:**
- Actively maintained libraries with regular security updates
- Large libraries where vendoring would significantly increase repository size
- Libraries with complex build requirements better handled by the package manager
- Dependencies with frequent releases where vendoring would create maintenance burden

**Hybrid Approach:** Lutufi uses a hybrid approach, vendoring select small, critical dependencies while using Cargo's dependency management for most libraries.

### Stability vs Innovation Trade-off

Dependency selection balances stability and innovation:

**Mature Dependencies:** For core functionality, prefer mature, widely-used libraries with stable APIs. These have been battle-tested, have extensive documentation, and are less likely to introduce breaking changes.

**Cutting-Edge Dependencies:** For specialized functionality where mature solutions don't exist, newer libraries may be acceptable. These should be isolated behind abstractions to allow replacement if they don't mature as expected.

**Abstraction Layers:** Dependencies are isolated behind internal abstractions where possible. If a dependency needs replacement, the abstraction boundary localizes changes.

---

## Core Dependencies

### Essential Rust Crates

The core functionality of Lutufi depends on a carefully selected set of Rust crates that provide foundational capabilities:

**std (Standard Library):**
- **Purpose:** Fundamental types, collections, I/O, threading
- **Necessity:** Essential for all Rust programs
- **Version Policy:** Tied to Rust compiler version (MSRV: 1.70.0)
- **Risk Assessment:** Minimal risk; maintained by Rust core team

**thiserror:**
- **Purpose:** Derive macro for std::error::Error
- **Usage:** Error type definitions throughout Lutufi
- **Version:** ^1.0
- **Rationale:** Eliminates error boilerplate, standard error handling pattern
- **Risk:** Low; widely used, stable API

**anyhow:**
- **Purpose:** Flexible error handling for application code
- **Usage:** Error propagation in binaries and tests
- **Version:** ^1.0
- **Rationale:** Ergonomic error handling without custom error types for non-library code
- **Risk:** Low; ubiquitous in Rust ecosystem

**log:**
- **Purpose:** Logging facade
- **Usage:** Structured logging throughout library
- **Version:** ^0.4
- **Rationale:** Ecosystem standard for logging, backend-agnostic
- **Risk:** Minimal; core ecosystem crate

### Data Structure Libraries

**hashbrown:**
- **Purpose:** High-performance HashMap and HashSet
- **Usage:** Node and edge lookup, deduplication
- **Version:** ^0.14
- **Rationale:** Better performance than std HashMap, used by std in nightly
- **Risk:** Low; widely used, API-compatible with std

**indexmap:**
- **Purpose:** HashMap with insertion order preservation
- **Usage:** Ordered node/edge attributes, deterministic iteration
- **Version:** ^2.0
- **Rationale:** Maintains order without Vec + HashMap combination
- **Risk:** Low; standard solution for ordered maps in Rust

**smallvec:**
- **Purpose:** Stack-allocated small vectors
- **Usage:** Node adjacency lists for low-degree nodes
- **Version:** ^1.11
- **Rationale:** Reduces allocations for common case of few neighbors
- **Risk:** Low; mature, widely used

### Algorithm Libraries

**petgraph:**
- **Purpose:** Graph data structure and algorithms
- **Usage:** Internal graph representation, basic graph algorithms
- **Version:** ^0.6
- **Rationale:** De facto standard graph library for Rust
- **Risk:** Medium; API has evolved significantly between versions

**rayon (optional):**
- **Purpose:** Data parallelism
- **Usage:** Parallel inference algorithms, parallel sampling
- **Version:** ^1.8
- **Rationale:** Easy parallelization with minimal code changes
- **Risk:** Low; widely used, stable API

### Version Constraints and Compatibility

**Semantic Versioning:** Lutufi follows semantic versioning for its own releases and expects dependencies to do the same. Version constraints in Cargo.toml use caret (^) requirements to allow compatible updates:

```toml
[dependencies]
petgraph = "^0.6"  # Compatible with 0.6.x, not 0.7.x
indexmap = "^2.0"  # Compatible with 2.x.x
```

**Minimum Supported Rust Version (MSRV):**
- Current MSRV: 1.70.0
- Rationale: Balances modern Rust features with distribution compatibility
- Policy: MSRV updates happen in minor releases with advance notice

**Compatibility Matrix:**

| Dependency | Current | MSRV Required | Update Policy |
|------------|---------|---------------|---------------|
| thiserror  | 1.0.x   | 1.56+         | Patch updates |
| anyhow     | 1.0.x   | 1.56+         | Patch updates |
| petgraph   | 0.6.x   | 1.60+         | Minor updates with API review |
| rayon      | 1.8.x   | 1.63+         | Patch updates |
| hashbrown  | 0.14.x  | 1.64+         | Patch updates |

### Core Dependency Justification

Each core dependency is justified by its specific contribution to Lutufi:

**petgraph vs Custom Implementation:** While Lutufi could implement its own graph structures, petgraph provides battle-tested implementations with efficient algorithms. The cost (one dependency) is justified by the savings in development and testing time, plus the benefit of community improvements.

**hashbrown vs std HashMap:** hashbrown offers better performance with the same API. Given that hash operations are frequent in network analysis, the performance improvement justifies the additional dependency.

**rayon Abstraction:** rayon enables parallelization with minimal code changes. The `par_iter()` method drops in for `iter()` with significant performance gains on multi-core systems.

---

## Linear Algebra Libraries

### BLAS/LAPACK Integration

Linear algebra operations are fundamental to network analysis, particularly for spectral methods, matrix factorization, and solving systems of equations. Lutufi integrates with BLAS (Basic Linear Algebra Subprograms) and LAPACK (Linear Algebra Package) for high-performance numerical computation.

**BLAS/LAPACK Providers:**
Different implementations optimize for different hardware:

**OpenBLAS:**
- **Description:** Open-source optimized BLAS implementation
- **Advantages:** Good performance across platforms, portable, liberal license (BSD)
- **Disadvantages:** Single-threaded by default (configurable), limited vendor optimization
- **Use Case:** Default for general distribution, development environments

**Intel MKL (Math Kernel Library):**
- **Description:** Intel's optimized math library
- **Advantages:** Excellent performance on Intel CPUs, multi-threaded, comprehensive functionality
- **Disadvantages:** Proprietary license (though free for use), Intel-specific optimizations
- **Use Case:** Production deployments on Intel hardware where maximum performance is critical

**Apple Accelerate:**
- **Description:** Apple's optimized framework for macOS/iOS
- **Advantages:** Highly optimized for Apple Silicon and Intel Macs, system framework
- **Disadvantages:** macOS only
- **Use Case:** macOS deployments

**BLIS:**
- **Description:** High-performance BLAS-like library
- **Advantages:** Modular design, good performance, portable
- **Disadvantages:** Less mature ecosystem than OpenBLAS
- **Use Case:** Research environments, alternative to OpenBLAS

**Rust Integration:** Lutufi uses the `blas-src` and `lapack-src` crates to abstract over BLAS/LAPACK providers:

```toml
[features]
default = ["openblas"]
openblas = ["openblas-src"]
intel-mkl = ["intel-mkl-src"]
accelerate = ["accelerate-src"]
```

### nalgebra (Rust)

For Rust-native linear algebra without external BLAS dependencies:

**nalgebra:**
- **Purpose:** Linear algebra library for Rust
- **Features:** Vectors, matrices, transformations, decompositions
- **Advantages:** Pure Rust (no external dependencies), const-generic dimensions, good ergonomics
- **Limitations:** Slower than optimized BLAS for large matrices
- **Version:** ^0.32

**Usage in Lutufi:**
- Small matrix operations where nalgebra's ergonomics outweigh performance concerns
- Const-generic matrix types for fixed-size problems
- Applications requiring pure Rust without system dependencies

### ndarray (Rust)

**ndarray:**
- **Purpose:** N-dimensional arrays for Rust
- **Features:** Similar to NumPy's ndarray, views, broadcasting
- **Advantages:** Flexible dimensionality, view semantics, ecosystem integration
- **Integration:** Works with BLAS through `ndarray-linalg`
- **Version:** ^0.15

**Usage in Lutufi:**
- Multi-dimensional arrays for factor tables in Bayesian networks
- Network adjacency tensors for multi-layer networks
- Interface with NumPy arrays in Python bindings

### Eigen (C++) Considerations

While Lutufi is implemented in Rust, some specialized algorithms may benefit from Eigen (C++ linear algebra library):

**Potential Integration:**
- Through C++ bindings for specific algorithms not available in Rust
- CXX or autocxx for safe C++ interop
- Considered only when Rust alternatives are insufficient

**Current Stance:** No direct Eigen dependency; nalgebra and ndarray provide sufficient functionality. Re-evaluated if specific use cases emerge.

### Linear Library Selection Strategy

**By Use Case:**

| Use Case | Library | Justification |
|----------|---------|---------------|
| Small matrices (<100x100) | nalgebra | Ergonomics, compile-time dimensions |
| Large dense matrices | BLAS/LAPACK | Performance |
| Sparse matrices | sprs (see below) | Memory efficiency |
| N-dimensional arrays | ndarray | Flexibility, NumPy compatibility |
| Mixed operations | Combination | Best tool for each job |

**Compile-Time vs Runtime Dimensions:**
- nalgebra supports const-generic dimensions, enabling compile-time size checks and optimizations
- ndarray uses runtime dimensions, providing flexibility for dynamic network sizes
- Lutufi uses both: nalgebra for fixed-size problems (e.g., 3D spatial networks), ndarray for general cases

---

## Sparse Matrix Libraries

### Sparse Matrix Requirements

Network adjacency matrices are typically sparse (most entries are zero). For large networks (millions of nodes), dense matrix representations are infeasible. Sparse matrix libraries provide memory-efficient storage and optimized algorithms for sparse structures.

### sprs (Rust)

**sprs:**
- **Purpose:** Sparse matrix library for Rust
- **Formats:** CSR (Compressed Sparse Row), CSC (Compressed Sparse Column), COO (Coordinate)
- **Operations:** Sparse matrix-vector multiplication, sparse matrix addition, conversion between formats
- **Version:** ^0.11
- **License:** MIT/Apache-2.0 (dual)

**Usage in Lutufi:**
- Adjacency matrix storage for large networks
- Sparse belief propagation operations
- Spectral computations on sparse Laplacian matrices

**Integration:**
```rust
use sprs::{CsMat, CsVec};

// Sparse adjacency matrix
let adjacency: CsMat<f64> = CsMat::zero((n_nodes, n_nodes));

// Efficient sparse matrix-vector multiplication
let beliefs = &adjacency * &initial_beliefs;
```

### SuiteSparse Integration

For advanced sparse matrix operations, Lutufi can interface with SuiteSparse (C library):

**SuiteSparse Components:**
- **UMFPACK:** Sparse LU factorization
- **CHOLMOD:** Sparse Cholesky factorization
- **SPQR:** Sparse QR factorization
- **KLU:** Sparse LU for circuit simulation matrices

**Integration Approach:**
- Optional dependency via FFI bindings
- Used for specialized applications requiring advanced sparse solvers
- Not included in default build due to complexity

### scipy.sparse Integration (Python)

In Python bindings, Lutufi integrates with scipy.sparse:

**Conversion:**
- `scipy.sparse.csr_matrix` ↔ Lutufi sparse matrix
- `scipy.sparse.csc_matrix` ↔ Lutufi sparse matrix
- `scipy.sparse.coo_matrix` ↔ Lutufi sparse matrix

**Zero-Copy Views:** Where possible, Lutufi creates views of scipy.sparse data without copying.

### Custom Sparse Implementations

For specialized network structures, Lutufi implements custom sparse representations:

**Adjacency Lists:** Standard graph adjacency list representation for fast neighbor access.

**Factor Graph Storage:** Specialized storage for factor graphs in Bayesian networks, optimizing for factor table access patterns.

**Compressed Structures:** Run-length encoded edge lists for networks with regular structure (e.g., grid graphs).

---

## Random Number Generation

### Deterministic RNG Requirements

Probabilistic algorithms in Lutufi (Gibbs sampling, stochastic inference, network generation) require random number generation. Scientific reproducibility demands deterministic, seedable random number generators.

**Requirements:**
- **Determinism:** Same seed produces same sequence across runs and platforms
- **Statistical Quality:** High-quality randomness for statistical validity
- **Performance:** Fast generation for Monte Carlo methods
- **Parallel Safety:** Independent streams for parallel sampling

### PCG (Permuted Congruential Generator)

**pcg_rand:**
- **Purpose:** PCG random number generation
- **Features:** Multiple PCG variants (PCG32, PCG64), different output functions
- **Advantages:** Excellent statistical quality, small state, fast, well-tested
- **Version:** ^0.13

**Usage:** Default RNG for most probabilistic algorithms due to balance of quality and performance.

### Xoshiro/Xoroshiro

**rand_xoshiro:**
- **Purpose:** Xoshiro/Xoroshiro RNGs
- **Features:** Multiple variants (Xoshiro256+, Xoroshiro128+)
- **Advantages:** Very fast, small state, good statistical properties
- **Use Case:** High-performance Monte Carlo where speed is critical

### Seed Management

**Seed Types:**
- `u64` seeds for single RNG instances
- `SeedSequence` for initializing multiple independent RNGs
- `getrandom` for cryptographically secure seed generation when needed

**Reproducibility:**
```rust
// Deterministic sampling
let rng = Pcg64::seed_from_u64(42);
let sample = network.gibbs_sample(evidence, 1000, rng);
// Same seed always produces same sample
```

**Parallel Sampling:**
```rust
// Independent RNGs for parallel chains
let seeds = SeedSequence::from_entropy().spawn_keys(4);
let rngs: Vec<_> = seeds.into_iter()
    .map(|s| Pcg64::from_seed(s))
    .collect();

// Each thread gets independent RNG
let samples: Vec<_> = rngs.into_par_iter()
    .map(|rng| network.gibbs_sample(evidence, 1000, rng))
    .collect();
```

### rand Crate Ecosystem

**rand:**
- **Purpose:** Rust's standard random number library
- **Features:** Traits for RNGs, distributions, shuffling
- **Version:** ^0.8
- **Status:** Core ecosystem crate, maintained by Rust team

**rand_distr:**
- **Purpose:** Probability distributions
- **Features:** Normal, Beta, Gamma, Dirichlet, and many others
- **Usage:** Probability distributions for Bayesian networks

**rand_chacha:**
- **Purpose:** ChaCha RNG (cryptographically secure)
- **Usage:** When cryptographic security is required

---

## Serialization Libraries

### serde (Rust)

**serde:**
- **Purpose:** Serialization framework for Rust
- **Approach:** Derive macros for automatic serialization implementations
- **Version:** ^1.0
- **Status:** De facto standard for Rust serialization

**Usage in Lutufi:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct BayesianNetwork {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    factors: Vec<Factor>,
}
```

### Format-Specific Crates

**serde_json:**
- **Purpose:** JSON serialization
- **Usage:** Human-readable network format, configuration files, API responses
- **Version:** ^1.0

**serde_yaml:**
- **Purpose:** YAML serialization
- **Usage:** Human-readable configuration, model specifications
- **Version:** ^0.9

**toml:**
- **Purpose:** TOML serialization
- **Usage:** Configuration files (following Rust ecosystem convention)
- **Version:** ^0.8

**bincode:**
- **Purpose:** Compact binary serialization
- **Usage:** Efficient storage of large networks
- **Version:** ^1.3

**postcard:**
- **Purpose:** Compact, schema-based binary serialization
- **Usage:** Embedded systems, constrained environments
- **Version:** ^1.0

### Protocol Buffers

**prost:**
- **Purpose:** Protocol Buffers for Rust
- **Features:** Code generation from .proto files, efficient binary format
- **Version:** ^0.12

**Usage:**
- Inter-language communication (Python, R, Julia bindings)
- Efficient storage with schema evolution support
- RPC communication for distributed inference

### FlatBuffers

**flatbuffers:**
- **Purpose:** Zero-copy serialization
- **Features:** Direct access to serialized data without parsing
- **Version:** ^23.0

**Usage:**
- Memory-mapped network storage
- Zero-overhead reading of large networks
- Games and real-time applications

### MessagePack

**rmp-serde:**
- **Purpose:** MessagePack serialization
- **Features:** Efficient binary format with JSON-like schema flexibility
- **Version:** ^1.1

**Usage:**
- Compact, fast serialization
- Language-agnostic format for data exchange

### Serialization Format Selection

| Format | Use Case | Human Readable | Size | Speed |
|--------|----------|----------------|------|-------|
| JSON | APIs, configs | Yes | Large | Medium |
| YAML | Complex configs | Yes | Large | Slow |
| TOML | Simple configs | Yes | Medium | Medium |
| bincode | Storage | No | Small | Fast |
| Protocol Buffers | Interop, RPC | No | Small | Fast |
| FlatBuffers | Zero-copy | No | Medium | Very Fast |
| MessagePack | General binary | No | Small | Fast |

---

## Compression Libraries

### Compression Strategy

Network data can be large, especially for temporal networks with many snapshots or large factor tables in Bayesian networks. Compression reduces storage and transfer costs.

**Optional Compression:** Compression is an optional feature, not a core dependency. Users enable compression support when needed.

### zstd

**zstd:**
- **Purpose:** Zstandard compression
- **Features:** High compression ratios, fast decompression, adjustable compression levels
- **Version:** ^0.13

**Advantages:**
- Excellent ratio/speed tradeoff
- Dictionary compression for repetitive network structures
- Widely supported format

**Usage:**
- Compressed network storage
- Compressed serialization streams
- Transfer compression for distributed systems

### lz4

**lz4:**
- **Purpose:** LZ4 compression
- **Features:** Extremely fast compression and decompression, lower ratios than zstd
- **Version:** ^1.24

**Advantages:**
- Speed-focused: GB/s compression/decompression
- Suitable for real-time compression
- Low memory overhead

**Usage:**
- Real-time compression in streaming scenarios
- When decompression speed is critical
- Temporary compression for intermediate results

### zlib

**flate2:**
- **Purpose:** DEFLATE/zlib/gzip compression
- **Features:** Ubiquitous format, good compatibility
- **Version:** ^1.0

**Usage:**
- Maximum compatibility (gzip is universally supported)
- When file size is less critical than compatibility

### Compression Integration

**Transparent Compression:**
```rust
// Automatic compression based on file extension
network.save("network.ltn.zst")?;  // Compressed with zstd
network.save("network.ltn")?;      // Uncompressed
```

**Compression Levels:**
```rust
// Trade compression ratio for speed
let config = SaveConfig::default()
    .with_compression(Compression::Zstd { level: 3 });  // Fast
let config = SaveConfig::default()
    .with_compression(Compression::Zstd { level: 19 }); // Maximum compression
```

---

## Build Dependencies

### Rust Build Toolchain

**Cargo:**
- **Purpose:** Rust's build system and package manager
- **Version:** Bundled with Rust (MSRV: 1.70.0)
- **Features:** Dependency resolution, compilation, testing, documentation

**Rustfmt:**
- **Purpose:** Code formatting
- **Usage:** Enforced via CI, editor integration

**Clippy:**
- **Purpose:** Linting
- **Usage:** CI lint checks, catches common mistakes and anti-patterns

### C/C++ Build Tools

When building with C/C++ dependencies (BLAS, SuiteSparse):

**CMake:**
- **Purpose:** Cross-platform C/C++ build system
- **Usage:** Building dependencies that use CMake

**PkgConfig:**
- **Purpose:** Query installed libraries
- **Usage:** Finding system BLAS/LAPACK installations

### Python Build Tools

**maturin:**
- **Purpose:** Build and publish Rust/Python hybrid packages
- **Usage:** Building Python wheels with Rust extensions
- **Version:** ^1.4

**setuptools-rust:**
- **Purpose:** Alternative to maturin, setuptools integration
- **Usage:** When setuptools-based build is preferred

**cibuildwheel:**
- **Purpose:** Build wheels across platforms
- **Usage:** CI wheel building for PyPI distribution

### Compiler Requirements

**Rust Compiler:**
- **Minimum:** 1.70.0 (MSRV)
- **Recommended:** Latest stable
- **Nightly:** Not required, but some optimizations available

**C Compiler:**
- **Required for:** BLAS/LAPACK dependencies
- **Supported:** GCC, Clang, MSVC

**Linking:**
- Static linking preferred for portability
- Dynamic linking for system libraries (BLAS)

---

## Testing Dependencies

### Unit Testing Framework

**Built-in test framework:**
- **Purpose:** Rust's built-in testing (`#[test]`)
- **Usage:** Primary unit testing mechanism
- **No dependency:** Part of Rust standard tooling

### Property-Based Testing

**proptest:**
- **Purpose:** Property-based testing (similar to Hypothesis in Python)
- **Features:** Automatic test case generation, shrinking
- **Version:** ^1.4

**Usage:**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn belief_propagation_converges(
        network in network_strategy(10..100),
        evidence in evidence_strategy()
    ) {
        let result = network.belief_propagation(&evidence);
        prop_assert!(result.is_converged() || result.iterations() < 1000);
    }
}
```

**quickcheck:**
- **Purpose:** Property-based testing (QuickCheck style)
- **Alternative to:** proptest
- **Usage:** When QuickCheck-style testing is preferred

### Fuzz Testing

**cargo-fuzz:**
- **Purpose:** Fuzz testing with libFuzzer
- **Usage:** Finding crashes and panics from random inputs

**afl.rs:**
- **Purpose:** American Fuzzy Lop integration
- **Usage:** Coverage-guided fuzzing

### Benchmark Testing

**Criterion:**
- **Purpose:** Statistics-driven benchmarking
- **Features:** Confidence intervals, regression detection
- **Version:** ^0.5

**Usage:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn inference_benchmark(c: &mut Criterion) {
    let network = create_test_network();
    c.bench_function("belief_propagation", |b| {
        b.iter(|| network.belief_propagation(black_box(&evidence)))
    });
}
```

**iai:**
- **Purpose:** Instruction-count based benchmarking
- **Advantages:** Deterministic, CI-friendly
- **Usage:** When noise-resistant benchmarking is needed

### Python Testing (for bindings)

**pytest:**
- **Purpose:** Python testing framework
- **Usage:** Testing Python bindings
- **Version:** ^7.4

**hypothesis:**
- **Purpose:** Property-based testing for Python
- **Usage:** Testing Python API invariants

**pytest-benchmark:**
- **Purpose:** Benchmark testing in pytest
- **Usage:** Performance regression tests for Python API

---

## Documentation Dependencies

### Rust Documentation

**rustdoc:**
- **Purpose:** Rust documentation generator
- **Usage:** API documentation from doc comments
- **No dependency:** Built into Rust toolchain

**Docs.rs:**
- **Purpose:** Hosts documentation for all crates.io crates
- **Usage:** Automatic documentation hosting

### Python Documentation

**Sphinx:**
- **Purpose:** Python documentation generator
- **Features:** RST and Markdown support, extensions, theming
- **Version:** ^7.1

**Extensions:**
- **sphinx-autodoc:** Automatic documentation from docstrings
- **sphinx-rtd-theme:** ReadTheDocs theme
- **myst-parser:** Markdown support in Sphinx
- **sphinx-gallery:** Example gallery generation

**MkDocs (alternative):**
- **Purpose:** Static site generator for project documentation
- **Features:** Markdown-based, simpler than Sphinx
- **Version:** ^1.5

### Math Rendering

**KaTeX:**
- **Purpose:** Fast math rendering
- **Advantages:** Server-side rendering option, fast client-side

**MathJax:**
- **Purpose:** Math rendering
- **Advantages:** Comprehensive LaTeX support

**Usage in Lutufi:** Documentation uses math extensively for probabilistic notation. Both KaTeX and MathJax are supported, with KaTeX as default for speed.

---

## Visualization Dependencies

### Visualization Philosophy

Visualization is an optional feature. Core Lutufi does not depend on visualization libraries; they are available through optional features or Python ecosystem integration.

### Rust Visualization

**plotters:**
- **Purpose:** Drawing library for Rust
- **Features:** Multiple backends (SVG, Cairo, WebAssembly), various plot types
- **Version:** ^0.3

**Usage:**
- Static plot generation in Rust
- SVG output for documentation
- WASM-compatible for web visualization

### Python Visualization (via bindings)

**Matplotlib:**
- **Purpose:** Python plotting library
- **Integration:** Standard Python visualization, used through Python bindings

**plotly:**
- **Purpose:** Interactive web-based visualization
- **Integration:** Interactive network visualizations in Jupyter

**graphviz:**
- **Purpose:** Graph visualization software
- **Integration:** DOT format export from Lutufi, rendered by graphviz

### Network Visualization

**Cytoscape.js (web):**
- **Purpose:** Web-based network visualization
- **Integration:** Via WebAssembly bindings for interactive browser visualization

---

## Dependency Risk Assessment

### Risk Assessment Framework

Each major dependency is assessed across multiple dimensions:

**Maturity:**
- **High:** Stable API, production use for years
- **Medium:** Established but still evolving
- **Low:** New or experimental

**Maintenance Status:**
- **Active:** Regular releases, responsive maintainers
- **Maintenance mode:** Bug fixes only, stable
- **Uncertain:** Infrequent updates, unclear future

**License Compatibility:**
- **Compatible:** MIT, Apache-2.0, BSD (permits proprietary use)
- **Copyleft:** GPL variants (may restrict usage)
- **Incompatible:** Proprietary, custom licenses

**Bus Factor:**
- **High (>5):** Multiple active maintainers
- **Medium (2-5):** Small team
- **Low (1):** Single maintainer

### Core Dependencies Risk Assessment

| Dependency | Maturity | Maintenance | License | Bus Factor | Overall Risk |
|------------|----------|-------------|---------|------------|--------------|
| thiserror | High | Active | MIT/Apache | Medium | Low |
| anyhow | High | Active | MIT/Apache | Medium | Low |
| log | High | Active | MIT/Apache | High | Low |
| petgraph | High | Active | MIT/Apache | Medium | Low |
| rayon | High | Active | MIT/Apache | High | Low |
| hashbrown | High | Active | MIT/Apache | High | Low |
| indexmap | High | Active | MIT/Apache | Medium | Low |
| serde | High | Active | MIT/Apache | High | Low |
| nalgebra | High | Active | BSD-3 | Medium | Low |
| ndarray | High | Active | MIT/Apache | Medium | Low |
| sprs | Medium | Active | MIT/Apache | Low | Medium |
| rand | High | Active | MIT/Apache | High | Low |
| proptest | High | Active | MIT/Apache | Medium | Low |
| criterion | High | Active | Apache-2.0 | Medium | Low |

### Linear Algebra Risk Assessment

| Dependency | Maturity | Maintenance | License | Bus Factor | Overall Risk |
|------------|----------|-------------|---------|------------|--------------|
| OpenBLAS | High | Active | BSD | High | Low |
| Intel MKL | High | Active | Proprietary* | High | Low* |
| BLIS | Medium | Active | BSD | Medium | Low |

*Intel MKL is proprietary but freely redistributable. License terms must be reviewed for specific deployments.

### Mitigation Strategies

**For Medium Risk Dependencies:**
- **sprs (bus factor):** Monitor for maintenance issues, consider contributing, maintain fork if necessary
- Monitor security advisories, have migration plan

**For All Dependencies:**
- Regular review of maintenance status
- Participation in community (reporting issues, contributing)
- Abstraction layers to facilitate replacement
- Version pinning for reproducibility

---

## Version Pinning Strategy

### Semantic Versioning Interpretation

Lutufi follows semantic versioning and expects dependencies to do the same:

**MAJOR:** Breaking API changes. Pin to specific major version.
**MINOR:** New features, backward compatible. Allow minor updates.
**PATCH:** Bug fixes, backward compatible. Allow patch updates.

### Pinning in Libraries vs Applications

**Libraries (like Lutufi):**
- Use caret (^) requirements: `petgraph = "^0.6"`
- Allow compatible updates (patch and minor for 0.x, patch for 1.x+)
- Don't use exact pinning (prevents dependency resolution)

**Applications:**
- Use Cargo.lock for reproducible builds
- Consider exact pinning for critical security-sensitive dependencies
- Update dependencies on a schedule with testing

### Cargo.lock Policy

**Library Crates:**
- Cargo.lock not committed for library crates
- Allows downstream users flexibility in dependency resolution

**Binary Crates:**
- Cargo.lock committed for reproducible builds
- Updated during release process after testing

### Update Cadence

**Patch Updates:**
- Applied promptly (security fixes)
- Monthly review and batch update otherwise

**Minor Updates:**
- Reviewed for compatibility
- Quarterly update cycle
- Changelog review before updating

**Major Updates:**
- Evaluated for breaking changes
- Migration plan created
- Updated during minor Lutufi releases

---

## Supply Chain Security

### Checksum Verification

**Cargo.lock Checksums:**
- Cargo verifies crate checksums on download
- Checksums stored in Cargo.lock
- Prevents tampering with published crates

**Vendoring for Audit:**
- Vendor dependencies for security audit
- Review vendored code for suspicious patterns
- Establish baseline of trusted code

### Dependency Scanning

**cargo-audit:**
- **Purpose:** Audit Cargo.lock for security vulnerabilities
- **Usage:** CI integration, pre-release checks
- **Database:** RustSec Advisory Database

```bash
# Check for known vulnerabilities
cargo audit
```

**cargo-deny:**
- **Purpose:** Policy enforcement for dependencies
- **Features:** License checking, security audit, banned crate lists
- **Usage:** CI policy enforcement

```toml
# deny.toml
[advisories]
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
```

**safety (safety-db):**
- **Purpose:** Alternative vulnerability database
- **Usage:** Supplementary to cargo-audit

### Reproducible Builds

**Deterministic Compilation:**
- Fixed Rust compiler version
- Fixed dependency versions (Cargo.lock)
- Same build environment (Docker containers)

**Verification:**
- Build hashes for verification
- Comparison across different build environments

### Vendor Response to Vulnerabilities

**Security Contact:**
- security@lutufi.org for vulnerability reports
- GPG key for encrypted communication

**Vulnerability Response Process:**
1. Receive report (acknowledge within 24 hours)
2. Assess severity and impact
3. Develop fix or workaround
4. Coordinate with upstream if dependency issue
5. Release patched version
6. Publish security advisory

---

## Vendoring Strategy

### When to Vendor

**Vendoring Criteria:**
1. **Critical Path:** Dependency is on critical path, unmaintained would block releases
2. **Small Size:** Library is small enough that vendoring doesn't bloat repository
3. **Stability:** API is stable, unlikely to need updates
4. **License Modification:** Need to modify license file or include additional notices
5. **Platform Patches:** Need platform-specific patches not accepted upstream

### Vendoring Process

**Vendor Directory:**
```
lutufi/
├── vendor/
│   ├── small_critical_lib/
│   │   ├── LICENSE
│   │   └── src/
│   └── another_lib/
```

**Cargo Configuration:**
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "vendored"

[source.vendored]
directory = "vendor"
```

### Vendored Dependencies in Lutufi

Currently vendored:
- None (using Cargo ecosystem exclusively)

Potential future vendoring:
- Small utility crates with high bus factor
- Customized versions of libraries

### Size Implications

**Repository Size:**
- Vendored dependencies increase repository size
- Git handles this reasonably well for stable code
- Consider git-subtree for large vendored deps

**Distribution Size:**
- Vendored source not included in distributed binaries
- Only affects source distribution

### Security Updates for Vendored Code

**Monitoring:**
- Watch upstream for security updates
- Subscribe to security advisories

**Update Process:**
1. Review upstream security fix
2. Apply to vendored copy
3. Verify fix with tests
4. Release updated Lutufi version

---

## Future Dependency Evolution

### Monitoring for Alternatives

**Continuous Evaluation:**
- Track emerging libraries in Rust ecosystem
- Evaluate for performance, ergonomics, maintenance
- Benchmark against current dependencies

**Evaluation Criteria:**
1. Performance improvement >20% for critical paths
2. Better API ergonomics
3. More active maintenance
4. Better license compatibility
5. Reduced dependency tree

### Migration Strategies

**Abstraction Layers:**
- Isolate dependencies behind internal traits
- Easier to swap implementations
- Example: RNG trait abstracts over PCG, Xoshiro, etc.

**Gradual Migration:**
- Feature flags for new implementation
- Side-by-side comparison
- Gradual cutover after validation

**Deprecation Cycles:**
- Mark old API as deprecated
- Maintain for one major version
- Remove in next major version

### Emerging Dependencies to Watch

**Linear Algebra:**
- **faer-rs:** New Rust-native linear algebra with BLAS-like performance
- **matrixmultiply:** Focused on matrix multiplication

**Graph Algorithms:**
- **pathfinding:** Additional graph algorithms
- **graphalgs:** Specialized graph algorithms

**Probabilistic Programming:**
- **peroxide:** Rust numerical library with statistics
- **rv:** Random variables and probability distributions

### Deprecation Planning

**Current Dependencies Under Review:**
- None currently scheduled for deprecation
- Regular review as part of release planning

**Deprecation Criteria:**
- Maintenance stopped
- Better alternative available
- Security issues unaddressed
- License changes making usage problematic

---

## How Lutufi Manages Dependencies

### Dependency Management Workflow

**Adding Dependencies:**
1. Evaluate necessity (can functionality be implemented simply?)
2. Assess risk (maturity, maintenance, license, bus factor)
3. Check compatibility with existing dependencies
4. Add with minimal version constraint
5. Update documentation and LICENSE files

**Updating Dependencies:**
1. Review changelog for breaking changes
2. Update in development branch
3. Run full test suite
4. Check with cargo-audit
5. Update Cargo.lock
6. Document in CHANGELOG

**Removing Dependencies:**
1. Verify no longer used (cargo-udeps)
2. Remove from Cargo.toml
3. Update documentation
4. Note in CHANGELOG

### Cargo.toml Structure

**Workspace Structure:**
```toml
[workspace]
members = ["lutufi-core", "lutufi-python", "lutufi-cli"]
resolver = "2"

[workspace.dependencies]
# Core dependencies
thiserror = "1.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
# ...
```

**Crate-Specific Dependencies:**
```toml
[package]
name = "lutufi-core"
version = "0.5.0"
edition = "2021"
rust-version = "1.70.0"

[dependencies]
thiserror = { workspace = true }
petgraph = { version = "0.6", optional = true }
rayon = { version = "1.8", optional = true }

[features]
default = ["petgraph", "rayon"]
parallel = ["rayon"]
```

### Automated Dependency Management

**Dependabot:**
- Automated dependency update PRs
- Security update prioritization
- Weekly update checks

**Renovate (alternative):**
- More configurable dependency updates
- Grouping of related updates
- Custom update schedules

### Documentation of Dependencies

**DEPENDENCIES.md:**
- Human-readable dependency list
- Rationale for each dependency
- Risk assessment summary

**LICENSE Aggregation:**
- LICENSE file includes all dependency licenses
- Automated generation during release
- Ensures compliance

---

## Key References

### Dependency Management Best Practices

1. **Cargo Book - Specifying Dependencies:**
   https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
   - Official guide to Cargo dependency management

2. **Rust API Guidelines:**
   https://rust-lang.github.io/api-guidelines/
   - Best practices for API design affecting dependencies

3. **Semantic Versioning:**
   https://semver.org/
   - Standard for version numbering

4. **Cargo.toml Format:**
   https://doc.rust-lang.org/cargo/reference/manifest.html
   - Complete Cargo.toml reference

### Supply Chain Security

1. **RustSec Advisory Database:**
   https://rustsec.org/
   - Security advisories for Rust crates

2. **cargo-audit:**
   https://github.com/RustSec/rustsec/tree/main/cargo-audit
   - Tool for auditing dependencies

3. **cargo-deny:**
   https://github.com/EmbarkStudios/cargo-deny
   - Dependency policy enforcement

4. **SLSA (Supply-chain Levels for Software Artifacts):**
   https://slsa.dev/
   - Framework for supply chain security

5. **OpenSSF Best Practices:**
   https://bestpractices.coreinfrastructure.org/
   - Security best practices for open source

### License Compliance

1. **Apache 2.0 License:**
   https://www.apache.org/licenses/LICENSE-2.0
   - Lutufi's primary license

2. **SPDX License List:**
   https://spdx.org/licenses/
   - Standardized license identifiers

3. **FOSSA:**
   https://fossa.com/
   - License compliance automation (tool reference)

4. **Choose a License:**
   https://choosealicense.com/
   - Guide to open source licenses

### Related Projects' Approaches

1. **Rust Cookbook - Dependencies:**
   https://rust-lang-nursery.github.io/rust-cookbook/development_tools.html
   - Community best practices

2. **Rustls Dependency Philosophy:**
   https://github.com/rustls/rustls/blob/main/CONTRIBUTING.md#dependencies
   - Example of minimal dependency approach

3. **Tokio Dependency Management:**
   https://github.com/tokio-rs/tokio/blob/master/CONTRIBUTING.md#dependency-minimization
   - Large project's dependency strategy

---

## Conclusion

The dependency strategy for Lutufi, authored by Wasswa Lutufi Sebbanja and licensed under Apache 2.0, reflects a careful balance between functionality and simplicity. The minimal dependencies principle ensures that users aren't burdened with unnecessary transitive dependencies, while optional features allow power users to access advanced capabilities when needed.

Key aspects of the dependency management approach include:

1. **Minimal Core:** Only essential dependencies are included in the core library, reducing attack surface and compile times.

2. **Optional Features:** Extended functionality is available through feature flags, allowing users to customize their dependency footprint.

3. **Risk Assessment:** Each dependency is evaluated for maturity, maintenance status, license compatibility, and bus factor before inclusion.

4. **Supply Chain Security:** Automated vulnerability scanning, reproducible builds, and checksum verification protect against supply chain attacks.

5. **Abstraction Layers:** Dependencies are isolated behind internal abstractions, facilitating future replacement if necessary.

6. **Continuous Monitoring:** The dependency landscape is continuously monitored for security advisories, maintenance issues, and emerging alternatives.

The dependency graph is designed to be stable for production use while remaining flexible enough to evolve with the Rust ecosystem. Regular reviews ensure that Lutufi continues to use the best available libraries for its computational requirements while maintaining the security and reliability standards required by its users in research, finance, intelligence, and industry.
