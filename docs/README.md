# Lutufi Documentation

**Document Version**: 1.0  
**Status**: Working Draft  
**Author**: Wasswa Lutufi Sebbanja  
**Last Updated**: March 2026  

---

## Overview

This directory contains the comprehensive documentation for **Lutufi** — a library unifying Bayesian networks with social and economic network analysis. The documentation serves multiple audiences:

- **Developers** implementing the library
- **Researchers** evaluating the methodology
- **Users** applying the library to domain problems
- **Contributors** extending the project
- **Stakeholders** assessing project viability

The documentation is organized into logical sections covering theory, design, domain applications, governance, and reference materials. Documents are written as "living documents" — they evolve with the project and are updated continuously.

---

## Quick Start

### For New Developers Getting Started

Start here to understand the project and begin contributing:

1. **[EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md)** — Project overview and value proposition
2. **[PROPOSAL.md](PROPOSAL.md)** — Detailed project proposal and scope
3. **[design/ARCHITECTURE.md](design/ARCHITECTURE.md)** — System architecture overview
4. **[design/API_DESIGN.md](design/API_DESIGN.md)** — API design specifications
5. **[governance/CONTRIBUTING.md](governance/CONTRIBUTING.md)** — How to contribute

### For Researchers Evaluating Lutufi

Start here to understand the scientific contribution:

1. **[EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md)** — Overview of scientific contribution
2. **[RESEARCH_ROADMAP.md](RESEARCH_ROADMAP.md)** — Research agenda and methodology
3. **[foundations/BAYESIAN_NETWORKS.md](foundations/BAYESIAN_NETWORKS.md)** — Theoretical foundations
4. **[foundations/CAUSAL_MODELS.md](foundations/CAUSAL_MODELS.md)** — Causal inference framework
5. **[analysis/](analysis/)** — Comparative analysis with existing tools

### For Potential Contributors

Start here to find ways to contribute:

1. **[governance/CONTRIBUTING.md](governance/CONTRIBUTING.md)** — Contribution guidelines
2. **[KNOWLEDGE_AUDIT.md](KNOWLEDGE_AUDIT.md)** — Skills assessment and needs
3. **[domains/POTENTIAL_PROJECTS.md](domains/POTENTIAL_PROJECTS.md)** — Project ideas
4. **[reference/OPEN_QUESTIONS.md](reference/OPEN_QUESTIONS.md)** — Open research questions
5. **[communication/COMMUNITY_BUILDING.md](communication/COMMUNITY_BUILDING.md)** — Community information

### For Users Looking for Domain Applications

Start here to understand applications in your domain:

1. **[domains/RESEARCH_FIELDS.md](domains/RESEARCH_FIELDS.md)** — Overview of application areas
2. Domain-specific documents:
   - **[domains/FINANCIAL_CONTAGION.md](domains/FINANCIAL_CONTAGION.md)** — Banking and finance
   - **[domains/EPIDEMIOLOGICAL_MODELS.md](domains/EPIDEMIOLOGICAL_MODELS.md)** — Disease spread
   - **[domains/MISINFORMATION_OPERATIONS.md](domains/MISINFORMATION_OPERATIONS.md)** — Information ecosystems
   - **[domains/SOCIAL_INFLUENCE_MODELS.md](domains/SOCIAL_INFLUENCE_MODELS.md)** — Social networks
   - **[domains/SUPPLY_CHAIN_NETWORKS.md](domains/SUPPLY_CHAIN_NETWORKS.md)** — Logistics
   - **[domains/ORGANIZATIONAL_NETWORKS.md](domains/ORGANIZATIONAL_NETWORKS.md)** — Management

### For Stakeholders Assessing the Project

Start here for high-level project assessment:

1. **[EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md)** — Executive overview
2. **[PROPOSAL.md](PROPOSAL.md)** — Project proposal
3. **[THEORY_OF_CHANGE.md](THEORY_OF_CHANGE.md)** — Impact theory
4. **[JOSS_PREP.md](JOSS_PREP.md)** — Software publication preparation
5. **[governance/ETHICS.md](governance/ETHICS.md)** — Ethical framework
6. **[governance/LICENSE_RATIONALE.md](governance/LICENSE_RATIONALE.md)** — Licensing approach

---

## Document Organization

### [Root Directory](.)

Core project documents providing overview and orientation.

