# Lutufi Library Guardrails
### The Complete Development Constitution

**Version:** 2.0  
**Status:** Authoritative — every line of Lutufi code is subject to these rules  
**Principle:** These guardrails exist not to constrain you but to protect the people who will depend on Lutufi for their research.

---

> When you are uncertain whether to do something, ask: *would a careful, senior researcher trust a result produced by a library that behaves this way?* If the answer is no, the guardrail applies.

---

## How to Use This Document

This document is divided into thirteen sections. Each guardrail has a number, a rule, and a rationale. The rationale is not decoration — it explains *why* the rule exists so that when you encounter a case the rule does not perfectly cover, you can reason from the principle rather than from the letter.

Rules marked **[HARD]** are non-negotiable. Violating them is a bug that must be fixed before any code is merged.  
Rules marked **[STRONG]** require explicit documented justification to deviate from.  
Rules marked **[GUIDE]** are strong defaults that may be overridden with team agreement.

---

## Section 1 — Numerical Integrity

*The most important section. A library that produces numerically incorrect results is worse than no library at all — it produces confident wrong answers.*

**N1. [HARD] All probability arithmetic runs in log-space.**  
Never multiply probabilities directly. Always work in log-space and convert back only at output boundaries. A chain of twenty 0.9 probabilities multiplied naively gives `0.9^20 ≈ 0.12`. A chain of two hundred gives `0.9^200 ≈ 7e-10`, which underflows to zero on many probability chains in real networks, silently corrupting every inference result downstream. Log-space converts multiplication to addition, which never underflows. There are no exceptions to this rule.

**N2. [HARD] Use the log-sum-exp trick for all log-space summations.**  
When summing quantities in log-space, never exponentiate, sum, and re-take the log naively. Use `log_sum_exp(a, b) = a + ln(1 + exp(b - a))` where `a = max(a, b)`. This maintains numerical stability regardless of the magnitude of the inputs. Every marginalization, every normalization, every partition function computation uses this. Write the function once, test it exhaustively, use it everywhere.

**N3. [HARD] Never produce NaN or Inf silently.**  
If a computation produces NaN or Inf, it must immediately surface as a `LutufiNumericalError` with the exact location (which factor, which variable, which operation) and the inputs that caused it. A NaN propagating silently through ten inference steps and appearing in the final result is a research integrity failure. Instrument every numerical boundary.

**N4. [HARD] Never compare floating point values with `==`.**  
Use tolerance-based comparison throughout. Standard tolerance for probability values: `1e-9`. Standard tolerance for log-probabilities: `1e-7`. Standard tolerance for CPT normalization checks: `1e-9`. Define these as named constants in a `numerics.rs` module, not as magic numbers scattered through the codebase.

```rust
// numerics.rs — single source of truth for all tolerances
pub const PROBABILITY_TOLERANCE: f64 = 1e-9;
pub const LOG_PROBABILITY_TOLERANCE: f64 = 1e-7;
pub const CPT_NORMALIZATION_TOLERANCE: f64 = 1e-9;
pub const CONVERGENCE_TOLERANCE: f64 = 1e-8;

pub fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() <= tolerance
}

pub fn is_valid_probability(p: f64) -> bool {
    p >= 0.0 && p <= 1.0 + PROBABILITY_TOLERANCE
}
```

**N5. [HARD] Validate probability values at all entry points.**  
Any value that enters Lutufi as a probability must be validated: it must be non-negative, finite, and not NaN. Any value that enters as a CPT must be validated to sum to 1.0 within `CPT_NORMALIZATION_TOLERANCE` for each parent configuration. This validation happens at the Rust-Python boundary before any computation begins.

**N6. [STRONG] Use sparse storage when factor density is below 30%.**  
Large CPTs in social network models — especially for variables with many parents — are often sparse. A full dense array wastes memory and slows computation. When the fraction of non-zero entries is below 30%, use sparse COO (coordinate) format for storage and convert to CSR for computation. The threshold and format choice live in `numerics.rs`.

**N7. [GUIDE] Prefer f64 over f32 throughout.**  
The memory savings of f32 are not worth the precision loss in probabilistic inference. All internal computations use f64. The only exception is when explicitly implementing a memory-optimized path for very large networks, which must be clearly documented and tested for precision degradation.

