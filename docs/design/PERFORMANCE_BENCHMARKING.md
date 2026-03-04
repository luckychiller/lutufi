# Lutufi Performance Benchmarking Plan

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Benchmarking Philosophy](#benchmarking-philosophy)
3. [What to Benchmark](#what-to-benchmark)
4. [Benchmark Datasets](#benchmark-datasets)
5. [Comparison Baselines](#comparison-baselines)
6. [Hardware Specifications](#hardware-specifications)
7. [Measurement Methodology](#measurement-methodology)
8. [Benchmark Categories](#benchmark-categories)
9. [Regression Testing](#regression-testing)
10. [Accuracy Benchmarks](#accuracy-benchmarks)
11. [Scalability Benchmarks](#scalability-benchmarks)
12. [Profiling Tools](#profiling-tools)
13. [Benchmark Reporting](#benchmark-reporting)
14. [Competitive Benchmarking](#competitive-benchmarking)
15. [User-Facing Performance Metrics](#user-facing-performance-metrics)
16. [How Lutufi Benchmarks](#how-lutufi-benchmarks)
17. [Key References](#key-references)

---

## Executive Summary

Performance benchmarking is essential for validating Lutufi's design goals and ensuring it meets the computational demands of real-world probabilistic modeling. This document establishes a comprehensive benchmarking framework that covers inference speed, learning performance, scalability characteristics, memory efficiency, and accuracy trade-offs.

The benchmarking strategy encompasses:

1. **Standardized Datasets**: Reference networks spanning multiple scales and domains
2. **Competitive Baselines**: Systematic comparison with pgmpy, bnlearn, NetworkX, and other libraries
3. **Rigorous Methodology**: Statistical rigor with confidence intervals and multiple runs
4. **Continuous Monitoring**: Automated regression detection in CI/CD pipelines
5. **User-Facing Metrics**: Clear performance expectations for different use cases

---

## Benchmarking Philosophy

### Why We Benchmark

Performance is a first-class concern for Lutufi because:

1. **User Experience**: Slow inference breaks interactive workflows
2. **Scalability**: Performance determines maximum problem size
3. **Resource Efficiency**: Faster computation reduces infrastructure costs
4. **Competitive Positioning**: Users compare libraries before adopting
5. **Quality Assurance**: Performance regressions indicate underlying issues

### What Matters

**Primary Metrics:**
- **Wall time**: End-to-end elapsed time (user perception)
- **Memory**: Peak resident set size (resource constraints)
- **Accuracy**: Quality of approximate inference (correctness)
- **Throughput**: Queries per second (batch processing)

**Secondary Metrics:**
- **CPU utilization**: Parallel efficiency
- **Cache efficiency**: Memory access patterns
- **Convergence rate**: Iterative algorithm efficiency
- **Startup time**: Library initialization overhead

**Trade-off Analysis:**

| Scenario | Prioritize | Accept |
|----------|-----------|--------|
| Interactive analysis | Wall time | Approximate results |
| Production inference | Throughput | Higher latency |
| Large-scale learning | Scalability | Longer training |
| Embedded systems | Memory | Slower speed |
| Research | Accuracy | Longer computation |

### User-Perceived Performance

```rust
pub struct UserExperienceMetrics {
    /// Time to first result (interactive feel)
    pub time_to_first_result: Duration,
    
    /// Responsiveness during computation
    pub progress_update_frequency: f64,  // Hz
    
    /// Perceived latency for common operations
    pub common_operation_latencies: HashMap<String, Duration>,
}

impl UserExperienceMetrics {
    pub fn assess_interactive_quality(&self) -> InteractiveQuality {
        if self.time_to_first_result < Duration::from_millis(100) {
            InteractiveQuality::Instant
        } else if self.time_to_first_result < Duration::from_secs(1) {
            InteractiveQuality::Responsive
        } else if self.time_to_first_result < Duration::from_secs(5) {
            InteractiveQuality::Acceptable
        } else {
            InteractiveQuality::Slow
        }
    }
}
```

---

## What to Benchmark

### Inference Speed

```rust
pub struct InferenceBenchmark {
    /// Model configuration
    pub model: BenchmarkModel,
    
    /// Query configuration
    pub query: BenchmarkQuery,
    
    /// Algorithm to test
    pub algorithm: InferenceAlgorithm,
    
    /// Metrics to collect
    pub metrics: Vec<InferenceMetric>,
}

pub enum InferenceMetric {
    TotalWallTime,
    SetupTime,
    InferenceTime,
    ResultConversionTime,
    MemoryAllocated,
    CacheHitRate,
    IterationsUntilConvergence,
    MessagesPassed,
}
```

**Inference Benchmark Scenarios:**

1. **Marginal Queries**: Single and multiple variable marginals
2. **Conditional Queries**: With varying evidence complexity
3. **MAP Inference**: Most probable explanation
4. **Interventional Queries**: Causal inference
5. **Batch Queries**: Multiple queries on same model

### Learning Speed

```rust
pub struct LearningBenchmark {
    pub algorithm: LearningAlgorithm,
    pub dataset: BenchmarkDataset,
    pub model_complexity: ModelComplexity,
    pub convergence_criteria: ConvergenceCriteria,
}

pub enum LearningMetric {
    StructureLearningTime,
    ParameterLearningTime,
    ScoringFunctionCalls,
    CacheHitRate,
    FinalScore,
    ModelComplexity,
    CrossValidationAccuracy,
}
```

### Scalability Curves

```rust
pub struct ScalabilityBenchmark {
    /// Varying parameter
    pub dimension: ScalabilityDimension,
    
    /// Range to test
    pub range: Range<usize>,
    
    /// Step size
    pub step: usize,
}

pub enum ScalabilityDimension {
    NodeCount,
    EdgeCount,
    AverageDegree,
    Treewidth,
    StateSpaceSize,
    EvidenceCount,
    QueryComplexity,
}
```

### Accuracy vs Speed Tradeoffs

```rust
pub struct AccuracySpeedTradeoff {
    pub exact_result: QueryResult,
    pub approximate_results: Vec<(InferenceAlgorithm, QueryResult, Duration)>,
}

impl AccuracySpeedTradeoff {
    pub fn compute_pareto_frontier(&self) -> Vec<(InferenceAlgorithm, f64, Duration)> {
        // Returns algorithms on Pareto frontier of accuracy vs speed
        let mut points: Vec<_> = self.approximate_results.iter()
            .map(|(alg, result, time)| {
                let accuracy = 1.0 - self.kl_divergence(&self.exact_result, result);
                (alg.clone(), accuracy, *time)
            })
            .collect();
        
        // Filter to Pareto optimal points
        points.into_iter()
            .filter(|(_, acc1, time1)| {
                !points.iter().any(|(_, acc2, time2)| {
                    acc2 > acc1 && time2 < time1
                })
            })
            .collect()
    }
}
```

### Memory Usage

```rust
pub struct MemoryBenchmark {
    pub peak_rss_bytes: usize,
    pub peak_heap_bytes: usize,
    pub allocations_count: usize,
    pub fragmentation_ratio: f64,
    pub cache_efficiency: f64,
}
```

---

## Benchmark Datasets

### Reference Networks

**Small Networks (< 100 nodes):**

| Name | Nodes | Edges | Domain | Source |
|------|-------|-------|--------|--------|
| Asia | 8 | 8 | Medical | Lauritzen & Spiegelhalter |
| Alarm | 37 | 46 | Medical | Beinlich et al. |
| Insurance | 27 | 52 | Insurance | Binder et al. |
| Hailfinder | 56 | 66 | Weather | Abramson et al. |
| Mildew | 35 | 46 | Agriculture | Jensen |

**Medium Networks (100-1,000 nodes):**

| Name | Nodes | Edges | Domain | Treewidth |
|------|-------|-------|--------|-----------|
| Barley | 48 | 84 | Agriculture | 7 |
| Diabetes | 413 | 602 | Medical | - |
| Link | 724 | 1,125 | Communication | - |
| Munin1 | 189 | 282 | Fault diagnosis | - |
| Pigs | 441 | 592 | Biology | - |

**Large Networks (1,000+ nodes):**

| Name | Nodes | Edges | Domain | Notes |
|------|-------|-------|--------|-------|
| Andes | 223 | 338 | Education | |
| Munin2 | 1,003 | 1,242 | Fault diagnosis | |
| Munin3 | 1,044 | 1,747 | Fault diagnosis | |
| Munin4 | 1,041 | 1,843 | Fault diagnosis | |
| Pathfinder | 109 | 195 | Medical | |
| Win95pts | 76 | 112 | Troubleshooting | |

### Synthetic Benchmarks

```rust
pub struct SyntheticNetworkGenerator;

impl SyntheticNetworkGenerator {
    /// Generate random tree-structured network
    pub fn generate_tree(n_nodes: usize, n_states: usize) -> BayesianNetwork {
        // Implementation
    }
    
    /// Generate scale-free network (Barabási-Albert)
    pub fn generate_scale_free(
        n_nodes: usize,
        m_edges: usize,
        n_states: usize
    ) -> BayesianNetwork {
        // Implementation
    }
    
    /// Generate grid network (high treewidth)
    pub fn generate_grid(width: usize, height: usize, n_states: usize) -> BayesianNetwork {
        // Implementation
    }
    
    /// Generate network with controlled treewidth
    pub fn generate_bounded_treewidth(
        n_nodes: usize,
        max_treewidth: usize,
        n_states: usize
    ) -> BayesianNetwork {
        // Implementation
    }
}
```

### Real-World Datasets

**Social Networks:**
- **Karate Club**: 34 nodes, social interactions
- **Dolphins**: 62 nodes, dolphin associations
- **Email-EU-Core**: 1,005 nodes, email communications
- **Facebook Circles**: 4,039 nodes, social circles

**Biological Networks:**
- **Yeast**: Protein-protein interactions
- **Human Gene**: Gene regulatory networks
- **Ecological Food Webs**: Species interactions

**Economic Networks:**
- **Trade Networks**: Country trade relationships
- **Interbank Networks**: Financial institution lending
- **Supply Chains**: Production network dependencies

### Baseline Comparisons

**Exact Inference Baselines:**
- Brute-force enumeration (ground truth for small networks)
- Junction tree with optimal elimination order
- Variable elimination with perfect ordering

**Approximate Inference Baselines:**
- Mean field with exact moments
- Loopy BP on tree (should be exact)
- Gibbs sampling with long runs (gold standard)

---

## Comparison Baselines

### pgmpy

Python library for probabilistic graphical models.

**Comparison Points:**
- Exact inference algorithms
- Structure learning
- API ease of use
- Scalability

```python
# pgmpy comparison benchmark
def benchmark_pgmpy(model_path, query_config):
    from pgmpy.models import BayesianNetwork
    from pgmpy.inference import VariableElimination
    import time
    
    # Load model
    start = time.time()
    model = BayesianNetwork.load(model_path)
    load_time = time.time() - start
    
    # Run inference
    infer = VariableElimination(model)
    start = time.time()
    result = infer.query(
        variables=query_config['variables'],
        evidence=query_config['evidence']
    )
    inference_time = time.time() - start
    
    return {
        'load_time': load_time,
        'inference_time': inference_time,
        'result': result
    }
```

### bnlearn (R)

R package for Bayesian network learning.

**Comparison Points:**
- Structure learning algorithms
- Constraint-based learning
- Parameter estimation

### NetworkX

Python library for network analysis.

**Comparison Points:**
- Graph operations
- Centrality measures
- Path algorithms
- Visualization

```python
# NetworkX graph operations comparison
def benchmark_networkx_conversion(model):
    import networkx as nx
    import time
    
    # To NetworkX
    start = time.time()
    nx_graph = model.to_networkx()
    to_nx_time = time.time() - start
    
    # Betweenness centrality
    start = time.time()
    centrality = nx.betweenness_centrality(nx_graph)
    centrality_time = time.time() - start
    
    # From NetworkX
    start = time.time()
    model_back = BayesianNetwork.from_networkx(nx_graph)
    from_nx_time = time.time() - start
    
    return {
        'to_networkx': to_nx_time,
        'centrality': centrality_time,
        'from_networkx': from_nx_time,
    }
```

### graph-tool

High-performance Python network library.

**Comparison Points:**
- Large graph processing
- Parallel algorithms
- C++ backend performance

### PyMC

Probabilistic programming in Python.

**Comparison Points:**
- Sampling methods
- Variational inference
- Model specification

### Fair Comparison Guidelines

```rust
pub struct FairComparisonConfig {
    /// Same algorithm implementation
    pub algorithm: String,
    
    /// Same convergence criteria
    pub convergence_threshold: f64,
    pub max_iterations: usize,
    
    /// Same hardware
    pub hardware: HardwareConfig,
    
    /// Same problem instance
    pub random_seed: u64,
    
    /// Warm-up runs
    pub warmup_iterations: usize,
    
    /// Measured runs
    pub measurement_iterations: usize,
}
```

---

## Hardware Specifications

### Reference Hardware for Standardized Benchmarks

**Tier 1: Consumer Hardware**
```yaml
name: "Consumer Reference"
cpu: "Intel Core i7-12700K or AMD Ryzen 7 5800X"
ram: "32 GB DDR4-3200"
storage: "NVMe SSD (3.5 GB/s)"
os: "Ubuntu 22.04 LTS / Windows 11"
cost: "~$1,500"
```

**Tier 2: Workstation**
```yaml
name: "Workstation Reference"
cpu: "Intel Xeon W-3365 or AMD Threadripper PRO 3975WX"
ram: "128 GB DDR4-3200 ECC"
storage: "NVMe SSD RAID"
os: "Ubuntu 22.04 LTS"
cost: "~$8,000"
```

**Tier 3: Server**
```yaml
name: "Server Reference"
cpu: "2x Intel Xeon Gold 6348 or 2x AMD EPYC 7763"
ram: "512 GB DDR4-3200 ECC"
storage: "NVMe SSD Array"
interconnect: "100 GbE"
os: "Ubuntu 22.04 LTS"
cost: "~$50,000"
```

### Cloud Instance Types

| Provider | Instance | Specs | Use Case |
|----------|----------|-------|----------|
| AWS | c6i.2xlarge | 8 vCPU, 16 GB | Development |
| AWS | c6i.8xlarge | 32 vCPU, 64 GB | Production |
| GCP | c2-standard-16 | 16 vCPU, 64 GB | Benchmarking |
| Azure | F64s v2 | 64 vCPU, 128 GB | Large-scale |

### Documenting Hardware

```rust
pub struct HardwareInfo {
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub cpu_base_frequency_ghz: f64,
    pub cpu_turbo_frequency_ghz: f64,
    pub ram_gb: usize,
    pub ram_speed_mhz: usize,
    pub storage_type: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
}

impl HardwareInfo {
    pub fn capture() -> Self {
        Self {
            cpu_model: Self::get_cpu_model(),
            cpu_cores: num_cpus::get_physical(),
            cpu_threads: num_cpus::get(),
            // ... capture all fields
        }
    }
    
    pub fn to_benchmark_context(&self) -> String {
        format!(
            "{}-core {} @ {:.1}GHz, {}GB RAM",
            self.cpu_cores, self.cpu_model, 
            self.cpu_base_frequency_ghz, self.ram_gb
        )
    }
}
```

---

## Measurement Methodology

### Timing Methods

```rust
pub struct TimingMeasurement {
    /// Wall clock time (what users experience)
    pub wall_time: Duration,
    
    /// CPU time (actual computation)
    pub cpu_time: Duration,
    
    /// System time (kernel operations)
    pub system_time: Duration,
}

pub struct HighPrecisionTimer;

impl HighPrecisionTimer {
    pub fn measure<F, T>(f: F) -> (T, TimingMeasurement)
    where F: FnOnce() -> T {
        let start_wall = Instant::now();
        let start_cpu = ProcessTime::now();
        
        let result = f();
        
        let cpu_time = start_cpu.elapsed();
        let wall_time = start_wall.elapsed();
        
        (result, TimingMeasurement {
            wall_time,
            cpu_time,
            system_time: Duration::from_secs(0), // Platform-specific
        })
    }
}
```

### Memory Profiling

```rust
pub struct MemoryMeasurement {
    pub peak_rss_bytes: usize,
    pub peak_heap_bytes: usize,
    pub allocations: usize,
    pub deallocations: usize,
}

#[cfg(target_os = "linux")]
impl MemoryMeasurement {
    pub fn measure<F, T>(f: F) -> (T, Self)
    where F: FnOnce() -> T {
        let start_rss = Self::current_rss();
        let mut peak_rss = start_rss;
        
        // Set up sampling
        let sampling = Arc::new(AtomicUsize::new(start_rss));
        let sampling_clone = sampling.clone();
        
        let handle = thread::spawn(move || {
            loop {
                let current = Self::current_rss();
                let prev = sampling_clone.load(Ordering::Relaxed);
                if current > prev {
                    sampling_clone.store(current, Ordering::Relaxed);
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
        
        let result = f();
        
        peak_rss = sampling.load(Ordering::Relaxed);
        
        (result, Self {
            peak_rss_bytes: peak_rss,
            peak_heap_bytes: 0, // From allocator hooks
            allocations: 0,
            deallocations: 0,
        })
    }
    
    fn current_rss() -> usize {
        // Read from /proc/self/status
        let status = std::fs::read_to_string("/proc/self/status").unwrap();
        // Parse VmRSS line
        // ...
    }
}
```

### Statistical Rigor

```rust
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

pub struct BenchmarkConfig {
    /// Number of warm-up runs
    pub warmup_iterations: usize,
    
    /// Number of measured runs
    pub measurement_iterations: usize,
    
    /// Minimum total measurement time
    pub min_measurement_duration: Duration,
    
    /// Confidence level for intervals
    pub confidence_level: f64,
    
    /// Outlier detection threshold
    pub outlier_threshold: f64,
}

impl BenchmarkRunner {
    pub fn run<T>(&self, benchmark: impl Fn() -> T) -> BenchmarkResults {
        // Warm-up
        for _ in 0..self.config.warmup_iterations {
            let _ = benchmark();
        }
        
        // Measurement
        let mut measurements = Vec::new();
        let start = Instant::now();
        
        for i in 0..self.config.measurement_iterations {
            let (_, measurement) = HighPrecisionTimer::measure(&benchmark);
            measurements.push(measurement);
            
            // Ensure minimum duration
            if i >= 10 && start.elapsed() >= self.config.min_measurement_duration {
                break;
            }
        }
        
        // Remove outliers
        let filtered = self.remove_outliers(measurements);
        
        // Compute statistics
        BenchmarkResults {
            mean: Self::mean(&filtered),
            median: Self::median(&filtered),
            std_dev: Self::std_dev(&filtered),
            confidence_interval: Self::confidence_interval(&filtered, self.config.confidence_level),
            min: filtered.iter().min().copied().unwrap(),
            max: filtered.iter().max().copied().unwrap(),
        }
    }
    
    fn confidence_interval(times: &[Duration], level: f64) -> (Duration, Duration) {
        let mean = Self::mean(times);
        let std_err = Self::std_dev(times) / (times.len() as f64).sqrt();
        
        // t-distribution critical value (approximate)
        let z = 1.96; // For 95% confidence
        
        let margin = Duration::from_secs_f64(z * std_err.as_secs_f64());
        (mean - margin, mean + margin)
    }
}
```

---

## Benchmark Categories

### Microbenchmarks

Individual operation performance:

```rust
pub struct MicroBenchmarks;

impl Microbenchmarks {
    #[bench]
    fn factor_multiplication(b: &mut Bencher) {
        let f1 = DenseFactor::random(5, 4);
        let f2 = DenseFactor::random(5, 4);
        
        b.iter(|| {
            let _ = f1.multiply(&f2);
        });
    }
    
    #[bench]
    fn message_passing_step(b: &mut Bencher) {
        let graph = FactorGraph::random_tree(100, 3);
        let mut bp = BeliefPropagation::new(&graph);
        
        b.iter(|| {
            bp.iteration();
        });
    }
    
    #[bench]
    fn cpd_lookup(b: &mut Bencher) {
        let cpd = TabularCPD::random(10, 5);
        let parent_values = vec![0, 1, 0, 1, 0];
        
        b.iter(|| {
            let _ = cpd.get_probability(0, &parent_values);
        });
    }
    
    #[bench]
    fn graph_traversal(b: &mut Bencher) {
        let graph = FactorGraph::random(1000, 5000);
        
        b.iter(|| {
            for node in graph.nodes() {
                let _ = graph.neighbors(node).count();
            }
        });
    }
}
```

### Mesobenchmarks

Full inference on reference networks:

```rust
pub struct MesoBenchmarks;

impl MesoBenchmarks {
    pub fn benchmark_network(network_path: &Path) -> MesoResults {
        let model = BayesianNetwork::load(network_path).unwrap();
        
        let mut results = MesoResults::new();
        
        // Test different query types
        results.marginal = benchmark_marginal_inference(&model);
        results.conditional = benchmark_conditional_inference(&model);
        results.map = benchmark_map_inference(&model);
        
        // Test different algorithms
        results.variable_elimination = benchmark_with_algorithm(&model, "variable_elimination");
        results.belief_propagation = benchmark_with_algorithm(&model, "belief_propagation");
        results.gibbs_sampling = benchmark_with_algorithm(&model, "gibbs_sampling");
        
        results
    }
}
```

### Macrobenchmarks

End-to-end workflows:

```rust
pub struct MacroBenchmarks;

impl MacroBenchmarks {
    /// Complete workflow: load data, learn model, run inference
    pub fn benchmark_complete_workflow(dataset: &BenchmarkDataset) -> WorkflowResults {
        let data = load_dataset(dataset);
        
        // Structure learning
        let (structure, structure_time) = timed(|| {
            BayesianNetwork::fit_structure(&data, "hc")
        });
        
        // Parameter learning
        let (model, parameter_time) = timed(|| {
            let mut m = structure;
            m.fit_parameters(&data, "mle");
            m
        });
        
        // Inference
        let queries = generate_test_queries(&model);
        let (inference_results, inference_time) = timed(|| {
            queries.iter()
                .map(|q| model.query(&q.variables, &q.evidence))
                .collect::<Vec<_>>()
        });
        
        WorkflowResults {
            structure_learning_time: structure_time,
            parameter_learning_time: parameter_time,
            inference_time,
            total_time: structure_time + parameter_time + inference_time,
        }
    }
}
```

---

## Regression Testing

### Automated Benchmark Runs

```rust
pub struct RegressionDetector {
    baseline: BenchmarkHistory,
    threshold: f64,  // Percentage change threshold
}

impl RegressionDetector {
    pub fn detect_regression(&self, new_results: &BenchmarkResults) -> Option<RegressionReport> {
        let mut regressions = Vec::new();
        
        for (benchmark_name, new_result) in &new_results.measurements {
            if let Some(baseline) = self.baseline.get(benchmark_name) {
                let change = (new_result.mean.as_secs_f64() - baseline.mean.as_secs_f64())
                    / baseline.mean.as_secs_f64();
                
                if change > self.threshold {
                    regressions.push(Regression {
                        benchmark: benchmark_name.clone(),
                        baseline_time: baseline.mean,
                        new_time: new_result.mean,
                        change_percent: change * 100.0,
                        severity: if change > 0.5 {
                            Severity::Critical
                        } else {
                            Severity::Warning
                        },
                    });
                }
            }
        }
        
        if regressions.is_empty() {
            None
        } else {
            Some(RegressionReport { regressions })
        }
    }
}
```

### CI Integration

```yaml
# .github/workflows/benchmark.yml
name: Performance Regression

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Run benchmarks
      run: cargo bench --features benchmark
    
    - name: Compare with baseline
      run: |
        cargo bench --features benchmark -- --compare baseline.json
    
    - name: Comment PR
      if: github.event_name == 'pull_request'
      uses: actions/github-script@v6
      with:
        script: |
          const results = require('./benchmark_results.json');
          github.rest.issues.createComment({
            issue_number: context.issue.number,
            owner: context.repo.owner,
            repo: context.repo.repo,
            body: formatBenchmarkResults(results)
          })
```

### Performance Budgets

```rust
pub struct PerformanceBudget {
    /// Maximum allowed time for operation
    pub max_duration: Duration,
    
    /// Maximum allowed memory
    pub max_memory_mb: usize,
    
    /// Maximum allowed regression from baseline
    pub max_regression_percent: f64,
}

impl PerformanceBudget {
    pub fn check(&self, result: &BenchmarkResult) -> Result<(), BudgetViolation> {
        if result.mean > self.max_duration {
            return Err(BudgetViolation::TimeExceeded {
                limit: self.max_duration,
                actual: result.mean,
            });
        }
        
        if result.peak_memory_mb > self.max_memory_mb {
            return Err(BudgetViolation::MemoryExceeded {
                limit: self.max_memory_mb,
                actual: result.peak_memory_mb,
            });
        }
        
        Ok(())
    }
}
```

---

## Accuracy Benchmarks

### Comparing Approximate to Exact

```rust
pub struct AccuracyBenchmark {
    pub exact_result: QueryResult,
    pub approximate_results: Vec<(String, QueryResult)>,
}

impl AccuracyBenchmark {
    pub fn compute_metrics(&self) -> AccuracyMetrics {
        let mut metrics = AccuracyMetrics::new();
        
        for (alg_name, approx_result) in &self.approximate_results {
            // KL divergence
            let kl_div = self.kl_divergence(&self.exact_result, approx_result);
            
            // Max absolute error
            let max_error = self.max_absolute_error(&self.exact_result, approx_result);
            
            // Mean absolute error
            let mae = self.mean_absolute_error(&self.exact_result, approx_result);
            
            // Rank correlation (for MAP)
            let rank_corr = self.spearman_correlation(&self.exact_result, approx_result);
            
            metrics.add(alg_name.clone(), AccuracyMetric {
                kl_divergence: kl_div,
                max_absolute_error: max_error,
                mean_absolute_error: mae,
                rank_correlation: rank_corr,
            });
        }
        
        metrics
    }
    
    fn kl_divergence(&self, exact: &QueryResult, approx: &QueryResult) -> f64 {
        exact.probabilities.iter()
            .zip(approx.probabilities.iter())
            .map(|(p, q)| {
                if *p > 0.0 {
                    p * (p / q.max(1e-10)).ln()
                } else {
                    0.0
                }
            })
            .sum()
    }
}
```

### Validation Against Analytical Solutions

```rust
pub struct AnalyticalValidation;

impl AnalyticalValidation {
    /// Validate on tree-structured networks (where BP is exact)
    pub fn validate_belief_propagation_on_trees() -> ValidationReport {
        let mut report = ValidationReport::new();
        
        for size in [10, 50, 100, 500] {
            let tree = BayesianNetwork::random_tree(size, 3);
            let evidence = generate_random_evidence(&tree, 0.2);
            
            // Exact inference via junction tree
            let exact = tree.query_with_algorithm(
                &all_variables(&tree),
                &evidence,
                "junction_tree"
            );
            
            // BP should give same result on trees
            let bp = tree.query_with_algorithm(
                &all_variables(&tree),
                &evidence,
                "belief_propagation"
            );
            
            let error = compute_max_error(&exact, &bp);
            report.add_result(size, error);
            
            assert!(error < 1e-10, "BP should be exact on trees");
        }
        
        report
    }
}
```

### Convergence Diagnostics

```rust
pub struct ConvergenceBenchmark {
    pub iteration_history: Vec<BeliefState>,
    pub convergence_threshold: f64,
}

impl ConvergenceBenchmark {
    pub fn analyze_convergence(&self) -> ConvergenceAnalysis {
        let deltas: Vec<f64> = self.iteration_history.windows(2)
            .map(|w| belief_difference(&w[0], &w[1]))
            .collect();
        
        ConvergenceAnalysis {
            iterations_to_convergence: deltas.iter()
                .position(|&d| d < self.convergence_threshold)
                .map(|i| i + 1)
                .unwrap_or(deltas.len()),
            
            convergence_rate: self.estimate_convergence_rate(&deltas),
            
            oscillation_detected: self.detect_oscillation(&deltas),
            
            final_delta: deltas.last().copied().unwrap_or(f64::INFINITY),
        }
    }
    
    fn estimate_convergence_rate(&self, deltas: &[f64]) -> f64 {
        // Fit exponential decay: delta_i = a * exp(-r * i)
        // Return r (convergence rate)
        let log_deltas: Vec<f64> = deltas.iter()
            .map(|&d| d.ln())
            .collect();
        
        // Linear regression on log(deltas)
        linear_regression_slope(&log_deltas)
    }
}
```

---

## Scalability Benchmarks

### How Performance Degrades with Size

```rust
pub struct ScalabilityAnalysis {
    pub dimensions: Vec<ScalabilityDimension>,
    pub results: Vec<ScalabilityResult>,
}

pub struct ScalabilityResult {
    pub problem_size: usize,
    pub wall_time: Duration,
    pub memory_mb: usize,
    pub throughput: f64,  // queries per second
}

impl ScalabilityAnalysis {
    pub fn compute_complexity_class(&self) -> ComplexityClass {
        // Fit different complexity models
        let models = [
            ("O(1)", Self::fit_constant),
            ("O(log n)", Self::fit_log),
            ("O(n)", Self::fit_linear),
            ("O(n log n)", Self::fit_nlogn),
            ("O(n^2)", Self::fit_quadratic),
            ("O(2^n)", Self::fit_exponential),
        ];
        
        // Select best fit
        models.iter()
            .map(|(name, fit_fn)| (name, fit_fn(&self.results)))
            .min_by(|(_, err1), (_, err2)| err1.partial_cmp(err2).unwrap())
            .map(|(name, _)| ComplexityClass::from(*name))
            .unwrap()
    }
    
    pub fn find_breakpoint(&self, max_acceptable_time: Duration) -> Option<usize> {
        self.results.iter()
            .find(|r| r.wall_time > max_acceptable_time)
            .map(|r| r.problem_size)
    }
}
```

### Identifying Breakpoints

```rust
pub struct BreakpointAnalysis;

impl BreakpointAnalysis {
    /// Find where algorithm needs to switch from exact to approximate
    pub fn find_exact_inference_breakpoint() -> BreakpointReport {
        let mut report = BreakpointReport::new();
        
        for n in (10..=1000).step_by(10) {
            let model = generate_test_network(n);
            let tw = estimate_treewidth(&model);
            
            let start = Instant::now();
            let result = std::panic::catch_unwind(|| {
                model.query_with_algorithm(&[VariableId(0)], &EvidenceSet::new(), "junction_tree")
            });
            let elapsed = start.elapsed();
            
            if result.is_err() || elapsed > Duration::from_secs(60) {
                report.breakpoint = Some(Breakpoint {
                    node_count: n,
                    treewidth: tw,
                    reason: if result.is_err() {
                        "Out of memory".to_string()
                    } else {
                        "Timeout".to_string()
                    },
                });
                break;
            }
            
            report.add_data_point(n, tw, elapsed);
        }
        
        report
    }
}
```

---

## Profiling Tools

### CPU Profiling

```rust
#[cfg(feature = "profiling")]
pub struct CpuProfiler {
    sampler: Sampler,
}

#[cfg(feature = "profiling")]
impl CpuProfiler {
    pub fn profile_inference<F, T>(&self, f: F) -> (T, ProfileReport)
    where F: FnOnce() -> T {
        let guard = pprof::ProfilerGuard::new(100).unwrap();
        
        let result = f();
        
        let report = guard.report().build().unwrap();
        let profile_report = ProfileReport {
            top_functions: report.top_functions(20),
            flamegraph: report.flamegraph(),
        };
        
        (result, profile_report)
    }
}
```

### Memory Profiling

```rust
pub struct MemoryProfiler;

impl MemoryProfiler {
    pub fn profile<F, T>(f: F) -> (T, MemoryProfile)
    where F: FnOnce() -> T {
        // Enable allocator tracking
        let tracker = AllocatorTracker::enable();
        
        let result = f();
        
        let profile = MemoryProfile {
            allocations: tracker.allocations(),
            deallocations: tracker.deallocations(),
            peak_usage: tracker.peak_usage(),
            leaked: tracker.leaked(),
            hot_allocations: tracker.hotspots(10),
        };
        
        (result, profile)
    }
}
```

### I/O Profiling

```rust
pub struct IoProfiler;

impl IoProfiler {
    pub fn profile_file_operations<F, T>(path: &Path, f: F) -> (T, IoProfile)
    where F: FnOnce() -> T {
        use std::os::unix::fs::FileExt;
        
        // Wrap file operations to track
        let tracker = IoTracker::new();
        
        let result = f();
        
        let profile = IoProfile {
            bytes_read: tracker.bytes_read(),
            bytes_written: tracker.bytes_written(),
            read_operations: tracker.read_ops(),
            write_operations: tracker.write_ops(),
            sequential_ratio: tracker.sequential_ratio(),
        };
        
        (result, profile)
    }
}
```

### Identifying Bottlenecks

```rust
pub struct BottleneckAnalyzer;

impl BottleneckAnalyzer {
    pub fn analyze(profile: &CombinedProfile) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();
        
        // CPU bottlenecks
        if let Some(hot_function) = profile.cpu.hotspots.first() {
            if hot_function.percentage > 50.0 {
                bottlenecks.push(Bottleneck {
                    category: BottleneckCategory::Cpu,
                    description: format!("Hot function: {}", hot_function.name),
                    severity: Severity::Critical,
                    suggestion: "Consider optimizing this function or parallelizing".to_string(),
                });
            }
        }
        
        // Memory bottlenecks
        let allocation_rate = profile.memory.allocations as f64 / profile.duration.as_secs_f64();
        if allocation_rate > 1_000_000.0 {
            bottlenecks.push(Bottleneck {
                category: BottleneckCategory::Memory,
                description: format!("High allocation rate: {:.0} allocs/sec", allocation_rate),
                severity: Severity::Warning,
                suggestion: "Use memory pools or reduce allocations".to_string(),
            });
        }
        
        // I/O bottlenecks
        if profile.io.sequential_ratio < 0.5 {
            bottlenecks.push(Bottleneck {
                category: BottleneckCategory::Io,
                description: "Random I/O pattern detected".to_string(),
                severity: Severity::Warning,
                suggestion: "Consider sequential access patterns or caching".to_string(),
            });
        }
        
        bottlenecks
    }
}
```

---

## Benchmark Reporting

### Tables

```rust
pub struct BenchmarkTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl BenchmarkTable {
    pub fn from_results(results: &[BenchmarkResult]) -> Self {
        let headers = vec![
            "Benchmark".to_string(),
            "Mean".to_string(),
            "Std Dev".to_string(),
            "95% CI".to_string(),
            "Memory".to_string(),
        ];
        
        let rows = results.iter().map(|r| {
            vec![
                r.name.clone(),
                format!("{:.3} ms", r.mean.as_secs_f64() * 1000.0),
                format!("{:.3} ms", r.std_dev.as_secs_f64() * 1000.0),
                format!("[{:.3}, {:.3}] ms", 
                    r.confidence_interval.0.as_secs_f64() * 1000.0,
                    r.confidence_interval.1.as_secs_f64() * 1000.0),
                format!("{:.1} MB", r.memory_mb),
            ]
        }).collect();
        
        Self { headers, rows }
    }
    
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        
        // Header
        md.push_str("| ");
        md.push_str(&self.headers.join(" | "));
        md.push_str(" |\n");
        
        // Separator
        md.push_str("|");
        md.push_str(&self.headers.iter().map(|_| "---").join("|"));
        md.push_str("|\n");
        
        // Rows
        for row in &self.rows {
            md.push_str("| ");
            md.push_str(&row.join(" | "));
            md.push_str(" |\n");
        }
        
        md
    }
}
```

### Graphs

```rust
pub struct BenchmarkGraph;

impl BenchmarkGraph {
    pub fn scalability_curve(results: &[ScalabilityResult]) -> Plot {
        let x: Vec<f64> = results.iter().map(|r| r.problem_size as f64).collect();
        let y: Vec<f64> = results.iter()
            .map(|r| r.wall_time.as_secs_f64())
            .collect();
        
        Plot::new()
            .add_trace(Scatter::new(x, y)
                .mode(Mode::LinesMarkers)
                .name("Measured"))
            .x_axis(Axis::new().title("Problem Size (nodes)"))
            .y_axis(Axis::new().title("Time (seconds)").type_(AxisType::Log))
    }
    
    pub fn comparison_bar_chart(
        lutufi_results: &[f64],
        competitor_results: &[f64],
        labels: &[String]
    ) -> Plot {
        Plot::new()
            .add_trace(Bar::new(labels.to_vec(), lutufi_results.to_vec())
                .name("Lutufi"))
            .add_trace(Bar::new(labels.to_vec(), competitor_results.to_vec())
                .name("Competitor"))
            .layout(Layout::new()
                .bar_mode(BarMode::Group))
    }
}
```

### Machine-Readable Formats

```rust
#[derive(Serialize)]
pub struct BenchmarkReport {
    pub metadata: ReportMetadata,
    pub environment: EnvironmentInfo,
    pub results: Vec<BenchmarkResult>,
    pub comparisons: Vec<ComparisonResult>,
    pub regressions: Vec<Regression>,
}

impl BenchmarkReport {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    
    pub fn to_csv(&self) -> String {
        let mut csv = "benchmark,mean_ms,std_dev_ms,median_ms\n".to_string();
        
        for result in &self.results {
            csv.push_str(&format!("{},{:.3},{:.3},{:.3}\n",
                result.name,
                result.mean.as_secs_f64() * 1000.0,
                result.std_dev.as_secs_f64() * 1000.0,
                result.median.as_secs_f64() * 1000.0,
            ));
        }
        
        csv
    }
}
```

---

## Competitive Benchmarking

### Keeping Up with Other Libraries

```rust
pub struct CompetitiveAnalysis {
    pub libraries: Vec<LibraryBenchmarks>,
    pub test_suite: Vec<BenchmarkCase>,
}

pub struct LibraryBenchmarks {
    pub name: String,
    pub version: String,
    pub results: HashMap<String, BenchmarkResult>,
}

impl CompetitiveAnalysis {
    pub fn run_comparison(&self) -> ComparisonReport {
        let mut report = ComparisonReport::new();
        
        for case in &self.test_suite {
            let mut case_results = Vec::new();
            
            for lib in &self.libraries {
                let result = self.run_benchmark(&lib.name, case);
                case_results.push((lib.name.clone(), result));
            }
            
            // Find winner
            let winner = case_results.iter()
                .min_by(|(_, r1), (_, r2)| r1.mean.cmp(&r2.mean))
                .map(|(name, _)| name.clone())
                .unwrap();
            
            report.add_case_result(case.name.clone(), case_results, winner);
        }
        
        report
    }
}
```

### Understanding Where Lutufi Wins/Loses

```rust
pub struct CompetitiveAnalysisReport {
    pub wins: Vec<WinRecord>,
    pub losses: Vec<LossRecord>,
    pub ties: Vec<TieRecord>,
}

impl CompetitiveAnalysisReport {
    pub fn generate_insights(&self) -> Vec<Insight> {
        let mut insights = Vec::new();
        
        // Analyze win patterns
        if !self.wins.is_empty() {
            let avg_win_margin = self.wins.iter()
                .map(|w| w.speedup)
                .sum::<f64>() / self.wins.len() as f64;
            
            insights.push(Insight {
                category: "Strengths".to_string(),
                description: format!(
                    "Lutufi is {:.1}x faster on average in {} scenarios",
                    avg_win_margin, self.wins.len()
                ),
                examples: self.wins.iter().take(3).map(|w| w.scenario.clone()).collect(),
            });
        }
        
        // Analyze loss patterns
        if !self.losses.is_empty() {
            let common_loss_reasons = self.analyze_loss_reasons();
            
            insights.push(Insight {
                category: "Improvement Areas".to_string(),
                description: "Lutufi is slower in these scenarios".to_string(),
                examples: common_loss_reasons,
            });
        }
        
        insights
    }
}
```

---

## User-Facing Performance Metrics

### What Users Can Expect

```rust
pub struct PerformanceExpectations;

impl PerformanceExpectations {
    pub fn generate_report(tier: ScalabilityTier) -> UserPerformanceGuide {
        UserPerformanceGuide {
            tier: tier.clone(),
            max_recommended_nodes: tier.max_nodes,
            expected_inference_times: vec![
                ("Small model (100 nodes)", "< 10 ms"),
                ("Medium model (1,000 nodes)", "< 100 ms"),
                ("Large model (10,000 nodes)", "< 1 s"),
            ],
            memory_requirements: format!("~{} MB per 1000 nodes", tier.memory_per_1k_nodes),
            recommended_algorithms: tier.recommended_algorithms,
            hardware_recommendations: tier.hardware_recommendations,
        }
    }
}
```

### Performance Guarantees

```rust
pub struct PerformanceGuarantees;

impl PerformanceGuarantees {
    /// Worst-case time complexity guarantees
    pub fn time_complexity(algorithm: &InferenceAlgorithm) -> ComplexityGuarantee {
        match algorithm {
            InferenceAlgorithm::VariableElimination => ComplexityGuarantee {
                worst_case: "O(n * d^tw)".to_string(),
                average_case: "O(n * d^(tw/2))".to_string(),
                notes: "tw is treewidth, d is max domain size".to_string(),
            },
            InferenceAlgorithm::BeliefPropagation => ComplexityGuarantee {
                worst_case: "O(n * d^2 * iterations)".to_string(),
                average_case: "O(n * d * log(1/epsilon))".to_string(),
                notes: "Converges quickly for tree-like graphs".to_string(),
            },
            _ => ComplexityGuarantee::default(),
        }
    }
    
    /// Memory complexity guarantees
    pub fn memory_complexity(algorithm: &InferenceAlgorithm) -> ComplexityGuarantee {
        match algorithm {
            InferenceAlgorithm::JunctionTree => ComplexityGuarantee {
                worst_case: "O(d^tw)".to_string(),
                average_case: "O(d^(tw/2))".to_string(),
                notes: "Exponential in treewidth".to_string(),
            },
            InferenceAlgorithm::GibbsSampling => ComplexityGuarantee {
                worst_case: "O(n * d)".to_string(),
                average_case: "O(n)".to_string(),
                notes: "Linear in model size".to_string(),
            },
            _ => ComplexityGuarantee::default(),
        }
    }
}
```

### Tier Documentation

```rust
pub struct TierPerformanceDoc;

impl TierPerformanceDoc {
    pub fn generate_all() -> HashMap<ScalabilityTier, TierDoc> {
        let mut docs = HashMap::new();
        
        docs.insert(ScalabilityTier::Small, TierDoc {
            description: "Small networks suitable for interactive use".to_string(),
            examples: vec!["Medical diagnosis (Asia)", "Student grade prediction"],
            performance: "Exact inference < 100 ms".to_string(),
            algorithms: vec!["Variable Elimination", "Junction Tree", "Belief Propagation"],
            hardware: "Any modern computer".to_string(),
        });
        
        docs.insert(ScalabilityTier::Medium, TierDoc {
            description: "Medium networks for research and production".to_string(),
            examples: vec!["Alarm network", "Insurance models"],
            performance: "Exact inference < 1 s, approximate < 100 ms".to_string(),
            algorithms: vec!["Junction Tree (bounded)", "Loopy BP", "Gibbs Sampling"],
            hardware: "8+ GB RAM, multi-core CPU".to_string(),
        });
        
        docs.insert(ScalabilityTier::Large, TierDoc {
            description: "Large networks requiring careful algorithm selection".to_string(),
            examples: vec!["Social networks", "Gene regulatory networks"],
            performance: "Approximate inference < 5 min".to_string(),
            algorithms: vec!["Loopy BP", "Stochastic VI", "Distributed Sampling"],
            hardware: "32+ GB RAM, many-core CPU or cluster".to_string(),
        });
        
        docs
    }
}
```

---

## How Lutufi Benchmarks

### Implementation of Benchmark Suite

```rust
pub mod benchmark_suite {
    use criterion::{Criterion, BenchmarkId, black_box};
    
    pub fn run_all_benchmarks() {
        let mut criterion = Criterion::default()
            .sample_size(100)
            .measurement_time(Duration::from_secs(10));
        
        // Microbenchmarks
        microbenchmarks::register(&mut criterion);
        
        // Mesobenchmarks
        mesobenchmarks::register(&mut criterion);
        
        // Macrobenchmarks
        macrobenchmarks::register(&mut criterion);
        
        criterion.final_summary();
    }
    
    mod microbenchmarks {
        use super::*;
        
        pub fn register(c: &mut Criterion) {
            let mut group = c.benchmark_group("micro");
            
            group.bench_function("factor_multiply_5_vars", |b| {
                let f1 = DenseFactor::random(5, 3);
                let f2 = DenseFactor::random(5, 3);
                b.iter(|| black_box(f1.multiply(&f2)));
            });
            
            group.bench_function("cpd_lookup", |b| {
                let cpd = TabularCPD::random(5, 3);
                let parents = vec![0, 1, 0, 1];
                b.iter(|| black_box(cpd.get_probability(0, &parents)));
            });
            
            group.finish();
        }
    }
}
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench factor_multiply

# Save baseline
cargo bench -- --save-baseline main

# Compare to baseline
cargo bench -- --baseline main

# Generate report
cargo bench -- --format json > benchmark_results.json
```

### Continuous Benchmarking

```rust
// In CI pipeline
pub fn check_performance_regression() -> Result<(), RegressionError> {
    // Load baseline
    let baseline = BenchmarkHistory::load("baseline.json")?;
    
    // Run current benchmarks
    let current = run_benchmark_suite()?;
    
    // Compare
    let detector = RegressionDetector::new(baseline, 0.10);  // 10% threshold
    
    if let Some(report) = detector.detect_regression(&current) {
        eprintln!("Performance regression detected!");
        eprintln!("{}", report);
        return Err(RegressionError::Detected(report));
    }
    
    Ok(())
}
```

---

## Key References

### Benchmarking Methodology

1. **Fleming, P. J., & Wallace, J. J. (1986).** "How Not to Lie with Statistics: The Correct Way to Summarize Benchmark Results." *Communications of the ACM,* 29(3), 218-221.
   - Proper statistical treatment of benchmark results

2. **McGeoch, C. C. (2012).** *A Guide to Experimental Algorithmics.* Cambridge University Press.
   - Experimental methodology for algorithm analysis

3. **Hoefler, T., & Belli, R. (2015).** "Scientific Benchmarking of Parallel Computing Systems." *SC 2015.*
   - Rigorous benchmarking practices

4. **Beyer, B., et al. (2016).** *Site Reliability Engineering.* O'Reilly Media.
   - Monitoring and performance analysis

### Performance Testing

5. **Jain, R. (1991).** *The Art of Computer Systems Performance Analysis.* Wiley.
   - Comprehensive performance analysis methodology

6. **Gregg, B. (2013).** *Systems Performance: Enterprise and the Cloud.* Prentice Hall.
   - Systems performance analysis and profiling

7. **Schwartz, B. (2012).** *High Performance MySQL* (3rd ed.). O'Reilly Media.
   - Database benchmarking patterns (applicable concepts)

### Statistical Analysis

8. **Efron, B., & Tibshirani, R. J. (1994).** *An Introduction to the Bootstrap.* CRC Press.
   - Bootstrap methods for confidence intervals

9. **Box, G. E. P., Hunter, J. S., & Hunter, W. G. (2005).** *Statistics for Experimenters* (2nd ed.). Wiley.
   - Experimental design and analysis

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial draft |
| 1.0 | 2026-03-03 | Wasswa Lutufi Sebbanja | Complete performance benchmarking document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
