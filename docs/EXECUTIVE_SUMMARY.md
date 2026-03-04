# Executive Summary: Lutufi

---

**Document Version:** 1.0
**Status:** Working Draft — Research Phase
**Author:** Wasswa Lutufi Sebbanja
**Last Updated:** March 2026
**License:** Apache 2.0

---

## The Problem

Researchers, intelligence analysts, economists, and policymakers routinely need to ask probabilistic questions about social and economic networks: How likely is a financial shock at one institution to cascade through an interbank lending network? What is the probability that misinformation originating from a particular account will reach a specific community? How does removing a node from a covert network affect the network's operational capacity under uncertainty? Today, answering these questions requires combining multiple incompatible tools — a graph library like NetworkX for structural analysis, a Bayesian network library like pgmpy for probabilistic inference, and substantial custom glue code to bridge the two — resulting in fragile, error-prone, and irreproducible workflows. No existing tool provides a unified framework where network structure and probabilistic reasoning coexist natively. This gap blocks rigorous research at the intersection of network science and probabilistic reasoning, precisely where some of the most consequential questions about systemic risk, social influence, and information warfare reside.

## The Solution

**Lutufi** is an open-source library that unifies probabilistic graphical models (Bayesian networks, Markov random fields, factor graphs, dynamic Bayesian networks) with social and economic network analysis into a single, coherent computational framework. In Lutufi, a network node is simultaneously a social actor and a random variable; an edge is simultaneously a relational tie and a probabilistic dependency. This dual representation eliminates the translation boundary between structural and probabilistic analysis. The core engine is implemented in a high-performance systems language (Rust or C++) with Python as the primary user-facing interface, integrating seamlessly with pandas, NetworkX, and the scientific Python stack. Lutufi enables researchers to perform probabilistic inference, causal reasoning (do-calculus), dynamic temporal modeling, and robust analysis under incomplete and adversarial data conditions — all within a single, reproducible framework.

## Key Differentiators

- **Unified Model:** Network structure and probabilistic semantics coexist as a single object, eliminating fragile translation layers between tools.
- **Causal Inference on Networks:** Native do-calculus support enables interventional and counterfactual reasoning over social and economic structures — distinguishing correlation from causation at the network level.
- **Dynamic Bayesian Networks:** Temporal modeling is built in from the start, supporting networks that evolve over time with incremental inference updates rather than full recomputation.
- **Robustness by Design:** Principled missing data handling (EM algorithm, multiple imputation) and adversarial input detection are first-class features, not afterthoughts — making Lutufi suitable for real-world institutional data that is always incomplete.
- **Rich Example Library:** A minimum of 30 domain-specific case studies — spanning epidemiology, finance, political science, intelligence analysis, organizational behavior, and social influence — each structured as a self-contained research artifact with problem statement, model, results, and interpretation.
- **Reproducibility:** A standardized model serialization format encodes complete models (structure, parameters, evidence, inference configuration, results) for sharing and exact reproduction.
- **Research-Centric API:** Designed around questions researchers actually ask, not abstract graph operations, with native integration into Jupyter notebook workflows.

## Target Users

| User Category | Primary Need |
|---|---|
| **Academic Researchers** (social scientists, economists, epidemiologists) | Probabilistic reasoning over network data without becoming a graphical models specialist |
| **Intelligence Analysts** | Uncertainty quantification over incomplete, adversarial network data; covert network reconstruction |
| **Central Banks & Financial Regulators** | Systemic risk assessment with formal uncertainty quantification; stress-test scenario modeling |
| **Policy Bodies & International Organizations** | Evidence-based policy analysis grounded in probabilistic network models |
| **Data Scientists & Quantitative Analysts** | Scalable network-based probabilistic analytics integrated into standard data science workflows |

## Current Status and Development Plan

Lutufi is currently in the **Research and Foundation Phase** (Phase 0). This phase involves completing approximately 60 research and design documents that demonstrate deep understanding of the problem space before any code is written. The complete development plan spans six phases over approximately 40 months:

| Phase | Timeline | Deliverable |
|---|---|---|
| **Phase 0:** Research & Foundation | Months 1–6 | Complete foundation documents; core language decision finalized |
| **Phase 1:** Core Implementation | Months 7–14 | Unified data model; basic inference; Python bindings |
| **Phase 2:** Extended Inference & Causality | Months 15–20 | Multiple inference backends; do-calculus; parameter learning |
| **Phase 3:** Dynamics & Example Library | Months 21–28 | Dynamic Bayesian networks; missing data handling; 30+ examples |
| **Phase 4:** Scalability & Stabilization | Months 29–34 | Large-scale optimization; model serialization; Lutufi 1.0 RC |
| **Phase 5:** Publication & Community | Months 35–40 | JOSS paper; conference presentations; contributor community |

## Why This Matters

The questions Lutufi enables — about systemic financial risk, epidemic propagation, information warfare, policy diffusion, and social influence — are among the most consequential questions facing researchers and institutions today. These questions sit at the intersection of network science and probabilistic reasoning, precisely where current tools fail. Lutufi does not merely provide a new library; it provides a new *capability* — the ability to reason rigorously under uncertainty over the relational structures that shape societies and economies.

## Contact and Attribution

- **Author:** Wasswa Lutufi Sebbanja
- **Repository:** [github.com/lutufi/lutufi](https://github.com/lutufi/lutufi)
- **License:** Apache 2.0 (permissive with patent protection and attribution requirement)
- **Email:** [Contact via repository]

---

*"Trust in Allah, then tie your camel." — Prophet Muhammad (ﷺ)*