---

## Section 2 — Technical & Architectural

**A1. [HARD] Never use `unwrap()` or `expect()` in library code.**  
Every `unwrap()` is a promise that this code path will never fail, signed in the blood of your users. You cannot keep that promise. Use `?` for error propagation, `ok_or_else()` to convert `Option` to `Result` with a meaningful error, and `match` for cases that genuinely require branching. `expect()` is permitted only in test code and with a message that describes what invariant was violated.

**A2. [HARD] Follow the single responsibility principle at module level.**  
No module does more than one thing. `inference/variable_elimination.rs` implements variable elimination. It does not also implement junction trees. It does not define data structures. If you find yourself writing "and also" in a module description, the module needs to be split. The test for this: can you explain what this module does in one sentence without using "and"?

**A3. [STRONG] Composition over inheritance. Always.**  
Rust does not have inheritance and this is correct. Model behaviors through trait composition. An inference algorithm is a type that implements the `InferenceEngine` trait. A serializer is a type that implements the `ModelSerializer` trait. A model is a type that implements the `GraphModel` trait. Behavior is added by implementing traits, not by subclassing. The Factor Graph is composed of Variable nodes and Factor nodes. It does not inherit from either.

**A4. [STRONG] Apply the Strategy pattern for all algorithm selection.**  
Every place Lutufi makes an algorithmic choice — inference algorithm, elimination ordering, structure learning method, missing data handler — that choice is represented as a strategy object implementing a trait. The `InferenceOrchestrator` selects strategies. Strategies are composable, testable, and swappable without touching the code that uses them.

**A5. [STRONG] Use the Builder pattern for all complex object construction.**  
Any type with more than three configuration parameters uses a builder. The builder validates as it accumulates (fail fast on invalid configuration) and produces a fully valid object on `build()`. Builders are the only place where partial state is acceptable. Constructed objects are always valid.

**A6. [STRONG] Use newtypes for domain-specific identifiers.**  
`VariableId`, `FactorId`, `ClusterNodeId`, `MessageId` are all distinct newtypes wrapping UUIDs. They are not interchangeable. The compiler enforces this. A function that takes a `VariableId` cannot accidentally receive a `FactorId`. This eliminates an entire class of bugs that would be silent at runtime.

**A7. [STRONG] Maintain modularity: no file exceeds 500 lines.**  
A file that grows beyond 500 lines is communicating that it is doing too much. Split it. The threshold is a signal, not a hard limit — a 520-line file with a single clean responsibility is acceptable. A 480-line file that does three unrelated things is not. When in doubt, split.

**A8. [GUIDE] Prefer the most correct algorithm over the fastest algorithm.**  
Lutufi's value is correctness first, performance second. An approximate algorithm should only be preferred when the exact algorithm is computationally infeasible, not merely slower. The `InferenceOrchestrator` automatically makes this determination — individual algorithm implementations should not optimize for speed by sacrificing correctness.

**A9. [GUIDE] Lazy evaluation for expensive operations.**  
Junction tree compilation, treewidth estimation, factor graph construction from a Bayesian network — these are expensive and should not run until a result is actually needed. Cache compiled structures and invalidate the cache on structural changes. Never recompute what has not changed.

---

## Section 3 — Automatic vs Explicit: Resolving the Tension

*This section replaces guardrail #14 from the original document and clarifies a real tension in the design.*

**E1. [HARD] The user specifies WHAT. Lutufi determines HOW.**  
The user's domain is the research question: which variables to query, what evidence to condition on, what intervention to apply. Lutufi's domain is computational mechanism: which algorithm to use, what data structure to compile, how much memory to allocate. Treewidth, junction tree compilation, message scheduling, elimination ordering — these are HOW concepts. They never appear in the standard user API. The user writes `model.query(variables, evidence)`. Lutufi does the rest.

**E2. [HARD] Automatic algorithm selection is the default. Manual override is the escape hatch.**  
The `InferenceOrchestrator` selects the best algorithm automatically based on network profile and resource budget. The user may override this with `algorithm="junction_tree"` or `exact=True` only if they have specific reasons. The override path is always available but never the default. This is the resolution of the original "explicit over implicit" rule — be explicit about *what*, automatic about *how*.

