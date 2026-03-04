# Research Roadmap: Lutufi

---

**Document Version:** 1.0
**Status:** Working Draft — Research Phase
**Author:** Wasswa Lutufi Sebbanja
**Last Updated:** March 2026
**License:** Apache 2.0

---

## Table of Contents

1. [Purpose](#1-purpose)
2. [Phase 1: Pre-Development Research](#2-phase-1-pre-development-research)
3. [Phase 2: During-Development Research](#3-phase-2-during-development-research)
4. [Phase 3: Post-Launch Research](#4-phase-3-post-launch-research)
5. [Research Dependencies](#5-research-dependencies)
6. [Estimated Time Allocations](#6-estimated-time-allocations)

---

## 1. Purpose

This document maps every research topic that must be studied before, during, and after Lutufi's development. Each topic includes what to study, key references, expected outputs, and how the research directly informs Lutufi's design. The goal is threefold: to prevent the author from starting to code before the intellectual foundations are solid, to prevent unnecessary deep rabbit holes during development, and to identify research that can proceed in parallel with or after implementation.

Research is organized by phase and by topic area within each phase. Dependencies between topics are explicitly noted so that the order of study can be optimized.

---

## 2. Phase 1: Pre-Development Research

Phase 1 research must be completed before any code is written. These topics form the intellectual foundation on which every design decision rests. Attempting to code without completing this research would result in an architecture that must be rebuilt once the implications of the theory are understood.

**Estimated Duration:** 6 months (Months 1–6)

---

### 2.1 Probability Theory and Measure Theory

**What to Study:** Axioms of probability (Kolmogorov). Conditional probability and Bayes' theorem at a rigorous level. Sigma-algebras and measurable spaces (sufficient to understand continuous distributions on graphs). Law of large numbers and central limit theorem as they apply to network statistics. Concentration inequalities relevant to sampling-based inference.

**Key References:**
- Billingsley, P. (2012). *Probability and Measure* (Anniversary ed.). Wiley.
- Grimmett, G. R., & Stirzaker, D. R. (2020). *Probability and Random Processes* (4th ed.). Oxford University Press.
- Casella, G., & Berger, R. L. (2002). *Statistical Inference* (2nd ed.). Cengage Learning.

**Expected Output:** A foundation document summarizing the probability theory relevant to Lutufi, with worked derivations showing how each concept applies to network contexts. Specific attention to where continuous and discrete formulations diverge and what this means for Lutufi's data model.

**How It Informs Design:** Determines whether Lutufi's internal probability representation uses discrete tables, continuous densities, or a hybrid approach. Informs the numerical precision requirements of the inference engine. Establishes the mathematical vocabulary used throughout all subsequent documents.

---

### 2.2 Bayesian Networks and Directed Graphical Models

**What to Study:** Formal definition of Bayesian networks (DAGs with conditional probability distributions). Factorization theorem. Conditional independence and d-separation. Exact inference: variable elimination, junction tree algorithm, bucket elimination. Model equivalence classes and Markov equivalence. Parameter learning (MLE, MAP, Bayesian estimation). Structure learning (constraint-based, score-based, hybrid methods). The historical development from Pearl's original formulation through modern extensions.

**Key References:**
- Pearl, J. (1988). *Probabilistic Reasoning in Intelligent Systems.* Morgan Kaufmann.
- Koller, D., & Friedman, N. (2009). *Probabilistic Graphical Models.* MIT Press. (Chapters 3–9, 16–18)
- Darwiche, A. (2009). *Modeling and Reasoning with Bayesian Networks.* Cambridge University Press.
- Lauritzen, S. L., & Spiegelhalter, D. J. (1988). Local computations with probabilities on graphical structures. *JRSS-B.*

**Expected Output:** A comprehensive document detailing Bayesian networks — their mathematical foundations, inference algorithms with complexity analysis, learning methods, and known limitations. Worked examples showing inference on small networks with hand-calculated results for test validation.

**How It Informs Design:** Directly determines the inference engine architecture. The choice and implementation of exact inference algorithms, the data structures for conditional probability tables, and the API for specifying directed models all flow from this research. The hand-calculated examples become ground truth for the test suite.

---

### 2.3 Markov Random Fields and Undirected Graphical Models

**What to Study:** Formal definition of Markov random fields (undirected graphs with potential functions). Hammersley-Clifford theorem. Global, local, and pairwise Markov properties. Factor graphs as a unifying representation. Conversion between directed and undirected models (moralization). Inference: belief propagation on factor graphs, junction tree for undirected models. The relationship between MRFs and Ising/Potts models in statistical physics.

**Key References:**
- Koller, D., & Friedman, N. (2009). *Probabilistic Graphical Models.* MIT Press. (Chapters 4, 10–11)
- Wainwright, M. J., & Jordan, M. I. (2008). Graphical models, exponential families, and variational inference. *Foundations and Trends in Machine Learning.*
- Kindermann, R., & Snell, J. L. (1980). *Markov Random Fields and Their Applications.* AMS.

**Expected Output:** A document explaining how undirected graphical models work and how they relate to Bayesian networks, with specific attention to why MRFs are necessary for modeling social networks (which are typically undirected and cyclic). Includes worked examples and decision criteria for when to use MRFs versus Bayesian networks.

**How It Informs Design:** Determines how Lutufi represents undirected social relationships probabilistically. Informs the factor graph implementation that serves as Lutufi's internal unifying representation. Determines the moralization strategy for converting between directed and undirected formulations.

---

### 2.4 Belief Propagation — Exact, Loopy, and Variational

**What to Study:** Message-passing on trees (exact). Sum-product and max-product algorithms. Extension to loopy graphs (loopy belief propagation): convergence conditions, convergence failures, damping strategies. Connection to Bethe free energy. Variational inference: mean-field approximation, expectation propagation. Comparison of approximation quality across methods. Computational complexity of each approach.

**Key References:**
- Yedidia, J. S., Freeman, W. T., & Weiss, Y. (2003). Understanding belief propagation and its generalizations. In *Exploring AI in the New Millennium.*
- Minka, T. P. (2001). Expectation propagation for approximate Bayesian inference. *UAI.*
- Wainwright, M. J., & Jordan, M. I. (2008). Graphical models, exponential families, and variational inference.
- Murphy, K. P., Weiss, Y., & Jordan, M. I. (1999). Loopy belief propagation for approximate inference. *UAI.*

**Expected Output:** A document covering all major variants of belief propagation with convergence analysis, worked examples on small graphs, and a decision framework for selecting the appropriate method given network properties (size, density, cycle structure). Documents specific failure modes and how Lutufi should handle them.

**How It Informs Design:** Directly determines the implementation of Lutufi's approximate inference engine. The convergence detection mechanisms, damping strategies, and method selection heuristics all flow from this research. Error messages for non-convergence can reference the specific mathematical conditions.

---

### 2.5 Causal Inference and Structural Causal Models

**What to Study:** Structural causal models (SCMs). The do-operator and interventional distributions. The three rules of do-calculus. Identifiability of causal effects. Counterfactual reasoning. The front-door and back-door criteria. Instrumental variables in network contexts. The distinction between causal and correlational claims in network science. The Rubin potential outcomes framework and its relationship to the Pearl framework.

**Key References:**
- Pearl, J. (2009). *Causality: Models, Reasoning, and Inference* (2nd ed.). Cambridge University Press.
- Pearl, J., Glymour, M., & Jewell, N. P. (2016). *Causal Inference in Statistics: A Primer.* Wiley.
- Peters, J., Janzing, D., & Schölkopf, B. (2017). *Elements of Causal Inference.* MIT Press.
- Imbens, G. W., & Rubin, D. B. (2015). *Causal Inference for Statistics, Social, and Biomedical Sciences.* Cambridge University Press.

**Expected Output:** A document on causal inference theory with specific attention to how do-calculus applies to network structures. Worked examples of causal queries over social networks. Identification of the algorithmic requirements for implementing do-calculus in Lutufi.

**How It Informs Design:** Determines the API for causal queries in Lutufi. Informs the internal representation of interventions (node clamping, edge removal). Defines the identifiability checks that Lutufi must perform before returning causal estimates. Shapes the documentation that explains to users the difference between observational and interventional network queries.

---

### 2.6 Dynamic Bayesian Networks

**What to Study:** Formal definition of dynamic Bayesian networks (DBNs). Two-timeslice Bayesian networks (2TBNs). Unrolling and interface variables. Inference in DBNs: exact (junction tree on unrolled networks) and approximate (particle filtering, online variational inference). Learning DBN structure from temporal data. The relationship between DBNs and hidden Markov models. Computational cost of temporal inference and when approximations become necessary.

**Key References:**
- Murphy, K. P. (2002). *Dynamic Bayesian Networks: Representation, Inference and Learning.* PhD thesis, UC Berkeley.
- Dean, T., & Kanazawa, K. (1989). A model for reasoning about persistence and causation. *Computational Intelligence.*
- Ghahramani, Z. (1997). Learning dynamic Bayesian networks. *Adaptive Processing of Sequences and Data Structures.*
- Koller, D., & Friedman, N. (2009). *Probabilistic Graphical Models.* (Chapter 6)

**Expected Output:** A document on DBN theory with worked examples of temporal network evolution. Decision criteria for when exact temporal inference is feasible versus when approximate methods are needed. Analysis of how DBNs handle the specific temporal dynamics of social and economic networks (edge formation/dissolution, belief evolution, contagion progression).

**How It Informs Design:** Directly determines the architecture of Lutufi's dynamics layer. Informs whether time-stepping uses unrolled representations, filtering approaches, or hybrid strategies. Determines the API for specifying temporal transitions and querying temporal posteriors.

---

### 2.7 Social Network Analysis Foundations

**What to Study:** Classical social network analysis: centrality measures (degree, betweenness, closeness, eigenvector, Katz), community detection algorithms, structural equivalence, structural holes, tie strength (Granovetter), homophily, transitivity and clustering. Network formation models: preferential attachment, triadic closure, strategic formation. Social influence versus social selection. Ego networks versus complete networks. Methodological challenges: boundary specification, missing data, network measurement error.

**Key References:**
- Wasserman, S., & Faust, K. (1994). *Social Network Analysis: Methods and Applications.* Cambridge University Press.
- Scott, J. (2017). *Social Network Analysis* (4th ed.). SAGE.
- Borgatti, S. P., Everett, M. G., & Johnson, J. C. (2018). *Analyzing Social Networks* (2nd ed.). SAGE.
- Granovetter, M. S. (1973). The strength of weak ties. *American Journal of Sociology.*
- Burt, R. S. (1992). *Structural Holes.* Harvard University Press.

**Expected Output:** A document covering the core concepts, metrics, and methods of social network analysis, with specific attention to how each concept creates opportunities or constraints for probabilistic modeling. A mapping from social network concepts to probabilistic formulations (e.g., centrality as a prior, homophily as a conditional dependency structure).

**How It Informs Design:** Determines which network metrics Lutufi should compute natively (versus delegating to NetworkX). Informs the API vocabulary — node and edge attributes, community structures, tie types. Identifies the social network properties that have natural probabilistic interpretations and should be supported as first-class features.

---

### 2.8 Economic Network Theory

**What to Study:** Interbank lending networks and systemic risk. Financial contagion mechanisms: counterparty risk, fire sales, information contagion. DebtRank and cascade models. Core-periphery structure in financial networks. Supply chain network models. Trade network analysis. Labor market networks. Network formation in economic contexts: strategic versus random. Game theory on networks. Auction and bargaining on networks.

**Key References:**
- Jackson, M. O. (2008). *Social and Economic Networks.* Princeton University Press.
- Acemoglu, D., Ozdaglar, A., & Tahbaz-Salehi, A. (2015). Systemic risk and stability in financial networks. *American Economic Review.*
- Elliott, M., Golub, B., & Jackson, M. O. (2014). Financial networks and contagion. *American Economic Review.*
- Battiston, S., et al. (2012). DebtRank: Too central to fail? *Scientific Reports.*
- Goyal, S. (2007). *Connections: An Introduction to the Economics of Networks.* Princeton University Press.

**Expected Output:** A document on economic network theory with specific focus on how economic networks differ from social networks (directed flows, quantitative weights, regulatory constraints) and what this implies for probabilistic modeling. Analysis of systemic risk models as candidate Lutufi use cases.

**How It Informs Design:** Determines how Lutufi represents weighted, directed financial flows. Informs the data model for economic quantities (exposures, transaction volumes, asset values) as probabilistic variables. Identifies the specific inference queries that financial regulators need (cascade probabilities, exposure distributions, intervention effects).

---

### 2.9 Random Graph Models

**What to Study:** Erdős–Rényi random graphs. Barabási–Albert preferential attachment model. Watts–Strogatz small-world model. Configuration models. Stochastic block models. Exponential random graph models (ERGMs). Latent space models. When each model is appropriate. How random graph models serve as null models for social network analysis. Computational challenges of fitting ERGMs.

**Key References:**
- Erdős, P., & Rényi, A. (1959). On random graphs. *Publicationes Mathematicae.*
- Barabási, A.-L., & Albert, R. (1999). Emergence of scaling in random networks. *Science.*
- Watts, D. J., & Strogatz, S. H. (1998). Small-world networks. *Nature.*
- Robins, G., Pattison, P., Kalish, Y., & Lusher, D. (2007). Introduction to ERGMs. *Social Networks.*
- Holland, P. W., Laskey, K. B., & Leinhardt, S. (1983). Stochastic blockmodels. *Social Networks.*

**Expected Output:** A document covering the major random graph model families, with analysis of how each relates to Lutufi's modeling framework. ERGMs receive particular attention as they are the most widely used generative models in social network analysis and represent both a complement and a potential competitor to Lutufi's approach.

**How It Informs Design:** Determines whether Lutufi should include random graph generation capabilities (for synthetic data, null model comparison, and testing). Informs the relationship between Lutufi and the ERGM community — whether Lutufi incorporates ERGM-like features or explicitly differentiates from them. Provides benchmark network structures for testing.

---

### 2.10 Missing Data Theory

**What to Study:** Rubin's classification: missing completely at random (MCAR), missing at random (MAR), missing not at random (MNAR). Implications for inference. Maximum likelihood with missing data (EM algorithm). Multiple imputation. Bayesian approaches to missing data. Missing data in networks: missing nodes, missing edges, partial attribute observation. Network sampling bias. How incompleteness affects structural metrics and probabilistic inference differently.

**Key References:**
- Little, R. J. A., & Rubin, D. B. (2019). *Statistical Analysis with Missing Data* (3rd ed.). Wiley.
- Schafer, J. L., & Graham, J. W. (2002). Missing data: Our view of the state of the art. *Psychological Methods.*
- Kossinets, G. (2006). Effects of missing data in social networks. *Social Networks.*
- Handcock, M. S., & Gile, K. J. (2010). Modeling social networks from sampled data. *Annals of Applied Statistics.*

**Expected Output:** A document on missing data theory as it applies to network analysis, with specific classification of the types of missingness Lutufi will encounter and the appropriate handling strategy for each. Decision framework for which missing data method to apply based on missingness mechanism and network properties.

**How It Informs Design:** Directly determines the missing data handling pipeline in Lutufi. Informs how the API communicates uncertainty about missing data to the user. Determines which methods (EM, multiple imputation, Bayesian inference with latent variables) Lutufi implements and when each is recommended.

---

### 2.11 Numerical Stability in Probabilistic Computation

**What to Study:** Floating-point arithmetic: IEEE 754 standard, machine epsilon, loss of significance. Log-space arithmetic for probability computation. The log-sum-exp trick. Numerical stability of matrix operations relevant to inference (matrix inversion, eigenvalue computation, Cholesky decomposition). Underflow and overflow in probability products. Condition numbers and when inference results should not be trusted. Testing strategies for numerical correctness.

**Key References:**
- Higham, N. J. (2002). *Accuracy and Stability of Numerical Algorithms* (2nd ed.). SIAM.
- Goldberg, D. (1991). What every computer scientist should know about floating-point arithmetic. *ACM Computing Surveys.*
- Press, W. H., et al. (2007). *Numerical Recipes* (3rd ed.). Cambridge University Press.

**Expected Output:** A document covering the numerical pitfalls specific to probabilistic computation on networks, with concrete examples of where naive implementation produces wrong results. Specification of the numerical guard layer that Lutufi must implement.

**How It Informs Design:** Directly determines the numerical implementation strategy throughout Lutufi. All probability operations must use log-space arithmetic. All matrix operations must check conditioning. The test suite must include numerical edge cases. Error messages must warn users when results may be numerically unreliable.

---

### 2.12 Existing Tools — Comparative Analysis

**What to Study:** Detailed analysis of each major existing tool: pgmpy (architecture, API, strengths, weaknesses, user complaints), bnlearn (same), NetworkX (same), igraph (same), graph-tool (same), PyMC (same), Pyro/NumPyro (same), Stan (same), Gephi (same), DoWhy (same). Focus on GitHub issues, user forums, and published reviews that document specific pain points. Identification of design decisions that were successful and should be emulated versus decisions that caused problems.

**Key References:**
- GitHub repositories and issue trackers for each tool
- SciPy conference proceedings papers for pgmpy, NetworkX
- PyMC and Pyro documentation and tutorials
- User forum discussions (Stack Overflow, Reddit r/statistics, r/networkscience)

**Expected Output:** Individual comparative analysis documents for each tool, plus a synthesis gap analysis document that identifies what none of them do that Lutufi will. This gap analysis is the definitive statement of Lutufi's differentiation.

**How It Informs Design:** Every API design decision should be informed by what worked and what failed in existing tools. Naming conventions, parameter ordering, error message style, documentation structure — all should be evaluated against real user experiences with existing tools. The gap analysis confirms that Lutufi is building something genuinely new rather than duplicating existing capability.

---

### 2.13 Epidemiological Models and Diffusion Processes

**What to Study:** SIR, SEIR, and SIS compartmental models. Network epidemiology: epidemic thresholds on networks, super-spreaders, contact tracing. Independent cascade model. Linear threshold model. Bass diffusion model. The formal relationship between epidemic models and information/belief diffusion. How probabilistic uncertainty enters epidemiological models. Where network structure changes the dynamics compared to mean-field approximations.

**Key References:**
- Pastor-Satorras, R., & Vespignani, A. (2001). Epidemic spreading in scale-free networks. *Physical Review Letters.*
- Kiss, I. Z., Miller, J. C., & Simon, P. L. (2017). *Mathematics of Epidemics on Networks.* Springer.
- Kempe, D., Kleinberg, J., & Tardos, É. (2003). Maximizing influence spread. *KDD.*
- Centola, D. (2010). Spread of behavior in online social networks. *Science.*

**Expected Output:** A document covering epidemiological and diffusion models with analysis of how each maps to Lutufi's framework. Template specifications for the epidemiology examples in Lutufi's example library. Identification of which diffusion models are natural Bayesian network formulations and which require MRF or factor graph representations.

**How It Informs Design:** Determines the structure of Lutufi's epidemiology-focused examples. Informs the API for specifying transmission models, compartmental states, and intervention protocols. Ensures that Lutufi's temporal dynamics capabilities align with the requirements of epidemiological and diffusion modeling.

---

### 2.14 Intelligence and Security Applications (Open Source)

**What to Study:** Open-source intelligence (OSINT) methodology. Dark network analysis: how covert networks differ structurally from open ones. Influence operation detection. Radicalization pathway models. Financial crime network analysis (money laundering, shell company detection). Adversarial conditions: how intelligence data differs from academic data (deliberate deception, partial observation, source reliability variation). Data provenance and analytic tradecraft standards.

**Key References:**
- Krebs, V. E. (2002). Mapping networks of terrorist cells. *Connections.*
- Raab, J., & Milward, H. B. (2003). Dark networks as problems. *Journal of Public Administration Research and Theory.*
- Gerdes, L. M. (ed.) (2015). *Illuminating Dark Networks.* Cambridge University Press.
- Everton, S. F. (2012). *Disrupting Dark Networks.* Cambridge University Press.

**Expected Output:** A document on intelligence applications of network analysis, covering publicly available literature only. Identification of the specific features that intelligence users need (adversarial robustness, uncertainty quantification, provenance tracking, air-gapped deployment). Assessment of ethical implications.

**How It Informs Design:** Informs the adversarial data handling capabilities. Shapes the provenance tracking features. Identifies deployment constraints (air-gapped systems, minimal dependencies) that affect build and packaging decisions. Provides context for the ethical framework document.

---

### 2.15 Ethical Framework and Misuse Analysis

**What to Study:** Ethics of network analysis and surveillance technology. Dual-use technology considerations. Published ethical frameworks from comparable projects (TensorFlow Responsible AI, Microsoft FATE). Privacy-preserving network analysis. Differential privacy on graphs. The tension between analytical power and potential for harm. Export control regulations applicable to network analysis tools.

**Key References:**
- Zook, M., et al. (2017). Ten simple rules for responsible big data research. *PLOS Computational Biology.*
- Nissenbaum, H. (2009). *Privacy in Context.* Stanford University Press.
- Narayanan, A., & Shmatikov, V. (2009). De-anonymizing social networks. *IEEE S&P.*
- European Union AI Act provisions on risk classification (2024).

**Expected Output:** The ethical framework document for Lutufi. A misuse analysis enumerating concrete harmful use scenarios and the design responses to each. An assessment of export control implications.

**How It Informs Design:** Determines which safeguards are built into the library (input validation, anonymization utilities, audit logging). Shapes the ETHICS.md governance document. Informs the contributor policy regarding contributions that could enable specific harms.

---

## 3. Phase 2: During-Development Research

Phase 2 research happens in parallel with coding. These topics deepen understanding in areas where the Phase 1 foundation is sufficient to begin implementation but where additional depth will improve the quality of the implementation.

**Estimated Duration:** Concurrent with Months 7–34

---

### 3.1 MCMC Methods — Advanced

**What to Study:** Gibbs sampling derivations for graphical models. Metropolis-Hastings with application to network inference. Hamiltonian Monte Carlo as used by Stan and PyMC. Convergence diagnostics: R-hat, effective sample size, trace plots. Mixing problems in high-dimensional network models. Rao-Blackwellization for variance reduction.

**Key References:**
- Gelman, A., et al. (2013). *Bayesian Data Analysis* (3rd ed.). CRC Press. (Chapters 11–12)
- Robert, C. P., & Casella, G. (2004). *Monte Carlo Statistical Methods* (2nd ed.). Springer.
- Brooks, S., et al. (2011). *Handbook of Markov Chain Monte Carlo.* CRC Press.

**Expected Output:** Implementation specifications for each MCMC method in Lutufi. Convergence diagnostic criteria that Lutufi will apply automatically. Performance benchmarks comparing MCMC methods on standard network inference problems.

**How It Informs Design:** Determines the implementation details of the sampling-based inference engines. Informs automatic convergence detection and user-facing diagnostics. Shapes the API for configuring sampling parameters.

---

### 3.2 Scalability: Sparse Representations and Parallel Computation

**What to Study:** Sparse matrix formats (CSR, CSC, COO) and their suitability for network probability representations. Graph partitioning algorithms for parallel inference. GPU acceleration of belief propagation and matrix operations. Memory-mapped data structures for networks that exceed RAM. Distributed inference across multiple machines.

**Key References:**
- Davis, T. A. (2006). *Direct Methods for Sparse Linear Systems.* SIAM.
- Gonzalez, J. E., et al. (2012). PowerGraph: Distributed graph-parallel computation on natural graphs. *OSDI.*
- Peixoto, T. P. (2014). The graph-tool Python library. *figshare.* (Graph-tool's design for performance)

**Expected Output:** A scalability design document specifying the sparse representations, memory management strategies, and parallelization approaches that Lutufi will use. Benchmarking targets for networks at different scales (1K, 10K, 100K, 1M nodes).

**How It Informs Design:** Directly determines the core data structure implementation. Informs the build system's support for optional GPU acceleration and multi-core parallelism. Shapes the lazy evaluation strategy.

---

### 3.3 Structure Learning in Social Network Contexts

**What to Study:** Constraint-based structure learning (PC algorithm, FCI algorithm). Score-based structure learning (BIC, BDeu). Hybrid approaches. How structure learning must be adapted when the underlying graph has social network properties (scale-free degree distributions, community structure, temporal dynamics). Causal discovery from observational data versus experimental data.

**Key References:**
- Spirtes, P., Glymour, C., & Scheines, R. (2000). *Causation, Prediction, and Search* (2nd ed.). MIT Press.
- Chickering, D. M. (2002). Optimal structure identification with greedy search. *JMLR.*
- Colombo, D., & Maathuis, M. H. (2014). Order-independent constraint-based causal structure learning. *JMLR.*

**Expected Output:** Specification of which structure learning algorithms Lutufi will implement, with adaptations for social/economic network properties. Documentation of when structure learning is appropriate and when it is not (a distinction many tools fail to communicate).

**How It Informs Design:** Determines the structure learning API. Informs the documentation on best practices for model construction — when to specify structure from domain knowledge versus learning it from data.

---

### 3.4 Graph Neural Networks and Message Passing Connections

**What to Study:** The formal relationship between belief propagation (from probabilistic graphical models) and message passing neural networks (from deep learning). Graph neural networks: GCN, GAT, GraphSAGE. Where the two paradigms overlap and where they diverge. Whether Lutufi should provide interoperability with GNN frameworks. Node embeddings (node2vec, DeepWalk) as potential initialization for probabilistic models.

**Key References:**
- Gilmer, J., et al. (2017). Neural message passing for quantum chemistry. *ICML.*
- Kipf, T. N., & Welling, M. (2016). Semi-supervised classification with graph convolutional networks. *ICLR.*
- Yoon, K., et al. (2019). Inference in probabilistic graphical models by graph neural networks. *ICLR Workshop.*

**Expected Output:** A document analyzing the relationship between GNNs and Lutufi's probabilistic inference, with recommendations on interoperability pathways. Assessment of whether GNN-learned representations can serve as priors or initializations for Lutufi models.

**How It Informs Design:** Determines whether Lutufi builds explicit GNN interoperability. Shapes the ML/AI integration strategy. May influence the message-passing implementation if GNN insights improve Lutufi's inference efficiency.

---

### 3.5 Serialization and Reproducibility Standards

**What to Study:** Existing serialization formats for graphical models (XMLBIF, BIF, HUGIN). Network serialization formats (GraphML, GML, GEXF, JSON-based formats). Scientific data formats (HDF5, Parquet, Zarr). Version compatibility challenges. Cryptographic hashing for result verification. How other tools handle model versioning and backward compatibility.

**Key References:**
- GraphML specification (graphml.graphdrawing.org)
- HDF5 documentation (hdfgroup.org)
- JOSS submission guidelines on reproducibility
- Stodden, V., McNutt, M., et al. (2016). Enhancing reproducibility for computational methods. *Science.*

**Expected Output:** A detailed specification for the Lutufi model file format, including structure, versioning strategy, and backward compatibility policy. Assessment of existing formats and justification for creating a custom format (if existing formats are insufficient).

**How It Informs Design:** Directly determines the serialization implementation. Informs the API for saving, loading, and validating models. Shapes the documentation on reproducibility practices.

---

### 3.6 API Design Principles and Usability

**What to Study:** API design as a discipline. Study of successful scientific Python APIs (scikit-learn's estimator interface, pandas DataFrame API, NetworkX's graph API). Principles: consistency, discoverability, progressive disclosure, fail-fast with informative errors. How naming conventions affect learnability. The role of type hints and documentation strings. Accessibility for non-programmer researchers.

**Key References:**
- Bloch, J. (2006). How to design a good API and why it matters. *OOPSLA.*
- scikit-learn developer guidelines on API consistency
- pandas API design documentation
- Wilson, G., et al. (2014). Best practices for scientific computing. *PLOS Biology.*

**Expected Output:** An API design document specifying naming conventions, method signatures, return types, error handling patterns, and integration interfaces. Style guide for all public-facing code.

**How It Informs Design:** Determines every public API decision. Establishes consistency rules that prevent ad hoc naming and structural inconsistencies. Ensures the Python interface feels native to users of the scientific Python stack.

---

### 3.7 Testing Strategies for Probabilistic Software

**What to Study:** Testing probabilistic correctness (as opposed to functional correctness). Analytical ground truth cases for common graphical model structures. Statistical testing for MCMC samples (KS test against known distributions). Property-based testing for probabilistic code. Regression testing for numerical stability. Fuzzing for robustness. How PyMC, Stan, and pgmpy approach testing.

**Key References:**
- Grosse, R. B., et al. (2015). Testing MCMC code. *arXiv:1412.5218.*
- Cook, S. R., Gelman, A., & Rubin, D. B. (2006). Validation of software for Bayesian models using posterior quantiles. *JCGS.*
- Talts, S., et al. (2018). Validating Bayesian inference algorithms with simulation-based calibration. *arXiv:1804.06788.*

**Expected Output:** A testing philosophy document specifying what kinds of tests Lutufi requires, what ground truth solutions are available, and how probabilistic correctness is validated. Specification of the test suite structure.

**How It Informs Design:** Determines the testing infrastructure from the start. Ensures that algorithms are validated against known solutions before exposure to users. Shapes the CI/CD pipeline requirements.

---

## 4. Phase 3: Post-Launch Research

Phase 3 research continues after Lutufi's initial release, informed by user feedback and emerging research. These topics deepen the library's capabilities without blocking the initial release.

**Estimated Duration:** Months 35+ (ongoing)

---

### 4.1 Advanced Temporal Models

**What to Study:** Continuous-time Bayesian networks. Point processes on networks (Hawkes processes for event cascades). Non-parametric temporal models. Real-time inference with streaming network data. Comparison to agent-based modeling approaches for temporal dynamics.

**Expected Output:** Feasibility assessment and design specification for continuous-time extensions to Lutufi's dynamics layer.

**How It Informs Design:** Determines whether Lutufi extends beyond discrete time-stepping to continuous temporal models — a significant capability expansion.

---

### 4.2 Privacy-Preserving Inference

**What to Study:** Differential privacy on graph statistics. Federated learning applied to network models. Secure multi-party computation for collaborative network analysis (where multiple institutions contribute partial network data without revealing it fully). Anonymization of network data and its limitations.

**Expected Output:** Design specification for privacy-preserving features in Lutufi. Assessment of which privacy techniques are compatible with Lutufi's inference methods.

**How It Informs Design:** Determines whether Lutufi adds privacy-preserving inference as a first-class feature — critical for institutional adoption where data sensitivity is paramount.

---

### 4.3 Multilayer and Multiplex Network Extensions

**What to Study:** Formal theory of multiplex networks. Inference on networks with multiple edge types. Interdependent network models. Layer coupling and cross-layer inference. Tensor representations for multilayer networks.

**Expected Output:** Design specification for multilayer network support in Lutufi's core data model.

**How It Informs Design:** Determines how Lutufi represents relationships of different types between the same actors (e.g., financial ties, social ties, and information flows simultaneously).

---

### 4.4 Visualization Design

**What to Study:** Semantically aware network visualization. Uncertainty visualization on graphs. Dynamic network visualization (animation, small multiples, timelines). Color theory for probability displays. User studies on network visualization comprehension. Existing visualization tools (Gephi, D3.js, Cytoscape) and their strengths.

**Expected Output:** Specification for Lutufi's visualization module, emphasizing semantic awareness (nodes colored by posterior probability, edges weighted by influence strength, uncertainty displayed through visual encodings).

**How It Informs Design:** Determines the visualization API and the integration with matplotlib, Plotly, or web-based visualization tools.

---

### 4.5 Benchmarking and Performance Optimization

**What to Study:** Standardized benchmarking methodologies for network analysis software. Profiling tools for Rust/C++ and Python. Cache-friendly data layouts for graph operations. SIMD vectorization for probability computations. Comparison with state-of-the-art performance in graph-tool and igraph.

**Expected Output:** A comprehensive benchmarking suite with results comparing Lutufi to existing tools on standardized problems. Performance optimization roadmap based on profiling results.

**How It Informs Design:** Identifies specific bottlenecks and guides optimization investment. Provides credible performance claims for documentation and publications.

---

## 5. Research Dependencies

The following dependency graph shows which research topics must be completed before others can begin:

```
Phase 1 Dependencies:
├── 2.1 Probability Theory ──────────┐
│                                     ├── 2.2 Bayesian Networks
│                                     ├── 2.3 Markov Random Fields
│                                     └── 2.11 Numerical Stability
│
├── 2.2 Bayesian Networks ───────────┐
│                                     ├── 2.4 Belief Propagation
│                                     ├── 2.5 Causal Inference
│                                     └── 2.6 Dynamic Bayesian Networks
│
├── 2.3 Markov Random Fields ────────┤
│                                     └── 2.4 Belief Propagation
│
├── 2.7 Social Network Analysis ─────┐
│                                     ├── 2.8 Economic Network Theory
│   (can proceed independently)       ├── 2.9 Random Graph Models
│                                     ├── 2.10 Missing Data Theory
│                                     └── 2.13 Epidemiological Models
│
├── 2.12 Existing Tools Analysis ──── (independent, can begin immediately)
│
├── 2.14 Intelligence Applications ── (depends on 2.7 and 2.10)
│
└── 2.15 Ethical Framework ────────── (depends on 2.14, can begin conceptually earlier)

Phase 2 Dependencies:
├── 3.1 MCMC Methods ──────────────── (depends on 2.2, 2.4)
├── 3.2 Scalability ────────────────── (depends on 2.2, 2.3)
├── 3.3 Structure Learning ──────────── (depends on 2.2, 2.5)
├── 3.4 GNN Connections ─────────────── (depends on 2.4)
├── 3.5 Serialization ──────────────── (depends on 2.2, 2.3, 2.6)
├── 3.6 API Design ──────────────────── (depends on 2.12)
└── 3.7 Testing Strategies ──────────── (depends on 2.2, 2.4, 2.11)
```

---

## 6. Estimated Time Allocations

### Phase 1 (Months 1–6): Pre-Development — 26 weeks total

| Topic | Estimated Weeks | Priority |
|---|---|---|
| 2.1 Probability Theory | 2 | Critical |
| 2.2 Bayesian Networks | 4 | Critical |
| 2.3 Markov Random Fields | 2 | Critical |
| 2.4 Belief Propagation | 3 | Critical |
| 2.5 Causal Inference | 3 | Critical |
| 2.6 Dynamic Bayesian Networks | 2 | Critical |
| 2.7 Social Network Analysis | 2 | Critical |
| 2.8 Economic Network Theory | 2 | High |
| 2.9 Random Graph Models | 1 | Medium |
| 2.10 Missing Data Theory | 2 | High |
| 2.11 Numerical Stability | 1 | Critical |
| 2.12 Existing Tools Analysis | 2 | High |
| 2.13 Epidemiological Models | 1 | Medium |
| 2.14 Intelligence Applications | 1 | Medium |
| 2.15 Ethical Framework | 1 | High |

*Note: Some topics can be studied in parallel given their independence. The 26-week estimate assumes partial parallelism fits within the 6-month Phase 1 window.*

### Phase 2 (Months 7–34): During-Development — ongoing, interleaved with coding

| Topic | Estimated Weeks | Priority |
|---|---|---|
| 3.1 MCMC Methods | 3 | High |
| 3.2 Scalability | 3 | High |
| 3.3 Structure Learning | 2 | Medium |
| 3.4 GNN Connections | 2 | Low |
| 3.5 Serialization | 2 | High |
| 3.6 API Design | 2 | Critical |
| 3.7 Testing Strategies | 2 | Critical |

### Phase 3 (Months 35+): Post-Launch — ongoing

| Topic | Estimated Weeks | Priority |
|---|---|---|
| 4.1 Advanced Temporal Models | 4 | Medium |
| 4.2 Privacy-Preserving Inference | 3 | Medium |
| 4.3 Multilayer Networks | 3 | Medium |
| 4.4 Visualization Design | 2 | Medium |
| 4.5 Benchmarking | 2 | High |

---

*"Trust in Allah, then tie your camel." — Prophet Muhammad (ﷺ)*

---

*This roadmap is a living document. Topics may be reordered, expanded, or added as research reveals new dependencies and requirements. The critical principle is: understand before you build.*
