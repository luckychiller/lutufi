# Lutufi

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Python Versions](https://img.shields.io/badge/python-3.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12-blue.svg)](pyproject.toml)
[![Build Status](https://img.shields.io/badge/build-pre--alpha-yellow.svg)](docs/DEVELOPMENT_ROADMAP.md)
[![Rust](https://img.shields.io/badge/rust-%E2%9C%93-orange.svg)](Cargo.toml)

> **High-Performance Probabilistic Inference Engine for Network Analysis**

---

## Table of Contents

- [What Lutufi Is](#what-lutufi-is)
- [What Problem It Solves](#what-problem-it-solves)
- [Current Status](#current-status)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

---

## What Lutufi Is

Lutufi is a high-performance probabilistic inference engine specifically designed for network analysis. It implements state-of-the-art algorithms for Bayesian networks, Markov random fields, and related graphical models. Built primarily in Rust for maximum performance and memory safety, Lutufi provides Python bindings that make it accessible to data scientists and researchers without sacrificing speed. The library targets critical domains including epidemiology (disease spread modeling), finance (risk contagion analysis), social networks (influence propagation), and intelligence analysis (threat network detection), where understanding complex probabilistic relationships in networked data is essential.

## What Problem It Solves

Existing probabilistic inference tools such as pgmpy and bnlearn, while functional, struggle with the computational demands of large-scale networks, often becoming prohibitively slow when dealing with thousands of nodes or complex dependency structures. Current solutions also lack comprehensive support for real-world challenges: proper handling of missing data without biased imputation, modeling temporal dynamics in evolving networks, and analyzing multilayer networks where different types of relationships interact. No open-source tool currently combines all these advanced features with the high performance necessary for production-scale analysis. Lutufi fills this critical gap by leveraging GPU acceleration, sparse matrix operations, and modern inference algorithms to deliver enterprise-grade probabilistic network analysis capabilities.

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
git clone https://github.com/lutufi/lutufi.git
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

Below is an aspirational example demonstrating the intended Lutufi API. This code represents the target functionality and will run once the library reaches feature completeness:

```python
import lutufi
import pandas as pd

# Create a Bayesian network for disease outbreak analysis
# Define nodes: Symptoms, Diseases, Risk Factors
network = lutufi.BayesianNetwork()

# Add nodes with their domains
network.add_node("Fever", domain=["low", "high", "none"])
network.add_node("Cough", domain=["mild", "severe", "none"])
network.add_node("Disease", domain=["flu", "cold", "none"])
network.add_node("AgeGroup", domain=["child", "adult", "elderly"])

# Define the network structure (directed edges)
network.add_edges([
    ("AgeGroup", "Disease"),
    ("Disease", "Fever"),
    ("Disease", "Cough"),
])

# Learn parameters from observational data
data = pd.read_csv("patient_data.csv")
network.fit(data, method="maximum_likelihood")

# Run inference using Belief Propagation
inference = lutufi.BeliefPropagation(network)

# Query: What's the probability of flu given high fever?
result = inference.query(
    variables=["Disease"],
    evidence={"Fever": "high", "AgeGroup": "elderly"}
)
print(result)  # Probability distribution over diseases

# Predict most likely disease
prediction = inference.map_query(
    variables=["Disease"],
    evidence={"Fever": "high", "Cough": "severe"}
)
print(f"Most likely diagnosis: {prediction['Disease']}")
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
git clone https://github.com/lutufi/lutufi.git
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