**E3. [STRONG] Automatic decisions must be transparent on request.**  
Every automatic decision Lutufi makes is recorded in the `InferenceMetadata` attached to every result. The user can always inspect `result.metadata` to see exactly which algorithm was selected, why (the network profile that drove the decision), and what the quality guarantees are. Automatic does not mean opaque.

**E4. [STRONG] Fail fast and loudly at the user-facing boundary.**  
Validate all inputs — domain values, CPT shapes, evidence types, variable names — at the Python-Rust FFI boundary before any computation begins. An invalid input should surface as a clear Python exception within microseconds, not after minutes of failed computation deep inside the inference engine. The cost of validation is always worth paying.

**E5. [GUIDE] Provide progressive disclosure of complexity.**  
The simplest use case requires the simplest code. Advanced use cases are possible but require explicit opt-in. A researcher who just wants marginal inference writes three lines. A researcher who needs fine-grained control over inference can access it — but they have to ask for it. Design the API in concentric circles: the center circle is simple and covers 80% of use cases, outer circles add power at the cost of complexity.

---

## Section 4 — Scientific & Causal Integrity

**S1. [HARD] Correlation is not causation. Enforce the boundary in code.**  
The `do()` operator, `counterfactual()`, `probability_of_necessity()`, `probability_of_sufficiency()`, `identify()`, and any other causal operation must raise `LutufiNonCausalModelError` if called on a model not explicitly marked as causal via `mark_as_causal()`. This check happens before any computation. There are no exceptions. A researcher who accidentally runs do-calculus on an associational model will get a wrong answer with high confidence — this is the single most dangerous error mode in causal inference software.

**S2. [HARD] Identifiability must be checked before causal effect estimation.**  
Before computing `P(Y | do(X))`, run the ID algorithm to verify the causal effect is identifiable from observational data given the graph structure. If the effect is not identifiable, raise `LutufiNotIdentifiableError` with the specific hedge structure (subgraph) that proves non-identifiability. Never return a numerical estimate for a non-identifiable causal query — the number would be meaningless.

**S3. [HARD] Models must pass structural validation before inference.**  
A `BayesianNetwork` with missing CPTs, a `MarkovRandomField` with disconnected factors, a `DynamicBayesianNetwork` with inconsistent time-slice definitions — none of these may run inference. The `validate()` method runs before every inference call and returns all errors found, not just the first one. The error list is the actionable output.

**S4. [HARD] All inference results carry uncertainty. No point estimates without distributions.**  
Lutufi never returns a single number as an inference result. It returns a distribution. If a user asks for the most probable state of a variable, they get the distribution first and the MAP state as a convenience attribute on the result. The distribution is always the primary output. This is not pedantry — a researcher who sees only the MAP estimate and not its probability may draw very different conclusions than one who sees the full distribution.

**S5. [STRONG] Approximate inference results declare their approximation.**  
Every result from an approximate algorithm carries: the algorithm used, whether convergence was achieved, the convergence residual, the number of iterations, and a calibrated statement of expected accuracy. A result from loopy belief propagation on a network where LBP is known to be inaccurate (networks with many short cycles) must state this. The user may proceed — but they may not proceed without knowing.

**S6. [STRONG] The faithfulness assumption must be disclosed in structure learning.**  
All constraint-based structure learning algorithms (PC, FCI) rely on the faithfulness assumption — that every conditional independence in the data reflects a missing edge in the true graph. This assumption can fail. When returning a learned structure, attach a `StructureLearningMetadata` that states: the algorithm used, the faithfulness assumption it relies on, the sample size, the significance level used for independence tests, and a warning that the result is the Markov equivalence class, not a unique causal graph.

**S7. [STRONG] EM convergence is a property of the algorithm, not a guarantee of model correctness.**  
When EM converges, it has found a local maximum of the likelihood. It has not necessarily found the correct parameters. Document this clearly in the API. When EM terminates, report: whether it converged, the number of iterations, the final log-likelihood change, and a note that the result may be a local optimum. Offer multiple restarts as the mitigation.

