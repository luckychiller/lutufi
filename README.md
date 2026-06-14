# Lutufi

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Python Versions](https://img.shields.io/badge/python-3.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12-blue.svg)](pyproject.toml)
[![Build Status](https://img.shields.io/badge/build-pre--alpha-yellow.svg)](docs/DEVELOPMENT_ROADMAP.md)
[![Rust](https://img.shields.io/badge/rust-%E2%9C%93-orange.svg)](Cargo.toml)

> **High-Performance Probabilistic Inference Engine for Large-Scale Network Analysis**

---

## 📄 Academic & Research Context

For researchers and potential collaborators, please see the following core documents:
- [**Two-Page Technical Summary**](docs/outreach/TECHNICAL_SUMMARY.md) — Architectural decisions, benchmarks, and gap analysis.
- [**Personal Research Statement**](docs/outreach/RESEARCH_STATEMENT.md) — Scientific motivation and future research directions.

---

## Table of Contents

- [What Lutufi Is](#what-lutufi-is)
- [Core Capabilities](#core-capabilities)
- [Current Status](#current-status)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

---

## What Lutufi Is

Lutufi is a high-performance probabilistic inference engine designed to bridge the gap between **network science** (structural analysis) and **probabilistic graphical models** (PGMs). Built in Rust for memory safety and zero-cost abstractions, it scales to networks with hundreds of thousands of nodes and millions of edges, providing researchers with the tools to reason about uncertainty in complex social and economic systems.

## Core Capabilities

- **Log-Space Inference:** Numerical stability for high-dimensional models and long causal chains.
- **Causal Reasoning:** Native implementation of Pearl's **do-calculus** and the **ID Algorithm** for causal effect identification.
- **Scalability:** Optimized sparse matrix operations and automatic algorithm selection (Exact vs. Approximate).
- **Dynamic Networks:** Full support for Dynamic Bayesian Networks (DBNs) with temporal filtering and smoothing.
- **Dark Networks:** Bayesian reconstruction of hidden structures from unreliable observational data.
- **Multilayer Support:** Inference over networks with multiple types of edges (e.g., social + financial).

## Current Status

**⚠️ Pre-Alpha: Under Active Development**

Lutufi is currently in pre-alpha status with APIs subject to significant change. The library is **not suitable for production use** at this stage. We are actively building the core inference engine, establishing the Python API surface, and implementing foundational algorithms. Breaking changes may occur between releases as we refine the architecture based on early feedback and performance benchmarks. Our target timeline expects a beta release in Q3 2026, at which point the API will stabilize and production use will be supported. We welcome early adopters and contributors who want to help shape the future of probabilistic network analysis.

---

## Installation

### Future (PyPI - Coming Soon)

Once published to PyPI, installation will be as simple as:

```bash
pip install lutufi
```

### Current: Build from Source

To use Lutufi today, you must build it from source. This requires:

- **Rust toolchain** (1.70 or later) - [Install Rust](https://rustup.rs/)
- **Python** 3.8 or later (3.11+ recommended)
- **maturin** for building Python bindings

#### Installation Steps

1. **Clone the repository:**

```bash
git clone https://github.com/luckychiller/lutufi.git
cd lutufi
```

2. **Create a Python virtual environment:**

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

3. **Install maturin and build:**

```bash
pip install maturin
maturin develop --release
```

4. **Verify installation:**

```bash
python -c "import lutufi; print(lutufi.__version__)"
```

For development with all optional dependencies:

```bash
pip install -e ".[dev,visualization,docs]"
```

---

## Quick Start

Lutufi uses a builder pattern for efficient network construction and parameter setting. Below is a working example based on a simple epidemiology model:

```python
import lutufi
from lutufi.inference import InferenceEngine

# 1. Build the network using the builder pattern
builder = (lutufi.BayesianNetwork.builder()
    .add_variable("Fever", domain=["high", "none"])
    .add_variable("Cough", domain=["mild", "none"])
    .add_variable("Flu", domain=["yes", "no"])
    .add_edge("Flu", "Fever")
    .add_edge("Flu", "Cough"))

# 2. Set Conditional Probability Tables (CPTs)
# P(Flu=yes) = 0.01
builder.set_cpd("Flu", [0.01, 0.99])

# P(Fever | Flu)
# Row 0: high, Row 1: none
# Columns: Flu=yes, Flu=no
builder.set_cpd("Fever", [
    [0.9, 0.05], # Fever=high
    [0.1, 0.95]  # Fever=none
])

# P(Cough | Flu)
# Row 0: mild, Row 1: none
# Columns: Flu=yes, Flu=no
builder.set_cpd("Cough", [
    [0.8, 0.1],  # Cough=mild
    [0.2, 0.9]   # Cough=none
])

network = builder.build()

# 3. Run inference
engine = InferenceEngine(network)

# Query: Probability of Flu given Fever=high
result = engine.query(["Flu"], evidence={"Fever": "high"})
print(result.to_dict()["distributions"]["Flu"])
# Output: {'yes': 0.1538, 'no': 0.8462}
```

### Advanced: Temporal Network Analysis

```python
import lutufi

# Analyze evolving networks with temporal dynamics
temporal_net = lutufi.TemporalBayesianNetwork(time_steps=10)

# Add time-slice structure
temporal_net.add_intra_slice_edges([
    ("MarketSentiment_t", "StockPrice_t"),
    ("NewsVolume_t", "MarketSentiment_t"),
])

# Add inter-slice dependencies (temporal evolution)
temporal_net.add_inter_slice_edges([
    ("MarketSentiment_t", "MarketSentiment_t+1"),
    ("StockPrice_t", "StockPrice_t+1"),
])

# Perform filtering: infer current state from history
filtering = lutufi.TemporalInference(temporal_net)
current_beliefs = filtering.filter(
    observations=time_series_data,
    method="particle_filter",
    n_particles=10000
)
```

---

## Documentation

Comprehensive documentation is available in the [`docs/`](docs/) directory:

| Document | Description |
|----------|-------------|
| [`docs/README.md`](docs/README.md) | Full documentation index and getting started guide |
| [`docs/design/ARCHITECTURE.md`](docs/design/ARCHITECTURE.md) | System architecture and design principles |
| [`docs/design/API_DESIGN.md`](docs/design/API_DESIGN.md) | Detailed API specifications |
| [`docs/DEVELOPMENT_ROADMAP.md`](docs/DEVELOPMENT_ROADMAP.md) | Project roadmap and milestones |
| [`docs/foundations/`](docs/foundations/) | Theoretical foundations and algorithms |

### Quick Links

- **User Guide**: [`docs/README.md`](docs/README.md)
- **API Reference**: Coming soon
- **Examples**: [`examples/`](examples/)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)

---

## Contributing

We welcome contributions from the community! Please read our [Contributing Guidelines](docs/governance/CONTRIBUTING.md) before submitting pull requests.

Key areas where help is needed:
- Algorithm implementations (inference, learning, sampling)
- Python API design and documentation
- Performance benchmarking and optimization
- Domain-specific examples and tutorials

### Development Setup

```bash
# Clone and setup
git clone https://github.com/luckychiller/lutufi.git
cd lutufi

# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest tests/

# Run Rust tests
cargo test

# Build documentation
mkdocs serve  # or: cd docs && make html
```

---

## License

Lutufi is licensed under the **Apache License 2.0**. See [LICENSE](LICENSE) for details.

```
Copyright 2026 Lutufi Contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

---

<div align="center">

**[⬆ Back to Top](#lutufi)**

*Built with Rust 🦀 and Python 🐍 for high-performance probabilistic inference*

</div>
