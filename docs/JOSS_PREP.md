# JOSS Submission Preparation: Lutufi

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [What is JOSS](#what-is-joss)
2. [Submission Requirements](#submission-requirements)
3. [The paper.md File](#the-papermd-file)
4. [Statement of Need](#statement-of-need)
5. [Software Repository Checklist](#software-repository-checklist)
6. [Review Process](#review-process)
7. [Pre-Submission Preparation](#pre-submission-preparation)
8. [Reviewer Selection](#reviewer-selection)
9. [Responding to Reviews](#responding-to-reviews)
10. [Post-Acceptance](#post-acceptance)
11. [JOSS Paper Outline](#joss-paper-outline)
12. [Timeline](#timeline)

---

## What is JOSS

The Journal of Open Source Software (JOSS) is an academic journal that provides a publication venue for research software packages. Unlike traditional journals that focus on novel research findings, JOSS focuses on the software itself as a research output worthy of academic recognition.

### The Purpose of JOSS

JOSS addresses a critical gap in academic incentives: software developers rarely receive formal academic credit for their work, despite software being fundamental to modern research. By providing a peer-reviewed publication venue with a DOI, JOSS enables developers to:

- **Receive academic credit** for software in hiring, promotion, and tenure decisions
- **Increase software visibility** through indexing in academic databases
- **Establish scholarly credibility** through peer review
- **Enable citation** of software in academic papers
- **Document software** for long-term sustainability

### Why Submit Lutufi to JOSS

Lutufi is an ideal candidate for JOSS publication because:

- **Fills a clear gap:** No existing library unifies Bayesian networks with structural network analysis
- **Substantial development:** The library represents thousands of lines of code across multiple domains
- **Research applications:** Direct applications in epidemiology, finance, social science, and intelligence analysis
- **Open source commitment:** Released under Apache 2.0 with comprehensive documentation

### Credibility Value of JOSS Publication

A JOSS publication provides multiple credibility benefits:

**Academic Recognition:**
- Indexed in Google Scholar and other academic databases
- Citable with a DOI
- Recognized by hiring and promotion committees
- Formal peer review provides quality validation

**Community Trust:**
- Signals software maturity and reliability
- Demonstrates commitment to maintenance
- Provides independent quality assessment
- Increases adoption among cautious researchers

---

## Submission Requirements

JOSS has specific requirements that must be met before submission:

### Software License Requirement

JOSS requires an OSI-approved open source license. Lutufi uses Apache 2.0, which satisfies this requirement.

**License Requirements:**
- Must be included in the repository (LICENSE file)
- Must be mentioned in the README
- Must be compatible with dependencies
- Should be applied to all source files

**Lutufi License Compliance:**
- [x] LICENSE file present at repository root
- [x] Apache 2.0 is OSI-approved
- [x] README mentions license
- [ ] Add license headers to all source files (pre-submission task)

### Documentation Requirements

**README Requirements:**
- Clear statement of need (what problem does it solve?)
- Installation instructions
- Minimal example of usage
- Information on how to get support
- License information
- Citation information

**API Documentation:**
- Must be automatically generated from docstrings
- Must be hosted and accessible (e.g., ReadTheDocs)
- Must cover all public APIs

### Test Requirements

JOSS requires automated tests to ensure software reliability:

**Test Suite Requirements:**
- Automated tests that can be run with a single command
- Tests should cover core functionality
- Continuous integration (CI) is strongly recommended
- Aim for >80% code coverage

### Repository Structure

```
lutufi/
├── LICENSE                 # License file
├── README.md              # Project overview
├── paper.md               # JOSS paper
├── setup.py/pyproject.toml # Installation configuration
├── src/                   # Source code
├── tests/                 # Test suite
├── docs/                  # Documentation
├── examples/              # Example code
└── .github/               # GitHub templates, CI config
```

---

## The paper.md File

The `paper.md` file is the core of the JOSS submission. It describes the software and its purpose in a concise, scholarly format.

### Structure

```markdown
---
title: 'Lutufi: A Unified Library for Probabilistic Inference on Social and Economic Networks'
tags:
  - Python
  - Bayesian networks
  - network analysis
  - probabilistic graphical models
authors:
  - name: Wasswa Lutufi Sebbanja
    orcid: 0000-0000-0000-0000
    affiliation: 1
affiliations:
  - name: Independent Researcher
    index: 1
date: 4 March 2026
bibliography: paper.bib
---

# Summary

[250-500 words summarizing the software]

# Statement of Need

[Explanation of the problem and how the software solves it]

# Key Features

[Major capabilities and innovations]

# Example Usage

[Brief code example showing key functionality]

# References
```

### Length Requirements

JOSS papers are intentionally short:
- **Minimum:** 250 words (approximately half a page)
- **Maximum:** 1000 words (approximately 2 pages)
- **Typical:** 500-750 words (approximately 1 page)

---

## Statement of Need

The Statement of Need is the most critical section of the JOSS paper. It must clearly articulate why the software is necessary and what gap it fills.

### Writing a Compelling Statement of Need

**Step 1: Establish the Problem Domain**

Probabilistic reasoning and network analysis are fundamental to understanding complex systems across domains including epidemiology, finance, and social science. Bayesian networks provide rigorous frameworks for probabilistic inference, while network science offers tools for analyzing relational structures.

**Step 2: Identify the Gap**

Despite their complementary strengths, these approaches remain fragmented in software implementations. Existing Bayesian network libraries (pgmpy, bnlearn) focus on directed acyclic graphs and lack structural network analysis capabilities. Conversely, network analysis libraries (NetworkX, graph-tool) handle complex topologies but lack native probabilistic inference.

**Step 3: Present the Solution**

Lutufi addresses this gap by providing a unified library for probabilistic inference on complex networks. It integrates Bayesian network methods with structural network analysis, supporting cyclic graphs, dynamic processes, and multi-layer networks within a consistent API.

### Lutufi Statement of Need Draft

```markdown
# Statement of Need

Probabilistic graphical models and network analysis are essential tools for 
understanding complex systems, yet they remain fragmented in software ecosystems. 
Bayesian network libraries such as pgmpy and bnlearn provide sophisticated 
inference algorithms but are limited to directed acyclic graphs and lack 
structural network analysis capabilities. Conversely, network analysis libraries 
such as NetworkX and graph-tool excel at topology analysis but do not provide 
native probabilistic inference.

This fragmentation creates significant barriers for research domains requiring 
both probabilistic reasoning and complex network structures. Epidemiologists 
modeling disease spread on contact networks, financial analysts assessing systemic 
risk through interbank networks, and social scientists studying opinion dynamics 
all require unified probabilistic-network models. Currently, these researchers 
must integrate multiple libraries manually, resulting in brittle code, 
reproducibility challenges, and steep learning curves.

Lutufi addresses this gap by providing a unified library for probabilistic 
inference on social and economic networks. It combines Bayesian network methods 
with structural network analysis within a consistent API, supporting cyclic 
graphs, temporal dynamics, and multi-layer networks. The library provides 
domain-specific abstractions for epidemiology, finance, and social science while 
maintaining the flexibility needed for custom research applications.

By integrating these capabilities, Lutufi enables researchers to focus on 
scientific questions rather than software integration, accelerating research 
and improving reproducibility in computational social science.
```

---

## Software Repository Checklist

### Pre-Submission Verification

**License and Legal:**
- [x] OSI-approved license (Apache 2.0) included
- [x] License mentioned in README
- [ ] License headers in all source files
- [ ] CONTRIBUTING.md present
- [ ] CODE_OF_CONDUCT.md present

**Documentation:**
- [x] README.md with statement of need
- [x] Installation instructions
- [ ] Quickstart guide (under 5 minutes)
- [ ] Full API documentation generated
- [ ] Hosted documentation

**Testing:**
- [ ] Unit test suite with >80% coverage
- [ ] Integration tests
- [ ] CI/CD pipeline

**JOSS-Specific:**
- [ ] paper.md in repository root
- [ ] paper.bib bibliography file
- [ ] Word count between 250-1000 words

---

## Review Process

### Submission Steps

1. **Pre-Submission Inquiry (Optional):** Confirm scope fit with editors
2. **Formal Submission:** Submit through JOSS website
3. **Editorial Check:** Editor verifies completeness
4. **Reviewer Assignment:** Minimum two reviewers
5. **Review:** Iterative feedback process
6. **Acceptance:** Paper compiled, DOI minted

### Timeline

| Phase | Duration |
|-------|----------|
| Editorial check | 1-2 weeks |
| Reviewer assignment | 1-2 weeks |
| Initial review | 2-4 weeks |
| Revisions | 2-6 weeks |
| Final acceptance | 1 week |
| **Total** | **1-4 months** |

### Review Criteria

- License suitable?
- Documentation sufficient?
- Software functional?
- Tests present and passing?
- Clear contribution?
- Appropriate authorship?

---

## Pre-Submission Preparation

### Code Quality Preparation

- Run linters (pylint, flake8, black)
- Run type checker (mypy)
- Run security scanners (bandit)
- Profile critical paths

### Documentation Preparation

- Verify installation instructions work
- Check quickstart produces expected output
- Generate and review API docs
- Run all examples

### Preprint Considerations

**arXiv Submission:**
- JOSS allows preprint on arXiv before submission
- Helps establish priority
- Provides early visibility
- Recommended but not required

---

## Reviewer Selection

### Ideal Reviewer Characteristics

- **Domain Expertise:** Bayesian networks, network science, computational social science
- **Technical Expertise:** Python development, scientific computing
- **Practical:** Not conflicted, available, GitHub active

### Suggested Categories

1. **Probabilistic Graphical Models:** pgmpy contributors, PyMC developers
2. **Network Science:** NetworkX contributors, graph-tool developers
3. **Computational Social Science:** CSS researchers with software experience

### How to Suggest

Provide 3-5 reviewers with:
- Name
- GitHub handle
- Affiliation
- Expertise justification

---

## Responding to Reviews

### General Principles

- **Be Responsive:** Acknowledge comments quickly
- **Be Respectful:** Reviewers volunteer their time
- **Be Thorough:** Address every comment

### Response Template

```markdown
Thank you for this feedback. I have made the following changes:

1. [Specific change made]
2. [Specific change made]

[Link to commit showing changes]
```

---

## Post-Acceptance

### Immediate Actions

- Add JOSS badge to README
- Update citation information
- Tag release with DOI
- Announce on social media

### Citation Format

```bibtex
@article{sebbanja2026lutufi,
  title={Lutufi: A Unified Library for Probabilistic Inference on Social and Economic Networks},
  author={Sebbanja, Wasswa Lutufi},
  journal={Journal of Open Source Software},
  year={2026},
  publisher={Open Journals},
  doi={10.21105/joss.xxxxx}
}
```

---

## JOSS Paper Outline

### Proposed Title

"Lutufi: A Unified Library for Probabilistic Inference on Social and Economic Networks"

### Structure

```markdown
---
title: 'Lutufi: A Unified Library for Probabilistic Inference on Social and Economic Networks'
tags:
  - Python
  - Bayesian networks
  - network analysis
  - probabilistic graphical models
  - social networks
  - economic networks
authors:
  - name: Wasswa Lutufi Sebbanja
    orcid: 0000-0000-0000-0000
    affiliation: 1
affiliations:
  - name: Independent Researcher
    index: 1
date: 4 March 2026
bibliography: paper.bib
---

# Summary

Probabilistic graphical models and network analysis are essential tools for 
understanding complex systems across domains including epidemiology, finance, 
and social science. However, existing software implementations remain fragmented: 
Bayesian network libraries provide sophisticated inference but are limited to 
directed acyclic graphs, while network analysis libraries excel at topology 
analysis but lack native probabilistic reasoning.

Lutufi is an open-source Python library that unifies these approaches, enabling 
probabilistic inference on complex network structures including cyclic graphs, 
temporal networks, and multi-layer systems. The library provides domain-specific 
abstractions for epidemiological modeling, financial contagion analysis, and 
social influence studies while maintaining the flexibility required for custom 
research applications.

# Statement of Need

Probabilistic reasoning and network analysis are fundamental to understanding 
complex systems, yet they remain fragmented in software ecosystems. Bayesian 
network libraries such as pgmpy and bnlearn provide sophisticated inference 
algorithms but are limited to directed acyclic graphs and lack structural 
network analysis capabilities. Conversely, network analysis libraries such as 
NetworkX and graph-tool excel at topology analysis but do not provide native 
probabilistic inference.

This fragmentation creates significant barriers for research domains requiring 
both probabilistic reasoning and complex network structures. Epidemiologists, 
financial analysts, and social scientists all require unified probabilistic-
network models. Currently, these researchers must integrate multiple libraries 
manually, resulting in brittle code and reproducibility challenges.

Lutufi addresses this gap by providing a unified library for probabilistic 
inference on social and economic networks.

# Key Features

- Unified probabilistic-network modeling
- Cyclic graph support
- Temporal dynamics
- Multi-layer networks
- Domain-specific modules
- Numerical stability

# Example Usage

```python
import lutufi as lt
import networkx as nx

# Create a probabilistic network model
model = lt.ProbabilisticNetwork()
model.add_node('A', distribution='bernoulli', p=0.3)

# Add structural edges
G = nx.karate_club_graph()
model.set_topology(G)

# Run inference
result = model.infer(query=['B'], evidence={'A': 1})
```

# References
```

---

## Timeline

### Pre-Submission Phase (Months 1-6)

**Months 1-2:** Core Development
- Complete inference engine
- Basic network integration
- Initial test suite

**Months 3-4:** Documentation
- Comprehensive tutorials
- API documentation
- Quickstart guide

**Months 5-6:** Polish
- Code quality review
- Example implementations
- Paper.md writing

### Submission Phase (Months 6-8)

**Month 6:** Preparation
- Final repository review
- Tag release version

**Month 7:** Submission
- Submit to JOSS
- Editorial checks

**Month 8:** Review
- Address reviewer comments
- Final acceptance

### Recommended Submission Timing

**Target:** Month 7 (v0.9.0 or v1.0.0)
- Core functionality working
- Documentation complete
- Examples functional
- Tests passing

---

## Conclusion

JOSS publication provides significant value for Lutufi:

1. **Academic credibility** through peer review
2. **Citation pathway** for software
3. **Community visibility** in research community
4. **Sustainability signal** to potential users

Success requires meeting submission requirements, writing clear documentation, and responding professionally to reviews. The target submission date is Month 7, with acceptance expected 1-4 months after submission.
