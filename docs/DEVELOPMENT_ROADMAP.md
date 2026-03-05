# Lutufi — Full Development Roadmap

### From First Commit to Community-Adopted Research Library

**Document Version:** 1.0
**Status:** Active Planning Document
**Author:** Wasswa Lutufi Sebbanja
**Last Updated:** March 2026

---

> *"Trust in Allah, then tie your camel."*
> This roadmap is the camel. Every sprint is a knot in the rope.

---

## How to Read This Document

This roadmap is organized into **Phases** and **Sprints**. Each phase has a single overarching objective — a state of the world that did not exist before it. Each sprint is two weeks long, has a clear goal, a concrete list of tasks, and a definition of done that is either true or false with no ambiguity.

**Sprint conventions:**

- Every sprint ends with a working, tested, committed state. No sprint ends with broken code.
- Every sprint has a mandatory review day on day 14. You read what you built, write notes on what surprised you, and update the open questions log before the next sprint begins.
- The definition of done is not "I wrote the code." It is "the tests pass, the code is committed, and I can explain every line of it to a skeptical senior engineer."

---

## Phase Overview

| Phase | Name                         | Duration            | End State                                                   |
| ----- | ---------------------------- | ------------------- | ----------------------------------------------------------- |
| 0     | Pre-Code Finalization        | 2 sprints (4 weeks) | All blocking decisions resolved, environment ready          |
| 1     | Core Representation          | 4 sprints (8 weeks) | You can build and query a probabilistic network             |
| 2     | Exact Inference              | 3 sprints (6 weeks) | Variable elimination and junction tree working and verified |
| 3     | Approximate Inference        | 4 sprints (8 weeks) | LBP, MCMC, and variational inference working                |
| 4     | Learning from Data           | 3 sprints (6 weeks) | Parameter and structure learning from real data             |
| 5     | Causal Inference             | 3 sprints (6 weeks) | Do-calculus, interventions, counterfactuals                 |
| 6     | Dynamic Networks             | 3 sprints (6 weeks) | DBNs, HMMs, temporal inference                              |
| 7     | Missing Data                 | 2 sprints (4 weeks) | Principled handling of incomplete networks                  |
| 8     | Python Bindings              | 3 sprints (6 weeks) | Clean Pythonic API over the core                            |
| 9     | Example Library              | 4 sprints (8 weeks) | 30+ domain-specific examples                                |
| 10    | Scalability                  | 3 sprints (6 weeks) | 1M+ edge networks, benchmarks published                     |
| 11    | Serialization                | 2 sprints (4 weeks) | Full reproducibility format                                 |
| 12    | Documentation                | 3 sprints (6 weeks) | Publication-quality docs                                    |
| 13    | Hardening                    | 2 sprints (4 weeks) | Security, edge cases, v1.0 release                          |
| 14    | Community and Publication    | 4 sprints (8 weeks) | JOSS paper, active community                                |
| 15    | Simulation and Visualization | 4 sprints (8 weeks) | v2.0 capabilities                                           |

**Total: ~54 sprints (~108 weeks / ~27 months)**

---

## Phase 0 — Pre-Code Finalization

### Duration: 2 Sprints (4 Weeks)

### Objective: Resolve every blocking decision, set up the environment, write the first commit.

Before a single line of library code exists, three things must be true: you know exactly what language the core is written in and why, your development environment is fully configured, and your repository structure is established. These decisions cannot be changed after coding begins without catastrophic cost.

---

### Sprint 0.1 — Language Decision and Environment Setup

**Goal:** Make the core language decision with full written justification and have a working development environment.

**Tasks:**

1. **Resolve the Rust vs C++ decision.** Write a three-page technical decision document covering: memory safety implications for each, FFI complexity for Python bindings (PyO3 for Rust), ecosystem maturity for sparse matrix operations and linear algebra, build system complexity (Cargo), community support, and your own current competence level in each. Document it in `docs/design/LANGUAGE_DECISION.md`. This decision will not be revisited.
2. **Install and configure the development environment.** Your environment must include: the chosen systems language toolchain (rustup + cargo OR clang + cmake), Python 3.11+ with virtualenv, pytest, numpy, scipy, networkx, pandas, pre-commit hooks, black/ruff for Python formatting, clippy/rustfmt OR clang-format for systems language, git with signed commits configured.
3. **Establish the repository structure.** Create the following directory layout and commit it:

```
lutufi/
  src/                    # Rust core
    core/
      models/
      representation/
      inference/
      learning/
      io/
    ffi/                  # PyO3 FFI layer
  python/                 # Installable Python package
    lutufi/
      __init__.py
      models.py
      inference.py
      learning.py
      io.py
  tests/                  # Rust tests (cargo test)
    unit/
    integration/
    ground_truth/         # Analytical solutions, Rust
  benches/                # Rust benchmarks (cargo bench)
  examples/               # Python examples via bindings
    epidemiology/
    finance/
    social/
    intelligence/
    validation/           # Examples that assert correctness
  bindings/               # Future R, Julia
  docs/
  Cargo.toml
  pyproject.toml
```

4. **Write the project README.** One paragraph on what Lutufi is, one on what problem it solves, one on current status (pre-alpha, do not use in production), installation instructions (pip install once published), and a minimal example that will eventually run when the library is complete. Commit this as the aspirational target.
5. **Set up CI/CD.** Configure GitHub Actions (or equivalent) with: build pipeline for the systems language, Python test runner, code coverage reporting, and linting checks. Every push should trigger these automatically. A failing pipeline should block merge.

**Definition of Done:**

- `docs/design/LANGUAGE_DECISION.md` exists with a clear, justified decision.
- Running `cargo build` (or `cmake --build`) produces no errors.
- Running `pytest tests/` produces "no tests collected" (not an error).
- CI pipeline runs and passes on every push.
- Repository structure matches the layout above.

---

### Sprint 0.2 — Ground Truth Infrastructure and Contribution Setup

**Goal:** Build the testing infrastructure that will validate mathematical correctness throughout the entire project.

**Tasks:**

1. **Build the analytical ground truth library.** This is the most important testing infrastructure in the project. Collect and implement analytical solutions for the following known networks and queries. Every inference result Lutufi ever produces for these cases must match these solutions exactly within floating point precision:

   - **Asia Network** (Lauritzen & Spiegelhalter 1988): 8-node medical diagnosis network with known exact marginals under multiple evidence configurations.
   - **Alarm Network**: 37-node anesthesia monitoring network. Known marginals, available from the bnlearn repository.
   - **Simple chain A → B → C**: Exact marginals calculable by hand for any CPT values.
   - **V-structure (collider)**: A → C ← B. Verify that A and B are independent marginally but dependent given C.
   - **Fork**: A ← B → C. Verify that A and C are dependent marginally but independent given B.
   - **Simple MRF on a grid**: 3×3 grid with known partition function (computable by brute force for small grids).

   Store these in `tests/ground_truth/` as JSON files with the network structure, CPTs, and known correct answers for specific queries.
2. **Write the ground truth test runner.** A pytest fixture that loads each ground truth case, runs inference with Lutufi (once implemented), and asserts the results match within `1e-10` tolerance. This test will fail until inference is implemented but it should be written now so that the target is always visible.
3. **Document the contribution guidelines.** Write `CONTRIBUTING.md` covering: code style requirements, how to run tests, how to add a new ground truth case, how to add a new example, the PR review process, and the commit message format.
4. **Write the first passing test.** Write a test for something that is currently implementable: that the repository structure is correct, that the Python package imports without error, that the version string matches `0.1.0-dev`. Make it pass. This is your first green test. It matters psychologically.
5. **Finalize the remaining two open architectural issues.** Write one page each on: (a) the causal vs statistical graph distinction and how the API enforces it, and (b) the incremental inference architecture for dynamic networks. Add these to `docs/design/`.

**Definition of Done:**

- `tests/ground_truth/` contains JSON files for all six cases above.
- `tests/test_ground_truth.py` exists and runs (failing gracefully with "NotImplemented" until inference exists).
- `CONTRIBUTING.md` exists and is complete.
- `pytest tests/` runs and shows at least one passing test.
- Both architectural gap documents exist in `docs/design/`.

---

## Phase 1 — Core Representation

### Duration: 4 Sprints (8 Weeks)

### Objective: A fully correct, tested data model that can represent any probabilistic graphical model Lutufi will ever need to support.

This phase produces no inference. It produces only the data structures. By the end, you can build a Bayesian network, a Markov random field, and a factor graph in code, inspect their structure, add nodes and edges, set probability tables, and serialize them to memory. You cannot yet run inference. That is correct. Get the representation exactly right before building anything on top of it.

