# Assumptions Log

**Document Version**: 1.0  
**Status**: Working Draft  
**Author**: Wasswa Lutufi Sebbanja  
**Last Updated**: March 2026  

---

## Table of Contents

1. [Purpose](#purpose)
2. [Categories](#categories)
3. [Mathematical Assumptions](#mathematical-assumptions)
4. [Statistical Assumptions](#statistical-assumptions)
5. [Computational Assumptions](#computational-assumptions)
6. [User Behavior Assumptions](#user-behavior-assumptions)
7. [Domain Assumptions](#domain-assumptions)
8. [Testing Assumptions](#testing-assumptions)
9. [When Assumptions Fail](#when-assumptions-fail)
10. [Relaxing Assumptions](#relaxing-assumptions)

---

## Purpose

The Assumptions Log explicitly documents all assumptions underlying the Lutufi library's design, implementation, and application. Explicit documentation of assumptions serves to:

- **Guide users**: Help users understand when Lutufi is appropriate for their problem
- **Support validation**: Identify assumptions that need testing
- **Enable debugging**: When results are unexpected, check assumption violations
- **Inform development**: Guide future work to relax restrictive assumptions
- **Ensure transparency**: Make implicit assumptions explicit

Each assumption includes:
- **Statement**: Clear articulation of what is assumed
- **Rationale**: Why this assumption is made
- **Impact**: Consequences if the assumption is violated
- **Validation**: How to check if the assumption holds
- **Detection**: How Lutufi detects or reports violations
- **Relaxation**: Future work to remove or weaken the assumption

---

## Categories

Assumptions are organized into six categories:

| Category | Description | Examples |
|----------|-------------|----------|
| **Mathematical** | Formal properties of models and algorithms | Faithfulness, Markov conditions |
| **Statistical** | Properties of data and sampling | Sample size, independence |
| **Computational** | System and resource properties | Memory, precision, parallelism |
| **User Behavior** | How users interact with the library | Input validity, intended use |
| **Domain** | Properties of application domains | Network structure, causal relationships |
| **Testing** | Properties enabling testability | Determinism, reproducibility |

---

## Mathematical Assumptions

### MATH-001: Markov Condition for Bayesian Networks

**Statement**: In any Bayesian network used with Lutufi, the Markov condition holds: every variable is conditionally independent of its non-descendants given its parents.

**Rationale**: The Markov condition is the foundational assumption enabling factorization of joint distributions in Bayesian networks. Without it, the directed graph structure loses its probabilistic meaning.

**Impact if Violated**:
- Inference results will be incorrect
- Conditional independence queries return wrong answers
- Learned models do not accurately represent the true distribution

**Validation**:
- Cannot be validated from data alone (untestable assumption)
- Must be justified by domain knowledge and causal understanding
- Sensitivity analysis can check robustness to violations

**Detection**: Not automatically detectable. Users must ensure their model structure reflects true conditional independencies.

**Relaxation**: None possible—this is definitional for Bayesian networks. Users needing weaker assumptions should consider undirected models or non-graphical methods.

---

### MATH-002: Faithfulness

**Statement**: The conditional independences in the data match the d-separation properties of the true graph (no "accidental" independences).

**Rationale**: Faithfulness is required for constraint-based causal discovery (PC algorithm) to recover the correct graph structure. It ensures that observed conditional independences reflect true structural relationships.

**Impact if Violated**:
- Structure learning may omit true edges (false negatives)
- May fail to orient edges correctly
- Learned graph may not represent true causal structure

**Validation**:
- Cannot be tested in finite samples
- Can check for near-violations (very weak dependencies)
- Domain knowledge can suggest likely violations (e.g., balancing in biological systems)

**Detection**: 
- Lutufi warns when very weak dependencies are found during structure learning
- User can request faithfulness diagnostics

**Relaxation**:
- Score-based methods (GES) make weaker assumptions
- Allowing latent variables relaxes faithfulness requirements
- Future work: Implement algorithms robust to certain faithfulness violations

---

### MATH-003: Causal Sufficiency (for Structure Learning)

**Statement**: All common causes of measured variables are themselves measured (no latent confounders).

**Rationale**: Without causal sufficiency, constraint-based algorithms may incorrectly orient edges or include spurious ones. This assumption enables the standard causal discovery framework.

**Impact if Violated**:
- Spurious edges between variables with common unmeasured causes
- Incorrect causal conclusions
- Models that don't generalize to interventions

**Validation**:
- Domain knowledge about potential confounders
- Use FCI algorithm (which doesn't assume sufficiency) and compare results
- Sensitivity analysis

**Detection**:
- FCI algorithm can detect violations and represent uncertainty
- Lutufi can flag models where FCI finds significant latent structure

**Relaxation**:
- FCI and related algorithms handle latent variables
- Future work: Full FCI implementation with PAG output

---

### MATH-004: Positivity (or Strict Positivity)

**Statement**: All joint probability configurations have positive probability (P(X=x) > 0 for all x, or at least all conditionally possible x).

**Rationale**: Positivity ensures that conditional probabilities are well-defined and that conditioning on any observed value is valid. Required for many identification results.

**Impact if Violated**:
- Division by zero in conditional probability calculations
- Undefined causal effects in some subpopulations
- Numerical instability

**Validation**:
- Check for zero probabilities in observed data
- Domain knowledge about impossible configurations

**Detection**:
- Lutufi checks for zero probabilities during inference
- Warnings issued for structural zeros in CPTs

**Relaxation**:
- Handle structural zeros explicitly
- Use bounds instead of point estimates when positivity fails
- Future work: Implement non-parametric bounds for positivity violations

---

### MATH-005: DAG Structure (for Bayesian Networks)

**Statement**: The underlying graphical model is a directed acyclic graph (no directed cycles).

**Rationale**: Cycles in directed graphs create feedback loops that require dynamic modeling (DBNs) or equilibrium assumptions. Standard BN inference assumes acyclicity.

**Impact if Violated**:
- Infinite loops in inference
- Ill-defined joint distribution
- Non-convergence of message passing

**Validation**:
- Cycle detection algorithms (standard graph algorithm)
- User review of model structure

**Detection**:
- Lutufi validates graph structure on model construction
- `CycleError` raised with information about detected cycle

**Relaxation**:
- Use dynamic Bayesian networks for temporal feedback
- Use cyclic causal models with equilibrium assumptions
- Future work: Support specific cyclic models (e.g., SCMs with unique solutions)

---

### MATH-006: Modularity (for Causal Inference)

**Statement**: Interventions on one variable do not change the conditional distributions of other variables (modularity assumption).

**Rationale**: Modularity enables the do-calculus: P(Y|do(X=x)) uses the same factors as P(Y|X=x) except for the intervened variable. Without modularity, causal effects are not identifiable from the graph structure.

**Impact if Violated**:
- do-calculus results do not match true intervention effects
- Causal predictions fail
- Counterfactuals are incorrect

**Validation**:
- Domain knowledge about intervention mechanisms
- Experimental validation when possible

**Detection**:
- Not directly detectable from observational data
- Users must validate through experiments or domain expertise

**Relaxation**:
- Soft interventions (conditional interventions)
- Non-modular models (explicit representation of intervention effects)
- Future work: Support non-modular causal models

---

## Statistical Assumptions

### STAT-001: Independent and Identically Distributed (IID) Samples

**Statement**: Training data consists of independent samples from the same distribution.

**Rationale**: Standard parameter estimation (MLE, Bayesian) assumes IID data. Violations require specialized methods.

**Impact if Violated**:
- Biased parameter estimates
- Overconfident uncertainty estimates
- Poor generalization

**Validation**:
- Check for temporal or spatial autocorrelation
- Examine data collection process
- Use robust standard errors or clustered estimation

**Detection**:
- Statistical tests for autocorrelation (Durbin-Watson, etc.)
- Lutufi can warn about suspicious patterns in residuals

**Relaxation**:
- Use models with explicit dependence structure
- Robust estimation methods
- Future work: Support for network-correlated data

---

### STAT-002: Sufficient Sample Size

**Statement**: The sample size is sufficient to reliably estimate model parameters and structure.

**Rationale**: Small samples lead to overfitting, unstable estimates, and unreliable structure learning.

**Impact if Violated**:
- Overfitting (too many parameters)
- Incorrect structure (spurious edges)
- Poor predictive performance
- Numerical instability

**Validation**:
- Rule of thumb: at least 5-10 samples per parameter
- Cross-validation performance
- Bootstrap stability analysis

**Detection**:
- Lutufi warns when sample size is low relative to model complexity
- Effective sample size calculations for structure learning

**Relaxation**:
- Bayesian methods with strong priors
- Regularization (L1, L2)
- Constraint-based structure learning (more sample-efficient)
- Future work: Implement sample-size aware structure learning

---

### STAT-003: No Selection Bias

**Statement**: The sample is representative of the target population (no selection on variables affected by the phenomenon of interest).

**Rationale**: Selection bias can distort observed relationships and make causal inference invalid (M-bias, collider bias).

**Impact if Violated**:
- Spurious associations
- Biased causal estimates
- Non-generalizable results

**Validation**:
- Examine data collection and inclusion criteria
- Compare sample to population demographics
- Use selection models when bias structure is known

**Detection**:
- Cannot be detected from data alone
- Requires domain knowledge about data collection

**Relaxation**:
- Explicit selection models
- Inverse probability weighting
- Bounds under selection (partial identification)
- Future work: Selection bias correction methods

---

### STAT-004: Correct Model Specification

**Statement**: The functional form and distributional assumptions of the model are correct.

**Rationale**: Inference assumes the model family contains the true data-generating process. Misspecification leads to biased or inconsistent estimates.

**Impact if Violated**:
- Biased parameter estimates
- Incorrect predictions
- Invalid confidence intervals
- Misleading causal conclusions

**Validation**:
- Goodness-of-fit tests
- Residual analysis
- Out-of-sample prediction
- Model comparison (cross-validation, information criteria)

**Detection**:
- Lutufi provides model diagnostics
- Residual plots and goodness-of-fit statistics
- Discrepancy measures

**Relaxation**:
- Non-parametric or semi-parametric models
- Model averaging
- Robust estimation (sandwich estimators)
- Future work: Flexible non-parametric factors

---

## Computational Assumptions

### COMP-001: Sufficient Memory

**Statement**: The system has sufficient RAM to hold the model structure, parameters, and intermediate results.

**Rationale**: Many inference algorithms require storing factor tables, messages, and intermediate computations in memory.

**Impact if Violated**:
- Program crash (out-of-memory)
- Excessive swapping (performance degradation)
- Inability to load large models

**Validation**:
- Estimate memory requirements before inference
- Monitor memory usage
- Use sparse representations to reduce memory

**Detection**:
- Lutufi estimates memory requirements and warns if insufficient
- `MemoryError` with suggestions for mitigation

**Relaxation**:
- Out-of-core algorithms (disk-based)
- Streaming algorithms
- Model compression
- Future work: Out-of-core inference implementation

---

### COMP-002: Floating-Point Precision

**Statement**: IEEE 754 double-precision floating-point (f64) provides sufficient precision for inference.

**Rationale**: Probabilities can span many orders of magnitude. Underflow/overflow can occur in naive implementations.

**Impact if Violated**:
- Numerical underflow (probabilities rounded to zero)
- Numerical overflow
- Incorrect inference results
- Non-convergence

**Validation**:
- Log-space computation
- Numerical stability tests
- Comparison with higher precision

**Detection**:
- Lutufi uses log-space computation by default
- Warnings for extreme probability values
- Numerical stability diagnostics

**Relaxation**:
- Log-space throughout computation
- Arbitrary precision (slower)
- Scaling and normalization strategies
- Already implemented: Log-space factors as standard

---

### COMP-003: Shared-Memory Parallelism

**Statement**: The target hardware supports shared-memory parallelism (multi-core processors).

**Rationale**: Parallel inference implementations assume shared memory for efficient communication between threads.

**Impact if Violated**:
- Cannot exploit parallel speedups
- Sequential performance only

**Validation**:
- Check CPU core count
- Verify threading support

**Detection**:
- Lutufi detects available cores and adjusts parallelism
- Falls back to sequential execution gracefully

**Relaxation**:
- Distributed computing for very large problems
- GPU acceleration
- Already implemented: Works on single core

---

### COMP-004: Deterministic Execution

**Statement**: Algorithms execute deterministically given the same inputs (for reproducibility).

**Rationale**: Reproducibility requires deterministic behavior. Some algorithms (Gibbs sampling, initialization) have stochastic elements.

**Impact if Violated**:
- Non-reproducible results
- Difficult debugging
- Unreliable tests

**Validation**:
- Set random seeds for stochastic algorithms
- Use deterministic algorithms where possible
- Test for reproducibility

**Detection**:
- Lutufi requires seed specification for stochastic algorithms
- Warnings for potentially non-deterministic operations

**Relaxation**:
- Explicit random seed control
- Deterministic approximations
- Already implemented: Seed control standard

---

## User Behavior Assumptions

### USER-001: Valid Model Structure

**Statement**: Users provide model structures (graphs, factors) that represent their domain correctly.

**Rationale**: Lutufi cannot verify that a user's graph accurately reflects their domain. It only checks structural validity (e.g., acyclicity).

**Impact if Violated**:
- Misleading results
- Wrong conclusions
- Models that don't generalize

**Validation**:
- Domain expert review
- Sensitivity analysis
- Cross-validation

**Detection**:
- Structural validation (cycles, connectivity)
- Cannot detect semantic errors

**Relaxation**:
- Structure learning from data
- Model criticism diagnostics
- Future work: Automated model validation

---

### USER-002: Appropriate Algorithm Selection

**Statement**: Users either accept automatic algorithm selection or understand the implications of manual selection.

**Rationale**: Different algorithms have different properties (exact vs. approximate, convergence guarantees, speed).

**Impact if Violated**:
- Poor performance
- Non-convergence
- Inaccurate results
- Wasted computation time

**Validation**:
- Use automatic selection when uncertain
- Consult documentation
- Benchmark different algorithms

**Detection**:
- Lutufi provides algorithm recommendations
- Warnings for potentially poor algorithm choices

**Relaxation**:
- Improved automatic selection
- Algorithm performance prediction
- Future work: Better meta-learning for selection

---

### USER-003: Data Preprocessing

**Statement**: Users appropriately preprocess data (handle missing values, encode variables, normalize if needed) before modeling.

**Rationale**: Lutufi assumes data is in appropriate format for modeling. Raw data often requires cleaning and transformation.

**Impact if Violated**:
- Poor model fit
- Numerical issues
- Biased estimates

**Validation**:
- Data validation checks
- Exploratory data analysis
- Preprocessing pipelines

**Detection**:
- Lutufi validates input data format
- Warnings for suspicious patterns (all zeros, extreme values)

**Relaxation**:
- Built-in preprocessing utilities
- Automated data cleaning
- Future work: Data validation pipeline

---

### USER-004: Correct Interpretation

**Statement**: Users interpret results correctly, understanding the difference between association and causation, and the limitations of the methods used.

**Rationale**: Probabilistic and causal inference are subtle. Misinterpretation leads to wrong conclusions.

**Impact if Violated**:
- False causal claims
- Overconfident conclusions
- Misleading reports

**Validation**:
- Documentation education
- Clear output labeling
- Uncertainty quantification

**Detection**:
- Cannot detect misinterpretation
- Documentation and warnings attempt to prevent it

**Relaxation**:
- Clear documentation
- Tutorial materials
- Conservative default outputs
- Future work: Interpretability features

---

## Domain Assumptions

### DOMAIN-001: Network Structure is Known or Learnable

**Statement**: For network analysis applications, the network structure is either known or can be accurately learned from data.

**Rationale**: Network measures and dynamics depend on structure. Missing or incorrect edges lead to wrong conclusions.

**Impact if Violated**:
- Incorrect centrality measures
- Wrong community assignments
- Misleading diffusion predictions

**Validation**:
- Multiple data sources for structure
- Sensitivity to missing edges
- Validation of learned structure

**Detection**:
- Network diagnostics (density, degree distribution)
- Comparison with known structures
- Cross-validation

**Relaxation**:
- Probabilistic network models (uncertainty about edges)
- Network reconstruction from signals
- Future work: Uncertainty quantification for network structure

---

### DOMAIN-002: Causal Relationships are Stable

**Statement**: The causal mechanisms represented in the model are stable over the time period of interest.

**Rationale**: Causal inference assumes the causal structure doesn't change during observation or intervention.

**Impact if Violated**:
- Time-varying causal effects
- Model misspecification
- Failed predictions

**Validation**:
- Temporal validation
- Structural break tests
- Domain knowledge about stability

**Detection**:
- Time-series diagnostics
- Model fit degradation over time
- Future work: Change-point detection

**Relaxation**:
- Time-varying models
- Dynamic Bayesian networks
- Regime-switching models
- Future work: Online structure adaptation

---

### DOMAIN-003: Interventions are Well-Defined

**Statement**: For causal applications, interventions are well-defined and implementable in practice.

**Rationale**: The do-operator represents an ideal intervention. Real interventions may not match this ideal.

**Impact if Violated**:
- Predicted intervention effects don't match reality
- Policy recommendations fail
- Wasted resources

**Validation**:
- Pilot interventions
- Domain expert input
- Mechanism understanding

**Detection**:
- Cannot detect from data alone
- Requires validation experiments

**Relaxation**:
- Soft interventions (conditional on context)
- Policy-relevant treatment effects
- Future work: Support for realistic intervention modeling

---

### DOMAIN-004: Network Effects are Local

**Statement**: For large network inference, dependencies are primarily local (friends of friends matter, but distant connections have negligible effects).

**Rationale**: Locality enables efficient approximate inference. Long-range dependencies make inference computationally difficult.

**Impact if Violated**:
- Poor approximation quality
- Biased estimates
- Slow convergence

**Validation**:
- Correlation decay analysis
- Domain knowledge about interaction ranges

**Detection**:
- Diagnostics for long-range dependencies
- Comparison of local vs. global methods

**Relaxation**:
- Methods for long-range dependencies
- Hierarchical models
- Future work: Multi-scale inference

---

## Testing Assumptions

### TEST-001: Reference Implementation Correctness

**Statement**: Reference implementations (pgmpy, libDAI) used for testing are themselves correct.

**Rationale**: We validate Lutufi against established libraries. If they have bugs, our tests may pass incorrectly or fail incorrectly.

**Impact if Violated**:
- False confidence in correctness
- Chasing non-existent bugs
- Incorrect "fixes" that introduce real bugs

**Validation**:
- Multiple reference implementations
- Theoretical test cases (known answers)
- Community verification

**Detection**:
- Discrepancy between references indicates need for investigation
- Mathematical properties as independent checks

**Relaxation**:
- Pure mathematical tests without external references
- Formal verification for critical components
- Future work: Formal verification project

---

### TEST-002: Test Coverage is Representative

**Statement**: The test suite covers the range of real-world use cases and edge cases.

**Rationale**: Bugs in untested code paths only manifest in production.

**Impact if Violated**:
- Production bugs
- User frustration
- Loss of trust

**Validation**:
- Code coverage analysis
- Property-based testing for wide input ranges
- User-reported bugs as coverage indicators

**Detection**:
- Coverage reports
- Fuzzing results

**Relaxation**:
- Continuous expansion of test suite
- Beta testing program
- Already ongoing: Coverage improvements

---

## When Assumptions Fail

### Detection Mechanisms

Lutufi implements several mechanisms to detect potential assumption violations:

| Mechanism | Purpose | Implementation |
|-----------|---------|----------------|
| **Structural Validation** | Check graph properties | Cycle detection, connectivity checks |
| **Numerical Checks** | Detect precision issues | Underflow/overflow warnings, log-space computation |
| **Convergence Monitoring** | Detect algorithm failure | Iteration limits, residual tracking |
| **Sample Size Warnings** | Alert to insufficient data | Complexity vs. sample size comparison |
| **Model Diagnostics** | Check goodness-of-fit | Residual analysis, fit statistics |
| **Input Validation** | Catch invalid inputs | Type checking, range validation |

### Response to Violations

When assumptions are violated, Lutufi responds according to severity:

| Severity | Response | Example |
|----------|----------|---------|
| **Fatal** | Error with guidance | Cyclic graph for BN |
| **Warning** | Continue with caution | Small sample size |
| **Info** | Suggest alternatives | Algorithm selection recommendation |
| **Silent** | Handle automatically | Log-space conversion |

### User Guidance

Documentation provides guidance for each assumption:
- When the assumption typically holds
- How to check if it holds for your problem
- What to do if it doesn't hold
- Alternative approaches to consider

---

## Relaxing Assumptions

### Priority for Relaxation

Assumptions are prioritized for relaxation based on:

1. **User impact**: How often does this assumption fail in practice?
2. **Feasibility**: How difficult is relaxation to implement?
3. **Benefit**: How much would relaxation improve applicability?

| Assumption | Priority | Approach | Timeline |
|------------|----------|----------|----------|
| Faithfulness violations | High | FCI algorithm, robust methods | Phase 2 |
| Causal sufficiency | High | Full FCI implementation | Phase 2 |
| Sufficient memory | Medium | Out-of-core algorithms | Phase 3 |
| Sample size | Medium | Regularization, priors | Phase 2 |
| Network structure uncertainty | Medium | Probabilistic networks | Phase 3 |
| Positivity violations | Low | Bounds, partial identification | Future |

### Implementation Strategy

Relaxing assumptions follows this strategy:

1. **Document current assumption**: Clearly state what is required
2. **Implement detection**: Warn users when assumption may be violated
3. **Provide alternatives**: Offer methods that don't require the assumption
4. **Gradual enhancement**: Improve relaxation over releases

### Research Directions

Some assumptions require research advances to relax:

- **Faithfulness**: Algorithms robust to near-violations
- **Cyclic models**: Causal models with feedback loops
- **Non-modularity**: Interventions that change mechanisms
- **Long-range dependencies**: Efficient inference with global interactions

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-03-04 | Initial assumptions log with 20 documented assumptions across 6 categories |

---

## Cross-References

- [Open Questions](OPEN_QUESTIONS.md) — Questions about when assumptions hold
- [Design Decisions](DESIGN_DECISIONS.md) — Decisions that encode assumptions
- Mathematical foundations documented in `/docs/foundations/`
- Statistical assumptions tested in validation suite