| Document | Purpose | Status |
|----------|---------|--------|
| [EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md) | Project overview for all audiences | Complete |
| [PROPOSAL.md](PROPOSAL.md) | Detailed project proposal | Complete |
| [RESEARCH_ROADMAP.md](RESEARCH_ROADMAP.md) | Research agenda and timeline | Complete |
| [THEORY_OF_CHANGE.md](THEORY_OF_CHANGE.md) | Theory of impact and outcomes | Complete |
| [KNOWLEDGE_AUDIT.md](KNOWLEDGE_AUDIT.md) | Skills and knowledge assessment | Complete |
| [GLOSSARY.md](GLOSSARY.md) | Terminology and definitions | Complete |
| [BIBLIOGRAPHY.md](BIBLIOGRAPHY.md) | Master bibliography with annotations | Working Draft |
| [JOSS_PREP.md](JOSS_PREP.md) | Journal of Open Source Software preparation | Working Draft |

### [analysis/](analysis/)

Comparative analysis of existing tools and libraries.

| Document | Purpose | Status |
|----------|---------|--------|
| [PGMPY_ANALYSIS.md](analysis/PGMPY_ANALYSIS.md) | Analysis of pgmpy (Python BN library) | Complete |
| [BNLEARN_ANALYSIS.md](analysis/BNLEARN_ANALYSIS.md) | Analysis of bnlearn (R BN library) | Complete |
| [PYMC_PYRO_ANALYSIS.md](analysis/PYMC_PYRO_ANALYSIS.md) | Analysis of PyMC and Pyro | Complete |
| [NETWORKX_ANALYSIS.md](analysis/NETWORKX_ANALYSIS.md) | Analysis of NetworkX | Complete |
| [GRAPHTOOL_IGRAPH_ANALYSIS.md](analysis/GRAPHTOOL_IGRAPH_ANALYSIS.md) | Analysis of graph-tool and igraph | Complete |
| [GEPHI_ANALYSIS.md](analysis/GEPHI_ANALYSIS.md) | Analysis of Gephi | Complete |
| [GAP_ANALYSIS.md](analysis/GAP_ANALYSIS.md) | Synthesis of gap analysis | Complete |

### [communication/](communication/)

Documents related to communication, outreach, and dissemination.

| Document | Purpose | Status |
|----------|---------|--------|
| [DOCUMENTATION_PHILOSOPHY.md](communication/DOCUMENTATION_PHILOSOPHY.md) | Approach to documentation | Complete |
| [COMMUNITY_BUILDING.md](communication/COMMUNITY_BUILDING.md) | Community development strategy | Complete |
| [ACADEMIC_PAPER_OUTLINE.md](communication/ACADEMIC_PAPER_OUTLINE.md) | Outline for academic publications | Complete |
| [EXAMPLE_LIBRARY_STRUCTURE.md](communication/EXAMPLE_LIBRARY_STRUCTURE.md) | Structure for example code | Complete |

### [design/](design/)

Technical design documents for implementation.

| Document | Purpose | Status |
|----------|---------|--------|
| [ARCHITECTURE.md](design/ARCHITECTURE.md) | System architecture | Complete |
| [API_DESIGN.md](design/API_DESIGN.md) | API design specifications | Complete |
| [DATA_MODEL.md](design/DATA_MODEL.md) | Data structures and models | Complete |
| [DATA_FLOW.md](design/DATA_FLOW.md) | Data flow and processing | Complete |
| [DEPENDENCY_ANALYSIS.md](design/DEPENDENCY_ANALYSIS.md) | Dependency management | Complete |
| [ERROR_HANDLING.md](design/ERROR_HANDLING.md) | Error handling strategy | Complete |
| [TESTING_PHILOSOPHY.md](design/TESTING_PHILOSOPHY.md) | Testing approach | Complete |
| [PERFORMANCE_BENCHMARKING.md](design/PERFORMANCE_BENCHMARKING.md) | Performance requirements | Complete |
| [SCALABILITY.md](design/SCALABILITY.md) | Scalability design | Complete |
| [SERIALIZATION.md](design/SERIALIZATION.md) | Data serialization | Complete |
| [SECURITY.md](design/SECURITY.md) | Security considerations | Complete |
| [MULTILANGUAGE_BINDINGS.md](design/MULTILANGUAGE_BINDINGS.md) | Python/Rust integration | Complete |

### [domains/](domains/)

Domain-specific applications and use cases.