---

### Sprint 1.1 — Variable and Domain Representation

**Goal:** A correct, type-safe representation of random variables and their domains.

**Tasks:**

1. Implement the `Variable` type in the core language. A variable has: a unique identifier (UUID), a human-readable name (string), and a domain. Domains are: `Discrete` (a list of named states, e.g., `["low", "medium", "high"]`), `Continuous` (a real-valued range with optional bounds), or `Binary` (a special case of Discrete for performance).
2. Implement the `Domain` type with operations: size (number of states for discrete), validation (is this value in the domain?), and iteration (for discrete domains).
3. Implement an `Assignment` type — a mapping from variable IDs to their current values. Assignments are used throughout inference to represent evidence and to index into probability tables. Must support: setting a value, getting a value, checking if a variable is assigned, and iterating over assigned variables.
4. Write unit tests for every operation on Variable, Domain, and Assignment. Test boundary cases: empty domains, single-state domains, assigning values not in domain (should error), assigning the same variable twice (should overwrite), iterating over an empty assignment.
5. Implement Python wrappers for Variable and Domain. A Python user should be able to write `v = Variable("income", domain=["low", "medium", "high"])` and inspect `v.name`, `v.domain`, `v.domain.size`.

**Definition of Done:** All unit tests pass. Python wrapping works. Variable, Domain, Assignment are documented with docstrings.

---

### Sprint 1.2 — Factor Representation

**Goal:** A correct, memory-efficient representation of factor functions over sets of variables.

**Tasks:**

