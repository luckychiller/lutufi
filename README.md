# Lutufi

[![PyPI](https://img.shields.io/pypi/v/lutufi.svg)](https://pypi.org/project/lutufi/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://github.com/luckychiller/lutufi/blob/main/LICENSE)
[![Python Versions](https://img.shields.io/badge/python-3.9%20%7C%203.10%20%7C%203.11%20%7C%203.12%20%7C%203.13-blue.svg)](https://github.com/luckychiller/lutufi/blob/main/pyproject.toml)
[![Rust](https://img.shields.io/badge/rust-core-orange.svg)](https://github.com/luckychiller/lutufi/blob/main/Cargo.toml)

> **High-Performance Probabilistic Inference Engine for Large-Scale Network Analysis**

Lutufi bridges the gap between **network science** (structural analysis) and
**probabilistic graphical models**. The core engine is written in Rust for
memory safety and speed, and is exposed to Python through a clean,
Pythonic API that plays well with numpy, pandas, and NetworkX.

---

## Core Capabilities

- **Exact inference** — variable elimination and junction tree propagation,
  with automatic algorithm selection based on network treewidth.
- **Approximate inference** — loopy belief propagation, Gibbs sampling (MCMC),
  and mean-field variational inference for networks where exact inference is
  intractable.
- **Log-space computation** — numerically stable inference for
  high-dimensional models and long causal chains.
- **Learning** — parameter estimation (MLE and Bayesian) and structure
  learning directly from pandas DataFrames.
- **Structural / causal analysis** — d-separation queries, Markov blankets,
  and explicit causal-model semantics (`mark_as_causal()`); Pearl's
  do-calculus and the ID algorithm are implemented in the Rust core.
- **Dynamic Bayesian networks** — two-slice temporal models with intra-slice
  and inter-slice edges.
- **Interoperability** — import/export of BIF, XMLBIF, UAI, and Lutufi's own
  LMF format; NetworkX round-trips; GraphML/GEXF/GML/Pajek I/O.

## Installation

```bash
pip install lutufi
```

Prebuilt wheels are published for Linux, macOS, and Windows (x86_64 and
arm64), so no Rust toolchain is required.

<details>
<summary><strong>Building from source</strong></summary>

Requires the [Rust toolchain](https://rustup.rs/) (1.70+), Python 3.9+, and
[maturin](https://github.com/PyO3/maturin):

```bash
git clone https://github.com/luckychiller/lutufi.git
cd lutufi
python -m venv venv
source venv/bin/activate   # On Windows: venv\Scripts\activate
pip install maturin
maturin develop --release
python -c "import lutufi; print(lutufi.__version__)"
```

See [INSTALL.md](https://github.com/luckychiller/lutufi/blob/main/INSTALL.md)
for platform-specific instructions.

</details>

## Quick Start

Networks are constructed with a fluent builder. CPT values are given with
**one row per child state and one column per parent configuration** (each
column must sum to 1):

```python
import lutufi
from lutufi.inference import InferenceEngine

# 1. Build the network
builder = (lutufi.BayesianNetwork.builder()
    .add_variable("Fever", domain=["high", "none"])
    .add_variable("Cough", domain=["mild", "none"])
    .add_variable("Flu", domain=["yes", "no"])
    .add_edge("Flu", "Fever")
    .add_edge("Flu", "Cough"))

# 2. Set conditional probability tables
# P(Flu): [yes, no]
builder.set_cpd("Flu", [0.01, 0.99])

# P(Fever | Flu) — columns: Flu=yes, Flu=no
builder.set_cpd("Fever", [
    [0.9, 0.05],  # Fever=high
    [0.1, 0.95],  # Fever=none
])

# P(Cough | Flu) — columns: Flu=yes, Flu=no
builder.set_cpd("Cough", [
    [0.8, 0.1],   # Cough=mild
    [0.2, 0.9],   # Cough=none
])

network = builder.build()

# 3. Run inference
engine = InferenceEngine(network)
result = engine.query(["Flu"], evidence={"Fever": "high"})

print(result.to_dict()["distributions"]["Flu"])
# [0.1538..., 0.8461...]  — order matches network.get_states("Flu"): ['yes', 'no']
print(result.most_probable())
# {'Flu': 'no'}
```

### Learning from data

```python
import pandas as pd
import lutufi
from lutufi.inference import InferenceEngine

data = pd.DataFrame({
    "Rain":      ["yes", "no", "yes", "no", "no", "yes", "no", "no"],
    "Sprinkler": ["off", "on", "off", "on", "on", "off", "on", "off"],
    "WetGrass":  ["wet", "wet", "wet", "dry", "wet", "wet", "dry", "dry"],
})

# Learn CPTs from data given a known structure
bn = lutufi.BayesianNetwork.from_dataframe(
    data,
    structure=[("Rain", "WetGrass"), ("Sprinkler", "WetGrass")],
)

result = InferenceEngine(bn).query(["Rain"], evidence={"WetGrass": "wet"})
```

### Dynamic Bayesian networks

```python
import lutufi

dbn = (lutufi.DynamicBayesianNetwork(name="market")
    .add_variable("Sentiment", ["bullish", "bearish"])
    .add_variable("Price", ["up", "down"])
    # Within a time slice: sentiment drives price
    .add_intraslice_edge("Sentiment", "Price")
    # Across time slices: sentiment persists
    .add_interslice_edge("Sentiment", "Sentiment"))
```

More runnable, end-to-end examples (inference algorithms, NetworkX and file
I/O, causal reasoning, MRFs, error handling, visualization, and domain case
studies) live in
[`examples/`](https://github.com/luckychiller/lutufi/tree/main/examples).

## Documentation

| Document | Description |
|----------|-------------|
| [`docs/README.md`](https://github.com/luckychiller/lutufi/blob/main/docs/README.md) | Documentation index and getting-started guide |
| [`docs/design/ARCHITECTURE.md`](https://github.com/luckychiller/lutufi/blob/main/docs/design/ARCHITECTURE.md) | System architecture and design principles |
| [`docs/design/API_DESIGN.md`](https://github.com/luckychiller/lutufi/blob/main/docs/design/API_DESIGN.md) | API specifications |
| [`docs/foundations/`](https://github.com/luckychiller/lutufi/tree/main/docs/foundations) | Theoretical foundations and algorithms |
| [`examples/`](https://github.com/luckychiller/lutufi/tree/main/examples) | Runnable API walkthroughs and domain examples |
| [`CHANGELOG.md`](https://github.com/luckychiller/lutufi/blob/main/CHANGELOG.md) | Release history |

## Contributing

Contributions are welcome! Please read the
[Contributing Guidelines](https://github.com/luckychiller/lutufi/blob/main/CONTRIBUTING.md)
before submitting pull requests.

### Development setup

```bash
git clone https://github.com/luckychiller/lutufi.git
cd lutufi

# Build the extension and install dev dependencies
pip install -e ".[dev]"
maturin develop --release

# Rust tests
cargo test --features python

# Python validation suite
pytest validation/
```

## License

Lutufi is licensed under the **Apache License 2.0**. See
[LICENSE](https://github.com/luckychiller/lutufi/blob/main/LICENSE) for details.

---

<div align="center">

*Built with Rust 🦀 and Python 🐍 for high-performance probabilistic inference*

</div>