**S8. [STRONG] D-separation testing must be exact.**  
The Bayes Ball algorithm for d-separation is exact. Never approximate it. A wrong d-separation result will silently mislead researchers about what their model implies about variable relationships. This is a correctness requirement, not a performance one. Even for networks with millions of nodes, d-separation queries on specific variable pairs are tractable.

**S9. [GUIDE] Distinguish between Markov equivalence and causal identification.**  
Observational data can only identify the Markov equivalence class of a DAG, not a unique causal graph. PC and GES return a CPDAG (completed partially directed acyclic graph) representing the equivalence class, not a unique DAG. The API should surface this distinction — do not automatically orient all edges and imply a unique causal structure has been identified. FCI handles hidden confounders and returns a PAG. Communicate what each output type means.

---

## Section 5 — Testing & Verification

*A scientific library without a rigorous testing philosophy is not a scientific library. It is a script with packaging.*

**T1. [HARD] Every inference algorithm has a ground truth verification test.**  
Before any inference algorithm is considered complete, it must pass tests against known exact analytical results. These live in `tests/ground_truth/`. For variable elimination: the Asia network marginals match Lauritzen & Spiegelhalter's analytical results within `1e-10`. For LBP on trees: results match junction tree results within `1e-10` (LBP is exact on trees). For MCMC: results match exact inference within 2% at 50,000 samples. These tests run on every CI build. If any fails, the build fails.

**T2. [HARD] Tests are written before the implementation they test.**  
Write the test. Watch it fail. Implement the code. Watch the test pass. This is not ceremonial — it ensures the test actually exercises the code and that the code was written to satisfy the test rather than the test written to confirm the code. For mathematical algorithms, the test is a precise specification of what "correct" means.

**T3. [HARD] No silent test skips.**  
A skipped test is a known failure that has been hidden. If a test cannot run in CI (e.g., requires a GPU, requires a large dataset), it must be tagged and the reason documented in the test. `#[ignore = "requires 32GB RAM: run manually with cargo test -- --include-ignored"]` is acceptable. `#[ignore]` with no explanation is a violation.

**T4. [STRONG] Every public function has at least one unit test.**  
Coverage is not the metric — thoughtful test cases are. But no public function ships without at least one test that exercises its primary behavior and at least one test that exercises its primary failure mode.

**T5. [STRONG] Property-based testing for all mathematical operations.**  
Factor marginalization, factor product, CPT normalization, d-separation — these operations have mathematical properties that must hold for all valid inputs, not just the specific inputs in unit tests. Use `proptest` to generate random valid inputs and assert properties:
- Marginalization is associative: marginalizing X then Y equals marginalizing Y then X.
- Factor product is commutative: `f * g == g * f` up to variable ordering.
- Normalization is idempotent: normalizing a normalized factor produces the same factor.

**T6. [STRONG] Fuzz all file format parsers.**  
LMF, BIF, XMLBIF, UAI parsers must survive 24 hours of fuzz testing without crashing, panicking, or producing incorrect results from malformed input. A parser that panics on malformed input is a security vulnerability. Use `cargo-fuzz` with coverage-guided fuzzing. Run this before every major release.

**T7. [STRONG] Benchmark regression testing on every release.**  
Before any release, run the full benchmark suite. Compare to the previous release baseline. Any regression greater than 10% on any benchmark blocks the release until explained and justified. Performance regressions are bugs.

**T8. [GUIDE] Integration tests test the pipeline, not individual components.**  
A unit test for factor marginalization checks that one operation is correct. An integration test builds a complete network, loads real data, runs inference, and checks that the result is scientifically correct. Both are necessary. Neither replaces the other. The integration test suite lives in `tests/integration/` and exercises the full research workflow end-to-end.

---

## Section 6 — Performance & Scalability

**P1. [HARD] Profile before optimizing.**  
No performance optimization is written without first profiling the code under a realistic workload and identifying the actual bottleneck. Optimizing code that is not a bottleneck wastes time and adds complexity. Use `perf`, `flamegraph`, or `cargo-flamegraph` to identify hot paths. Document what you found and what you changed in a comment at the optimization site.