1. Implement the `Factor` trait/interface with required operations: `scope()` (which variables this factor covers), `evaluate(assignment)` (the factor's value for a given assignment), `marginalize(variables)` (sum out specified variables), `multiply(other_factor)` (product of two factors), and `normalize()` (normalize so values sum to 1).
2. Implement `TabularFactor` — a factor backed by an explicit table of values. The table is stored as a multi-dimensional array indexed by variable states. Use sparse storage when density is below 30%. Implement all Factor operations.
3. Implement `ConditionalProbabilityTable (CPT)` as a specialization of TabularFactor. A CPT has a distinguished child variable and parent variables. It enforces that the values for each parent configuration sum to 1. Add a `validate_cpt()` method that checks this invariant and raises an informative error if violated.
4. Implement `PotentialFunction` for MRFs — an unnormalized factor with no conditional structure requirement.
5. Write unit tests verifying: factor evaluation on known values, marginalization against hand-computed results, factor product against hand-computed results, CPT normalization validation (both passing and failing cases), and sparse vs dense storage selection.

**Definition of Done:** All unit tests pass. Marginalization and product results match hand-computed ground truth within `1e-10`.

---

### Sprint 1.3 — Graph Structure and Graphical Models

**Goal:** The three user-facing model types — BayesianNetwork, MarkovRandomField, DynamicBayesianNetwork — as correct structural containers.

**Tasks:**

1. Implement the core graph structure: an adjacency representation supporting directed and undirected edges, cycle detection for DAG validation, topological sort, parent/child/neighbor queries, and Markov blanket computation.
2. Implement `BayesianNetwork`: a DAG where each node is a Variable and each node has an associated CPT. Must enforce: acyclicity (raise `LutufiCyclicGraphError` with the cycle listed if violated), that every node has a CPT before inference, that CPT parent sets match the graph parents. Implement `add_node()`, `add_edge()`, `set_cpd()`, `remove_node()`, `remove_edge()`, `markov_blanket()`, `topological_order()`.
3. Implement `MarkovRandomField`: an undirected graph where factors are attached to cliques. Implement `add_node()`, `add_edge()`, `add_factor()`, `cliques()`, `markov_blanket()`.
4. Implement `DynamicBayesianNetwork`: a two-slice temporal model. Stores a prior network (time 0) and a transition network (time t to t+1). Variables exist in time slices and are referred to as `variable_name_t` and `variable_name_t1`. Implement `add_node()`, `add_intraslice_edge()`, `add_interslice_edge()`, `unroll(T)` (produces a static BN unrolled over T time steps).
5. Implement `FactorGraph` as the canonical internal representation. Implement `from_bayesian_network()`, `from_markov_random_field()`, and `to_factor_graph()` conversion methods on BN and MRF.
6. Write the `from_networkx()` import for BayesianNetwork and the `to_networkx()` export. Verify round-trip: `BN.to_networkx()` followed by `BN.from_networkx()` produces an equal network.
7. Write unit tests for all structural operations on all three model types. The cycle detection test must cover: single-node self-loop, two-node cycle, multi-node cycle, and deeply nested cycle. The CPT validation test must cover: correct CPT, CPT that doesn't sum to 1, CPT with wrong number of parent states.

**Definition of Done:** All unit tests pass. `LutufiCyclicGraphError` fires correctly on cyclic input. Round-trip NetworkX conversion is exact.

---

### Sprint 1.4 — Model Construction API and Python Bindings

**Goal:** The fluent Python API for building models, with full error messages.

**Tasks:**

1. Implement the builder pattern for `BayesianNetwork` in Python:

```python
model = (BayesianNetwork.builder()
    .add_variable("A", domain=["0", "1"])
    .add_variable("B", domain=["0", "1"])
    .add_edge("A", "B")
    .set_cpd("A", [0.3, 0.7])
    .set_cpd("B", [[0.9, 0.1], [0.2, 0.8]])
    .build())
```

2. Implement the error message library. For every error Lutufi can raise, write the exact message text. Messages must: name the specific error, explain what the user did wrong, and suggest what to do instead. Minimum 15 errors covering: cyclic graph, CPT doesn't normalize, variable not found, variable already exists, edge to non-existent node, calling inference before setting all CPTs, calling do-operator on a non-causal model, domain mismatch in evidence.
3. Implement pandas import: `BayesianNetwork.from_dataframe(df)` where df has columns `variable`, `parents`, `states`.
4. Implement model inspection methods in Python: `model.nodes()`, `model.edges()`, `model.cpd("A")`, `model.markov_blanket("A")`, `model.is_valid()` (returns True/False + list of validation errors).
5. Implement `__repr__` and `__str__` for all model types — a printed model should show its structure clearly.
6. Run all ground truth tests. They should still fail on inference calls but pass on construction calls (building the Asia and Alarm networks should now work).

**Definition of Done:** All unit tests pass. The Asia network can be fully constructed in Python with correct CPTs. All error messages are informative and tested.

---

## Phase 2 — Exact Inference

### Duration: 3 Sprints (6 Weeks)

### Objective: Variable elimination and the junction tree algorithm, verified against analytical ground truth.

This is the first phase where mathematical correctness becomes the primary concern. Every result produced by Phase 2's inference engine must be numerically verifiable. The ground truth infrastructure from Sprint 0.2 now earns its keep.

---

### Sprint 2.1 — Variable Elimination

**Goal:** A correct implementation of variable elimination with multiple elimination order heuristics.

**Tasks:**

1. Implement variable elimination (also called bucket elimination). The algorithm: given a query for variable Q given evidence E, eliminate all non-query, non-evidence variables one by one by summing them out of the factor product. Returns the exact marginal P(Q | E).
2. Implement three elimination order heuristics: min-fill (choose the variable whose elimination adds the fewest fill-in edges), min-degree (choose the variable with the fewest neighbors), and user-specified order (allow the user to pass an explicit order).
3. Implement all arithmetic in **log-space**. Every multiplication of factors is performed as addition of log-factors. Every summation uses the log-sum-exp trick. No probability value should ever underflow to zero due to floating point. Write a dedicated test that forces underflow in naive arithmetic and verifies log-space arithmetic handles it correctly.
4. Implement the `query()` API call:

```python
result = engine.query(
    variables=["Disease"],
    evidence={"Symptom": "fever"},
    algorithm="variable_elimination",
    elimination_order=None  # auto-select
)
```

5. Run the ground truth tests. Variable elimination on the Asia network must match known exact marginals within `1e-10`. If any test fails, fix it before proceeding.

**Definition of Done:** All six ground truth cases pass with variable elimination. Log-space underflow test passes.

---

### Sprint 2.2 — Junction Tree Algorithm

**Goal:** A correct implementation of the junction tree algorithm, faster than variable elimination for repeated queries on the same network.

**Tasks:**

1. Implement moralization: convert a BN to its moral graph (marry parents, remove directions).
2. Implement triangulation: add fill-in edges to make the moral graph chordal (no cycle of length 4+ without a chord). Implement the minimum fill-in triangulation heuristic.
3. Implement clique tree construction: find all maximal cliques of the triangulated graph, build a spanning tree where adjacent cliques share their separator, verify the running intersection property holds for all constructed junction trees.
4. Implement the two-pass message passing schedule: upward pass (leaves to root, collect evidence), downward pass (root to leaves, distribute evidence). After both passes, all clique beliefs and separator beliefs are correct marginals.
5. Implement the `JunctionTreeEngine` class that compiles a model into a junction tree once, then answers arbitrary marginal queries in O(clique_size) time after the initial compilation. The compilation is the expensive step; subsequent queries are fast.
6. Implement treewidth estimation. Before running exact inference, estimate the treewidth of the model. If treewidth exceeds a configurable threshold (default 15), raise `LutufiHighTreewidthWarning` advising the user to consider approximate inference.
7. Run all ground truth tests with the junction tree engine. Results must match variable elimination results within `1e-9`.

**Definition of Done:** Ground truth tests pass for both engines. Results match between engines. Treewidth warning fires correctly.

---

### Sprint 2.3 — D-Separation, MAP Queries, and MPE

**Goal:** Complete the exact inference API with structural independence testing and maximum probability queries.

**Tasks:**

1. Implement d-separation testing using the Bayes Ball algorithm (more efficient than path-based algorithms for large networks). API: `model.d_separated("A", "B", given=["C", "D"])`. Returns True/False.
2. Write d-separation tests for all three structural patterns: chain (blocked by observing middle), fork (blocked by observing common cause), collider (opened by observing collider or descendant). Each test must verify both the blocked and open cases.
3. Implement MAP inference (Maximum A Posteriori): find the single most probable state for query variables given evidence. Unlike marginal inference, MAP maximizes rather than marginalizes. Implement using the max-product variant of variable elimination.
4. Implement MPE (Most Probable Explanation): find the most probable assignment to all unobserved variables given evidence. This is a special case of MAP where all variables are query variables.
5. Implement the `QueryResult` class with full output format support: `result["Disease"]` returns the marginal distribution, `result.to_dataframe()` returns a pandas DataFrame, `result.to_dict()` returns a plain Python dict, `result.most_probable()` returns the MAP state.
6. Write integration tests that combine model construction, evidence setting, d-separation testing, marginal inference, and MAP inference in a single workflow. Use the Asia network.

**Definition of Done:** D-separation tests pass for all structural patterns. MAP and MPE produce correct results on ground truth cases. QueryResult conversions all work.

---

## Phase 3 — Approximate Inference

### Duration: 4 Sprints (8 Weeks)

### Objective: Loopy belief propagation, MCMC, and variational inference — all with convergence diagnostics and honest accuracy characterization.

Approximate inference is where the theoretical honesty of your design becomes visible in code. Every approximate method must report not just a result but the quality of that result — convergence status, number of iterations, and where applicable a bound on accuracy.

---

### Sprint 3.1 — Loopy Belief Propagation

**Goal:** A correct LBP implementation with convergence monitoring and damping.

**Tasks:**

1. Implement the sum-product message passing algorithm on factor graphs. Messages pass from variable nodes to factor nodes and from factor nodes to variable nodes. Implement in log-space.
2. Implement the asynchronous message update schedule (update messages in sequence rather than all at once). Async schedules typically converge faster than synchronous on social network structures.
3. Implement damping: new message = (1 - damping) × new_message + damping × old_message. Damping slows convergence but stabilizes it. Implement configurable damping factor with default 0.5.
4. Implement the `ConvergenceMonitor`: tracks the maximum message change across all edges at each iteration. Stops when max change < tolerance OR iterations > max_iterations. Reports `converged=True/False`, `iterations`, and the final convergence residual.
5. Implement `LBPEngine` with the query API. If the algorithm does not converge, return the current beliefs with a `LutufiConvergenceWarning` that states the residual and suggests increasing max_iterations or adjusting damping. Never raise an exception for non-convergence — return the best available approximation with a clear warning.
6. Test LBP on tree-structured networks. On trees, LBP must give exact results (identical to junction tree) within `1e-10`. This is the mathematical correctness test for LBP.
7. Test LBP on the Asia network with cycles (the Asia network has no cycles, so use a modified cyclic version). Verify that results are close to junction tree results (within 5% relative error on marginals) and that the convergence monitor reports correctly.

**Definition of Done:** LBP gives exact results on trees. Convergence monitor fires correctly. Damping reduces oscillation on a known non-converging case.

---

### Sprint 3.2 — Gibbs Sampling and Metropolis-Hastings

**Goal:** Two MCMC implementations with full diagnostics.

**Tasks:**

1. Implement Gibbs sampling: at each step, sample each unobserved variable from its full conditional distribution P(Xi | all other variables, evidence). The full conditional is proportional to the product of factors involving Xi. Implement using the factor graph structure to identify which factors involve each variable.
2. Implement the burn-in mechanism: discard the first N samples (configurable, default 1000) before collecting statistics. Implement thinning: collect only every Kth sample (configurable, default 1) to reduce autocorrelation.
3. Implement Metropolis-Hastings with a configurable proposal distribution. Default proposal: Gaussian perturbation for continuous variables, random state flip for discrete variables. Implement the acceptance ratio and the Metropolis accept/reject step.
4. Implement MCMC diagnostics:

   - **Gelman-Rubin statistic (R-hat)**: run multiple chains in parallel, compute the ratio of between-chain to within-chain variance. R-hat < 1.1 indicates good convergence.
   - **Effective Sample Size (ESS)**: measures how many independent samples your correlated MCMC chain is equivalent to. ESS should be at least 100 for reliable estimates.
   - **Autocorrelation function**: plot/report the correlation between samples at different lags.
5. Implement `MCMCEngine` with the standard query API plus additional parameters: `n_samples`, `burn_in`, `chains`, `thin`. Results include mean, variance, and credible intervals derived from samples.
6. Test both samplers on the Asia network. Run 50,000 samples after 5,000 burn-in. Marginals must be within 2% of exact results. Gelman-Rubin statistic must be < 1.1 for all variables.

**Definition of Done:** Gibbs and MH samplers match exact inference within 2% on Asia network with 50K samples. All diagnostics compute correctly.

---

### Sprint 3.3 — Mean Field Variational Inference

**Goal:** Mean field VI with ELBO monitoring and convergence diagnostics.

**Tasks:**

1. Implement the mean field approximation: approximate the true posterior P(X|E) with a fully factorized distribution Q(X) = ∏ Q_i(X_i). The optimal Q_i for each variable is proportional to exp(E_Q[-i][log f(X)]) where the expectation is over all other variables under Q.
2. Implement coordinate ascent variational inference (CAVI): iteratively update each Q_i while holding all others fixed. Repeat until the ELBO converges.
3. Implement the Evidence Lower Bound (ELBO): ELBO = E_Q[log P(X,E)] - E_Q[log Q(X)]. The ELBO is a lower bound on log P(E). Higher ELBO = better approximation. Monitor ELBO at each iteration. The ELBO must increase monotonically; if it decreases, there is a bug.
4. Implement multiple random restarts: run CAVI from K different random initializations (default K=5), return the result with the highest ELBO. This mitigates local optima.
5. Implement `VariationalEngine` with query API plus: `n_restarts`, `max_iterations`, `tolerance`. Results include the approximate marginals, the final ELBO value, whether ELBO converged, and the ELBO value at each iteration (for diagnostic plotting).
6. Test on Asia network. Variational marginals must be within 10% of exact results (mean field is less accurate than LBP in general). ELBO must be monotonically increasing. Multiple restarts must improve results compared to single restart.

**Definition of Done:** ELBO is monotonically increasing in all tests. Variational marginals are within 10% of exact results. Multiple restarts are demonstrably better than single restart on a test case.

---

### Sprint 3.4 — Automatic Algorithm Selection and Inference API Unification

**Goal:** A unified inference API that selects the right algorithm automatically.

**Tasks:**

1. Implement the `InferenceEngine` factory class. Given a model and a query, it analyzes: estimated treewidth, model size, query type (marginal/MAP/MPE), and user preferences. It selects: Junction Tree for treewidth ≤ 15, LBP for sparse networks with treewidth > 15, Variational for dense networks, MCMC for complex posteriors or user-specified preference.
2. Implement the unified `model.query()` interface:

```python
result = model.query(
    variables=["A", "B"],
    evidence={"C": "high"},
    algorithm="auto"  # or "exact", "lbp", "mcmc", "variational"
)
```

3. Implement the `InferenceResult` class hierarchy. Base class has: `variables`, `distributions`, `algorithm_used`, `computation_time`. Subclasses add algorithm-specific diagnostics: `JunctionTreeResult` (treewidth), `LBPResult` (converged, iterations, residual), `MCMCResult` (n_samples, ess, r_hat), `VariationalResult` (elbo, converged).
4. Write the full inference integration test suite. Test all four algorithms on: Asia network, a 50-node sparse synthetic network, a 200-node dense synthetic network, a network with continuous variables. Compare results across algorithms.
5. Write the performance benchmarking baseline. Time each algorithm on each test network. Record results in `tests/benchmarks/baseline_YYYY-MM-DD.json`. This is the baseline you will compare against in Phase 10.

**Definition of Done:** Auto-selection produces correct algorithm choices on all test cases. All four algorithms run through the unified query API. Benchmark baseline file exists.

---

## Phase 4 — Learning from Data

### Duration: 3 Sprints (6 Weeks)

### Objective: Parameter learning and structure learning from real observational data.

---

### Sprint 4.1 — Parameter Learning

**Goal:** MLE and Bayesian parameter estimation from complete and incomplete data.

**Tasks:**

1. Implement Maximum Likelihood Estimation for discrete CPTs from complete data. The MLE estimator is empirical frequency: count joint occurrences and normalize. Handle zero counts with Laplace smoothing (add pseudocount α, default 0.5).
2. Implement Bayesian parameter estimation with Dirichlet priors. The BDeu (Bayesian Dirichlet equivalent uniform) prior uses hyperparameter α/|domain| for each state, where α is the equivalent sample size (default 1.0). The posterior predictive distribution is the empirical counts plus prior pseudocounts, normalized.
3. Implement the EM algorithm for networks with latent (never-observed) variables and for datasets with missing values. E-step: run inference to compute expected sufficient statistics for each latent variable. M-step: update CPTs using expected counts. Monitor the log-likelihood at each iteration; it must increase monotonically (if it decreases, there is a bug in the implementation).
4. Implement `model.fit(data)` where data is a pandas DataFrame with one column per variable. If a column is absent, that variable is treated as latent. If a cell is NaN, that observation is treated as missing. Returns the fitted model with updated CPTs.
5. Write learning tests: fit the Asia network structure to a synthetic dataset of 10,000 samples generated from the known CPTs. Verify that the learned CPTs are within 5% of the generating CPTs for all parameters. This tests that learning converges to the correct answer with sufficient data.

**Definition of Done:** Learned CPTs on 10K synthetic data are within 5% of generating CPTs. EM log-likelihood is monotonically increasing. Zero-count Laplace smoothing prevents zero probabilities.

---

### Sprint 4.2 — Score-Based Structure Learning

**Goal:** Hill climbing structure search with BIC and BDeu scoring.

**Tasks:**

1. Implement local scoring: given a variable and a candidate parent set, compute the BIC and BDeu scores. Scoring must be decomposable (the score of the full graph is the sum of local scores) for efficient search.
2. Implement the hill climbing search: start from an empty graph (or a user-specified start), repeatedly apply the local modification (add edge, remove edge, reverse edge) that most improves the score, stopping when no modification improves the score. Implement random restarts (default 5) to escape local optima.
3. Implement the Greedy Equivalence Search (GES) algorithm. GES operates in two phases: forward (add edges that improve score) and backward (remove edges that improve score). GES is provably correct under faithfulness in the large-sample limit. This is the preferred algorithm for structure learning in Lutufi.
4. Implement the acyclicity constraint enforcement in structure search. Candidate edges that would create a cycle must be rejected without computing their score. Use incremental cycle detection rather than full topological sort.
5. Implement forbidden and required edge constraints: `model.learn_structure(data, forbidden=[("A","B")], required=[("C","D")])`. The search never adds forbidden edges and always includes required edges.
6. Test on the Asia network: generate 10,000 samples, run GES, verify that the learned structure is in the same Markov equivalence class as the true structure (since observational data cannot distinguish Markov-equivalent structures).

**Definition of Done:** GES recovers the correct Markov equivalence class for Asia network with 10K samples with probability > 0.8 over 10 runs.

---

### Sprint 4.3 — Constraint-Based Structure Learning

**Goal:** PC and FCI algorithms for causal discovery.

**Tasks:**

1. Implement the conditional independence tests needed by constraint-based methods: chi-square test for discrete variables (with sample size correction), G-test (likelihood ratio), Fisher's Z-test for continuous (Gaussian) variables.
2. Implement the PC algorithm: start with complete undirected graph, apply independence tests to remove edges, orient remaining edges using Meek's orientation rules and v-structure orientation.
3. Implement the FCI algorithm: an extension of PC that handles latent common causes. FCI produces a Partial Ancestral Graph (PAG) with three edge types: directed (→), undirected (—), and circle (○). FCI is the correct algorithm when the researcher suspects unmeasured confounders — which is almost always the case in social network data.
4. Implement causal model marking. A model learned with FCI or PC from causal data can be marked as causal: `model.mark_as_causal(discovery_algorithm="FCI")`. Models not marked as causal raise `LutufiNonCausalModelError` if the user attempts to call do-calculus operations.
5. Write tests comparing PC and FCI on synthetic data: generate data from a known DAG with a hidden confounder, run both algorithms, verify FCI correctly identifies the hidden confounder signature while PC produces an incorrect structure.

**Definition of Done:** PC and FCI produce correct structures on synthetic data. FCI correctly identifies hidden confounders. Causal model marking prevents misuse of do-calculus.

---

## Phase 5 — Causal Inference

### Duration: 3 Sprints (6 Weeks)

### Objective: Do-calculus, interventional distributions, counterfactuals — the capability that separates Lutufi from all existing tools.

---

### Sprint 5.1 — Structural Causal Models and the Do-Operator

**Goal:** The core causal model and the do-operator implementation.

**Tasks:**

1. Implement `CausalModel` — a subclass of BayesianNetwork with structural causal semantics. Each edge represents a direct causal mechanism, not just a probabilistic dependency. Add: `mark_hidden_confounder(var1, var2)` for marking known hidden common causes using bidirected edges.
2. Implement the `do()` operator: `model.do(X="x")` returns a new model (the mutilated model) where all incoming edges to X are removed and X is set to value x. The mutilated model can then be queried normally: `mutilated.query(["Y"])` returns P(Y | do(X=x)).
3. Implement the back-door criterion: given a treatment variable X and outcome variable Y, check whether a given set of covariates Z satisfies the back-door criterion (blocks all back-door paths from X to Y without conditioning on descendants of X). Implement `model.satisfies_backdoor(X, Y, Z)`.
4. Implement back-door adjustment: if the back-door criterion is satisfied by Z, compute P(Y | do(X)) = Σ_z P(Y | X, Z=z) P(Z=z) without running any simulation.
5. Implement the front-door criterion and front-door adjustment for cases where back-door adjustment is not applicable due to unmeasured confounders.
6. Write causal inference tests: generate data from a known SCM with a hidden confounder, run back-door adjustment, verify the estimated causal effect matches the true causal effect. This is the fundamental test that Lutufi's causal claims are correct.

**Definition of Done:** Do-operator produces correct interventional distributions on synthetic SCMs. Back-door adjustment recovers correct causal effects with hidden confounders.

---

### Sprint 5.2 — The ID Algorithm and General Identifiability

**Goal:** Automatic identification of causal effects in arbitrary graphs.

**Tasks:**

1. Implement the ID algorithm (Shpitser & Pearl 2006). Given a causal query P(Y | do(X)) and a causal graph possibly with hidden variables (bidirected edges), the ID algorithm either returns an identification formula (an expression in terms of observable distributions that computes the causal effect) or returns FAIL (proof that the causal effect is not identifiable from observational data).
2. Implement the IDC algorithm for conditional interventional distributions P(Y | do(X), Z).
3. Implement the `model.identify(Y, do_X)` API. Returns either: an `IdentificationResult` with the formula and a method to evaluate it, or an `IdentificationFailure` with a proof of non-identifiability (a hedge structure — a specific subgraph pattern that proves non-identifiability).
4. Implement evaluation of identification formulas. Once identified, `result.evaluate(data)` computes the causal effect numerically from observational data using the derived formula.
5. Write tests on known identifiable and non-identifiable graphs from the literature. Verify that identifiable queries are correctly identified and non-identifiable queries correctly return FAIL.

**Definition of Done:** ID algorithm correctly identifies causal effects on all test cases from the literature. Non-identifiable queries return FAIL with a proof.

---

### Sprint 5.3 — Counterfactual Inference

**Goal:** Counterfactual queries using the three-step abduction-action-prediction procedure.

**Tasks:**

1. Implement counterfactual inference using the three steps: (1) Abduction: update the prior distribution over exogenous noise variables U using the observed evidence. (2) Action: apply the intervention do(X=x) by modifying the structural equations. (3) Prediction: compute the distribution over the counterfactual outcome under the modified model with updated U.
2. Implement the `model.counterfactual(observed, intervention, query)` API:

```python
# "Would Y have been different if X had been 1, given that we observed Z=z?"
result = model.counterfactual(
    observed={"Z": "z"},
    intervention={"X": 1},
    query=["Y"]
)
```

3. Write counterfactual tests. The canonical test: in a deterministic SCM where X causes Y, and we observe Y=1 under X=0, the counterfactual P(Y=1 | do(X=1), X=0) should equal 1 (the intervention would not change the outcome since it was already 1). Verify this.
4. Implement `probability_of_necessity`: P(Y=1 had X been 0 | X=1, Y=1) — probability that X was necessary for Y.
5. Implement `probability_of_sufficiency`: P(Y=1 | do(X=1), X=0, Y=0) — probability that X would have been sufficient for Y.

**Definition of Done:** Counterfactual queries pass all standard tests from Pearl's causality textbook chapter 9.

---

## Phase 6 — Dynamic Networks

### Duration: 3 Sprints (6 Weeks)

### Objective: Dynamic Bayesian networks with temporal inference, non-stationary transitions, and simulation.

---

### Sprint 6.1 — DBN Inference: Filtering and Smoothing

**Goal:** Online (filtering) and offline (smoothing) temporal inference.

**Tasks:**

1. Implement the `DBNInferenceEngine` for the two-slice DBN structure. Implements the forward algorithm (filtering): at each time step, incorporate new evidence and propagate forward.
2. Implement filtering: `P(X_t | E_{1:t})` — the posterior over the current state given all observations so far. This is online inference, suitable for real-time applications.
3. Implement smoothing: `P(X_t | E_{1:T})` — the posterior over a past state given the full observation sequence. Uses the forward-backward algorithm. Requires the complete sequence upfront.
4. Implement prediction: `P(X_{t+k} | E_{1:t})` for k > 0 — forecasting future states.
5. Implement the `HMMEngine` as a specialized DBNInferenceEngine for Hidden Markov Models. Include the Viterbi algorithm for finding the most likely hidden state sequence.
6. Implement the Kalman Filter as a specialized exact inference engine for linear Gaussian DBNs. The Kalman Filter must give results identical to the DBN forward algorithm within numerical precision for linear Gaussian models.
7. Write integration tests with known ground truth. For an HMM with known parameters, run filtering and verify against analytically computed posteriors. For a linear Gaussian model, verify Kalman Filter matches general DBN inference.

**Definition of Done:** Filtering and smoothing match analytical solutions on HMM test cases. Kalman Filter matches general DBN inference on linear Gaussian models.

---

### Sprint 6.2 — Non-Stationary DBNs and Online Learning

**Goal:** DBNs where the transition model changes over time.

**Tasks:**

1. Implement `NonStationaryDBN`: a DBN where the transition model `P(X_t | X_{t-1})` is parameterized by time t or by a higher-level regime variable. At each time step, the transition model can be different.
2. Implement regime-switching DBNs: a hidden regime variable R_t that switches between a finite set of transition models. The regime itself follows a Markov chain. This models phase transitions — the network dynamics in one regime (e.g., a market in crisis) differ from another (e.g., a market in normal conditions).
3. Implement online parameter learning for DBNs: update the transition model parameters incrementally as new observations arrive. Use online EM with a forgetting factor that downweights old observations.
4. Implement `model.simulate(T, n_samples)`: run T steps of the DBN forward from the prior, sampling from the transition model at each step. Returns a dataset of simulated trajectories. This is the first simulation capability in Lutufi.
5. Write tests verifying that simulation produces the correct stationary distribution for an ergodic DBN (run long simulations, verify marginal distributions match the theoretical stationary distribution).

**Definition of Done:** Non-stationary DBN inference produces correct results on synthetic test cases. Simulation matches stationary distribution after sufficient steps.

---

### Sprint 6.3 — Temporal Network Analysis and Incremental Inference

**Goal:** Incremental belief updates for evolving networks.

**Tasks:**

1. Implement incremental inference: when a single edge is added to or removed from a network, update the junction tree and recompute only the affected beliefs without full recompilation. Formalize the conditions under which incremental update is valid vs. full recompilation is required.
2. Implement `EvidenceManager`: maintains the current evidence set and efficiently propagates evidence changes through the junction tree without full recompilation. Adding a new observation should update beliefs in O(clique_size) time, not O(network_size) time.
3. Implement the temporal query API:

```python
# Query the state at time t given observations up to time T
result = dbn.query(
    variables=["X"],
    evidence_sequence=[{"E": "e1"}, {"E": "e2"}, {"E": "e3"}],
    time=1,           # query at t=1
    mode="smooth"     # or "filter" or "predict"
)
```

4. Write performance tests for incremental inference. Adding one observation to a 1,000-node network should be at least 10x faster with incremental updates than with full recompilation.

**Definition of Done:** Incremental inference is at least 10x faster than full recompilation for single-observation updates. Temporal query API handles all three modes correctly.

---

## Phase 7 — Missing Data

### Duration: 2 Sprints (4 Weeks)

### Objective: Principled handling of MCAR, MAR, and MNAR — the dominant data condition in real-world network analysis.

---

### Sprint 7.1 — MCAR and MAR Handling

**Goal:** Missing data handling for the two tractable missing data mechanisms.

**Tasks:**

1. Implement missingness indicators: augment each variable with a binary missingness indicator M_i that tracks whether the variable's value was observed. The full model now includes both the original variables and their indicators.
2. Implement the EM algorithm for networks with MAR missingness. E-step: for each observation, run inference to compute the expected values of missing variables conditioned on observed variables. M-step: update parameters using expected sufficient statistics. Iterate until convergence.
3. Implement multiple imputation: generate K complete datasets by sampling missing values from their posterior distributions, fit the model to each complete dataset, and pool the results using Rubin's rules.
4. Implement the missing data diagnostic: given a dataset, classify the missing data mechanism by testing whether missingness is correlated with observed variables. Report whether the MCAR assumption is tenable (Little's MCAR test), whether MAR is plausible, and whether MNAR should be suspected.

**Definition of Done:** EM on MAR data recovers correct parameters within 5% on synthetic test cases. Multiple imputation produces correctly calibrated uncertainty intervals.

---

### Sprint 7.2 — MNAR Handling and Network Reconstruction

**Goal:** MNAR missingness and Bayesian network reconstruction from partial observations.

**Tasks:**

1. Implement MNAR modeling: add the missingness mechanism as a submodel within the probabilistic model. The missingness indicators M_i are modeled as depending on both observed and missing values. This requires specifying (or learning) the missingness mechanism parameters.
2. Implement sensitivity analysis for MNAR: vary the assumed missingness mechanism parameters over a plausible range and report how much the inference conclusions change. This allows the researcher to assess robustness to MNAR assumptions.
3. Implement network reconstruction for structural missingness: given a partially observed network (some edges are known to be absent, some to be present, and many are unknown), compute the posterior distribution over the full network structure using a prior from a random graph model (e.g., Erdős-Rényi, preferential attachment).
4. Implement the `missing_edge_probability(node1, node2, model)` API: given the observed network and a prior model, return the posterior probability that an unobserved edge between node1 and node2 exists.

**Definition of Done:** MNAR sensitivity analysis produces correct bounds on synthetic data. Network reconstruction recovers unobserved edges with posterior probabilities that are well-calibrated against held-out ground truth.

---

## Phase 8 — Python Bindings

### Duration: 3 Sprints (6 Weeks)

### Objective: The clean, Pythonic API that researchers will actually use — including ecosystem integration and async support.

---

### Sprint 8.1 — Core Python API Finalization

**Goal:** The complete Python API surface, polished and documented.

**Tasks:**

1. Review every public-facing Python class and function for API consistency. Names must be consistent (snake_case for functions, PascalCase for classes), parameters must be consistently ordered (model always first, evidence always a dict, variables always a list), and defaults must be sensible.
2. Implement the full error message library. Every exception must: have a descriptive class name (`LutufiCyclicGraphError`, `LutufiNonCausalModelError`, `LutufiHighTreewidthWarning`), include the specific context (which variable, which edge, what the user did), and suggest the correct action.
3. Implement context managers for model modification:

```python
with model.edit() as m:
    m.add_edge("A", "B")
    m.set_cpd("B", new_cpd)
# Changes are committed atomically; if any step fails, all changes are rolled back
```

4. Implement lazy evaluation: `model.query()` returns a `LazyQueryResult` that doesn't compute until accessed. Computation is triggered when the user accesses `.distributions`, `.to_dataframe()`, etc. This enables query optimization when multiple queries are submitted together.
5. Implement the async inference API:

```python
result = await model.query_async(variables=["A"], evidence={"B": "1"})
```

**Definition of Done:** All public APIs are consistent. Error messages are informative. Context managers roll back on failure. Async queries work.

---

### Sprint 8.2 — Ecosystem Integration

**Goal:** Deep integration with numpy, pandas, networkx, and matplotlib.

**Tasks:**

1. Implement numpy array inputs and outputs throughout. Any place a user can pass a list, they can pass a numpy array. Any place Lutufi returns probability values, they are numpy arrays.
2. Implement the full pandas integration: `model.fit(df)` where df is a DataFrame, `result.to_dataframe()` that returns a tidy DataFrame, `QueryResult.plot()` that calls matplotlib automatically.
3. Implement NetworkX round-trip for all model types. `BayesianNetwork.from_networkx(G)`, `BayesianNetwork.to_networkx()`, `MarkovRandomField.from_networkx(G)`, `MarkovRandomField.to_networkx()`. Verify that the graph structure is preserved exactly but probabilistic information (CPTs) is lost (NetworkX has no native CPT storage) — document this limitation.
4. Implement matplotlib visualization integration: `model.plot()` draws the graph structure with node labels, `model.plot_cpd("A")` draws the CPT as a heatmap, `result.plot()` draws the probability distribution as a bar chart.
5. Write ecosystem integration tests using real datasets. Download and use the bnlearn repository's standard networks (Asia, Alarm, Sachs) to verify the full pipeline works end-to-end.

**Definition of Done:** Full pandas, numpy, networkx, matplotlib integration tested with real datasets.

---

### Sprint 8.3 — Installation and Distribution

**Goal:** `pip install lutufi` works on Linux, macOS, and Windows.

**Tasks:**

1. Set up the build system for distributing a package with a native extension. Use `maturin` (for Rust) or `scikit-build-core` (for C++). Configure pyproject.toml correctly.
2. Build wheels for: Python 3.9, 3.10, 3.11, 3.12 on Linux (x86_64 and arm64), macOS (x86_64 and arm64), Windows (x86_64). Use cibuildwheel with GitHub Actions to automate this.
3. Set up a TestPyPI deployment. Every merge to main triggers a TestPyPI release. Verify that `pip install --index-url https://test.pypi.org/simple/ lutufi` produces a working installation.
4. Write the installation test suite: a set of tests that run immediately after installation (not in the source tree) and verify basic functionality works in a clean environment.
5. Write the `INSTALL.md` with clear platform-specific instructions and common troubleshooting steps.

**Definition of Done:** `pip install lutufi` from TestPyPI works on all three platforms. Installation tests pass in a clean virtualenv.

---

## Phase 9 — Example Library

### Duration: 4 Sprints (8 Weeks)

### Objective: 30+ domain-specific examples, each structured as a research case study.

This phase is not an afterthought. It is the mechanism through which Lutufi reaches researchers. Each example is a demonstration that a previously blocked research question is now answerable.

**The structure of every example:**

1. Problem statement (what research question this addresses)
2. Data (real dataset where available, synthetic with documented generation process otherwise)
3. Model specification (the Lutufi code)
4. Inference (running the query)
5. Results (interpreting the output)
6. Discussion (what this demonstrates, what a researcher could do next)
7. References (papers this example draws from)

---

### Sprint 9.1 — Epidemiology and Public Health Examples

**Goal:** 6 epidemiology examples covering disease spread, vaccination, and contact networks.

1. SIR epidemic model as a DBN: model disease spread over a contact network, compute probability of epidemic given initial infection at a specific node.
2. Vaccination targeting: given a partially observed contact network, identify the optimal nodes to vaccinate to minimize expected infections using interventional queries.
3. Transmission probability inference: given an outbreak and a contact network, infer the posterior distribution over transmission probability.
4. Healthcare worker network: model infection risk propagation through a hospital staff contact network.
5. Vaccine hesitancy diffusion: model how vaccine hesitancy propagates through a social network using a DBN.
6. Missing contact data: demonstrate network reconstruction and its effect on epidemic risk estimates using simulated incomplete contact tracing data.

---

### Sprint 9.2 — Finance and Economics Examples

**Goal:** 6 financial examples covering systemic risk, contagion, and supply chains.

1. Interbank contagion: model default cascade probability in a simple interbank network, compute P(systemic crisis | bank X defaults).
2. Portfolio systemic risk: identify the nodes in a financial exposure network whose failure most increases systemic risk using interventional queries.
3. Supply chain disruption: model supply chain as a DBN, simulate disruption propagation, compute probability of production halt given upstream failure.
4. Money laundering network detection: model a layering network as a factor graph, compute posterior probability that a cluster of entities is part of a money laundering scheme.
5. Credit risk propagation: model credit risk as a Bayesian network over counterparty exposure, compute conditional VaR given correlated defaults.
6. Market regime detection: use a regime-switching DBN to infer the current market regime (crisis vs. normal) from observable indicators.

---

### Sprint 9.3 — Social Science and Intelligence Examples

**Goal:** 8 social science, political, and intelligence examples.

1. Opinion dynamics: model belief propagation over a social network using a DBN, compute probability of majority adoption given seeding at specific nodes.
2. Misinformation spread: compare interventional distributions under different correction strategies — which network position to target to most effectively halt misinformation.
3. Influence detection: given partial observations of a social network, infer which nodes are most likely to be high-influence using structural inference.
4. Political coalition formation: model coalition stability as a Bayesian network, compute probability of coalition collapse given defection of key member.
5. Covert network reconstruction: given partial observations of a covert network, compute posterior probabilities over the full network structure using Bayesian network reconstruction.
6. Dark network resilience: model an adversarial network, compute the probability that it retains operational capacity after targeted node removal.
7. Radicalization pathway modeling: model progression through an ideological network as an HMM, infer most likely pathway given observable behaviors.
8. Social mobility as a network problem: model access to high-status network positions using a causal model, compute causal effect of network position on economic outcomes.

---

### Sprint 9.4 — Organizational, Policy, and Methodological Examples

**Goal:** 10 examples covering organizational science, policy analysis, and methodological demonstrations.

1. Organizational knowledge loss: model knowledge flow in an organization, compute probability of critical knowledge loss given employee departures.
2. Policy intervention simulation: compute interventional distribution under different policy actions using do-calculus.
3. Causal structure discovery: demonstrate FCI algorithm on synthetic data with known ground truth causal structure.
4. Missing data handling: demonstrate the difference between MCAR, MAR, and MNAR assumptions on the same network and data.
5. Sensitivity analysis: show how inference conclusions change as model parameters vary, demonstrating model robustness testing.
6. Reproducibility demonstration: show the complete workflow from data to published result using Lutufi's serialization format.
7. Model comparison: compare two competing causal models using Bayesian model comparison.
8. Identifiability analysis: demonstrate the ID algorithm finding and failing to find identification formulas on different graph structures.
9. Counterfactual analysis: demonstrate counterfactual reasoning in a medical diagnosis context.
10. The Loom integration example: a demonstration of Lutufi being used as the probabilistic backbone for a risk scoring system over a financial entity network — connecting directly to the Loom architecture.

---

## Phase 10 — Scalability

### Duration: 3 Sprints (6 Weeks)

### Objective: 1M+ edge networks on commodity hardware.

---

### Sprint 10.1 — Sparse Representations and Memory Optimization

**Goal:** Memory-efficient representations that do not scale exponentially with parent set size.

**Tasks:**

1. Implement and benchmark sparse CPT storage. For CPTs where density < 30%, use COO (coordinate) format. For inference, convert to CSR (compressed sparse row) for efficient row access during message passing.
2. Implement Noisy-OR factors: a compact parameterization for CPTs with many binary parents. Instead of 2^k table entries for k parents, Noisy-OR requires only k parameters. Verify that inference with Noisy-OR factors produces the same results as the equivalent full CPT.
3. Implement context-specific independence (CSI): allow CPTs that simplify to smaller tables for specific parent configurations. A parent's effect may only be relevant in certain contexts. CSI can reduce CPT size by orders of magnitude.
4. Profile memory usage on the 1M-edge benchmark network. Identify the top 3 memory bottlenecks. Address each one. Re-benchmark.

**Definition of Done:** 1M-edge sparse network loads without out-of-memory error on a machine with 16GB RAM.

---

### Sprint 10.2 — Parallel Inference and Algorithmic Optimization

**Goal:** Multi-core parallelism for message passing.

**Tasks:**

1. Implement parallel message passing for LBP: messages on different edges can be updated independently in the asynchronous schedule. Parallelize across a thread pool using a work-stealing queue.
2. Implement parallel factor products: when computing the product of a large set of factors (as in variable elimination), parallelize the products across the thread pool.
3. Implement lazy evaluation throughout the inference engine: do not compute factor products or messages until they are actually needed for the current query.
4. Profile inference time on the 100K-node benchmark. Identify the top 3 compute bottlenecks. Address each one. Re-benchmark.
5. Run the full benchmark suite from Sprint 3.4 baseline comparison. Every metric should be better than the baseline. Document the improvement.

**Definition of Done:** Inference on a 100K-node sparse network completes in under 60 seconds on 8-core commodity hardware. Speedup from parallelism is at least 4x on 8 cores.

---

### Sprint 10.3 — Out-of-Core Computation and Streaming

**Goal:** Networks that exceed available RAM.

**Tasks:**

1. Implement memory-mapped factor storage: large CPT tables can be stored on disk and memory-mapped for access. Pages are loaded on demand and evicted when memory pressure requires.
2. Implement chunked variable elimination: for very large networks, eliminate variables in chunks, writing intermediate factors to disk if they exceed a size threshold.
3. Implement the streaming inference API for DBNs: process observations one at a time from a Kafka stream or file iterator without loading the full dataset into memory.
4. Write a stress test: a network with 1M nodes and 10M edges. Inference should run to completion without out-of-memory errors even on a machine with 8GB RAM (using memory-mapped storage and chunking).

**Definition of Done:** 1M-node, 10M-edge stress test completes without OOM error on 8GB RAM machine.

---

## Phase 11 — Serialization

### Duration: 2 Sprints (4 Weeks)

### Objective: A complete, versioned, reproducible model format.

---

### Sprint 11.1 — The Lutufi Model Format

**Goal:** A single file that captures everything needed to reproduce any Lutufi result exactly.

**Tasks:**

1. Design and implement the Lutufi Model Format (LMF) — a JSON-based format with the following sections: metadata (version, creation date, Lutufi version, author), graph structure (nodes, edges, model type), parameters (CPTs, factor potentials in compressed form), evidence (observed values), inference settings (algorithm, parameters used), and results (computed distributions with algorithm provenance).
2. Implement `model.save("network.lmf")` and `model.load("network.lmf")`. Round-trip must be exact: a loaded model must be bitwise-identical to the saved model for discrete models, and within 1e-15 for continuous models.
3. Implement versioning: the LMF format is versioned. When loading a file, Lutufi checks the format version and applies migration logic if needed. Write a migration framework that can upgrade old format files to the current format.
4. Implement the reproducibility verification: `lutufi verify network.lmf` re-runs the inference recorded in the file and verifies the results match the stored results within numerical tolerance.

**Definition of Done:** Perfect round-trip for all model types. Version migration framework tested on an artificial version upgrade scenario. Reproducibility verification passes on all example files.

---

### Sprint 11.2 — Interoperability Formats

**Goal:** Import from and export to external formats used by other tools.

**Tasks:**

1. Implement BIF (Bayesian Interchange Format) import and export. Test against the standard BIF files from the bnlearn repository.
2. Implement XMLBIF import and export. Test against known XMLBIF files.
3. Implement UAI format import and export (used in the UAI inference competition).
4. Implement CSV export of results: a standardized CSV format for query results that any statistics package can read.
5. Implement the `lutufi convert` command-line utility: `lutufi convert network.bif network.lmf` for format conversion.

**Definition of Done:** Round-trip through BIF, XMLBIF, and UAI formats preserves all structural and probabilistic information.

---

## Phase 12 — Documentation

### Duration: 3 Sprints (6 Weeks)

### Objective: Documentation that makes researchers trust Lutufi and use it correctly.

---

### Sprint 12.1 — API Reference Documentation

**Goal:** Complete, accurate, example-rich API documentation for every public symbol.

**Tasks:**

1. Write docstrings for every public class, method, and function. Each docstring follows the NumPy format: summary line, extended description, Parameters section, Returns section, Raises section, Examples section with executable code.
2. Set up Sphinx documentation build. Configure autodoc to extract docstrings. Configure the theme (use PyData theme for the scientific Python aesthetic).
3. Write the API reference pages for: Models (BayesianNetwork, MarkovRandomField, DynamicBayesianNetwork, CausalModel), Inference (VariableElimination, JunctionTree, BeliefPropagation, GibbsSampling, VariationalInference), Learning (parameter_learning, structure_learning), Causal (do, identify, counterfactual), Missing Data, I/O.
4. Verify that every code example in the documentation actually runs correctly. Use doctest to run them automatically.

**Definition of Done:** Zero undocumented public symbols. All doctest examples pass.

---

### Sprint 12.2 — Conceptual Guides and Tutorials

**Goal:** Narrative documentation that teaches researchers how to think with Lutufi.

**Tasks:**

1. Write the Getting Started tutorial: installs Lutufi, builds a small network, runs inference, interprets results. Must be completable in under 20 minutes by a researcher with no prior Lutufi experience.
2. Write conceptual guides: "Choosing a Model Type" (when to use BN vs MRF vs DBN), "Choosing an Inference Algorithm" (when exact inference is feasible, when to use LBP vs MCMC vs variational), "Working with Missing Data" (understanding MCAR/MAR/MNAR), "Causal vs Statistical Models" (when causal semantics are valid and when they are not).
3. Write the mathematical foundations guide: a concise but rigorous treatment of the mathematics behind Lutufi's core operations, with links to the academic references. This is the document a reviewer reads when they want to know whether Lutufi's implementation is correct.
4. Write the troubleshooting guide: common errors and what they mean, how to diagnose convergence problems, how to handle high-treewidth models, how to detect model misspecification.
5. Write the reproducibility guide: how to attach a Lutufi model file to a paper, how a reviewer reproduces results, how to cite Lutufi.

**Definition of Done:** Getting Started tutorial is tested by two people who have never used Lutufi and completed in under 20 minutes.

---

### Sprint 12.3 — Mathematical Derivation Documents and References

**Goal:** The academic-quality reference layer that establishes Lutufi's intellectual credibility.

**Tasks:**

1. For each inference algorithm, write a mathematical derivation document: variable elimination, junction tree, LBP, Gibbs sampling, variational inference, forward-backward, Kalman filter. Each document should: state the algorithm precisely, derive it from first principles, state its computational complexity, state its assumptions and when they fail, and provide a worked numerical example.
2. Write the test validation document: for each ground truth test case, document the expected result, how it was computed analytically, and the numerical tolerance used for comparison.
3. Write the known limitations document: a comprehensive list of what Lutufi does not do, what it does poorly, and where researchers should use alternative tools.
4. Publish the complete bibliography as `BIBLIOGRAPHY.md` with annotation of what each reference contributes to Lutufi.

**Definition of Done:** Mathematical derivation documents exist for all 7 inference algorithms. Known limitations document is honest and complete.

---

## Phase 13 — Hardening

### Duration: 2 Sprints (4 Weeks)

### Objective: Security, robustness, edge cases — preparing for v1.0 release.

---

### Sprint 13.1 — Security and Input Validation

**Goal:** Lutufi handles adversarial or malformed inputs gracefully.

**Tasks:**

1. Implement comprehensive input validation at all entry points. Every public API function must validate its inputs before processing. Malformed inputs must produce clear error messages, not undefined behavior or crashes.
2. Audit the file format parsers (LMF, BIF, XMLBIF, UAI) for injection vulnerabilities. A malicious LMF file should not be able to execute arbitrary code, exhaust memory, or produce incorrect results.
3. Implement resource limits: configurable maximum network size, maximum CPT size, maximum inference time. Exceeding limits raises `LutufiResourceLimitError` with a clear message, not an OOM crash.
4. Run the full test suite with a fuzzing library (cargo-fuzz for Rust, atheris for Python) against all file parsers and API entry points. Any crash from fuzz inputs is a bug.
5. Write the security disclosure policy in `SECURITY.md`.

**Definition of Done:** No crashes from 24 hours of fuzz testing. Resource limits tested and working.

---

### Sprint 13.2 — Final Polish and v1.0 Release

**Goal:** v1.0 release on PyPI.

**Tasks:**

1. Run the complete test suite one final time. Coverage must be above 90%. All ground truth tests must pass. All example library examples must run without errors.
2. Run the benchmark suite and publish the performance results in `docs/benchmarks/v1.0.md`. Include comparison with pgmpy and bnlearn on standard test networks.
3. Write the CHANGELOG for v1.0. Every public API change since 0.1.0-dev must be documented.
4. Tag version 1.0.0 in git. Build and publish wheels to PyPI. Verify `pip install lutufi` works on all platforms.
5. Write the announcement post for relevant communities: Reddit r/MachineLearning, Reddit r/statistics, INSNA mailing list, cross-post to social network analysis forums.

**Definition of Done:** `pip install lutufi` from PyPI works on Linux, macOS, Windows. All tests pass. Benchmark results published.

---

## Phase 14 — Community and Publication

### Duration: 4 Sprints (8 Weeks)

### Objective: JOSS paper accepted, active community, first external contributions.

---

### Sprint 14.1 — JOSS Paper Preparation

**Goal:** Submit to the Journal of Open Source Software.

**Tasks:**

1. Write the JOSS paper (1000–2000 words). JOSS requires: a summary of the software, a statement of need, mentions of comparable projects and how Lutufi differs, a description of the target audience, and a mention of any published uses. Read at least 10 accepted JOSS papers for software of similar scope to calibrate the writing level.
2. Review the JOSS submission requirements against Lutufi's repository. Verify: clear installation instructions, comprehensive documentation, automated tests, contribution guidelines, a license file, a code of conduct.
3. Submit to JOSS. Respond promptly and thoroughly to all reviewer comments.

**Definition of Done:** JOSS submission submitted.

---

### Sprint 14.2 — Conference Presentations

**Goal:** Submit abstracts to relevant conferences.

**Tasks:**

1. Submit a poster or talk abstract to NetSci (network science conference).
2. Submit to INSNA Sunbelt (International Network for Social Network Analysis).
3. Submit to a domain-specific conference in one of Lutufi's target applications (an epidemiology conference, an econometrics conference, or an intelligence studies conference).
4. Prepare the presentation materials: slides, poster, demo script.

---

### Sprint 14.3 — Community Infrastructure

**Goal:** Infrastructure that makes contributing to Lutufi easy and welcoming.

**Tasks:**

1. Set up GitHub Discussions for questions and feature requests. Write pinned posts explaining: how to ask a question, how to request a feature, how to report a bug, the project's roadmap.
2. Write the "Good First Issue" guide and label 10 existing GitHub issues as good-first-issue. These should be genuine contributions, not busy work — documentation improvements, additional examples, new format importers.
3. Write the core contributor guide explaining how to make significant contributions (new inference algorithms, new model types, new domains).
4. Respond to every issue and PR within 48 hours. Merging a first-time contributor's PR is a milestone event — acknowledge it.

---

### Sprint 14.4 — Feedback Integration and v1.1

**Goal:** Address the first wave of community feedback.

**Tasks:**

1. Collect and triage all user feedback from the first month of public availability. Categorize as: bug (fix immediately), documentation gap (address in v1.1), feature request (prioritize for v2.0 roadmap), won't fix (document the reason publicly).
2. Fix all reported bugs. Publish v1.0.1 with bug fixes.
3. Address the top 5 documentation gaps reported by users.
4. Write the v1.1 roadmap based on user feedback.

---

## Phase 15 — Simulation and Visualization

### Duration: 4 Sprints (8 Weeks)

### Objective: v2.0 capabilities — forward simulation with interventions and semantically aware visualization.

---

### Sprint 15.1 — Forward Simulation Engine

**Goal:** Monte Carlo simulation from current network state with interventional support.

**Tasks:**

1. Implement `model.simulate(n_samples, horizon, intervention)`: samples trajectories forward in time from a DBN or from the current posterior of a static BN. Interventions use the do-operator to define modified transition dynamics.
2. Implement simulation queries: P(Y_{t+k} | current_state, do(X=x)) estimated from simulation. Returns full probability distributions over future outcomes.
3. Implement the `SimulationResult` class: stores all trajectories, computes posterior distributions, credible intervals, and first passage times (time until a threshold is crossed).
4. Connect the simulation engine to the causal inference layer: `model.simulate_intervention(do={"X": 1}, n_samples=10000, horizon=30)` compares the simulated future under the intervention vs. no intervention.

**Definition of Done:** Simulation produces correct stationary distributions. Interventional simulation correctly differs from observational simulation on a known test case.

---

### Sprint 15.2 — Visualization Engine

**Goal:** Semantically aware visualization that makes probabilistic network models legible.

**Tasks:**

1. Implement `model.plot()` with probabilistic semantics: node size proportional to marginal entropy (high entropy = high uncertainty), node color reflecting posterior probability of a specified state, edge width proportional to mutual information between connected variables.
2. Implement `model.plot_inference(result)`: shows the network before and after evidence is observed, with nodes colored by how much their beliefs changed.
3. Implement `model.plot_causal(X, Y)`: highlights the causal pathways from X to Y, with edge coloring showing the contribution of each path to the total causal effect.
4. Implement interactive visualization using Plotly: nodes are clickable, clicking a node shows its CPT and current beliefs, hovering shows the Markov blanket.
5. Write visualization examples for the example library: show what belief propagation looks like visually, show how evidence changes beliefs across the network.

**Definition of Done:** All visualization functions produce informative, publication-quality figures. Interactive visualization works in Jupyter notebooks.

---

### Sprint 15.3 — R Bindings

**Goal:** Lutufi accessible from R for the statistics and epidemiology communities.

**Tasks:**

1. Implement R bindings using Rcpp (for C++ core) or rextendr (for Rust core). Expose the core model types, inference engines, and learning algorithms to R.
2. Implement an R-native API following R conventions: S3 classes, formula interface for specifying models, tidy output formats compatible with the tidyverse.
3. Write R vignettes demonstrating Lutufi from R for: a social network analysis use case, a financial contagion use case.
4. Submit to CRAN.

**Definition of Done:** `install.packages("lutufi")` from CRAN works. Vignettes run without errors.

---

### Sprint 15.4 — v2.0 Release

**Goal:** v2.0 with simulation, visualization, R bindings, and the second generation of examples.

**Tasks:**

1. Run the full test suite. Fix all failures.
2. Run all examples. Fix all failures.
3. Write the v2.0 release notes and CHANGELOG.
4. Publish v2.0 to PyPI and CRAN.
5. Write a blog post announcing v2.0 with a showcase of simulation and visualization capabilities.

---

## Sprint Rhythm and Personal Discipline

**Day 1 of every sprint:** Read the sprint objectives. Write them in your own words in a daily log. Identify the hardest task. Start it first.

**Day 7 of every sprint:** Mid-sprint check. Are you on pace? If behind, cut scope — do fewer tasks perfectly rather than all tasks partially. Never carry incomplete work forward.

**Day 13 of every sprint:** Write the definition-of-done checklist. Run through it item by item. If anything fails, fix it today.

**Day 14 of every sprint:** Sprint review day. Commit everything. Update the open questions log. Note what surprised you. Rest before the next sprint begins.

**The rule that overrides all others:** At the end of every sprint, the codebase must be in a state where someone else could check it out and understand it. If it is not, the sprint is not done.

---

## The North Star Metric

At every point in development, you should be able to answer this question:

**"Can a researcher with a network and a question use Lutufi to get an answer they can publish?"**

- After Phase 2: The answer is yes, for small networks with known structure and complete data.
- After Phase 4: The answer is yes, for any network where you can provide or learn the structure from data.
- After Phase 5: The answer is yes, including causal questions.
- After Phase 8: The answer is yes, and the experience is pleasant.
- After Phase 9: The answer is yes, and there is an example showing exactly how to do it for their domain.
- After Phase 13 (v1.0): The answer is yes, reliably, at scale, with documentation, with reproducibility, with a citable paper.

Every sprint moves that answer from "not yet" to "yes, and more."

---

*Document ends. The camel is tied. Now ride.*
