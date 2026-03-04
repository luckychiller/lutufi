# Lutufi Error Handling Philosophy Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Error Handling Philosophy](#error-handling-philosophy)
3. [Error Hierarchy](#error-hierarchy)
4. [Error Information](#error-information)
5. [Validation Errors](#validation-errors)
6. [Inference Errors](#inference-errors)
7. [Learning Errors](#learning-errors)
8. [Numerical Errors](#numerical-errors)
9. [Resource Errors](#resource-errors)
10. [User-Friendly Error Messages](#user-friendly-error-messages)
11. [Error Recovery Strategies](#error-recovery-strategies)
12. [Logging and Diagnostics](#logging-and-diagnostics)
13. [Debugging Support](#debugging-support)
14. [Error Reporting from Users](#error-reporting-from-users)
15. [Testing Error Conditions](#testing-error-conditions)
16. [Documentation of Errors](#documentation-of-errors)
17. [How Lutufi Handles Errors](#how-lutufi-handles-errors)
18. [Key References](#key-references)

---

## Executive Summary

Error handling is a critical aspect of the Lutufi library's design. As a library that bridges complex probabilistic algorithms with user applications, Lutufi must provide clear, actionable error information while maintaining system stability and offering paths to recovery. This document establishes the error handling philosophy, defines the comprehensive error hierarchy, and details implementation patterns for robust error management.

The error handling strategy is built on five pillars:

1. **Fail Fast**: Detect and report errors as early as possible
2. **Clear Communication**: Provide error messages that explain what happened, why it matters, and how to fix it
3. **Never Silent**: Never silently swallow errors or continue with undefined state
4. **Graceful Degradation**: When exact methods fail, offer approximate alternatives
5. **Actionable Recovery**: Provide specific suggestions and automated recovery when possible

---

## Error Handling Philosophy

### Fail Fast vs Graceful Degradation

Lutufi employs a nuanced approach balancing fail-fast with graceful degradation:

**Fail Fast (Default for Development):**
```rust
// Development mode: fail immediately with detailed information
let result = model.query(&["X"], &evidence, 
    QueryOptions::new()
        .fail_on_approximation(true)
        .fail_on_timeout(true)
        .fail_on_warning(true)
)?;
```

**Graceful Degradation (Production Mode):**
```rust
// Production mode: try alternatives automatically
let result = model.query(&["X"], &evidence,
    QueryOptions::new()
        .allow_approximation(true)
        .fallback_chain(vec![
            InferenceMethod::JunctionTree,
            InferenceMethod::BeliefPropagation,
            InferenceMethod::GibbsSampling,
        ])
        .timeout(Duration::from_secs(60))
)?;
```

**Decision Framework:**

| Scenario | Strategy | Rationale |
|----------|----------|-----------|
| Invalid input (syntax, type) | Fail Fast | User must fix input |
| Invalid model structure | Fail Fast | Undefined behavior risk |
| Treewidth too high for exact | Graceful → Approximate | Approximation may be sufficient |
| Memory exhausted | Graceful → Paging/Disk | May still complete |
| Timeout | Graceful → Partial results | Progress information valuable |
| Numerical instability | Fail Fast with details | Results would be unreliable |
| Convergence failure | Graceful → Alternative method | Different method may work |

### User-Centric Error Design

Every error is designed from the user's perspective:

**Before (System-Centric):**
```
Error: Matrix inversion failed
Code: ERR_LINAlg_001
```

**After (User-Centric):**
```
Inference failed: The conditional probability table for variable 'Disease' 
could not be inverted, which is required for exact inference on this query.

This typically occurs when:
- Evidence creates a deterministic dependency (P(Disease|Symptom) is 0 or 1)
- The model contains redundant variables

Suggested fixes:
1. Use approximate inference: model.query(..., algorithm='belief_propagation')
2. Remove evidence causing determinism
3. Check your CPD tables for structural zeros

For more information: https://lutufi.org/errors/inference/linear_algebra_failure
```

### Never Silently Swallow Errors

Every error condition must be explicitly handled:

```rust
// WRONG: Silently ignoring potential errors
fn load_model(path: &Path) -> Option<Model> {
    let data = std::fs::read(path).ok()?;  // Silent failure
    let model = bincode::deserialize(&data).ok()?;  // Silent failure
    Some(model)
}

// CORRECT: Explicit error handling
fn load_model(path: &Path) -> Result<Model, ModelLoadError> {
    let data = std::fs::read(path)
        .map_err(|e| ModelLoadError::FileRead {
            path: path.to_path_buf(),
            source: e,
        })?;
    
    let model = bincode::deserialize(&data)
        .map_err(|e| ModelLoadError::Deserialization {
            path: path.to_path_buf(),
            format: Format::Binary,
            source: e,
        })?;
    
    // Validate loaded model
    model.validate()
        .map_err(|e| ModelLoadError::Validation {
            path: path.to_path_buf(),
            validation_error: e,
        })?;
    
    Ok(model)
}
```

---

## Error Hierarchy

### Base Exception Class

```rust
/// The root of all Lutufi errors
#[derive(Debug)]
pub struct LutufiError {
    /// Error kind for programmatic handling
    pub kind: ErrorKind,
    
    /// Human-readable message
    pub message: String,
    
    /// Error context (chain of causation)
    pub context: ErrorContext,
    
    /// Suggestions for resolution
    pub suggestions: Vec<Suggestion>,
    
    /// Documentation link
    pub help_url: Option<String>,
    
    /// Error code for reference
    pub error_code: ErrorCode,
    
    /// Source error (if wrapping another error)
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    // Model-related
    Model,
    Validation,
    Structure,
    Cpd,
    
    // Inference-related
    Inference,
    Convergence,
    Intractability,
    Numerical,
    
    // Learning-related
    Learning,
    StructureLearning,
    ParameterLearning,
    
    // I/O and resources
    Io,
    Serialization,
    Resource,
    Timeout,
    
    // Configuration
    Configuration,
    NotImplemented,
    Internal,
}

pub type ErrorCode = String;  // e.g., "LUT-INF-001"
```

### Error Hierarchy Structure

```
LutufiError (base)
├── ModelError
│   ├── StructureError
│   │   ├── CycleError
│   │   ├── DisconnectedGraphError
│   │   └── InvalidEdgeError
│   ├── CpdError
│   │   ├── InvalidDimensionsError
│   │   ├── NonNormalizedError
│   │   ├── InvalidProbabilityError
│   │   └── MissingCpdError
│   └── ValidationError
│       ├── InvalidVariableError
│       ├── InvalidEvidenceError
│       └── InconsistentStateError
├── InferenceError
│   ├── ConvergenceError
│   │   ├── MaxIterationsError
│   │   ├── OscillationError
│   │   └── DivergenceError
│   ├── IntractabilityError
│   │   ├── TreewidthExceededError
│   │   ├── MemoryLimitError
│   │   └── TimeLimitError
│   ├── AlgorithmError
│   │   ├── UnsupportedQueryError
│   │   ├── InvalidAlgorithmError
│   │   └── AlgorithmFailureError
│   └── NumericalError
│       ├── UnderflowError
│       ├── OverflowError
│       ├── SingularMatrixError
│       └── PrecisionLossError
├── LearningError
│   ├── StructureLearningError
│   │   ├── InsufficientDataError
│   │   ├── IdentifiabilityError
│   │   └── LocalOptimaError
│   └── ParameterLearningError
│       ├── ZeroVarianceError
│       ├── EmptyParentConfigError
│       └── PriorConflictError
├── IoError
│   ├── FileError
│   │   ├── FileNotFoundError
│   │   ├── PermissionError
│   │   └── CorruptedFileError
│   ├── SerializationError
│   │   ├── IncompatibleVersionError
│   │   ├── InvalidFormatError
│   │   └── SchemaMismatchError
│   └── NetworkError
│       ├── ConnectionError
│       └── TimeoutError
└── ResourceError
    ├── OutOfMemoryError
    ├── DiskSpaceError
    └── CpuLimitError
```

### Rust Implementation

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Invalid graph structure: {message}")]
    Structure { message: String, details: StructureDetails },
    
    #[error("Invalid CPD for variable '{variable}': {message}")]
    Cpd { variable: String, message: String, cpd_error: CpdErrorKind },
    
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),
}

#[derive(Error, Debug)]
pub enum InferenceError {
    #[error("Algorithm '{algorithm}' failed to converge after {iterations} iterations")]
    ConvergenceFailure { algorithm: String, iterations: usize, diagnostics: ConvergenceDiagnostics },
    
    #[error("Query intractable: treewidth {treewidth} exceeds limit {limit}")]
    IntractableQuery { treewidth: usize, limit: usize, suggestions: Vec<Suggestion> },
    
    #[error("Numerical error: {kind}")]
    Numerical { kind: NumericalErrorKind, operation: String, values: Vec<f64> },
    
    #[error("Inference algorithm '{algorithm}' failed: {message}")]
    AlgorithmFailure { algorithm: String, message: String, cause: Option<String> },
}

#[derive(Error, Debug)]
pub enum LearningError {
    #[error("Structure learning failed: {message}")]
    StructureLearning { message: String, data_stats: DataStatistics },
    
    #[error("Parameter learning failed for '{variable}': {message}")]
    ParameterLearning { variable: String, message: String },
    
    #[error("Insufficient data: {message}")]
    InsufficientData { message: String, required: usize, available: usize },
}

#[derive(Error, Debug)]
pub enum IoError {
    #[error("File error: {path}: {source}")]
    File { path: PathBuf, #[source] source: std::io::Error },
    
    #[error("Serialization error: {message}")]
    Serialization { message: String, format: Format, position: Option<usize> },
    
    #[error("Incompatible version: file version {file_version}, library version {library_version}")]
    IncompatibleVersion { file_version: String, library_version: String },
}
```

---

## Error Information

### What Errors Must Include

Every Lutufi error includes:

```rust
pub struct ErrorInfo {
    // 1. Message: What happened?
    pub message: String,
    
    // 2. Context: Where did it happen?
    pub context: ErrorContext,
    
    // 3. Why: Why did it happen?
    pub explanation: String,
    
    // 4. Fix: How to fix it?
    pub suggestions: Vec<Suggestion>,
    
    // 5. Learn: Where to learn more?
    pub documentation_url: Option<String>,
    
    // 6. Code: For programmatic handling
    pub error_code: ErrorCode,
    
    // 7. Severity: How serious is it?
    pub severity: ErrorSeverity,
}

pub struct ErrorContext {
    pub operation: String,
    pub model_state: Option<ModelSnapshot>,
    pub variable_id: Option<VariableId>,
    pub query: Option<Query>,
    pub stack_trace: Vec<StackFrame>,
    pub timestamp: DateTime<Utc>,
}

pub struct Suggestion {
    pub description: String,
    pub code_example: Option<String>,
    pub automatic_fix: Option<Box<dyn Fn() -> Result<(), Error>>>,
}

pub enum ErrorSeverity {
    Warning,    // Operation completed but may have issues
    Error,      // Operation failed but system is stable
    Critical,   // System may be in inconsistent state
    Fatal,      // Cannot continue, requires restart
}
```

### Error Code System

```rust
// Error code format: LUT-[CATEGORY]-[NUMBER]
// Examples:
// LUT-MOD-001: Model structure error
// LUT-INF-042: Inference convergence failure
// LUT-LRN-015: Insufficient data for learning

pub struct ErrorCode {
    prefix: &'static str,  // "LUT"
    category: ErrorCategory,
    number: u16,
}

pub enum ErrorCategory {
    MOD,  // Model
    INF,  // Inference
    LRN,  // Learning
    IO,   // I/O
    CFG,  // Configuration
    INT,  // Internal
}

impl ErrorCode {
    pub fn new(category: ErrorCategory, number: u16) -> Self {
        Self { prefix: "LUT", category, number }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}-{:?}-{:03}", self.prefix, self.category, self.number)
    }
}
```

### Contextual Information

```rust
impl LutufiError {
    pub fn with_context(mut self, context: impl Into<ErrorContext>) -> Self {
        self.context = context.into();
        self
    }
    
    pub fn with_suggestion(mut self, suggestion: Suggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }
    
    pub fn with_help_url(mut self, url: impl Into<String>) -> Self {
        self.help_url = Some(url.into());
        self
    }
}

// Usage
let error = LutufiError::new(ErrorKind::Inference, "Convergence failed")
    .with_context(InferenceContext {
        algorithm: "loopy_belief_propagation",
        iterations: 1000,
        convergence_delta: 0.5,
    })
    .with_suggestion(Suggestion::new(
        "Try increasing damping",
        Some("model.query(..., damping=0.5)"),
        None,
    ))
    .with_suggestion(Suggestion::new(
        "Use a different algorithm",
        Some("model.query(..., algorithm='gibbs_sampling')"),
        None,
    ))
    .with_help_url("https://lutufi.org/errors/convergence");
```

---

## Validation Errors

### Input Validation

```rust
pub struct InputValidator;

impl InputValidator {
    pub fn validate_variable_name(name: &str) -> Result<(), ValidationError> {
        if name.is_empty() {
            return Err(ValidationError::invalid_input(
                "Variable name cannot be empty"
            ).with_suggestion("Use a descriptive name like 'Temperature' or 'HasDisease'"));
        }
        
        if name.starts_with(|c: char| c.is_numeric()) {
            return Err(ValidationError::invalid_input(
                format!("Variable name '{}' cannot start with a number", name)
            ));
        }
        
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(ValidationError::invalid_input(
                format!("Variable name '{}' contains invalid characters. Use only letters, numbers, and underscores", name)
            ));
        }
        
        if RESERVED_KEYWORDS.contains(name) {
            return Err(ValidationError::invalid_input(
                format!("'{}' is a reserved keyword and cannot be used as a variable name", name)
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_evidence(
        variable: &Variable,
        value: &Value
    ) -> Result<(), ValidationError> {
        match (&variable.domain, value) {
            (Domain::Discrete { values }, Value::Discrete(v)) => {
                if !values.contains(v) {
                    return Err(ValidationError::invalid_evidence(
                        variable.name.clone(),
                        format!("Value '{}' not in domain {:?}", v, values)
                    ).with_suggestion(format!(
                        "Valid values are: {}",
                        values.join(", ")
                    )));
                }
            }
            (Domain::Continuous { lower, upper }, Value::Continuous(v)) => {
                if let Some(lo) = lower {
                    if v < lo {
                        return Err(ValidationError::out_of_range(
                            variable.name.clone(),
                            *v,
                            *lo,
                            upper.unwrap_or(f64::INFINITY)
                        ));
                    }
                }
                if let Some(hi) = upper {
                    if v > hi {
                        return Err(ValidationError::out_of_range(
                            variable.name.clone(),
                            *v,
                            lower.unwrap_or(f64::NEG_INFINITY),
                            *hi
                        ));
                    }
                }
            }
            (expected, actual) => {
                return Err(ValidationError::type_mismatch(
                    variable.name.clone(),
                    format!("{:?}", expected),
                    format!("{:?}", actual)
                ));
            }
        }
        
        Ok(())
    }
}
```

### Model Validation

```rust
pub struct ModelValidator;

impl ModelValidator {
    pub fn validate_bayesian_network(model: &BayesianNetwork) -> Result<(), ModelValidationError> {
        let mut errors = Vec::new();
        
        // 1. Check for cycles
        if let Some(cycle) = Self::detect_cycle(model) {
            errors.push(ModelValidationError::Cycle {
                cycle: cycle.clone(),
                suggestion: format!("Remove one of these edges to break the cycle: {:?}", 
                    cycle.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>()),
            });
        }
        
        // 2. Check for missing CPDs
        for node in model.nodes() {
            if !model.has_cpd(node) {
                errors.push(ModelValidationError::MissingCpd {
                    variable: node.clone(),
                    suggestion: format!("Add CPD: model.set_cpd('{}', [...])", node),
                });
            }
        }
        
        // 3. Check CPD dimensions
        for node in model.nodes() {
            if let Some(cpd) = model.get_cpd(node) {
                let expected_shape = Self::expected_cpd_shape(model, node);
                if cpd.shape() != expected_shape {
                    errors.push(ModelValidationError::CpdDimensionMismatch {
                        variable: node.clone(),
                        expected: expected_shape,
                        actual: cpd.shape(),
                    });
                }
            }
        }
        
        // 4. Check CPD normalization
        for node in model.nodes() {
            if let Some(cpd) = model.get_cpd(node) {
                if let Some(non_normalized) = Self::check_normalization(cpd) {
                    errors.push(ModelValidationError::NonNormalizedCpd {
                        variable: node.clone(),
                        parent_config: non_normalized.config_index,
                        sum: non_normalized.sum,
                        suggestion: "Use normalize=True when setting CPD".to_string(),
                    });
                }
            }
        }
        
        // 5. Check for isolated nodes
        for node in model.nodes() {
            if model.degree(node) == 0 && model.nodes().len() > 1 {
                errors.push(ModelValidationError::IsolatedNode {
                    variable: node.clone(),
                    suggestion: format!("Connect '{}' to other variables or remove it", node),
                });
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(ModelValidationError::Multiple(errors))
        }
    }
}
```

### Data Validation

```rust
pub struct DataValidator;

impl DataValidator {
    pub fn validate_learning_data(
        data: &DataFrame,
        model: Option<&BayesianNetwork>
    ) -> Result<DataValidationReport, DataValidationError> {
        let mut report = DataValidationReport::new();
        
        // Check for empty data
        if data.is_empty() {
            return Err(DataValidationError::EmptyData);
        }
        
        // Check for missing values
        let missing_summary = data.null_count();
        for (column, count) in missing_summary.iter() {
            if *count > 0 {
                let ratio = *count as f64 / data.height() as f64;
                if ratio > 0.5 {
                    report.add_warning(DataWarning::HighMissingRate {
                        column: column.clone(),
                        missing_ratio: ratio,
                        suggestion: "Consider imputation or removing this variable".to_string(),
                    });
                }
            }
        }
        
        // Check for low variance
        for column in data.get_columns() {
            if column.n_unique()? < 2 {
                report.add_warning(DataWarning::ZeroVariance {
                    column: column.name().to_string(),
                    suggestion: "Remove constant variables from learning".to_string(),
                });
            }
        }
        
        // Check for sufficient samples per configuration
        if let Some(model) = model {
            for node in model.nodes() {
                let parents = model.get_parents(node);
                let parent_configs = Self::count_parent_configurations(data, &parents);
                
                for (config, count) in parent_configs {
                    if count < MIN_SAMPLES_PER_CONFIG {
                        report.add_warning(DataWarning::InsufficientSamples {
                            variable: node.clone(),
                            parent_config: config,
                            sample_count: count,
                            minimum_required: MIN_SAMPLES_PER_CONFIG,
                        });
                    }
                }
            }
        }
        
        // Check data types against model
        if let Some(model) = model {
            for node in model.nodes() {
                if let Some(column) = data.column(&node) {
                    let variable = model.get_variable(node).unwrap();
                    if let Err(e) = Self::check_type_compatibility(column, variable) {
                        report.add_error(e);
                    }
                } else {
                    report.add_error(DataValidationError::MissingColumn {
                        column: node.clone(),
                        available: data.get_column_names(),
                    });
                }
            }
        }
        
        if report.has_errors() {
            Err(DataValidationError::from_report(report))
        } else {
            Ok(report)
        }
    }
}
```

### Early Detection Strategies

```rust
pub struct EarlyDetector;

impl EarlyDetector {
    // Detect potential issues before they become errors
    pub fn check_inference_feasibility(
        model: &FactorGraph,
        query: &Query
    ) -> FeasibilityReport {
        let mut warnings = Vec::new();
        
        // 1. Treewidth check
        let tw_estimate = estimate_treewidth(model);
        if tw_estimate.lower_bound > 30 {
            warnings.push(FeasibilityWarning::HighTreewidth {
                estimate: tw_estimate,
                suggestion: "Consider approximate inference methods".to_string(),
            });
        }
        
        // 2. Memory estimate
        let memory_estimate = estimate_memory_requirements(model, query);
        let available_memory = get_available_memory();
        if memory_estimate > available_memory {
            warnings.push(FeasibilityWarning::MemoryIntensive {
                required_gb: memory_estimate as f64 / 1e9,
                available_gb: available_memory as f64 / 1e9,
                suggestion: "Enable memory-mapped files or use approximate methods".to_string(),
            });
        }
        
        // 3. Numerical stability check
        for factor in model.factors() {
            if Self::has_extreme_values(factor) {
                warnings.push(FeasibilityWarning::NumericalRisk {
                    factor_id: factor.id(),
                    suggestion: "Consider log-space computation or value clamping".to_string(),
                });
            }
        }
        
        // 4. Convergence prediction for iterative methods
        if model.has_loops() && !model.is_singly_connected() {
            let convergence_prediction = predict_convergence_difficulty(model);
            if convergence_prediction.difficulty > 0.7 {
                warnings.push(FeasibilityWarning::ConvergenceRisk {
                    difficulty: convergence_prediction.difficulty,
                    suggestion: "Use damping or consider sampling methods".to_string(),
                });
            }
        }
        
        FeasibilityReport { warnings }
    }
}
```

---

## Inference Errors

### Convergence Failures

```rust
#[derive(Error, Debug)]
pub enum ConvergenceError {
    #[error("Belief propagation failed to converge after {max_iterations} iterations")]
    BeliefPropagationDivergence {
        max_iterations: usize,
        final_delta: f64,
        oscillation_detected: bool,
        suggestions: Vec<Suggestion>,
    },
    
    #[error("Gibbs sampler stuck in local mode after {iterations} iterations")]
    GibbsSamplerStuck {
        iterations: usize,
        stuck_variables: Vec<VariableId>,
        estimated_mixing_time: usize,
    },
    
    #[error("Variational inference ELBO not improving")]
    VariationalStagnation {
        iterations_without_improvement: usize,
        best_elbo: f64,
        current_elbo: f64,
    },
}

impl ConvergenceError {
    pub fn with_suggestions(self) -> Self {
        match &self {
            ConvergenceError::BeliefPropagationDivergence { oscillation_detected, .. } => {
                let mut suggestions = vec![
                    Suggestion::new(
                        "Increase damping factor",
                        Some("model.query(..., damping=0.5)"),
                        None,
                    ),
                    Suggestion::new(
                        "Try different message schedule",
                        Some("model.query(..., schedule='residual')"),
                        None,
                    ),
                ];
                
                if *oscillation_detected {
                    suggestions.push(Suggestion::new(
                        "Oscillations detected - use double-loop algorithm",
                        Some("model.query(..., algorithm='generalized_bp')"),
                        None,
                    ));
                }
                
                suggestions.push(Suggestion::new(
                    "Switch to sampling method",
                    Some("model.query(..., algorithm='gibbs_sampling')"),
                    None,
                ));
                
                // Update with suggestions
                self
            }
            _ => self,
        }
    }
}
```

### Numerical Instability

```rust
#[derive(Error, Debug)]
pub enum NumericalError {
    #[error("Underflow in {operation}: values too small to represent")]
    Underflow {
        operation: String,
        min_value: f64,
        threshold: f64,
        location: CodeLocation,
    },
    
    #[error("Overflow in {operation}: values too large to represent")]
    Overflow {
        operation: String,
        max_value: f64,
        threshold: f64,
        location: CodeLocation,
    },
    
    #[error("Singular matrix in {operation}")]
    SingularMatrix {
        operation: String,
        matrix_info: MatrixInfo,
        remedy: String,
    },
    
    #[error("Precision loss detected: {details}")]
    PrecisionLoss {
        details: String,
        original_precision: u32,
        effective_precision: u32,
    },
}

impl NumericalError {
    pub fn remedy(&self) -> NumericalRemedy {
        match self {
            NumericalError::Underflow { .. } => NumericalRemedy::UseLogSpace,
            NumericalError::Overflow { .. } => NumericalRemedy::NormalizeIntermediate,
            NumericalError::SingularMatrix { .. } => NumericalRemedy::AddRegularization,
            NumericalError::PrecisionLoss { .. } => NumericalRemedy::IncreasePrecision,
        }
    }
}

// Automatic numerical stability handling
pub struct NumericalStabilityGuard {
    use_log_space: bool,
    check_interval: usize,
    operation_count: usize,
}

impl NumericalStabilityGuard {
    pub fn check(&mut self, values: &[f64]) -> Result<(), NumericalError> {
        self.operation_count += 1;
        
        if self.operation_count % self.check_interval != 0 {
            return Ok(());
        }
        
        let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        if min_val < UNDERFLOW_THRESHOLD && !self.use_log_space {
            return Err(NumericalError::Underflow {
                operation: "factor_computation".to_string(),
                min_value: min_val,
                threshold: UNDERFLOW_THRESHOLD,
                location: CodeLocation::current(),
            });
        }
        
        if max_val > OVERFLOW_THRESHOLD {
            return Err(NumericalError::Overflow {
                operation: "factor_computation".to_string(),
                max_value: max_val,
                threshold: OVERFLOW_THRESHOLD,
                location: CodeLocation::current(),
            });
        }
        
        Ok(())
    }
}
```

### Algorithm Selection Failures

```rust
#[derive(Error, Debug)]
pub enum AlgorithmSelectionError {
    #[error("No algorithm available for query type '{query_type}' on this model")]
    UnsupportedQuery {
        query_type: String,
        model_characteristics: ModelCharacteristics,
        available_algorithms: Vec<String>,
    },
    
    #[error("Algorithm '{algorithm}' not suitable: {reason}")]
    AlgorithmMismatch {
        algorithm: String,
        reason: String,
        alternatives: Vec<String>,
    },
    
    #[error("All algorithms in fallback chain failed")]
    FallbackExhausted {
        tried_algorithms: Vec<(String, AlgorithmFailureReason)>,
        final_error: Box<dyn std::error::Error>,
    },
}

pub struct AlgorithmSelector {
    fallback_chain: Vec<InferenceAlgorithm>,
    current_index: usize,
}

impl AlgorithmSelector {
    pub fn select_and_run(
        &mut self,
        model: &FactorGraph,
        query: &Query,
    ) -> Result<QueryResult, AlgorithmSelectionError> {
        let mut failures = Vec::new();
        
        for (i, algorithm) in self.fallback_chain[self.current_index..].iter().enumerate() {
            match self.try_algorithm(algorithm, model, query) {
                Ok(result) => {
                    self.current_index = i;
                    return Ok(result);
                }
                Err(e) => {
                    failures.push((algorithm.name(), e));
                    
                    // Log failure but continue to next algorithm
                    log::warn!("Algorithm {} failed: {}", algorithm.name(), e);
                }
            }
        }
        
        Err(AlgorithmSelectionError::FallbackExhausted {
            tried_algorithms: failures,
            final_error: Box::new(failures.last().unwrap().1.clone()),
        })
    }
}
```

---

## Learning Errors

### Insufficient Data

```rust
#[derive(Error, Debug)]
pub enum DataSufficiencyError {
    #[error("Insufficient data for variable '{variable}': {available} samples, need {required}")]
    InsufficientSamples {
        variable: String,
        available: usize,
        required: usize,
        context: String,
    },
    
    #[error("Empty parent configuration for '{variable}': no observations with parent values {parent_config:?}")]
    EmptyParentConfiguration {
        variable: String,
        parent_config: Vec<(String, Value)>,
        total_configurations: usize,
        non_empty_configurations: usize,
    },
    
    #[error("Data does not span the domain of '{variable}'")]
    IncompleteDomainCoverage {
        variable: String,
        observed_values: Vec<Value>,
        domain_values: Vec<Value>,
        missing_values: Vec<Value>,
    },
}

impl DataSufficiencyError {
    pub fn suggest_data_augmentation(&self) -> Vec<Suggestion> {
        match self {
            DataSufficiencyError::InsufficientSamples { required, available, .. } => {
                let ratio = *available as f64 / *required as f64;
                if ratio > 0.5 {
                    vec![
                        Suggestion::new(
                            "Use Bayesian parameter learning with strong prior",
                            Some("model.fit(data, algorithm='bayesian', prior='bdeu', equivalent_sample_size=10)"),
                            None,
                        ),
                        Suggestion::new(
                            "Collect more data or reduce model complexity",
                            None,
                            None,
                        ),
                    ]
                } else {
                    vec![
                        Suggestion::new(
                            "Use expert-specified parameters instead of learning",
                            None,
                            None,
                        ),
                        Suggestion::new(
                            "Drastically reduce model complexity",
                            None,
                            None,
                        ),
                    ]
                }
            }
            _ => vec![],
        }
    }
}
```

### Identifiability Issues

```rust
#[derive(Error, Debug)]
pub enum IdentifiabilityError {
    #[error("Model is not identifiable: {reason}")]
    NonIdentifiable {
        reason: String,
        non_identifiable_parameters: Vec<String>,
        suggested_constraints: Vec<String>,
    },
    
    #[error("Equivalent models exist: multiple structures fit the data equally well")]
    EquivalentModels {
        equivalent_structures: Vec<Graph>,
        score_difference: f64,
    },
    
    #[error("Local unidentifiability near current parameters")]
    LocalUnidentifiability {
        parameter: String,
        gradient_magnitude: f64,
        hessian_condition_number: f64,
    },
}

pub struct IdentifiabilityChecker;

impl IdentifiabilityChecker {
    pub fn check_structure_identifiability(
        model: &BayesianNetwork,
        data: &DataFrame
    ) -> Result<IdentifiabilityReport, IdentifiabilityError> {
        // Check for structural unidentifiability
        let independence_relations = find_independence_relations(model, data);
        let v_structures = find_v_structures(model);
        
        // Check if v-structures are identifiable from data
        for v_struct in v_structures {
            if !Self::is_v_structure_identifiable(&v_struct, &independence_relations) {
                return Err(IdentifiabilityError::NonIdentifiable {
                    reason: format!("V-structure at {} not identifiable", v_struct.center),
                    non_identifiable_parameters: vec![format!("Edge directions around {}", v_struct.center)],
                    suggested_constraints: vec![
                        format!("Add prior knowledge about edge direction near {}", v_struct.center),
                        "Use interventional data".to_string(),
                    ],
                });
            }
        }
        
        Ok(IdentifiabilityReport::new())
    }
}
```

### Local Optima Detection

```rust
#[derive(Error, Debug)]
pub enum OptimizationError {
    #[error("Stuck in local optimum after {iterations} iterations")]
    LocalOptimum {
        iterations: usize,
        score: f64,
        best_known_score: Option<f64>,
        plateau_length: usize,
    },
    
    #[error("Score oscillation detected: not converging")]
    ScoreOscillation {
        recent_scores: Vec<f64>,
        oscillation_amplitude: f64,
    },
    
    #[error("Gradient vanished: cannot continue optimization")]
    VanishingGradient {
        gradient_norm: f64,
        layer: Option<String>,
    },
}

pub struct LocalOptimaDetector {
    score_history: Vec<f64>,
    patience: usize,
    tolerance: f64,
}

impl LocalOptimaDetector {
    pub fn check(&mut self, current_score: f64) -> Option<OptimizationError> {
        self.score_history.push(current_score);
        
        // Check for plateau
        if self.score_history.len() >= self.patience {
            let recent = &self.score_history[self.score_history.len()-self.patience..];
            let max_change = recent.iter()
                .zip(recent.iter().skip(1))
                .map(|(a, b)| (b - a).abs())
                .fold(0.0, f64::max);
            
            if max_change < self.tolerance {
                return Some(OptimizationError::LocalOptimum {
                    iterations: self.score_history.len(),
                    score: current_score,
                    best_known_score: self.score_history.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()),
                    plateau_length: self.patience,
                });
            }
        }
        
        // Check for oscillation
        if self.score_history.len() >= 10 {
            let recent = &self.score_history[self.score_history.len()-10..];
            let mean = recent.iter().sum::<f64>() / recent.len() as f64;
            let variance = recent.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / recent.len() as f64;
            
            if variance > self.tolerance * 100.0 {
                return Some(OptimizationError::ScoreOscillation {
                    recent_scores: recent.to_vec(),
                    oscillation_amplitude: variance.sqrt(),
                });
            }
        }
        
        None
    }
}
```

---

## Numerical Errors

### Underflow/Overflow Detection

```rust
pub struct NumericalMonitor {
    underflow_count: AtomicUsize,
    overflow_count: AtomicUsize,
    min_observed: AtomicF64,
    max_observed: AtomicF64,
}

impl NumericalMonitor {
    pub fn check_value(&self, value: f64, context: &str) -> Result<f64, NumericalError> {
        // Update statistics
        self.update_stats(value);
        
        // Check for special values
        if value.is_nan() {
            return Err(NumericalError::InvalidValue {
                value: "NaN".to_string(),
                context: context.to_string(),
            });
        }
        
        if value.is_infinite() {
            if value.is_sign_positive() {
                self.overflow_count.fetch_add(1, Ordering::Relaxed);
                return Err(NumericalError::Overflow {
                    operation: context.to_string(),
                    max_value: value,
                    threshold: f64::MAX,
                    location: CodeLocation::current(),
                });
            } else {
                return Err(NumericalError::Underflow {
                    operation: context.to_string(),
                    min_value: value,
                    threshold: f64::MIN_POSITIVE,
                    location: CodeLocation::current(),
                });
            }
        }
        
        // Check for near-underflow
        if value > 0.0 && value < UNDERFLOW_WARNING_THRESHOLD {
            let count = self.underflow_count.fetch_add(1, Ordering::Relaxed);
            if count == UNDERFLOW_WARNING_COUNT {
                log::warn!(
                    "Numerical underflow detected in {}. Consider using log-space computation.",
                    context
                );
            }
        }
        
        // Check for near-overflow
        if value > OVERFLOW_WARNING_THRESHOLD {
            let count = self.overflow_count.fetch_add(1, Ordering::Relaxed);
            if count == OVERFLOW_WARNING_COUNT {
                log::warn!(
                    "Numerical overflow risk in {}. Consider normalization.",
                    context
                );
            }
        }
        
        Ok(value)
    }
}
```

### NaN/Inf Handling

```rust
pub fn safe_divide(a: f64, b: f64, context: &str) -> Result<f64, NumericalError> {
    if b == 0.0 {
        return Err(NumericalError::DivisionByZero {
            numerator: a,
            context: context.to_string(),
        });
    }
    
    let result = a / b;
    
    if result.is_nan() {
        return Err(NumericalError::InvalidOperation {
            operation: format!("{} / {}", a, b),
            context: context.to_string(),
            result: "NaN".to_string(),
        });
    }
    
    if result.is_infinite() {
        return Err(NumericalError::Overflow {
            operation: context.to_string(),
            max_value: result,
            threshold: f64::MAX,
            location: CodeLocation::current(),
        });
    }
    
    Ok(result)
}

pub fn safe_log(x: f64, context: &str) -> Result<f64, NumericalError> {
    if x <= 0.0 {
        return Err(NumericalError::InvalidLog {
            value: x,
            context: context.to_string(),
        });
    }
    
    let result = x.ln();
    
    if result.is_infinite() && x.is_finite() {
        // This shouldn't happen for positive finite x
        return Err(NumericalError::UnexpectedInfinity {
            operation: format!("log({})", x),
            context: context.to_string(),
        });
    }
    
    Ok(result)
}
```

### Precision Loss Warnings

```rust
pub struct PrecisionTracker {
    original_values: Vec<f64>,
    current_values: Vec<f64>,
}

impl PrecisionTracker {
    pub fn check_precision_loss(&self) -> Option<PrecisionLossWarning> {
        if self.original_values.len() != self.current_values.len() {
            return None;
        }
        
        let mut max_relative_error = 0.0;
        let mut max_absolute_error = 0.0;
        
        for (orig, curr) in self.original_values.iter().zip(&self.current_values) {
            let abs_error = (orig - curr).abs();
            let rel_error = if *orig != 0.0 {
                abs_error / orig.abs()
            } else {
                0.0
            };
            
            max_absolute_error = max_absolute_error.max(abs_error);
            max_relative_error = max_relative_error.max(rel_error);
        }
        
        if max_relative_error > PRECISION_LOSS_THRESHOLD {
            return Some(PrecisionLossWarning {
                max_relative_error,
                max_absolute_error,
                affected_values: self.count_affected_values(),
                suggestion: "Consider using higher precision (f128) or log-space computation",
            });
        }
        
        None
    }
}
```

---

## Resource Errors

### Out of Memory

```rust
#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Out of memory: requested {requested_gb:.2} GB, available {available_gb:.2} GB")]
    OutOfMemory {
        requested: usize,
        available: usize,
        allocation_context: String,
        suggestions: Vec<Suggestion>,
    },
    
    #[error("Memory limit exceeded: using {current_gb:.2} GB, limit {limit_gb:.2} GB")]
    MemoryLimitExceeded {
        current: usize,
        limit: usize,
        allocation_context: String,
    },
}

pub struct MemoryManager {
    soft_limit: usize,
    hard_limit: usize,
    current_usage: AtomicUsize,
    allocator: Box<dyn MemoryAllocator>,
}

impl MemoryManager {
    pub fn allocate(&self, size: usize, context: &str) -> Result<Allocation, MemoryError> {
        let current = self.current_usage.load(Ordering::Relaxed);
        let projected = current + size;
        
        // Check hard limit
        if projected > self.hard_limit {
            return Err(MemoryError::OutOfMemory {
                requested: size,
                available: self.hard_limit - current,
                allocation_context: context.to_string(),
                suggestions: vec![
                    Suggestion::new(
                        "Use approximate inference methods",
                        Some("model.query(..., algorithm='belief_propagation')"),
                        None,
                    ),
                    Suggestion::new(
                        "Enable memory-mapped storage",
                        Some("lutufi.config.set('memory.use_mmap', True)"),
                        None,
                    ),
                    Suggestion::new(
                        "Reduce batch size",
                        Some("model.query(..., batch_size=1000)"),
                        None,
                    ),
                    Suggestion::new(
                        "Use a machine with more RAM",
                        None,
                        None,
                    ),
                ],
            });
        }
        
        // Check soft limit
        if projected > self.soft_limit {
            log::warn!(
                "Memory usage ({:.2} GB) approaching soft limit ({:.2} GB)",
                projected as f64 / 1e9,
                self.soft_limit as f64 / 1e9
            );
            
            // Trigger garbage collection or cache eviction
            self.reduce_memory_pressure();
        }
        
        // Perform allocation
        let allocation = self.allocator.allocate(size)
            .map_err(|e| MemoryError::OutOfMemory {
                requested: size,
                available: 0,
                allocation_context: context.to_string(),
                suggestions: vec![Suggestion::new(
                    "System out of memory - close other applications",
                    None,
                    None,
                )],
            })?;
        
        self.current_usage.fetch_add(size, Ordering::Relaxed);
        
        Ok(allocation)
    }
    
    fn reduce_memory_pressure(&self) {
        // 1. Clear LRU caches
        // 2. Evict cold data to disk
        // 3. Request garbage collection
        // 4. Compress in-memory data
    }
}
```

### File System Errors

```rust
#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf, searched_paths: Vec<PathBuf> },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf, required: Permissions, available: Permissions },
    
    #[error("Disk full: {path}. Need {required_gb:.2} GB, available {available_gb:.2} GB")]
    DiskFull { path: PathBuf, required: usize, available: usize },
    
    #[error("File corrupted: {path}")]
    FileCorrupted { path: PathBuf, checksum_expected: String, checksum_actual: String },
    
    #[error("I/O error: {path}: {source}")]
    Io { path: PathBuf, #[source] source: std::io::Error },
}

impl FileSystemError {
    pub fn suggestion(&self) -> Suggestion {
        match self {
            FileSystemError::FileNotFound { searched_paths, .. } => Suggestion::new(
                format!("File not found in: {:?}", searched_paths),
                None,
                None,
            ),
            FileSystemError::PermissionDenied { required, .. } => Suggestion::new(
                format!("Need {:?} permissions", required),
                Some("Run with elevated permissions or change file ownership"),
                None,
            ),
            FileSystemError::DiskFull { .. } => Suggestion::new(
                "Free up disk space or use a different location",
                None,
                None,
            ),
            _ => Suggestion::new("Check file path and permissions", None, None),
        }
    }
}
```

### Timeout Handling

```rust
#[derive(Error, Debug)]
pub enum TimeoutError {
    #[error("Operation timed out after {elapsed:?} (limit: {limit:?})")]
    Timeout { elapsed: Duration, limit: Duration, operation: String },
    
    #[error("Partial results available after timeout")]
    PartialResult { 
        elapsed: Duration,
        limit: Duration,
        partial_result: Box<dyn Any>,
        completion_estimate: Duration,
    },
}

pub struct TimeoutGuard {
    deadline: Instant,
    check_interval: usize,
    operation_count: AtomicUsize,
}

impl TimeoutGuard {
    pub fn new(timeout: Duration) -> Self {
        Self {
            deadline: Instant::now() + timeout,
            check_interval: 1000,
            operation_count: AtomicUsize::new(0),
        }
    }
    
    pub fn check(&self) -> Result<(), TimeoutError> {
        let count = self.operation_count.fetch_add(1, Ordering::Relaxed);
        
        if count % self.check_interval == 0 {
            if Instant::now() > self.deadline {
                return Err(TimeoutError::Timeout {
                    elapsed: self.deadline.elapsed(),
                    limit: self.deadline - Instant::now() + self.deadline.elapsed(),
                    operation: "inference".to_string(),
                });
            }
        }
        
        Ok(())
    }
}

// Async timeout support
pub async fn run_with_timeout<F, T>(
    future: F,
    timeout: Duration,
) -> Result<T, TimeoutError>
where F: Future<Output = T> {
    match tokio::time::timeout(timeout, future).await {
        Ok(result) => Ok(result),
        Err(_) => Err(TimeoutError::Timeout {
            elapsed: timeout,
            limit: timeout,
            operation: "async operation".to_string(),
        }),
    }
}
```

---

## User-Friendly Error Messages

### Principles of Good Error Messages

Every Lutufi error message follows these principles:

1. **Clear**: State what happened in plain language
2. **Specific**: Include relevant values and context
3. **Actionable**: Suggest concrete steps to fix
4. **Educational**: Explain why it matters
5. **Referenced**: Link to documentation

```rust
pub struct ErrorMessageBuilder {
    what_happened: String,
    why_it_matters: Option<String>,
    how_to_fix: Vec<String>,
    where_to_learn: Option<String>,
}

impl ErrorMessageBuilder {
    pub fn new(what: impl Into<String>) -> Self {
        Self {
            what_happened: what.into(),
            why_it_matters: None,
            how_to_fix: Vec::new(),
            where_to_learn: None,
        }
    }
    
    pub fn because(mut self, reason: impl Into<String>) -> Self {
        self.why_it_matters = Some(reason.into());
        self
    }
    
    pub fn try_this(mut self, suggestion: impl Into<String>) -> Self {
        self.how_to_fix.push(suggestion.into());
        self
    }
    
    pub fn learn_more(mut self, url: impl Into<String>) -> Self {
        self.where_to_learn = Some(url.into());
        self
    }
    
    pub fn build(self) -> String {
        let mut message = format!("Error: {}", self.what_happened);
        
        if let Some(why) = self.why_it_matters {
            message.push_str(&format!("\n\nWhy this matters: {}", why));
        }
        
        if !self.how_to_fix.is_empty() {
            message.push_str("\n\nHow to fix:\n");
            for (i, fix) in self.how_to_fix.iter().enumerate() {
                message.push_str(&format!("{}. {}\n", i + 1, fix));
            }
        }
        
        if let Some(url) = self.where_to_learn {
            message.push_str(&format!("\nLearn more: {}", url));
        }
        
        message
    }
}

// Example usage
let msg = ErrorMessageBuilder::new(
    "Inference failed: Treewidth (47) exceeds maximum for exact inference (25)"
)
.because(
    "Exact inference algorithms require exponential time and space in the treewidth. \
     Your model's high connectivity makes exact computation infeasible."
)
.try_this("Use approximate inference: model.query(..., algorithm='belief_proagation')")
.try_this("Simplify the model by removing weak dependencies")
.try_this("Query smaller subsets of variables at a time")
.learn_more("https://lutufi.org/errors/inference/treewidth")
.build();
```

### Python API Error Messages

```rust
use pyo3::exceptions::{PyValueError, PyRuntimeError, PyMemoryError};
use pyo3::prelude::*;

impl From<LutufiError> for PyErr {
    fn from(err: LutufiError) -> PyErr {
        match err.kind {
            ErrorKind::Validation | ErrorKind::Model => {
                PyValueError::new_err(err.to_user_string())
            }
            ErrorKind::Inference => {
                PyRuntimeError::new_err(err.to_user_string())
            }
            ErrorKind::Resource => {
                if err.is_out_of_memory() {
                    PyMemoryError::new_err(err.to_user_string())
                } else {
                    PyRuntimeError::new_err(err.to_user_string())
                }
            }
            _ => PyRuntimeError::new_err(err.to_user_string()),
        }
    }
}
```

**Example Python Error Output:**
```python
>>> model.query(variables=["A"], evidence={"B": "invalid_value"})
ValueError: Evidence validation failed for variable 'B'

  Value provided: 'invalid_value'
  Expected one of: ['low', 'medium', 'high']
  
  Similar valid values: ['high', 'low']
  
  Fix: Use one of the valid values:
    model.query(variables=["A"], evidence={"B": "high"})
    
  Or check your data preprocessing for typos.
```

---

## Error Recovery Strategies

### Automatic Fallback Algorithms

```rust
pub struct RecoveryStrategy {
    max_attempts: usize,
    fallback_chain: Vec<RecoveryAction>,
}

pub enum RecoveryAction {
    TryAlgorithm(InferenceAlgorithm),
    ReducePrecision(u8),
    IncreaseTimeout(Duration),
    SimplifyModel(f64),  // Remove edges below threshold
    UseSampling(usize),
    EnablePaging,
}

impl RecoveryStrategy {
    pub fn attempt_recovery(&self, error: &LutufiError, context: &mut InferenceContext) -> Result<QueryResult, RecoveryFailure> {
        for (attempt, action) in self.fallback_chain.iter().enumerate() {
            if attempt >= self.max_attempts {
                break;
            }
            
            log::info!("Recovery attempt {}: {:?}", attempt + 1, action);
            
            match self.apply_action(action, context) {
                Ok(result) => {
                    log::info!("Recovery successful using {:?}", action);
                    return Ok(result);
                }
                Err(e) => {
                    log::warn!("Recovery attempt failed: {}", e);
                    continue;
                }
            }
        }
        
        Err(RecoveryFailure::Exhausted {
            original_error: error.clone(),
            attempts: self.max_attempts,
        })
    }
    
    fn apply_action(&self, action: &RecoveryAction, context: &mut InferenceContext) -> Result<QueryResult, LutufiError> {
        match action {
            RecoveryAction::TryAlgorithm(alg) => {
                context.set_algorithm(alg.clone());
                context.run_inference()
            }
            RecoveryAction::ReducePrecision(bits) => {
                context.set_precision(*bits);
                context.run_inference()
            }
            RecoveryAction::UseSampling(n_samples) => {
                context.set_sampling(*n_samples);
                context.run_sampling_inference()
            }
            _ => unimplemented!(),
        }
    }
}
```

### Partial Results

```rust
pub struct PartialQueryResult {
    pub completed_variables: HashMap<VariableId, Belief>,
    pub pending_variables: Vec<VariableId>,
    pub partial_evidence: EvidenceSet,
    pub completion_percentage: f64,
    pub estimated_remaining_time: Duration,
    pub quality_metrics: QualityMetrics,
}

impl PartialQueryResult {
    pub fn is_sufficient(&self, requirements: &QueryRequirements) -> bool {
        // Check if partial result meets minimum requirements
        self.completion_percentage >= requirements.min_completion_percentage
            && self.quality_metrics.worst_case_error <= requirements.max_error
    }
    
    pub fn merge(self, other: PartialQueryResult) -> PartialQueryResult {
        let mut completed = self.completed_variables;
        completed.extend(other.completed_variables);
        
        PartialQueryResult {
            completed_variables: completed,
            pending_variables: other.pending_variables,
            partial_evidence: self.partial_evidence,
            completion_percentage: (self.completion_percentage + other.completion_percentage) / 2.0,
            estimated_remaining_time: self.estimated_remaining_time + other.estimated_remaining_time,
            quality_metrics: self.quality_metrics.combine(&other.quality_metrics),
        }
    }
}
```

### Checkpointing for Long Operations

```rust
pub struct CheckpointManager {
    checkpoint_dir: PathBuf,
    interval: Duration,
    last_checkpoint: Instant,
}

impl CheckpointManager {
    pub async fn maybe_checkpoint<T: Serialize>(
        &mut self,
        state: &T,
        operation_id: &str
    ) -> Result<Option<PathBuf>, CheckpointError> {
        if self.last_checkpoint.elapsed() < self.interval {
            return Ok(None);
        }
        
        let checkpoint_path = self.checkpoint_dir
            .join(format!("{}_{}.checkpoint", operation_id, timestamp()));
        
        let serialized = bincode::serialize(state)
            .map_err(|e| CheckpointError::Serialization(e))?;
        
        tokio::fs::write(&checkpoint_path, serialized).await
            .map_err(|e| CheckpointError::Io(e))?;
        
        self.last_checkpoint = Instant::now();
        
        log::info!("Checkpoint saved to {:?}", checkpoint_path);
        
        Ok(Some(checkpoint_path))
    }
    
    pub fn resume_from_checkpoint<T: DeserializeOwned>(
        &self,
        operation_id: &str
    ) -> Result<Option<T>, CheckpointError> {
        // Find most recent checkpoint
        let checkpoints: Vec<_> = std::fs::read_dir(&self.checkpoint_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with(operation_id))
            .collect();
        
        if checkpoints.is_empty() {
            return Ok(None);
        }
        
        let latest = checkpoints.iter()
            .max_by_key(|e| e.metadata().unwrap().modified().unwrap())
            .unwrap();
        
        let data = std::fs::read(latest.path())?;
        let state = bincode::deserialize(&data)?;
        
        log::info!("Resumed from checkpoint {:?}", latest.path());
        
        Ok(Some(state))
    }
}
```

---

## Logging and Diagnostics

### Structured Logging

```rust
use tracing::{info, warn, error, debug, span, Level};

pub struct StructuredLogger;

impl StructuredLogger {
    pub fn log_error(error: &LutufiError) {
        error!(
            error.code = %error.error_code,
            error.kind = ?error.kind,
            error.message = %error.message,
            error.severity = ?error.severity,
            "Lutufi error occurred"
        );
        
        if let Some(source) = &error.source {
            error!(
                error.source = %source,
                "Caused by"
            );
        }
    }
    
    pub fn log_inference_start(query: &Query, algorithm: &str) {
        let span = span!(Level::INFO, "inference", 
            query.variables = ?query.variables,
            query.evidence_count = query.evidence.len(),
            algorithm = algorithm,
        );
        let _enter = span.enter();
        
        info!("Starting inference");
    }
    
    pub fn log_convergence_status(iteration: usize, delta: f64, converged: bool) {
        debug!(
            iteration = iteration,
            delta = delta,
            converged = converged,
            "Convergence check"
        );
    }
}
```

### Log Levels

```rust
pub enum LogLevel {
    /// Detailed execution trace for debugging
    Trace,
    
    /// Detailed information for development
    Debug,
    
    /// General operational information
    Info,
    
    /// Potentially harmful situations
    Warn,
    
    /// Error events that might still allow continuation
    Error,
    
    /// Severe errors requiring immediate attention
    Fatal,
}

pub struct LogConfiguration {
    pub level: LogLevel,
    pub destinations: Vec<LogDestination>,
    pub filters: Vec<LogFilter>,
    pub format: LogFormat,
    pub sampling: Option<LogSampling>,
}

pub enum LogDestination {
    Console,
    File(PathBuf),
    Syslog,
    Remote(String),  // URL for log aggregation
}

pub enum LogFormat {
    Pretty,     // Human-readable with colors
    Json,       // Structured for machine parsing
    Compact,    // Single line, minimal fields
}
```

### What to Log

**Always Log:**
- Error conditions with full context
- Algorithm selection decisions
- Performance milestones (start, progress, completion)
- Resource usage warnings
- Configuration changes

**Log at Debug Level:**
- Individual message passing iterations
- Detailed convergence metrics
- Memory allocation/deallocation
- Cache hits/misses

**Log at Trace Level:**
- Factor evaluations
- Individual arithmetic operations
- Cache line access patterns

```rust
pub struct LoggingPolicy;

impl LoggingPolicy {
    pub fn should_log(event: &LogEvent, level: LogLevel) -> bool {
        match (event, level) {
            // Always log errors
            (LogEvent::Error(_), _) => true,
            
            // Log algorithm changes at info level
            (LogEvent::AlgorithmChange { from, to }, LogLevel::Info) => {
                info!("Switching inference algorithm from {} to {}", from, to);
                true
            }
            
            // Log convergence at debug level
            (LogEvent::ConvergenceUpdate { iteration, delta }, LogLevel::Debug) => {
                iteration % 100 == 0 || *delta < 1e-6  // Sample or at convergence
            }
            
            // Log memory at warn level when high
            (LogEvent::MemoryUsage { used, limit }, LogLevel::Warn) => {
                *used as f64 / *limit as f64 > 0.9
            }
            
            _ => false,
        }
    }
}
```

### Privacy Considerations

```rust
pub struct PrivacyFilter;

impl PrivacyFilter {
    pub fn sanitize(log_entry: &mut LogEntry) {
        // Remove sensitive data from logs
        for field in &mut log_entry.fields {
            if Self::is_sensitive(&field.name) {
                field.value = Self::redact(&field.value);
            }
        }
    }
    
    fn is_sensitive(field_name: &str) -> bool {
        let sensitive_patterns = [
            "password",
            "secret",
            "token",
            "credential",
            "ssn",
            "email",
            // Patient/customer data
            "patient_name",
            "customer_id",
        ];
        
        sensitive_patterns.iter().any(|p| field_name.to_lowercase().contains(p))
    }
    
    fn redact(value: &str) -> String {
        format!("[REDACTED: {} chars]", value.len())
    }
}
```

---

## Debugging Support

### Verbose Modes

```rust
pub struct DebugConfiguration {
    pub verbose_level: VerboseLevel,
    pub enable_profiling: bool,
    pub save_intermediate_results: bool,
    pub debug_output_dir: Option<PathBuf>,
    pub dump_factor_graphs: bool,
    pub trace_message_passing: bool,
}

pub enum VerboseLevel {
    Silent,
    Normal,      // Basic progress
    Verbose,     // Detailed progress
    Debug,       // Internal state dumps
    Trace,       // Every operation
}

impl InferenceEngine {
    pub fn with_verbose(mut self, level: VerboseLevel) -> Self {
        self.verbose_level = level;
        self
    }
    
    pub fn run_verbose(&self, model: &FactorGraph, query: &Query) -> QueryResult {
        match self.verbose_level {
            VerboseLevel::Trace => self.run_with_tracing(model, query),
            VerboseLevel::Debug => self.run_with_debugging(model, query),
            VerboseLevel::Verbose => self.run_with_progress(model, query),
            _ => self.run(model, query),
        }
    }
    
    fn run_with_tracing(&self, model: &FactorGraph, query: &Query) -> QueryResult {
        println!("=== INFERENCE TRACE ===");
        println!("Model: {} variables, {} factors", model.n_vars(), model.n_factors());
        println!("Query: {:?}", query);
        
        let result = self.run_with_callbacks(model, query, |event| {
            match event {
                InferenceEvent::MessageComputed { from, to, message } => {
                    println!("Message {} -> {}: {:?}", from, to, message);
                }
                InferenceEvent::BeliefUpdated { variable, belief } => {
                    println!("Belief[{}] = {:?}", variable, belief);
                }
                _ => {}
            }
        });
        
        println!("=== END TRACE ===");
        result
    }
}
```

### Model Inspection on Error

```rust
pub struct ErrorInspector;

impl ErrorInspector {
    pub fn inspect_on_error(error: &LutufiError, model: &FactorGraph) -> InspectionReport {
        let mut report = InspectionReport::new();
        
        // Inspect model state related to error
        match error.kind {
            ErrorKind::Inference => {
                // Check for common issues
                if Self::has_loops(model) {
                    report.add_finding("Model contains cycles - may affect convergence");
                }
                
                if Self::has_deterministic_cpts(model) {
                    report.add_finding("Model contains deterministic CPTs - may cause numerical issues");
                }
                
                // Analyze query complexity
                let tw = estimate_treewidth(model);
                report.add_metric("Estimated treewidth", tw);
            }
            
            ErrorKind::Numerical => {
                // Check for extreme probabilities
                let extreme_factors = Self::find_extreme_factors(model);
                if !extreme_factors.is_empty() {
                    report.add_finding(format!(
                        "Found {} factors with extreme values (< 1e-10 or > 1-1e-10)",
                        extreme_factors.len()
                    ));
                }
            }
            
            _ => {}
        }
        
        report
    }
    
    pub fn generate_diagnostic_bundle(
        &self,
        error: &LutufiError,
        model: &FactorGraph,
        query: &Query
    ) -> DiagnosticBundle {
        DiagnosticBundle {
            error: error.clone(),
            model_snapshot: model.snapshot(),
            query: query.clone(),
            system_info: SystemInfo::collect(),
            logs: self.collect_recent_logs(),
            timestamp: Utc::now(),
        }
    }
}
```

### Reproducible Error Reports

```rust
pub struct ReproducibleErrorReport {
    pub error_code: String,
    pub error_message: String,
    pub reproduction_steps: Vec<String>,
    pub minimal_example: Option<String>,
    pub environment: EnvironmentInfo,
    pub seed: Option<u64>,
}

impl ReproducibleErrorReport {
    pub fn from_error(error: &LutufiError, context: &ErrorContext) -> Self {
        Self {
            error_code: error.error_code.clone(),
            error_message: error.message.clone(),
            reproduction_steps: Self::extract_reproduction_steps(context),
            minimal_example: Self::generate_minimal_example(context),
            environment: EnvironmentInfo::current(),
            seed: context.random_seed,
        }
    }
    
    fn generate_minimal_example(context: &ErrorContext) -> Option<String> {
        // Generate minimal Python code to reproduce the error
        if let Some(model) = &context.model_state {
            let mut code = String::new();
            code.push_str("import lutufi as lf\n\n");
            code.push_str("# Reproduce error: ");
            code.push_str(&context.error_code);
            code.push('\n');
            
            // Add model construction
            code.push_str(&model.to_python_code());
            
            // Add query
            if let Some(query) = &context.query {
                code.push_str(&format!("\n# Trigger error\nresult = {}\n", 
                    query.to_python_code()));
            }
            
            Some(code)
        } else {
            None
        }
    }
}
```

---

## Error Reporting from Users

### Automated Error Reporting (Opt-In)

```rust
pub struct ErrorReporter {
    enabled: bool,
    endpoint: Option<String>,
    filter: ErrorFilter,
}

impl ErrorReporter {
    pub async fn maybe_report(&self, error: &LutufiError) {
        if !self.enabled {
            return;
        }
        
        if !self.filter.should_report(error) {
            return;
        }
        
        let report = self.create_report(error);
        
        if let Some(endpoint) = &self.endpoint {
            match self.send_report(endpoint, &report).await {
                Ok(_) => log::info!("Error report sent successfully"),
                Err(e) => log::warn!("Failed to send error report: {}", e),
            }
        }
    }
    
    fn create_report(&self, error: &LutufiError) -> ErrorReport {
        ErrorReport {
            error_code: error.error_code.clone(),
            error_message: error.message.clone(),
            error_kind: format!("{:?}", error.kind),
            severity: format!("{:?}", error.severity),
            // Anonymized information
            library_version: env!("CARGO_PKG_VERSION").to_string(),
            rust_version: rustc_version_runtime::version().to_string(),
            operating_system: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            // NO user data, NO file paths, NO variable names
        }
    }
}
```

### Crash Reports

```rust
pub struct CrashHandler {
    previous_handler: Option<Box<dyn Fn(&std::panic::PanicInfo<'_>) + Sync + Send + 'static>>,
}

impl CrashHandler {
    pub fn install() -> Self {
        let handler = Box::new(|info: &std::panic::PanicInfo<'_>| {
            // Generate crash report
            let report = CrashReport {
                panic_message: info.to_string(),
                backtrace: Backtrace::new(),
                timestamp: Utc::now(),
                system_info: SystemInfo::collect(),
            };
            
            // Save to file
            if let Err(e) = report.save_to("lutufi_crash_report.json") {
                eprintln!("Failed to save crash report: {}", e);
            }
            
            // Offer to send
            eprintln!("\nLutufi has encountered a fatal error.");
            eprintln!("A crash report has been saved to lutufi_crash_report.json");
            eprintln!("Please submit this report at https://github.com/lutufi/lutufi/issues");
        });
        
        let previous = std::panic::take_hook();
        std::panic::set_hook(handler);
        
        Self { previous_handler: Some(previous) }
    }
}

impl Drop for CrashHandler {
    fn drop(&mut self) {
        if let Some(handler) = self.previous_handler.take() {
            std::panic::set_hook(handler);
        }
    }
}
```

### Telemetry Design

```rust
pub struct Telemetry {
    enabled: bool,
    session_id: Uuid,
    events: Vec<TelemetryEvent>,
}

#[derive(Serialize)]
pub struct TelemetryEvent {
    timestamp: DateTime<Utc>,
    event_type: String,
    // Anonymized metrics only
    duration_ms: Option<u64>,
    success: Option<bool>,
    algorithm: Option<String>,
    model_size_bucket: Option<String>,  // "small", "medium", "large", not exact numbers
}

impl Telemetry {
    pub fn record_inference(&mut self, algorithm: &str, duration: Duration, success: bool) {
        if !self.enabled {
            return;
        }
        
        self.events.push(TelemetryEvent {
            timestamp: Utc::now(),
            event_type: "inference".to_string(),
            duration_ms: Some(duration.as_millis() as u64),
            success: Some(success),
            algorithm: Some(algorithm.to_string()),
            model_size_bucket: None,  // Set by caller based on actual size
        });
    }
    
    pub fn flush(&mut self) {
        if self.events.is_empty() {
            return;
        }
        
        // Send to telemetry endpoint
        // Only aggregated, anonymized data
        let batch = std::mem::take(&mut self.events);
        
        // Async send without blocking
        tokio::spawn(async move {
            // Send batch
        });
    }
}
```

---

## Testing Error Conditions

### Fuzz Testing

```rust
#[cfg(test)]
mod fuzz_tests {
    use super::*;
    use arbitrary::Arbitrary;
    
    #[derive(Arbitrary, Debug)]
    struct RandomModel {
        n_nodes: u8,
        edge_probability: f64,
        max_states: u8,
    }
    
    #[test]
    fn fuzz_model_validation() {
        let mut engine = FuzzEngine::new();
        
        for _ in 0..10000 {
            let random: RandomModel = engine.arbitrary();
            
            // Should never panic, always return Result
            let result = std::panic::catch_unwind(|| {
                let model = generate_random_model(&random);
                model.validate()
            });
            
            assert!(result.is_ok(), "Model validation panicked");
        }
    }
    
    #[test]
    fn fuzz_inference() {
        let mut engine = FuzzEngine::new();
        
        for _ in 0..10000 {
            let model = engine.arbitrary::<RandomModel>();
            let query = engine.arbitrary::<RandomQuery>();
            
            let result = std::panic::catch_unwind(|| {
                let bn = generate_random_model(&model);
                bn.query(&query.variables, &query.evidence)
            });
            
            assert!(result.is_ok(), "Inference panicked");
            
            // If Ok, result should be valid or proper error
            if let Ok(Ok(inference_result)) = result {
                assert!(inference_result.is_valid());
            }
        }
    }
}
```

### Property-Based Testing

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn inference_never_panics(
            model in any::<BayesianNetwork>(),
            query in any::<Query>()
        ) {
            // Should never panic
            let _ = model.query(&query);
        }
        
        #[test]
        fn exact_inference_matches_sampling(
            small_model in small_bayesian_network(),
            query in any::<Query>()
        ) {
            // For small models, exact and approximate should be close
            let exact = small_model.query(&query)
                .expect("Exact inference should work for small models");
            
            let approximate = small_model.query_with_algorithm(&query, "gibbs_sampling")
                .expect("Sampling should work");
            
            // KL divergence should be small
            let kl = exact.kl_divergence(&approximate);
            prop_assert!(kl < 0.1, "KL divergence too large: {}", kl);
        }
        
        #[test]
        fn invalid_cpd_always_rejected(
            invalid_probabilities in vec(0.0f64..1.0, 0..100)
        ) {
            // If probabilities don't sum to 1, should be rejected
            let sum: f64 = invalid_probabilities.iter().sum();
            
            if (sum - 1.0).abs() > 1e-6 {
                let result = CPD::new(invalid_probabilities);
                prop_assert!(result.is_err(), "Non-normalized CPD should be rejected");
            }
        }
    }
}
```

### Error Injection Testing

```rust
#[cfg(test)]
mod error_injection_tests {
    use super::*;
    
    #[test]
    fn test_memory_exhaustion_handling() {
        let allocator = FailingAllocator::new()
            .fail_after(1000);
        
        let result = with_allocator(allocator, || {
            let model = create_large_model();
            model.query_all_marginals()
        });
        
        assert!(matches!(result, Err(LutufiError::MemoryError(_))));
    }
    
    #[test]
    fn test_timeout_handling() {
        let clock = MockClock::new()
            .advance_after(100, Duration::from_secs(3600));
        
        let result = with_clock(clock, || {
            let model = slow_convergence_model();
            model.query_with_timeout(Duration::from_secs(60))
        });
        
        assert!(matches!(result, Err(LutufiError::Timeout(_))));
    }
    
    #[test]
    fn test_network_partition_handling() {
        let network = MockNetwork::new()
            .partition_after(5);
        
        let result = with_network(network, || {
            distributed_inference(large_model())
        });
        
        // Should handle gracefully, not panic
        assert!(result.is_ok() || matches!(result, Err(LutufiError::NetworkError(_))));
    }
}
```

---

## Documentation of Errors

### Error Code Reference

```rust
pub struct ErrorDocumentation;

impl ErrorDocumentation {
    pub fn generate_reference() -> String {
        let mut doc = String::new();
        
        doc.push_str("# Lutufi Error Code Reference\n\n");
        
        for category in ErrorCategory::all() {
            doc.push_str(&format!("## {:?} Errors\n\n", category));
            doc.push_str("| Code | Description | Resolution |\n");
            doc.push_str("|------|-------------|------------|\n");
            
            for error_def in Self::errors_in_category(category) {
                doc.push_str(&format!(
                    "| {} | {} | {} |\n",
                    error_def.code,
                    error_def.description,
                    error_def.resolution_summary
                ));
            }
            
            doc.push('\n');
        }
        
        doc
    }
}

// Example generated documentation:
// | LUT-INF-001 | Convergence failure in belief propagation | Try increasing damping or use sampling |
// | LUT-INF-042 | Treewidth exceeds limit for exact inference | Use approximate inference methods |
// | LUT-LRN-015 | Insufficient data for parameter learning | Collect more data or use stronger priors |
```

### Troubleshooting Guide

```markdown
# Troubleshooting Guide

## Common Errors

### LUT-INF-001: Convergence Failure

**Symptoms:**
- "Belief propagation failed to converge after 1000 iterations"
- Oscillating beliefs between iterations

**Possible Causes:**
1. Model contains many short cycles
2. Deterministic dependencies create hard constraints
3. Extreme probability values cause numerical issues

**Solutions:**

1. **Increase damping:**
   ```python
   result = model.query(variables=["X"], damping=0.5)
   ```

2. **Use a different algorithm:**
   ```python
   result = model.query(variables=["X"], algorithm="gibbs_sampling")
   ```

3. **Simplify the model:**
   Remove edges with weak dependencies

### LUT-INF-042: High Treewidth

**Symptoms:**
- "Treewidth (47) exceeds limit for exact inference (25)"
- Out of memory errors during junction tree construction

**Solutions:**

1. **Use approximate inference:**
   ```python
   result = model.query(variables=["X"], algorithm="belief_propagation")
   ```

2. **Query fewer variables:**
   ```python
   # Instead of querying all variables at once
   for var in variables:
       result = model.query(variables=[var])
   ```
```

---

## How Lutufi Handles Errors

### Specific Implementation Patterns

**1. Result Type Throughout:**
```rust
// All fallible operations return Result
pub fn query(&self, variables: &[String]) -> Result<QueryResult, InferenceError>;
pub fn fit(data: &DataFrame) -> Result<BayesianNetwork, LearningError>;
pub fn load(path: &Path) -> Result<BayesianNetwork, IoError>;
```

**2. Error Context Enrichment:**
```rust
fn load_cpd(path: &Path) -> Result<CPD, CpdError> {
    let data = std::fs::read(path)
        .map_err(|e| CpdError::io_error(path, e))?;
    
    parse_cpd(&data)
        .map_err(|e| CpdError::parse_error(path, e)
            .with_context("Loading CPD from file")
            .with_suggestion("Check file format matches expected schema"))
}
```

**3. Graceful Degradation:**
```rust
fn query_with_fallback(&self, query: &Query) -> Result<QueryResult, InferenceError> {
    // Try exact first
    if let Ok(result) = self.try_exact(query) {
        return Ok(result);
    }
    
    // Fall back to approximate
    if let Ok(result) = self.try_approximate(query) {
        log::warn!("Used approximate inference for query");
        return Ok(result);
    }
    
    // Last resort: sampling
    self.try_sampling(query)
}
```

**4. Defensive Programming:**
```rust
fn normalize_probabilities(probs: &mut [f64]) {
    let sum: f64 = probs.iter().sum();
    
    if sum == 0.0 {
        // Handle degenerate case
        let uniform = 1.0 / probs.len() as f64;
        probs.fill(uniform);
        log::warn!("All-zero probabilities detected, using uniform");
    } else if !sum.is_finite() {
        // Handle overflow/underflow
        log::error!("Non-finite probability sum: {}", sum);
        // Use log-space computation instead
    } else {
        for p in probs.iter_mut() {
            *p /= sum;
        }
    }
}
```

---

## Key References

### Error Handling Best Practices

1. **Martin, R. C. (2008).** *Clean Code: A Handbook of Agile Software Craftsmanship.* Prentice Hall.
   - Error handling as a separate concern

2. **Google (2023).** "Google Rust Style Guide - Error Handling." https://google.github.io/styleguide/rust/
   - Rust-specific error handling patterns

3. **The Rust Programming Language.** "Error Handling." https://doc.rust-lang.org/book/ch09-00-error-handling.html
   - Rust's error handling philosophy

4. **BurntSushi (2020).** "Error Handling in Rust." https://blog.burntsushi.net/rust-error-handling/
   - Practical error handling patterns

### Reliability Engineering

5. **Allspaw, J. (2012).** *Web Operations: Keeping the Data On Time.* O'Reilly Media.
   - Incident response and error analysis

6. **Beyer, B., et al. (2016).** *Site Reliability Engineering: How Google Runs Production Systems.* O'Reilly Media.
   - Reliability patterns applicable to libraries

7. **Vaughan, D. (1996).** *The Challenger Launch Decision: Risky Technology, Culture, and Deviance at NASA.* University of Chicago Press.
   - Normalization of deviance - lessons for error handling

### Numerical Stability

8. **Higham, N. J. (2002).** *Accuracy and Stability of Numerical Algorithms* (2nd ed.). SIAM.
   - Numerical error analysis

9. **Goldberg, D. (1991).** "What Every Computer Scientist Should Know About Floating-Point Arithmetic." *ACM Computing Surveys,* 23(1), 5-48.
   - Floating-point error handling

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial draft |
| 1.0 | 2026-03-03 | Wasswa Lutufi Sebbanja | Complete error handling document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