**P2. [HARD] Performance targets are verified by tests, not assumed.**  
The roadmap specifies: inference on a 100K-node sparse network in under 60 seconds; 1M+ edge support; incremental update 10x faster than full recompilation. These are requirements, not aspirations. They are verified by benchmark tests in `benches/` that run against synthetic networks of the specified sizes. If a target is not met, it is a bug.

**P3. [STRONG] Parallelism is correct before it is fast.**  
Parallel message passing, parallel factor products, parallel structure search — all of these must first be verified correct on single-threaded execution, then parallelized. Parallelism introduces ordering dependencies, race conditions, and non-determinism. Each parallel algorithm must have a `--single-threaded` test mode that produces bit-identical results to the parallel mode on the same input.

**P4. [STRONG] Memory allocation in hot paths must be justified.**  
The inference hot path — message passing, factor evaluation, marginalization — should not allocate heap memory on every call. Pre-allocate message buffers at junction tree compilation time and reuse them across queries. When you find yourself calling `Vec::new()` inside a loop that runs millions of times, stop and redesign.

**P5. [STRONG] Out-of-core computation is a first-class concern, not an afterthought.**  
Networks that exceed available RAM are not an edge case — they are the reality for financial crime networks and large social networks. Design data structures with memory-mapped backing from the start. The `TabularFactor` must support: in-memory storage for small factors, memory-mapped storage for large factors. The choice is automatic based on factor size relative to available RAM.

**P6. [GUIDE] Benchmark against pgmpy and bnlearn on standard networks.**  
For every release, publish benchmark comparisons against pgmpy on the Asia, Alarm, and Sachs networks. This is not competitive posturing — it is evidence that the compiled core delivers the expected performance advantage. It also catches regressions that would not be visible from absolute timings alone.

---

## Section 7 — Memory Management

**M1. [HARD] No memory leaks in the Rust core.**  
Rust's ownership model makes this largely automatic, but PyO3 reference-counted objects crossing the FFI boundary require care. Every Python object that holds a reference to a Rust object must properly release that reference when dropped. Run `valgrind` or use Rust's built-in leak detection (`RUSTFLAGS="-Z sanitizer=leak"`) on the test suite before every release.

**M2. [STRONG] Large factor tables are reference-counted, not copied.**  
Factors are frequently shared between inference algorithms — the same CPT may be referenced by variable elimination, the junction tree, and the factor graph simultaneously. Use `Arc<TabularFactor>` (atomically reference-counted) for shared ownership. Clone the Arc (cheap: increments a counter), not the underlying data (expensive: copies the table).

**M3. [STRONG] Define and enforce memory budgets for inference operations.**  
The `ResourceBudget` struct specifies the maximum RAM Lutufi may use for inference data structures. The `InferenceOrchestrator` checks estimated memory against this budget before attempting junction tree compilation. If the budget would be exceeded, fall back to a lower-memory algorithm. Never let Lutufi silently use all available system RAM.

**M4. [GUIDE] Prefer stack allocation for small fixed-size structures.**  
Variable domains with fewer than 16 states, small assignment maps, message buffers for binary variables — these can be stack-allocated using fixed-size arrays rather than heap-allocated Vecs. This eliminates allocator overhead in the hot path. Use `arrayvec` or `smallvec` for collections whose maximum size is known at compile time.

---

## Section 8 — Error Handling & Observability

**Err1. [HARD] Every error carries actionable context.**  
An error message that says "inference failed" is useless. An error message that says "Junction tree compilation ran out of memory for variable 'NetworkExposure' (estimated 4.2GB required, budget 2.0GB). Tip: use `algorithm='lbp'` for networks with high treewidth, or increase the memory budget with `ResourceBudget(max_memory_gb=8)`" is useful. Every `LutufiError` variant carries the specific context and a suggestion for resolution.

**Err2. [HARD] Errors are structured types, not strings.**  
`LutufiError` is an enum with typed variants. Each variant carries typed fields, not format strings. This allows programmatic handling — a user can `match` on error type, an IDE can provide autocompletion on error fields, a test can assert on the specific error variant. `LutufiError::CptNotNormalized { variable, parent_config, actual_sum }` is right. `LutufiError::Message(String)` is wrong.

