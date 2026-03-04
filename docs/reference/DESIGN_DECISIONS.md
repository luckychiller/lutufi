# Design Decisions Log

**Document Version**: 1.0  
**Status**: Working Draft  
**Author**: Wasswa Lutufi Sebbanja  
**Last Updated**: March 2026  

---

## Table of Contents

1. [Purpose](#purpose)
2. [ADR Format](#adr-format)
3. [Core Language Choice](#core-language-choice)
4. [Canonical Representation](#canonical-representation)
5. [Python as Primary Interface](#python-as-primary-interface)
6. [Sparse Representation Strategy](#sparse-representation-strategy)
7. [Inference Algorithm Selection](#inference-algorithm-selection)
8. [API Design Principles](#api-design-principles)
9. [Error Handling Approach](#error-handling-approach)
10. [Serialization Format](#serialization-format)
11. [Testing Strategy](#testing-strategy)
12. [Rejected Alternatives](#rejected-alternatives)
13. [Decision Status](#decision-status)

---

## Purpose

The Design Decisions Log records significant architectural and design decisions made during the Lutufi project, along with their context, rationale, and consequences. This document serves as:

- **Historical record**: Why decisions were made, preserving context for future developers
- **Onboarding aid**: New contributors can understand the system's design philosophy
- **Change guidance**: When reconsidering decisions, the original rationale is preserved
- **Stakeholder communication**: Explains technical choices to non-technical audiences

Each decision is documented using the Architecture Decision Record (ADR) format, providing a consistent structure for capturing the decision lifecycle.

---

## ADR Format

All design decisions follow this Architecture Decision Record format:

```
### ADR-NNN: [Decision Title]

**Status**: [PROPOSED/ACCEPTED/DEPRECATED/SUPERSEDED]

**Date**: [YYYY-MM-DD]

**Deciders**: [Who was involved in the decision]

#### Context
[The forces at play, including technological, political, social, and project-local.]

#### Decision
[The decision that was made.]

#### Consequences
[The resulting context after applying the decision. Includes positive, negative, and neutral consequences.]

#### Alternatives Considered
[Other options that were evaluated and why they were not selected.]

#### Related Decisions
[Links to related ADRs.]
```

---

## Core Language Choice

### ADR-001: Rust for Core Implementation

**Status**: ACCEPTED

**Date**: 2025-12-15

**Deciders**: Core development team

#### Context

Lutufi requires high-performance computation for probabilistic inference on large networks. The core implementation language must provide:

1. **Memory safety**: Probabilistic inference involves complex data structures; memory errors are difficult to debug
2. **Performance**: Inference algorithms are computationally intensive; overhead from garbage collection or interpretation is unacceptable
3. **Parallelism**: Modern hardware requires effective use of multi-core and SIMD capabilities
4. **Maintainability**: The codebase must remain maintainable over years of development
5. **Ecosystem**: Access to libraries for linear algebra, optimization, and serialization

Candidates evaluated:
- **C++**: Industry standard for high-performance computing
- **Rust**: Modern systems language with memory safety guarantees
- **Julia**: High-level language designed for scientific computing
- **C**: Maximum performance but minimal abstraction

#### Decision

Implement Lutufi's core engine in **Rust**.

#### Consequences

**Positive**:
- Memory safety without garbage collection eliminates entire classes of bugs
- Zero-cost abstractions enable high-level code with C-level performance
- Excellent package ecosystem (ndarray, nalgebra, serde, rayon)
- Strong type system catches errors at compile time
- First-class WebAssembly support for future web deployment
- Growing adoption in scientific computing (polars, datafusion)

**Negative**:
- Steeper learning curve than C++ for some developers
- Smaller talent pool than C++ or Python
- Some scientific libraries still more mature in C++ (though gap is closing)
- Longer compile times than C
- Ecosystem immaturity in some niche areas

**Neutral**:
- Python bindings via PyO3 provide user-friendly interface (see ADR-003)
- Rust's borrow checker requires adjustment for developers used to GC languages

#### Alternatives Considered

**C++ (rejected)**:
- Pros: Maximum ecosystem, existing expertise, mature tooling
- Cons: Memory safety issues common; undefined behavior difficult to prevent; build system complexity (CMake, etc.)
- Rationale: Memory safety issues were considered unacceptable risk for numerical code correctness

**Julia (rejected)**:
- Pros: Designed for scientific computing; excellent syntax for math; JIT compilation
- Cons: Runtime overhead; garbage collection pauses problematic for real-time inference; slower for data structure-heavy code
- Rationale: GC pauses and runtime overhead conflict with performance requirements

**C (rejected)**:
- Pros: Maximum performance; universal compatibility
- Cons: No abstraction mechanisms; manual memory management error-prone; development velocity too low
- Rationale: Development productivity and safety requirements outweigh performance gains

#### Related Decisions

- ADR-003: Python as Primary Interface
- ADR-006: PyO3 for Python Bindings

---

## Canonical Representation

### ADR-002: Factor Graphs as Internal Representation

**Status**: ACCEPTED

**Date**: 2026-01-10

**Deciders**: Core development team

#### Context

Lutufi must support both Bayesian networks (directed) and Markov random fields (undirected), as well as convert between them. Users may input models in either format, and algorithms may be more natural in one representation than another.

Options for internal representation:
1. **Maintain both**: Store both directed and undirected representations
2. **Directed only**: Convert everything to Bayesian networks
3. **Undirected only**: Convert everything to Markov random fields
4. **Factor graphs**: Use bipartite factor graphs as canonical form

#### Decision

Use **factor graphs** as the canonical internal representation.

#### Consequences

**Positive**:
- Factor graphs can represent both directed and undirected models uniformly
- Message-passing algorithms have natural expression on factor graphs
- Factor graphs make conditional independence explicit through graph structure
- Many inference algorithms (BP, mean field) are naturally expressed on factor graphs
- Conversion from BN/MRF to factor graph is straightforward and lossless
- Factor graphs unify treatment of variables and factors

**Negative**:
- Users familiar with DAGs need to understand factor graph concepts
- Some BN-specific algorithms require conversion back to DAG representation
- Visualization of factor graphs is less intuitive than DAGs for some users

**Neutral**:
- Factor graphs have bipartite structure (variables and factors as separate node types)
- Message passing defined on edges between variables and factors

#### Alternatives Considered

**Maintain both directed and undirected representations (rejected)**:
- Pros: Native support for each model type; algorithms can use natural representation
- Cons: Code duplication; synchronization issues; complexity in maintaining consistency
- Rationale: Duplication and complexity outweigh benefits

**Directed only (Bayesian networks) (rejected)**:
- Pros: DAGs are intuitive for causal modeling; many users think in terms of BNs
- Cons: Converting undirected models to directed requires moralization (loss of structure); some MRFs cannot be faithfully represented as BNs
- Rationale: Cannot faithfully represent all models users may want to use

**Undirected only (Markov random fields) (rejected)**:
- Pros: Natural for spatial and relational data; symmetric relationships
- Cons: Converting directed models loses causal interpretation; parameterization less intuitive
- Rationale: Loses causal semantics important for many applications

#### Related Decisions

- ADR-007: API Design for Model Specification
- ADR-012: Visualization Strategy

---

## Python as Primary Interface

### ADR-003: Python as Primary User Interface

**Status**: ACCEPTED

**Date**: 2026-01-15

**Deciders**: Core development team

#### Context

While the core engine is in Rust (ADR-001), users need an accessible interface. The target users (data scientists, researchers, analysts) have varying programming backgrounds.

Candidates:
- **Python**: Dominant in data science
- **Julia**: Growing in scientific computing
- **R**: Strong in statistics
- **Rust**: Direct API for performance-critical users
- **Multiple**: Support several languages equally

#### Decision

**Python** is the primary user-facing interface, with Rust API available for advanced users.

#### Consequences

**Positive**:
- Python has largest data science ecosystem (NumPy, pandas, NetworkX, scikit-learn)
- Network analysis community predominantly uses Python
- Jupyter notebooks provide excellent interactive environment
- Rich visualization ecosystem (matplotlib, plotly, networkx drawing)
- Easy integration with ML frameworks (PyTorch, TensorFlow)

**Negative**:
- Python's performance limitations require careful Rust-Python boundary design
- Two-language problem complicates debugging
- Python packaging and distribution is complex
- Type system less expressive than Rust's

**Neutral**:
- PyO3 enables efficient Rust-Python interop
- Python API can evolve independently of Rust core

#### Alternatives Considered

**Julia as primary (rejected)**:
- Pros: Designed for scientific computing; excellent performance for high-level code
- Cons: Smaller user base; less mature ecosystem; harder to hire for
- Rationale: User base and ecosystem size favor Python

**R as primary (rejected)**:
- Pros: Strong statistics community; excellent for statistical modeling
- Cons: Smaller network science community; language quirks limit adoption
- Rationale: Network science more established in Python ecosystem

**Multiple equal interfaces (rejected)**:
- Pros: Flexibility for users
- Cons: Development resources spread thin; harder to maintain consistency
- Rationale: Focus resources on best single interface

#### Related Decisions

- ADR-001: Rust for Core Implementation
- ADR-006: PyO3 for Python Bindings

---

## Sparse Representation Strategy

### ADR-004: Sparse Matrix-Centric Data Structures

**Status**: ACCEPTED

**Date**: 2026-01-20

**Deciders**: Core development team

#### Context

Network data is inherently sparse—most nodes connect to only a small fraction of other nodes. Factor graphs inherit this sparsity. Efficient storage and computation on sparse structures is critical for scalability.

Key questions:
1. What sparse matrix format? (CSR, CSC, COO, custom)
2. How to handle heterogeneous factor sizes?
3. How to support both static and dynamic networks?

#### Decision

Use **compressed sparse row (CSR)** as the primary sparse matrix format, with **coordinate (COO)** format for construction and updates.

#### Consequences

**Positive**:
- CSR provides O(1) row access and O(nnz) memory for row-oriented operations
- Efficient matrix-vector products for message passing
- COO enables efficient incremental construction
- Conversion between CSR and COO is O(nnz)
- Well-supported in Rust ecosystem (sprs crate)

**Negative**:
- Column operations slower (relevant for some transpose operations)
- Dynamic updates (add/remove edges) require format conversion
- Factor-specific storage needs may not map perfectly to matrix structures

**Neutral**:
- Factors are stored as separate arrays indexed by the sparse structure
- Message passing uses CSR for factor-to-variable and CSC for variable-to-factor (transposed view)

#### Implementation Details

```rust
// Factor graph structure
struct FactorGraph {
    // Variable nodes: n variables
    n_variables: usize,
    
    // Factor nodes: m factors
    n_factors: usize,
    
    // CSR structure for factor-to-variable connectivity
    // factors_indices[factor_i] gives variables connected to factor_i
    factors_indices: Vec<Vec<usize>>,
    
    // Variable-to-factor connectivity (derived from above)
    variables_factors: Vec<Vec<usize>>,
    
    // Factor definitions
    factors: Vec<Factor>,
}
```

#### Alternatives Considered

**Dense matrices (rejected)**:
- Pros: Simple implementation; cache-friendly for small networks
- Cons: O(n²) memory and time; infeasible for networks > 10k nodes
- Rationale: Scalability requirements demand sparse representation

**Adjacency lists (rejected)**:
- Pros: Simple; flexible for irregular structures
- Cons: Less cache-friendly; harder to optimize linear algebra operations
- Rationale: Matrix formats provide better hardware utilization

**Custom factor-centric storage (rejected)**:
- Pros: Optimized for factor graph operations
- Cons: Lose benefits of existing sparse linear algebra libraries
- Rationale: Prefer proven libraries over custom implementations

#### Related Decisions

- ADR-002: Factor Graphs as Internal Representation
- ADR-014: Out-of-Core Processing Strategy

---

## Inference Algorithm Selection

### ADR-005: Automatic Algorithm Selection with User Override

**Status**: ACCEPTED

**Date**: 2026-01-25

**Deciders**: Core development team, advisory board

#### Context

Multiple inference algorithms exist for probabilistic graphical models: exact methods (variable elimination, junction tree), approximate methods (loopy BP, mean field, Gibbs sampling), and hybrid approaches. No single algorithm is best for all problems.

Options:
1. **Single algorithm**: Implement one algorithm well
2. **Manual selection**: User must choose algorithm
3. **Automatic selection**: System chooses based on problem properties
4. **Automatic with override**: System recommends, user can override

#### Decision

Implement **automatic algorithm selection** with **user override capability**.

#### Consequences

**Positive**:
- Users get good performance without deep algorithmic expertise
- System can adapt to problem structure (tree vs. loopy, discrete vs. continuous)
- Advanced users retain control for specialized cases
- Can improve selection logic over time without breaking user code

**Negative**:
- Selection logic adds complexity
- Wrong automatic choices can frustrate users
- Must maintain multiple algorithm implementations
- Selection criteria may be opaque

**Neutral**:
- Selection based on: graph structure (treewidth), variable types (discrete/continuous), size, requested precision

#### Selection Heuristics

| Property | Algorithm Selection |
|----------|-------------------|
| Tree structure | Exact belief propagation (linear time) |
| Low treewidth (< 20) | Junction tree with variable elimination |
| Discrete, medium size | Loopy belief propagation |
| Continuous | Gaussian BP or nonparametric BP |
| Mixed | Hybrid EP or sampling |
| Very large, loopy | Stochastic methods (Gibbs, particle BP) |

#### Alternatives Considered

**Single algorithm (loopy BP only) (rejected)**:
- Pros: Simpler implementation; uniform API
- Cons: Loopy BP fails to converge on some graphs; exact inference faster for trees
- Rationale: Unacceptable performance and reliability limitations

**Manual selection only (rejected)**:
- Pros: User control; simpler implementation
- Cons: Requires expertise; easy to make poor choices
- Rationale: Usability requirement demands intelligent defaults

#### Related Decisions

- ADR-008: API Design Principles
- OPEN-ALGO-002 (adaptive algorithm selection question)

---

## API Design Principles

### ADR-006: Fluent API with Builder Pattern

**Status**: ACCEPTED

**Date**: 2026-02-01

**Deciders**: Core development team

#### Context

Lutufi's API must serve users ranging from data scientists (who want simplicity) to researchers (who want control). The API design must balance these needs.

Options:
1. **Simple API**: Minimal configuration, opinionated defaults
2. **Complete API**: Expose all parameters
3. **Layered API**: Simple high-level, complete low-level
4. **Fluent API**: Chainable methods with progressive disclosure

#### Decision

Implement a **fluent API** using the **builder pattern** with progressive disclosure of complexity.

#### Consequences

**Positive**:
- Simple operations remain simple: `model.infer()`
- Complex operations discoverable through IDE autocomplete
- Method chaining reads naturally: `model.with_algorithm("bp").with_max_iterations(100).infer()`
- Progressive disclosure: basic → advanced → expert parameters
- Type safety through Rust/Python type systems

**Negative**:
- Builder implementations add boilerplate
- Chaining can lead to long, unwieldy expressions
- Learning curve for method chaining patterns

**Neutral**:
- Default implementations provide good starting points

#### Example API

```python
# Simple usage
results = model.infer()

# With options
results = (model
    .with_algorithm("loopy_bp")
    .with_tolerance(1e-6)
    .with_max_iterations(1000)
    .infer())

# Causal query
effect = (model
    .do_intervention({"X": 1})
    .query("Y")
    .infer())
```

#### Alternatives Considered

**Configuration dictionaries (rejected)**:
- Pros: Simple to implement; familiar to scikit-learn users
- Cons: Not type-safe; errors at runtime; IDE support poor
- Rationale: Prefer compile-time/type-checking safety

**Multiple API levels (rejected)**:
- Pros: Clear separation of concerns
- Cons: Duplication; confusion about which level to use
- Rationale: Single coherent API preferable

#### Related Decisions

- ADR-003: Python as Primary Interface
- ADR-008: Error Handling Approach

---

## Error Handling Approach

### ADR-007: Structured Errors with Context

**Status**: ACCEPTED

**Date**: 2026-02-05

**Deciders**: Core development team

#### Context

Inference can fail for many reasons: numerical issues (underflow, overflow), convergence failures, invalid model specifications, resource exhaustion. Users need clear, actionable error information.

Options:
1. **Exceptions with strings**: Simple but limited
2. **Error codes**: Machine-processable but user-hostile
3. **Structured errors with context**: Rich information for debugging
4. **Result types**: Force error handling (Rust-style)

#### Decision

Use **structured error types** with rich context and **suggestions for resolution**.

#### Consequences

**Positive**:
- Error types enable programmatic handling
- Context helps debugging (which variable, which factor, iteration number)
- Suggestions guide users toward resolution
- Rust's Result type enforces error handling in core
- Python exceptions carry structured data

**Negative**:
- More complex error hierarchy to maintain
- Error message localization more difficult
- Larger binary size from error metadata

**Neutral**:
- Users can choose to handle or propagate errors

#### Error Hierarchy

```rust
enum InferenceError {
    ConvergenceError { iteration: usize, residual: f64 },
    NumericalError { variable: usize, operation: String },
    InvalidModelError { reason: String, suggestion: String },
    ResourceError { resource: String, required: usize, available: usize },
}
```

#### Alternatives Considered

**Panic on error (rejected)**:
- Pros: Simple
- Cons: Unacceptable for library code; crashes user programs
- Rationale: Library must never crash calling code

**Simple error strings (rejected)**:
- Pros: Easy to implement
- Cons: Hard to handle programmatically; poor debugging experience
- Rationale: Users need actionable error information

#### Related Decisions

- ADR-006: Fluent API with Builder Pattern
- DESIGN-Error-Handling (detailed error design document)

---

## Serialization Format

### ADR-008: Custom Binary Format with Versioning

**Status**: ACCEPTED

**Date**: 2026-02-10

**Deciders**: Core development team

#### Context

Models and inference results need to be saved and loaded. The serialization format must balance efficiency, portability, and future compatibility.

Options:
1. **JSON**: Human-readable, universal
2. **Protocol Buffers**: Efficient, schema evolution
3. **MessagePack**: Binary JSON
4. **Cap'n Proto**: Zero-copy, schema evolution
5. **Custom binary**: Maximum control

#### Decision

Implement a **custom binary format** with **explicit versioning** and **backward compatibility guarantees**.

#### Consequences

**Positive**:
- Optimal layout for Lutufi's data structures
- No external dependencies for core format
- Versioning enables format evolution
- Can include compression for large models
- Full control over backward compatibility

**Negative**:
- Must maintain format specification
- Interoperability requires explicit converters
- More development effort than using existing format

**Neutral**:
- Export to GraphML, GEXF, and other formats for interoperability
- Separate format versions for internal (performance) and external (interoperability) use

#### Format Structure

```
Lutufi Model Format v1
├── Header
│   ├── Magic bytes: "LUTUFI"
│   ├── Version: u16
│   ├── Flags: compression, etc.
│   └── Checksum
├── Metadata
│   ├── Creation timestamp
│   ├── Source information
│   └── Custom attributes
├── Graph Structure
│   ├── Variable definitions
│   ├── Factor definitions
│   └── Connectivity (CSR format)
└── Parameters
    ├── Factor tables/matrices
    └── Optional learned parameters
```

#### Alternatives Considered

**Protocol Buffers (rejected)**:
- Pros: Battle-tested; schema evolution; cross-language
- Cons: Additional dependency; overhead for simple structures; less control
- Rationale: Prefer control and zero dependencies for core format

**MessagePack (rejected)**:
- Pros: Simple; efficient; language support
- Cons: No schema; less compact than possible; limited evolution
- Rationale: Need schema and versioning for long-term maintainability

**JSON (rejected)**:
- Pros: Human-readable; universal
- Cons: Verbose; slow to parse; large files
- Rationale: Performance requirements demand binary format

#### Related Decisions

- ADR-004: Sparse Representation Strategy
- OPEN-IMPL-003 (serialization format question)

---

## Testing Strategy

### ADR-009: Property-Based Testing with Reference Implementations

**Status**: ACCEPTED

**Date**: 2026-02-15

**Deciders**: Core development team

#### Context

Correctness is critical for probabilistic inference—wrong results undermine trust. Testing must catch numerical errors, algorithmic bugs, and API regressions.

Testing approaches:
1. **Unit tests**: Test individual functions
2. **Integration tests**: Test full workflows
3. **Property-based tests**: Verify mathematical properties
4. **Reference implementations**: Compare against known-good implementations

#### Decision

Implement a **multi-layer testing strategy** with emphasis on **property-based testing** and **comparison against reference implementations**.

#### Consequences

**Positive**:
- Property-based testing catches edge cases manual tests miss
- Reference implementations (pgmpy, libDAI) verify correctness
- Mathematical invariants provide oracle for testing
- CI ensures no regressions

**Negative**:
- Property-based testing requires investment in generators
- Reference comparison adds CI time
- Floating-point comparisons require tolerance handling

**Neutral**:
- Tests run at multiple precision levels (f32, f64)

#### Testing Layers

| Layer | Purpose | Tools |
|-------|---------|-------|
| Unit | Function correctness | Rust built-in |
| Property | Mathematical invariants | proptest, hypothesis |
| Integration | End-to-end workflows | pytest |
| Reference | Correctness verification | pgmpy, libDAI comparisons |
| Performance | Regression detection | Criterion.rs, airspeed velocity |

#### Mathematical Properties Tested

- **Normalization**: Marginals sum to 1
- **Consistency**: Different inference methods agree (within tolerance)
- **Conditioning**: P(X|Y=y) integrates to P(Y=y)
- **Do-calculus**: P(Y|do(X=x)) equals back-door formula when applicable

#### Alternatives Considered

**Unit tests only (rejected)**:
- Pros: Simple; fast
- Cons: Miss edge cases; insufficient for numerical code
- Rationale: Numerical correctness requires stronger verification

**Golden file testing (rejected)**:
- Pros: Catches changes
- Cons: Brittle; doesn't verify correctness, only consistency
- Rationale: Need to verify correctness, not just stability

#### Related Decisions

- ADR-001: Rust for Core Implementation
- OPEN-IMPL-006 (stochastic algorithm testing)

---

## Rejected Alternatives

This section documents significant alternatives that were considered but rejected at various decision points, providing context for why current decisions were made.

### REJ-001: Pure Python Implementation

**Context**: Early consideration of implementing entirely in Python for simplicity.

**Decision**: Rejected in favor of Rust core with Python interface (ADR-001, ADR-003).

**Rationale**: Performance requirements for large-scale inference cannot be met by pure Python. Numba and Cython were considered but would require significant additional complexity without Rust's safety guarantees.

---

### REJ-002: Apache Spark for Distributed Computing

**Context**: Consideration of using Spark for large-scale distributed inference.

**Decision**: Rejected as primary approach; may revisit for specific large-scale features.

**Rationale**: Spark's batch-oriented model poorly fits iterative inference algorithms. Communication overhead dominates for typical network sizes. Single-node shared-memory parallelism more appropriate for target problem sizes.

---

### REJ-003: Automatic Differentiation for All Gradients

**Context**: Proposal to use autodiff (via Enzyme or similar) for all gradient computations in learning.

**Decision**: Rejected as universal approach; used selectively where beneficial.

**Rationale**: Autodiff adds complexity and can be less efficient than analytical gradients for structured problems. Manual gradients implemented for common cases; autodiff available as fallback.

---

### REJ-004: Support for Arbitrary Precision Arithmetic

**Context**: Proposal to support arbitrary precision (BigRational, etc.) for exact inference.

**Decision**: Rejected for core implementation; may provide as extension.

**Rationale**: Arbitrary precision is too slow for practical problems. Log-space computation and careful numerical engineering sufficient for stability. Support would significantly complicate the codebase.

---

### REJ-005: Real-Time Collaborative Editing

**Context**: Proposal for multi-user real-time model editing (like Google Docs for networks).

**Decision**: Rejected as out of scope.

**Rationale**: Significant complexity for uncertain value. Focus on single-user performance and correctness first. Can revisit if user demand emerges.

---

## Decision Status

| ADR | Title | Status | Date | Review Date |
|-----|-------|--------|------|-------------|
| ADR-001 | Rust for Core Implementation | ACCEPTED | 2025-12-15 | 2026-12-15 |
| ADR-002 | Factor Graphs as Internal Representation | ACCEPTED | 2026-01-10 | 2027-01-10 |
| ADR-003 | Python as Primary Interface | ACCEPTED | 2026-01-15 | 2027-01-15 |
| ADR-004 | Sparse Matrix-Centric Data Structures | ACCEPTED | 2026-01-20 | 2027-01-20 |
| ADR-005 | Automatic Algorithm Selection | ACCEPTED | 2026-01-25 | 2026-07-25 |
| ADR-006 | Fluent API with Builder Pattern | ACCEPTED | 2026-02-01 | 2027-02-01 |
| ADR-007 | Structured Errors with Context | ACCEPTED | 2026-02-05 | 2027-02-05 |
| ADR-008 | Custom Binary Format with Versioning | ACCEPTED | 2026-02-10 | 2027-02-10 |
| ADR-009 | Property-Based Testing with Reference Implementations | ACCEPTED | 2026-02-15 | 2027-02-15 |

**Status Definitions**:
- **PROPOSED**: Decision under discussion, not yet finalized
- **ACCEPTED**: Decision accepted and being implemented
- **DEPRECATED**: Decision no longer recommended but still supported
- **SUPERSEDED**: Decision replaced by a newer ADR

**Review Schedule**:
- Core architecture decisions: Annual review
- Implementation decisions: Review after 6 months or significant experience
- All decisions: Review upon major version changes

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-03-04 | Initial design decisions log with 9 ADRs and 5 rejected alternatives |
