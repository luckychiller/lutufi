# Open Questions Log

**Document Version**: 1.0  
**Status**: Working Draft  
**Author**: Wasswa Lutufi Sebbanja  
**Last Updated**: March 2026  

---

## Table of Contents

1. [Purpose](#purpose)
2. [How to Use This Document](#how-to-use-this-document)
3. [Question Format](#question-format)
4. [Mathematical Questions](#mathematical-questions)
5. [Algorithmic Questions](#algorithmic-questions)
6. [Implementation Questions](#implementation-questions)
7. [Domain Questions](#domain-questions)
8. [API/Design Questions](#apidesign-questions)
9. [Questions for Domain Experts](#questions-for-domain-experts)
10. [Questions for Users](#questions-for-users)
11. [Resolved Questions](#resolved-questions)
12. [Priority System](#priority-system)

---

## Purpose

The Open Questions Log serves as a living document tracking unanswered questions, research gaps, design uncertainties, and areas requiring further investigation throughout the Lutufi project lifecycle. This document ensures that important questions are not lost and provides visibility into the project's intellectual frontier.

Questions in this log span multiple dimensions:
- **Theoretical**: Mathematical questions about algorithms, convergence, complexity
- **Practical**: Implementation choices, performance tradeoffs, API design
- **Domain**: Questions about how models apply to specific problem domains
- **Strategic**: Questions about project direction, prioritization, and scope

---

## How to Use This Document

### Adding Questions

When you encounter an unanswered question:

1. **Check for duplicates**: Search existing questions to avoid redundancy
2. **Assign an ID**: Use format `CATEGORY-NNN` (e.g., `MATH-001`, `ALGO-042`)
3. **Provide context**: Explain why the question matters and what triggered it
4. **Assign priority**: Use the priority system defined below
5. **Add metadata**: Date added, relevant components, tags

### Answering Questions

When a question is resolved:

1. **Move to Resolved Questions**: Do not delete—archive with the answer
2. **Document the answer**: Provide sufficient detail for future reference
3. **Update related code**: Ensure implementation reflects the resolution
4. **Cross-reference**: Link to design decisions or code changes

### Retiring Questions

Questions may be retired (without answer) if:
- The question becomes irrelevant due to scope changes
- The question is superseded by a better-formulated question
- The question is determined to be out of scope

Retired questions are moved to a "Retired" subsection with explanation.

---

## Question Format

All questions follow this standardized format:

```
### [ID]: [Brief Question Title]

**Question**: [Full question text]

**Context**: [Background and motivation]

**Priority**: [BLOCKER/HIGH/MEDIUM/LOW]

**Date Added**: [YYYY-MM-DD]

**Status**: [OPEN/IN-INVESTIGATION/ANSWERED/RETIRED]

**Tags**: [relevant, tags, for, categorization]

**Related**: [links to related questions, design decisions, or issues]

**Notes**: [Additional observations, partial answers, hypotheses]
```

---

## Mathematical Questions

### MATH-001: Improved Bounds on Loopy BP Convergence

**Question**: What are tighter sufficient conditions for loopy belief propagation convergence beyond the Dobrushin condition and walk-summability?

**Context**: Current convergence guarantees for loopy BP are either too restrictive (excluding practical cases where BP works) or too loose (including cases where BP diverges). Improved bounds would enable automatic algorithm selection with stronger guarantees.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: convergence, belief-propagation, theoretical-bounds

**Related**: ALGO-003 (algorithm selection criteria)

**Notes**: Recent work by Ruozzi and others on reweighted BP provides some directions. The local polytope perspective may offer new analysis tools.

---

### MATH-002: Exact Complexity of Junction Tree Construction

**Question**: What are the precise complexity bounds for optimal triangulation and junction tree construction, parameterized by treewidth?

**Context**: The standard O(n * exp(w)) bound is loose for many graph classes. Tighter bounds would inform algorithm selection and help users understand when exact inference is feasible.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: complexity, junction-tree, treewidth, exact-inference

**Related**: ALGO-005 (treewidth heuristics)

**Notes**: Fixed-parameter tractability results exist but are not tight. Need to survey recent parameterized complexity literature.

---

### MATH-003: Faithfulness Testing

**Question**: Can we develop statistical tests for the faithfulness assumption that have acceptable power in practical sample sizes?

**Context**: Faithfulness (conditional independences in data match those in the true graph) is required for constraint-based causal discovery but is untestable in finite samples. Practical diagnostic tests would improve confidence in structure learning results.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: faithfulness, causal-discovery, statistical-testing, structure-learning

**Related**: ASSUMPTIONS-002 (faithfulness assumption documentation)

**Notes**: Uhler et al. have work on geometry of faithfulness violations. May need to settle for detecting near-violations rather than violations.

---

### MATH-004: Scalable Causal Effect Identification

**Question**: What is the computational complexity of the complete do-calculus identification algorithm, and can it be made polynomial for bounded-degree graphs?

**Context**: Shpitser and Pearl provided complete algorithms but complexity analysis is incomplete. Scalable identification is essential for large causal networks.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: causal-inference, do-calculus, computational-complexity, scalability

**Related**: MATH-002, DOMAIN-003 (economic network causal queries)

**Notes**: The identification problem reduces to graph reachability in some cases but general complexity remains open.

---

### MATH-005: Hybrid Network Inference

**Question**: What are convergence guarantees for mixed discrete-continuous belief propagation algorithms that combine particle and parametric representations?

**Context**: Lutufi needs to support hybrid networks (discrete and continuous variables). Nonparametric belief propagation provides one approach but convergence theory is weak.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: hybrid-networks, nonparametric-BP, continuous-variables, convergence

**Related**: IMPL-007 (sparse matrix formats for hybrid networks)

**Notes**: Sudderth et al. provide empirical success but limited theory. May need to develop Lutufi-specific theory for our hybrid approach.

---

### MATH-006: Network Resilience Phase Transitions

**Question**: Can we characterize phase transitions in network resilience (percolation thresholds) for realistic network models beyond configuration models?

**Context**: Understanding when networks undergo catastrophic failures is essential for financial contagion and infrastructure applications. Current theory is strongest for random graph models that don't match real network properties.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: network-resilience, percolation, phase-transitions, financial-networks

**Related**: DOMAIN-002 (financial contagion thresholds)

---

### MATH-007: Causal Effect Bounds Under Non-Identification

**Question**: When causal effects are not point-identified from observed data, what are the tightest possible bounds, and can they be computed efficiently?

**Context**: Many practical causal queries are not point-identified. Providing bounds rather than point estimates is more honest but requires efficient computation methods.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: causal-bounds, partial-identification, optimization, partial-identification

**Related**: MATH-004

---

## Algorithmic Questions

### ALGO-001: Best Heuristic for Treewidth Estimation

**Question**: Which treewidth estimation heuristic provides the best tradeoff between accuracy and computation time for real-world network structures?

**Context**: Exact treewidth computation is NP-hard, but many heuristics exist (min-fill, min-degree, metaheuristics). The choice affects junction tree construction quality and inference performance.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: treewidth, heuristic, junction-tree, algorithm-selection

**Related**: MATH-002, IMPL-002 (sparse representation impact on treewidth)

**Notes**: Need empirical evaluation on network science benchmark graphs. Gogate & Dechter's anytime methods may be relevant.

---

### ALGO-002: Adaptive Algorithm Selection

**Question**: Can we develop lightweight predictors that select the best inference algorithm (loopy BP, Gibbs, mean field, etc.) based on graph properties without running all candidates?

**Context**: Lutufi plans automatic algorithm selection. Running multiple algorithms and comparing is expensive. Predicting the best algorithm from graph structure would be more efficient.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: algorithm-selection, meta-learning, inference-optimization

**Related**: MATH-001 (convergence bounds for BP)

**Notes**: Could use features like treewidth, clustering coefficient, degree distribution. May need ML-based meta-learner trained on benchmarks.

---

### ALGO-003: Parallel Inference Load Balancing

**Question**: What dynamic load balancing strategy minimizes makespan for parallel belief propagation on irregular graphs?

**Context**: Factor graphs from real networks have highly irregular structure. Static partitioning leads to load imbalance. Dynamic strategies may have overhead that negates benefits.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: parallelization, load-balancing, belief-propagation, performance

**Related**: IMPL-004 (GPU acceleration strategy)

---

### ALGO-004: Incremental Inference

**Question**: How can we efficiently update inference results when the network structure or parameters change incrementally (few edges/nodes added)?

**Context**: Many applications (streaming networks, interactive modeling) require updating beliefs after small changes. Full recomputation is wasteful.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: incremental-inference, streaming, dynamic-networks, efficiency

**Related**: DOMAIN-006 (temporal network updates)

---

### ALGO-005: Structure Learning Scalability

**Question**: What is the largest network (nodes/edges) for which we can tractably learn structure with statistical guarantees, and how can we extend this limit?

**Context**: Structure learning for Bayesian networks is computationally demanding. Understanding scalability limits helps set user expectations and guide feature development.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: structure-learning, scalability, computational-limits, learning-guarantees

**Related**: MATH-003 (faithfulness testing)

---

### ALGO-006: Approximate Causal Discovery

**Question**: Can we develop approximate causal discovery algorithms with polynomial-time guarantees that provide useful outputs even when exact methods fail?

**Context**: Exact causal discovery is infeasible for large networks. Approximate methods exist but lack theoretical guarantees. Users need to know when outputs are reliable.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: causal-discovery, approximation, polynomial-time, guarantees

**Related**: ALGO-005

---

## Implementation Questions

### IMPL-001: Optimal Sparse Matrix Format

**Question**: What sparse matrix format (CSR, CSC, COO, custom) provides the best performance for factor graph operations across our target use cases?

**Context**: Factor graphs are sparse by nature. Matrix format affects memory usage, cache efficiency, and operation performance. Different operations may favor different formats.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: IN-INVESTIGATION

**Tags**: sparse-matrices, data-structures, performance, memory-layout

**Related**: DESIGN-005 (sparse representation strategy)

**Notes**: Initial benchmarks suggest CSR for factor storage, COO for message passing. Need comprehensive evaluation.

---

### IMPL-002: Rust-Python FFI Overhead

**Question**: What is the practical overhead of Rust-Python FFI calls, and what batching strategies minimize it for interactive use cases?

**Context**: Lutufi uses Python as primary interface with Rust core. FFI overhead could limit interactivity. Understanding this informs API granularity decisions.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: IN-INVESTIGATION

**Tags**: ffi, python-bindings, performance-overhead, pyo3

**Related**: DESIGN-004 (Python as primary interface)

**Notes**: Preliminary tests suggest overhead is acceptable for batch operations but may impact tight loops. Need systematic measurement.

---

### IMPL-003: Serialization Format Tradeoffs

**Question**: Should Lutufi use Protocol Buffers, MessagePack, Cap'n Proto, or a custom binary format for model serialization?

**Context**: Serialization affects file size, loading speed, and cross-language compatibility. The choice has long-term implications for file format stability.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: IN-INVESTIGATION

**Tags**: serialization, binary-formats, interoperability, file-format

**Related**: DESIGN-009 (serialization format decisions)

**Notes**: Custom format offers maximum control but requires maintenance. Cap'n Proto offers zero-copy but adds dependency. MessagePack simpler but less efficient.

---

### IMPL-004: GPU Acceleration Strategy

**Question**: Which inference algorithms benefit sufficiently from GPU acceleration to justify the implementation complexity and hardware requirements?

**Context**: GPUs can accelerate matrix operations but belief propagation has irregular memory access patterns. Not all algorithms will benefit equally.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: gpu, acceleration, cuda, performance, hardware

**Related**: ALGO-003 (parallel load balancing)

---

### IMPL-005: Error Propagation Strategy

**Question**: How should numerical errors propagate through inference computations, and what precision guarantees can we provide users?

**Context**: Iterative algorithms accumulate numerical error. Users need to know result reliability. Error bounds would be valuable but are computationally expensive.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: numerical-stability, error-propagation, precision, reliability

**Related**: DESIGN-008 (error handling approach)

---

### IMPL-006: Testing Strategy for Stochastic Algorithms

**Question**: How can we effectively test algorithms with stochastic components (MCMC, stochastic BP) that produce non-deterministic outputs?

**Context**: Stochastic algorithms are essential for approximate inference but resist standard unit testing approaches. Need statistical testing frameworks.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: testing, stochastic-algorithms, mcmc, quality-assurance

**Related**: DESIGN-010 (testing strategy)

---

### IMPL-007: Memory-Constrained Inference

**Question**: What strategies enable inference on networks that exceed available RAM (out-of-core, streaming, compression)?

**Context**: Large networks may not fit in memory. Out-of-core algorithms would extend applicability but add complexity and potentially reduce performance.

**Priority**: LOW

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: memory-management, out-of-core, streaming, large-networks

**Related**: SCALABILITY-001 (documented in separate roadmap)

---

## Domain Questions

### DOMAIN-001: Validating Financial Contagion Models

**Question**: What empirical validation approaches can establish confidence in financial contagion predictions without access to confidential bank data?

**Context**: Financial contagion models are difficult to validate due to data confidentiality and rarity of systemic events. Synthetic data and partial validation are possible but unsatisfying.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: financial-contagion, validation, empirical-testing, banking-networks

**Related**: DOCS-Financial-Contagion (domain documentation)

---

### DOMAIN-002: Threshold for Financial Networks

**Question**: What network density and connectivity thresholds mark the transition between stable and vulnerable financial networks?

**Context**: Understanding when financial networks become vulnerable to contagion is essential for macroprudential policy. Current models give theoretical results but practical thresholds depend on unobserved parameters.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: financial-networks, contagion-thresholds, systemic-risk, phase-transitions

**Related**: MATH-006 (resilience phase transitions)

---

### DOMAIN-003: Causal Queries in Economic Networks

**Question**: What are the most common causal queries in economic network analysis, and can they be expressed within the do-calculus framework?

**Context**: Economic networks involve complex causal questions (policy effects, shock propagation). Understanding typical queries helps prioritize causal inference features.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: economic-networks, causal-queries, policy-analysis, do-calculus

**Related**: MATH-004 (causal effect identification)

---

### DOMAIN-004: Social Influence Model Selection

**Question**: Which social influence models (threshold, cascade, independent cascade, etc.) best fit different types of empirical diffusion data?

**Context**: Multiple influence models exist but guidance on model selection is limited. Empirical guidance would help users choose appropriate models.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: social-influence, diffusion-models, model-selection, empirical-validation

**Related**: DOCS-Social-Influence (domain documentation)

---

### DOMAIN-005: Misinformation Network Data

**Question**: What publicly available datasets enable validation of misinformation propagation models while respecting privacy?

**Context**: Misinformation research is limited by data access. Public datasets are often small or unrepresentative. Need guidance on available data sources.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: misinformation, datasets, privacy, empirical-validation

**Related**: DOCS-Misinformation (domain documentation)

---

### DOMAIN-006: Temporal Network Update Rates

**Question**: What are realistic time scales for network evolution in different domains (social, financial, organizational), and how do they affect algorithm design?

**Context**: Temporal networks evolve at different rates. Social networks change daily; financial networks change transaction-by-transaction. Algorithm design should match domain requirements.

**Priority**: LOW

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: temporal-networks, time-scales, streaming, domain-requirements

**Related**: ALGO-004 (incremental inference)

---

## API/Design Questions

### API-001: Counterfactual Query Interface

**Question**: What is the most intuitive API for counterfactual queries ("what if X had been different") that remains consistent with the probabilistic interface?

**Context**: Counterfactuals are powerful but conceptually complex. The API must make them accessible without misleading users about assumptions.

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: api-design, counterfactuals, usability, causal-inference

**Related**: MATH-004, DOMAIN-003

---

### API-002: Streaming Data Interface

**Question**: What is the appropriate abstraction for streaming network data that enables both online algorithms and batch processing of windows?

**Context**: Streaming data requires different abstractions than static networks. The API should support both incremental updates and windowed processing.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: api-design, streaming, temporal-networks, data-flow

**Related**: ALGO-004, DOMAIN-006

---

### API-003: Interoperability Standards

**Question**: Which existing standards (if any) should Lutufi support for network data interchange (GraphML, GEXF, etc.)?

**Context**: Interoperability with existing tools is valuable but supporting too many formats creates maintenance burden. Need to prioritize based on user needs.

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: api-design, interoperability, file-formats, standards

**Related**: IMPL-003 (serialization format)

---

### API-004: Visualization Integration

**Question**: Should Lutufi include built-in visualization, delegate to external tools, or provide a bridge layer?

**Context**: Visualization is essential for understanding networks but requires different expertise and dependencies than core inference. Design choice affects maintainability.

**Priority**: LOW

**Date Added**: 2026-03-04

**Status**: OPEN

**Tags**: api-design, visualization, scope, dependencies

**Related**: PROJECT-Visualization-Tool (potential project)

---

## Questions for Domain Experts

### EXPERT-001: Statisticians

**Questions to ask consulting statisticians:**

1. What are the most common violations of causal inference assumptions in observational network studies, and how can they be detected?
2. How should we handle network-based dependence in confidence intervals and hypothesis tests?
3. What sample size guidelines apply for reliable structure learning in sparse networks?

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

---

### EXPERT-002: Network Scientists

**Questions to ask practicing network scientists:**

1. What network properties most strongly affect the performance of inference algorithms in practice?
2. How do you validate probabilistic models of network dynamics when ground truth is unavailable?
3. What are the most important network measures that current tools fail to compute efficiently?

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: OPEN

---

### EXPERT-003: Economists

**Questions to ask economists working with network models:**

1. What are the most pressing computational bottlenecks in current economic network research?
2. How important is endogeneity handling in your network applications, and what approaches work?
3. What would make a network analysis library valuable for economic policy analysis?

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

---

### EXPERT-004: Software Engineers

**Questions to ask experienced scientific software engineers:**

1. What are proven patterns for maintaining numerical stability across algorithm implementations?
2. How should we balance API stability with the need to evolve based on research advances?
3. What testing strategies have worked for complex probabilistic software systems?

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: OPEN

---

## Questions for Users

### USER-001: Workflow Integration

**Research question**: How do users currently integrate network analysis into their workflows, and what friction points exist?

**Investigation approach**: User interviews with researchers in target domains

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: PLANNED

---

### USER-002: Documentation Needs

**Research question**: What background knowledge do users have, and what documentation depth is appropriate?

**Investigation approach**: Survey of potential users about their mathematical and programming backgrounds

**Priority**: HIGH

**Date Added**: 2026-03-04

**Status**: PLANNED

---

### USER-003: Performance Expectations

**Research question**: What network sizes do users need to analyze, and what inference latency is acceptable?

**Investigation approach**: Analysis of published network studies to determine typical problem sizes

**Priority**: MEDIUM

**Date Added**: 2026-03-04

**Status**: PLANNED

---

### USER-004: Visualization Requirements

**Research question**: What visualization capabilities are essential versus nice-to-have?

**Investigation approach**: Analysis of network analysis papers to see what visualizations are commonly produced

**Priority**: LOW

**Date Added**: 2026-03-04

**Status**: PLANNED

---

## Resolved Questions

### RES-001: Core Implementation Language

**Original Question**: Should Lutufi's core be implemented in C++, Rust, or Julia?

**Answer**: Rust was selected for the core implementation.

**Resolution Date**: 2025-12

**Rationale**: Rust provides memory safety without garbage collection, excellent performance, strong ecosystem for scientific computing, and easier Python integration than C++. The borrow checker prevents common bugs in high-performance numerical code.

**Documented In**: DESIGN-001 (Core Language Choice)

---

### RES-002: Internal Representation

**Original Question**: Should the internal representation be Bayesian networks (directed), Markov random fields (undirected), or factor graphs?

**Answer**: Factor graphs as canonical internal representation.

**Resolution Date**: 2026-01

**Rationale**: Factor graphs can represent both directed and undirected models, provide uniform treatment of inference algorithms, and enable efficient message-passing implementations. Conversions from BN and MRF representations are well-defined.

**Documented In**: DESIGN-002 (Canonical Representation)

---

### RES-003: Primary Interface Language

**Original Question**: What should be the primary user-facing language?

**Answer**: Python with optional Rust API for advanced users.

**Resolution Date**: 2026-01

**Rationale**: Python is the dominant language for scientific computing and network analysis. It provides the best ecosystem integration (NumPy, SciPy, NetworkX, pandas). Rust is available for performance-critical extensions or systems integration.

**Documented In**: DESIGN-003 (Python as Primary Interface)

---

## Priority System

### BLOCKER
- Prevents development progress on critical path
- Must be resolved within 2 weeks
- Examples: Architecture decisions blocking implementation, fundamental algorithmic uncertainties

### HIGH
- Significantly impacts design or implementation
- Should be resolved within 1 month
- Examples: Performance-critical algorithm choices, API design for core features

### MEDIUM
- Important but not time-critical
- Should be resolved within 3 months
- Examples: Optimization opportunities, extended feature design

### LOW
- Would be nice to answer but not required
- No specific timeline
- Examples: Future research directions, nice-to-have optimizations

---

## Summary Statistics

| Category | Open | In Investigation | Answered | Total |
|----------|------|------------------|----------|-------|
| Mathematical | 7 | 0 | 0 | 7 |
| Algorithmic | 6 | 0 | 0 | 6 |
| Implementation | 5 | 2 | 0 | 7 |
| Domain | 6 | 0 | 0 | 6 |
| API/Design | 4 | 0 | 0 | 4 |
| Expert Questions | 4 | 0 | 0 | 4 |
| User Research | 0 | 0 | 4 (planned) | 4 |
| **Total** | **32** | **2** | **3** | **38** |

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-03-04 | Initial open questions log with 38 questions across all categories |
