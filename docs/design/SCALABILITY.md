# Lutufi Scalability Design Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Scalability Goals](#scalability-goals)
3. [The Treewidth Problem](#the-treewidth-problem)
4. [Sparse Representations](#sparse-representations)
5. [Memory Management](#memory-management)
6. [Lazy Evaluation](#lazy-evaluation)
7. [Chunked Processing](#chunked-processing)
8. [Approximate Methods for Scale](#approximate-methods-for-scale)
9. [Stochastic Variational Inference](#stochastic-variational-inference)
10. [Distributed Computation](#distributed-computation)
11. [Graph Partitioning for Scale](#graph-partitioning-for-scale)
12. [Progressive Loading](#progressive-loading)
13. [Compression](#compression)
14. [Scalability Tiers](#scalability-tiers)
15. [Profiling and Monitoring](#profiling-and-monitoring)
16. [How Lutufi Scales](#how-lutufi-scales)
17. [Key References](#key-references)

---

## Executive Summary

Scalability is a fundamental design concern for Lutufi. Probabilistic graphical models face inherent computational barriers—exact inference is NP-hard in general, and approximate methods struggle with convergence as networks grow. This document presents a comprehensive scalability strategy that enables Lutufi to handle networks ranging from small pedagogical examples to massive real-world systems with millions of nodes.

The scalability approach is multi-pronged:

- **Algorithmic**: Automatic selection between exact and approximate methods based on problem characteristics
- **Structural**: Sparse representations that exploit the natural sparsity in real-world networks
- **Memory**: Sophisticated memory management including pools, arenas, and memory-mapped files
- **Computational**: Parallel and distributed processing capabilities
- **Progressive**: Out-of-core processing for networks exceeding available RAM

This document defines scalability targets, explains the fundamental barriers (particularly treewidth), and details the architectural decisions that enable Lutufi to scale gracefully across problem sizes.

---

## Scalability Goals

### Target Scale Definitions

Lutufi defines three primary scalability tiers based on network size and computational requirements:

| Tier | Nodes | Edges | Typical Use Cases | Target Response Time |
|------|-------|-------|-------------------|---------------------|
| **Small** | < 1,000 | < 5,000 | Education, prototyping, unit tests, small research models | < 100 ms |
| **Medium** | 1,000 - 100,000 | 5,000 - 500,000 | Research models, organizational networks, epidemiological models, supply chains | < 5 seconds |
| **Large** | 100,000 - 10,000,000 | 500,000 - 50,000,000 | Social networks, financial systems, large biological networks, IoT sensor networks | < 5 minutes |
| **Massive** | > 10,000,000 | > 50,000,000 | Web-scale graphs, national infrastructure, global supply chains | Minutes to hours |

### Use Cases at Each Scale

**Small Scale (< 1,000 nodes):**
- **Medical diagnosis models**: Disease-symptom relationships with dozens of variables
- **Academic examples**: Alarm network, Asia network, Student network for teaching
- **Rapid prototyping**: Iterative model development and testing
- **Embedded systems**: Resource-constrained environments

**Medium Scale (1,000 - 100,000 nodes):**
- **Organizational network analysis**: Company structures, communication patterns
- **Epidemiological models**: City or regional disease spread with individual-level detail
- **Supply chain optimization**: Multi-tier supplier networks with demand uncertainty
- **Financial portfolio risk**: Correlated asset models with sector and geographic factors
- **Scientific research**: Gene regulatory networks, ecological food webs

**Large Scale (100,000 - 10,000,000 nodes):**
- **Social network influence**: Viral marketing, opinion dynamics on platforms
- **Financial contagion**: Interbank lending networks, systemic risk analysis
- **Misinformation spread**: Information cascade modeling on social media
- **Smart city models**: Traffic, energy, and infrastructure interdependencies
- **Cybersecurity**: Attack graphs, vulnerability dependency networks

**Massive Scale (> 10,000,000 nodes):**
- **Web graph analysis**: PageRank variants, link prediction
- **National infrastructure**: Power grid with component-level detail
- **Global supply chains**: Multi-national, multi-tier production networks
- **Population health**: Individual-based epidemic models for entire countries

### Scalability Dimensions

Scalability is not just about node count. Lutufi addresses multiple dimensions:

**1. Structural Scalability**
- Number of nodes (variables)
- Number of edges (dependencies)
- Maximum degree (connectivity)
- Treewidth (computational complexity)

**2. Parameter Scalability**
- CPD table sizes (state space explosion)
- Number of parameters in continuous distributions
- Precision requirements

**3. Data Scalability**
- Learning from datasets with millions of records
- Streaming data for online learning
- High-dimensional data (many features per observation)

**4. Query Scalability**
- Number of simultaneous queries
- Complexity of queries (marginal vs. MAP vs. interventional)
- Number of evidence variables

**5. Temporal Scalability**
- Number of time slices in dynamic models
- Length of observation sequences
- Prediction horizons

### Performance Targets

For each tier, Lutufi targets specific performance characteristics:

**Inference Performance:**
```
Small:    Exact inference on any query < 100 ms
Medium:   Exact inference for low-treewidth queries < 1 s
          Approximate inference for high-treewidth < 5 s
Large:    Approximate inference with convergence guarantees < 5 min
          Streaming inference for dynamic updates < 1 s per update
Massive:  Distributed approximate inference with quality monitoring
```

**Learning Performance:**
```
Small:    Structure + parameter learning < 1 second
Medium:   Structure learning < 10 minutes
          Parameter learning < 1 minute
Large:    Mini-batch structure learning < 1 hour
          Distributed parameter learning < 10 minutes
Massive:  Sampling-based structure learning with quality bounds
```

**Memory Efficiency:**
```
Target:   < 100 bytes per node (average) for structure
          < 1 KB per edge (average) for CPDs
          Overhead factor < 2x for sparse representations
```

---

## The Treewidth Problem

### Why Treewidth Matters

The treewidth of a graphical model is the single most important determinant of inference complexity. Exact inference algorithms like variable elimination and junction tree have time and space complexity exponential in the treewidth.

**Formal Definition:**
The treewidth of a graph G is one less than the size of the largest clique in the optimal triangulation of G. Intuitively, it measures how "tree-like" the graph is:
- Trees have treewidth 1
- Series-parallel graphs have treewidth ≤ 2
- n × n grids have treewidth n
- Complete graphs on n nodes have treewidth n-1

**Complexity Implications:**
- Variable elimination: O(n · d^(tw+1)) time, where d is maximum domain size
- Junction tree: O(n · d^tw) time and space
- Belief propagation on junction tree: O(n · d^tw) per iteration

### Treewidth in Practice

**Real-World Treewidth Distributions:**

| Network Type | Typical Treewidth | Maximum Tractable |
|--------------|-------------------|-------------------|
| Trees and forests | 1 | Exact always feasible |
| Sparse random graphs (Erdős-Rényi, p=0.01) | 3-5 | Exact typically feasible |
| Grid graphs (2D) | O(√n) | Exact feasible for n < 100 |
| Small-world networks | 10-30 | Approximate required for n > 1000 |
| Scale-free networks | Logarithmic in n | Exact feasible for hubs only |
| Dense graphs | O(n) | Approximate always required |
| Complete graphs | n-1 | Only MAP feasible |

**The Scalability Barrier:**
For a network with binary variables and treewidth 20:
- Junction tree requires storing tables of size 2^20 ≈ 1 million entries
- For treewidth 30: 2^30 ≈ 1 billion entries
- For treewidth 40: 2^40 ≈ 1 trillion entries (infeasible)

This exponential growth means that even modest increases in treewidth render exact inference impossible.

### Treewidth Estimation

Since computing exact treewidth is NP-hard, Lutufi uses approximation algorithms:

**Upper Bound Estimation (Min-Fill Heuristic):**
```rust
fn estimate_treewidth_upper_bound(graph: &Graph) -> usize {
    let mut max_clique_size = 0;
    let mut graph = graph.clone();
    
    while graph.nodes().count() > 0 {
        // Find node that creates fewest fill edges when eliminated
        let best_node = graph.nodes()
            .min_by_key(|n| count_fill_edges(&graph, n))
            .unwrap();
        
        // Count neighbors (clique size when eliminated)
        let degree = graph.neighbors(best_node).count();
        max_clique_size = max_clique_size.max(degree + 1);
        
        // Add fill edges and remove node
        add_fill_edges(&mut graph, best_node);
        graph.remove_node(best_node);
    }
    
    max_clique_size - 1  // Treewidth is max clique minus 1
}
```

**Lower Bound Estimation (Maximum Minimum Degree):**
```rust
fn estimate_treewidth_lower_bound(graph: &Graph) -> usize {
    let mut graph = graph.clone();
    let mut lower_bound = 0;
    
    while graph.nodes().count() > 0 {
        let min_degree = graph.nodes()
            .map(|n| graph.neighbors(n).count())
            .min()
            .unwrap();
        
        lower_bound = lower_bound.max(min_degree);
        
        // Remove minimum degree node
        let min_node = graph.nodes()
            .find(|n| graph.neighbors(n).count() == min_degree)
            .unwrap();
        graph.remove_node(min_node);
    }
    
    lower_bound
}
```

**Practical Estimation:**
```rust
struct TreewidthEstimate {
    lower_bound: usize,
    upper_bound: usize,
    heuristic_estimate: usize,
    confidence: f64,  // Based on gap between bounds
}

impl TreewidthEstimate {
    fn is_exact(&self) -> bool {
        self.lower_bound == self.upper_bound
    }
    
    fn quality(&self) -> TreewidthQuality {
        let gap = self.upper_bound - self.lower_bound;
        if gap == 0 {
            TreewidthQuality::Exact
        } else if gap <= 2 {
            TreewidthQuality::Tight
        } else if gap <= 5 {
            TreewidthQuality::Moderate
        } else {
            TreewidthQuality::Loose
        }
    }
}
```

### Adaptive Algorithm Selection

Lutufi uses treewidth estimates to automatically select appropriate inference algorithms:

```rust
fn select_inference_algorithm(
    model: &FactorGraph,
    query: &Query,
    constraints: &InferenceConstraints
) -> InferenceAlgorithm {
    let tw_estimate = estimate_treewidth(model);
    let max_domain_size = model.max_variable_cardinality();
    
    // Calculate effective complexity
    let effective_complexity = tw_estimate.heuristic_estimate 
        * (max_domain_size as f64).log2() as usize;
    
    match effective_complexity {
        0..=15 => {
            // Low treewidth: exact inference is efficient
            InferenceAlgorithm::JunctionTree
        }
        16..=25 => {
            // Medium treewidth: try bounded exact, fallback to approximate
            if constraints.require_exact {
                InferenceAlgorithm::BoundedJunctionTree {
                    max_clique_size: 2usize.pow(20)
                }
            } else {
                InferenceAlgorithm::GeneralizedBeliefPropagation
            }
        }
        26..=40 => {
            // High treewidth: approximate methods required
            if model.is_tree() || model.is_singly_connected() {
                InferenceAlgorithm::BeliefPropagation
            } else {
                InferenceAlgorithm::LoopyBeliefPropagation {
                    damping: 0.5,
                    convergence_threshold: 1e-6
                }
            }
        }
        _ => {
            // Very high treewidth: sampling or variational methods
            if query.requires_marginals() {
                InferenceAlgorithm::GibbsSampling {
                    n_samples: 10000,
                    burn_in: 1000
                }
            } else {
                InferenceAlgorithm::StochasticVariationalInference
            }
        }
    }
}
```

### Handling High-Treewidth Models

When treewidth exceeds practical limits:

**1. Model Simplification:**
```rust
fn simplify_for_inference(model: &FactorGraph, target_tw: usize) -> FactorGraph {
    let current_tw = estimate_treewidth(model);
    
    if current_tw.upper_bound <= target_tw {
        return model.clone();
    }
    
    // Strategy 1: Remove weak edges
    let mut simplified = model.clone();
    let edges_by_strength = model.edges_sorted_by_mutual_information();
    
    for edge in edges_by_strength {
        if estimate_treewidth(&simplified).upper_bound <= target_tw {
            break;
        }
        simplified.remove_edge(edge);
    }
    
    // Strategy 2: Merge similar nodes
    if estimate_treewidth(&simplified).upper_bound > target_tw {
        simplified = merge_similar_nodes(simplified, target_tw);
    }
    
    simplified
}
```

**2. Query-Specific Decomposition:**
```rust
fn decompose_query(
    model: &FactorGraph,
    query: &Query
) -> Vec<SubQuery> {
    // Find query-relevant subgraph
    let relevant_nodes = model.markov_blanket(&query.variables);
    
    // Check if relevant subgraph has tractable treewidth
    let subgraph = model.induced_subgraph(&relevant_nodes);
    let tw = estimate_treewidth(&subgraph);
    
    if tw.upper_bound <= MAX_TRACTABLE_TREEWIDTH {
        vec![SubQuery::new(subgraph, query.clone())]
    } else {
        // Decompose into conditionally independent components
        decompose_by_dseparation(model, query)
    }
}
```

---

## Sparse Representations

### Exploiting Structural Sparsity

Real-world networks are rarely dense. Most networks have:
- Average degree << n (often constant or logarithmic)
- Power-law degree distributions (few hubs, many leaves)
- Community structure (dense internal, sparse external)
- Local tree-like structure (few short cycles)

Lutufi exploits this sparsity through specialized data structures.

### CSR/CSC for Adjacency

**Compressed Sparse Row (CSR)** for fast successor access:
```rust
struct CsrAdjacency {
    n_nodes: usize,
    n_edges: usize,
    
    // Row pointers: indices[ptr[i]..ptr[i+1]] are successors of i
    indptr: Vec<usize>,
    
    // Column indices (successor nodes)
    indices: Vec<usize>,
    
    // Optional edge data
    data: Vec<EdgeMetadata>,
}

impl CsrAdjacency {
    fn successors(&self, node: usize) -> &[usize] {
        let start = self.indptr[node];
        let end = self.indptr[node + 1];
        &self.indices[start..end]
    }
    
    fn memory_bytes(&self) -> usize {
        // O(n + m) instead of O(n²)
        self.indptr.len() * size_of::<usize>() +
        self.indices.len() * size_of::<usize>()
    }
}
```

**Compressed Sparse Column (CSC)** for fast predecessor access:
```rust
struct CscAdjacency {
    n_nodes: usize,
    n_edges: usize,
    indptr: Vec<usize>,
    indices: Vec<usize>,
}

impl CscAdjacency {
    fn predecessors(&self, node: usize) -> &[usize] {
        let start = self.indptr[node];
        let end = self.indptr[node + 1];
        &self.indices[start..end]
    }
}
```

**Bidirectional Access:**
```rust
struct BidirectionalAdjacency {
    csr: CsrAdjacency,  // For successors
    csc: CscAdjacency,  // For predecessors
}
```

Memory savings for sparse graphs:
- Dense adjacency: n² bits = 125 MB for n = 100,000
- CSR/CSC: (n + m) × 4 bytes ≈ 2 MB for n = 100,000, m = 500,000
- **Savings: 60x for typical sparse graphs**

### Sparse Tensors for CPTs

Conditional probability tables are often sparse:
- Many zero probabilities (structural zeros)
- Near-deterministic relationships (most probability on one outcome)
- Context-specific independence (irrelevant parents)

**Sparse CPD Representation:**
```rust
enum CPDStorage {
    // Dense for small or non-sparse CPDs
    Dense {
        values: Vec<f64>,
        shape: Vec<usize>,
        strides: Vec<usize>,
    },
    
    // Coordinate format for moderate sparsity
    SparseCOO {
        entries: Vec<(Vec<usize>, f64)>,
        default_value: f64,
        shape: Vec<usize>,
    },
    
    // Compressed format for high sparsity
    SparseCSR3D {
        // For CPDs: P(child | parent1, parent2, ...)
        values: Vec<f64>,
        indices: Vec<usize>,  // Compressed indices
        indptr: Vec<usize>,   // Row pointers
        parent_configs: Vec<usize>,
    },
    
    // Function-based for complex distributions
    Functional {
        eval: Box<dyn Fn(&[usize]) -> f64>,
        cache: LruCache<Vec<usize>, f64>,
    },
}
```

**Sparsity Detection and Conversion:**
```rust
fn auto_select_cpd_storage(values: &[f64], shape: &[usize]) -> CPDStorage {
    let n_elements: usize = shape.iter().product();
    let n_nonzero = values.iter().filter(|&&v| v > 1e-10).count();
    let sparsity = 1.0 - (n_nonzero as f64 / n_elements as f64);
    
    if sparsity < 0.3 {
        // Less than 30% sparse: use dense
        CPDStorage::Dense {
            values: values.to_vec(),
            shape: shape.to_vec(),
            strides: compute_strides(shape),
        }
    } else if sparsity < 0.9 {
        // 30-90% sparse: use COO
        let entries: Vec<_> = values.iter().enumerate()
            .filter(|(_, &v)| v > 1e-10)
            .map(|(idx, &v)| (unflatten_index(idx, shape), v))
            .collect();
        
        CPDStorage::SparseCOO {
            entries,
            default_value: 0.0,
            shape: shape.to_vec(),
        }
    } else {
        // >90% sparse: use compressed
        to_compressed_storage(values, shape)
    }
}
```

### Hash Maps for Irregular Structures

For highly irregular graphs (e.g., scale-free networks with hub nodes):

```rust
struct HashMapAdjacency {
    n_nodes: usize,
    // Hash map for each node's neighbors
    neighbors: Vec<HashSet<usize>>,
    // For hub nodes: separate dense storage
    hub_threshold: usize,
    hub_neighbors: Vec<Vec<usize>>,
}

impl HashMapAdjacency {
    fn new(edges: &[(usize, usize)], n_nodes: usize) -> Self {
        let mut neighbors: Vec<HashSet<usize>> = vec![HashSet::new(); n_nodes];
        
        for &(u, v) in edges {
            neighbors[u].insert(v);
        }
        
        // Identify hub nodes
        let hub_threshold = (n_nodes as f64).sqrt() as usize;
        let mut hub_neighbors = Vec::new();
        
        for (i, neigh) in neighbors.iter().enumerate() {
            if neigh.len() > hub_threshold {
                hub_neighbors.push((i, neigh.iter().cloned().collect::<Vec<_>>()));
            }
        }
        
        Self {
            n_nodes,
            neighbors,
            hub_threshold,
            hub_neighbors,
        }
    }
    
    fn neighbors(&self, node: usize) -> NeighborIter {
        if self.neighbors[node].len() > self.hub_threshold {
            NeighborIter::Dense(&self.hub_neighbors[node])
        } else {
            NeighborIter::Hash(&self.neighbors[node])
        }
    }
}
```

### Memory Savings Calculations

**Example: Scale-Free Network with 100,000 nodes**

| Structure | Dense | Sparse | Savings |
|-----------|-------|--------|---------|
| Adjacency | 100,000² × 1 bit = 1.25 GB | CSR: 5 MB | 250x |
| Average CPD (10 parents, binary) | 2^11 × 8 bytes = 16 KB | Sparse: 2 KB | 8x |
| Total model | ~100 GB | ~2 GB | 50x |

**For 1,000,000 node scale-free network:**
- Dense: Impossible (would require 125 TB)
- Sparse CSR: ~50 MB (adjacency) + ~500 MB (CPDs) = ~550 MB
- **Enables billion-node networks on standard hardware**

---

## Memory Management

### Memory Pools for Small Objects

Frequent allocation of small objects (factors, messages, beliefs) causes fragmentation and GC overhead. Lutufi uses typed memory pools:

```rust
struct TypedMemoryPool<T> {
    chunk_size: usize,
    chunks: Vec<Vec<MaybeUninit<T>>>,
    free_list: Vec<*mut T>,
}

impl<T> TypedMemoryPool<T> {
    fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            chunks: Vec::new(),
            free_list: Vec::new(),
        }
    }
    
    fn allocate(&mut self) -> *mut T {
        // Return from free list if available
        if let Some(ptr) = self.free_list.pop() {
            return ptr;
        }
        
        // Allocate new chunk if needed
        if self.chunks.is_empty() || self.is_current_chunk_full() {
            self.grow();
        }
        
        // Allocate from current chunk
        let chunk_idx = self.chunks.len() - 1;
        let slot_idx = self.next_free_slot();
        let ptr = &mut self.chunks[chunk_idx][slot_idx] as *mut _ as *mut T;
        ptr
    }
    
    fn deallocate(&mut self, ptr: *mut T) {
        // Safety: caller must ensure ptr was allocated from this pool
        unsafe { ptr.drop_in_place(); }
        self.free_list.push(ptr);
    }
    
    fn grow(&mut self) {
        self.chunks.push(vec![MaybeUninit::uninit(); self.chunk_size]);
    }
}
```

**Pool usage during belief propagation:**
```rust
struct BeliefPropagationContext {
    message_pool: TypedMemoryPool<Message>,
    belief_pool: TypedMemoryPool<Belief>,
    factor_pool: TypedMemoryPool<Box<dyn Factor>>,
}

impl BeliefPropagationContext {
    fn new_message(&mut self, content: Belief) -> *mut Message {
        let msg = self.message_pool.allocate();
        unsafe {
            msg.write(Message::new(content));
        }
        msg
    }
    
    fn return_message(&mut self, msg: *mut Message) {
        self.message_pool.deallocate(msg);
    }
}
```

### Arena Allocation

For inference operations with known lifetime, arena allocation provides O(1) bulk deallocation:

```rust
struct InferenceArena {
    buffer: Vec<u8>,
    offset: usize,
    alignment: usize,
    checkpoints: Vec<usize>,
}

impl InferenceArena {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            offset: 0,
            alignment: 64,  // Cache line alignment
            checkpoints: Vec::new(),
        }
    }
    
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
    
    fn checkpoint(&mut self) {
        self.checkpoints.push(self.offset);
    }
    
    fn rollback(&mut self) {
        if let Some(cp) = self.checkpoints.pop() {
            self.offset = cp;
        }
    }
    
    fn reset(&mut self) {
        self.offset = 0;
        self.checkpoints.clear();
    }
}

// Usage during inference
fn run_inference_with_arena(model: &FactorGraph, query: &Query) -> QueryResult {
    let mut arena = InferenceArena::with_capacity(100 * 1024 * 1024);  // 100 MB
    
    arena.checkpoint();
    
    // Allocate temporary structures in arena
    let messages = arena.allocate::<Message>(model.n_edges());
    let beliefs = arena.allocate::<Belief>(model.n_variables());
    
    // Run inference...
    let result = belief_propagation(model, messages, beliefs);
    
    // Reset arena (O(1) - just reset offset)
    arena.reset();
    
    result
}
```

### Memory-Mapped Files

For out-of-core processing of large networks:

```rust
use memmap2::{MmapMut, MmapOptions};

struct MemoryMappedFactor {
    mmap: MmapMut,
    shape: Vec<usize>,
    dtype: DataType,
    cache: LruCache<Vec<usize>, f64>,
}

impl MemoryMappedFactor {
    fn new(file_path: &Path, shape: Vec<usize>) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
        
        let size: usize = shape.iter().product::<usize>() * size_of::<f64>();
        file.set_len(size as u64)?;
        
        let mmap = unsafe { MmapOptions::new().map_mut(&file)? };
        
        Ok(Self {
            mmap,
            shape,
            dtype: DataType::Float64,
            cache: LruCache::new(1024),
        })
    }
    
    fn get(&self, indices: &[usize]) -> f64 {
        // Check cache first
        if let Some(&value) = self.cache.get(indices) {
            return value;
        }
        
        // Compute offset and read from mmap
        let offset = self.compute_offset(indices);
        let bytes = &self.mmap[offset..offset + size_of::<f64>()];
        let value = f64::from_le_bytes(bytes.try_into().unwrap());
        
        // Cache and return
        self.cache.put(indices.to_vec(), value);
        value
    }
    
    fn set(&mut self, indices: &[usize], value: f64) {
        let offset = self.compute_offset(indices);
        let bytes = value.to_le_bytes();
        self.mmap[offset..offset + size_of::<f64>()].copy_from_slice(&bytes);
        self.cache.put(indices.to_vec(), value);
    }
}
```

**Paged Factor Graph for Large Models:**
```rust
struct PagedFactorGraph {
    // In-memory portions
    hot_factors: Vec<Box<dyn Factor>>,
    
    // On-disk portions
    cold_factors: Vec<MemoryMappedFactor>,
    
    // Paging policy
    access_tracker: AccessTracker,
}

impl PagedFactorGraph {
    fn get_factor(&mut self, id: FactorId) -> &dyn Factor {
        if let Some(idx) = self.hot_factors.iter().position(|f| f.id() == id) {
            self.access_tracker.record_access(id);
            &*self.hot_factors[idx]
        } else {
            // Load from disk
            self.page_in_factor(id);
            self.get_factor(id)
        }
    }
    
    fn page_in_factor(&mut self, id: FactorId) {
        // Find coldest factor in memory
        if self.hot_factors.len() >= MAX_HOT_FACTORS {
            let coldest = self.find_coldest_factor();
            self.page_out_factor(coldest);
        }
        
        // Load factor from mmap
        let factor = self.cold_factors[id.0 as usize].load();
        self.hot_factors.push(factor);
    }
}
```

### Garbage Collection Strategies

For Rust-based implementation, Lutufi minimizes garbage collection pressure:

**Reference Counting with Cycle Detection:**
```rust
struct FactorGraph {
    // Use Arc for shared ownership
    factors: Vec<Arc<dyn Factor>>,
    
    // Weak references to avoid cycles
    variable_to_factors: Vec<Vec<Weak<dyn Factor>>>,
}
```

**Generational Cleanup for Iterative Algorithms:**
```rust
struct GenerationalAllocator {
    young_generation: TypedMemoryPool<Message>,
    old_generation: TypedMemoryPool<Message>,
    promotion_threshold: usize,
}

impl GenerationalAllocator {
    fn allocate_message(&mut self) -> *mut Message {
        self.young_generation.allocate()
    }
    
    fn minor_gc(&mut self) {
        // Free unreachable young objects
        self.young_generation.sweep();
    }
    
    fn major_gc(&mut self) {
        // Promote survivors to old generation
        // Full sweep
    }
}
```

---

## Lazy Evaluation

### Deferred Computation Philosophy

Lazy evaluation defers expensive operations until their results are actually needed. This is crucial for scalability because:

1. **Query-specific optimization**: Only compute what's needed for the query
2. **Opportunity for cancellation**: Avoid work if intermediate results make computation unnecessary
3. **Memory efficiency**: Don't store intermediate results that won't be used
4. **Batching**: Collect multiple operations and execute together

### Promise-Based Results

```rust
enum Computation<T> {
    // Value is already computed
    Ready(T),
    
    // Computation deferred
    Pending {
        // Function to compute the value
        computation: Box<dyn FnOnce() -> T>,
        
        // Estimated cost
        estimated_cost: ComputationalCost,
        
        // Dependencies
        dependencies: Vec<ComputationId>,
    },
    
    // Computation in progress
    InProgress {
        handle: JoinHandle<T>,
        progress: Arc<AtomicUsize>,
    },
}

struct LazyFactor {
    scope: Vec<VariableId>,
    computation: Computation<Box<dyn Factor>>,
}

impl Factor for LazyFactor {
    fn evaluate(&self, assignment: &Assignment) -> f64 {
        match &self.computation {
            Computation::Ready(factor) => factor.evaluate(assignment),
            Computation::Pending { computation, .. } => {
                // Force evaluation
                let factor = computation();
                factor.evaluate(assignment)
            }
            Computation::InProgress { .. } => {
                // Wait for completion
                self.wait_and_evaluate(assignment)
            }
        }
    }
    
    fn marginalize(&self, vars: &[VariableId]) -> Arc<dyn Factor> {
        // Return lazy factor that defers actual marginalization
        Arc::new(LazyFactor {
            scope: self.scope.iter()
                .filter(|v| !vars.contains(v))
                .cloned()
                .collect(),
            computation: Computation::Pending {
                computation: Box::new(move || {
                    let inner = self.force();
                    inner.marginalize(vars)
                }),
                estimated_cost: self.estimate_marginalization_cost(vars),
                dependencies: vec![self.id()],
            },
        })
    }
}
```

### When to Compute vs Cache

```rust
struct ComputationPolicy {
    // Threshold for pre-computation
    precompute_threshold: ComputationalCost,
    
    // Cache size limits
    max_cached_factors: usize,
    max_cached_beliefs: usize,
    
    // Memory pressure threshold
    memory_pressure_threshold: f64,
}

impl ComputationPolicy {
    fn should_precompute(&self, cost: &ComputationalCost, usage_probability: f64) -> bool {
        // Precompute if:
        // 1. Cost is below threshold AND
        // 2. High probability of use AND
        // 3. Memory not under pressure
        
        cost < self.precompute_threshold 
            && usage_probability > 0.7
            && !self.is_memory_under_pressure()
    }
    
    fn should_cache(&self, factor: &dyn Factor, access_frequency: f64) -> bool {
        // Cache if access frequency justifies memory cost
        let memory_cost = factor.memory_bytes() as f64;
        let compute_cost = self.estimate_recompute_cost(factor);
        
        access_frequency * compute_cost > memory_cost
    }
}
```

### Lazy Message Passing

```rust
struct LazyMessagePassing {
    // Store message computations, not messages
    pending_messages: HashMap<EdgeId, MessageComputation>,
    
    // Cache for computed messages
    message_cache: LruCache<EdgeId, Message>,
}

impl LazyMessagePassing {
    fn get_message(&mut self, edge: EdgeId) -> &Message {
        // Check cache first
        if let Some(msg) = self.message_cache.get(&edge) {
            return msg;
        }
        
        // Check if computation is pending
        if let Some(comp) = self.pending_messages.remove(&edge) {
            let msg = comp.compute();
            self.message_cache.put(edge, msg);
            return self.message_cache.get(&edge).unwrap();
        }
        
        // Schedule computation
        self.schedule_message_computation(edge);
        self.wait_for_message(edge)
    }
    
    fn compute_all_messages(&mut self) {
        // Batch computation: compute all pending messages together
        let computations: Vec<_> = self.pending_messages.drain().collect();
        
        // Parallel computation
        let results: Vec<_> = computations.par_iter()
            .map(|(edge, comp)| (edge, comp.compute()))
            .collect();
        
        // Store results
        for (edge, msg) in results {
            self.message_cache.put(*edge, msg);
        }
    }
}
```

---

## Chunked Processing

### Processing Networks in Chunks

For networks too large to process in memory:

```rust
struct ChunkedInference {
    chunk_size: usize,
    overlap_size: usize,
}

impl ChunkedInference {
    fn process_in_chunks<F, T>(
        &self,
        model: &FactorGraph,
        chunk_processor: F
    ) -> Vec<T>
    where F: Fn(&FactorGraph) -> T
    {
        let chunks = self.partition_into_chunks(model);
        
        chunks.par_iter()
            .map(|chunk| {
                // Extend chunk with boundary variables
                let extended_chunk = self.extend_chunk(chunk, model);
                chunk_processor(&extended_chunk)
            })
            .collect()
    }
    
    fn partition_into_chunks(&self, model: &FactorGraph) -> Vec<NodeSet> {
        // Use graph partitioning (METIS or similar)
        partition_graph(model, self.chunk_size, self.overlap_size)
    }
    
    fn extend_chunk(&self, chunk: &NodeSet, model: &FactorGraph) -> FactorGraph {
        // Include Markov blanket of chunk for boundary conditions
        let boundary = self.compute_boundary(chunk, model);
        model.induced_subgraph(&chunk.union(&boundary))
    }
}
```

### Mini-Batch Inference

For inference with large datasets:

```rust
struct MiniBatchInference {
    batch_size: usize,
    n_samples: usize,
}

impl MiniBatchInference {
    fn approximate_marginals(
        &self,
        model: &FactorGraph,
        data: &DataFrame
    ) -> HashMap<VariableId, Belief> {
        let mut accumulated_beliefs: HashMap<VariableId, Vec<Belief>> = HashMap::new();
        
        for batch in data.chunks(self.batch_size) {
            // Create evidence batch
            let evidence_batch = self.create_evidence_batch(batch);
            
            // Run inference with evidence
            let beliefs = self.inference_engine.infer(
                model,
                &InferenceQuery::AllMarginals,
                &evidence_batch
            );
            
            // Accumulate
            for (var, belief) in beliefs {
                accumulated_beliefs.entry(var)
                    .or_default()
                    .push(belief);
            }
        }
        
        // Aggregate across batches
        self.aggregate_beliefs(accumulated_beliefs)
    }
}
```

### Streaming Algorithms

For continuous data streams:

```rust
struct StreamingInference {
    window_size: usize,
    update_frequency: Duration,
}

impl StreamingInference {
    async fn run_streaming_inference(
        &mut self,
        model: Arc<FactorGraph>,
        mut data_stream: DataStream
    ) {
        let mut rolling_window = RollingWindow::new(self.window_size);
        
        while let Some(observation) = data_stream.next().await {
            // Add to window
            rolling_window.add(observation);
            
            // Periodically update inference
            if rolling_window.should_update(self.update_frequency) {
                let subgraph = rolling_window.get_active_subgraph(&model);
                let beliefs = self.update_inference(&subgraph).await;
                
                // Emit updated beliefs
                self.emit_beliefs(beliefs);
            }
        }
    }
}
```

---

## Approximate Methods for Scale

### When to Switch from Exact to Approximate

```rust
enum InferenceApproach {
    Exact,
    BoundedExact { max_complexity: usize },
    Approximate { method: ApproximateMethod },
    Hybrid { exact_region: NodeSet, approximate_region: NodeSet },
}

fn select_inference_approach(
    model: &FactorGraph,
    query: &Query,
    constraints: &InferenceConstraints
) -> InferenceApproach {
    let tw_estimate = estimate_treewidth(model);
    let memory_estimate = estimate_memory_requirements(model, query);
    
    // Check if exact inference is feasible
    if tw_estimate.upper_bound <= 20 
        && memory_estimate < constraints.available_memory
        && !constraints.timeout_would_exceed(tw_estimate) {
        return InferenceApproach::Exact;
    }
    
    // Check bounded exact
    if !constraints.require_exact && tw_estimate.upper_bound <= 30 {
        return InferenceApproach::BoundedExact {
            max_complexity: constraints.available_memory / 1_000_000,
        };
    }
    
    // Determine best approximate method
    let approximate_method = select_approximate_method(model, query);
    
    // Check if hybrid approach makes sense
    if can_decompose(model) {
        let (exact_region, approx_region) = decompose_model(model, query);
        return InferenceApproach::Hybrid { 
            exact_region, 
            approximate_region: approx_region 
        };
    }
    
    InferenceApproach::Approximate { method: approximate_method }
}
```

### Automatic Selection Algorithm

```rust
fn select_approximate_method(
    model: &FactorGraph,
    query: &Query
) -> ApproximateMethod {
    let graph_properties = analyze_graph_properties(model);
    
    if model.is_tree() || model.is_singly_connected() {
        // Even "loopy" BP is exact on trees
        return ApproximateMethod::BeliefPropagation {
            damping: 0.0,
            max_iterations: 10,
        };
    }
    
    if graph_properties.average_cycle_length > 10 
        && graph_properties.clustering_coefficient < 0.1 {
        // Long cycles, low clustering: LBP often converges well
        return ApproximateMethod::LoopyBeliefPropagation {
            damping: 0.5,
            max_iterations: 1000,
        };
    }
    
    if graph_properties.n_potentials > 100_000 {
        // Very large model: use sampling
        return ApproximateMethod::GibbsSampling {
            n_samples: 10_000,
            burn_in: 1_000,
        };
    }
    
    if query.requires_joint_marginal() {
        // Joint queries: variational methods often better
        return ApproximateMethod::MeanField;
    }
    
    // Default: LBP with convergence monitoring
    ApproximateMethod::LoopyBeliefPropagation {
        damping: 0.5,
        max_iterations: 1000,
    }
}
```

### Quality Monitoring

```rust
struct ApproximationQualityMonitor {
    convergence_history: Vec<f64>,
    bound_computer: Option<Box<dyn BoundComputer>>,
}

impl ApproximationQualityMonitor {
    fn check_quality(&self, beliefs: &BeliefState) -> QualityAssessment {
        // Check convergence
        let convergence_score = self.assess_convergence();
        
        // Compute bounds if available
        let bound_quality = self.bound_computer
            .as_ref()
            .map(|bc| bc.compute_bounds(beliefs));
        
        // Compare with sampling if uncertain
        let sampling_check = if convergence_score < 0.9 {
            Some(self.quick_sampling_check(beliefs))
        } else {
            None
        };
        
        QualityAssessment {
            convergence_score,
            bound_quality,
            sampling_check,
            overall_confidence: self.compute_confidence(
                convergence_score, 
                bound_quality, 
                sampling_check
            ),
        }
    }
    
    fn quick_sampling_check(&self, beliefs: &BeliefState) -> SamplingResult {
        // Run short MCMC to verify beliefs
        let sampler = GibbsSampler::with_config(SamplerConfig {
            n_samples: 1000,
            burn_in: 100,
        });
        
        let sample_beliefs = sampler.sample_and_estimate(self.model);
        self.compare_beliefs(beliefs, &sample_beliefs)
    }
}
```

---

## Stochastic Variational Inference

### SVI for Massive Networks

Stochastic Variational Inference (SVI) scales variational methods to massive datasets by subsampling:

```rust
struct StochasticVariationalInference {
    // Global variational parameters
    global_params: VariationalParameters,
    
    // Local parameters (per data point)
    local_param_cache: LruCache<usize, LocalParameters>,
    
    // Optimization settings
    learning_rate: LearningRateSchedule,
    batch_size: usize,
    
    // Subsampling
    subsample_ratio: f64,
}

impl StochasticVariationalInference {
    fn fit(&mut self, data: &Dataset, model: &FactorGraph) {
        for iteration in 0..self.max_iterations {
            // Subsample data
            let batch = data.sample(self.batch_size);
            
            // Compute local variational parameters for batch
            let local_params: Vec<_> = batch.par_iter()
                .map(|(idx, observation)| {
                    self.compute_local_params(*idx, observation, &self.global_params)
                })
                .collect();
            
            // Compute gradient of ELBO
            let gradient = self.compute_natural_gradient(&local_params, data.len());
            
            // Update global parameters
            let lr = self.learning_rate.get(iteration);
            self.global_params = self.global_params.update(gradient, lr);
            
            // Check convergence
            if self.has_converged(iteration) {
                break;
            }
        }
    }
    
    fn compute_natural_gradient(
        &self,
        local_params: &[LocalParameters],
        dataset_size: usize
    ) -> VariationalGradient {
        // Scale gradient by dataset size / batch size
        let scale = dataset_size as f64 / local_params.len() as f64;
        
        local_params.iter()
            .map(|lp| self.local_gradient(lp))
            .fold(VariationalGradient::zero(), |acc, g| acc + g)
            * scale
    }
}
```

### Subsampling Nodes

For networks with millions of nodes:

```rust
struct NodeSubsamplingVI {
    // Partition nodes into groups
    node_groups: Vec<NodeSet>,
    
    // Active groups (sampled each iteration)
    active_groups: HashSet<usize>,
}

impl NodeSubsamplingVI {
    fn subsample_nodes(&mut self, model: &FactorGraph) -> NodeSet {
        // Sample groups rather than individual nodes
        let n_groups_to_sample = (self.node_groups.len() as f64 * 
            self.subsample_ratio).ceil() as usize;
        
        let sampled_groups: Vec<_> = self.node_groups.choose_multiple(
            &mut thread_rng(),
            n_groups_to_sample
        ).collect();
        
        // Include boundary nodes for consistency
        sampled_groups.iter()
            .flat_map(|g| g.iter().cloned())
            .chain(self.get_boundary_nodes(&sampled_groups))
            .collect()
    }
}
```

### Natural Gradients

```rust
struct NaturalGradientSVI {
    fisher_information: SparseMatrix,
}

impl NaturalGradientSVI {
    fn compute_natural_gradient(
        &self,
        vanilla_gradient: &VariationalGradient
    ) -> VariationalGradient {
        // Natural gradient = F^{-1} * ∇L
        // where F is Fisher information matrix
        self.fisher_information.solve(vanilla_gradient)
    }
    
    fn update_with_adagrad_natural(
        &mut self,
        params: &mut VariationalParameters,
        gradient: &VariationalGradient,
        iteration: usize
    ) {
        // AdaGrad with natural gradients
        let natural_grad = self.compute_natural_gradient(gradient);
        
        self.accumulated_squared_gradients += &natural_grad.elementwise_square();
        
        let adapted_lr = self.learning_rate / 
            (self.accumulated_squared_gradients.sqrt() + 1e-8);
        
        *params -= natural_grad * adapted_lr;
    }
}
```

### Convergence at Scale

```rust
struct DistributedConvergenceMonitor {
    // Local convergence metrics (per worker)
    local_metrics: Vec<ConvergenceMetric>,
    
    // Global coordinator
    coordinator: Arc<ConvergenceCoordinator>,
}

impl DistributedConvergenceMonitor {
    async fn check_global_convergence(&self) -> ConvergenceStatus {
        // Collect metrics from all workers
        let all_metrics = self.gather_metrics().await;
        
        // Check global convergence criteria
        let max_change = all_metrics.iter()
            .map(|m| m.param_change)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        
        let avg_elbo_change = all_metrics.iter()
            .map(|m| m.elbo_change)
            .sum::<f64>() / all_metrics.len() as f64;
        
        if max_change < self.param_tolerance 
            && avg_elbo_change < self.elbo_tolerance {
            ConvergenceStatus::Converged
        } else if self.iteration >= self.max_iterations {
            ConvergenceStatus::MaxIterations
        } else {
            ConvergenceStatus::Continue
        }
    }
}
```

---

## Distributed Computation

### Parallel Message Passing

```rust
struct ParallelBeliefPropagation {
    thread_pool: ThreadPool,
    partition: GraphPartition,
}

impl ParallelBeliefPropagation {
    fn parallel_message_update(&mut self, beliefs: &mut BeliefState) {
        // Partition messages into independent sets
        let independent_sets = self.partition.into_independent_sets();
        
        for message_set in independent_sets {
            // Update messages in parallel
            self.thread_pool.install(|| {
                message_set.par_iter().for_each(|msg_id| {
                    self.update_message(*msg_id, beliefs);
                });
            });
        }
    }
    
    fn update_message(&self, msg_id: MessageId, beliefs: &mut BeliefState) {
        let edge = self.graph.get_edge(msg_id);
        let incoming = self.get_incoming_messages(edge, beliefs);
        let new_message = self.compute_message(&incoming);
        beliefs.update_message(msg_id, new_message);
    }
}
```

### MapReduce Patterns

```rust
struct MapReduceInference {
    map_fn: Box<dyn Fn(&FactorGraph, &NodeSet) -> LocalResult + Send + Sync>,
    reduce_fn: Box<dyn Fn(&[LocalResult]) -> GlobalResult + Send + Sync>,
}

impl MapReduceInference {
    fn distributed_inference(
        &self,
        model: &FactorGraph,
        partitions: Vec<NodeSet>
    ) -> GlobalResult {
        // Map phase: process each partition independently
        let local_results: Vec<_> = partitions.par_iter()
            .map(|partition| {
                let subgraph = model.induced_subgraph(partition);
                (self.map_fn)(&subgraph, partition)
            })
            .collect();
        
        // Reduce phase: combine results
        (self.reduce_fn)(&local_results)
    }
}

// Example: Distributed parameter learning
let map_reduce = MapReduceInference {
    map_fn: Box::new(|subgraph, _| {
        // Compute local sufficient statistics
        compute_sufficient_statistics(subgraph)
    }),
    reduce_fn: Box::new(|local_stats| {
        // Aggregate statistics
        let global_stats = aggregate_sufficient_statistics(local_stats);
        // Update parameters
        update_parameters(global_stats)
    }),
};
```

### Distributed Junction Tree

Challenges and workarounds for distributed junction trees:

```rust
struct DistributedJunctionTree {
    // Clique tree partitioned across nodes
    local_cliques: Vec<Clique>,
    boundary_cliques: Vec<BoundaryClique>,
    
    // Communication pattern
    communication_graph: CommunicationGraph,
}

impl DistributedJunctionTree {
    fn distributed_message_passing(&mut self) {
        // Phase 1: Local collect (no communication)
        for clique in &mut self.local_cliques {
            self.local_collect(clique);
        }
        
        // Phase 2: Inter-node communication
        self.exchange_boundary_messages();
        
        // Phase 3: Local distribute
        for clique in &mut self.local_cliques {
            self.local_distribute(clique);
        }
    }
    
    fn exchange_boundary_messages(&mut self) {
        // Send messages across partition boundaries
        let send_futures: Vec<_> = self.boundary_cliques.iter()
            .map(|bc| {
                let message = self.compute_boundary_message(bc);
                self.send_to_partition(bc.target_partition, message)
            })
            .collect();
        
        // Wait for all sends and corresponding receives
        let received = block_on(join_all(send_futures));
        
        // Incorporate received messages
        for (bc, msg) in self.boundary_cliques.iter().zip(received) {
            self.incorporate_boundary_message(bc, msg);
        }
    }
}
```

**Workarounds for Distributed JT Challenges:**

1. **Large Cliques**: Split across nodes using tensor decomposition
2. **High Communication**: Use asynchronous message passing with convergence detection
3. **Load Imbalance**: Dynamic repartitioning based on clique sizes
4. **Fault Tolerance**: Checkpoint intermediate results

### GPU Acceleration Possibilities

```rust
#[cfg(feature = "gpu")]
struct GPUFactorOperations {
    context: CudaContext,
    factor_buffers: DeviceBuffers,
}

#[cfg(feature = "gpu")]
impl GPUFactorOperations {
    fn parallel_factor_multiply(
        &self,
        factors: &[&DenseFactor]
    ) -> DenseFactor {
        // Upload factors to GPU
        let device_factors: Vec<_> = factors.iter()
            .map(|f| self.upload_factor(f))
            .collect();
        
        // Launch CUDA kernel for parallel multiplication
        let result_buffer = self.context.launch_kernel(
            "parallel_factor_multiply",
            &device_factors,
            factors.len()
        );
        
        // Download result
        self.download_factor(&result_buffer)
    }
    
    fn gpu_belief_propagation(&self, graph: &FactorGraph) -> BeliefState {
        // Convert graph to GPU-friendly format
        let gpu_graph = self.convert_to_gpu_format(graph);
        
        // Run BP iterations on GPU
        let mut beliefs = self.initialize_beliefs(&gpu_graph);
        
        for _ in 0..self.max_iterations {
            let new_beliefs = self.gpu_message_passing_step(&gpu_graph, &beliefs);
            
            if self.gpu_convergence_check(&beliefs, &new_beliefs) {
                break;
            }
            
            beliefs = new_beliefs;
        }
        
        self.download_beliefs(&beliefs)
    }
}
```

**GPU Suitability Analysis:**

| Operation | GPU Suitability | Notes |
|-----------|-----------------|-------|
| Factor multiplication | High | Embarrassingly parallel across assignments |
| Message passing | Medium | Depends on graph structure; regular grids work well |
| Clique tree operations | Low | Irregular memory access patterns |
| Sampling | High | Independent samples parallelize perfectly |
| Gradient computation | High | Data-parallel across minibatches |

---

## Graph Partitioning for Scale

### Community-Based Partitioning

```rust
struct CommunityPartitioner {
    algorithm: CommunityAlgorithm,
    resolution: f64,
}

enum CommunityAlgorithm {
    Louvain,
    Leiden,
    LabelPropagation,
    Spectral,
}

impl CommunityPartitioner {
    fn partition(&self, graph: &FactorGraph, n_partitions: usize) -> Vec<Partition> {
        // Detect communities
        let communities = match self.algorithm {
            CommunityAlgorithm::Louvain => {
                self.louvain_communities(graph)
            }
            CommunityAlgorithm::Leiden => {
                self.leiden_communities(graph)
            }
            // ... other algorithms
        };
        
        // Merge small communities to reach target partition count
        self.merge_to_target(communities, n_partitions)
    }
    
    fn louvain_communities(&self, graph: &FactorGraph) -> Vec<Community> {
        let mut partition: Vec<usize> = (0..graph.n_nodes()).collect();
        let mut improved = true;
        
        while improved {
            improved = false;
            
            for node in 0..graph.n_nodes() {
                let current_comm = partition[node];
                let best_comm = self.find_best_community(node, &partition, graph);
                
                if best_comm != current_comm {
                    partition[node] = best_comm;
                    improved = true;
                }
            }
        }
        
        self.partition_to_communities(&partition)
    }
}
```

### METIS Integration

```rust
struct MetisPartitioner {
    options: MetisOptions,
}

struct MetisOptions {
    objective: PartitionObjective,
    n_parts: usize,
    ufactor: usize,  // Load imbalance tolerance
    niter: usize,    // Refinement iterations
}

impl MetisPartitioner {
    fn partition(&self, graph: &FactorGraph) -> Vec<Partition> {
        // Convert to METIS format
        let (xadj, adjncy) = self.to_metis_format(graph);
        
        // Call METIS
        let mut partition_ids = vec![0; graph.n_nodes()];
        
        unsafe {
            metis::METIS_PartGraphKway(
                &mut (graph.n_nodes() as i32),
                &mut 1,  // ncon
                xadj.as_ptr(),
                adjncy.as_ptr(),
                null(),  // vwgt
                null(),  // vsize
                null(),  // adjwgt
                &mut (self.options.n_parts as i32),
                null(),  // tpwgts
                &mut (self.options.ufactor as f32),
                self.options.to_metis_opts(),
                &mut 0,  // edgecut
                partition_ids.as_mut_ptr(),
            );
        }
        
        // Convert to partitions
        self.to_partitions(&partition_ids, graph)
    }
}
```

### Handling Cut Edges

```rust
struct CutEdgeHandler {
    // Strategies for handling edges cut by partitioning
    strategy: CutStrategy,
}

enum CutStrategy {
    // Replicate cut variables on both partitions
    Replication,
    
    // Create special boundary cliques
    BoundaryCliques,
    
    // Use approximate messages across cuts
    ApproximateMessages,
    
    // Iterative refinement across partition boundary
    IterativeRefinement,
}

impl CutEdgeHandler {
    fn handle_cuts(&self, partitions: &[Partition], graph: &FactorGraph) -> DistributedGraph {
        let cut_edges = self.identify_cut_edges(partitions, graph);
        
        match self.strategy {
            CutStrategy::Replication => {
                self.replicate_cut_variables(&cut_edges, partitions, graph)
            }
            CutStrategy::BoundaryCliques => {
                self.create_boundary_cliques(&cut_edges, partitions, graph)
            }
            CutStrategy::ApproximateMessages => {
                self.setup_approximate_messaging(&cut_edges, partitions)
            }
            CutStrategy::IterativeRefinement => {
                self.setup_iterative_refinement(&cut_edges, partitions)
            }
        }
    }
    
    fn replicate_cut_variables(
        &self,
        cut_edges: &[(NodeId, NodeId)],
        partitions: &[Partition],
        graph: &FactorGraph
    ) -> DistributedGraph {
        let mut replicated = HashMap::new();
        
        for (u, v) in cut_edges {
            let partition_u = self.find_partition(*u, partitions);
            let partition_v = self.find_partition(*v, partitions);
            
            // Replicate u in partition_v's copy
            replicated.entry(partition_v)
                .or_insert_with(Vec::new)
                .push(*u);
            
            // Replicate v in partition_u's copy
            replicated.entry(partition_u)
                .or_insert_with(Vec::new)
                .push(*v);
        }
        
        DistributedGraph::new(partitions, replicated)
    }
}
```

---

## Progressive Loading

### Loading Only Active Portions

```rust
struct ProgressiveLoader {
    cache: LruCache<NodeId, FactorGraph>,
    storage: Arc<dyn GraphStorage>,
    active_nodes: HashSet<NodeId>,
}

impl ProgressiveLoader {
    fn load_active_region(&mut self, center: NodeId, radius: usize) -> FactorGraph {
        // Compute region to load
        let region = self.compute_region(center, radius);
        
        // Check cache for existing nodes
        let (cached, to_load): (Vec<_>, Vec<_>) = region.iter()
            .partition(|n| self.cache.contains(*n));
        
        // Load missing nodes
        let loaded = self.storage.load_nodes(&to_load);
        
        // Merge cached and loaded
        let mut merged = self.merge_subgraphs(&cached);
        merged.merge(loaded);
        
        // Update active set
        self.active_nodes = region.iter().cloned().collect();
        
        merged
    }
    
    fn compute_region(&self, center: NodeId, radius: usize) -> Vec<NodeId> {
        // BFS to find all nodes within radius
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((center, 0));
        
        while let Some((node, dist)) = queue.pop_front() {
            if dist > radius || visited.contains(&node) {
                continue;
            }
            
            visited.insert(node);
            
            // Add neighbors
            for neighbor in self.storage.get_neighbors(node) {
                queue.push_back((neighbor, dist + 1));
            }
        }
        
        visited.into_iter().collect()
    }
}
```

### Virtual Memory Techniques

```rust
struct VirtualMemoryGraph {
    // Page table for node location
    page_table: Vec<PageEntry>,
    
    // Page cache
    page_cache: LruCache<PageId, GraphPage>,
    
    // Backing storage
    backing_file: MemoryMappedFile,
}

struct PageEntry {
    page_id: PageId,
    in_memory: bool,
    dirty: bool,
}

impl VirtualMemoryGraph {
    fn access_node(&mut self, node: NodeId) -> &Node {
        let page_id = self.page_table[node].page_id;
        
        // Page in if not in memory
        if !self.page_table[node].in_memory {
            self.page_in(page_id);
        }
        
        // Return reference
        let page = self.page_cache.get(&page_id).unwrap();
        &page.get_node(node)
    }
    
    fn page_in(&mut self, page_id: PageId) {
        // Evict if necessary
        if self.page_cache.len() >= MAX_CACHED_PAGES {
            self.evict_page();
        }
        
        // Load from backing file
        let page = self.backing_file.read_page(page_id);
        self.page_cache.put(page_id, page);
        
        // Update page table
        for node in page.nodes() {
            self.page_table[node].in_memory = true;
        }
    }
    
    fn evict_page(&mut self) {
        if let Some((evicted_id, evicted_page)) = self.page_cache.pop_lru() {
            // Write back if dirty
            if self.is_page_dirty(evicted_id) {
                self.backing_file.write_page(evicted_id, &evicted_page);
            }
            
            // Update page table
            for node in evicted_page.nodes() {
                self.page_table[node].in_memory = false;
                self.page_table[node].dirty = false;
            }
        }
    }
}
```

### Demand Paging

```rust
struct DemandPagedInference {
    loader: ProgressiveLoader,
    page_faults: AtomicUsize,
}

impl DemandPagedInference {
    fn infer_with_paging(&mut self, query: &Query) -> QueryResult {
        let mut beliefs = BeliefState::new();
        let mut nodes_to_process: VecDeque<NodeId> = query.variables.iter().cloned().collect();
        
        while let Some(node) = nodes_to_process.pop_front() {
            // Check if node is loaded
            if !self.loader.is_loaded(node) {
                self.page_faults.fetch_add(1, Ordering::Relaxed);
                
                // Load region around node
                let region = self.loader.load_active_region(node, PAGING_RADIUS);
                
                // Update processing queue with neighbors
                for neighbor in region.neighbors(node) {
                    if !beliefs.has_belief(neighbor) {
                        nodes_to_process.push_back(neighbor);
                    }
                }
            }
            
            // Compute belief for node
            let belief = self.compute_belief(node, &beliefs);
            beliefs.set(node, belief);
        }
        
        QueryResult::from_beliefs(beliefs, query)
    }
}
```

---

## Compression

### Compressing Sparse Structures

```rust
struct CompressedFactorGraph {
    // Compressed adjacency
    adjacency: CompressedSparseRow,
    
    // Compressed factors
    factors: Vec<CompressedFactor>,
    
    // Compression statistics
    compression_ratio: f64,
}

struct CompressedFactor {
    compression_scheme: CompressionScheme,
    compressed_data: Vec<u8>,
    uncompressed_size: usize,
}

enum CompressionScheme {
    // Run-length encoding for repetitive values
    RLE,
    
    // Dictionary coding for frequent values
    Dictionary,
    
    // Delta encoding for sorted values
    Delta,
    
    // Bit packing for discrete values
    BitPacking,
    
    // Lossy: quantize to fewer bits
    Quantization { bits: u8 },
}

impl CompressedFactor {
    fn compress(factor: &DenseFactor, scheme: CompressionScheme) -> Self {
        let uncompressed_size = factor.memory_bytes();
        
        let compressed_data = match scheme {
            CompressionScheme::RLE => Self::rle_compress(&factor.values),
            CompressionScheme::Dictionary => Self::dict_compress(&factor.values),
            CompressionScheme::Delta => Self::delta_compress(&factor.values),
            CompressionScheme::BitPacking => Self::bitpack_compress(&factor.values),
            CompressionScheme::Quantization { bits } => {
                Self::quantize_compress(&factor.values, bits)
            }
        };
        
        let compression_ratio = uncompressed_size as f64 / compressed_data.len() as f64;
        
        Self {
            compression_scheme: scheme,
            compressed_data,
            uncompressed_size,
        }
    }
    
    fn decompress(&self) -> DenseFactor {
        match self.compression_scheme {
            CompressionScheme::RLE => Self::rle_decompress(&self.compressed_data),
            CompressionScheme::Dictionary => Self::dict_decompress(&self.compressed_data),
            // ... other schemes
        }
    }
}
```

### Factor Compression

```rust
struct FactorCompression {
    // Threshold for determining when to compress
    compression_threshold: f64,
    
    // Maximum acceptable error (for lossy)
    max_error: f64,
}

impl FactorCompression {
    fn adaptive_compress(&self, factor: &DenseFactor) -> CompressedFactor {
        let sparsity = self.compute_sparsity(factor);
        let value_entropy = self.compute_value_entropy(factor);
        
        if sparsity > 0.9 {
            // Highly sparse: use sparse format + light compression
            CompressedFactor::sparse(factor)
        } else if value_entropy < 2.0 {
            // Low entropy: dictionary coding
            CompressedFactor::dict_coded(factor)
        } else if self.allows_lossy && self.can_quantize(factor) {
            // Quantize to 8 or 16 bits
            CompressedFactor::quantized(factor, self.optimal_bit_depth(factor))
        } else {
            // Default: delta + gzip
            CompressedFactor::delta_gzipped(factor)
        }
    }
    
    fn can_quantize(&self, factor: &DenseFactor) -> bool {
        // Check if quantization would preserve required precision
        let max_value = factor.values.iter().cloned().fold(0.0, f64::max);
        let min_nonzero = factor.values.iter()
            .filter(|&&v| v > 0.0)
            .cloned()
            .fold(f64::INFINITY, f64::min);
        
        let dynamic_range = max_value / min_nonzero;
        dynamic_range < 1e6  // Can represent with reasonable bit depth
    }
}
```

### Lossy Compression for Beliefs

```rust
struct LossyBeliefCompression {
    // Target precision for beliefs
    target_precision: f64,
    
    // Adaptive bit allocation
    bit_allocator: AdaptiveBitAllocator,
}

impl LossyBeliefCompression {
    fn compress_beliefs(&self, beliefs: &BeliefState) -> CompressedBeliefs {
        let mut compressed = Vec::new();
        
        for (var, belief) in beliefs.iter() {
            match belief {
                Belief::Discrete { probabilities } => {
                    // Quantize probabilities
                    let n_bits = self.bit_allocator.allocate_bits(probabilities);
                    let quantized = self.quantize_probabilities(probabilities, n_bits);
                    compressed.push((var, CompressedBelief::Quantized(quantized)));
                }
                Belief::Gaussian { mean, variance } => {
                    // Store mean and log-variance with limited precision
                    compressed.push((var, CompressedBelief::Gaussian {
                        mean: self.quantize_f64(*mean, 24),
                        log_variance: self.quantize_f64(variance.ln(), 16),
                    }));
                }
                _ => {
                    // Default: store as-is
                    compressed.push((var, CompressedBelief::Full(belief.clone())));
                }
            }
        }
        
        CompressedBeliefs::new(compressed)
    }
    
    fn quantize_probabilities(&self, probs: &[f64], n_bits: u8) -> QuantizedProbs {
        let n_levels = 2usize.pow(n_bits as u32);
        let quantized: Vec<u8> = probs.iter()
            .map(|&p| {
                let level = (p * (n_levels - 1) as f64).round() as usize;
                level.min(n_levels - 1) as u8
            })
            .collect();
        
        QuantizedProbs {
            n_bits,
            levels: quantized,
        }
    }
}
```

---

## Scalability Tiers

### Explicit Tier Documentation

```rust
pub struct ScalabilityTier {
    pub name: &'static str,
    pub max_nodes: usize,
    pub max_edges: usize,
    pub max_treewidth: usize,
    pub recommended_algorithms: Vec<InferenceAlgorithm>,
    pub memory_requirements: MemoryRequirements,
    pub hardware_recommendations: HardwareRecommendations,
}

pub const TIERS: &[ScalabilityTier] = &[
    ScalabilityTier {
        name: "Small",
        max_nodes: 1_000,
        max_edges: 5_000,
        max_treewidth: 15,
        recommended_algorithms: vec![
            InferenceAlgorithm::JunctionTree,
            InferenceAlgorithm::VariableElimination,
            InferenceAlgorithm::BeliefPropagation,
        ],
        memory_requirements: MemoryRequirements {
            min_ram_gb: 0.5,
            recommended_ram_gb: 2.0,
            disk_required: false,
        },
        hardware_recommendations: HardwareRecommendations {
            min_cores: 1,
            recommended_cores: 4,
            gpu_beneficial: false,
        },
    },
    ScalabilityTier {
        name: "Medium",
        max_nodes: 100_000,
        max_edges: 500_000,
        max_treewidth: 25,
        recommended_algorithms: vec![
            InferenceAlgorithm::BoundedJunctionTree,
            InferenceAlgorithm::LoopyBeliefPropagation,
            InferenceAlgorithm::GibbsSampling,
        ],
        memory_requirements: MemoryRequirements {
            min_ram_gb: 4.0,
            recommended_ram_gb: 16.0,
            disk_required: false,
        },
        hardware_recommendations: HardwareRecommendations {
            min_cores: 4,
            recommended_cores: 16,
            gpu_beneficial: true,
        },
    },
    ScalabilityTier {
        name: "Large",
        max_nodes: 10_000_000,
        max_edges: 50_000_000,
        max_treewidth: 40,
        recommended_algorithms: vec![
            InferenceAlgorithm::StochasticVariationalInference,
            InferenceAlgorithm::DistributedBeliefPropagation,
            InferenceAlgorithm::MiniBatchGibbs,
        ],
        memory_requirements: MemoryRequirements {
            min_ram_gb: 32.0,
            recommended_ram_gb: 128.0,
            disk_required: true,
        },
        hardware_recommendations: HardwareRecommendations {
            min_cores: 16,
            recommended_cores: 64,
            gpu_beneficial: true,
        },
    },
    ScalabilityTier {
        name: "Massive",
        max_nodes: usize::MAX,
        max_edges: usize::MAX,
        max_treewidth: usize::MAX,
        recommended_algorithms: vec![
            InferenceAlgorithm::DistributedSVI,
            InferenceAlgorithm::StreamingInference,
            InferenceAlgorithm::SamplingWithBounds,
        ],
        memory_requirements: MemoryRequirements {
            min_ram_gb: 128.0,
            recommended_ram_gb: 512.0,
            disk_required: true,
        },
        hardware_recommendations: HardwareRecommendations {
            min_cores: 64,
            recommended_cores: 256,
            distributed_required: true,
        },
    },
];
```

### Graceful Degradation Paths

```rust
struct GracefulDegradation {
    tier: ScalabilityTier,
    degradation_strategy: DegradationStrategy,
}

enum DegradationStrategy {
    // Reduce precision
    ReducePrecision { bits: u8 },
    
    // Increase approximation
    IncreaseApproximation { factor: f64 },
    
    // Sample instead of exact
    SwitchToSampling { n_samples: usize },
    
    // Process in smaller batches
    ReduceBatchSize { new_batch_size: usize },
    
    // Use disk-backed storage
    EnablePaging { page_size: usize },
    
    // Distribute computation
    EnableDistribution { n_workers: usize },
}

impl GracefulDegradation {
    fn apply(&self, model: &mut FactorGraph, query: &mut Query) {
        match &self.degradation_strategy {
            DegradationStrategy::ReducePrecision { bits } => {
                model.reduce_precision(*bits);
            }
            DegradationStrategy::IncreaseApproximation { factor } => {
                query.relax_convergence(*factor);
            }
            DegradationStrategy::SwitchToSampling { n_samples } => {
                query.set_algorithm(InferenceAlgorithm::GibbsSampling {
                    n_samples: *n_samples,
                    burn_in: n_samples / 10,
                });
            }
            DegradationStrategy::ReduceBatchSize { new_batch_size } => {
                query.set_batch_size(*new_batch_size);
            }
            DegradationStrategy::EnablePaging { page_size } => {
                model.enable_paging(*page_size);
            }
            DegradationStrategy::EnableDistribution { n_workers } => {
                query.enable_distribution(*n_workers);
            }
        }
    }
}
```

---

## Profiling and Monitoring

### Memory Profiling

```rust
struct MemoryProfiler {
    snapshots: Vec<MemorySnapshot>,
    allocation_tracker: HashMap<AllocationId, AllocationInfo>,
}

struct MemorySnapshot {
    timestamp: Instant,
    total_allocated: usize,
    by_category: HashMap<String, usize>,
    by_type: HashMap<String, usize>,
}

impl MemoryProfiler {
    fn record_allocation<T>(&mut self, ptr: *const T, size: usize, category: &str) {
        let id = AllocationId::new(ptr);
        self.allocation_tracker.insert(id, AllocationInfo {
            size,
            category: category.to_string(),
            type_name: std::any::type_name::<T>().to_string(),
            backtrace: Backtrace::capture(),
        });
        
        // Periodic snapshot
        if self.should_snapshot() {
            self.take_snapshot();
        }
    }
    
    fn take_snapshot(&mut self) {
        let snapshot = MemorySnapshot {
            timestamp: Instant::now(),
            total_allocated: self.allocation_tracker.values()
                .map(|info| info.size)
                .sum(),
            by_category: self.aggregate_by_category(),
            by_type: self.aggregate_by_type(),
        };
        
        self.snapshots.push(snapshot);
    }
    
    fn generate_report(&self) -> MemoryReport {
        MemoryReport {
            peak_memory: self.snapshots.iter()
                .map(|s| s.total_allocated)
                .max()
                .unwrap_or(0),
            memory_growth: self.compute_memory_growth(),
            top_allocators: self.find_top_allocators(),
            potential_leaks: self.detect_potential_leaks(),
        }
    }
}
```

### Timing Instrumentation

```rust
struct TimingProfiler {
    spans: Vec<TimingSpan>,
    current_stack: Vec<SpanId>,
}

struct TimingSpan {
    id: SpanId,
    name: String,
    start: Instant,
    end: Option<Instant>,
    parent: Option<SpanId>,
    metadata: HashMap<String, String>,
}

impl TimingProfiler {
    fn start_span(&mut self, name: &str) -> SpanGuard {
        let id = SpanId::new();
        let span = TimingSpan {
            id,
            name: name.to_string(),
            start: Instant::now(),
            end: None,
            parent: self.current_stack.last().cloned(),
            metadata: HashMap::new(),
        };
        
        self.spans.push(span);
        self.current_stack.push(id);
        
        SpanGuard { id, profiler: self }
    }
    
    fn end_span(&mut self, id: SpanId) {
        if let Some(span) = self.spans.iter_mut().find(|s| s.id == id) {
            span.end = Some(Instant::now());
        }
        
        if self.current_stack.last() == Some(&id) {
            self.current_stack.pop();
        }
    }
    
    fn generate_flamegraph(&self) -> Flamegraph {
        // Convert spans to flamegraph format
        let mut frames = Vec::new();
        
        for span in &self.spans {
            if let Some(end) = span.end {
                frames.push(FlameFrame {
                    name: span.name.clone(),
                    start_us: span.start.elapsed().as_micros(),
                    duration_us: (end - span.start).as_micros(),
                    depth: self.compute_depth(span),
                });
            }
        }
        
        Flamegraph::new(frames)
    }
}

// Usage
fn run_inference(profiler: &mut TimingProfiler, model: &FactorGraph) {
    let _span = profiler.start_span("inference");
    
    {
        let _span = profiler.start_span("triangulation");
        let triangulated = triangulate(model);
    }
    
    {
        let _span = profiler.start_span("message_passing");
        run_message_passing(&triangulated);
    }
}
```

### Bottleneck Identification

```rust
struct BottleneckAnalyzer {
    profiler: Arc<TimingProfiler>,
    memory_profiler: Arc<MemoryProfiler>,
}

impl BottleneckAnalyzer {
    fn identify_bottlenecks(&self) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();
        
        // CPU bottlenecks
        let cpu_bottlenecks = self.identify_cpu_bottlenecks();
        bottlenecks.extend(cpu_bottlenecks);
        
        // Memory bottlenecks
        let memory_bottlenecks = self.identify_memory_bottlenecks();
        bottlenecks.extend(memory_bottlenecks);
        
        // I/O bottlenecks
        let io_bottlenecks = self.identify_io_bottlenecks();
        bottlenecks.extend(io_bottlenecks);
        
        // Sort by impact
        bottlenecks.sort_by(|a, b| b.impact.partial_cmp(&a.impact).unwrap());
        
        bottlenecks
    }
    
    fn identify_memory_bottlenecks(&self) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();
        
        // Check for excessive allocations
        let allocation_rate = self.memory_profiler.allocation_rate();
        if allocation_rate > HIGH_ALLOCATION_THRESHOLD {
            bottlenecks.push(Bottleneck {
                category: "Memory",
                description: "High allocation rate detected".to_string(),
                impact: (allocation_rate / HIGH_ALLOCATION_THRESHOLD) as f64,
                suggestions: vec![
                    "Use memory pools for small objects".to_string(),
                    "Consider object reuse".to_string(),
                ],
            });
        }
        
        // Check for memory growth
        let growth_rate = self.memory_profiler.memory_growth_rate();
        if growth_rate > ACCEPTABLE_GROWTH_RATE {
            bottlenecks.push(Bottleneck {
                category: "Memory",
                description: "Potential memory leak or unbounded growth".to_string(),
                impact: (growth_rate / ACCEPTABLE_GROWTH_RATE) as f64,
                suggestions: vec![
                    "Check for circular references".to_string(),
                    "Verify cache size limits".to_string(),
                ],
            });
        }
        
        bottlenecks
    }
}
```

---

## How Lutufi Scales

### Architectural Decisions for Scalability

**1. Multi-Representation Architecture**
Lutufi's factor graph as canonical representation allows different scalability strategies for different model types:
- Sparse CPDs for discrete BNs
- Gaussian factors for continuous models
- Hybrid representations for mixed models

**2. Algorithm Selection by Complexity**
Automatic selection based on treewidth, node count, and query type ensures appropriate methods are used:
- Exact inference for tractable subproblems
- Bounded exact for medium complexity
- Approximate for intractable problems

**3. Progressive Degradation**
When resources are constrained, Lutufi gracefully degrades:
1. Reduce precision (f32 instead of f64)
2. Increase approximation tolerance
3. Switch to sampling
4. Enable paging
5. Distribute computation

**4. Memory Hierarchy Exploitation**
Explicit management of memory hierarchy:
- Hot data in L1/L2 cache (small factor caches)
- Working set in RAM (active graph portions)
- Cold data on disk (memory-mapped factors)
- Archived data in object storage

### Scalability Code Example

```rust
// Example: Processing a massive network
async fn process_massive_network(model_path: &Path) -> Result<InferenceResult> {
    // 1. Estimate scale and select tier
    let scale_estimate = estimate_network_scale(model_path).await?;
    let tier = select_tier(scale_estimate);
    
    println!("Processing {} nodes, {} edges (Tier: {})",
        scale_estimate.n_nodes,
        scale_estimate.n_edges,
        tier.name
    );
    
    // 2. Configure based on tier
    let config = InferenceConfig::for_tier(tier);
    
    // 3. Load with appropriate strategy
    let loader = ProgressiveLoader::new(model_path, config.memory_config);
    
    // 4. Run distributed inference if needed
    if tier.requires_distribution {
        let cluster = Cluster::connect().await?;
        let result = cluster.run_distributed_inference(loader, config).await?;
        Ok(result)
    } else {
        let model = loader.load_full().await?;
        let result = run_inference(&model, config)?;
        Ok(result)
    }
}
```

### Scalability Validation

```rust
#[cfg(test)]
mod scalability_tests {
    use super::*;
    
    #[test]
    fn test_small_scale_performance() {
        let model = generate_random_tree(1000);
        
        let start = Instant::now();
        let result = model.query_all_marginals();
        let elapsed = start.elapsed();
        
        assert!(elapsed < Duration::from_millis(100),
            "Small scale inference too slow: {:?}", elapsed);
    }
    
    #[test]
    fn test_medium_scale_memory() {
        let model = generate_scale_free_network(100_000, 2.5);
        
        let memory_before = get_memory_usage();
        let _result = model.query_all_marginals();
        let memory_after = get_memory_usage();
        
        let memory_used = memory_after - memory_before;
        let bytes_per_node = memory_used / 100_000;
        
        assert!(bytes_per_node < 1000,
            "Memory usage too high: {} bytes/node", bytes_per_node);
    }
    
    #[test]
    fn test_large_scale_convergence() {
        let model = generate_social_network(1_000_000);
        
        let result = model.approximate_query(
            ApproximationConfig {
                method: ApproximateMethod::LoopyBeliefPropagation,
                max_iterations: 1000,
                convergence_threshold: 1e-6,
            }
        );
        
        assert!(result.converged, "Failed to converge on large network");
        assert!(result.bethel_bounds.tightness() < 0.1,
            "Approximation bounds too loose");
    }
}
```

---

## Key References

### Scalability Patterns

1. **Dean, J., & Ghemawat, S. (2008).** "MapReduce: Simplified Data Processing on Large Clusters." *Communications of the ACM,* 51(1), 107-113.
   - Distributed processing patterns applicable to inference

2. **Malewicz, G., et al. (2010).** "Pregel: A System for Large-Scale Graph Processing." *SIGMOD 2010.*
   - Graph partitioning and message passing at scale

3. **Gonzalez, J. E., et al. (2012).** "PowerGraph: Distributed Graph-Parallel Computation on Natural Graphs." *OSDI 2012.*
   - Handling power-law graphs in distributed systems

4. **Chen, R., et al. (2015).** "PowerLyra: Differentiated Graph Computation and Partitioning on Skewed Graphs." *EuroSys 2015.*
   - Partitioning strategies for heterogeneous graphs

### Large-Scale Graph Processing

5. **Kyrola, A., Blelloch, G., & Guestrin, C. (2012).** "GraphChi: Large-Scale Graph Computation on Just a PC." *OSDI 2012.*
   - Out-of-core graph processing techniques

6. **Zhu, X., et al. (2016).** "Gemini: A Computation-Centric Distributed Graph Processing System." *OSDI 2016.*
   - Compute-optimized distributed graph processing

7. **Shun, J., & Blelloch, G. E. (2013).** "Ligra: A Lightweight Graph Processing Framework for Shared Memory." *PPoPP 2013.*
   - Shared-memory parallel graph algorithms

### Probabilistic Inference at Scale

8. **Gonzalez, J., et al. (2011).** "Parallel Gibbs Sampling: From Colored Fields to Thin Junction Trees." *AISTATS 2011.*
   - Parallel MCMC methods

9. **Low, Y., et al. (2012).** "Distributed GraphLab: A Framework for Machine Learning and Data Mining in the Cloud." *VLDB 2012.*
   - Distributed factor graph computation

10. **Hoffman, M. D., Blei, D. M., Wang, C., & Paisley, J. (2013).** "Stochastic Variational Inference." *JMLR,* 14(1), 1303-1347.
    - SVI for massive datasets

### Treewidth and Complexity

11. **Bodlaender, H. L., & Koster, A. M. (2010).** "Treewidth Computations I. Upper Bounds." *Information and Computation,* 208(3), 259-275.
    - Treewidth estimation algorithms

12. **Gogate, V., & Dechter, R. (2004).** "A Complete Anytime Algorithm for Treewidth." *UAI 2004.*
    - Anytime treewidth bounds

13. **Darwiche, A. (2009).** *Modeling and Reasoning with Bayesian Networks.* Cambridge University Press.
    - Treewidth and inference complexity

### Sparse Matrix Methods

14. **Saad, Y. (2003).** *Iterative Methods for Sparse Linear Systems* (2nd ed.). SIAM.
    - Sparse matrix formats and algorithms

15. **Williams, S., Oliker, L., Vuduc, R., Shalf, J., Yelick, K., & Demmel, J. (2007).** "Optimization of Sparse Matrix-Vector Multiplication on Emerging Multicore Platforms." *SC 2007.*
    - High-performance sparse operations

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial draft |
| 1.0 | 2026-03-03 | Wasswa Lutufi Sebbanja | Complete scalability design document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