**Err3. [STRONG] Warnings are distinct from errors.**  
Some conditions are not errors but should be surfaced to the user: LBP convergence residual above threshold, network size approaching the memory budget, treewidth estimate above the exact inference threshold. These are `LutufiWarning` values, not exceptions. They are attached to results and visible in `result.metadata.warnings`. The computation proceeds; the user is informed.

**Err4. [STRONG] Instrument all major operations with structured logging.**  
Use the `tracing` crate for structured, level-filtered logging throughout the Rust core. Every inference run emits a `DEBUG` span with: algorithm selected, network size, estimated treewidth, time taken, memory used. Every fallback decision emits a `INFO` event. Every numerical instability emits a `WARN` event. Users enable this with `RUST_LOG=lutufi=debug`. This makes debugging user-reported issues tractable.

**Err5. [GUIDE] Surface the full error chain, not just the leaf error.**  
When an error propagates through multiple layers — CPT construction fails inside BayesianNetwork construction inside inference — the user should see the full chain. Use `thiserror` with `#[source]` to attach cause chains. A user who sees "Inference failed → CPT validation failed for variable 'Income' → values sum to 0.97, expected 1.0" can fix their problem. A user who sees "Inference failed" cannot.

---

## Section 9 — Documentation Standards

**D1. [HARD] Every public symbol is documented.**  
Every `pub fn`, `pub struct`, `pub enum`, `pub trait` has a documentation comment. No exceptions. This is enforced by `#![deny(missing_docs)]` in `lib.rs`. A CI failure for missing docs is a failing build.

**D2. [HARD] Every doc comment includes an example.**  
Every public function's documentation includes at least one example in a `# Examples` section. The example must be correct Rust or Python (depending on layer), must be executable via `cargo test --doc`, and must demonstrate the primary use case. Examples that are wrong or that do not compile are documentation bugs.

**D3. [STRONG] Mathematical operations reference their source.**  
Every implementation of a mathematical algorithm references the paper or textbook it implements. `/// Implements the Bayes Ball algorithm for d-separation [Shachter 1998].` This is both attribution and navigation — a future contributor can find the original algorithm to verify the implementation.

**D4. [STRONG] Limitations are documented prominently.**  
Every module that implements an approximate algorithm has a `# Limitations` section in its module documentation stating: under what conditions the algorithm is inaccurate, what the user should watch for, and what alternative to use when the limitations apply. The `KNOWN_LIMITATIONS.md` file in the repository root is the canonical collection.

**D5. [GUIDE] The CHANGELOG is written for users, not developers.**  
Every entry in `CHANGELOG.md` describes the user impact of the change, not the implementation detail. "Fixed buffer overflow in CPT parser" is a developer note. "Fixed a bug where loading a BIF file with more than 32 parent states would produce incorrect CPT values" is a user note. Write for the researcher who is deciding whether to upgrade.

---

## Section 10 — Dependency & Security

**Dep1. [HARD] All dependencies must be compatible with Apache 2.0.**  
Lutufi is Apache 2.0 licensed. All dependencies must have licenses that are compatible with Apache 2.0. GPL and LGPL dependencies are forbidden — they would infect Lutufi's license and prevent commercial use. Check every new dependency with `cargo-deny` before adding it. The `deny.toml` configuration file specifies allowed licenses and blocks incompatible ones at CI time.

**Dep2. [HARD] Security vulnerabilities in dependencies are treated as Lutufi bugs.**  
Run `cargo audit` on every CI build. Any known vulnerability in a direct or transitive dependency blocks the build until the dependency is updated or the vulnerability is explicitly acknowledged with a documented justification. A library with a known unpatched security vulnerability is not safe for use in financial crime or intelligence applications.

**Dep3. [STRONG] Minimize the dependency tree.**  
Every dependency is a maintenance burden, a security surface, and a potential license complication. Before adding a new dependency, ask: can this be implemented in fewer than 200 lines of Rust? If yes, implement it. If the dependency provides a substantial, well-maintained implementation of a non-trivial algorithm (e.g., `ndarray`, `rayon`, `pyo3`), add it. If it provides a single utility function, do not.

