# Personal Knowledge Audit: Lutufi Project

---

**Document Version:** 1.0
**Status:** Working Draft — Research Phase
**Author:** Wasswa Lutufi Sebbanja
**Last Updated:** March 2026
**License:** Apache 2.0

---

## Table of Contents

1. [Purpose of This Audit](#1-purpose-of-this-audit)
2. [Assessment Framework](#2-assessment-framework)
3. [Areas of Deep Knowledge](#3-areas-of-deep-knowledge)
4. [Areas of Working Knowledge](#4-areas-of-working-knowledge)
5. [Areas of Partial Knowledge](#5-areas-of-partial-knowledge)
6. [Areas of Acknowledged Gaps](#6-areas-of-acknowledged-gaps)
7. [Knowledge Acquisition Strategy](#7-knowledge-acquisition-strategy)
8. [Study Plans by Area](#8-study-plans-by-area)
9. [Accountability Mechanism](#9-accountability-mechanism)
10. [Audit Revision History](#10-audit-revision-history)

---

## 1. Purpose of This Audit

Building Lutufi — a library that unifies probabilistic graphical models with social and economic network analysis — requires genuine competence across an unusually wide range of disciplines. Probability theory, graph theory, Bayesian inference, causal reasoning, social network analysis, financial network modeling, numerical computing, software engineering, and research methodology all converge in this project. No single person can claim expert-level mastery across every one of these domains. The honest recognition of this fact is not a weakness; it is the precondition for building something rigorous rather than something that merely appears rigorous.

This document exists for several concrete reasons:

**Accountability before design.** Every design decision in Lutufi rests on the author's understanding of the underlying theory. If that understanding has gaps, those gaps will become design flaws — flaws that may not be visible to the author but will be immediately visible to domain experts. By cataloguing what I know well, what I know partially, and what I do not know, I create an honest map of where the design is on solid ground and where it is at risk. This map guides study priorities before any code is written.

**Preventing confident ignorance.** The most dangerous state for a library author is confident ignorance — believing one understands a topic well enough to design an API or algorithm around it, when in fact the understanding is superficial. The Dunning-Kruger effect is real and particularly treacherous in mathematical and algorithmic domains where partial understanding can produce code that runs correctly on simple cases but fails subtly on edge cases that experts would anticipate. This audit forces confrontation with the boundaries of my knowledge.

**Guiding the study plan.** The Research Phase (Phase 0) of Lutufi exists precisely to close knowledge gaps before implementation begins. But study without prioritization is inefficient. This audit identifies which gaps are most critical to the project's success and therefore which must be addressed first. A gap in, say, distributed computing is real but not blocking for an initial single-machine implementation. A gap in junction tree inference is a direct threat to core functionality.

**Establishing credibility with collaborators.** Any serious collaborator or reviewer will want to know that the author understands the limits of their own expertise. An honest knowledge audit signals intellectual maturity and makes collaboration more effective: a collaborator can see where their expertise complements the author's, rather than duplicating it.

**Creating a living document.** This audit is not a one-time exercise. It will be revised as knowledge is acquired, gaps are closed, and new gaps are discovered. Each revision provides a concrete record of intellectual progress — a log that serves both as motivation and as evidence that the Research Phase is producing real growth.

---

## 2. Assessment Framework

### Assessment Scale

Each knowledge area is assessed on a four-level scale. The levels are defined by functional criteria — what can the person *do* with their knowledge — rather than by subjective feelings of confidence.

| Level | Label | Definition | Functional Test |
|---|---|---|---|
| **4** | **Deep Knowledge** | Could teach the topic to a graduate student. Understands not just the standard material but the edge cases, historical debates, common misconceptions, and connections to adjacent topics. Can derive results from first principles and recognize when standard approaches fail. | Could write a comprehensive tutorial, design an API that experts would respect, and anticipate the questions a reviewer would ask. |
| **3** | **Working Knowledge** | Understands the topic well enough to implement algorithms correctly, select appropriate methods for a given problem, and read the primary literature without excessive difficulty. Knows where the standard references are and can navigate them effectively. | Could implement the core algorithms, explain design tradeoffs to a peer, and identify when a problem requires a method outside current competence. |
| **2** | **Conceptual Knowledge** | Understands what the topic is about, why it matters, and how it fits into the broader landscape. Can follow explanations and derivations with effort but could not independently reproduce them or implement algorithms without significant study. Knows enough to know what is not known. | Could explain the topic at a high level, identify when it is relevant to a problem, and formulate specific questions to guide further study. |
| **1** | **Acknowledged Gap** | Knows the topic exists and has some sense of why it might be relevant, but lacks even a conceptual understanding of its content. Cannot meaningfully evaluate claims or make design decisions in this area without external guidance. | Could name the topic and explain why it matters for the project, but could not go deeper without structured study. |

### Honesty Protocol

Several principles govern how assessments are made:

- **When in doubt, rate lower.** Overestimating knowledge is the more dangerous error. If the assessment is genuinely on the boundary between two levels, choose the lower one.
- **Assess based on what can be done *today*, not what was known in the past.** Knowledge atrophies. If a topic was studied years ago but the details have faded, the current level is what matters.
- **Distinguish between recognition and generation.** Being able to follow a derivation when reading it (recognition) is not the same as being able to reproduce it independently (generation). The functional test requires generation.
- **Separate the topic from adjacent topics.** Knowing probability theory well does not automatically mean knowing measure-theoretic probability well. Each subtopic is assessed independently.

---

## 3. Areas of Deep Knowledge

These are domains where the author's knowledge is sufficient to make confident design decisions, write authoritative documentation, and withstand expert scrutiny. Deep knowledge does not mean omniscience — there are always deeper layers — but it means the foundation is solid enough to build on without immediate risk.

### 3.1 Probability Theory (Discrete and Finite)

**Assessment: Level 4 — Deep**

The mathematical foundation of Bayesian networks is probability theory, and within the discrete and finite setting the author's knowledge is strong. This includes:

- **Axioms and foundations.** Kolmogorov's axioms, probability spaces (Ω, F, P), the distinction between frequentist and Bayesian interpretations, and why the axiomatic framework is interpretation-neutral.
- **Conditional probability and Bayes' theorem.** Formal definition of conditional probability P(A|B) = P(A∩B)/P(B), the chain rule for joint distributions, Bayes' theorem as an inversion of conditioning, and the role of prior, likelihood, and posterior in Bayesian reasoning.
- **Independence and conditional independence.** Statistical independence P(A∩B) = P(A)P(B), conditional independence P(A∩B|C) = P(A|C)P(B|C), the semi-graphoid axioms, and why conditional independence is the semantic foundation of graphical models.
- **Random variables and distributions.** Discrete random variables, probability mass functions, joint distributions, marginal distributions, expectation, variance, common discrete distributions (Bernoulli, binomial, categorical, multinomial, Poisson, geometric).
- **Information-theoretic quantities.** Entropy H(X), conditional entropy H(X|Y), mutual information I(X;Y), KL divergence D_KL(P||Q), and their roles in model selection and variational inference.

**Why this matters for Lutufi:** Every probabilistic computation in Lutufi rests on these foundations. The ability to verify that inference algorithms produce correct marginals, that parameter learning converges to correct estimates, and that causal interventions are computed properly all depend on a deep understanding of probability theory.

**Ongoing refinement:** While the discrete case is strong, the extension to continuous and measure-theoretic probability is assessed separately (see Section 5.3).

### 3.2 Basic Graph Theory

**Assessment: Level 4 — Deep**

Graph theory provides the structural language for both network science and graphical models. The author's knowledge of foundational graph theory is comprehensive:

- **Fundamental definitions.** Graphs G = (V, E), directed and undirected graphs, weighted graphs, multigraphs, hypergraphs, bipartite graphs. Adjacency matrices, incidence matrices, edge lists, adjacency lists.
- **Graph properties.** Degree (in-degree, out-degree), paths, walks, cycles, connectivity (strong and weak), connected components, trees, forests, DAGs.
- **Acyclicity and topological ordering.** Directed acyclic graphs and why they matter for Bayesian networks, topological sort algorithms, the relationship between DAGs and partial orders.
- **Graph traversal.** BFS, DFS, and their applications to reachability, cycle detection, topological sorting, and connected component identification.
- **Subgraph concepts.** Cliques, independent sets, vertex covers, complete subgraphs, induced subgraphs, moralization (converting a DAG to an undirected graph by marrying parents and dropping directions).
- **Trees and tree decomposition.** Spanning trees, minimum spanning trees, tree width, junction trees (clique trees), the role of tree decomposition in exact inference.
- **Separation and connectivity.** Vertex separators, d-separation in DAGs (the Bayes-Ball algorithm), the equivalence between d-separation and conditional independence in faithful distributions.

**Why this matters for Lutufi:** Lutufi's data model is fundamentally a graph. Every operation — from model specification to inference to network metric computation — relies on graph-theoretic concepts. The junction tree algorithm, which is the primary exact inference method, is itself a graph-theoretic construction. D-separation, which determines what can be inferred from a Bayesian network's structure, is a graph-theoretic criterion.

### 3.3 Python Programming

**Assessment: Level 4 — Deep**

Python is the primary user-facing language for Lutufi. The author's Python knowledge spans:

- **Core language.** Data model, object-oriented programming, iteration protocols, generators, context managers, decorators, descriptors, metaclasses, type hints, exception handling.
- **Standard library.** Collections (defaultdict, Counter, OrderedDict, deque), itertools, functools, pathlib, json, csv, logging, unittest, typing.
- **Scientific stack.** NumPy (arrays, broadcasting, ufuncs, linear algebra), SciPy (sparse matrices, optimization, statistical distributions), pandas (DataFrames, indexing, groupby, merge), matplotlib and visualization.
- **Package development.** setuptools, pyproject.toml, wheel distribution, virtual environments, dependency management, semantic versioning, documentation with Sphinx, testing with pytest.
- **Performance considerations.** When Python is fast enough and when it isn't, profiling, NumPy vectorization, Cython basics, C extension interfaces (ctypes, cffi), the GIL and its implications.
- **API design.** Pythonic API conventions, the principle of least surprise, method chaining, context managers for resource management, the distinction between data classes and mutable state.

**Why this matters for Lutufi:** The user-facing API must be idiomatic Python that integrates naturally with the scientific Python ecosystem. The author's ability to design this API well — to make common operations simple and complex operations possible — directly determines whether researchers will adopt Lutufi.

### 3.4 Software Engineering Fundamentals

**Assessment: Level 4 — Deep**

Building a library that will be used by researchers requires software engineering discipline:

- **Version control.** Git workflows, branching strategies, conventional commits, semantic versioning.
- **Testing.** Unit testing, integration testing, property-based testing, test coverage, mocking, fixtures, test-driven development.
- **Documentation.** API documentation, tutorial-style documentation, reference documentation, docstrings (NumPy style), README conventions, changelog maintenance.
- **Code quality.** Linting (flake8, ruff), formatting (black), type checking (mypy), code review practices, refactoring patterns.
- **Architecture.** Separation of concerns, layered architecture, dependency injection, interface design, the SOLID principles, package structure.
- **Open source practices.** LICENSE selection, CONTRIBUTING guides, issue templates, pull request workflows, community governance, code of conduct.

**Why this matters for Lutufi:** A library is only as trustworthy as its engineering. Correct algorithms poorly engineered — without tests, without documentation, without CI — will not be trusted by the research community. The engineering infrastructure must be as solid as the mathematics.

---

## 4. Areas of Working Knowledge

These are domains where the author can implement algorithms correctly, make informed design decisions, and engage with the primary literature, but where there are known sub-areas that require further study before the implementation is complete. Working knowledge is sufficient to begin design and prototyping but requires deepening in specific directions.

### 4.1 Bayesian Network Algorithms (Exact Inference)

**Assessment: Level 3 — Working**

The author understands the core exact inference algorithms and can implement basic versions, but deeper mastery of optimization and edge cases is needed:

- **Variable elimination.** The algorithm, elimination orderings, the relationship between elimination ordering and tree width, computational complexity, the creation of intermediate factors. *Can implement from scratch.*
- **Junction tree algorithm.** Moralization, triangulation, construction of the clique tree, message passing (Collect-Evidence / Distribute-Evidence), the Hugin and Shafer-Shenoy architectures. *Can implement with reference to Koller & Friedman, but not yet from memory for all steps.*
- **Complexity analysis.** Understanding that exact inference is NP-hard in general (Cooper, 1990), that tree width determines tractability, and the practical implications for network size. *Solid conceptually but need to review specific complexity proofs.*
- **Evidence handling.** Hard evidence (observed variables), soft/virtual evidence (likelihood ratios), Jeffrey's rule for updating. *Working knowledge of hard evidence; soft evidence handling needs review.*

**Study needs:** Optimization of elimination orderings (the minimum-degree and minimum-fill heuristics), incremental junction tree updates when evidence changes, the Shenoy-Shafer vs. Hugin architecture tradeoffs in detail, and Lauritzen-Spiegelhalter's original algorithm.

**Key resources:**
- Koller & Friedman, *Probabilistic Graphical Models* (2009), Chapters 9–10
- Lauritzen & Spiegelhalter, "Local Computations with Probabilities on Graphical Structures" (1988)
- Jensen & Nielsen, *Bayesian Networks and Decision Graphs* (2007)

**Timeline:** Deepen to near-Level-4 by end of Month 2 of Research Phase.

### 4.2 Network Science Metrics and Algorithms

**Assessment: Level 3 — Working**

The author has solid working knowledge of standard network metrics and can compute them, but the deeper theoretical foundations and edge cases of some measures need study:

- **Centrality measures.** Degree, betweenness, closeness, eigenvector centrality, PageRank, Katz centrality. *Can compute and interpret all of these. Understand the spectral theory behind eigenvector centrality at a working level.*
- **Community detection.** Modularity, the Louvain algorithm, label propagation, spectral clustering, stochastic block models. *Can apply these methods. Need deeper understanding of the statistical foundations of stochastic block models.*
- **Network models.** Erdős–Rényi, Barabási–Albert (preferential attachment), Watts–Strogatz (small world), configuration model, planted partition model. *Working knowledge of all. Need to study the mathematical properties of configuration models more deeply.*
- **Structural concepts.** Bridges, articulation points, k-cores, rich-club coefficient, assortativity, structural holes (Burt), structural equivalence. *Working knowledge. Need to study Burt's constraint measure in more mathematical detail.*

**Study needs:** Spectral graph theory foundations (Laplacian matrix, Fiedler vector, spectral gap), the relationship between stochastic block models and community detection, and the formal properties of centrality measures in weighted and directed networks.

**Key resources:**
- Newman, *Networks: An Introduction* (2010, 2nd ed. 2018)
- Barabási, *Network Science* (2016)
- Wasserman & Faust, *Social Network Analysis: Methods and Applications* (1994)

**Timeline:** Deepen specific sub-areas by end of Month 3.

### 4.3 Data Structures for Graphs and Probabilistic Models

**Assessment: Level 3 — Working**

The internal representation of Lutufi's unified model requires efficient data structures:

- **Graph representations.** Adjacency list, adjacency matrix, compressed sparse row (CSR), edge lists. *Understand tradeoffs; have implemented adjacency lists and CSR formats.*
- **Factor representations.** Factor tables (multi-dimensional arrays indexed by variable assignments), log-space representation to avoid underflow, sparse factor representations for high-arity variables. *Working knowledge; need to study sparse representations more.*
- **Junction tree data structures.** Clique representation, separator sets, message caches. *Can implement basic versions; optimization for incremental updates needs study.*
- **Index structures.** Hash maps for variable lookups, tries for conditional probability table indexing, cache-friendly layouts for factor operations. *Working knowledge.*

**Study needs:** Cache-friendly data layouts for factor operations (this becomes critical at scale), efficient representations for factors with context-specific independence, and data structures for dynamic graphs (where edges and nodes change over time).

**Key resources:**
- Koller & Friedman, *Probabilistic Graphical Models* (2009), Chapter 4 (Undirected models) and Chapter 10 (Clique tree data structures)
- Darwiche, *Modeling and Reasoning with Bayesian Networks* (2009), Chapter 7

**Timeline:** Month 2–3, concurrent with inference algorithm deepening.

### 4.4 Parameter Learning

**Assessment: Level 3 — Working**

Learning the numerical parameters (conditional probability distributions) of a Bayesian network from data:

- **Maximum Likelihood Estimation (MLE).** Closed-form solutions for complete data, sufficient statistics, the decomposition of likelihood into local terms for each node given its parents. *Solid working knowledge.*
- **Bayesian estimation.** Dirichlet priors as conjugate priors for categorical distributions, the Bayesian-Dirichlet (BD) family, BDeu scores, the role of equivalent sample size. *Working knowledge; need to study the theoretical justification for BDeu's uniform prior assumptions.*
- **EM algorithm for latent variables.** The E-step as inference, the M-step as parameter update, convergence guarantees (monotonic increase of ELBO, convergence to local optima), initialization strategies. *Can implement basic EM. Need to study convergence rate analysis and acceleration techniques.*
- **Online and incremental learning.** Updating parameters as new data arrives without full retraining. *Conceptual understanding; need to study specific algorithms.*

**Study needs:** EM convergence theory in more depth, variational EM as a bridge to variational inference, structural EM for learning structure with latent variables, parameter tying and sharing across network regions.

**Key resources:**
- Koller & Friedman, *Probabilistic Graphical Models* (2009), Chapters 17–19
- McLachlan & Krishnan, *The EM Algorithm and Extensions* (2008)
- Heckerman, "A Tutorial on Learning with Bayesian Networks" (1995)

**Timeline:** Month 3–4 of Research Phase.

### 4.5 Structure Learning

**Assessment: Level 3 — Working**

Learning the graph structure of a Bayesian network from data:

- **Score-based methods.** BIC (Bayesian Information Criterion), BDeu (Bayesian Dirichlet equivalent uniform), the GES (Greedy Equivalence Search) algorithm by Chickering (2002). *Working knowledge. Can implement greedy hill-climbing with BIC scoring. GES implementation needs study.*
- **Constraint-based methods.** The PC algorithm (Spirtes, Glymour, Scheines, 1993/2000), conditional independence testing, the FCI algorithm for latent confounders. *Understand the PC algorithm at a working level. FCI is at the partial/conceptual level.*
- **Hybrid methods.** The MMHC (Max-Min Hill Climbing) algorithm by Tsamardinos, Brown, & Aliferis (2006). *Conceptual knowledge; have not implemented.*
- **Equivalence classes.** Markov equivalence, CPDAG (Completed Partially Directed Acyclic Graph) representation, the conditions under which two DAGs encode the same conditional independencies. *Working knowledge.*

**Study needs:** The GES algorithm's correctness proof and implementation details, the FCI algorithm for causal discovery with latent variables, recent advances in structure learning (NOTEARS by Zheng et al., 2018, and continuous optimization approaches), and structure learning for dynamic Bayesian networks.

**Key resources:**
- Spirtes, Glymour & Scheines, *Causation, Prediction, and Search* (2000)
- Chickering, "Optimal Structure Identification with Greedy Search" (2002)
- Koller & Friedman, *Probabilistic Graphical Models* (2009), Chapters 18–20

**Timeline:** Month 3–5.

---

## 5. Areas of Partial Knowledge

These are domains where the author has conceptual understanding — enough to know why the topic matters, how it fits into Lutufi's design, and what questions to ask — but where the knowledge is not yet sufficient for confident implementation or design decisions. These areas require structured study during the Research Phase.

### 5.1 Variational Inference

**Assessment: Level 2 — Conceptual**

Variational inference is essential for Lutufi because exact inference is intractable for large networks, and MCMC sampling can be too slow for interactive analysis. The author's current understanding:

- **What is known:** Variational inference reframes inference as optimization — finding a tractable distribution q(Z) that is close (in KL divergence) to the true posterior p(Z|X). The mean-field approximation assumes q factorizes over individual variables. The ELBO (Evidence Lower Bound) is the objective being maximized. The CAVI (Coordinate Ascent Variational Inference) algorithm iterates over variables, updating each q_i to minimize KL divergence while holding others fixed.
- **What is not yet solid:** The derivation of CAVI update equations for specific exponential family distributions, the connection between variational inference and expectation propagation, structured variational approximations (that respect some dependencies rather than assuming full factorization), the theory of when mean-field approximations are accurate vs. when they give poor results, and stochastic variational inference for large datasets.
- **The Lutufi relevance:** For networks with hundreds or thousands of nodes, exact junction tree inference may be infeasible. Variational inference provides a principled way to approximate posteriors with controllable accuracy-speed tradeoffs. Lutufi's design must support both exact and variational inference behind a unified API.

**Study plan:**
1. Work through Blei, Kucukelbir & McAuliffe, "Variational Inference: A Review for Statisticians" (2017) — the best modern review
2. Implement mean-field VI for a simple conjugate model (e.g., Gaussian mixture) from scratch
3. Study Wainwright & Jordan, *Graphical Models, Exponential Families, and Variational Inference* (2008) — the definitive technical treatment
4. Study belief propagation as a variational method (the Bethe approximation) and its connection to loopy BP
5. Study stochastic variational inference (Hoffman et al., 2013) for scalability

**Key resources:**
- Blei, Kucukelbir & McAuliffe, "Variational Inference: A Review for Statisticians" (2017)
- Wainwright & Jordan, *Graphical Models, Exponential Families, and Variational Inference* (2008)
- Bishop, *Pattern Recognition and Machine Learning* (2006), Chapter 10

**Timeline:** Month 2–4 of Research Phase, with hands-on implementation by Month 4.

### 5.2 Causal Inference and Do-Calculus

**Assessment: Level 2 — Conceptual**

Causal inference is a core differentiator for Lutufi — the ability to answer not just "What is the probability of Y given we observe X?" but "What would happen to Y if we *intervened* to set X to a specific value?" The author's current state:

- **What is known:** The distinction between observational, interventional, and counterfactual distributions. Pearl's do-operator do(X = x) and its meaning as "setting X to x by external intervention." The truncated factorization formula for computing interventional distributions. The three rules of do-calculus. The back-door criterion and front-door criterion as sufficient conditions for causal identification. Structural Causal Models (SCMs) as a formal framework. The causal hierarchy: association → intervention → counterfactual.
- **What is not yet solid:** Proving completeness of do-calculus (Huang & Valtorta, 2006; Shpitser & Pearl, 2006), implementing the ID algorithm for general causal identification, counterfactual computation in complex models, transportability theory, and the relationship between causal inference and missing data (Mohan & Pearl, 2021). The algorithmic implementation of causal reasoning — translating Pearl's theoretical framework into executable code — needs significant study.
- **The Lutufi relevance:** Lutufi's causal inference capability is one of its primary differentiators from existing network analysis tools. Researchers need to ask interventional questions: "What happens to the financial network if we bail out bank X?" or "What happens to information flow if we remove node Y from a social network?" These are inherently causal questions that require do-calculus, not just probabilistic conditioning.

**Study plan:**
1. Re-read Pearl, *Causality* (2009), Chapters 3–4, with worked exercises
2. Study the ID algorithm (Tian & Pearl, 2002; Shpitser & Pearl, 2006) in sufficient detail to implement it
3. Implement the back-door and front-door adjustments on simple causal models
4. Study counterfactual computation: the three-step procedure (abduction, action, prediction)
5. Read Peters, Janzing & Schölkopf, *Elements of Causal Inference* (2017) for a complementary perspective

**Key resources:**
- Pearl, *Causality: Models, Reasoning, and Inference* (2nd ed., 2009)
- Pearl, Glymour & Jewell, *Causal Inference in Statistics: A Primer* (2016)
- Peters, Janzing & Schölkopf, *Elements of Causal Inference* (2017)
- Shpitser & Pearl, "Identification of Joint Interventional Distributions" (2006)

**Timeline:** Month 2–5 of Research Phase. This is a critical path item.

### 5.3 Measure-Theoretic Probability and Continuous Distributions

**Assessment: Level 2 — Conceptual**

While discrete probability is a strength, the extension to continuous and measure-theoretic settings is partial:

- **What is known:** Probability density functions, cumulative distribution functions, common continuous distributions (Gaussian, exponential, gamma, beta), multivariate Gaussian distribution (mean vector, covariance matrix), the central limit theorem, basic convergence concepts (convergence in probability, convergence in distribution).
- **What is not yet solid:** σ-algebras and measurability in depth, Radon-Nikodym derivatives as the formal foundation for conditional density, regular conditional distributions, disintegration of measures, the rigorous treatment of conditioning on zero-probability events, and Lebesgue integration. For Lutufi: continuous Bayesian networks (linear Gaussian models, conditional linear Gaussian models), and inference in hybrid networks (mixtures of discrete and continuous variables).
- **The Lutufi relevance:** Many real-world network variables are continuous — financial exposures, trade volumes, signal strengths, temporal durations. Lutufi cannot be limited to discrete variables. Conditional linear Gaussian (CLG) models and hybrid inference methods require a solid understanding of continuous probability.

**Study plan:**
1. Study Billingsley, *Probability and Measure* (1995), Chapters 1–5, for rigorous foundations
2. Work through the continuous BN treatment in Koller & Friedman, Chapter 14
3. Implement linear Gaussian inference for a simple network
4. Study hybrid inference methods (Lauritzen & Jensen, 2001; Murphy, 2002)

**Key resources:**
- Billingsley, *Probability and Measure* (3rd ed., 1995)
- Koller & Friedman, *Probabilistic Graphical Models* (2009), Chapter 14
- Lauritzen & Jensen, "Stable Local Computation with Conditional Gaussian Distributions" (2001)

**Timeline:** Month 3–5. Not on the critical path for the initial discrete-variable prototype, but essential before v1.0.

### 5.4 MCMC Methods and Convergence Theory

**Assessment: Level 2 — Conceptual**

Markov Chain Monte Carlo methods are a key class of approximate inference algorithms:

- **What is known:** The basic idea of MCMC — constructing a Markov chain whose stationary distribution is the target posterior. The Metropolis-Hastings algorithm: proposal, acceptance ratio, the detailed balance condition. Gibbs sampling as a special case. The concept of burn-in, thinning, and chain diagnostics (trace plots, R-hat, effective sample size). Basic awareness of Hamiltonian Monte Carlo (HMC) and the No-U-Turn Sampler (NUTS).
- **What is not yet solid:** Formal convergence theory — mixing time bounds, spectral gap analysis, coupling arguments. The theory behind when MCMC is efficient vs. when it mixes slowly (multimodality, strong correlations, high dimensionality). Rao-Blackwellization for variance reduction. The implementation details of HMC (leapfrog integration, mass matrix adaptation). Collapsed Gibbs sampling. MCMC for graphical models specifically (the relationship between graph structure and Gibbs sampling efficiency).
- **The Lutufi relevance:** MCMC provides a flexible approximate inference method that can handle models where both exact inference and variational inference struggle. For small-to-medium networks with complex dependencies, MCMC may be the best option. Lutufi needs at least basic MCMC support (Gibbs sampling over network variables) and should support convergence diagnostics.

**Study plan:**
1. Robert & Casella, *Monte Carlo Statistical Methods* (2004), Chapters 6–8
2. Implement Gibbs sampling for a Bayesian network from scratch
3. Study MCMC convergence diagnostics in depth: Gelman & Rubin, effective sample size, Geweke diagnostics
4. Study the relationship between graph structure and MCMC mixing (block Gibbs, cut-set conditioning)
5. Implement basic HMC for a continuous model

**Key resources:**
- Robert & Casella, *Monte Carlo Statistical Methods* (2nd ed., 2004)
- Brooks et al., *Handbook of Markov Chain Monte Carlo* (2011)
- Neal, "MCMC using Hamiltonian Dynamics" (2011) in the Handbook

**Timeline:** Month 3–5.

### 5.5 Dynamic Bayesian Networks

**Assessment: Level 2 — Conceptual**

Dynamic Bayesian networks (DBNs) model temporal evolution — how a network's probabilistic state changes over time:

- **What is known:** The basic structure of a DBN as a "rolled out" Bayesian network where variables at time t depend on variables at time t-1 (and possibly earlier). The 2-time-slice Bayesian network (2TBN) representation. Hidden Markov Models (HMMs) as a special case. The forward-backward algorithm for HMMs. The conceptual extension to general DBNs via interface algorithms.
- **What is not yet solid:** The interface algorithm for exact inference in DBNs (Liang & Jordan), approximate inference in DBNs (particle filtering, sequential Monte Carlo), the Boyen-Koller algorithm for tractable approximate inference, structure learning for DBNs from time-series data, and the practical challenges of specifying transition models for social and economic networks.
- **The Lutufi relevance:** Social and economic networks are inherently temporal — relationships form and dissolve, financial exposures change, influence propagates over time. Lutufi must support temporal modeling. DBNs are the principled framework for combining Bayesian network semantics with temporal dynamics.

**Study plan:**
1. Murphy, *Machine Learning: A Probabilistic Perspective* (2012), Chapter 18
2. Study the interface algorithm and the 1.5-slice representation
3. Implement the forward-backward algorithm for HMMs from scratch
4. Study particle filtering and sequential Monte Carlo: Doucet, De Freitas & Gordon (2001)
5. Study the Boyen-Koller algorithm for approximate DBN inference

**Key resources:**
- Murphy, *Machine Learning: A Probabilistic Perspective* (2012), Chapter 18
- Koller & Friedman, *Probabilistic Graphical Models* (2009), Chapter 6
- Doucet, De Freitas & Gordon, *Sequential Monte Carlo Methods in Practice* (2001)

**Timeline:** Month 4–6. Not on the critical path for Phase 1 but must be solid before Phase 3.

### 5.6 Missing Data Theory

**Assessment: Level 2 — Conceptual**

Real-world network data is always incomplete:

- **What is known:** Rubin's taxonomy — MCAR (Missing Completely At Random), MAR (Missing At Random), MNAR (Missing Not At Random). The ignorability condition: if data is MAR and the missingness mechanism's parameters are distinct from the data model's parameters, the missingness mechanism can be ignored for likelihood-based inference. The EM algorithm as a principled approach to MAR data. Multiple imputation as an alternative. The idea that network data has special missingness patterns (e.g., an unobserved node removes all its incident edges).
- **What is not yet solid:** The formal statistical theory of ignorability and its proofs. MNAR modeling — when the missingness depends on the missing values themselves (e.g., high-value financial transactions are more likely to be hidden). Network-specific missing data: Kossinets (2006) on missing data in social networks, the Hoff-Raftery-Handcock approach to latent space models, and the formal treatment of partially observed networks. Sensitivity analysis for MNAR assumptions. The connection between missingness and causal inference (Mohan & Pearl, 2021, "Graphical Models for Processing Missing Data").
- **The Lutufi relevance:** Missing data is a first-class concern for Lutufi. Intelligence analysts work with covert networks where most of the network is unobserved. Financial regulators work with incomplete exposure data. Social scientists work with survey data that is never complete. Lutufi must handle missing data principally, not as a preprocessing step but as an integral part of inference.

**Study plan:**
1. Little & Rubin, *Statistical Analysis with Missing Data* (2002), Chapters 1–7
2. Study Mohan & Pearl (2021) on graphical models for missing data
3. Implement EM-based inference for a BN with incomplete data
4. Study network-specific missing data: Kossinets (2006), Handcock & Gile (2010)
5. Develop a framework for missingness patterns in Lutufi's data model

**Key resources:**
- Little & Rubin, *Statistical Analysis with Missing Data* (2nd ed., 2002)
- Mohan & Pearl, "Graphical Models for Processing Missing Data" (2021)
- Kossinets, "Effects of Missing Data in Social Networks" (2006)

**Timeline:** Month 3–5.

---

## 6. Areas of Acknowledged Gaps

These are domains where the author's current knowledge is insufficient for even conceptual design decisions. Honest acknowledgment of these gaps is essential. For each gap, the key question is: *Is this gap blocking for the project, and if so, when must it be closed?*

### 6.1 Advanced Numerical Optimization

**Assessment: Level 1 — Gap**

- **What is known:** Basic gradient descent, stochastic gradient descent, the concept of convexity and local optima, Newton's method in one dimension.
- **What is not known:** Modern optimization methods in depth — L-BFGS, conjugate gradient methods, proximal gradient methods, mirror descent. The theory of convergence rates for different optimization algorithms. Constrained optimization (Lagrange multipliers beyond simple cases, KKT conditions). Numerical stability considerations for large-scale optimization. Automatic differentiation (forward mode, reverse mode, the relationship to backpropagation).
- **The Lutufi relevance:** Many Lutufi algorithms are fundamentally optimization problems. Variational inference optimizes the ELBO. Parameter learning via MLE optimizes the likelihood. Structure learning optimizes a score function. If the optimization is implemented naively, Lutufi will be slow and potentially numerically unstable. However, for the initial implementation, standard methods (EM for parameter learning, greedy search for structure learning) may suffice. Advanced optimization becomes critical for scalability in Phase 4.
- **Blocking assessment:** **Not blocking for Phase 1.** Becomes important in Phase 2 (variational inference) and critical in Phase 4 (scalability).

**Study plan:**
1. Nocedal & Wright, *Numerical Optimization* (2006), Chapters 1–7
2. Boyd & Vandenberghe, *Convex Optimization* (2004), selected chapters
3. Study automatic differentiation: Baydin et al., "Automatic Differentiation in Machine Learning: A Survey" (2018)

**Timeline:** Month 4–8, with the basic material needed by Month 4 for variational inference.

### 6.2 Distributed and Parallel Computing

**Assessment: Level 1 — Gap**

- **What is known:** Basic threading and multiprocessing in Python. The concept of MapReduce. Awareness that graph algorithms can be parallelized (e.g., parallel BFS) and that inference can be parallelized across independent subgraphs.
- **What is not known:** Distributed computing frameworks (Spark, Dask) in depth. GPU computing (CUDA, OpenCL) and its application to graph algorithms and probabilistic inference. Message passing interfaces (MPI). The theory and practice of distributed graph processing (Pregel, GraphX, PowerGraph). How to design algorithms that scale across multiple machines.
- **The Lutufi relevance:** For networks with millions of nodes (large social networks, national-scale financial networks), single-machine computation will be insufficient. Distributed inference is necessary for Lutufi to handle real-world scale. However, the initial target is single-machine computation on networks of up to ~10,000 nodes. Distributed computing is a Phase 4+ concern.
- **Blocking assessment:** **Not blocking for Phases 1–3.** Becomes relevant in Phase 4 (scalability).

**Study plan:**
1. Study Dask and its graph scheduler for single-machine parallelism
2. Study GPU computing for graph algorithms: Merrill et al., sparse linear algebra on GPUs
3. Study distributed graph processing: Pregel model, GraphX
4. Consider whether Lutufi's inference algorithms can be naturally parallelized

**Timeline:** Month 8–12, well after core functionality is implemented.

### 6.3 Formal Software Verification

**Assessment: Level 1 — Gap**

- **What is known:** The concept of formal verification — proving that software satisfies a specification. Awareness of tools like Coq, Isabelle, TLA+. The distinction between testing (showing the absence of specific bugs) and verification (proving correctness for all inputs).
- **What is not known:** How to use formal verification tools in practice. The theory of program correctness proofs. How to specify mathematical properties (e.g., "this inference algorithm produces the correct marginal distribution") in a verifiable way. What aspects of Lutufi, if any, are amenable to formal verification.
- **The Lutufi relevance:** Probabilistic inference algorithms are notoriously difficult to test because the "correct" answer is often unknown for complex models. Formal verification could provide much stronger guarantees than testing alone. However, formal verification of numerical algorithms is an active research area with limited practical tools. This is aspirational rather than practical for the current project phase.
- **Blocking assessment:** **Not blocking.** This is a long-term research interest, not a near-term need.

**Study plan:** Defer to Phase 5+ or a future project. Monitor developments in verified probabilistic programming (e.g., verified implementations of probabilistic algorithms in Lean 4 or Coq).

**Timeline:** Not scheduled. Revisit at Phase 5.

### 6.4 Domain-Specific Expertise (Intelligence Analysis, Epidemiology, Political Science)

**Assessment: Level 1 — Gap (varying by domain)**

Lutufi targets several application domains. While the author has general knowledge of these domains, deep domain expertise is lacking:

- **Intelligence analysis.** Understanding of link analysis and social network analysis for counter-terrorism at a high level. No experience with actual intelligence workflows, classified data handling, or the specific analytical frameworks used by intelligence agencies (e.g., Analysis of Competing Hypotheses, Structured Analytic Techniques).
- **Epidemiology.** Basic understanding of SIR/SIS models, contact tracing networks, R₀. No experience with epidemiological data sources, reporting standards, or the specific statistical methods used in epidemiological research.
- **Political science.** Awareness of policy diffusion networks, legislative cosponsorship networks, international relations networks. No deep understanding of political science methodology or the specific debates in the field.
- **Financial regulation.** Conceptual understanding of systemic risk, stress testing, macro-prudential regulation. No experience with actual regulatory frameworks, reporting standards, or supervisory data.

**The Lutufi relevance:** The example library (30+ domain-specific case studies) requires at least working knowledge of each application domain. The examples must be credible to domain experts, not just technically correct. This means the author must either acquire sufficient domain knowledge or collaborate with domain experts.

**Blocking assessment:** **Not blocking for core library implementation.** Becomes important in Phase 3 (example library) and Phase 5 (publication and community).

**Study plan:**
1. For each target domain, identify 2–3 key introductory texts and review papers
2. Identify potential domain expert collaborators or reviewers for each domain
3. Build example case studies iteratively, soliciting domain expert feedback
4. Prioritize domains: financial networks and social influence first (most directly relevant to Lutufi's value proposition)

**Timeline:** Month 6–12+ (ongoing throughout development).

### 6.5 Rust or C++ Systems Programming (for Core Engine)

**Assessment: Level 1 — Gap (Rust); Level 2 — Conceptual (C++)**

The Lutufi design calls for a high-performance core engine in a systems language with Python bindings:

- **Rust.** Awareness of Rust's ownership model, borrowing, lifetimes, and the safety guarantees they provide. Have completed introductory tutorials but not built anything substantial. Understanding of PyO3 for Python bindings at a high level.
- **C++.** Undergraduate-level knowledge (several years ago). Templates, STL containers, basic memory management. Would need significant refreshing for modern C++ (C++17/20 features, smart pointers, move semantics). Understanding of pybind11 for Python bindings at a conceptual level.
- **The decision between Rust and C++** is itself a design decision that depends on understanding both languages' ecosystems, performance characteristics, and Python binding story. The language decision document is scheduled for the Research Phase.

**The Lutufi relevance:** Performance is important for Lutufi because inference algorithms on large networks are computationally expensive. Python alone may not provide adequate performance for the core inference engine. A systems language core with Python bindings is the standard approach in the scientific computing world (NumPy/C, PyTorch/C++, polars/Rust).

**Blocking assessment:** **Blocking for Phase 1 implementation.** The language decision must be made during the Research Phase, and sufficient proficiency must be acquired before implementation begins.

**Study plan:**
1. Complete the Rust Book (Klabnik & Nichols) and build a small graph library in Rust as practice
2. Study PyO3 and the maturin build system for Rust-Python bindings
3. Build a small prototype of factor operations in both Rust and C++ to compare
4. Make the language decision by end of Month 4
5. Deep-dive into the chosen language for Months 5–6

**Key resources:**
- Klabnik & Nichols, *The Rust Programming Language* (2023)
- PyO3 documentation: https://pyo3.rs
- If C++: Stroustrup, *The C++ Programming Language* (4th ed.) and pybind11 documentation

**Timeline:** Month 1–6. Language decision by Month 4. Proficiency sufficient for Phase 1 by Month 6.

### 6.6 Information-Theoretic Methods for Model Selection

**Assessment: Level 1 — Gap**

- **What is known:** AIC and BIC as model selection criteria. The basic idea behind minimum description length (MDL). The concept of cross-validation for model selection.
- **What is not known:** The formal derivation and justification of BIC in the Bayesian framework (how it approximates the marginal likelihood). MDL theory in depth. The connection between information-theoretic model selection and Bayesian model comparison. Bayes factors and their computation. Reversible-jump MCMC for trans-dimensional model comparison. The practical challenges of model selection for probabilistic graphical models — comparing models with different graph structures, different numbers of latent variables, and different parameterizations.
- **The Lutufi relevance:** Lutufi must support model selection: given network data, which graphical model structure best explains the data? This requires well-motivated scoring functions. BIC and BDeu are the standard choices, but understanding *why* they work and *when* they break is essential for exposing the right API and defaults.

**Blocking assessment:** **Partially blocking for Phase 2** (structure learning).

**Study plan:**
1. Burnham & Anderson, *Model Selection and Multimodel Inference* (2002), Chapters 1–4
2. Grünwald, *The Minimum Description Length Principle* (2007), selected chapters
3. Study Bayesian model comparison for graphical models: Heckerman et al. (1995), Chickering (2002)

**Timeline:** Month 4–6.

---

## 7. Knowledge Acquisition Strategy

### 7.1 Principles

The following principles guide the study plan:

1. **Depth before breadth.** It is better to deeply understand the core algorithms that Lutufi depends on than to have shallow knowledge of many peripheral topics. The critical path items (exact inference, structure learning, causal inference) take priority.

2. **Implementation as learning.** The most effective way to deeply understand an algorithm is to implement it from scratch, test it against known results, and debug the inevitable mistakes. For every key algorithm, the study plan includes an implementation exercise.

3. **Primary sources.** Key algorithms and theoretical results should be studied from the original papers or from textbooks that present them rigorously, not from blog posts or tutorials alone. Blog posts are useful for initial orientation but insufficient for the depth needed to implement correctly.

4. **Spaced repetition.** Key definitions, theorems, and algorithmic steps should be added to a spaced repetition system (Anki or similar) to prevent knowledge from decaying. This is especially important for mathematical results that are not used daily.

5. **Teach to learn.** Writing documentation (like the foundation documents in `docs/foundations/`) serves as a forcing function for understanding. If something cannot be explained clearly, it is not yet understood deeply enough.

6. **Identify the minimum viable knowledge.** For each gap, determine what level of knowledge is needed for the current phase and what can be deferred. Not every gap needs to be fully closed before implementation begins — but every gap that affects the current phase's design decisions must be addressed.

### 7.2 Study Schedule Overview

| Month | Primary Focus | Secondary Focus |
|---|---|---|
| **Month 1** | Foundation documents writing (this audit, BN foundations, social/economic network foundations) | Rust language study begins |
| **Month 2** | Exact inference algorithms (junction tree, variable elimination) — deepen to Level 4 | Variational inference — begin study |
| **Month 3** | Structure learning, parameter learning, network science metrics — deepen to Level 4 | Causal inference — deep study begins |
| **Month 4** | Causal inference (do-calculus, ID algorithm) — target Level 3+ | Systems language decision and deep-dive |
| **Month 5** | MCMC, missing data theory, DBNs — target Level 3 | Numerical optimization basics |
| **Month 6** | Integration and review: verify all critical-path topics at Level 3+ | Domain-specific study begins |

### 7.3 Core Reference Library

The following texts form the core reference library for the Research Phase. Each will be studied systematically, not just consulted as references:

| Text | Primary Topics | Priority |
|---|---|---|
| Koller & Friedman, *Probabilistic Graphical Models* (2009) | BNs, MRFs, inference, learning | **Critical** — the primary reference for most of Lutufi |
| Pearl, *Causality* (2nd ed., 2009) | Causal inference, do-calculus, SCMs | **Critical** — the foundation for Lutufi's causal features |
| Newman, *Networks* (2nd ed., 2018) | Network science, metrics, models | **Critical** — the primary network science reference |
| Darwiche, *Modeling and Reasoning with Bayesian Networks* (2009) | BN algorithms, compilation methods | **High** — complementary to Koller & Friedman |
| Jackson, *Social and Economic Networks* (2008) | Economic networks, game theory on networks | **High** — the primary economic networks reference |
| Wasserman & Faust, *Social Network Analysis* (1994) | Social network analysis methods | **High** — the classic SNA reference |
| Bishop, *Pattern Recognition and Machine Learning* (2006) | Variational inference, EM, graphical models | **Medium** — excellent for inference topics |
| Robert & Casella, *Monte Carlo Statistical Methods* (2004) | MCMC theory and practice | **Medium** — primary MCMC reference |
| Spirtes, Glymour & Scheines, *Causation, Prediction, and Search* (2000) | Causal discovery algorithms | **Medium** — essential for structure learning |
| Little & Rubin, *Statistical Analysis with Missing Data* (2002) | Missing data theory | **Medium** — essential for robustness features |

---

## 8. Study Plans by Area

### 8.1 Critical Path Items (Must reach Level 3+ before Phase 1)

**Exact Inference (Junction Tree Algorithm)**
- Week 1–2: Re-derive variable elimination from scratch. Implement on a 5-node network. Verify against pgmpy.
- Week 3–4: Study moralization, triangulation, and clique tree construction. Implement each step.
- Week 5–6: Implement full junction tree inference (Hugin architecture). Test on networks of increasing size.
- Week 7–8: Study optimization: elimination ordering heuristics, incremental updates, Shenoy-Shafer architecture.
- Milestone: Independently implement junction tree inference that produces correct marginals on all test cases.

**Causal Inference (Do-Calculus)**
- Week 1–2: Re-read Pearl (2009), Chapters 1–3. Work through all examples.
- Week 3–4: Study the back-door criterion. Implement back-door adjustment on simple models.
- Week 5–6: Study the front-door criterion and the three rules of do-calculus. Work through proofs.
- Week 7–8: Study the ID algorithm. Implement a basic version.
- Week 9–10: Study counterfactual computation. Implement the three-step procedure.
- Milestone: Correctly compute interventional and counterfactual quantities on standard textbook examples.

**Structure Learning**
- Week 1–2: Implement BIC scoring and greedy hill-climbing search.
- Week 3–4: Study the PC algorithm. Implement conditional independence testing and skeleton recovery.
- Week 5–6: Study GES. Understand the proof of correctness and implement.
- Week 7–8: Study equivalence classes and CPDAG representation.
- Milestone: Recover known network structures from synthetic data using both score-based and constraint-based methods.

### 8.2 Important but Non-Blocking Items (Target Level 3 before Phase 2)

**Variational Inference**
- Study CAVI for conjugate models. Implement for Gaussian mixture.
- Study mean-field approximation for BNs.
- Study the Bethe approximation and its connection to loopy belief propagation.
- Study stochastic variational inference.
- Milestone: Implement mean-field VI for a Bayesian network and compare results to exact inference.

**MCMC Methods**
- Implement Gibbs sampling for a Bayesian network.
- Study convergence diagnostics and implement R-hat, effective sample size, trace plots.
- Study block Gibbs and collapsed Gibbs sampling.
- Implement basic HMC for continuous models.
- Milestone: Inference results from MCMC match exact inference results (within statistical tolerance) on test networks.

**Missing Data**
- Study Little & Rubin, Chapters 1–5.
- Implement EM for a BN with missing data.
- Study network-specific missing data patterns.
- Design Lutufi's missing data API.
- Milestone: Correctly learn parameters of a BN from incomplete data using EM, with results matching the complete-data MLE in expectation.

### 8.3 Longer-Term Items (Target Level 2+ before Phase 3)

**Dynamic Bayesian Networks**
- Implement forward-backward algorithm for HMMs.
- Study the interface algorithm for general DBNs.
- Study particle filtering and sequential Monte Carlo.
- Milestone: Implement DBN inference for a simple temporal model.

**Advanced Optimization**
- Study gradient-based optimization methods beyond basic SGD.
- Understand automatic differentiation at the implementation level.
- Milestone: Implement gradient-based optimization for variational inference.

**Systems Language Proficiency**
- Complete the Rust Book or refresh modern C++.
- Build a factor operations library in the chosen language.
- Build Python bindings for the factor library.
- Milestone: A working Python-callable library that performs factor operations at 10x+ the speed of pure Python.

---

## 9. Accountability Mechanism

### 9.1 Progress Tracking

Knowledge acquisition will be tracked through multiple concrete mechanisms:

**Weekly study log.** A simple log recording:
- Hours spent studying each topic
- Specific chapters/papers read
- Implementation exercises completed
- Concepts added to spaced repetition system
- Questions or confusions that arose

**Monthly self-re-assessment.** At the end of each month, revisit this audit and re-assess each knowledge area. The re-assessment must be concrete:
- What specific sub-topics have moved up a level?
- What new gaps have been discovered?
- Is the study schedule on track, behind, or ahead?
- What adjustments are needed?

**Implementation milestones.** Each study plan includes a concrete implementation milestone. The milestone is the primary evidence of knowledge acquisition — if the code works correctly on test cases, the understanding is sufficient. If it doesn't, the understanding has gaps that must be identified and addressed.

**Documentation as evidence.** Each foundation document in `docs/foundations/` serves as evidence of understanding. A document that explains a topic clearly, with correct examples and appropriate nuance, demonstrates Level 3+ knowledge. A document that contains errors or hand-wavy explanations reveals gaps.

### 9.2 Failure Modes and Mitigations

| Failure Mode | Detection | Mitigation |
|---|---|---|
| **Spending too long on one topic at the expense of others** | Weekly study log shows disproportionate time allocation | Enforce time-boxed study sessions; move on when diminishing returns are reached |
| **Confusing recognition with generation** | Implementation milestone fails despite feeling like the topic is understood | Implement from scratch without references; identify the specific sub-step that is unclear |
| **Study without retention** | Monthly re-assessment reveals knowledge has decayed | Add key concepts to spaced repetition; implement more exercises |
| **Avoiding difficult topics** | Study log shows consistent avoidance of certain areas | Acknowledge avoidance explicitly; schedule the difficult topic first in the next week |
| **Perfectionism blocking progress** | No Phase 1 code written by Month 7 because "not ready yet" | Define concrete "good enough" criteria for each gap; accept that some learning will happen during implementation |
| **Discovered gap is larger than expected** | Monthly re-assessment reveals a Level 1 gap in a critical-path area | Adjust timeline, seek external help (collaborator, mentor, or consultant), or redesign the affected feature |

### 9.3 External Accountability

- **Public documentation.** All foundation documents are committed to the public repository, making the author's claimed understanding subject to community scrutiny.
- **Peer review.** Where possible, foundation documents and implementation milestones should be reviewed by knowledgeable peers — other researchers, developers, or domain experts.
- **Conference presentations.** Presenting Lutufi's design at academic workshops or meetups (even informally) creates external pressure to ensure the claimed knowledge is genuine.
- **Issue tracking.** Knowledge gaps that affect design decisions should be logged as GitHub issues, creating a transparent record of what was uncertain and how it was resolved.

### 9.4 Revision Schedule

This audit will be formally revised at the following points:

| Date | Revision Focus |
|---|---|
| **End of Month 2** | First major revision. Update all assessments. Identify any critical-path items still at Level 2. |
| **End of Month 4** | Mid-phase revision. Systems language decision should be made. Causal inference should be at Level 3+. |
| **End of Month 6** | End-of-Research-Phase revision. All critical-path items should be at Level 3+. Phase 1 readiness assessment. |
| **End of Phase 1** | Post-implementation revision. Implementation experience will have revealed new gaps. Major update expected. |
| **Before each subsequent phase** | Phase-entry revision. Assess readiness for the next phase's requirements. |

---

## 10. Audit Revision History

| Version | Date | Changes |
|---|---|---|
| 1.0 | March 2026 | Initial audit. All assessments reflect the author's honest self-evaluation at the start of the Research Phase. |

---

*This document is a commitment to intellectual honesty. Building a library that claims to unify probabilistic reasoning with network analysis requires that the author genuinely understand both domains — not just well enough to write code that compiles, but well enough to write code that produces correct results for the right reasons. This audit is the first step in that commitment.*

---

*"Trust in Allah, then tie your camel." — Prophet Muhammad (ﷺ)*
