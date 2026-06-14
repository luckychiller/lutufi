# Testing Philosophy Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Testing Philosophy](#testing-philosophy)
3. [Levels of Testing](#levels-of-testing)
4. [Testing Probabilistic Code](#testing-probabilistic-code)
5. [Unit Testing](#unit-testing)
6. [Integration Testing](#integration-testing)
7. [Correctness Testing](#correctness-testing)
8. [Numerical Precision Testing](#numerical-precision-testing)
9. [Performance Regression Testing](#performance-regression-testing)
10. [Property-Based Testing](#property-based-testing)
11. [Fuzz Testing](#fuzz-testing)
12. [Test Data Management](#test-data-management)
13. [Continuous Integration](#continuous-integration)
14. [Test Coverage](#test-coverage)
15. [Debugging Test Failures](#debugging-test-failures)
16. [Testing Documentation](#testing-documentation)
17. [How Lutufi Tests](#how-lutufi-tests)
18. [Key References](#key-references)

---

## Executive Summary

This document presents the comprehensive testing philosophy for Lutufi, a library that unifies Bayesian networks with social and economic network analysis, authored by Wasswa Lutufi Sebbanja and licensed under Apache 2.0. Testing scientific software presents unique challenges: algorithms must be mathematically correct, numerically stable, and statistically valid, while maintaining acceptable performance on large-scale problems. Lutufi's testing strategy addresses these challenges through a multi-layered approach that combines traditional software testing with domain-specific validation techniques.

The core philosophy is that correctness is paramount. In scientific computing, a fast but wrong answer is worse than no answer at all. Lutufi's testing strategy prioritizes correctness validation through multiple mechanisms: analytical solution comparison for tractable cases, statistical tests for probabilistic algorithms, convergence verification for iterative methods, and property-based testing for invariant preservation. This is complemented by comprehensive unit and integration testing to ensure robustness against edge cases and real-world usage patterns.

A distinctive aspect of Lutufi's testing approach is the handling of probabilistic code. Unlike deterministic algorithms where outputs can be directly compared to expected values, probabilistic algorithms produce distributions or samples that must be validated statistically. The testing strategy includes seed management for reproducibility, statistical hypothesis testing for correctness verification, and convergence analysis for approximate inference methods.

The document covers all aspects of testing: unit tests for individual components, integration tests for workflows, property-based tests for invariant checking, fuzz tests for crash discovery, performance tests for regression detection, and continuous integration for automated validation across platforms and configurations.

---

## Testing Philosophy

### Why Testing Matters for Scientific Software

Scientific software occupies a unique position in the software landscape. Unlike business applications where approximate correctness may be acceptable, scientific software underpins research conclusions, policy decisions, and operational systems where errors can have significant consequences:

**Research Validity:** When researchers use Lutufi to analyze social networks, financial contagion, or epidemiological spread, the correctness of results directly affects the validity of their research conclusions. A bug in belief propagation could lead to incorrect conditional independence conclusions, undermining entire research programs.

**Operational Impact:** In production environments—whether detecting financial crime, optimizing supply chains, or analyzing intelligence data—incorrect results can lead to missed threats, suboptimal decisions, or resource misallocation.

**Trust and Reputation:** Scientific software builds trust through demonstrated correctness. Users must have confidence that the library produces accurate results before they will rely on it for critical applications.

**Reproducibility:** Scientific reproducibility requires that software produces the same results given the same inputs. Testing ensures that changes to the codebase don't inadvertently alter computational results.

### Correctness vs Functionality

A fundamental distinction in Lutufi's testing philosophy separates correctness from functionality:

**Correctness:** The algorithm produces mathematically valid results. For a belief propagation implementation, correctness means that computed marginals are consistent with the joint probability distribution defined by the network. Correctness is non-negotiable.

**Functionality:** The software performs its intended operations without crashing. A function that runs without error but produces incorrect results has functionality but lacks correctness. A function that crashes lacks both.

**Testing Emphasis:** Lutufi's testing prioritizes correctness over functionality. It is better for a test to fail (revealing a correctness issue) than to pass despite underlying errors. This leads to extensive use of reference solutions, statistical tests, and analytical comparisons.

### Testing as Specification

Tests serve as executable specifications of expected behavior:

**Documentation:** Well-written tests document the expected behavior of functions, including edge cases, error conditions, and performance characteristics.

**Contract Definition:** Tests define the contract between components. When a test verifies that module A produces output format X when given input Y, it documents the interface contract.

**Regression Prevention:** Tests encode accumulated knowledge about bugs and edge cases, preventing regression when code changes.

### Confidence Through Validation

Scientific software requires multiple validation mechanisms to establish confidence:

**Theoretical Validation:** Does the implementation match the mathematical specification?
**Empirical Validation:** Does the implementation produce expected results on known cases?
**Comparative Validation:** Does the implementation agree with established reference implementations?
**Statistical Validation:** For probabilistic methods, do results satisfy statistical properties?

Lutufi employs all four validation types to establish comprehensive confidence in correctness.

---

## Levels of Testing

### Testing Pyramid for Scientific Software

Lutufi's testing follows a modified pyramid structure, with special emphasis on correctness testing:

```
         /\
        /  \     Fuzz Testing
       /____\    (Edge case discovery)
      /      \
     /        \  Property-Based Testing
    /__________\ (Invariant verification)
   /            \
  /              \ Integration Testing
 /________________\ (Workflow validation)
/                  \
/                    \ Unit Testing
/______________________\ (Component isolation)
/                        \
/                          \ Correctness Testing
/____________________________\ (Reference solutions)
```

### Unit Tests

**Purpose:** Verify individual components in isolation.

**Scope:** Functions, methods, small modules.

**Characteristics:** Fast, isolated, deterministic.

**Examples:**
- Testing that graph node addition correctly updates adjacency lists
- Verifying that probability distributions normalize correctly
- Checking that factor table operations maintain invariants

**Volume:** Thousands of tests covering all public APIs and internal utilities.

### Integration Tests

**Purpose:** Verify that components work together correctly.

**Scope:** Multi-component workflows, end-to-end scenarios.

**Characteristics:** Moderate speed, may access filesystem or network, deterministic.

**Examples:**
- Loading a network from file, running inference, exporting results
- Bayesian network learning pipeline (structure learning → parameter learning → inference)
- Python binding integration (Rust core called from Python)

**Volume:** Hundreds of tests covering major workflows.

### System Tests

**Purpose:** Verify complete system behavior in realistic scenarios.

**Scope:** Full application scenarios, performance benchmarks.

**Characteristics:** Slower, use realistic data, may be non-deterministic (for probabilistic algorithms).

**Examples:**
- Running full analysis on benchmark networks (e.g., financial contagion on real bank network data)
- End-to-end machine learning workflows with scikit-learn integration
- Large-scale stress tests (millions of nodes)

**Volume:** Tens of tests covering major use cases.

### Property-Based Tests

**Purpose:** Verify that invariants hold across random inputs.

**Scope:** Properties that should always be true regardless of input.

**Characteristics:** Generates hundreds of test cases per property, may find edge cases not anticipated by developers.

**Examples:**
- Belief propagation should always produce normalized marginals
- Graph edge addition should always result in symmetric adjacency for undirected graphs
- Sampling should produce values within valid ranges

**Volume:** Properties defined for all major operations, generating thousands of test cases.

### Fuzz Tests

**Purpose:** Discover crashes and panics from unexpected inputs.

**Scope:** Any function accepting external input (files, network data, user input).

**Characteristics:** Non-deterministic, may run for extended periods, focuses on robustness not correctness.

**Examples:**
- Random network file generation to find parsing crashes
- Random graph structures to find algorithmic edge cases
- Random parameters to find numerical instability

**Volume:** Continuous fuzzing campaigns for critical parsers and algorithms.

---

## Testing Probabilistic Code

### The Challenge of Testing Randomized Algorithms

Probabilistic algorithms present unique testing challenges:

**Non-Determinism:** Randomized algorithms produce different outputs on different runs, making simple output comparison impossible.

**Approximation:** Many probabilistic algorithms (MCMC, variational inference) produce approximate results. Testing must verify that approximations are within acceptable bounds, not that they match exact values.

**Convergence:** Iterative probabilistic algorithms converge to correct results. Testing must verify convergence properties, not final results.

**Sample Quality:** For sampling algorithms, tests must verify statistical properties of samples (moments, distributions), not individual samples.

### Statistical Tests for Correctness

Instead of comparing exact outputs, probabilistic algorithms are tested using statistical methods:

**Hypothesis Testing:**
```rust
#[test]
fn gibbs_sampler_produces_correct_marginals() {
    let network = create_test_network();
    let true_marginals = compute_exact_marginals(&network);
    
    // Run Gibbs sampler
    let samples = network.gibbs_sample(n_samples=10000, burn_in=1000);
    let estimated_marginals = compute_marginals_from_samples(&samples);
    
    // Statistical test: are estimated marginals close to true marginals?
    for (node, (true_dist, est_dist)) in true_marginals.iter()
        .zip(estimated_marginals.iter()) 
    {
        // Chi-squared test for categorical distributions
        let chi2 = chi_squared_test(&true_dist, &est_dist);
        assert!(chi2.p_value > 0.05, 
            "Marginals for node {} significantly differ from expected", node);
    }
}
```

**Kolmogorov-Smirnov Test:** For continuous distributions, the KS test compares sample distributions to expected distributions.

**Monte Carlo Confidence Intervals:** Running algorithms multiple times and verifying that results fall within expected confidence intervals.

### Seeding for Determinism

For reproducibility, probabilistic algorithms accept seeds:

```rust
#[test]
fn seeded_sampling_is_deterministic() {
    let network = create_test_network();
    let seed = 42;
    
    let sample1 = network.sample(Seed::from_u64(seed));
    let sample2 = network.sample(Seed::from_u64(seed));
    
    assert_eq!(sample1, sample2, "Seeded sampling should be deterministic");
}
```

**Seed Management:**
- Tests use fixed seeds for reproducibility
- Different test cases use different seeds to avoid correlated "random" sequences
- Seeds are logged on test failure to enable reproduction

### Convergence Testing

For iterative algorithms, tests verify convergence properties:

```rust
#[test]
fn belief_propagation_converges_on_tree() {
    let tree = create_random_tree_network(100);
    let evidence = random_evidence(&tree, 0.1);
    
    let mut bp = BeliefPropagation::new(&tree);
    let convergence_history = bp.run_with_history(&evidence, max_iters=100);
    
    // Verify convergence
    assert!(convergence_history.did_converge(), 
        "BP should converge on tree structures");
    
    // Verify convergence rate
    let final_change = convergence_history.final_change();
    assert!(final_change < 1e-6, 
        "BP should converge to high precision on trees");
    
    // Verify convergence is monotonic (for tree BP)
    for window in convergence_history.changes.windows(2) {
        assert!(window[1] <= window[0] * 1.01, 
            "Convergence should be monotonic (with small tolerance)");
    }
}
```

**Convergence Metrics:**
- L1/L2 norm of belief changes between iterations
- Log-likelihood improvement
- Constraint violation (for constrained optimization)

### Moment Testing

For sampling algorithms, tests verify statistical moments:

```rust
#[test]
fn sampler_produces_correct_moments() {
    let network = create_gaussian_network();
    let samples = network.sample(n=100000);
    
    // First moment (mean)
    let sample_mean = samples.mean();
    let true_mean = network.theoretical_mean();
    let mean_error = (sample_mean - true_mean).abs();
    assert!(mean_error < 0.01, "Sample mean deviates from theoretical");
    
    // Second moment (variance)
    let sample_var = samples.variance();
    let true_var = network.theoretical_variance();
    let var_error = (sample_var - true_var).abs() / true_var;
    assert!(var_error < 0.05, "Sample variance deviates from theoretical");
}
```

---

## Unit Testing

### Coverage Goals

Lutufi aims for comprehensive test coverage:

**Line Coverage:** Target 85%+ line coverage, with critical paths at 95%+.

**Branch Coverage:** Target 80%+ branch coverage to ensure conditional logic is tested.

**Function Coverage:** All public API functions must have tests.

**Edge Case Coverage:** Explicit tests for boundary conditions (empty inputs, single elements, maximum sizes).

### Test Organization

Tests are organized to mirror the source structure:

```
src/
  network/
    mod.rs
    graph.rs
    operations.rs

tests/
  network/
    mod.rs
    graph.rs          # Tests for graph.rs
    operations.rs     # Tests for operations.rs
  unit/               # Additional unit tests
  integration/        # Integration tests
  fixtures/           # Test data and fixtures
```

**Module-Level Tests:** Unit tests in `#[cfg(test)]` modules alongside source code for tight coupling.

**Integration Tests:** Separate test crate for higher-level testing.

### Mocking External Dependencies

External dependencies are mocked to isolate unit tests:

```rust
// Define trait for external dependency
trait RandomGenerator {
    fn next_f64(&mut self) -> f64;
}

// Production implementation
struct PcgGenerator { ... }
impl RandomGenerator for PcgGenerator { ... }

// Mock implementation for testing
struct MockGenerator {
    values: Vec<f64>,
    index: usize,
}
impl RandomGenerator for MockGenerator {
    fn next_f64(&mut self) -> f64 {
        let val = self.values[self.index];
        self.index += 1;
        val
    }
}

#[test]
fn sampling_with_mock_rng() {
    let mock = MockGenerator { 
        values: vec![0.1, 0.5, 0.9] 
    };
    let sampler = Sampler::with_rng(mock);
    
    // Deterministic test with known "random" values
    let sample = sampler.sample();
    assert_eq!(sample, expected_value_for_0_1());
}
```

### Parameterized Tests

Tests are parameterized to cover multiple scenarios:

```rust
#[test]
fn test_with_multiple_network_types() {
    let network_configs = vec![
        ("chain", create_chain_network(10)),
        ("star", create_star_network(10)),
        ("grid", create_grid_network(5, 5)),
        ("random", create_erdos_renyi(20, 0.1)),
    ];
    
    for (name, network) in network_configs {
        let result = run_algorithm(&network);
        assert!(result.is_valid(), "Failed for network type: {}", name);
    }
}
```

**Using test parameterization crates:**
```rust
use test_case::test_case;

#[test_case(10, 0.1 ; "small sparse")]
#[test_case(100, 0.01 ; "large sparse")]
#[test_case(50, 0.5 ; "medium dense")]
fn test_algorithm_scales(n_nodes: usize, edge_prob: f64) {
    let network = create_erdos_renyi(n_nodes, edge_prob);
    let result = run_algorithm(&network);
    assert!(result.is_valid());
}
```

---

## Integration Testing

### End-to-End Workflows

Integration tests verify complete workflows:

```rust
#[test]
fn full_bayesian_network_workflow() {
    // 1. Create network structure
    let mut network = BayesianNetwork::new();
    network.add_variable("A", VariableType::Binary);
    network.add_variable("B", VariableType::Binary);
    network.add_edge("A", "B");
    
    // 2. Learn parameters from data
    let data = load_test_dataset("binary_1000.csv");
    network.fit_parameters(&data, Estimator::MaximumLikelihood);
    
    // 3. Run inference
    let evidence = Evidence::new().set("A", true);
    let result = network.infer(InferenceAlgorithm::BeliefPropagation, &evidence);
    
    // 4. Verify results
    let marginal_b = result.marginal("B");
    assert!(marginal_b.is_valid_probability());
    
    // 5. Export and re-import
    let exported = network.to_json();
    let imported = BayesianNetwork::from_json(&exported).unwrap();
    assert_eq!(network, imported);
}
```

### Realistic Data Usage

Integration tests use realistic data:

**Benchmark Networks:** Standard network datasets (Karate Club, Florentine Families, etc.)

**Synthetic Data:** Generated data with known properties for validation.

**Real Data:** Anonymized real-world datasets (financial networks, social networks) for stress testing.

### Fixture Management

Test fixtures are managed systematically:

```rust
// fixtures/mod.rs
pub struct TestFixtures;

impl TestFixtures {
    pub fn karate_club() -> Network {
        Network::from_edgelist(include_str!("karate.txt"))
    }
    
    pub fn random_tree(n: usize) -> Network {
        NetworkGenerator::tree(n).generate()
    }
    
    pub fn small_bayesian_network() -> BayesianNetwork {
        // Returns a simple BN with known structure for testing
    }
}
```

**Lazy Loading:** Large fixtures are loaded on-demand and cached.

**Fixture Generation:** Fixtures can be regenerated from scripts to update for format changes.

---

## Correctness Testing

### Validating Against Analytical Solutions

For tractable cases, implementations are validated against analytical solutions:

```rust
#[test]
fn belief_propagation_exact_on_tree() {
    // Trees have exact BP solutions
    let tree = create_random_tree(50);
    let evidence = random_evidence(&tree, 0.2);
    
    // BP result
    let bp_result = tree.belief_propagation(&evidence);
    
    // Exact result via variable elimination
    let exact_result = tree.variable_elimination(&evidence);
    
    // Should match exactly (within numerical precision)
    assert_marginals_close(&bp_result, &exact_result, tolerance=1e-10);
}
```

### Known Test Cases

Standard test cases with known solutions:

**Sprinkler Network:**
- Classic Bayesian network from AI textbooks
- Known joint distribution allows exact probability calculations
- Tests verify inference produces expected probabilities

**Alarm Network:**
- Medical diagnosis network
- Standard benchmark in Bayesian network literature
- Used to validate approximate inference against exact results on simplified version

**Insurance Network:**
- Larger network for scalability testing
- Known properties for validation

### Ground Truth Datasets

Datasets with known ground truth for validation:

**Synthetic Networks:**
- Networks generated from known models (Erdős-Rényi, Barabási-Albert)
- Properties (degree distribution, clustering coefficient) can be analytically predicted

**Real Networks with Measured Properties:**
- Social networks with surveyed attributes
- Financial networks with known systemic risk outcomes
- Epidemiological networks with documented spread patterns

**Reference Implementations:**
- Comparison with established libraries (pgmpy for Python, bnlearn for R)
- Cross-validation of results

---

## Numerical Precision Testing

### Floating Point Tolerance

Floating-point comparisons use appropriate tolerances:

```rust
fn assert_float_eq(a: f64, b: f64, msg: &str) {
    let tolerance = 1e-9;
    assert!((a - b).abs() < tolerance, 
        "{}: expected {}, got {} (diff: {})", msg, b, a, (a-b).abs());
}
```

**Absolute vs Relative Tolerance:**
```rust
fn assert_close(a: f64, b: f64, rel_tol: f64, abs_tol: f64) {
    let diff = (a - b).abs();
    let tolerance = abs_tol.max(rel_tol * b.abs());
    assert!(diff <= tolerance, 
        "Values not close: {} vs {} (diff: {})", a, b, diff);
}
```

- Use absolute tolerance for values near zero
- Use relative tolerance for large values
- Combined tolerance for general cases

### Edge Cases

Numerical edge cases are explicitly tested:

```rust
#[test]
fn handles_zero_probabilities() {
    let network = network_with_zero_probabilities();
    let result = network.infer();
    assert!(result.is_valid()); // Should not panic or produce NaN
}

#[test]
fn handles_very_small_probabilities() {
    let network = network_with_extreme_probabilities(1e-300);
    let result = network.infer();
    // Should handle underflow gracefully
    assert!(!result.contains_nan());
}

#[test]
fn handles_very_large_values() {
    let network = network_with_large_factors(1e300);
    let result = network.infer();
    // Should handle overflow gracefully
    assert!(!result.contains_inf());
}
```

### Numerical Stability

Tests verify numerical stability:

```rust
#[test]
fn log_space_computation_avoids_underflow() {
    // Direct multiplication might underflow
    let probs = vec![1e-200; 100];
    let product_direct: f64 = probs.iter().product();
    assert!(product_direct.is_zero(), "Direct product underflows to 0");
    
    // Log-space computation should work
    let product_logspace = log_sum_exp(&probs.iter().map(|p| p.ln()).collect::<Vec<_>>());
    assert!(product_logspace.is_finite());
    assert!(product_logspace > 0.0);
}
```

---

## Performance Regression Testing

### Benchmark Tests

Performance tests detect regressions:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_belief_propagation(c: &mut Criterion) {
    let network = create_large_network();
    let evidence = create_evidence();
    
    c.bench_function("bp_1000_nodes", |b| {
        b.iter(|| {
            black_box(network.belief_propagation(&evidence))
        })
    });
}

criterion_group!(benches, bench_belief_propagation);
criterion_main!(benches);
```

### Performance Budgets

Operations have performance budgets:

```rust
#[test]
fn inference_meets_performance_budget() {
    let network = create_benchmark_network();
    let budget = Duration::from_millis(100);
    
    let start = Instant::now();
    let _result = network.infer();
    let elapsed = start.elapsed();
    
    assert!(elapsed < budget, 
        "Inference took {:?}, exceeding budget of {:?}", elapsed, budget);
}
```

### CI Integration

Performance tests run in CI with consistent hardware:

- Dedicated performance test runners (consistent CPU, memory)
- Baseline comparison with previous releases
- Alert on significant (>10%) regressions
- Track performance trends over time

---

## Property-Based Testing

### Invariants That Should Always Hold

Property-based testing verifies invariants:

```rust
use proptest::prelude::*;

proptest! {
    // Property: Belief propagation always produces normalized marginals
    #[test]
    fn bp_produces_normalized_marginals(
        network in network_strategy(),
        evidence in evidence_strategy()
    ) {
        let result = network.belief_propagation(&evidence);
        
        for marginal in result.marginals() {
            let sum: f64 = marginal.iter().sum();
            prop_assert!((sum - 1.0).abs() < 1e-6, 
                "Marginal not normalized: sums to {}", sum);
        }
    }
    
    // Property: Adding evidence should not increase entropy
    #[test]
    fn evidence_reduces_entropy(
        network in network_strategy()
    ) {
        let result_no_evidence = network.infer(&Evidence::empty());
        let entropy_no_ev = result_no_evidence.entropy();
        
        let evidence = random_evidence(&network);
        let result_with_evidence = network.infer(&evidence);
        let entropy_with_ev = result_with_evidence.entropy();
        
        prop_assert!(entropy_with_ev <= entropy_no_ev + 1e-6,
            "Entropy increased with evidence: {} -> {}", 
            entropy_no_ev, entropy_with_ev);
    }
}
```

### Generating Random Test Cases

Custom strategies generate valid test inputs:

```rust
fn network_strategy() -> impl Strategy<Value = Network> {
    (10..1000_usize, 0.01..0.5_f64).prop_map(|(n, p)| {
        Network::erdos_renyi(n, p)
    })
}

fn evidence_strategy() -> impl Strategy<Value = Evidence> {
    vec![(any::<NodeId>(), any::<bool>()), 0..10].prop_map(|assignments| {
        Evidence::from_assignments(assignments)
    })
}
```

### Shrinking Failing Cases

When a property fails, the test framework shrinks the input to find a minimal failing case:

```
Test failed: bp_produces_normalized_marginals
Original failing input: network with 847 nodes, 23045 edges
Shrunk to: network with 2 nodes, 1 edge
Minimal counterexample found.
```

---

## Fuzz Testing

### Random Input Generation

Fuzz testing discovers crashes and panics:

```rust
// Using cargo-fuzz
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Try to parse as network file
    if let Ok(network) = Network::from_bytes(data) {
        // Run operations that should not panic
        let _ = network.validate();
        let _ = network.node_count();
        
        // Try inference with random evidence
        if let Ok(evidence) = random_evidence_from_bytes(&data[network.len()..]) {
            let _ = network.infer(&evidence);
        }
    }
});
```

### Finding Crashes and Panics

Fuzzing focuses on:

**Parser Robustness:**
- Malformed network files
- Invalid encodings
- Extreme values (huge node counts, negative probabilities)

**Algorithm Robustness:**
- Degenerate graphs (empty, single node, complete)
- Extreme parameter values
- Division by zero scenarios

**Resource Exhaustion:**
- Quadratic blowup cases
- Infinite loop scenarios
- Memory exhaustion attempts

### Sanitizers

Sanitizers detect memory issues:

**AddressSanitizer:**
```bash
RUSTFLAGS="-Zsanitizer=address" cargo test
```
Detects: use-after-free, buffer overflows, memory leaks

**MemorySanitizer:**
```bash
RUSTFLAGS="-Zsanitizer=memory" cargo test
```
Detects: use of uninitialized memory

**ThreadSanitizer:**
```bash
RUSTFLAGS="-Zsanitizer=thread" cargo test
```
Detects: data races

---

## Test Data Management

### Synthetic Data Generation

Synthetic data with known properties:

```rust
pub struct NetworkGenerator;

impl NetworkGenerator {
    pub fn erdos_renyi(n: usize, p: f64) -> Network { ... }
    pub fn barabasi_albert(n: usize, m: usize) -> Network { ... }
    pub fn watts_strogatz(n: usize, k: usize, beta: f64) -> Network { ... }
    pub fn powerlaw_cluster(n: usize, m: usize, p: f64) -> Network { ... }
}
```

### Reference Datasets

Curated datasets for testing:

```
test_data/
  networks/
    social/
      karate_club.graphml
      dolphins.graphml
    biological/
      protein_interactions.graphml
    synthetic/
      grid_10x10.json
      tree_1000.json
  bayesian_networks/
    sprinkler.net
    alarm.net
    insurance.net
  regression/
    benchmark_results_v0.5.json
```

### Data Versioning

Test data is versioned:

- Git LFS for large test files
- Versioned dataset releases
- Changelog for test data modifications
- Reproducibility documentation

---

## Continuous Integration

### GitHub Actions Workflow

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, "1.70.0"]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run cargo-audit
      run: cargo audit
```

### Multiple Platform Testing

Tests run on:
- Linux (Ubuntu)
- macOS (Intel and Apple Silicon)
- Windows

### Multiple Python Versions

For Python bindings:
- Python 3.9, 3.10, 3.11, 3.12
- PyPy (optional)

### Compiler Version Testing

- Latest stable Rust
- Beta (to catch upcoming changes)
- MSRV (1.70.0)

---

## Test Coverage

### Coverage Metrics

**tarpaulin** for coverage reporting:

```bash
cargo tarpaulin --out Html --out Lcov
```

**Coverage targets:**
- Line coverage: 85%+
- Function coverage: 95%+
- Branch coverage: 75%+

### Branch Coverage

Special attention to branch coverage:

```rust
// Both branches must be tested
if condition {
    path_a();
} else {
    path_b();
}
```

### Mutation Testing

**cargo-mutants** verifies test quality:

```bash
cargo mutants
```

Mutants (code changes) that don't cause test failures indicate missing test coverage.

---

## Debugging Test Failures

### Reproducibility

All test failures must be reproducible:

**Seeded Random Tests:**
```rust
let seed = std::env::var("TEST_SEED")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(random());
eprintln!("Test seed: {}", seed); // Print seed on failure
```

**Deterministic Ordering:**
- Hash maps use deterministic hashing in tests
- Parallel tests use fixed thread counts
- Sorting where order matters

### Minimal Reproduction

Failing tests should be minimized:

1. Property-based tests: Use shrinking
2. Fuzz tests: Corpus minimization
3. Integration tests: Reduce to minimal example

### Test Logging

Comprehensive logging for debugging:

```rust
#[test]
fn test_with_logging() {
    let _ = env_logger::try_init();
    
    log::debug!("Starting test with config: {:?}", config);
    let result = operation();
    log::debug!("Got result: {:?}", result);
    
    assert!(result.is_valid());
}
```

**RUST_LOG** environment variable controls log level.

---

## Testing Documentation

### Documenting Tested Behavior

Doc comments include usage examples that serve as tests:

```rust
/// Runs belief propagation inference.
///
/// # Examples
///
/// ```
/// use lutufi::BayesianNetwork;
///
/// let mut network = BayesianNetwork::new();
/// // ... configure network
///
/// let result = network.belief_propagation();
/// assert!(result.is_converged());
/// ```
pub fn belief_propagation(&self) -> InferenceResult { ... }
```

### Doctest

Documentation examples run as tests:

```bash
cargo test --doc
```

### Test as Specification

Tests serve as executable specifications:

```rust
// This test documents that BP converges on trees
#[test]
fn belief_propagation_converges_on_trees() {
    // Tree-structured networks have no loops
    let tree = create_tree_network();
    
    // BP is exact on tree structures
    let result = tree.belief_propagation();
    
    // Should converge in finite iterations
    assert!(result.is_converged());
    assert!(result.iterations() <= tree.node_count());
}
```

---

## How Lutufi Tests

### Testing Architecture

Lutufi's testing follows a layered architecture:

**Level 1: Unit Tests**
- Located alongside source code in `#[cfg(test)]` modules
- Test individual functions and methods
- Use mocks for external dependencies
- Fast execution (<1ms per test)

**Level 2: Integration Tests**
- Located in `tests/` directory
- Test component interactions
- Use test fixtures for realistic data
- Moderate execution time (<1s per test)

**Level 3: System Tests**
- Located in `tests/system/`
- Test complete workflows
- Use real and realistic data
- Slower execution (seconds to minutes)

**Level 4: Property Tests**
- Located in `tests/property/`
- Use proptest framework
- Generate hundreds of test cases per property
- Run in CI with reduced case count for speed

**Level 5: Fuzz Tests**
- Located in `fuzz/`
- Run continuously or on-demand
- Discover edge cases and crashes
- Not run in standard CI due to duration

### Test Organization

```
lutufi/
├── src/
│   ├── lib.rs
│   ├── network/
│   │   ├── mod.rs
│   │   └── tests.rs          # Unit tests
│   └── ...
├── tests/
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── workflows.rs
│   │   └── python_bindings.rs
│   ├── property/
│   │   ├── mod.rs
│   │   └── invariants.rs
│   ├── system/
│   │   ├── mod.rs
│   │   └── benchmarks.rs
│   └── fixtures/
│       ├── mod.rs
│       └── data/
├── fuzz/
│   └── fuzz_targets/
│       ├── parse_network.rs
│       └── run_inference.rs
└── benches/
    └── criterion/
        └── inference.rs
```

### CI/CD Integration

**Pre-commit hooks:**
- Formatting check
- Clippy lints
- Quick test subset

**Pull request CI:**
- Full test suite
- Coverage check
- Performance regression check
- Documentation build

**Release CI:**
- Extended test suite (all features, all platforms)
- Fuzz testing (short run)
- Security audit
- Benchmark trending

### Test Execution

**Development:**
```bash
# Quick feedback during development
cargo test --lib  # Unit tests only
cargo test network::  # Specific module

# Before commit
cargo test  # All tests
cargo test --all-features  # With optional features
```

**Pre-release:**
```bash
# Full validation
cargo test --all-features
cargo test --release  # Optimized build
cargo test --ignored  # Slow tests

cargo fuzz run parse_network  # Fuzz testing
cargo mutants  # Mutation testing
```

---

## Key References

### Testing Scientific Software

1. **"Testing of Scientific Software" by H. M. Deitel:**
   - Best practices for scientific computing validation

2. **"Software Engineering for Science" (Chapman & Hall):**
   - Comprehensive coverage of scientific software testing

3. **"Best Practices for Scientific Computing" by Wilson et al.:**
   - PLOS Biology, 2014
   - Foundational practices including testing

4. **NumPy Testing Guidelines:**
   https://numpy.org/doc/stable/reference/testing.html
   - Testing practices from scientific Python ecosystem

### Property-Based Testing

1. **"Property-Based Testing with PropEr, Erlang, and Elixir" by Fred Hebert:**
   - Concepts applicable to any language

2. **Hypothesis Documentation:**
   https://hypothesis.readthedocs.io/
   - Python property-based testing (concepts transfer to Rust)

3. **proptest Documentation:**
   https://altsysrq.github.io/proptest-book/
   - Rust property-based testing

### Fuzzing

1. **"The Fuzzing Book" by Zeller et al.:**
   https://www.fuzzingbook.org/
   - Comprehensive fuzzing resource

2. **Rust Fuzz Book:**
   https://rust-fuzz.github.io/book/
   - Fuzzing in Rust ecosystem

3. **cargo-fuzz Documentation:**
   https://github.com/rust-fuzz/cargo-fuzz
   - Practical fuzzing guide

### Performance Testing

1. **Criterion.rs Documentation:**
   https://bheisler.github.io/criterion.rs/book/
   - Statistical benchmarking in Rust

2. **"Systems Performance" by Brendan Gregg:**
   - Performance measurement methodologies

### Continuous Integration

1. **GitHub Actions Documentation:**
   https://docs.github.com/en/actions
   - CI/CD for GitHub repositories

2. **Rust CI Best Practices:**
   https://ehsanmkermani.com/2021/10/27/rust-ci-best-practices/
   - Rust-specific CI recommendations

---

## Conclusion

The testing philosophy for Lutufi, authored by Wasswa Lutufi Sebbanja and licensed under Apache 2.0, reflects the critical importance of correctness in scientific software. The multi-layered testing strategy combines traditional software testing with domain-specific validation techniques to ensure that the library produces mathematically correct, numerically stable, and statistically valid results.

Key elements of the testing approach include:

1. **Prioritizing Correctness:** Testing focuses on verifying mathematical correctness through analytical comparisons, statistical validation, and property verification, not just functional behavior.

2. **Handling Probabilistic Code:** Specialized techniques for testing randomized algorithms, including seed management for reproducibility, statistical hypothesis testing, and convergence verification.

3. **Comprehensive Coverage:** Multiple testing levels (unit, integration, system, property, fuzz) provide confidence across different aspects of the codebase.

4. **Continuous Validation:** Automated CI/CD ensures that tests run consistently across platforms, Rust versions, and configurations.

5. **Reproducibility:** All tests are designed to be reproducible, with seeded randomness, deterministic ordering, and clear failure diagnostics.

The testing strategy scales with the project, from fast unit tests for development feedback to comprehensive system tests for release validation. By maintaining high testing standards, Lutufi ensures that researchers and practitioners can trust its results for critical applications in network analysis, probabilistic inference, and decision-making under uncertainty.