**Dep4. [STRONG] Pin dependency versions in `Cargo.lock` and review updates deliberately.**  
Do not use `cargo update` carelessly. Dependency updates are reviewed, tested against the full test suite, and merged deliberately. A dependency update that causes a test failure is reverted, not worked around. The `Cargo.lock` file is committed to the repository.

**Dep5. [GUIDE] Prefer well-maintained dependencies with active communities.**  
A dependency that has not been updated in three years and has open security issues is a liability. Prefer dependencies that are part of the Rust ecosystem standard (e.g., `serde`, `rayon`, `ndarray`) over niche alternatives. Check the dependency's GitHub stars, recent commit activity, and open issue count before adopting it.

---

## Section 11 — Ethical & Social Responsibility

**Eth1. [HARD] The do-operator cannot be called on non-causal models.**  
This is repeated here because it is both a scientific guardrail (Section 4) and an ethical one. Causal claims made from associational models have caused real policy harm — social programs discontinued because an observational study was misread as causal evidence. Lutufi's technical enforcement of this boundary is a small but concrete contribution to research integrity.

**Eth2. [HARD] The library must not facilitate individual deanonymization.**  
Lutufi operates on aggregate network structures. It must not provide utilities that, given aggregate network statistics, reconstruct the identities of individuals. Any feature that could be used for deanonymization — structural uniqueness queries, re-identification attacks on anonymized graphs — is out of scope and must not be implemented. If a user's use case requires this, they have the wrong tool.

**Eth3. [STRONG] Bias propagation must be disclosed in learned models.**  
When a model is learned from data (`model.fit(data)` or `model.learn_structure(data)`), the result includes a `BiasWarning` in its metadata: "This model was learned from observational data. If the data reflects historical biases (e.g., discriminatory lending decisions, unequal surveillance), those biases are encoded in the model's parameters and will propagate to inference results. Validate the model's outputs against known ground truth before using it in decision-making." This warning is not suppressible.

**Eth4. [STRONG] Missing edges in dark networks are never treated as confirmed absences.**  
In covert network analysis, an unobserved edge is not evidence that the edge does not exist — it is evidence that observation was incomplete. The `DarkNetworkModel` type (an extension of `BayesianNetwork` for adversarial contexts) treats unobserved edges as having a prior probability of existence based on the network's structural model, not as confirmed zeros. Any analysis that assumes unobserved edges are absent must explicitly mark this assumption with `assume_closed_world=True` and receive a warning.

**Eth5. [STRONG] Explainability paths must be preserved for policy-relevant outputs.**  
For any inference result that might inform a policy decision — financial sanctions, network disruption, resource allocation — Lutufi must be able to produce an explanation: which variables most influenced the result, which evidence mattered most, what the counterfactual would have been under different assumptions. This is not a visualization feature. It is a accountability infrastructure requirement.

**Eth6. [GUIDE] Export control awareness.**  
Lutufi's capabilities — dark network analysis, financial crime detection, covert network reconstruction — may be subject to export control regulations in certain jurisdictions when deployed in specific operational contexts. The `EXPORT_CONTROL.md` document describes these considerations. The library itself is open-source software and is not a controlled item, but users building operational systems on Lutufi in sensitive domains should review this document and consult legal counsel as appropriate.

---

## Section 12 — API Versioning & Backward Compatibility

**V1. [HARD] Semantic versioning is followed strictly.**  
`MAJOR.MINOR.PATCH`. A PATCH release contains only bug fixes — no new public API surface, no behavior changes. A MINOR release may add new public API surface but may not change or remove existing API. A MAJOR release may change or remove existing API. There are no exceptions. A bug fix that requires changing a function signature is a MINOR or MAJOR release, not a PATCH.

**V2. [HARD] Public API items are deprecated before removal.**  
No public function, class, or parameter is removed without first being deprecated for at least one MINOR release. Deprecated items emit a `DeprecationWarning` in Python and a compiler warning in Rust. The deprecation notice states what to use instead and which version will remove the deprecated item.

