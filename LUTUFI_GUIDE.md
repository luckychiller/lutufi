# Lutufi: Unified Probabilistic Network Analysis

Lutufi is a high-performance library that unifies **Probabilistic Graphical Models (PGMs)** with **Social and Economic Network Analysis**. It provides a single framework where network structure and probabilistic reasoning coexist natively.

This guide provides a comprehensive overview of Lutufi's current capabilities (Phase 1) and its future roadmap.

---

## 1. Core Mission

Lutufi addresses the gap between structural network analysis (e.g., NetworkX) and probabilistic inference (e.g., pgmpy). In Lutufi:
- A **Node** is both a social actor and a random variable.
- An **Edge** is both a relational tie and a probabilistic dependency.

This dual representation enables rigorous reasoning under uncertainty over the relational structures that shape societies and economies.

---

## 2. Installation & Setup

Lutufi's core is implemented in **Rust** for performance and safety, with **Python** as the primary user-facing interface.

### Prerequisites
- Python 3.9+
- Rust toolchain (cargo)
- `maturin` (for building Python bindings)

### Building from Source
Currently, Lutufi is in active development (Phase 1). To install the development version:

```bash
git clone https://github.com/lutufi/lutufi.git
cd lutufi
maturin develop
```

---

## 3. Core Probabilistic Models

Lutufi supports several types of graphical models.

### Bayesian Networks (Directed)
Represent directed acyclic dependencies. Each node has a Conditional Probability Distribution (CPD).

```python
from lutufi import BayesianNetwork

# Using the fluent builder API
model = (BayesianNetwork.builder()
    .add_variable("Cloudy", domain=["False", "True"])
    .add_variable("Rain", domain=["False", "True"])
    .add_variable("Sprinkler", domain=["False", "True"])
    .add_edge("Cloudy", "Rain")
    .add_edge("Cloudy", "Sprinkler")
    .set_cpd("Cloudy", [0.5, 0.5])
    .set_cpd("Rain", [[0.8, 0.2], [0.2, 0.8]]) # P(Rain | Cloudy)
    .set_cpd("Sprinkler", [[0.5, 0.9], [0.5, 0.1]]) # P(Sprinkler | Cloudy)
    .build())
```

### Markov Random Fields (Undirected)
Represent undirected dependencies via factors over cliques.

```python
from lutufi.models import MarkovRandomField

mrf = MarkovRandomField()
mrf.add_variable("A", ["0", "1"]).add_variable("B", ["0", "1"])
mrf.add_edge("A", "B")
```

### Dynamic Bayesian Networks (Temporal)
Represent models that evolve over time (2-slice temporal Bayes nets).

```python
from lutufi.models import DynamicBayesianNetwork

dbn = DynamicBayesianNetwork()
dbn.add_variable("Position", ["Left", "Right"])
dbn.add_interslice_edge("Position", "Position") # Temporal dependency
```

---

## 4. Graph & Structural Analysis

Lutufi models provide native graph-theoretic operations.

### Basic Properties
```python
print(model.nodes())
print(model.edges())
print(model.topological_order())
print(model.markov_blanket("Rain"))
```

### d-Separation
Test conditional independence using the graph structure.
```python
# Is Rain independent of Sprinkler given Cloudy?
is_indep = model.d_separated("Rain", "Sprinkler", given=["Cloudy"])
```

### NetworkX Integration
Seamlessly convert to and from NetworkX for complex structural analysis.
```python
import networkx as nx

# Export to NetworkX
G = model.to_networkx()
centrality = nx.betweenness_centrality(G)

# Import from NetworkX
model_new = BayesianNetwork.from_networkx(G)
```

---

## 5. Probabilistic Inference

Lutufi provides several engines for querying your models.

### Exact Inference
Suitable for small to medium-sized networks.

- **Variable Elimination**: Standard exact inference.
- **Junction Tree**: Pre-compiles the graph into a tree for faster repeated queries.

```python
from lutufi.inference import InferenceEngine, JunctionTreeEngine

# 1. Initialize engine
engine = InferenceEngine(model, algorithm="variable_elimination")

# 2. Set evidence (observations)
engine.set_evidence("Rain", "True")

# 3. Query marginals
result = engine.query(variables=["Sprinkler"])
print(result["Sprinkler"]) # Probability distribution

# 4. Most Probable Explanation (MPE)
# Finds the most likely state for all unobserved variables
mpe = engine.mpe_query()
print(mpe.most_probable())
```

### Approximate Inference (In Development)
For large-scale networks where exact inference is intractable:
- **Loopy Belief Propagation**
- **Gibbs Sampling (MCMC)**
- **Variational Inference**

---

## 6. Causal Inference (do-calculus)

Lutufi explicitly distinguishes between statistical associations and causal mechanisms.

### Marking Causal Models
Before performing causal queries, you must explicitly mark your model as causal, signifying that edges represent direct mechanisms.

```python
model.mark_as_causal()
```

### The do-operator (Planned)
Perform interventional queries to estimate causal effects.
```python
# Estimate P(Outcome | do(Treatment=1))
# result = model.do("Treatment", 1).query("Outcome") # Planned API
```

---

## 7. Learning from Data (In Development)

Lutufi aims to support learning both parameters and structure from data.

### Parameter Learning
Learn CPTs from observed data using Maximum Likelihood (MLE) or Bayesian Estimation.

```python
from lutufi.learning import ParameterLearningEngine
import pandas as pd

data = pd.read_csv("observations.csv")
engine = ParameterLearningEngine(model, method="mle")
engine.fit(data)
```

### Structure Learning
Discover the dependency graph from raw data using algorithms like Hill Climbing or PC.

---

## 8. Data Ecosystem Integration

Lutufi is designed to play well with the scientific Python stack:
- **NumPy**: Zero-copy data exchange for probability tables.
- **Pandas**: Import/export evidence and datasets.
- **NetworkX**: Full structural interoperability.
- **Scikit-learn**: Future support for `fit`/`predict` Estimator API.

---

## 9. Future Roadmap

| Phase | Focus | Status |
|---|---|---|
| **Phase 1** | Core representation, exact inference, basic Python bindings | **Active** |
| **Phase 2** | Extended inference, Causal do-calculus, Parameter learning | Planned |
| **Phase 3** | Dynamic BNs, Missing data (EM), 30+ domain examples | Planned |
| **Phase 4** | Large-scale optimization, Serialization, 1.0 Release | Planned |

---

## 10. Contact & Community

- **Author**: Wasswa Lutufi Sebbanja
- **Repository**: [github.com/lutufi/lutufi](https://github.com/lutufi/lutufi)
- **License**: Apache 2.0

*"Lutufi enables the ability to reason rigorously under uncertainty over the relational structures that shape societies and economies."*
