# Lutufi Data Model Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Overview](#overview)
3. [The Factor Graph as Canonical Representation](#the-factor-graph-as-canonical-representation)
4. [Core Data Structures](#core-data-structures)
5. [Conditional Probability Distributions](#conditional-probability-distributions)
6. [Graph Representations](#graph-representations)
7. [Belief Representations](#belief-representations)
8. [Evidence Representation](#evidence-representation)
9. [Temporal Representation](#temporal-representation)
10. [Missing Data Representation](#missing-data-representation)
11. [Serialization Format](#serialization-format)
12. [Memory Layout](#memory-layout)
13. [Indexing and Lookup](#indexing-and-lookup)
14. [Immutable vs Mutable Design](#immutable-vs-mutable-design)
15. [Integration with External Libraries](#integration-with-external-libraries)
16. [Thread Safety](#thread-safety)
17. [Key References](#key-references)

---

## Executive Summary

This document defines the internal data model for the Lutufi library—the representations used to store probabilistic graphical models, their parameters, inference state, and learning artifacts. The data model is designed for efficiency, expressiveness, and seamless serialization.

Key design decisions include:
- **Factor graphs as the canonical internal representation** for all model types
- **Sparse data structures** throughout to exploit structural sparsity
- **Type-safe representations** distinguishing discrete, continuous, and hybrid variables
- **Lazy evaluation patterns** to defer expensive computations
- **Memory-efficient serialization** supporting large-scale models

---

## Overview

### What the Data Model Represents

The Lutufi data model represents:

1. **Probabilistic Graphical Models**: The structure and parameters of Bayesian networks, Markov random fields, and factor graphs
2. **Probability Distributions**: Tabular CPDs, Gaussian distributions, and custom parametric forms
3. **Inference State**: Messages, beliefs, and intermediate computations during inference
4. **Evidence**: Hard observations, soft evidence, and virtual evidence
5. **Learning State**: Sufficient statistics, gradients, and optimization state
6. **Temporal Dynamics**: Time-sliced representations and transition models

### Design Goals

**Efficiency:**
- Minimize memory footprint through sparse representations
- Maximize cache locality for graph traversal operations
- Support out-of-core computation for very large models
- Enable SIMD operations where applicable

**Expressiveness:**
- Support discrete, continuous, and hybrid variables
- Represent arbitrary factor graph structures
- Accommodate custom probability distributions
- Support temporal and dynamic models

**Serialization:**
- Fast binary serialization for model persistence
- Human-readable formats for debugging
- Backward compatibility across versions
- Streaming support for incremental loading

---

## The Factor Graph as Canonical Representation

### Why Factor Graphs?

Lutufi uses **factor graphs** as the canonical internal representation for all probabilistic models. This decision is based on several key advantages:

**1. Universality**
Any probabilistic graphical model can be represented as a factor graph:
- Bayesian networks: Convert CPDs to factors
- Markov random fields: Factors correspond to clique potentials
- Conditional random fields: Factors include feature functions
- Dynamic Bayesian networks: Factors span time slices

**2. Explicit Structure**
Factor graphs make variable dependencies explicit through the bipartite structure:
- Variable nodes clearly separated from factor nodes
- No ambiguity about factor scope
- Natural representation for message passing algorithms

**3. Algorithmic Efficiency**
Many inference algorithms have natural implementations on factor graphs:
- Sum-product belief propagation operates directly on factor graph edges
- Variable elimination corresponds to factor marginalization
- Junction tree construction starts from factor graph structure

**4. Extensibility**
New factor types can be added without changing the core data model:
- Neural network factors
- Gaussian process factors
- Custom user-defined factors

### Factor Graph Definition

A factor graph G = (V, F, E) is a bipartite graph consisting of:

- **V**: Set of variable nodes, each representing a random variable Xᵢ
- **F**: Set of factor nodes, each representing a function fⱼ over a subset of variables
- **E ⊆ V × F**: Edges connecting variables to factors they participate in

The joint probability distribution is:

```
P(X₁, ..., Xₙ) = (1/Z) ∏ⱼ fⱼ(scope(fⱼ))
```

where Z is the partition function (normalizing constant).

### Advantages Over Alternative Representations

| Aspect | Factor Graphs | Bayesian Networks | Markov Random Fields |
|--------|---------------|-------------------|----------------------|
| Factorization clarity | Explicit | Implicit via CPDs | Explicit via potentials |
| Message passing | Natural | Requires conversion | Requires conversion |
| Mixed models | Straightforward | Requires moralization | Requires directionality |
| Custom factors | Direct support | Requires CPD extension | Requires potential extension |
| Implementation complexity | Simple | Moderate | Moderate |

---

## Core Data Structures

### Variable

The `Variable` structure represents a random variable in the model.

```rust
struct Variable {
    /// Unique identifier (compact integer for efficiency)
    id: VariableId,
    
    /// Human-readable name
    name: String,
    
    /// Variable domain (possible values)
    domain: Domain,
    
    /// Variable type classification
    variable_type: VariableType,
    
    /// Evidence associated with this variable (if any)
    evidence: Option<Evidence>,
    
    /// User-defined metadata
    metadata: HashMap<String, MetadataValue>,
}

type VariableId = u32;

enum VariableType {
    /// Discrete variable with finite states
    Discrete,
    /// Continuous variable with real values
    Continuous,
    /// Hybrid: mixture of discrete and continuous
    Hybrid,
    /// Ordinal: ordered discrete values
    Ordinal,
}
```

**Key Design Decisions:**
- VariableId is a compact integer (u32) rather than string for memory efficiency
- Names are stored separately and only used for user-facing operations
- Evidence is stored at the variable level for efficient access during inference

### Domain

The `Domain` enum represents the set of possible values for a variable.

```rust
enum Domain {
    /// Discrete domain with explicit category values
    Discrete {
        /// Ordered list of category values
        values: Vec<DiscreteValue>,
        
        /// Map for fast value-to-index lookup
        value_to_index: HashMap<DiscreteValue, usize>,
    },
    
    /// Continuous domain with optional bounds
    Continuous {
        /// Lower bound (if any)
        lower: Option<f64>,
        /// Upper bound (if any)
        upper: Option<f64>,
        /// Discretization grid (optional, for approximation)
        discretization: Option<DiscretizationGrid>,
    },
    
    /// Mixed domain for hybrid variables
    Mixed {
        /// Discrete component
        discrete: Vec<DiscreteValue>,
        /// Continuous component
        continuous: ContinuousDomain,
    },
    
    /// Integer domain for count data
    Integer {
        lower: Option<i64>,
        upper: Option<i64>,
    },
}

enum DiscreteValue {
    String(String),
    Integer(i64),
    Boolean(bool),
}

struct DiscretizationGrid {
    /// Number of bins
    bins: usize,
    /// Bin edges
    edges: Vec<f64>,
    /// Bin centers (for representation)
    centers: Vec<f64>,
}
```

**Key Design Decisions:**
- Discrete domains maintain both value list and reverse map for O(1) lookup
- Continuous domains support optional discretization for algorithms requiring discrete approximation
- Mixed domains enable hybrid Bayesian networks

### Factor

The `Factor` trait defines the interface for factors in a factor graph.

```rust
/// A factor in a factor graph represents a function over a subset of variables.
trait Factor: Send + Sync + Debug {
    /// Get the scope (variables) of this factor
    fn scope(&self) -> &[VariableId];
    
    /// Evaluate the factor at a given assignment
    fn evaluate(&self, assignment: &Assignment) -> f64;
    
    /// Evaluate for a batch of assignments (for vectorization)
    fn evaluate_batch(&self, assignments: &[Assignment]) -> Vec<f64>;
    
    /// Marginalize out variables from this factor
    fn marginalize(&self, variables: &[VariableId]) -> Arc<dyn Factor>;
    
    /// Multiply this factor with another factor
    fn multiply(&self, other: &dyn Factor) -> Arc<dyn Factor>;
    
    /// Divide this factor by another factor
    fn divide(&self, other: &dyn Factor) -> Result<Arc<dyn Factor>, FactorError>;
    
    /// Get the maximum value (for MAP inference)
    fn max(&self) -> f64;
    
    /// Get the argument maximum (assignment with highest value)
    fn argmax(&self) -> Assignment;
    
    /// Convert to a dense representation
    fn to_dense(&self) -> DenseFactor;
    
    /// Get memory size in bytes
    fn memory_bytes(&self) -> usize;
}
```

### Factor Implementations

**DenseFactor:**
```rust
/// Dense multidimensional table representation
struct DenseFactor {
    /// Variables in scope (defines dimensionality)
    scope: Vec<VariableId>,
    
    /// Flattened values in C-order (row-major)
    values: Vec<f64>,
    
    /// Strides for each dimension
    strides: Vec<usize>,
    
    /// Shape (cardinality of each variable)
    shape: Vec<usize>,
}

impl DenseFactor {
    fn get(&self, indices: &[usize]) -> f64 {
        let flat_idx = indices.iter()
            .zip(&self.strides)
            .map(|(i, s)| i * s)
            .sum();
        self.values[flat_idx]
    }
    
    fn set(&mut self, indices: &[usize], value: f64) {
        let flat_idx = indices.iter()
            .zip(&self.strides)
            .map(|(i, s)| i * s)
            .sum();
        self.values[flat_idx] = value;
    }
}
```

**SparseFactor:**
```rust
/// Sparse factor using coordinate format
struct SparseFactor {
    scope: Vec<VariableId>,
    
    /// Non-zero entries: (variable assignment, value)
    entries: Vec<(Vec<usize>, f64)>,
    
    /// Default value for unlisted entries
    default_value: f64,
    
    /// Sorted index for binary search
    sorted_indices: Vec<usize>,
}

/// Compressed sparse factor (CSR-like)
struct CompressedSparseFactor {
    scope: Vec<VariableId>,
    
    /// Non-zero values
    values: Vec<f64>,
    
    /// Column indices (flattened)
    indices: Vec<usize>,
    
    /// Row pointers (one per first variable value)
    indptr: Vec<usize>,
    
    /// Cardinalities
    shape: Vec<usize>,
}
```

**FunctionalFactor:**
```rust
/// Factor defined by a function rather than explicit storage
struct FunctionalFactor<F: Fn(&Assignment) -> f64 + Send + Sync> {
    scope: Vec<VariableId>,
    function: F,
    
    /// Optional: pre-computed values for common assignments
    cache: Option<LruCache<Vec<usize>, f64>>,
}
```

**GaussianFactor:**
```rust
/// Gaussian factor for continuous variables
struct GaussianFactor {
    scope: Vec<VariableId>,
    
    /// Information vector (precision-weighted mean)
    information: DVector<f64>,
    
    /// Precision matrix (inverse covariance)
    precision: DMatrix<f64>,
    
    /// Log normalization constant
    log_normalizer: f64,
}

impl GaussianFactor {
    /// Convert from mean-covariance parameterization
    fn from_mean_covariance(
        scope: Vec<VariableId>,
        mean: DVector<f64>,
        covariance: DMatrix<f64>
    ) -> Self {
        let precision = covariance.try_inverse().expect("Covariance must be invertible");
        let information = &precision * &mean;
        let log_normalizer = -0.5 * (mean.dot(&information) + covariance.determinant().ln());
        
        Self {
            scope,
            information,
            precision,
            log_normalizer,
        }
    }
}
```

### Edge

Edges connect variables to factors in the bipartite factor graph.

```rust
/// Edge in the factor graph connecting variable to factor
struct Edge {
    /// Edge identifier
    id: EdgeId,
    
    /// Variable node (always one end)
    variable: VariableId,
    
    /// Factor node (always other end)
    factor: FactorId,
    
    /// Position of variable in factor's scope
    position_in_factor: usize,
    
    /// Metadata for algorithm-specific information
    metadata: EdgeMetadata,
}

struct EdgeMetadata {
    /// Current message from variable to factor (belief propagation)
    variable_to_factor_message: Option<Message>,
    
    /// Current message from factor to variable (belief propagation)
    factor_to_variable_message: Option<Message>,
    
    /// Message from previous iteration (for convergence checking)
    previous_message: Option<Message>,
    
    /// Weight for weighted belief propagation
    weight: f64,
}

type EdgeId = u64;
```

### FactorGraph

The top-level factor graph structure.

```rust
/// A factor graph representing a joint distribution
struct FactorGraph {
    /// Variables in the graph
    variables: Vec<Variable>,
    
    /// Variable lookup by name
    variable_name_to_id: HashMap<String, VariableId>,
    
    /// Factors in the graph
    factors: Vec<Arc<dyn Factor>>,
    
    /// Edges connecting variables to factors
    edges: Vec<Edge>,
    
    /// Adjacency: variable -> incident edges
    variable_adjacency: Vec<Vec<EdgeId>>,
    
    /// Adjacency: factor -> incident edges
    factor_adjacency: Vec<Vec<EdgeId>>,
    
    /// Sparse adjacency matrix representation
    adjacency_matrix: SparseBipartiteAdjacency,
    
    /// Graph metadata
    metadata: GraphMetadata,
    
    /// Version for cache invalidation
    version: u64,
}

struct GraphMetadata {
    /// Model name
    name: Option<String>,
    
    /// Creation timestamp
    created: DateTime<Utc>,
    
    /// Source information (provenance)
    source: Option<String>,
    
    /// Custom user metadata
    custom: HashMap<String, serde_json::Value>,
}
```

---

## Conditional Probability Distributions

### CPD Hierarchy

```rust
/// Base trait for all conditional probability distributions
trait CPD: Send + Sync + Debug {
    /// The child variable
    fn variable(&self) -> VariableId;
    
    /// Parent variables
    fn parents(&self) -> &[VariableId];
    
    /// Sample from P(X | parents)
    fn sample<R: Rng>(&self, parent_values: &Assignment, rng: &mut R) -> Value;
    
    /// Get probability P(X=x | parents)
    fn probability(&self, value: &Value, parent_values: &Assignment) -> f64;
    
    /// Get log probability
    fn log_probability(&self, value: &Value, parent_values: &Assignment) -> f64;
    
    /// Convert to factor representation
    fn to_factor(&self) -> Arc<dyn Factor>;
    
    /// Learn parameters from data
    fn fit(&mut self, data: &DataFrame) -> Result<(), LearningError>;
    
    /// Clone the CPD
    fn clone_box(&self) -> Box<dyn CPD>;
}
```

### TabularCPD

```rust
/// Tabular (multinomial) CPD for discrete variables
struct TabularCPD {
    variable: VariableId,
    variable_cardinality: usize,
    parents: Vec<VariableId>,
    parent_cardinalities: Vec<usize>,
    
    /// Flattened probability table
    /// Shape: (variable_cardinality, parent_cardinality_1 * ...)
    values: Vec<f64>,
    
    /// Strides for indexing
    strides: Vec<usize>,
}

impl TabularCPD {
    fn new(
        variable: VariableId,
        variable_card: usize,
        values: Vec<f64>,
        parents: Vec<VariableId>,
        parent_cards: Vec<usize>
    ) -> Result<Self, ValidationError> {
        // Validate dimensions
        let expected_len = variable_card * parent_cards.iter().product::<usize>();
        if values.len() != expected_len {
            return Err(ValidationError::DimensionMismatch { ... });
        }
        
        // Validate normalization
        for parent_config in 0..parent_cards.iter().product() {
            let sum: f64 = (0..variable_card)
                .map(|v| values[v * parent_config])
                .sum();
            if (sum - 1.0).abs() > 1e-6 {
                return Err(ValidationError::NonNormalized { ... });
            }
        }
        
        // Compute strides
        let strides = compute_strides(parent_cards);
        
        Ok(Self { ... })
    }
    
    fn get_probability(&self, value: usize, parent_assignment: &[usize]) -> f64 {
        let parent_idx = parent_assignment.iter()
            .zip(&self.strides)
            .map(|(v, s)| v * s)
            .sum::<usize>();
        self.values[value * parent_idx]
    }
}

/// Sparse representation for CPDs with many zeros
struct SparseTabularCPD {
    variable: VariableId,
    parents: Vec<VariableId>,
    
    /// For each parent configuration, sparse representation of child distribution
    conditional_distributions: Vec<SparseDistribution>,
}

struct SparseDistribution {
    /// Non-zero values
    values: Vec<(usize, f64)>,
    /// Default (background) probability for unlisted values
    default: f64,
}
```

### GaussianCPD

```rust
/// Linear Gaussian CPD: X ~ N(β₀ + β₁P₁ + ... + βₖPₖ, σ²)
struct GaussianCPD {
    variable: VariableId,
    parents: Vec<VariableId>,
    
    /// Intercept β₀
    intercept: f64,
    
    /// Coefficients [β₁, ..., βₖ]
    coefficients: Vec<f64>,
    
    /// Variance σ²
    variance: f64,
    
    /// Standard deviation (cached for sampling)
    std_dev: f64,
}

impl GaussianCPD {
    fn mean(&self, parent_values: &[f64]) -> f64 {
        self.intercept + parent_values.iter()
            .zip(&self.coefficients)
            .map(|(p, b)| p * b)
            .sum::<f64>()
    }
    
    fn sample<R: Rng>(&self, parent_values: &[f64], rng: &mut R) -> f64 {
        let mean = self.mean(parent_values);
        rng.sample(StandardNormal) * self.std_dev + mean
    }
    
    fn log_probability(&self, value: f64, parent_values: &[f64]) -> f64 {
        let mean = self.mean(parent_values);
        -0.5 * ((value - mean) / self.std_dev).powi(2)
            - self.std_dev.ln()
            - 0.5 * LN_2PI
    }
}
```

### Hybrid CPDs

```rust
/// Conditional linear Gaussian: mixture of Gaussians based on discrete parents
struct ConditionalLinearGaussianCPD {
    variable: VariableId,
    discrete_parents: Vec<VariableId>,
    continuous_parents: Vec<VariableId>,
    
    /// One GaussianCPD per discrete parent configuration
    components: Vec<GaussianCPD>,
}

/// Mixture of Gaussians CPD
struct MixtureGaussianCPD {
    variable: VariableId,
    parents: Vec<VariableId>,
    
    /// Number of mixture components
    n_components: usize,
    
    /// Mixture weights (as function of parents)
    weight_network: Arc<dyn Factor>,
    
    /// Component parameters
    components: Vec<GaussianCPD>,
}

/// Softmax CPD for discrete variable with continuous parents
struct SoftmaxCPD {
    variable: VariableId,
    variable_cardinality: usize,
    continuous_parents: Vec<VariableId>,
    
    /// Weight matrix: (variable_cardinality, n_continuous_parents)
    weights: DMatrix<f64>,
    
    /// Biases: variable_cardinality
    biases: DVector<f64>,
}

impl SoftmaxCPD {
    fn probabilities(&self, parent_values: &[f64]) -> Vec<f64> {
        let parent_vec = DVector::from_row_slice(parent_values);
        let logits = &self.weights * &parent_vec + &self.biases;
        softmax(logits.as_slice())
    }
}
```

### Custom CPD Interface

```rust
/// Trait for user-defined CPD types
trait CustomCPD: CPD {
    /// Serialize to bytes for persistence
    fn serialize(&self) -> Vec<u8>;
    
    /// Deserialize from bytes
    fn deserialize(bytes: &[u8]) -> Result<Box<dyn CustomCPD>, SerializationError>
    where Self: Sized;
    
    /// Get parameter gradients for learning
    fn gradients(&self, data: &DataFrame) -> Vec<f64>;
    
    /// Update parameters given gradients
    fn apply_gradients(&mut self, gradients: &[f64], learning_rate: f64);
}
```

---

## Graph Representations

### Adjacency Structures

**Dense Adjacency Matrix:**
```rust
/// Dense boolean adjacency matrix
struct DenseAdjacency {
    n_nodes: usize,
    /// Row-major bit matrix
    matrix: Vec<bool>,
}

impl DenseAdjacency {
    fn get(&self, i: usize, j: usize) -> bool {
        self.matrix[i * self.n_nodes + j]
    }
    
    fn set(&mut self, i: usize, j: usize, value: bool) {
        self.matrix[i * self.n_nodes + j] = value;
    }
}
```

**CSR (Compressed Sparse Row):**
```rust
/// CSR format for sparse directed graphs
struct CsrAdjacency {
    n_nodes: usize,
    n_edges: usize,
    
    /// Row pointers: indices[ptr[i]..ptr[i+1]] are successors of i
    indptr: Vec<usize>,
    
    /// Column indices (successor nodes)
    indices: Vec<usize>,
    
    /// Optional edge weights
    data: Option<Vec<f64>>,
}

impl CsrAdjacency {
    /// Get successors of node i
    fn successors(&self, i: usize) -> &[usize] {
        let start = self.indptr[i];
        let end = self.indptr[i + 1];
        &self.indices[start..end]
    }
    
    /// Iterate over all edges
    fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.n_nodes)
            .flat_map(move |i| {
                self.successors(i).iter().map(move |&j| (i, j))
            })
    }
}
```

**CSC (Compressed Sparse Column):**
```rust
/// CSC format for fast predecessor access
struct CscAdjacency {
    n_nodes: usize,
    n_edges: usize,
    
    /// Column pointers
    indptr: Vec<usize>,
    
    /// Row indices (predecessor nodes)
    indices: Vec<usize>,
    
    /// Optional edge weights
    data: Option<Vec<f64>>,
}

impl CscAdjacency {
    /// Get predecessors of node j
    fn predecessors(&self, j: usize) -> &[usize] {
        let start = self.indptr[j];
        let end = self.indptr[j + 1];
        &self.indices[start..end]
    }
}
```

**Edge List:**
```rust
/// Simple edge list for construction and dynamic graphs
struct EdgeList {
    edges: Vec<(usize, usize)>,
    
    /// Optional: sorted indices for binary search
    sorted: bool,
}
```

### Bipartite Factor Graph Adjacency

```rust
/// Specialized adjacency for bipartite factor graphs
struct SparseBipartiteAdjacency {
    /// Number of variable nodes
    n_variables: usize,
    
    /// Number of factor nodes
    n_factors: usize,
    
    /// Variable to factors: var_adj[var_id] = [factor_id, ...]
    var_adj: Vec<Vec<FactorId>>,
    
    /// Factor to variables: factor_adj[factor_id] = [var_id, ...]
    factor_adj: Vec<Vec<VariableId>>,
    
    /// Compressed representation for memory efficiency
    compressed: Option<CompressedBipartiteAdjacency>,
}

struct CompressedBipartiteAdjacency {
    /// Flattened adjacency list
    adj: Vec<FactorId>,
    
    /// Pointers into adj for each variable
    var_ptr: Vec<usize>,
    
    /// Flattened reverse adjacency
    factor_adj: Vec<VariableId>,
    
    /// Pointers for factors
    factor_ptr: Vec<usize>,
}
```

### Dynamic Graph Support

```rust
/// Versioned graph for dynamic modifications
struct VersionedGraph {
    base: FactorGraph,
    
    /// Delta log of modifications
    deltas: Vec<GraphDelta>,
    
    /// Current version number
    version: u64,
}

enum GraphDelta {
    AddVariable(Variable),
    RemoveVariable(VariableId),
    AddFactor(Arc<dyn Factor>),
    RemoveFactor(FactorId),
    AddEdge(Edge),
    RemoveEdge(EdgeId),
    UpdateCPD(VariableId, Box<dyn CPD>),
}

impl VersionedGraph {
    fn apply(&mut self, delta: GraphDelta) {
        self.deltas.push(delta);
        self.version += 1;
        // Apply delta to base graph
    }
    
    fn checkout(&self, version: u64) -> FactorGraph {
        // Replay deltas up to version
        let mut graph = self.base.clone();
        for delta in &self.deltas[..version as usize] {
            graph.apply_delta(delta);
        }
        graph
    }
}
```

---

## Belief Representations

### Belief Storage

Beliefs represent marginal probability distributions during and after inference.

```rust
/// Belief (marginal distribution) for a variable
enum Belief {
    /// Discrete probability distribution
    Discrete {
        /// Probability mass for each value
        probabilities: Vec<f64>,
        /// Value labels (optional)
        labels: Option<Vec<String>>,
    },
    
    /// Gaussian distribution
    Gaussian {
        mean: f64,
        variance: f64,
    },
    
    /// Gaussian mixture
    Mixture {
        components: Vec<(f64, GaussianBelief)>, // (weight, belief)
    },
    
    /// Histogram approximation for continuous
    Histogram {
        bin_edges: Vec<f64>,
        bin_probs: Vec<f64>,
    },
    
    /// Samples (particle representation)
    Particles {
        samples: Vec<f64>,
        weights: Vec<f64>,
    },
}

struct GaussianBelief {
    mean: f64,
    precision: f64,  // 1/variance for numerical stability
}
```

### Marginal Distributions

```rust
/// Marginal distribution for multiple variables
struct JointMarginal {
    /// Variables in the joint
    variables: Vec<VariableId>,
    
    /// Shape (cardinality of each variable)
    shape: Vec<usize>,
    
    /// Probability tensor (flattened)
    probabilities: Vec<f64>,
    
    /// Strides for indexing
    strides: Vec<usize>,
}

impl JointMarginal {
    fn get_probability(&self, assignment: &[usize]) -> f64 {
        let idx = assignment.iter()
            .zip(&self.strides)
            .map(|(a, s)| a * s)
            .sum::<usize>();
        self.probabilities[idx]
    }
    
    fn marginalize(&self, variables: &[VariableId]) -> JointMarginal {
        // Sum out specified variables
    }
    
    fn condition(&self, evidence: &[(VariableId, usize)]) -> JointMarginal {
        // Condition on evidence
    }
}
```

### Factor Beliefs

```rust
/// Belief for a factor (messages from all incident variables)
struct FactorBelief {
    factor: FactorId,
    
    /// Incoming messages from each variable in scope
    incoming_messages: Vec<Message>,
    
    /// Current factor belief (product of messages)
    belief: Arc<dyn Factor>,
    
    /// Convergence status
    converged: bool,
}

/// Message in belief propagation
struct Message {
    /// Sender
    from: NodeId,
    /// Receiver
    to: NodeId,
    /// Message content (belief over sender's variable)
    content: Belief,
    /// Iteration number
    iteration: usize,
}
```

---

## Evidence Representation

### Hard Evidence

Hard evidence specifies exact observed values.

```rust
/// Hard evidence: variable observed to have specific value
enum HardEvidence {
    /// Discrete observation
    Discrete {
        variable: VariableId,
        value: DiscreteValue,
        value_index: usize,
    },
    
    /// Continuous observation
    Continuous {
        variable: VariableId,
        value: f64,
    },
    
    /// Integer observation
    Integer {
        variable: VariableId,
        value: i64,
    },
}

/// Collection of evidence
struct EvidenceSet {
    evidence: Vec<HardEvidence>,
    
    /// Fast lookup by variable
    by_variable: HashMap<VariableId, usize>,
}
```

### Soft Evidence

Soft evidence specifies a distribution over possible values.

```rust
/// Soft evidence: distribution over variable values
struct SoftEvidence {
    variable: VariableId,
    distribution: Belief,
}

/// Soft evidence for discrete variable
struct DiscreteSoftEvidence {
    variable: VariableId,
    probabilities: Vec<f64>,
}

/// Soft evidence for continuous variable (Gaussian)
struct GaussianSoftEvidence {
    variable: VariableId,
    mean: f64,
    variance: f64,
}
```

### Virtual Evidence

Virtual evidence modifies the observation likelihood without specifying a full distribution.

```rust
/// Virtual evidence: likelihood ratios for each value
struct VirtualEvidence {
    variable: VariableId,
    
    /// Likelihood ratios (unnormalized)
    /// P(evidence | value) for each value
    likelihoods: Vec<f64>,
}

/// Jeffrey's rule for uncertain evidence
struct JeffreyEvidence {
    variable: VariableId,
    
    /// New marginal for the variable
    new_marginal: Belief,
}
```

### Evidence Integration

```rust
/// Integrates evidence into model
trait EvidenceIntegrator {
    /// Incorporate evidence into factor graph
    fn incorporate(&self, graph: &mut FactorGraph, evidence: &EvidenceSet);
    
    /// Create evidence factors
    fn create_evidence_factor(&self, evidence: &HardEvidence) -> Arc<dyn Factor>;
}

struct StandardEvidenceIntegrator;

impl EvidenceIntegrator for StandardEvidenceIntegrator {
    fn incorporate(&self, graph: &mut FactorGraph, evidence: &EvidenceSet) {
        for ev in &evidence.evidence {
            let factor = self.create_evidence_factor(ev);
            graph.add_factor(factor);
        }
    }
    
    fn create_evidence_factor(&self, evidence: &HardEvidence) -> Arc<dyn Factor> {
        match evidence {
            HardEvidence::Discrete { variable, value_index, .. } => {
                // Create indicator factor
                Arc::new(IndicatorFactor {
                    variable: *variable,
                    value: *value_index,
                })
            }
            // ... other variants
        }
    }
}
```

---

## Temporal Representation

### Time-Slice Representation

```rust
/// Time slice: variables and factors at a single time point
struct TimeSlice {
    /// Time index
    time: usize,
    
    /// Variables in this slice
    variables: Vec<VariableId>,
    
    /// Intra-slice factors
    factors: Vec<Arc<dyn Factor>>,
    
    /// Mapping from template to instance
    template_map: HashMap<TemplateId, VariableId>,
}

/// Dynamic Bayesian Network structure
struct DBNStructure {
    /// Initial time slice model (t=0)
    initial_slice: FactorGraph,
    
    /// Two-slice temporal model (t-1 → t)
    transition_model: FactorGraph,
    
    /// Variables that persist across time slices
    interface_variables: Vec<VariableId>,
    
    /// Template variables (instantiated per time slice)
    templates: Vec<VariableTemplate>,
}

struct VariableTemplate {
    id: TemplateId,
    name: String,
    domain: Domain,
    base_variable: VariableId,
}
```

### Transition Models

```rust
/// Transition model defines P(Xᵗ | Xᵗ⁻¹)
struct TransitionModel {
    /// Variables at time t-1 (source)
    from_variables: Vec<VariableId>,
    
    /// Variables at time t (target)
    to_variables: Vec<VariableId>,
    
    /// Transition factors
    factors: Vec<Arc<dyn Factor>>,
    
    /// Markov order (1 for first-order, etc.)
    order: usize,
}

impl TransitionModel {
    /// Get probability of transition
    fn transition_probability(
        &self,
        from_state: &Assignment,
        to_state: &Assignment
    ) -> f64 {
        self.factors.iter()
            .map(|f| f.evaluate(&combine_assignments(from_state, to_state)))
            .product()
    }
}
```

### Rolling Window for Streaming

```rust
/// Rolling window for online inference
struct RollingWindow {
    /// Maximum number of time slices to maintain
    max_slices: usize,
    
    /// Current time index
    current_time: usize,
    
    /// Active slices (circular buffer)
    slices: VecDeque<TimeSlice>,
    
    /// Sufficient statistics for marginalized slices
    statistics: SufficientStatistics,
}

struct SufficientStatistics {
    /// Counts or aggregated information
    /// Type depends on inference algorithm
    data: HashMap<String, Vec<f64>>,
}

impl RollingWindow {
    fn add_slice(&mut self, slice: TimeSlice) {
        if self.slices.len() >= self.max_slices {
            // Marginalize oldest slice and update statistics
            let old = self.slices.pop_front().unwrap();
            self.update_statistics(&old);
        }
        self.slices.push_back(slice);
        self.current_time += 1;
    }
    
    fn get_factor_graph(&self) -> FactorGraph {
        // Unroll active slices into factor graph
        self.unroll_slices()
    }
}
```

---

## Missing Data Representation

### Latent Variable Markers

```rust
/// Marker for latent (unobserved) variables
struct LatentVariable {
    variable: VariableId,
    
    /// Whether the variable is completely unobserved
    /// (as opposed to partially observed in dataset)
    always_latent: bool,
    
    /// Inference method preference
    inference_hint: LatentInferenceHint,
}

enum LatentInferenceHint {
    /// Use exact marginalization if possible
    Exact,
    /// Use sampling-based methods
    Sample,
    /// Use variational approximation
    Variational,
}
```

### Missingness Mechanisms

```rust
/// Representation of missing data mechanisms (Rubin's taxonomy)
enum MissingnessMechanism {
    /// Missing Completely At Random
    MCAR,
    
    /// Missing At Random (depends on observed data)
    MAR {
        /// Variables that explain missingness
        predictors: Vec<VariableId>,
    },
    
    /// Missing Not At Random (depends on missing values themselves)
    MNAR {
        /// Model for missingness mechanism
        mechanism: Arc<dyn Factor>,
    },
}

/// Variable with missing data handling
struct VariableWithMissing {
    variable: VariableId,
    missingness: MissingnessMechanism,
    
    /// Indicator variable for missingness (if explicitly modeled)
    missing_indicator: Option<VariableId>,
}
```

### Data with Missing Values

```rust
/// Dataset with missing value support
struct IncompleteData {
    /// Observed values (None for missing)
    observations: Vec<Vec<Option<Value>>>,
    
    /// Variable IDs for columns
    variables: Vec<VariableId>,
    
    /// Missingness mask
    missing_mask: Vec<Vec<bool>>,
    
    /// Pattern statistics
    missing_patterns: HashMap<Vec<bool>, usize>,
}

impl IncompleteData {
    /// Get complete cases only
    fn complete_cases(&self) -> DataFrame {
        // Filter rows with no missing values
    }
    
    /// Get pattern for each row
    fn missing_patterns(&self) -> Vec<MissingPattern> {
        // Analyze missingness patterns
    }
}
```

---

## Serialization Format

### Internal Binary Format

The binary serialization format is designed for speed and compactness.

```rust
/// Binary format header
struct BinaryHeader {
    /// Magic number: "LUTU" (0x4C555455)
    magic: [u8; 4],
    
    /// Version of serialization format
    version: u16,
    
    /// Flags for compression, endianness, etc.
    flags: u16,
    
    /// Number of sections
    n_sections: u32,
    
    /// CRC32 checksum of header
    checksum: u32,
}

/// Section types
enum SectionType {
    Variables = 0x01,
    Factors = 0x02,
    Edges = 0x03,
    CPDs = 0x04,
    Metadata = 0x05,
    Evidence = 0x06,
    Beliefs = 0x07,
}

/// Section header
struct SectionHeader {
    section_type: SectionType,
    offset: u64,
    length: u64,
    compressed: bool,
    compression_method: u8,
}
```

### Variable Section

```rust
struct VariableSection {
    n_variables: u32,
    variables: Vec<VariableRecord>,
}

struct VariableRecord {
    id: VariableId,
    name_len: u16,
    name: Vec<u8>,  // UTF-8 encoded
    domain_type: u8,
    domain_data: Vec<u8>,  // Domain-specific encoding
}

/// Domain encoding
enum DomainEncoding {
    Discrete { n_values: u32, values: Vec<String> },
    Continuous { has_bounds: bool, lower: f64, upper: f64 },
    Integer { has_bounds: bool, lower: i64, upper: i64 },
}
```

### Factor Section

```rust
struct FactorSection {
    n_factors: u32,
    factors: Vec<FactorRecord>,
}

struct FactorRecord {
    factor_type: u8,
    scope_len: u8,
    scope: Vec<VariableId>,
    data: Vec<u8>,  // Factor-specific data
}

/// Factor type encoding
const FACTOR_TYPE_DENSE: u8 = 0x01;
const FACTOR_TYPE_SPARSE: u8 = 0x02;
const FACTOR_TYPE_FUNCTIONAL: u8 = 0x03;
const FACTOR_TYPE_GAUSSIAN: u8 = 0x04;

/// Dense factor data
struct DenseFactorData {
    shape: Vec<u32>,
    values: Vec<f64>,  // Little-endian double precision
}

/// Sparse factor data
struct SparseFactorData {
    n_entries: u32,
    default_value: f64,
    entries: Vec<(Vec<u32>, f64)>,
}
```

### Schema Evolution

```rust
/// Schema version management
struct SchemaVersion {
    major: u16,
    minor: u16,
}

/// Migration functions between versions
trait SchemaMigration {
    fn from_version(&self) -> SchemaVersion;
    fn to_version(&self) -> SchemaVersion;
    fn migrate(&self, data: &[u8]) -> Result<Vec<u8>, MigrationError>;
}

/// Registry of migrations
struct MigrationRegistry {
    migrations: HashMap<(SchemaVersion, SchemaVersion), Box<dyn SchemaMigration>>,
}

impl MigrationRegistry {
    fn migrate(&self, from: SchemaVersion, to: SchemaVersion, data: &[u8]) 
        -> Result<Vec<u8>, MigrationError> 
    {
        // Find migration path (may require intermediate steps)
        let path = self.find_path(from, to)?;
        let mut result = data.to_vec();
        for migration in path {
            result = migration.migrate(&result)?;
        }
        Ok(result)
    }
}
```

### Backward Compatibility

```rust
/// Backward-compatible deserialization
struct CompatibleDeserializer {
    target_version: SchemaVersion,
}

impl CompatibleDeserializer {
    fn deserialize_factor_graph(&self, bytes: &[u8]) -> Result<FactorGraph, Error> {
        let header = self.read_header(bytes)?;
        
        if header.version != CURRENT_VERSION {
            // Migrate to current version
            let migrated = self.migrate(bytes, header.version, CURRENT_VERSION)?;
            return self.deserialize_current(&migrated);
        }
        
        self.deserialize_current(bytes)
    }
    
    /// Deserialize unknown fields into extension map
    fn deserialize_with_extensions<T: Deserialize>(&self, bytes: &[u8]) 
        -> Result<(T, HashMap<String, serde_json::Value>), Error> 
    {
        // Use serde's UnknownField behavior
    }
}
```

---

## Memory Layout

### Cache-Friendly Structures

**Variable Storage:**
```rust
/// SoA (Structure of Arrays) layout for variables
struct VariableStorage {
    /// Contiguous array of variable IDs
    ids: Vec<VariableId>,
    
    /// Contiguous array of domain pointers
    domains: Vec<Arc<Domain>>,
    
    /// Contiguous array of evidence pointers
    evidences: Vec<Option<Arc<Evidence>>>,
    
    /// Names stored separately (less frequently accessed)
    names: Vec<String>,
}

impl VariableStorage {
    /// Get variable by index (cache-friendly sequential access)
    fn get(&self, idx: usize) -> VariableRef {
        VariableRef {
            id: self.ids[idx],
            domain: &self.domains[idx],
            evidence: &self.evidences[idx],
            name: &self.names[idx],
        }
    }
}
```

**Factor Value Layout:**
```rust
/// Cache-optimized factor storage
struct CacheOptimizedFactor {
    scope: Vec<VariableId>,
    
    /// Values stored in Morton order for 2D/3D factors
    /// Improves locality for common access patterns
    values: Vec<f64>,
    
    /// Shape and strides
    shape: Vec<usize>,
    strides: Vec<usize>,
}
```

### Memory Pools for Small Objects

```rust
/// Pool allocator for small, frequently allocated objects
struct MemoryPool<T> {
    /// Fixed-size chunks
    chunks: Vec<Vec<MaybeUninit<T>>>,
    
    /// Current chunk index
    current_chunk: usize,
    
    /// Next free slot in current chunk
    next_slot: usize,
    
    /// Chunk size
    chunk_size: usize,
    
    /// Free list for reuse
    free_list: Vec<*mut T>,
}

impl<T> MemoryPool<T> {
    fn allocate(&mut self) -> *mut T {
        // Try free list first
        if let Some(ptr) = self.free_list.pop() {
            return ptr;
        }
        
        // Allocate from current chunk
        if self.next_slot >= self.chunk_size {
            self.grow();
        }
        
        let ptr = &mut self.chunks[self.current_chunk][self.next_slot] as *mut _;
        self.next_slot += 1;
        ptr as *mut T
    }
    
    fn deallocate(&mut self, ptr: *mut T) {
        self.free_list.push(ptr);
    }
    
    fn grow(&mut self) {
        self.chunks.push(vec![MaybeUninit::uninit(); self.chunk_size]);
        self.current_chunk += 1;
        self.next_slot = 0;
    }
}
```

### Arena Allocation

```rust
/// Arena for temporary allocations during inference
struct InferenceArena {
    /// Large pre-allocated buffer
    buffer: Vec<u8>,
    
    /// Current position
    offset: usize,
    
    /// Alignment requirement
    alignment: usize,
}

impl InferenceArena {
    fn allocate<T>(&mut self, count: usize) -> &mut [T] {
        let size = std::mem::size_of::<T>() * count;
        let align = std::mem::align_of::<T>();
        
        // Align offset
        self.offset = (self.offset + align - 1) & !(align - 1);
        
        let start = self.offset;
        self.offset += size;
        
        assert!(self.offset <= self.buffer.len(), "Arena overflow");
        
        unsafe {
            std::slice::from_raw_parts_mut(
                self.buffer.as_mut_ptr().add(start) as *mut T,
                count
            )
        }
    }
    
    /// Reset arena (doesn't free memory)
    fn clear(&mut self) {
        self.offset = 0;
    }
}
```

---

## Indexing and Lookup

### Fast Variable Lookup

```rust
/// Variable lookup with multiple indices
struct VariableIndex {
    /// Primary index: ID → Variable
    by_id: Vec<Variable>,
    
    /// Name → ID
    by_name: HashMap<String, VariableId>,
    
    /// Sorted by name for prefix search
    sorted_names: Vec<(String, VariableId)>,
}

impl VariableIndex {
    fn lookup(&self, id: VariableId) -> Option<&Variable> {
        self.by_id.get(id as usize)
    }
    
    fn lookup_by_name(&self, name: &str) -> Option<&Variable> {
        self.by_name.get(name).and_then(|&id| self.lookup(id))
    }
    
    fn search_by_prefix(&self, prefix: &str) -> Vec<&Variable> {
        // Binary search on sorted_names
        let start = self.sorted_names.partition_point(|(n, _)| n < prefix);
        self.sorted_names[start..]
            .iter()
            .take_while(|(n, _)| n.starts_with(prefix))
            .filter_map(|(_, id)| self.lookup(*id))
            .collect()
    }
}
```

### Factor Indexing

```rust
/// Index factors by variable scope
struct FactorIndex {
    /// All factors
    factors: Vec<Arc<dyn Factor>>,
    
    /// Variable → factors containing it
    by_variable: Vec<Vec<FactorId>>,
    
    /// Scope signature → factors
    by_scope_signature: HashMap<ScopeSignature, Vec<FactorId>>,
}

type ScopeSignature = Vec<VariableId>;  // Sorted variable IDs

impl FactorIndex {
    fn factors_containing(&self, variable: VariableId) -> &[FactorId] {
        &self.by_variable[variable as usize]
    }
    
    fn factors_with_scope(&self, scope: &[VariableId]) -> &[FactorId] {
        let mut sorted_scope = scope.to_vec();
        sorted_scope.sort();
        self.by_scope_signature.get(&sorted_scope)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}
```

### Neighbor Access Optimization

```rust
/// Optimized neighbor access for factor graphs
struct NeighborCache {
    /// Pre-computed neighbor lists
    variable_neighbors: Vec<Vec<(FactorId, EdgeId)>>,
    factor_neighbors: Vec<Vec<(VariableId, EdgeId)>>,
    
    /// Hot path: frequently accessed neighbors cached together
    hot_neighbors: Vec<HotNeighborSet>,
}

struct HotNeighborSet {
    /// Center node
    center: NodeId,
    
    /// Neighbors in contiguous memory
    neighbors: SmallVec<[(NodeId, EdgeId); 8]>,
}
```

---

## Immutable vs Mutable Design

### Where Immutability Provides Benefits

**Immutable Factor Graphs:**
- Thread-safe sharing without locks
- Cache-friendly reference counting
- Deterministic behavior
- Easy to snapshot and restore

```rust
/// Immutable factor graph (Arc sharing)
struct ImmutableFactorGraph {
    inner: Arc<FactorGraphData>,
}

impl ImmutableFactorGraph {
    fn clone(&self) -> Self {
        // Just increment reference count
        Self { inner: Arc::clone(&self.inner) }
    }
    
    fn get_variable(&self, id: VariableId) -> &Variable {
        &self.inner.variables[id as usize]
    }
}
```

**Mutable Builder Pattern:**
```rust
/// Mutable builder for constructing factor graphs
struct FactorGraphBuilder {
    variables: Vec<Variable>,
    factors: Vec<Box<dyn Factor>>,
    edges: Vec<Edge>,
}

impl FactorGraphBuilder {
    fn add_variable(&mut self, var: Variable) -> VariableId {
        let id = self.variables.len() as VariableId;
        self.variables.push(var);
        id
    }
    
    fn add_factor(&mut self, factor: Box<dyn Factor>) -> FactorId {
        let id = self.factors.len() as FactorId;
        self.factors.push(factor);
        id
    }
    
    fn build(self) -> ImmutableFactorGraph {
        ImmutableFactorGraph {
            inner: Arc::new(FactorGraphData {
                variables: self.variables,
                factors: self.factors.into_iter().map(|f| Arc::from(f)).collect(),
                edges: self.edges,
                // ... compute derived structures
            })
        }
    }
}
```

### Copy-on-Write Patterns

```rust
/// Copy-on-Write factor graph for efficient modification
struct CowFactorGraph {
    data: Arc<FactorGraphData>,
}

impl CowFactorGraph {
    /// Read-only access doesn't require copy
    fn get(&self, id: VariableId) -> &Variable {
        &self.data.variables[id as usize]
    }
    
    /// Modification triggers copy if shared
    fn modify_variable<F>(&mut self, id: VariableId, f: F)
    where F: FnOnce(&mut Variable) 
    {
        // Check if we're the only reference
        if Arc::get_mut(&mut self.data).is_none() {
            // Need to clone
            self.data = Arc::new(self.data.deep_clone());
        }
        
        // Now we have unique access
        f(&mut Arc::get_mut(&mut self.data).unwrap().variables[id as usize]);
    }
}
```

---

## Integration with External Libraries

### NumPy Array Views

```rust
use numpy::{PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};

/// Zero-copy conversion to NumPy arrays
impl DenseFactor {
    fn to_numpy<'py>(&self, py: Python<'py>) -> &'py PyArray2<f64> {
        // Create view without copying
        unsafe {
            PyArray2::from_raw_pointer(
                py,
                self.shape.clone(),
                self.values.as_ptr() as *mut f64,
            )
        }
    }
    
    fn from_numpy(array: &PyReadonlyArray2<f64>) -> Self {
        let view = array.as_array();
        Self {
            shape: view.shape().to_vec(),
            values: view.iter().cloned().collect(),
            strides: compute_strides(&view.shape()),
        }
    }
}
```

### scipy.sparse Compatibility

```rust
use sprs::{CsMat, CsVec};

/// Convert to scipy.sparse format
impl SparseFactor {
    fn to_scipy_sparse(&self) -> scipy_sparse::CsrMatrix {
        // Convert to CSR format compatible with scipy
        let (data, indices, indptr) = self.to_csr_parts();
        scipy_sparse::CsrMatrix::new(data, indices, indptr, self.shape.clone())
    }
    
    fn from_scipy_sparse(matrix: &scipy_sparse::CsrMatrix) -> Self {
        Self::from_csr_parts(
            matrix.data().to_vec(),
            matrix.indices().to_vec(),
            matrix.indptr().to_vec(),
            matrix.shape().to_vec(),
        )
    }
}
```

### NetworkX Graph Conversion

```rust
use petgraph::graph::DiGraph;

/// Convert to NetworkX-compatible format
impl BayesianNetwork {
    fn to_networkx(&self) -> PyObject {
        Python::with_gil(|py| {
            let nx = py.import("networkx")?;
            let graph = nx.call_method0("DiGraph")?;
            
            // Add nodes with attributes
            for var in &self.variables {
                graph.call_method1(
                    "add_node",
                    (&var.name, var.domain.to_python_dict(py))
                )?;
            }
            
            // Add edges
            for edge in &self.edges {
                graph.call_method1("add_edge", (&edge.0, &edge.1))?;
            }
            
            Ok(graph.into())
        })
    }
    
    fn from_networkx(graph: &PyObject) -> Result<Self, ConversionError> {
        Python::with_gil(|py| {
            // Extract nodes and edges from NetworkX graph
            let nodes: Vec<String> = graph.call_method0(py, "nodes")?.extract(py)?;
            let edges: Vec<(String, String)> = graph.call_method0(py, "edges")?.extract(py)?;
            
            // Build BayesianNetwork
            let mut model = BayesianNetwork::new();
            for node in nodes {
                model.add_node(&node);
            }
            for (u, v) in edges {
                model.add_edge(&u, &v)?;
            }
            
            Ok(model)
        })
    }
}
```

---

## Thread Safety

### Thread-Safe Structures

**Immutable Structures (Always Thread-Safe):**
```rust
/// Immutable factor graph is Send + Sync
unsafe impl Send for ImmutableFactorGraph {}
unsafe impl Sync for ImmutableFactorGraph {}

/// Arc<dyn Factor> is thread-safe
impl Factor for MyFactor {
    fn evaluate(&self, assignment: &Assignment) -> f64 {
        // Thread-safe implementation
    }
}
```

**Mutable Structures with Synchronization:**
```rust
/// Thread-safe model cache
struct ThreadSafeModelCache {
    cache: RwLock<HashMap<String, Arc<FactorGraph>>>,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl ThreadSafeModelCache {
    fn get(&self, key: &str) -> Option<Arc<FactorGraph>> {
        let cache = self.cache.read().unwrap();
        if let Some(graph) = cache.get(key) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            Some(Arc::clone(graph))
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }
    
    fn insert(&self, key: String, graph: Arc<FactorGraph>) {
        let mut cache = self.cache.write().unwrap();
        cache.insert(key, graph);
    }
}
```

### Locking Strategies

**Read-Write Locks:**
```rust
/// Factor graph with read-write lock for shared modification
struct RwFactorGraph {
    data: RwLock<FactorGraphData>,
}

impl RwFactorGraph {
    fn query(&self, variables: &[VariableId]) -> QueryResult {
        // Multiple readers can hold read lock simultaneously
        let data = self.data.read().unwrap();
        self.infer(&data, variables)
    }
    
    fn update_evidence(&self, evidence: &EvidenceSet) {
        // Write lock excludes all other access
        let mut data = self.data.write().unwrap();
        data.apply_evidence(evidence);
    }
}
```

**Lock-Free Structures:**
```rust
use crossbeam::atomic::AtomicCell;

/// Lock-free message storage for belief propagation
struct LockFreeMessageStore {
    messages: Vec<AtomicCell<Option<Message>>>,
}

impl LockFreeMessageStore {
    fn get_message(&self, edge: EdgeId) -> Option<Message> {
        self.messages[edge as usize].load()
    }
    
    fn set_message(&self, edge: EdgeId, message: Message) {
        self.messages[edge as usize].store(Some(message));
    }
}
```

---

## Key References

### Data Structure Design Patterns

1. **Sedgewick, R., & Wayne, K. (2011).** *Algorithms* (4th ed.). Addison-Wesley.
   - Fundamental graph data structures
   - Search and indexing algorithms

2. **Cormen, T. H., et al. (2009).** *Introduction to Algorithms* (3rd ed.). MIT Press.
   - Advanced data structures
   - Memory-efficient algorithms

3. **Brodal, G. S. (1996).** "Worst-Case Efficient Data Structures." *BRICS Dissertation Series.*
   - Cache-oblivious data structures

### Sparse Matrix Formats

4. **Saad, Y. (2003).** *Iterative Methods for Sparse Linear Systems* (2nd ed.). SIAM.
   - CSR, CSC, COO formats
   - Sparse matrix algorithms

5. **Davis, T. A. (2006).** *Direct Methods for Sparse Linear Systems*. SIAM.
   - Sparse matrix data structures
   - Graph representations of matrices

6. **Buluc, A., & Gilbert, J. R. (2011).** "The Combinatorial BLAS: Design, Implementation, and Applications." *International Journal of High Performance Computing Applications.*
   - Distributed sparse matrix formats

### Memory Management

7. **Wilson, P. R., et al. (1995).** "Dynamic Storage Allocation: A Survey and Critical Review." *IWMM 1995.*
   - Memory allocator design
   - Pool and arena allocation

8. **Berger, E. D., et al. (2000).** "Hoard: A Scalable Memory Allocator for Multithreaded Applications." *ASPLOS 2000.*
   - Thread-safe memory allocation

### Serialization

9. **Google Protocol Buffers.** https://developers.google.com/protocol-buffers
   - Binary serialization format
   - Schema evolution patterns

10. **Apache Arrow.** https://arrow.apache.org/
    - Columnar memory format
    - Zero-copy data sharing

### Probabilistic Model Representations

11. **Kschischang, F. R., et al. (2001).** "Factor Graphs and the Sum-Product Algorithm." *IEEE Transactions on Information Theory.*
    - Factor graph representation theory

12. **Bilmes, J., & Dechter, R. (2006).** "Evaluation of Probabilistic Inference Systems." *UAI 2006.*
    - Model representation trade-offs

13. **Mooij, J. M. (2010).** libDAI documentation. https://github.com/dbaml/libDAI
    - Practical factor graph implementation

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial draft |
| 1.0 | 2026-03-03 | Wasswa Lutufi Sebbanja | Complete data model document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