**V3. [STRONG] The serialization format is versioned independently of the library.**  
The Lutufi Model Format has its own version number, separate from the library version. A model file saved with Lutufi 1.0 must be loadable by Lutufi 2.0. The migration framework in the serialization module handles format version upgrades automatically. There is no acceptable scenario where a user's saved models become unreadable because they updated the library.

**V4. [STRONG] Breaking changes require a migration guide.**  
Every MAJOR release includes a `MIGRATION.md` document that explains every breaking change, why it was made, and exactly what the user must change in their code. The migration guide is written before the breaking change is merged, not after.

**V5. [GUIDE] Experimental features are explicitly marked.**  
Features that are not yet stable — whose API may change without a major version bump — are marked with `#[cfg(feature = "experimental")]` in Rust and accessible via `lutufi.experimental.*` in Python. Users who use experimental features accept that they may break. This allows rapid iteration on new capabilities without the full deprecation overhead.

---

## Section 13 — Concurrency & Thread Safety

**C1. [HARD] The Rust core is `Send + Sync` throughout.**  
All types that cross thread boundaries implement `Send` (can be sent to another thread) and `Sync` (can be shared between threads). This is enforced by the compiler for types that use `Arc<>` and `Mutex<>`. Any type that cannot be safely shared across threads must be explicitly marked `!Send` or `!Sync` with a documented reason.

**C2. [HARD] Shared mutable state uses `Arc<RwLock<>>`, not `Arc<Mutex<>>`.**  
The junction tree is read by many concurrent inference queries and written only during compilation. Use `RwLock` which allows multiple concurrent readers or one exclusive writer, not `Mutex` which serializes all access. Lock contention on the junction tree would eliminate the parallelism gains of concurrent inference.

**C3. [STRONG] Parallel algorithms produce deterministic results.**  
Floating point addition is not associative — summing values in different orders produces slightly different results. Parallel algorithms that sum over factors or messages must use a deterministic reduction order. The result of a parallel inference run must be bit-identical to the single-threaded result on the same input, or the difference must be bounded and documented.

**C4. [STRONG] No blocking operations in async contexts.**  
When the async inference API is used (`model.query_async()`), no inference code may perform blocking I/O or blocking synchronization (no `Mutex::lock()` in async contexts — use `tokio::sync::RwLock` instead). Blocking in an async context starves the executor and negates the benefits of async.

**C5. [GUIDE] Thread pool size is configurable and documented.**  
The default thread pool size for parallel inference is `min(available_cpus, 8)`. This is configurable via `ResourceBudget.n_threads`. The documentation explains the tradeoff: more threads are faster for large networks but increase memory pressure (each thread needs message buffers). Users running Lutufi on shared computing infrastructure should set this explicitly.

---

## Summary: The Decision Rules

When you are about to write code and you are uncertain, run through this checklist in order:

1. **Is there a HARD guardrail that applies?** If yes, follow it. No exceptions.
2. **Does this produce a numerical result?** Is it in log-space? Does it use log-sum-exp? Is the output validated?
3. **Does this cross the user-facing boundary?** Validate inputs. Produce a structured error with context and suggestion on failure.
4. **Is this an inference operation?** Has the model been validated? Is algorithm selection automatic? Is the result accompanied by metadata and uncertainty?
5. **Is this a causal operation?** Has `mark_as_causal()` been called? Has identifiability been checked?
6. **Does this involve learned parameters?** Is the bias warning attached? Is the convergence status reported?
7. **Does this create shared state?** Is it behind an `Arc<RwLock<>>`? Is it `Send + Sync`?
8. **Does this add a new public API symbol?** Does it have a doc comment? Does it have an example? Does it have a test?
9. **Does this change an existing public API symbol?** Is the old version deprecated first? Is there a migration guide?
10. **Have you written the test before the implementation?** Go write the test.

---

*These guardrails are a living document. When a rule proves insufficient, update the rule. When a new class of problems is discovered, add a rule. The document grows with the library's maturity. Every addition requires a rationale. Every rule that cannot be justified by a concrete failure mode it prevents should be removed.*

*Last principle: guardrails exist to protect the researchers who trust Lutufi with their work. When in doubt, ask what a careful researcher would need to trust a result. Build that.*