| Document | Purpose | Status |
|----------|---------|--------|
| [RESEARCH_FIELDS.md](domains/RESEARCH_FIELDS.md) | Overview of application domains | Complete |
| [FINANCIAL_CONTAGION.md](domains/FINANCIAL_CONTAGION.md) | Banking and financial networks | Complete |
| [EPIDEMIOLOGICAL_MODELS.md](domains/EPIDEMIOLOGICAL_MODELS.md) | Disease spread modeling | Complete |
| [MISINFORMATION_OPERATIONS.md](domains/MISINFORMATION_OPERATIONS.md) | Misinformation dynamics | Complete |
| [SOCIAL_INFLUENCE_MODELS.md](domains/SOCIAL_INFLUENCE_MODELS.md) | Social influence and diffusion | Complete |
| [OPINION_DYNAMICS.md](domains/OPINION_DYNAMICS.md) | Opinion formation models | Complete |
| [SUPPLY_CHAIN_NETWORKS.md](domains/SUPPLY_CHAIN_NETWORKS.md) | Supply chain resilience | Complete |
| [ORGANIZATIONAL_NETWORKS.md](domains/ORGANIZATIONAL_NETWORKS.md) | Organizational analysis | Complete |
| [POLITICAL_NETWORKS.md](domains/POLITICAL_NETWORKS.md) | Political and policy networks | Complete |
| [FINANCIAL_CRIME_NETWORKS.md](domains/FINANCIAL_CRIME_NETWORKS.md) | Financial crime detection | Complete |
| [POTENTIAL_PROJECTS.md](domains/POTENTIAL_PROJECTS.md) | Potential research projects | Working Draft |

### [foundations/](foundations/)

Theoretical and mathematical foundations.

| Document | Purpose | Status |
|----------|---------|--------|
| [MATHEMATICS.md](foundations/MATHEMATICS.md) | Mathematical preliminaries | Complete |
| [PROBABILITY_THEORY.md](foundations/PROBABILITY_THEORY.md) | Probability foundations | Complete |
| [BAYESIAN_NETWORKS.md](foundations/BAYESIAN_NETWORKS.md) | Bayesian network theory | Complete |
| [BELIEF_PROPAGATION.md](foundations/BELIEF_PROPAGATION.md) | Inference algorithms | Complete |
| [CAUSAL_MODELS.md](foundations/CAUSAL_MODELS.md) | Causal inference theory | Complete |
| [CONDITIONAL_INDEPENDENCE.md](foundations/CONDITIONAL_INDEPENDENCE.md) | CI and d-separation | Complete |
| [INFORMATION_THEORY.md](foundations/INFORMATION_THEORY.md) | Information-theoretic concepts | Complete |
| [NETWORK_SCIENCE.md](foundations/NETWORK_SCIENCE.md) | Network theory | Complete |
| [SOCIAL_NETWORKS.md](foundations/SOCIAL_NETWORKS.md) | Social network analysis | Complete |
| [ECONOMIC_NETWORKS.md](foundations/ECONOMIC_NETWORKS.md) | Economic network theory | Complete |
| [RANDOM_GRAPH_MODELS.md](foundations/RANDOM_GRAPH_MODELS.md) | Random graph theory | Complete |
| [EXPONENTIAL_RANDOM_GRAPH_MODELS.md](foundations/EXPONENTIAL_RANDOM_GRAPH_MODELS.md) | ERGM theory | Complete |
| [MULTILAYER_NETWORKS.md](foundations/MULTILAYER_NETWORKS.md) | Multilayer networks | Complete |
| [TEMPORAL_NETWORKS.md](foundations/TEMPORAL_NETWORKS.md) | Dynamic networks | Complete |
| [NETWORK_RESILIENCE.md](foundations/NETWORK_RESILIENCE.md) | Resilience and robustness | Complete |
| [STOCHASTIC_PROCESSES.md](foundations/STOCHASTIC_PROCESSES.md) | Stochastic processes on networks | Complete |
| [NUMERICAL_STABILITY.md](foundations/NUMERICAL_STABILITY.md) | Numerical considerations | Complete |
| [MISSING_DATA.md](foundations/MISSING_DATA.md) | Handling missing data | Complete |
| [DARK_NETWORKS.md](foundations/DARK_NETWORKS.md) | Hidden/population networks | Complete |

### [governance/](governance/)

Governance, ethics, and contribution guidelines.

| Document | Purpose | Status |
|----------|---------|--------|
| [CONTRIBUTING.md](governance/CONTRIBUTING.md) | Contribution guidelines | Complete |
| [ETHICS.md](governance/ETHICS.md) | Ethical framework | Complete |
| [LICENSE_RATIONALE.md](governance/LICENSE_RATIONALE.md) | License choice rationale | Complete |
| [DATA_PRIVACY.md](governance/DATA_PRIVACY.md) | Privacy considerations | Complete |
| [EXPORT_CONTROL.md](governance/EXPORT_CONTROL.md) | Export control guidance | Complete |
| [MISUSE_ANALYSIS.md](governance/MISUSE_ANALYSIS.md) | Potential misuse scenarios | Complete |

### [reference/](reference/)

Reference materials for ongoing development.

| Document | Purpose | Status |
|----------|---------|--------|
| [OPEN_QUESTIONS.md](reference/OPEN_QUESTIONS.md) | Open research questions | Working Draft |
| [DESIGN_DECISIONS.md](reference/DESIGN_DECISIONS.md) | Architecture decision records | Working Draft |
| [ASSUMPTIONS_LOG.md](reference/ASSUMPTIONS_LOG.md) | Documented assumptions | Working Draft |

