# Lutufi Testing Guide

> *"The definition of done is not 'I wrote the code.' It is 'the tests pass, the code is committed, and I can explain every line of it to a skeptical senior engineer.'"* — Lutufi Roadmap

This document outlines the multi-layered testing strategy used to ensure Lutufi remains mathematically rigorous and numerically stable as it scales.

---

## 1. Prerequisites

Ensure your environment is fully configured:
- **Rust Toolchain**: `rustc` and `cargo` (latest stable).
- **Python**: 3.11+ with `pytest`, `numpy`, and `pandas`.
- **Maturin**: For building Python bindings (`pip install maturin`).

---

## 2. Rust Core Testing (Mathematical Foundation)

The Rust core contains the fundamental logic. These tests must pass before any Python-level testing begins.

### Unit Tests
Test individual components (`Factor`, `Variable`, `Domain`, `Graph`):
```bash
cargo test --lib core
```
**Key areas to watch:**
- `src/core/factor.rs`: Log-space arithmetic and normalization.
- `src/core/graph.rs`: Cycle detection and topological sorting.

### Integration Tests
Test full inference workflows in Rust:
```bash
cargo test --test inference_unification
```
This verifies that all four engines (Exact, LBP, MCMC, Variational) produce results within expected tolerances on the same network.

### Numerical Stability Tests
Specifically verify that log-sum-exp prevents underflow:
```bash
cargo test --lib core::log_space_tests
```

---

## 3. Python Bindings & Validation

Python tests verify the FFI layer and provide high-level mathematical validation.

### Build First
Always rebuild the bindings before testing Python changes:
```bash
maturin develop
```

### Running Pytest
Run the full suite:
```bash
pytest validation/
```

**Validation Suite Overview:**
- `test_ground_truth.py`: **The most critical test.** Compares Lutufi results against analytical solutions in `fixtures/ground_truth/`.
- `test_asia_construction.py`: Verifies the fluent builder API and NetworkX round-trips.
- `test_variable_elimination.py`: Tests elimination heuristics (Min-Fill, Min-Degree).

---

## 4. Ground Truth Infrastructure

Lutufi uses a "Gold Standard" approach. Analytical solutions for known networks are stored in `fixtures/ground_truth/` (e.g., `asia.json`, `alarm.json`).

**To add a new ground truth case:**
1. Generate/find an analytical solution for a network.
2. Format it as a JSON file in `fixtures/ground_truth/` following the existing schema.
3. The `test_ground_truth.py` runner will automatically pick it up.

---

## 5. Performance Benchmarking

To ensure Phase 10 (Scalability) stays on track, we track performance baselines for every algorithm.

### Running Baselines
Execute the benchmark script to generate a new report:
```bash
cargo run --release --bin benchmark_baseline
```
This produces a file like `tests/benchmarks/baseline_2026-04-21.json`.

**Performance Invariants:**
- **Exact (Junction Tree)**: Should be fastest for treewidth < 15.
- **LBP**: Should converge in < 100ms for sparse 50-node networks.
- **MCMC**: Accuracy should scale linearly with `n_samples`.

---

## 6. Convergence & Honesty Monitoring

When testing approximate inference, check the `diagnostics` attribute:

```python
result = model.query(["A"], algorithm="lbp")
print(result.diagnostics["converged"])  # Must be True for valid LBP results
print(result.diagnostics["residual"])   # Should be below 1e-6
```

---

## 7. CI/CD Pipeline

Every PR is automatically checked via GitHub Actions:
1. `cargo fmt --check` & `cargo clippy` (Linting)
2. `cargo test` (Rust Core)
3. `maturin build` (FFI)
4. `pytest` (Python Validation)

**A task is only complete when all layers are green.**
