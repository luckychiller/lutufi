# Lutufi: High-Performance Probabilistic Inference over Large-Scale Networks

## 1. Overview and Scientific Gap
Lutufi is a high-performance Rust library with Python bindings designed for probabilistic inference and causal reasoning over large social and economic networks.

**The Gap:** Existing tools (e.g., pgmpy, bnlearn) excel at traditional Bayesian network tasks on small-to-medium scales but struggle with the structural complexity and computational demands of real-world networks (thousands to millions of nodes). Furthermore, network analysis libraries (e.g., NetworkX, graph-tool) focus on structural metrics but lack native support for probabilistic flow and causal semantics. Lutufi bridges this gap by unifying network science and probabilistic graphical models (PGMs) in a single, high-performance architecture.

## 2. Key Architectural Decisions
Lutufi's design is driven by three foundational principles required for research-grade network inference:

### A. Log-Space Arithmetic & Numerical Stability
Traditional inference often suffers from underflow when dealing with long causal chains or high-dimensional covariate adjustments. Lutufi performs all internal factor operations in log-space, ensuring numerical stability even in networks with millions of parameters where joint probabilities would otherwise vanish.

### B. Automatic Algorithm Selection & Optimization
Lutufi evaluates network topology (e.g., treewidth, sparsity, presence of cycles) to automatically select the optimal inference engine:
- **Exact Inference:** Variable Elimination and Junction Tree algorithms for sparse or small-treewidth networks.
- **Approximate Inference:** Loopy Belief Propagation and MCMC (Gibbs Sampling) for dense, cyclic, or extremely large networks.
- **Incremental Updates:** Optimized for dynamic networks where only a subset of nodes or edges changes.

### C. Causal Semantics & Identifiability Enforcement
Unlike pure PGM tools, Lutufi natively implements the **ID Algorithm** and **do-calculus** as first-class citizens. It enforces causal consistency, automatically detecting when a causal effect is non-identifiable and providing the symbolic identification formula (e.g., front-door/back-door adjustment) when possible.

## 3. Benchmarks & Performance
Lutufi is engineered for scale using Rust's memory safety and zero-cost abstractions, combined with sparse matrix representations.

| Network Size | Lutufi (Inference) | pgmpy (Inference) | Improvement |
|--------------|-------------------|-------------------|-------------|
| 1,000 nodes  | < 0.1s            | ~2s               | 20x         |
| 10,000 nodes | ~2s               | ~45s              | 22.5x       |
| 100,000 nodes| ~15s              | Timeout/OOM       | ∞           |
| 1,000,000 edges| ~55s            | N/A               | Scalable    |

*Note: Benchmarks performed on a standard 16GB RAM machine using sparse Variable Elimination.*

## 4. Complete Capability List
- **Exact & Approximate Inference:** Sum-Product and Max-Product algorithms.
- **Causal Reasoning:** Full do-calculus, ID algorithm, and counterfactual identification.
- **Dynamic Bayesian Networks (DBNs):** Temporal inference, filtering, and smoothing for evolving networks.
- **Missing Data Handling:** Robust inference over incomplete networks without biased imputation.
- **Dark/Covert Networks:** Bayesian reconstruction of hidden network structures from unreliable observational data.
- **Multilayer Support:** Handling overlapping relationship types (e.g., financial + social + political).

## 5. Repository and Access
- **Core:** Compiled Rust (Zero-cost abstraction, high concurrency).
- **Interface:** Python (Full integration with pandas, NetworkX, and Jupyter).
- **Access:** Private repository (Access available for research collaboration upon request).