### [research/](research/)

User research and workflow analysis.

| Document | Purpose | Status |
|----------|---------|--------|
| [RESEARCHER_WORKFLOWS.md](research/RESEARCHER_WORKFLOWS.md) | Academic researcher workflows | Complete |
| [INTELLIGENCE_WORKFLOWS.md](research/INTELLIGENCE_WORKFLOWS.md) | Intelligence analyst workflows | Complete |
| [REGULATOR_WORKFLOWS.md](research/REGULATOR_WORKFLOWS.md) | Regulator workflows | Complete |
| [REPRODUCIBILITY_CRISIS.md](research/REPRODUCIBILITY_CRISIS.md) | Reproducibility considerations | Complete |

---

## Status Legend

| Status | Meaning |
|--------|---------|
| **Complete** | Document is finalized for current phase; updates are refinements |
| **Working Draft** | Document is actively being developed; content may change significantly |
| **Stub** | Document outline exists; content to be written |
| **Deprecated** | Document is no longer maintained; superseded by other documents |

---

## Contributing to Documentation

### How to Suggest Improvements

1. **Open an issue**: Describe the documentation improvement needed
2. **Propose changes**: Submit a pull request with proposed edits
3. **Request clarification**: Ask questions via discussion forums
4. **Report errors**: Point out factual errors or broken links

### Documentation Standards

- **Substantive**: Documents must provide real value, not just formal compliance
- **Accurate**: Technical content must be correct and verified
- **Accessible**: Write for the intended audience; define jargon
- **Cross-referenced**: Link to related documents
- **Versioned**: Include version and last-updated date

### Review Process

1. Technical accuracy review by subject matter expert
2. Clarity review by representative of target audience
3. Integration check for cross-document consistency
4. Approval by documentation maintainer

---

## Document Map

### By Audience

#### For Developers (Implementation Focus)
- design/ARCHITECTURE.md
- design/API_DESIGN.md
- design/DATA_MODEL.md
- design/ERROR_HANDLING.md
- design/TESTING_PHILOSOPHY.md
- reference/DESIGN_DECISIONS.md
- reference/ASSUMPTIONS_LOG.md
- governance/CONTRIBUTING.md

#### For Researchers (Scientific Focus)
- EXECUTIVE_SUMMARY.md
- RESEARCH_ROADMAP.md
- foundations/BAYESIAN_NETWORKS.md
- foundations/CAUSAL_MODELS.md
- foundations/BELIEF_PROPAGATION.md
- analysis/ (all comparative analyses)
- reference/OPEN_QUESTIONS.md
- JOSS_PREP.md

#### For Domain Users (Application Focus)
- domains/RESEARCH_FIELDS.md
- domains/FINANCIAL_CONTAGION.md
- domains/EPIDEMIOLOGICAL_MODELS.md
- domains/MISINFORMATION_OPERATIONS.md
- domains/SOCIAL_INFLUENCE_MODELS.md
- research/ (workflow documents)

#### For Stakeholders (Assessment Focus)
- EXECUTIVE_SUMMARY.md
- PROPOSAL.md
- THEORY_OF_CHANGE.md
- governance/ETHICS.md
- governance/LICENSE_RATIONALE.md

### By Phase

#### Phase 1: Foundation (Complete)
- All foundations/ documents
- design/ARCHITECTURE.md, API_DESIGN.md
- analysis/ documents
- PROPOSAL.md, EXECUTIVE_SUMMARY.md

#### Phase 2: Core Implementation (Working Draft)
- design/ (detailed design documents)
- reference/ (ongoing decisions and questions)
- governance/CONTRIBUTING.md

#### Phase 3: Applications (In Progress)
- domains/ documents
- research/ workflow documents
- POTENTIAL_PROJECTS.md

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | March 2026 | Complete documentation index with all current documents; added reference/ section; reorganized by audience and phase |

---

## Navigation Tips

### Finding Information

1. **Use the glossary**: [GLOSSARY.md](GLOSSARY.md) defines all technical terms
2. **Check cross-references**: Documents link to related content
3. **Follow reading paths**: See [Quick Start](#quick-start) for curated paths
4. **Search by tag**: Documents include keywords for searchability

### Document Conventions

- **Code references**: Function names, file paths, and code snippets use `monospace`
- **Document links**: Internal links use relative paths
- **Citations**: Academic citations link to BIBLIOGRAPHY.md
- **Warnings**: Important caveats use blockquotes
- **Examples**: Code examples are complete and runnable

---

## Contact

For questions about documentation:
- Open an issue in the project repository
- Contact the documentation maintainer
- Join the community discussion forum

---

**License**: This documentation is licensed under Apache 2.0, consistent with the Lutufi project license.
