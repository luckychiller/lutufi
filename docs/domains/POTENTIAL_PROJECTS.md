# Potential Projects

**Document Version**: 1.0  
**Status**: Working Draft  
**Author**: Wasswa Lutufi Sebbanja  
**Last Updated**: March 2026  

---

## Table of Contents

1. [Introduction](#introduction)
2. [Research Projects](#research-projects)
3. [Tool and Feature Projects](#tool-and-feature-projects)
4. [Application Projects](#application-projects)
5. [Collaboration Opportunities](#collaboration-opportunities)
6. [Student Projects](#student-projects)
7. [Grant Opportunities](#grant-opportunities)
8. [Project Format](#project-format)
9. [Priority Rankings](#priority-rankings)

---

## Introduction

This document catalogs potential projects that extend, apply, or complement the Lutufi library. These projects serve multiple purposes:

- **Validation**: Real applications test and validate Lutufi's capabilities
- **Extension**: New tools and features enhance the ecosystem
- **Research**: Scientific projects advance the underlying methodology
- **Adoption**: Successful applications demonstrate value to potential users

Projects are organized by type and include prerequisites, estimated effort, and expected impact. They are intended as starting points for researchers, developers, students, and organizations interested in contributing to or building upon Lutufi.

---

## Research Projects

### RES-001: Comparative Evaluation of Inference Algorithms on Social Networks

**Description**: Systematic comparison of loopy belief propagation, Gibbs sampling, variational methods, and exact inference on real-world social networks. Identify network structural features (clustering, diameter, degree distribution) that predict algorithm performance.

**Research Questions**:
- Which inference algorithms perform best on different classes of social networks?
- Can network metrics predict algorithm convergence and accuracy?
- What is the tradeoff between approximation quality and computation time?

**Prerequisites**:
- Understanding of probabilistic inference algorithms
- Access to social network datasets (or use public datasets: SNAP, KONECT)
- Statistical analysis skills

**Estimated Effort**: 6-12 months (PhD-level project)

**Impact**: HIGH — Would provide practical guidance for algorithm selection and potentially inform automatic selection heuristics in Lutufi.

**Deliverables**:
- Benchmark dataset of social networks with ground truth
- Comparative evaluation paper
- Recommendations for algorithm selection

---

### RES-002: Causal Discovery in the Presence of Network Interference

**Description**: Develop and evaluate methods for causal discovery when the stable unit treatment value assumption (SUTVA) is violated due to network interference (treatment of one unit affects outcomes of neighbors).

**Research Questions**:
- How does network interference affect structure learning algorithms?
- Can we detect interference from observational data?
- What modifications to causal discovery algorithms account for interference?

**Prerequisites**:
- Background in causal inference and graphical models
- Understanding of network effects and interference
- Programming skills for implementation

**Estimated Effort**: 12-18 months (PhD dissertation project)

**Impact**: VERY HIGH — Network interference is ubiquitous but largely unaddressed in causal discovery literature. Would significantly advance methodology.

**Deliverables**:
- Novel algorithm for causal discovery with interference
- Theoretical analysis (consistency, sample complexity)
- Empirical evaluation on real and synthetic data
- Implementation in Lutufi

---

### RES-003: Scalable Structure Learning for High-Dimensional Networks

**Description**: Extend constraint-based and score-based structure learning algorithms to scale to networks with 10,000+ nodes, relevant for genome-wide association studies, large social networks, and infrastructure networks.

**Research Questions**:
- What approximations enable scalable structure learning while maintaining quality?
- Can distributed computing be effectively used for structure learning?
- What are the fundamental complexity limits?

**Prerequisites**:
- Background in structure learning algorithms
- High-performance computing experience
- Optimization and algorithm design skills

**Estimated Effort**: 12-24 months (PhD dissertation or postdoc project)

**Impact**: HIGH — Would extend Lutufi's applicability to large-scale problems currently intractable.

**Deliverables**:
- Scalable structure learning algorithm
- Distributed implementation
- Complexity analysis
- Application to large real-world dataset

---

### RES-004: Robust Causal Inference Under Unmeasured Confounding

**Description**: Develop sensitivity analysis tools and bounds for causal effects when unmeasured confounding is suspected. Extend to network settings where confounding may be structured.

**Research Questions**:
- How sensitive are network causal estimates to unmeasured confounding?
- Can network structure provide bounds on confounding bias?
- What sensitivity parameters are interpretable to practitioners?

**Prerequisites**:
- Background in causal inference and sensitivity analysis
- Statistical estimation theory
- Programming skills

**Estimated Effort**: 6-12 months

**Impact**: HIGH — Addresses a major limitation in observational network studies.

**Deliverables**:
- Sensitivity analysis framework for network causal inference
- Implementation in Lutufi
- Application paper demonstrating methodology

---

### RES-005: Dynamic Network Models with Structural Breaks

**Description**: Develop methods for detecting and modeling structural breaks (abrupt changes in network structure) in temporal networks. Applications to financial contagion and organizational change.

**Research Questions**:
- How can we detect structural breaks in network formation models?
- What are the implications for causal inference in dynamic networks?
- Can we predict structural breaks from network features?

**Prerequisites**:
- Background in temporal networks and change-point detection
- Time series analysis
- Bayesian or frequentist model selection

**Estimated Effort**: 12-18 months

**Impact**: MEDIUM-HIGH — Important for applications where network structure evolves.

**Deliverables**:
- Structural break detection algorithm for networks
- Theoretical properties (consistency, detection delay)
- Empirical applications
- Lutufi extension

---

## Tool and Feature Projects

### TOOL-001: Interactive Network Visualization Module

**Description**: Develop an interactive visualization module for Lutufi that displays networks, inference results, and causal relationships. Integration with web-based notebooks (Jupyter, Observable).

**Features**:
- Interactive graph layout with force-directed algorithms
- Overlay of inference results (marginals, interventions)
- Animation of message passing and dynamics
- Export to standard formats (PNG, SVG, interactive HTML)

**Prerequisites**:
- Frontend development (JavaScript/TypeScript, D3.js or WebGL)
- Python/Jupyter widget development
- Understanding of network layout algorithms

**Estimated Effort**: 4-6 months

**Impact**: HIGH — Visualization is essential for understanding network models; current solutions require manual integration.

**Deliverables**:
- Lutufi-viz package
- Jupyter extension
- Documentation and examples

---

### TOOL-002: Domain-Specific Language for Causal Queries

**Description**: Design and implement a declarative DSL for specifying causal queries that compiles to Lutufi operations. Enables expressive queries without deep programming knowledge.

**Example**:
```
EFFECT OF intervention(treatment = 1) ON outcome
    GIVEN age > 50
    ADJUSTING FOR income, education
    IN population WHERE region = "urban"
```

**Prerequisites**:
- Parser/compiler construction
- API design
- Understanding of causal inference semantics

**Estimated Effort**: 3-6 months

**Impact**: MEDIUM — Lowers barrier to entry for non-programmers.

**Deliverables**:
- Query parser and compiler
- Python integration
- Documentation and tutorial

---

### TOOL-003: Benchmark Suite and Leaderboard

**Description**: Create a comprehensive benchmark suite for network inference algorithms with public leaderboard. Similar to MLPerf or Kaggle but focused on probabilistic inference on networks.

**Components**:
- Standardized problem instances across domains
- Reference implementations and baselines
- Evaluation metrics and protocols
- Public leaderboard with results

**Prerequisites**:
- Benchmark design expertise
- Infrastructure for hosting
- Community outreach

**Estimated Effort**: 6-9 months

**Impact**: HIGH — Would drive algorithm development and provide standard evaluation.

**Deliverables**:
- Benchmark website and infrastructure
- Initial problem suite
- Baseline implementations
- Research paper establishing benchmarks

---

### TOOL-004: Model Repository and Hub

**Description**: Create a repository for sharing Lutufi models, similar to Hugging Face model hub. Enables reproducibility and model reuse across research groups.

**Features**:
- Model versioning and metadata
- Search and discovery
- Community contributions
- Integration with serialization format

**Prerequisites**:
- Web development
- Database design
- Community management

**Estimated Effort**: 4-6 months

**Impact**: MEDIUM — Facilitates reproducibility and model sharing.

**Deliverables**:
- Model hub website
- API for uploading/downloading
- Integration with Lutufi

---

### TOOL-005: Educational Tutorial Platform

**Description**: Create interactive tutorials for learning probabilistic network analysis. JupyterBook-based with executable examples, conceptual explanations, and exercises.

**Content**:
- Introduction to Bayesian networks
- Network science fundamentals
- Causal inference tutorial
- Domain-specific tutorials (finance, epidemiology, social science)
- Advanced topics (structure learning, approximate inference)

**Prerequisites**:
- Technical writing
- Teaching/pedagogy experience
- JupyterBook or similar platform

**Estimated Effort**: 6-12 months

**Impact**: MEDIUM — Essential for adoption by new users.

**Deliverables**:
- Tutorial website
- 10+ complete tutorials
- Exercise sets with solutions
- Video lectures (optional)

---

## Application Projects

### APP-001: Financial Systemic Risk Dashboard

**Description**: Build a real-time or near-real-time dashboard for monitoring systemic risk in financial networks using Lutufi. Integrate with regulatory data feeds.

**Components**:
- Data pipeline from regulatory sources
- Network construction from exposures
- Real-time contagion simulation
- Alert system for vulnerability thresholds
- Visualization and reporting

**Prerequisites**:
- Financial data access (or synthetic data for prototype)
- Regulatory domain knowledge
- Real-time system development

**Estimated Effort**: 12-18 months

**Impact**: VERY HIGH — Direct application to financial stability; potential regulator adoption.

**Deliverables**:
- Operational dashboard
- Research paper on methodology
- Regulatory engagement report

---

### APP-002: Misinformation Spread Simulator

**Description**: Develop a simulator for misinformation spread on social networks that accounts for platform-specific features (Twitter/X, Facebook, WhatsApp). Enable scenario testing for intervention strategies.

**Components**:
- Platform-specific network models
- Multi-state information propagation (true/false/mixed)
- Intervention modeling (fact-checking, content moderation)
- Scenario comparison tools
- Policy recommendation engine

**Prerequisites**:
- Social media data access (API or public datasets)
- Epidemiological modeling background
- Policy analysis experience

**Estimated Effort**: 12-18 months

**Impact**: HIGH — Important for public health and democratic discourse.

**Deliverables**:
- Working simulator
- Validation against real outbreaks
- Policy recommendation report
- Open-source release

---

### APP-003: Supply Chain Resilience Optimizer

**Description**: Build a tool for analyzing and optimizing supply chain resilience using network models. Identify critical nodes, evaluate redundancy, and simulate disruption scenarios.

**Components**:
- Supply chain network builder from procurement data
- Resilience metrics (connectivity, redundancy, flexibility)
- Disruption simulation (natural disasters, supplier failure)
- Optimization for resilience under cost constraints

**Prerequisites**:
- Supply chain domain knowledge
- Optimization expertise
- Industry partnerships for data

**Estimated Effort**: 9-12 months

**Impact**: MEDIUM-HIGH — Relevant post-pandemic supply chain concerns.

**Deliverables**:
- Optimization tool
- Case studies with industry partners
- Research paper

---

### APP-004: Epidemiological Intervention Planner

**Description**: Develop a tool for planning public health interventions (vaccination, social distancing) using network-based epidemic models. Integrate contact network data.

**Components**:
- Contact network construction from mobility/survey data
- SEIR/SEIRS models on networks
- Intervention optimization (who to vaccinate, where to close)
- Uncertainty quantification
- Policy dashboard

**Prerequisites**:
- Epidemiological modeling expertise
- Public health domain knowledge
- Health data access or partnerships

**Estimated Effort**: 12-18 months

**Impact**: VERY HIGH — Direct public health application; lessons from COVID-19.

**Deliverables**:
- Planning tool
- Validation with historical outbreaks
- Public health stakeholder engagement

---

### APP-005: Organizational Network Analysis Suite

**Description**: Comprehensive tool for organizational network analysis: map communication patterns, identify silos and bottlenecks, simulate change interventions.

**Components**:
- Email/calendar data integration (privacy-preserving)
- Network metrics for organizations (density, hierarchy, clustering)
- Intervention simulation (reorganization, collaboration tools)
- Change management recommendations

**Prerequisites**:
- Organizational behavior expertise
- Privacy and HR data experience
- Corporate partnerships

**Estimated Effort**: 9-12 months

**Impact**: MEDIUM — Large market for organizational analytics tools.

**Deliverables**:
- Analysis suite
- Case studies
- Commercialization plan (if applicable)

---

## Collaboration Opportunities

### COLLAB-001: Academic Partnership — Probabilistic Programming

**Description**: Collaboration with probabilistic programming groups (Stan, PyMC, Turing.jl) to integrate Lutufi's network-specific inference with general probabilistic programming frameworks.

**Activities**:
- Joint workshops or summer schools
- Co-supervised student projects
- Shared benchmark development
- Cross-citation and methodology sharing

**Potential Partners**:
- Stan Development Team
- PyMC Developers
- Turing.jl (Julia)

**Timeline**: Ongoing

**Contact**: Open to discussion

---

### COLLAB-002: Industry Partnership — Financial Services

**Description**: Partnership with banks or regulators to validate and deploy financial network models. Access to confidential data enables real-world validation.

**Activities**:
- Pilot studies with anonymized data
- Co-development of regulatory tools
- Joint research publications
- Training and knowledge transfer

**Potential Partners**:
- Central banks (stress testing divisions)
- Systemically important banks
- Financial regulatory agencies

**Timeline**: 12-24 months initial engagement

**Value**: Essential for validating financial contagion models; access to ground truth data.

---

### COLLAB-003: Open Source Collaboration — NetworkX

**Description**: Collaboration with NetworkX developers to ensure interoperability and potentially share algorithms. Lutufi focuses on probabilistic inference; NetworkX on graph algorithms.

**Activities**:
- Shared data format standards
- Algorithm porting where appropriate
- Joint documentation and tutorials
- Community cross-pollination

**Potential Partners**:
- NetworkX core team
- SciPy/NumPy community

**Timeline**: Ongoing

**Value**: NetworkX has large user base; interoperability extends Lutufi's reach.

---

### COLLAB-004: Research Network — African Data Science

**Description**: Build capacity for network analysis in African universities and research institutions. Training, curriculum development, and collaborative research.

**Activities**:
- Workshops at African universities
- Curriculum integration
- Collaborative research projects
- Student exchange programs

**Potential Partners**:
- African Institute for Mathematical Sciences (AIMS)
- Regional universities
- African data science initiatives

**Timeline**: 24-36 months

**Value**: Capacity building; diverse applications; author mission alignment.

---

## Student Projects

### STUDENT-001: Bachelor's Thesis — Comparing Network Layout Algorithms for Causal Graphs

**Description**: Evaluate different graph layout algorithms (force-directed, hierarchical, spectral) for visualizing causal Bayesian networks. Focus on readability and preservation of causal semantics.

**Research Questions**:
- Which layout algorithms best preserve causal flow?
- How does layout affect user understanding of causal relationships?
- Can we design layout objectives specifically for causal graphs?

**Prerequisites**:
- Graph algorithms
- Visualization
- User study design (optional)

**Timeline**: 6 months

**Supervision**: Computer science or information visualization faculty

---

### STUDENT-002: Master's Thesis — GPU Acceleration of Belief Propagation

**Description**: Implement and evaluate GPU-accelerated belief propagation for factor graphs. Compare speedups across different graph structures and problem sizes.

**Research Questions**:
- What speedup does GPU acceleration provide for BP?
- Which graph structures benefit most from GPU acceleration?
- What are the memory bottlenecks?

**Prerequisites**:
- CUDA or OpenCL programming
- Parallel algorithms
- Understanding of belief propagation

**Timeline**: 12 months

**Supervision**: Computer science or computational science faculty

---

### STUDENT-003: Master's Thesis — Structure Learning Benchmark Dataset

**Description**: Create a comprehensive benchmark dataset for evaluating causal discovery algorithms. Include ground truth networks, realistic data generation, and diverse problem characteristics.

**Research Questions**:
- What network characteristics affect structure learning difficulty?
- How can we generate realistic synthetic networks?
- What evaluation metrics best capture algorithm performance?

**Prerequisites**:
- Graph theory
- Statistical methodology
- Programming

**Timeline**: 12 months

**Supervision**: Statistics or machine learning faculty

---

### STUDENT-004: PhD Dissertation — Causal Inference on Temporal Networks

**Description**: Develop methodology for causal inference when both the network and node attributes evolve over time. Address time-varying confounding and network formation.

**Research Questions**:
- How does network evolution affect causal identification?
- What assumptions enable causal inference on dynamic networks?
- How can we estimate time-varying treatment effects?

**Prerequisites**:
- Causal inference
- Temporal network analysis
- Advanced statistical theory

**Timeline**: 3-4 years

**Supervision**: Methodology faculty with network and causal expertise

---

### STUDENT-005: PhD Dissertation — Robustness of Network Causal Estimates

**Description**: Develop theory and methods for assessing robustness of causal estimates to network misspecification, measurement error, and model violations.

**Research Questions**:
- How sensitive are network causal estimates to measurement error?
- Can we develop bounds under network uncertainty?
- What diagnostics detect model misspecification?

**Prerequisites**:
- Causal inference
- Robust statistics
- Network analysis

**Timeline**: 3-4 years

**Supervision**: Statistics or econometrics faculty

---

## Grant Opportunities

### GRANT-001: NSF Division of Information and Intelligent Systems

**Program**: Robust Intelligence  
**Fit**: Development of robust inference algorithms for complex networks  
**Budget**: $500K-$1.2M over 3-4 years  
**Deadline**: Annually (October)  
**Requirements**: US institution or collaboration  
**Contact Strategy**: Develop preliminary results; identify US collaborator

---

### GRANT-002: ERC Starting Grant

**Program**: Starting Grant (any panel)  
**Fit**: Novel methodology for causal inference on networks  
**Budget**: €1.5M over 5 years  
**Deadline**: Annually (varies by year)  
**Requirements**: EU host institution, 2-7 years post-PhD  
**Contact Strategy**: Develop proposal with EU-based PI

---

### GRANT-003: Wellcome Trust Data Science

**Program**: Early-Career Awards in Data Science  
**Fit**: Network methods for public health and epidemiology  
**Budget**: £300K-£500K over 3 years  
**Deadline**: Rolling/annual  
**Requirements**: Health application focus  
**Contact Strategy**: Partner with epidemiology group; develop health application

---

### GRANT-004: Chan Zuckerberg Initiative

**Program**: Essential Open Source Software for Science  
**Fit**: Open source development for scientific computing  
**Budget**: $50K-$200K over 1-2 years  
**Deadline**: Annual  
**Requirements**: Open source; scientific community  
**Contact Strategy**: Demonstrate community adoption; sustainability plan

---

### GRANT-005: Sloan Foundation

**Program**: Research in Economics  
**Fit**: Economic network applications  
**Budget**: $100K-$500K  
**Deadline**: Rolling  
**Requirements**: Economic research focus  
**Contact Strategy**: Develop economic application; collaborate with economists

---

### GRANT-006: Industry Research Grants

**Programs**:
- Google Research Scholar Program
- Microsoft Research PhD Fellowship
- Meta Research Awards

**Fit**: Scalable inference, causal ML, network analysis  
**Budget**: $50K-$150K  
**Deadline**: Varies  
**Requirements**: Research alignment with company interests  
**Contact Strategy**: Demonstrate technical innovation; potential for collaboration

---

## Project Format

All projects in this document follow this standard format:

| Element | Description |
|---------|-------------|
| **ID** | Unique identifier (e.g., RES-001, TOOL-001) |
| **Description** | What the project entails |
| **Research Questions** | (Research projects) Key questions addressed |
| **Features** | (Tool projects) Key capabilities |
| **Components** | (Application projects) Major parts |
| **Activities** | (Collaboration projects) What will be done |
| **Prerequisites** | Required background and resources |
| **Estimated Effort** | Duration and FTE requirements |
| **Impact** | Expected contribution (VERY HIGH/HIGH/MEDIUM/LOW) |
| **Deliverables** | Tangible outputs |
| **Timeline** | (Grants, Collaborations) Schedule |

---

## Priority Rankings

### Immediate (0-12 months)

Projects that should be started now:

1. **TOOL-005**: Educational Tutorial Platform — Essential for adoption
2. **TOOL-001**: Interactive Visualization Module — High user demand
3. **STUDENT-001, 002, 003**: Bachelor's and Master's projects — Build capacity
4. **GRANT-004**: Chan Zuckerberg Initiative — Fund open source development

### Near-Term (12-24 months)

Projects for the next phase:

1. **RES-001**: Comparative Evaluation of Inference Algorithms — Validates core functionality
2. **TOOL-003**: Benchmark Suite and Leaderboard — Drives research
3. **APP-002**: Misinformation Spread Simulator — High societal impact
4. **COLLAB-002**: Financial Industry Partnership — Validates domain applications
5. **GRANT-001**: NSF Robust Intelligence — Major research funding

### Future (24+ months)

Longer-term projects requiring significant resources:

1. **RES-002**: Causal Discovery with Network Interference — Major research contribution
2. **RES-003**: Scalable Structure Learning — Enables large-scale applications
3. **APP-001**: Financial Systemic Risk Dashboard — Full deployment
4. **APP-004**: Epidemiological Intervention Planner — Public health system
5. **STUDENT-004, 005**: PhD Dissertations — Fundamental advances

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-03-04 | Initial project catalog with 20+ projects across 5 categories |

---

## Contributing

To propose additional projects:

1. Follow the project format specified above
2. Include clear prerequisites and deliverables
3. Assess priority based on strategic alignment
4. Submit via the project issue tracker

Projects that demonstrate Lutufi's capabilities, advance the methodology, or serve user needs are welcome.
