# Example Library Structure for Lutufi

**Document Version:** 1.0  
**Status:** Working Draft  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Purpose of Examples](#purpose-of-examples)
2. [Example Categories](#example-categories)
3. [Directory Structure](#directory-structure)
4. [Example Components](#example-components)
5. [Data for Examples](#data-for-examples)
6. [Documentation Integration](#documentation-integration)
7. [Testing Examples](#testing-examples)
8. [Jupyter Notebook Examples](#jupyter-notebook-examples)
9. [Domain-Specific Collections](#domain-specific-collections)
10. [Example Quality Standards](#example-quality-standards)
11. [Contributing Examples](#contributing-examples)
12. [List of Priority Examples](#list-of-priority-examples)

---

## Purpose of Examples

Examples serve multiple critical functions in the success of a scientific software library like Lutufi. They are not merely demonstrations but essential components of the user experience and learning pathway.

### Why Examples Matter

**For Adoption:**
- **First impressions:** Examples are often the first thing users try
- **Proof of concept:** Working code proves the library functions
- **Trust building:** Successful examples build confidence in the software
- **Viral potential:** Impressive examples get shared in communities

**For Learning:**
- **Concrete starting point:** Users learn best from working code
- **Pattern recognition:** Examples show common usage patterns
- **Progressive complexity:** Structured examples enable skill building
- **Reference material:** Users return to examples when implementing

**For Credibility:**
- **Reproducibility:** Examples demonstrate reproducible research
- **Validation:** Working examples validate software claims
- **Domain relevance:** Domain examples prove utility in real contexts
- **Benchmarking:** Examples serve as performance benchmarks

**For Maintenance:**
- **Regression tests:** Examples catch breaking changes
- **Documentation sync:** Examples verify documentation accuracy
- **API evolution:** Examples guide API refinement

### The Psychology of Examples

Research on software learning shows that users typically follow this path:

1. **Discovery:** Find the library through search or recommendation
2. **Quick test:** Try the simplest example (5-minute test)
3. **Exploration:** Try examples related to their domain
4. **Adaptation:** Modify examples for their use case
5. **Creation:** Write original code using library

Examples must support each stage of this journey.

---

## Example Categories

Examples should be organized along multiple dimensions to serve different user needs.

### By Domain

Domain examples demonstrate Lutufi's utility in specific research areas:

**Epidemiology:**
- Disease spread modeling
- Contact tracing analysis
- Intervention effectiveness
- SEIR models on networks

**Finance:**
- Systemic risk assessment
- Financial contagion modeling
- Credit risk networks
- Market correlation networks

**Social Science:**
- Opinion dynamics
- Influence maximization
- Community detection with uncertainty
- Diffusion of innovations

**Intelligence/Security:**
- Dark network analysis
- Misinformation propagation
- Threat assessment
- Covert network detection

### By Method

Method examples focus on specific techniques:

**Inference:**
- Exact inference on trees
- Loopy belief propagation
- Gibbs sampling
- Variational inference
- MAP estimation

**Learning:**
- Parameter learning from data
- Structure learning algorithms
- Hybrid learning approaches
- Online learning

**Causal Analysis:**
- Causal effect estimation
- Intervention analysis
- Counterfactual reasoning
- Causal discovery

### By Complexity

**Basic (Beginner):**
- 5-20 lines of code
- Single concept demonstrated
- No external data required
- Runs in seconds
- Extensive comments

**Intermediate:**
- 50-200 lines of code
- Multiple concepts combined
- Small dataset included
- Runs in minutes
- Moderate comments

**Advanced:**
- 200+ lines of code
- Complex workflows
- Real or realistic data
- May take hours
- Strategic comments

---

## Directory Structure

A well-organized directory structure makes examples discoverable and maintainable.

```
examples/
├── README.md                    # Overview of examples
├── requirements.txt             # Common dependencies
├── basic/                       # Beginner examples
│   ├── README.md
│   ├── 01_hello_network.py
│   ├── 02_simple_inference.py
│   ├── 03_discrete_distributions.py
│   └── 04_conditional_probability.py
├── intermediate/                # Intermediate examples
│   ├── README.md
│   ├── inference/
│   │   ├── loopy_bp_example.py
│   │   ├── gibbs_sampling.py
│   │   └── variational_inference.py
│   ├── networks/
│   │   ├── custom_topology.py
│   │   ├── temporal_networks.py
│   │   └── multilayer_example.py
│   └── learning/
│       ├── parameter_learning.py
│       └── structure_learning.py
├── advanced/                    # Advanced examples
│   ├── README.md
│   ├── large_scale_inference.py
│   ├── custom_inference_engine.py
│   └── performance_benchmarking.py
├── domains/                     # Domain-specific examples
│   ├── epidemiology/
│   │   ├── README.md
│   │   ├── seir_basic.py
│   │   ├── contact_tracing.py
│   │   └── intervention_scenarios.py
│   ├── finance/
│   │   ├── README.md
│   │   ├── systemic_risk.py
│   │   ├── contagion_model.py
│   │   └── stress_testing.py
│   ├── social_science/
│   │   ├── README.md
│   │   ├── opinion_dynamics.py
│   │   ├── influence_maximization.py
│   │   └── diffusion_analysis.py
│   └── intelligence/
│       ├── README.md
│       ├── dark_networks.py
│       └── misinformation.py
├── notebooks/                   # Jupyter notebooks
│   ├── basic_tutorial.ipynb
│   ├── epidemiology_walkthrough.ipynb
│   ├── finance_case_study.ipynb
│   └── api_overview.ipynb
├── data/                        # Example datasets
│   ├── README.md
│   ├── synthetic/              # Generated data
│   └── real/                   # Public datasets
└── gallery/                     # Gallery examples (for docs)
    ├── generate_gallery.py
    └── thumbnails/
```

---

## Example Components

Every example should contain specific components to maximize utility.

### Required Components

**1. Problem Statement**
- Clear description of the problem being solved
- Domain context
- Key concepts demonstrated

**2. Dependencies and Imports**
- All imports at the top
- Version checks where relevant
- Clear comments on dependencies

**3. Data Description**
- Source of data
- Data characteristics
- How data was prepared

**4. Implementation Code**
- Clear, commented code
- Progressive building of the solution
- Best practices demonstrated

**5. Expected Output**
- What the code produces
- How to interpret results
- Expected numerical values or ranges

**6. Visualization/Results**
- Plots where appropriate
- Tables of results
- Clear labeling

**7. Interpretation**
- What the results mean
- Domain implications
- How to extend the analysis

**8. References**
- Academic sources
- Related examples
- Further reading

---

## Data for Examples

Data is crucial for meaningful examples.

### Data Sources

**Real Data (Preferred when available):**
- Network Repository (networkrepository.com)
- SNAP (Stanford Large Network Dataset Collection)
- KONECT (Koblenz Network Collection)
- SocioPatterns (contact networks)

**Synthetic Data:**
- Generated with known properties
- Controlled experiments
- Scalable testing

### Data Licensing

- Only use data with permissive licenses
- Document license for each dataset
- Provide attribution as required
- Respect usage restrictions

### Data Storage

- Small datasets (< 1 MB): Include in repository
- Medium datasets (1-100 MB): Git LFS or separate download
- Large datasets (> 100 MB): External download

---

## Documentation Integration

Examples should be tightly integrated with documentation.

### Sphinx Gallery

Use Sphinx-Gallery to generate gallery pages from examples.

### Notebook Integration

Convert notebooks to documentation using nbsphinx.

### Cross-Referencing

Link examples to relevant documentation sections.

---

## Testing Examples

Examples must be tested to ensure they keep working.

### Testing Strategy

**Level 1: Smoke Tests**
- Import all example modules
- Check for syntax errors
- Fast (< 5 minutes total)

**Level 2: Execution Tests**
- Run each example
- Verify no exceptions
- Moderate time (< 30 minutes)

**Level 3: Output Verification**
- Run examples
- Verify specific outputs
- Slower (< 2 hours)

### CI Integration

Run example tests in continuous integration to catch regressions.

---

## Jupyter Notebook Examples

Notebooks provide an interactive learning environment.

### Notebook Structure

1. **Header Cell (Markdown):** Title, author, objectives
2. **Setup Cell:** Install and import
3. **Concept Introduction:** Background explanation
4. **Implementation Cells:** Working code
5. **Visualization Cells:** Plots and outputs
6. **Exercise Cells:** Practice opportunities

### Binder Integration

Enable one-click launch with Binder badges.

### Google Colab

Provide Colab links for cloud execution.

---

## Domain-Specific Collections

Curated collections help domain experts find relevant content.

### Epidemiology Collection

```
examples/domains/epidemiology/
├── README.md                    # Collection overview
├── 01_basic_seir.py            # Basic SEIR model
├── 02_network_seir.py          # Network-based spread
├── 03_interventions.py         # Vaccination, quarantine
├── 04_contact_tracing.py       # Contact tracing analysis
├── 05_variant_modeling.py      # Multiple variants
├── 06_uncertainty_quant.py     # Uncertainty analysis
├── data/                       # Example datasets
└── notebooks/                  # Interactive tutorials
```

### Finance Collection

```
examples/domains/finance/
├── README.md
├── 01_systemic_risk_basic.py
├── 02_contagion_dynamics.py
├── 03_stress_testing.py
├── 04_credit_networks.py
└── data/
    └── synthetic_banking_data/
```

### Social Science Collection

```
examples/domains/social_science/
├── README.md
├── 01_opinion_dynamics.py
├── 02_influence_maximization.py
├── 03_diffusion_analysis.py
├── 04_community_detection.py
└── data/
    └── social_network_datasets/
```

---

## Example Quality Standards

Establish clear standards for example quality.

### Code Quality

- Follow PEP 8
- Use clear variable names
- Include type hints where helpful
- Keep functions small and focused

### Documentation

- Docstrings for all functions
- Inline comments explaining "why," not "what"
- Module-level docstrings

### Correctness

- Code runs without errors
- Results are verified
- Edge cases handled

### Review Checklist

Before adding an example:
- [ ] Problem statement clear?
- [ ] Code runs without errors?
- [ ] Output documented?
- [ ] Interpretation provided?
- [ ] References included?
- [ ] Follows style guide?
- [ ] Has been reviewed?
- [ ] Tested in CI?

---

## Contributing Examples

Community-contributed examples expand coverage.

### Contribution Process

1. **Proposal:** Open issue describing proposed example
2. **Development:** Follow example structure
3. **Submission:** Pull request with example
4. **Review:** Code and educational review

### Contribution Guidelines

- Original work or properly licensed
- Follows example standards
- Includes documentation
- Passes tests

### Recognition

- Contributors listed in example header
- Acknowledged in changelog
- Listed in contributors file

---

## List of Priority Examples

This section lists essential examples to create first, organized by priority.

### Priority 1: Core Examples (Essential for Release)

**1. Hello Network (5 minutes)**
```
File: examples/basic/01_hello_network.py
Difficulty: Basic
Concepts: Installation check, basic imports, create simple network
Purpose: Verify installation works
```

**2. Simple Inference (10 minutes)**
```
File: examples/basic/02_simple_inference.py
Difficulty: Basic
Concepts: Create model, add nodes, run inference, get results
Purpose: Core API demonstration
```

**3. Discrete Distributions (15 minutes)**
```
File: examples/basic/03_discrete_distributions.py
Difficulty: Basic
Concepts: Bernoulli, categorical, conditional distributions
Purpose: Distribution system overview
```

**4. Loopy Belief Propagation (20 minutes)**
```
File: examples/intermediate/inference/loopy_bp_example.py
Difficulty: Intermediate
Concepts: Cyclic networks, approximate inference, convergence
Purpose: Show key differentiator from other libraries
```

**5. SEIR Epidemiology Model (30 minutes)**
```
File: examples/domains/epidemiology/seir_basic.py
Difficulty: Intermediate
Concepts: SEIR model, network spread, temporal dynamics
Purpose: Domain demonstration - epidemiology
```

**6. Systemic Risk Analysis (30 minutes)**
```
File: examples/domains/finance/systemic_risk.py
Difficulty: Intermediate
Concepts: Financial networks, contagion, stress testing
Purpose: Domain demonstration - finance
```

### Priority 2: Important Examples (Post-Release)

**7. Opinion Dynamics (30 minutes)**
```
File: examples/domains/social_science/opinion_dynamics.py
Difficulty: Intermediate
Concepts: Voter model, influence, consensus
Purpose: Domain demonstration - social science
```

**8. Parameter Learning (45 minutes)**
```
File: examples/intermediate/learning/parameter_learning.py
Difficulty: Intermediate
Concepts: Maximum likelihood, Bayesian learning, EM algorithm
Purpose: Learning capabilities demonstration
```

**9. Structure Learning (45 minutes)**
```
File: examples/intermediate/learning/structure_learning.py
Difficulty: Intermediate
Concepts: Constraint-based, score-based, hybrid methods
Purpose: Structure learning demonstration
```

**10. Temporal Networks (45 minutes)**
```
File: examples/intermediate/networks/temporal_networks.py
Difficulty: Intermediate
Concepts: Dynamic graphs, temporal inference, evolving networks
Purpose: Temporal capabilities
```

**11. Multi-Layer Networks (45 minutes)**
```
File: examples/intermediate/networks/multilayer_example.py
Difficulty: Intermediate
Concepts: Layer coupling, cross-layer inference, multiplex networks
Purpose: Multi-layer capabilities
```

**12. Dark Network Analysis (60 minutes)**
```
File: examples/domains/intelligence/dark_networks.py
Difficulty: Advanced
Concepts: Incomplete data, hidden nodes, covert structure
Purpose: Intelligence/security applications
```

### Priority 3: Extended Examples (Ongoing)

**13. Causal Inference (60 minutes)**
```
File: examples/intermediate/causal/causal_inference.py
Difficulty: Advanced
Concepts: Do-calculus, interventions, counterfactuals
Purpose: Causal analysis capabilities
```

**14. Large-Scale Inference (60 minutes)**
```
File: examples/advanced/large_scale_inference.py
Difficulty: Advanced
Concepts: Scalability, approximate methods, performance
Purpose: Demonstrate scalability
```

**15. Custom Inference Engine (90 minutes)**
```
File: examples/advanced/custom_inference_engine.py
Difficulty: Advanced
Concepts: Extending Lutufi, custom algorithms, plugin architecture
Purpose: Extension capabilities
```

**16. COVID-19 Case Study (90 minutes)**
```
File: examples/domains/epidemiology/covid_case_study.py
Difficulty: Advanced
Concepts: Real data, calibration, forecasting, validation
Purpose: Realistic application
```

**17. Financial Contagion Stress Test (90 minutes)**
```
File: examples/domains/finance/contagion_stress_test.py
Difficulty: Advanced
Concepts: Realistic banking network, multiple scenarios, policy analysis
Purpose: Realistic application
```

**18. Misinformation Campaign Analysis (90 minutes)**
```
File: examples/domains/intelligence/misinformation.py
Difficulty: Advanced
Concepts: Information operations, detection, mitigation
Purpose: Complex security application
```

**19. Integration with NetworkX (30 minutes)**
```
File: examples/intermediate/networks/networkx_integration.py
Difficulty: Intermediate
Concepts: Interoperability, ecosystem integration
Purpose: Show ecosystem compatibility
```

**20. Integration with PyMC (30 minutes)**
```
File: examples/intermediate/inference/pymc_integration.py
Difficulty: Intermediate
Concepts: MCMC methods, probabilistic programming integration
Purpose: Show ecosystem compatibility
```

### Example Development Schedule

| Month | Examples |
|-------|----------|
| 1-2 | 1-4 (Core basics) |
| 3-4 | 5-6 (Domain basics) |
| 5-6 | 7-12 (Extended coverage) |
| 7-12 | 13-20 (Advanced examples) |
| 12+ | Community contributions |

---

## Conclusion

Examples are not optional add-ons—they are essential components of Lutufi's success. This structure provides:

1. **Clear organization** for finding relevant examples
2. **Progressive complexity** for skill building
3. **Domain coverage** for diverse users
4. **Quality standards** for maintainability
5. **Integration** with documentation and testing

Following this structure ensures examples serve their purpose: accelerating user adoption, building credibility, and demonstrating Lutufi's value across research domains.
