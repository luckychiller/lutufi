# Lutufi Library Architecture

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architectural Overview](#architectural-overview)
3. [Design Principles](#design-principles)
4. [Layered Architecture](#layered-architecture)
5. [Core Components](#core-components)
6. [The Multi-Representation Architecture](#the-multi-representation-architecture)
7. [Plugin Architecture](#plugin-architecture)
8. [Memory Management](#memory-management)
9. [Concurrency Model](#concurrency-model)
10. [Error Handling Strategy](#error-handling-strategy)
11. [Core Language Decision](#core-language-decision)
12. [Python Binding Architecture](#python-binding-architecture)
13. [Build System and Packaging](#build-system-and-packaging)
14. [Versioning and Compatibility](#versioning-and-compatibility)
15. [Architectural Decisions Log](#architectural-decisions-log)
16. [Key References](#key-references)

---

## Executive Summary

Lutufi is a high-performance library that unifies Bayesian networks with social and economic network analysis. The architecture is designed to handle the dual nature of probabilistic graphical models—combining graph-theoretic structure with statistical inference—while maintaining extensibility, performance, and usability.

This document describes the comprehensive architectural design of Lutufi, including its layered structure, core components, memory management strategies, concurrency model, and binding architecture. The design emphasizes separation of concerns, allowing domain experts to work at appropriate abstraction levels while enabling systems programmers to optimize critical paths.

---

## Architectural Overview

### High-Level System Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           USER APPLICATION LAYER                            │
│  (Research Scripts, Jupyter Notebooks, Production Systems, Web Services)   │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PYTHON API LAYER (lutufi.*)                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │   Models    │ │  Inference  │ │  Learning   │ │  Network Analysis   │   │
│  │  (models)   │ │ (inference) │ │  (learning) │ │     (networks)      │   │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────────┘   │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │   Causal    │ │     Viz     │ │    Data     │ │   Configuration     │   │
│  │  (causal)   │ │    (viz)    │ │    I/O      │ │    (config)         │   │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼ PyO3 / pybind11 FFI
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CORE LIBRARY (Rust/C++)                            │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      APPLICATION LAYER                               │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐ │   │
│  │  │    Query    │ │   Builder   │ │  Validation │ │  Orchestration  │ │   │
│  │  │  Processor  │ │   Pattern   │ │    Engine   │ │     Engine      │ │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        DOMAIN LAYER                                  │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                  MODEL REPRESENTATION MODULE                     │ │   │
│  │  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │ │   │
│  │  │  │   Factor    │ │   Graph     │ │  Variable   │ │   CPD     │ │ │   │
│  │  │  │   Graph     │ │   Engine    │ │   Registry  │ │  Registry │ │ │   │
│  │  │  └─────────────┘ └─────────────┘ └─────────────┘ └───────────┘ │ │   │
│  │  └─────────────────────────────────────────────────────────────────┘ │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                   INFERENCE ENGINE MODULE                        │ │   │
│  │  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │ │   │
│  │  │  │   Exact     │ │Approximate  │ │  Algorithm  │ │Convergence│ │ │   │
│  │  │  │   Solvers   │ │   Solvers   │ │   Selector  │ │  Monitor  │ │ │   │
│  │  │  └─────────────┘ └─────────────┘ └─────────────┘ └───────────┘ │ │   │
│  │  └─────────────────────────────────────────────────────────────────┘ │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                   LEARNING MODULE                                │ │   │
│  │  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │ │   │
│  │  │  │  Structure  │ │  Parameter  │ │  Constraint │ │   Score   │ │ │   │
│  │  │  │   Learning  │ │   Learning  │ │   Testing   │ │  Engine   │ │ │   │
│  │  │  └─────────────┘ └─────────────┘ └─────────────┘ └───────────┘ │ │   │
│  │  └─────────────────────────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     INFRASTRUCTURE LAYER                             │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐   │
│  │  │   Sparse    │ │     I/O     │ │  Numerical  │ │   Concurrency   │   │
│  │  │   Matrix    │ │   Adapters  │ │   Library   │ │   Primitives    │   │
│  │  │   Library   │ │             │ │             │ │                 │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────┘   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐   │
│  │  │  Memory     │ │     FFI     │ │  Plugin     │ │   Serialization │   │
│  │  │  Management │ │   Bridge    │ │   System    │ │     Engine      │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────────┘   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         EXTERNAL DEPENDENCIES                               │
│  BLAS/LAPACK  │  CUDA/ROCm  │  MPI/OpenMP  │  NumPy/SciPy  │  NetworkX     │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Major Components and Responsibilities

The Lutufi architecture is organized around five major subsystems:

**1. Model Representation Subsystem**
Responsible for storing and manipulating probabilistic graphical models. This subsystem uses factor graphs as the canonical internal representation, enabling unified handling of Bayesian networks, Markov random fields, and dynamic Bayesian networks.

**2. Inference Engine Subsystem**
Provides computational methods for probabilistic reasoning. Supports both exact inference (variable elimination, junction tree) and approximate inference (belief propagation, sampling, variational methods). The subsystem includes automatic algorithm selection based on model characteristics.

**3. Learning Subsystem**
Implements structure learning and parameter learning algorithms. Structure learning includes score-based methods (BIC, BDeu, MDL), constraint-based methods (PC algorithm, IC algorithm), and hybrid approaches. Parameter learning includes maximum likelihood estimation, Bayesian estimation, and EM for incomplete data.

**4. Query Interface Subsystem**
Provides high-level APIs for probabilistic queries, causal queries, and network analysis. Supports d-separation testing, marginal queries, MAP inference, interventional queries (do-calculus), and counterfactual reasoning.

**5. Integration Subsystem**
Handles interoperability with external libraries and formats. Includes adapters for NetworkX graph objects, pandas DataFrames, NumPy arrays, and various file formats (GraphML, GEXF, BIF, XMLBIF, UAI).

---

## Design Principles

### Separation of Concerns

The architecture enforces strict separation between:

- **Structural representation** (graph topology) and **probabilistic content** (CPDs/factors)
- **Model specification** and **inference execution**
- **Core algorithms** and **language bindings**
- **Domain logic** and **infrastructure concerns**

This separation enables independent evolution of components, facilitates testing, and supports multiple frontend interfaces.

### Unification of Representations

Rather than maintaining separate codebases for different model types (BNs, MRFs, DBNs), Lutufi uses **factor graphs as the canonical internal representation**. All model types are converted to factor graphs upon ingestion, operated upon using unified algorithms, and converted back to user-facing representations when needed.

Benefits of this approach:
- Algorithmic code is written once and reused across model types
- Conversions are well-defined and tested
- Users can work with familiar representations while benefiting from unified internals
- New model types can be added by defining conversions to/from factor graphs

### Performance Awareness

Performance is a first-class concern, not an afterthought:

- **Sparse representations** are used throughout to exploit structure
- **Cache-friendly memory layouts** optimize for modern CPU architectures
- **Algorithm selection** considers computational complexity and available resources
- **Lazy evaluation** defers expensive computations until necessary
- **Memory-mapped files** support out-of-core computation for large networks

### Extensibility

The architecture supports extension at multiple points:

- **Plugin architecture** allows custom CPD types, inference algorithms, and file formats
- **Trait-based interfaces** (Rust) or **abstract base classes** (Python) define extension points
- **Configuration-driven behavior** enables customization without code changes
- **Clear internal APIs** facilitate contribution of new components

---

## Layered Architecture

### Presentation Layer (Python Bindings)

The presentation layer provides the primary user interface through Python bindings. This layer is responsible for:

**API Translation**
Converting Pythonic, dynamically-typed calls into the statically-typed, performance-oriented core. Handles type conversions, argument validation, and error translation.

**Object Lifecycle Management**
Managing the lifetime of Python wrapper objects and their underlying native counterparts. Ensures proper cleanup and prevents dangling references.

**Integration with Python Ecosystem**
Providing seamless interoperability with NumPy arrays, pandas DataFrames, NetworkX graphs, and Matplotlib visualizations.

**Example code pattern:**
```python
# Python API translates to native calls
import lutufi as lf
import numpy as np

# NumPy array automatically converted to native format
data = np.random.choice([0, 1], size=(1000, 5))
model = lf.BayesianNetwork.fit(data, algorithm='hc')  # Hill climbing

# Python dictionary converted to evidence map
evidence = {'Node_A': 1, 'Node_B': 0}
result = model.query(variables=['Node_C'], evidence=evidence)

# Result converted back to Python-friendly format
print(result['Node_C'])  # Probability distribution
```

### Application Layer (API Layer)

The application layer sits between the Python bindings and the domain layer, providing:

**Query Processing**
Parsing and validating user queries, optimizing query plans, and dispatching to appropriate domain components. Query optimization includes determining variable elimination orders, selecting inference algorithms, and caching intermediate results.

**Builder Pattern Support**
Fluent interfaces for constructing models incrementally. The builder pattern enables:
```python
model = (lf.BayesianNetwork.builder()
    .add_variable('A', domain=[0, 1])
    .add_variable('B', domain=[0, 1])
    .add_edge('A', 'B')
    .set_cpd('A', [0.3, 0.7])
    .set_cpd('B', [[0.9, 0.1], [0.2, 0.8]])
    .build())
```

**Orchestration Engine**
Coordinating multi-step operations like learning workflows that involve structure learning, parameter learning, and validation. The orchestration engine manages dependencies between steps and enables resumable execution.

### Domain Layer

The domain layer contains the core business logic of probabilistic modeling:

**Models Module**
Defines the hierarchy of probabilistic models:
- `FactorGraph`: Canonical internal representation
- `BayesianNetwork`: Directed acyclic graph with CPDs
- `MarkovRandomField`: Undirected graph with factor potentials
- `DynamicBayesianNetwork`: Time-sliced network for temporal models

Each model type implements conversion methods to/from the canonical factor graph representation.

**Inference Module**
Implements inference algorithms:
- **Exact Inference**: Variable elimination, junction tree algorithm
- **Approximate Inference**: Loopy belief propagation, Gibbs sampling, variational inference
- **Specialized Inference**: MAP inference, MPE queries

The inference module includes an `InferenceEngine` abstraction that selects appropriate algorithms based on model characteristics (treewidth, evidence pattern, query type).

**Learning Module**
Implements learning algorithms:
- **Structure Learning**: Score-based (hill climbing, simulated annealing), constraint-based (PC, IC), hybrid methods
- **Parameter Learning**: MLE, Bayesian estimation, EM for latent variables

### Infrastructure Layer

The infrastructure layer provides cross-cutting concerns:

**Sparse Matrix Library**
Custom sparse matrix implementations optimized for factor graph operations:
- Coordinate format (COO) for construction
- Compressed sparse row (CSR) for row access
- Compressed sparse column (CSC) for column access
- Specialized factor storage with dimension indexing

**I/O Adapters**
Format-specific parsers and serializers:
- Graph formats: GraphML, GEXF, Pajek
- Probabilistic formats: BIF, XMLBIF, UAI, JSON
- Data formats: CSV, TSV, HDF5, Parquet

**Numerical Library**
Mathematical utilities:
- Log-space arithmetic for numerical stability
- Special functions (log-gamma, digamma)
- Random number generation with reproducible seeds
- Linear algebra operations (leveraging BLAS/LAPACK)

**Concurrency Primitives**
Platform-agnostic abstractions for:
- Thread pools for parallel inference
- Work queues for distributed computation
- Synchronization primitives (mutexes, condition variables)
- Lock-free data structures for high-contention scenarios

---

## Core Components

### Model Representation Module

The model representation module uses **factor graphs as the canonical form** for all probabilistic models.

**Factor Graph Structure**
A factor graph is a bipartite graph consisting of:
- **Variable nodes**: Represent random variables with domains
- **Factor nodes**: Represent functions over subsets of variables
- **Edges**: Connect variables to factors they participate in

**Why Factor Graphs?**
Factor graphs were chosen as the canonical representation because:
1. **Universality**: Any graphical model (BN, MRF, FG) can be represented as a factor graph
2. **Clarity**: The bipartite structure makes variable dependencies explicit
3. **Algorithmic efficiency**: Message passing algorithms have natural implementations on factor graphs
4. **Extensibility**: New factor types can be added without changing core infrastructure

**Key Data Structures:**
```rust
// Rust-like pseudocode for core structures
struct FactorGraph {
    variables: Vec<Variable>,
    factors: Vec<Box<dyn Factor>>,
    variable_factor_edges: SparseBipartiteGraph,
    metadata: GraphMetadata,
}

struct Variable {
    id: VariableId,
    name: String,
    domain: Domain,
    evidence: Option<Evidence>,
}

enum Domain {
    Discrete { values: Vec<String> },
    Continuous { range: (f64, f64) },
    Mixed { components: Vec<Domain> },
}

trait Factor {
    fn scope(&self) -> &[VariableId];
    fn evaluate(&self, assignment: &Assignment) -> f64;
    fn marginalize(&self, variables: &[VariableId]) -> Box<dyn Factor>;
    fn multiply(&self, other: &dyn Factor) -> Box<dyn Factor>;
}
```

### Inference Engine Module

The inference engine module provides a unified interface for probabilistic inference while supporting multiple algorithmic backends.

**Algorithm Selection Strategy**
The engine automatically selects inference algorithms based on:
- Model treewidth (exact vs. approximate)
- Query type (marginal, MAP, conditional)
- Evidence pattern (single vs. multiple variables)
- Available resources (memory, CPU cores, GPU)
- User-specified preferences and constraints

**Exact Inference Implementations:**

*Variable Elimination*
- Supports multiple elimination heuristics (min-fill, min-degree, weighted min-fill)
- Implements bucket elimination for memory efficiency
- Provides query-specific elimination for multiple queries

*Junction Tree Algorithm*
- Moralization and triangulation
- Clique tree construction with optimal weighting
- Message passing schedules (collect-distribute, Hugin, Shafer-Shenoy)
- Incremental updates for dynamic evidence changes

**Approximate Inference Implementations:**

*Belief Propagation*
- Loopy belief propagation with damping
- Generalized belief propagation (region-based)
- Convergence diagnostics and damping strategies
- Fractional belief propagation for improved convergence

*Sampling Methods*
- Gibbs sampling with adaptive scanning
- Metropolis-Hastings with proposal optimization
- Importance sampling for rare events
- Sequential Monte Carlo for dynamic models

*Variational Inference*
- Mean field approximation
- Structured variational methods
- Expectation propagation

**Convergence Monitoring**
All iterative algorithms include convergence monitoring:
```rust
struct ConvergenceMonitor {
    max_iterations: usize,
    tolerance: f64,
    damping_factor: f64,
    history: Vec<f64>,  // Track changes over iterations
}

impl ConvergenceMonitor {
    fn check_convergence(&self, current_change: f64) -> ConvergenceStatus {
        if current_change < self.tolerance {
            ConvergenceStatus::Converged
        } else if self.history.len() >= self.max_iterations {
            ConvergenceStatus::MaxIterationsReached
        } else {
            ConvergenceStatus::Continue
        }
    }
}
```

### Structure Learning Module

The structure learning module discovers network topology from data.

**Score-Based Methods**

*Search Algorithms:*
- Hill climbing with random restarts
- Simulated annealing with adaptive cooling
- Genetic algorithms for population-based search
- Exact methods (dynamic programming, integer programming) for small networks

*Scoring Functions:*
- BIC (Bayesian Information Criterion)
- BDeu (Bayesian Dirichlet equivalent uniform)
- MDL (Minimum Description Length)
- AIC (Akaike Information Criterion)
- Custom user-defined scores

*Scoring Optimization:*
- Local scoring for efficient parent set evaluation
- Cache-friendly score computation
- BIC pre-computation for discrete variables

**Constraint-Based Methods**

*Independence Testing:*
- Chi-square test for categorical variables
- G-test (likelihood ratio)
- Fisher's Z-test for continuous variables
- Kernel-based tests for non-linear dependencies

*Algorithms:*
- PC algorithm (Peter-Clark)
- IC algorithm (Inductive Causation)
- FCI algorithm (Fast Causal Inference) for latent variables
- RFCI (Really Fast Causal Inference)

**Hybrid Methods**

Combining score-based and constraint-based approaches:
- Use constraint-based methods to restrict search space
- Apply score-based optimization within restricted space
- Max-Min Hill Climbing (MMHC)
- Hybrid HPC (Hiton PC)

### Parameter Learning Module

The parameter learning module estimates conditional probability distributions from data.

**Maximum Likelihood Estimation (MLE)**
- Direct counting for complete data
- Laplace smoothing for zero-frequency handling
- Add-one smoothing variants

**Bayesian Estimation**
- Dirichlet priors for discrete variables
- BDeu prior for structure learning consistency
- Normal-Wishart priors for Gaussian variables
- User-specified priors

**Expectation-Maximization (EM)**
- Standard EM for networks with latent variables
- Structural EM for structure and parameter learning
- Online EM for streaming data
- Hard EM (classification EM) for faster convergence

**Gradient-Based Methods**
- Gradient descent for differentiable parameterizations
- Natural gradient for information geometry
- L-BFGS for large-scale optimization
- Conjugate gradient methods

### Data I/O Module

The data I/O module handles importing and exporting models and data.

**Format Adapters**

*Graph Formats:*
- GraphML (XML-based, widely supported)
- GEXF (Gephi exchange format)
- Pajek (.net format)
- Edge list (simple text format)
- Adjacency matrix (CSV, NumPy)

*Probabilistic Model Formats:*
- BIF (Bayesian Interchange Format)
- XMLBIF (XML version of BIF)
- UAI (Uncertainty in AI competition format)
- PMML (Predictive Model Markup Language)
- Custom JSON format with schema

*Data Formats:*
- CSV/TSV with automatic type inference
- HDF5 for large datasets
- Parquet for columnar storage
- NumPy arrays (.npy, .npz)
- Pandas DataFrames (via Python bindings)

**Validation**

All input undergoes validation:
- Schema validation for structured formats
- Semantic validation (acyclicity for BNs, consistency checks)
- Data type compatibility
- Missing value handling strategies

### Query Interface Module

The query interface module provides high-level APIs for extracting information from models.

**D-Separation Testing**
- Path-based d-separation algorithm
- Bayes ball algorithm for efficiency
- Batch testing for multiple queries

**Marginal Queries**
- Single variable marginals
- Joint marginals for variable sets
- Conditional marginals (posterior distributions)

**MAP Inference**
- Exact MAP via variable elimination
- Approximate MAP via local search
- Integer programming formulations
- Dual decomposition for complex queries

**Causal Queries**
- Interventional queries using do-calculus
- Counterfactual reasoning
- Causal effect estimation
- Identification algorithms (Tian-Shpitser, ID algorithm)

---

## The Multi-Representation Architecture

### Unified Factor Graph Representation

Lutufi's multi-representation architecture enables users to work with familiar model types (Bayesian networks, Markov random fields) while the system internally uses a unified factor graph representation for all operations.

**Conversion Architecture:**

```
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│  User-Facing    │      │   Conversion    │      │   Canonical     │
│  Representation │◄────►│    Layer        │◄────►│  Representation │
│  (BN/MRF/DBN)   │      │                 │      │ (Factor Graph)  │
└─────────────────┘      └─────────────────┘      └─────────────────┘
```

**Benefits of Unified Representation:**

1. **Single Algorithm Implementation**: Inference and learning algorithms are written once against the factor graph interface
2. **Consistent Semantics**: Operations have well-defined meaning regardless of input representation
3. **Interoperability**: Models can be freely converted between representations
4. **Extensibility**: New model types require only conversion functions

### Bayesian Network to Factor Graph Conversion

**Conversion Process:**

1. **Variable Mapping**: Each BN variable becomes a factor graph variable node
2. **CPD to Factor**: Each CPD P(X|Parents(X)) becomes a factor node
3. **Edge Creation**: Connect each factor to its corresponding variable and its parents

**Example:**
```
BN Structure:     A → B → C
                  ↓       ↑
                  └── D ──┘

CPDs:
  P(A), P(B|A), P(C|B,D), P(D|A)

Factor Graph:
  Variables: A, B, C, D
  Factors: f_A(A), f_B(A,B), f_C(B,C,D), f_D(A,D)
  Edges: f_A-A, f_B-A, f_B-B, f_C-B, f_C-C, f_C-D, f_D-A, f_D-D
```

**Semantic Preservation:**
- The joint distribution is preserved: P(A,B,C,D) = f_A * f_B * f_C * f_D
- Conditional independence structure is preserved
- Evidence propagation semantics are identical

### Markov Random Field to Factor Graph Conversion

MRFs already use factor representations, making conversion straightforward:

**Conversion Process:**

1. **Variable Mapping**: Each MRF variable becomes a factor graph variable node
2. **Clique to Factor**: Each maximal clique potential becomes a factor node
3. **Edge Creation**: Connect each factor to its corresponding clique variables

**Example:**
```
MRF Structure:    A — B
                  | X |
                  C — D

Cliques: {A,B,C}, {B,C,D}
Potentials: φ_ABC(A,B,C), φ_BCD(B,C,D)

Factor Graph:
  Variables: A, B, C, D
  Factors: f_ABC(A,B,C), f_BCD(B,C,D)
  Edges: f_ABC-A, f_ABC-B, f_ABC-C, f_BCD-B, f_BCD-C, f_BCD-D
```

### Dynamic Bayesian Network Representation

DBNs are handled by unrolling or using specialized factor graph extensions:

**Time-Slice Representation:**
- Each time-slice is a factor graph
- Inter-slice factors connect variables across time
- Initial slice (t=0) has special prior factors

**Rolling Window for Streaming:**
- Maintain fixed number of recent time-slices
- Discard older slices while preserving sufficient statistics
- Efficient for online inference

### Conversion Strategies and Optimization

**Lazy Conversion:**
Models are converted to factor graphs only when needed for inference or learning. User-facing operations on the original representation (structure queries, visualization) avoid conversion overhead.

**Incremental Updates:**
When models are modified (adding nodes, changing CPDs), only affected factors are recomputed, not the entire factor graph.

**Caching:**
Converted factor graphs are cached with invalidation on model changes:
```rust
struct ModelCache {
    factor_graph: Option<FactorGraph>,
    version: u64,  // Increment on model changes
    last_accessed: Instant,
}

impl ModelCache {
    fn get_factor_graph(&mut self, model: &Model) -> &FactorGraph {
        if self.version != model.version() || self.factor_graph.is_none() {
            self.factor_graph = Some(convert_to_factor_graph(model));
            self.version = model.version();
        }
        self.last_accessed = Instant::now();
        self.factor_graph.as_ref().unwrap()
    }
}
```

---

## Plugin Architecture

### Extensibility Design

Lutufi implements a plugin architecture that allows extension without modifying core code. The plugin system is based on:

- **Trait-based interfaces** (Rust) defining extension points
- **Dynamic loading** of plugin libraries
- **Registration patterns** for discovery
- **Configuration-based** plugin activation

### Algorithm Plugins

New inference and learning algorithms can be added as plugins:

**Plugin Interface:**
```rust
trait InferenceAlgorithm: Send + Sync {
    fn name(&self) -> &str;
    fn capabilities(&self) -> AlgorithmCapabilities;
    fn can_solve(&self, query: &Query, model: &FactorGraph) -> bool;
    fn solve(&self, query: &Query, model: &FactorGraph) -> Result<QueryResult, InferenceError>;
}

struct AlgorithmCapabilities {
    supports_exact: bool,
    supports_continuous: bool,
    max_treewidth: Option<usize>,
    parallelizable: bool,
}
```

**Registration:**
```rust
// In plugin library
inventory::submit! {
    InferencePlugin::new("custom_bp", || Box::new(CustomBeliefPropagation::new()))
}
```

### Format Plugins

Custom file formats are supported through format plugins:

```rust
trait FormatPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn extensions(&self) -> &[&str];
    fn can_read(&self, source: &mut dyn Read) -> bool;
    fn read(&self, source: &mut dyn Read) -> Result<Box<dyn Model>, FormatError>;
    fn write(&self, model: &dyn Model, destination: &mut dyn Write) -> Result<(), FormatError>;
}
```

### Custom CPD Types

Users can define custom conditional probability distributions:

```rust
trait CPD: Factor + Send + Sync {
    fn variable(&self) -> VariableId;
    fn parents(&self) -> &[VariableId];
    fn fit(&mut self, data: &DataFrame) -> Result<(), LearningError>;
    fn sample<R: Rng>(&self, parent_values: &Assignment, rng: &mut R) -> Value;
}

// Example: Neural network CPD
struct NeuralNetCPD {
    variable: VariableId,
    parents: Vec<VariableId>,
    network: NeuralNetwork,
}

impl CPD for NeuralNetCPD {
    // Implementation details...
}
```

### Plugin Discovery and Loading

Plugins are discovered through:

1. **Compile-time registration** using inventory/linkme macros
2. **Runtime loading** from shared libraries in plugin directories
3. **Explicit registration** in configuration files

**Configuration:**
```yaml
# lutufi.yaml
plugins:
  inference:
    - name: "custom_bp"
      path: "/path/to/custom_bp.so"
      priority: 100
  formats:
    - name: "special_format"
      path: "/path/to/special_format.so"
```

---

## Memory Management

### Sparse Representation Strategy

Probabilistic graphical models often exhibit sparsity that can be exploited for memory efficiency:

**Structural Sparsity:**
- Factor graphs typically have sparse connectivity
- Adjacency stored in compressed sparse formats (CSR/CSC)
- Neighbor lists use adaptive storage (small vectors for few neighbors)

**Value Sparsity:**
- CPD tables often contain many zeros or near-zeros
- Factors use sparse tensor representations when beneficial
- Threshold-based sparsification for approximate inference

**Sparse Factor Storage:**
```rust
enum FactorStorage {
    // Dense n-dimensional array
    Dense { values: Vec<f64>, strides: Vec<usize> },
    // Coordinate format: (index_tuple, value) pairs
    SparseCOO { entries: Vec<(Vec<usize>, f64)> },
    // Compressed format for high sparsity
    SparseCSR { values: Vec<f64>, indices: Vec<usize>, indptr: Vec<usize> },
    // Function-based for complex distributions
    Functional { eval: Box<dyn Fn(&Assignment) -> f64> },
}
```

### Lazy Evaluation

Expensive computations are deferred until their results are needed:

**Lazy Factor Operations:**
Factor multiplication and marginalization return proxy objects that compute values on demand:
```rust
struct LazyProductFactor {
    factors: Vec<Box<dyn Factor>>,
    // Actual multiplication performed only when evaluate() is called
}

impl Factor for LazyProductFactor {
    fn evaluate(&self, assignment: &Assignment) -> f64 {
        self.factors.iter()
            .map(|f| f.evaluate(assignment))
            .product()
    }
}
```

**Lazy Message Computation:**
In belief propagation, messages are computed only when requested by receiving nodes.

### Memory-Mapped Files

For large networks that exceed available RAM:

**Memory-Mapped Storage:**
- Large factor tables stored in memory-mapped files
- Operating system handles paging between disk and memory
- Transparent access through pointer-like interfaces

**Paged Data Structures:**
```rust
struct MemoryMappedFactor {
    file: MmapMut,
    shape: Vec<usize>,
    dtype: DataType,
    cache: LruCache<Vec<usize>, f64>,  // Cache recently accessed values
}

impl Factor for MemoryMappedFactor {
    fn evaluate(&self, assignment: &Assignment) -> f64 {
        let index = self.compute_index(assignment);
        
        // Check cache first
        if let Some(value) = self.cache.get(&index) {
            return *value;
        }
        
        // Read from memory-mapped file
        let offset = self.compute_offset(index);
        let value = self.read_value_at(offset);
        self.cache.put(index, value);
        value
    }
}
```

**Out-of-Core Algorithms:**
Inference algorithms adapted to work with disk-resident data:
- Streaming variable elimination
- Disk-based junction tree with selective loading
- Mini-batch sampling for large datasets

### Memory Pools and Arenas

Frequent allocation of small objects is optimized through memory pools:

**Arena Allocation:**
```rust
struct FactorGraphArena {
    variable_pool: Arena<Variable>,
    factor_pool: Arena<Box<dyn Factor>>,
    edge_pool: Arena<Edge>,
}

impl FactorGraphArena {
    fn new_variable(&self, name: String, domain: Domain) -> &mut Variable {
        self.variable_pool.alloc(Variable::new(name, domain))
    }
    
    // All allocations are freed when arena is dropped
}
```

**Benefits:**
- Reduced allocation overhead
- Improved cache locality
- Predictable memory usage
- Easy bulk deallocation

---

## Concurrency Model

### Thread Safety Considerations

Lutufi distinguishes between:
- **Immutable data**: Safe for concurrent read access
- **Mutable data**: Requires synchronization
- **Per-thread data**: No sharing needed

**Thread-Safe Structures:**
- Factor graphs are immutable after construction (Arc sharing)
- Evidence can be updated atomically
- Inference results are immutable

**Structures Requiring Synchronization:**
- Model caches (read-write locks)
- Learning state (mutex-protected)
- Convergence monitors (atomic operations)

### Parallel Inference

Multiple inference strategies exploit parallelism:

**Data Parallelism:**
- Multiple queries executed concurrently on shared model
- Batch evidence processing
- Cross-validation during learning

**Model Parallelism:**
- Partition large models for distributed inference
- Parallel clique tree operations
- Concurrent factor operations in junction tree

**Task Parallelism:**
```rust
fn parallel_inference(
    model: &FactorGraph,
    queries: &[Query],
    num_threads: usize
) -> Vec<QueryResult> {
    queries.par_iter()
        .map(|query| {
            let engine = InferenceEngine::new(model);
            engine.solve(query)
        })
        .collect()
}
```

### Distributed Computation Hooks

While the core library focuses on single-node performance, hooks are provided for distributed extensions:

**Partitioning Interface:**
```rust
trait GraphPartitioner {
    fn partition(&self, graph: &FactorGraph, num_parts: usize) -> Vec<Partition>;
}

struct Partition {
    id: PartitionId,
    variables: HashSet<VariableId>,
    boundary_variables: HashSet<VariableId>,  // Variables shared with other partitions
}
```

**Message Passing Interface:**
- Standard MPI-like interface for inter-node communication
- Gossip protocols for approximate distributed inference
- Parameter server pattern for distributed learning

### Parallel Message Passing for Belief Propagation

Belief propagation—both exact (on junction trees) and approximate (loopy BP)—is embarrassingly parallel at multiple levels. Lutufi employs a hybrid parallelization strategy:

**1. Variable-Level Parallelism**
Variables with disjoint neighborhoods can receive messages in parallel using graph coloring to identify independent sets.

**2. Factor-Level Parallelism**
Factor nodes can send messages to all adjacent variables in parallel. This coarse-grained approach has lower synchronization overhead.

**3. Clique Tree Parallelism**
For exact inference, cliques can be processed in parallel using wavefront scheduling—cliques at the same depth in the tree can update simultaneously.

**4. Partition-Level Parallelism**
For distributed settings, partition the graph and communicate boundary messages asynchronously.

The system automatically selects the appropriate level based on graph characteristics:
- **Low treewidth (< 10):** Clique tree parallelism for exact inference
- **Dense with small factors:** Variable-level parallelism
- **Large factors:** Factor-level parallelism
- **Very large graphs (>1M nodes):** Partition-level for distributed execution

**Memory and Synchronization Trade-offs:**

| Level | Memory Overhead | Synchronization Pattern | Cache Locality |
|-------|-----------------|------------------------|----------------|
| Variable | Low | Barrier per iteration | Poor (random access) |
| Factor | Medium | Barrier per phase | Good (factor-local) |
| Clique Tree | Medium | Wavefront barrier | Excellent (cache-friendly) |
| Partition | High | Async gossip | Good (partition-local) |

**Future Extensions:**
- Ray integration for Python distributed computing
- Apache Spark bindings for large-scale learning
- Kubernetes operators for cloud deployment

---

## Incremental Inference Subsystem

### Problem: Dynamic Networks

Real social and economic networks are not static—they evolve through time:
- New edges form as relationships develop
- Edges dissolve as relationships end
- Node attributes change (beliefs, health status, financial condition)
- Evidence accumulates from ongoing observation

Recomputing inference from scratch after every change is computationally prohibitive. Lutufi implements an **incremental inference subsystem** that efficiently updates beliefs given network changes.

### State Maintenance Between Updates

The incremental inference subsystem maintains state across updates:

**1. Versioned Beliefs**
```rust
struct VersionedBelief {
    belief: ProbabilityDistribution,
    version: u64,
    timestamp: Instant,
    source: UpdateSource,
}
```

**2. Delta Tracking**
```rust
enum NetworkDelta {
    EdgeAdded { source: NodeId, target: NodeId },
    EdgeRemoved { source: NodeId, target: NodeId },
    EvidenceAdded { variable: VariableId, value: Value },
}
```

**3. Cached Factor Computations**
The system caches factor computations and uses invalidation rules to determine which cached values remain valid after updates.

### Triggers: Full Recomputation vs. Incremental Update

| Scenario | Strategy | Correctness Guarantee |
|----------|----------|----------------------|
| Evidence on leaf node | Incremental | Exact |
| Evidence on high-centrality node | Full recompute | Required for correctness |
| Edge addition creating large clique | Full recompute | Required for efficiency |
| Temporal evolution (DBN) | Incremental with sliding window | Approximate (bounded error) |

### Correctness Guarantees

- **Incremental Update Correctness:** For evidence updates on leaf nodes, incremental update produces results identical to full recomputation.
- **Approximate Update Bounds:** When approximate updates are used, the system tracks error bounds.
- **Rollback Capability:** Versioned beliefs enable rollback to any previous state.

### API for Incremental Updates

```python
# Incremental update: new evidence
model.add_evidence('Observation', value='observed')
result = model.incremental_query(variables=['Risk'])

# Batch updates
delta_tracker = model.batch_updates()
    model.add_evidence('A', 1)
    model.add_edge('B', 'C')
result = model.apply_updates(delta_tracker)
```

---

## Error Handling Strategy

### Error Propagation

Lutufi uses a structured error hierarchy with context propagation:

```rust
#[derive(Error, Debug)]
pub enum LutufiError {
    #[error("Model error: {0}")]
    Model(#[from] ModelError),
    
    #[error("Inference error: {0}")]
    Inference(#[from] InferenceError),
    
    #[error("Learning error: {0}")]
    Learning(#[from] LearningError),
    
    #[error("I/O error: {0}")]
    Io(#[from] IoError),
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}

#[derive(Error, Debug)]
pub enum InferenceError {
    #[error("Algorithm {algorithm} failed to converge after {iterations} iterations")]
    ConvergenceFailure { algorithm: String, iterations: usize },
    
    #[error("Insufficient memory for inference: required {required} MB, available {available} MB")]
    InsufficientMemory { required: usize, available: usize },
    
    #[error("Query {query:?} is intractable for model with treewidth {treewidth}")]
    IntractableQuery { query: Query, treewidth: usize },
}
```

### Graceful Degradation

When exact inference fails, the system automatically falls back to approximate methods:

**Degradation Chain:**
1. Try exact inference (junction tree)
2. If treewidth too high, try bounded treewidth methods
3. If still intractable, use loopy belief propagation
4. If LBP fails to converge, use sampling
5. If all else fails, return error with suggestions

**User Control:**
```python
# Explicit degradation control
result = model.query(
    variables=['X'],
    evidence={'Y': 1},
    exact=True,  # Fail rather than approximate
    timeout=60   # Maximum seconds to wait
)
```

### User-Friendly Error Messages

Error messages are designed to be actionable:

**Example Error Messages:**
- "This network contains cycles which violate the assumption of your chosen inference method. Consider switching to `method='loopy_bp'` or moralizing the graph."
- "Node 'A' has inconsistent probability tables (probabilities sum to 1.03, expected 1.0). Use `normalize=True` to automatically normalize."
- "Evidence value 'blue' for variable 'Color' is not in the domain ['red', 'green', 'yellow']. Check for typos or update the variable definition."
- "Inference would require 47 GB of memory but only 16 GB is available. Consider using approximate inference or reducing the model size."

**Error Context:**
Errors include:
- Location in user code (when available)
- Relevant model state
- Suggested fixes
- Link to documentation

---

## Core Language Decision

### Evaluation Criteria

The choice between Rust and C++ as the core implementation language was evaluated against:

1. **Performance**: Raw computation speed, memory efficiency
2. **Safety**: Memory safety, thread safety, prevention of undefined behavior
3. **Productivity**: Development speed, debugging ease, maintainability
4. **Ecosystem**: Library availability, tooling, community support
5. **FFI**: Ease of creating Python bindings
6. **Long-term viability**: Language trajectory, corporate backing

### Rust Evaluation

**Advantages:**
- **Memory safety without GC**: Zero-cost abstractions with compile-time guarantees
- **Fearless concurrency**: Ownership system prevents data races
- **Modern tooling**: Cargo for build/dependency management, excellent compiler messages
- **FFI**: PyO3 provides excellent Python integration
- **Ecosystem**: Growing scientific computing ecosystem (nalgebra, ndarray)
- **WASM support**: Potential for browser-based deployment

**Challenges:**
- **Learning curve**: Ownership and borrowing concepts require training
- **Ecosystem maturity**: Some specialized scientific libraries less mature than C++
- **Compilation times**: Slower than C++ for large projects
- **Debugging**: Less mature debugging tools than C++

### C++ Evaluation

**Advantages:**
- **Mature ecosystem**: Extensive scientific computing libraries (Eigen, Boost)
- **Industry standard**: Widely used in high-performance computing
- **Tuning control**: Fine-grained control over memory and optimization
- **pybind11**: Mature Python binding library

**Challenges:**
- **Memory safety**: Manual memory management leads to vulnerabilities
- **Build complexity**: CMake configuration can be complex
- **Longer development**: More code required for equivalent functionality

### Recommendation: Rust

**Rationale:**
1. **Safety-first**: For a library handling complex data structures, memory safety is critical
2. **Concurrency**: Rust's ownership model makes parallel inference implementation safer
3. **Modern development**: Cargo and Rust's tooling streamline development
4. **Growing adoption**: Increasing use in scientific computing (polars, rust-numpy)
5. **Python integration**: PyO3 provides seamless Python bindings

**Mitigations:**
- Use well-vetted crates for linear algebra (nalgebra, ndarray)
- Contribute to ecosystem gaps rather than reinventing
- Maintain comprehensive test suite for unsafe code sections
- Provide excellent documentation to help contributors

---

## Python Binding Architecture

### PyO3 vs pybind11

**PyO3** is recommended for Lutufi based on:

1. **Rust-native**: No C++ intermediary needed
2. **Ergonomics**: Rust macros provide cleaner binding code
3. **Type safety**: Compile-time checking of Python API conformance
4. **Performance**: Minimal overhead in translation layer
5. **Maturity**: Production-ready with extensive documentation

**Example PyO3 Binding:**
```rust
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
struct PyBayesianNetwork {
    inner: BayesianNetwork,
}

#[pymethods]
impl PyBayesianNetwork {
    #[new]
    fn new() -> Self {
        PyBayesianNetwork {
            inner: BayesianNetwork::new(),
        }
    }
    
    fn query(&self, py: Python, variables: Vec<String>, evidence: Option<&PyDict>) -> PyResult<PyObject> {
        let evidence_map = evidence.map(|e| convert_dict_to_evidence(e))
            .transpose()?
            .unwrap_or_default();
        
        let result = self.inner.query(&variables, &evidence_map)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        
        convert_result_to_python(py, result)
    }
    
    #[staticmethod]
    fn fit(data: &PyArray2<f64>) -> PyResult<Self> {
        let rust_array = data.as_array();
        let model = BayesianNetwork::fit(&rust_array)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        
        Ok(PyBayesianNetwork { inner: model })
    }
}
```

### API Translation Layer

The translation layer handles:

**Type Conversions:**
- Python `dict` ↔ Rust `HashMap`
- NumPy `ndarray` ↔ Rust `ndarray::Array`
- Pandas `DataFrame` ↔ Rust data frame representation
- Python `list`/`tuple` ↔ Rust `Vec`

**Error Translation:**
```rust
impl From<LutufiError> for PyErr {
    fn from(err: LutufiError) -> PyErr {
        match err {
            LutufiError::Validation(e) => PyValueError::new_err(e.to_string()),
            LutufiError::Inference(e) => PyRuntimeError::new_err(e.to_string()),
            LutufiError::Io(e) => PyIOError::new_err(e.to_string()),
            _ => PyException::new_err(err.to_string()),
        }
    }
}
```

### NumPy Integration

Zero-copy data exchange with NumPy:

```rust
use numpy::{PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};

#[pymethods]
impl PyBayesianNetwork {
    fn get_cpd_matrix(&self, py: Python, variable: &str) -> PyResult<&PyArray2<f64>> {
        let cpd = self.inner.get_cpd(variable)
            .ok_or_else(|| PyKeyError::new_err(format!("Variable {} not found", variable)))?;
        
        let matrix = cpd.as_matrix();
        Ok(PyArray2::from_array(py, &matrix))
    }
    
    fn set_cpd_from_array(&mut self, variable: &str, array: PyReadonlyArray2<f64>) -> PyResult<()> {
        let view = array.as_array();
        self.inner.set_cpd(variable, view.to_owned())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
```

### pandas/NetworkX Compatibility

**pandas Integration:**
```python
import lutufi as lf
import pandas as pd

# DataFrame input for learning
df = pd.read_csv('data.csv')
model = lf.BayesianNetwork.fit(df, algorithm='hc')

# DataFrame output for queries
result = model.query(variables=['A', 'B'])
result_df = result.to_dataframe()  # Returns pandas DataFrame
```

**NetworkX Integration:**
```python
import networkx as nx
import lutufi as lf

# Convert NetworkX graph to Bayesian network
nx_graph = nx.DiGraph()
nx_graph.add_edges_from([('A', 'B'), ('B', 'C')])
model = lf.BayesianNetwork.from_networkx(nx_graph)

# Convert back to NetworkX for visualization
nx_viz = model.to_networkx()
nx.draw(nx_viz, with_labels=True)
```

---

## Build System and Packaging

### Rust Build System (Cargo)

**Workspace Structure:**
```
lutufi/
├── Cargo.toml              # Workspace root
├── lutufi-core/            # Core library
│   └── Cargo.toml
├── lutufi-python/          # Python bindings (PyO3)
│   └── Cargo.toml
├── lutufi-ffi/             # C FFI bindings
│   └── Cargo.toml
└── lutufi-cli/             # Command-line interface
    └── Cargo.toml
```

**Core Cargo.toml:**
```toml
[package]
name = "lutufi-core"
version = "0.1.0"
edition = "2021"
authors = ["Wasswa Lutufi Sebbanja <author@example.com>"]
license = "Apache-2.0"
repository = "https://github.com/lutufi/lutufi"

[dependencies]
ndarray = { version = "0.15", features = ["rayon"] }
nalgebra = "0.32"
rayon = "1.7"
petgraph = "0.6"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
tracing = "0.1"

[dev-dependencies]
criterion = "0.5"
proptest = "1.2"
```

### Python Wheel Generation

**maturin** is used for building and publishing Python wheels:

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "lutufi"
version = "0.1.0"
description = "Unified Bayesian networks and network analysis"
readme = "README.md"
license = {text = "Apache-2.0"}
requires-python = ">=3.9"
classifiers = [
    "Development Status :: 3 - Alpha",
    "Intended Audience :: Science/Research",
    "License :: OSI Approved :: Apache Software License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Rust",
]
dependencies = [
    "numpy>=1.20",
    "pandas>=1.3",
    "networkx>=2.6",
]
```

**CI/CD for Wheels:**
```yaml
# .github/workflows/wheels.yml
name: Build Wheels

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        python: ['3.9', '3.9', '3.10', '3.11', '3.12']
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: ${{ matrix.python }}
      - uses: PyO3/maturin-action@v1
        with:
          command: build
          args: --release --strip
      - uses: actions/upload-artifact@v3
        with:
          name: wheels
          path: target/wheels/*.whl
```

### Platform Support

**Tier 1 (Fully Supported):**
- Linux x86_64 (glibc 2.17+)
- macOS x86_64 (10.14+)
- macOS ARM64 (Apple Silicon)
- Windows x86_64

**Tier 2 (Best Effort):**
- Linux ARM64
- Linux aarch32
- Windows ARM64

**Tier 3 (Community):**
- FreeBSD
- Other platforms via source compilation

---

## Versioning and Compatibility

### Semantic Versioning

Lutufi follows SemVer (MAJOR.MINOR.PATCH):

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality, backward compatible
- **PATCH**: Bug fixes, backward compatible

**Pre-1.0 Policy:**
While in 0.x, MINOR versions may include breaking changes, but these will be documented in migration guides.

### ABI Compatibility

Rust ABI is not stable, so:
- Each MAJOR.MINOR version has distinct ABI
- Plugins must be compiled against specific versions
- FFI exports use C ABI for stability

### Migration Strategies

**Deprecation Policy:**
1. Features marked deprecated in version N
2. Deprecation warnings issued during N
3. Features removed in version N+2
4. Migration guide provided for each removal

**Example Deprecation:**
```rust
#[deprecated(
    since = "0.5.0",
    note = "Use `InferenceEngine::new()` instead. Will be removed in 0.7.0"
)]
pub fn create_engine() -> InferenceEngine {
    InferenceEngine::new()
}
```

**Python API Deprecation:**
```python
import warnings

def old_function():
    warnings.warn(
        "old_function() is deprecated, use new_function() instead. "
        "Will be removed in version 0.7.0.",
        DeprecationWarning,
        stacklevel=2
    )
    return new_function()
```

---

## Architectural Decisions Log

### ADR-001: Factor Graphs as Canonical Representation

**Status:** Accepted  
**Date:** March 2026  
**Context:** Need to support Bayesian networks, Markov random fields, and dynamic Bayesian networks with unified algorithms.

**Decision:** Use factor graphs as the canonical internal representation, with bidirectional conversion from user-facing model types.

**Consequences:**
- (+) Single algorithm implementation for all model types
- (+) Clear semantic foundation based on factor graph theory
- (+) Easy to add new model types via conversions
- (-) Conversion overhead for some operations
- (-) May obscure domain-specific optimizations

### ADR-002: Rust as Core Implementation Language

**Status:** Accepted  
**Date:** March 2026  
**Context:** Need high performance with memory safety for complex graph algorithms.

**Decision:** Implement core library in Rust, with Python bindings via PyO3.

**Consequences:**
- (+) Memory safety without garbage collection
- (+) Fearless concurrency for parallel inference
- (+) Modern tooling and package management
- (-) Smaller ecosystem than C++ for scientific computing
- (-) Steeper learning curve for contributors

### ADR-003: PyO3 for Python Bindings

**Status:** Accepted  
**Date:** March 2026  
**Context:** Need efficient, maintainable Python bindings.

**Decision:** Use PyO3 crate for Rust-Python bindings rather than C++ intermediaries.

**Consequences:**
- (+) Native Rust solution, no C++ bridge needed
- (+) Compile-time type checking of Python API
- (+) Excellent NumPy integration
- (-) Ties Python bindings to Rust ecosystem

### ADR-004: Plugin Architecture for Extensibility

**Status:** Accepted  
**Date:** March 2026  
**Context:** Need to support custom algorithms, formats, and CPD types without core modifications.

**Decision:** Implement trait-based plugin system with dynamic loading support.

**Consequences:**
- (+) Core library remains focused and maintainable
- (+) Users can extend without forking
- (+) Clear extension points defined by traits
- (-) Plugin API stability requires careful design
- (-) Dynamic loading adds complexity

### ADR-005: Lazy Evaluation for Factor Operations

**Status:** Accepted  
**Date:** March 2026  
**Context:** Factor multiplication and marginalization can be expensive; not always immediately needed.

**Decision:** Implement lazy evaluation for factor operations, computing only when values are requested.

**Consequences:**
- (+) Reduced computation for partial queries
- (+) Opportunities for optimization before execution
- (-) More complex debugging
- (-) Memory overhead for operation graphs

### ADR-006: Memory-Mapped Files for Large Networks

**Status:** Accepted  
**Date:** March 2026  
**Context:** Networks may exceed available RAM, especially with large CPD tables.

**Decision:** Support memory-mapped file storage for large factors with transparent access.

**Consequences:**
- (+) Enables working with very large networks
- (+) OS-managed paging is efficient
- (-) Performance depends on access patterns
- (-) Adds platform-specific code

---

## Key References

### Software Architecture Patterns

1. **Martin, R. C. (2017).** *Clean Architecture: A Craftsman's Guide to Software Structure and Design.* Prentice Hall.
   - Layered architecture and dependency management

2. **Richards, M., & Ford, N. (2020).** *Fundamentals of Software Architecture: An Engineering Approach.* O'Reilly Media.
   - Architectural patterns and trade-off analysis

3. **Newman, S. (2021).** *Building Microservices: Designing Fine-Grained Systems.* O'Reilly Media.
   - Component design principles (adapted for library architecture)

4. **Microsoft (2023).** *Azure Architecture Center.* https://docs.microsoft.com/en-us/azure/architecture/
   - Cloud-native design patterns

### Related Library Architectures

5. **PyTorch Team (2023).** *PyTorch Internals.* https://pytorch.org/docs/stable/notes/internals.html
   - Python binding architecture and autograd system

6. **NumPy Developers (2023).** *NumPy Internal Documentation.* https://numpy.org/doc/stable/dev/internals.html
   - Array handling and C integration patterns

7. **scikit-learn Developers (2023).** *Developing scikit-learn Estimators.* https://scikit-learn.org/stable/developers/develop.html
   - API design patterns for ML libraries

8. **NetworkX Developers (2023).** *NetworkX Documentation.* https://networkx.org/documentation/stable/
   - Graph library design and algorithm organization

9. **Stan Development Team (2023).** *Stan Math Library.* https://mc-stan.org/math/
   - Automatic differentiation and probabilistic computation

10. **pgmpy Team (2023).** *pgmpy Documentation.* https://pgmpy.org/
    - Probabilistic graphical model library design (Python reference)

### Scientific Computing

11. **Saad, Y. (2003).** *Iterative Methods for Sparse Linear Systems.* SIAM.
    - Sparse matrix formats and algorithms

12. **Golub, G. H., & Van Loan, C. F. (2013).** *Matrix Computations.* Johns Hopkins University Press.
    - Numerical linear algebra foundations

13. **Sanders, P., & Schulz, C. (2013).** "Think Locally, Act Globally: Highly Balanced Graph Partitioning." *SEA 2013.*
    - Graph partitioning algorithms for distributed computation

### Factor Graphs and Inference

14. **Kschischang, F. R., Frey, B. J., & Loeliger, H. A. (2001).** "Factor Graphs and the Sum-Product Algorithm." *IEEE Transactions on Information Theory,* 47(2), 498-519.
    - Theoretical foundation for factor graph representation

15. **Mooij, J. M. (2010).** *libDAI: A Free and Open Source C++ Library for Discrete Approximate Inference in Graphical Models.* https://github.com/dbaml/libDAI
    - Reference implementation architecture

16. **Koller, D., & Friedman, N. (2009).** *Probabilistic Graphical Models: Principles and Techniques.* MIT Press.
    - Algorithm design and implementation considerations

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial draft |
| 1.0 | 2026-03-03 | Wasswa Lutufi Sebbanja | Complete architecture document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
