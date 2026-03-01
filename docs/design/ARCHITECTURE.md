# System Architecture: Lutufi

## Design Philosophy
Lutufi's architecture is built on the principle of **"Anticipated Integration."** Features like simulation, visualization, and ML/AI are not bolted on as afterthoughts but are designed into the core data structures and flow from day one.

## Core Components

### 1. Representation Layer (`lutufi.core`)
The foundation of the library. It handles the dual nature of Lutufi models:
- **Graph Engine:** Manages the structural topology (nodes and edges). It supports sparse representations and multi-layer (multiplex) connections.
- **Probabilistic Registry:** Manages the conditional probability tables (CPTs) or factor potentials associated with the graph.

### 2. Inference Engine (`lutufi.inference`)
The computational heart of the library. It separates the **Solver** from the **Model**.
- **Exact Solvers:** Implementation of Variable Elimination and Junction Tree algorithms.
- **Approximate Solvers:** Loopy Belief Propagation, Gibbs Sampling, and Variational Inference.
- **Numerical Guard:** A middleware layer that ensures log-space arithmetic and handles floating-point stability.

### 3. Dynamics & Simulation (`lutufi.simulation`)
Designed for time-stepping and state evolution.
- **Time-Stepping:** Allows the network to evolve over $T$ steps, where node states at $t$ influence node states at $t+1$.
- **Structural Dynamics:** APIs for adding/removing edges and nodes during a simulation run.

### 4. Interface & Bindings (`lutufi.api`)
- **Pythonic Interface:** Designed to feel native to users of `pandas`, `NetworkX`, and `scikit-learn`.
- **FFI (Foreign Function Interface):** A clean C/C++ core with bindings for R and potentially Julia in later phases.

## Data Flow
1. **Input:** Data enters via CSV, JSON, or native Python structures (DataFrames/Dicts).
2. **Scaffolding:** The Graph Engine builds the structural network.
3. **Parametrization:** The Probabilistic Registry assigns weights or CPTs to the structure.
4. **Query:** The user submits an inference query (e.g., "What is $P(X \mid Y)$?").
5. **Inference:** The solver executes the query across the network.
6. **Output:** Results are returned as probability distributions, compatible with visualization and analysis tools.

## Scalability Strategy
- **Lazy Evaluation:** Computation is deferred until a query is actually made.
- **Chunked Processing:** Large networks are processed in modular chunks to prevent memory overflow.
- **Sparse Storage:** Probabilistic tables with many zero-probability states are stored using coordinate-based sparse formats.
