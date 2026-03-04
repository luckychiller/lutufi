# Documentation Philosophy for Lutufi

**Document Version:** 1.0  
**Status:** Working Draft  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Why Documentation Matters](#why-documentation-matters)
2. [Documentation Types](#documentation-types)
3. [The Diátaxis Framework](#the-diátaxis-framework)
4. [Tutorial Strategy](#tutorial-strategy)
5. [How-To Guide Strategy](#how-to-guide-strategy)
6. [Explanation Strategy](#explanation-strategy)
7. [Reference Strategy](#reference-strategy)
8. [Writing Style](#writing-style)
9. [Code Examples in Documentation](#code-examples-in-documentation)
10. [Visual Documentation](#visual-documentation)
11. [Documentation Maintenance](#documentation-maintenance)
12. [User Feedback on Docs](#user-feedback-on-docs)
13. [Documentation Tools](#documentation-tools)
14. [Measuring Documentation Quality](#measuring-documentation-quality)
15. [Comparison to Other Libraries](#comparison-to-other-libraries)

---

## Why Documentation Matters

Documentation is not an afterthought—it is a core product feature that determines the success or failure of a software library.

### Documentation Drives Adoption

**First Impressions:** Users form opinions about software within minutes. Documentation quality signals overall software quality.

**Learning Curve:** Well-documented software has a gentler learning curve. Users can achieve their first success quickly.

**Trust Building:** Comprehensive documentation demonstrates commitment to the project.

### Documentation Enables User Success

**Self-Service Support:** Good documentation reduces support burden.

**Correct Usage:** Documentation guides users toward correct usage patterns.

**Error Recovery:** When things go wrong, documentation helps users diagnose and fix problems.

### Documentation Supports Credibility

**Academic Credibility:** Comprehensive documentation is required for JOSS publication.

**Professional Credibility:** Organizations evaluating software examine documentation quality.

**Community Credibility:** Open source communities respect projects with good documentation.

---

## Documentation Types

Different types of documentation serve different user needs.

### Tutorials (Learning-Oriented)

**Purpose:** Help beginners get started and learn core concepts through hands-on experience.

**Characteristics:**
- Lesson-based approach
- Step-by-step instructions
- Assumes no prior knowledge
- Focuses on learning, not doing

**Examples:**
- "Getting Started with Lutufi"
- "Your First Probabilistic Network"
- "Understanding Inference Algorithms"

### How-To Guides (Task-Oriented)

**Purpose:** Guide users through solving specific real-world problems.

**Characteristics:**
- Problem-solution format
- Assumes basic knowledge
- Focuses on practical outcomes
- Addresses specific use cases

**Examples:**
- "How to Model Disease Spread"
- "How to Assess Financial Systemic Risk"
- "How to Calibrate Model Parameters"

### Explanation (Understanding-Oriented)

**Purpose:** Deepen understanding of concepts, theory, and design decisions.

**Characteristics:**
- Conceptual and theoretical
- Background information
- Design rationale
- Connections between ideas

**Examples:**
- "The Mathematics of Belief Propagation"
- "Understanding Cyclic Networks"
- "Design Philosophy: Why Anticipated Integration?"

### Reference (Information-Oriented)

**Purpose:** Provide precise, complete information for lookup during work.

**Characteristics:**
- Comprehensive coverage
- Organized for quick lookup
- Factual and concise
- Version-specific

**Examples:**
- API documentation
- Function/method reference
- Configuration options
- Error code reference
- Glossary

---

## The Diátaxis Framework

The Diátaxis Framework provides a systematic approach to documentation structure.

### Framework Overview

Diátaxis organizes documentation along two axes:

**Theory vs. Practice (Vertical):**
- Theory: Understanding concepts
- Practice: Doing things

**Acquisition vs. Application (Horizontal):**
- Acquisition: Learning for the first time
- Application: Using learned knowledge

This creates four quadrants:
- **Tutorials:** Learning by doing
- **How-To Guides:** Task-oriented
- **Explanation:** Understanding-oriented
- **Reference:** Information-oriented

### Applying Diátaxis to Lutufi

**Tutorials:**
```
docs/tutorials/
├── getting-started/
│   ├── installation.rst
│   ├── first-network.rst
│   └── basic-inference.rst
├── epidemiology/
│   ├── first-disease-model.rst
│   └── analyzing-spread.rst
└── finance/
    ├── first-risk-model.rst
    └── stress-testing.rst
```

**How-To Guides:**
```
docs/how-to/
├── inference/
│   ├── exact-inference.rst
│   ├── approximate-inference.rst
│   └── choosing-algorithm.rst
├── networks/
│   ├── create-custom-topology.rst
│   ├── handle-missing-data.rst
│   └── temporal-networks.rst
└── domains/
    ├── epidemiology-seir.rst
    ├── finance-contagion.rst
    └── social-opinion-dynamics.rst
```

**Explanation:**
```
docs/explanation/
├── concepts/
│   ├── probabilistic-graphical-models.rst
│   ├── network-science-fundamentals.rst
│   ├── bayesian-inference.rst
│   └── cyclic-networks.rst
├── algorithms/
│   ├── belief-propagation.rst
│   ├── gibbs-sampling.rst
│   └── variational-methods.rst
└── design/
    ├── architecture-overview.rst
    └── api-design-principles.rst
```

**Reference:**
```
docs/reference/
├── api/
│   ├── core.rst
│   ├── inference.rst
│   ├── networks.rst
│   └── utils.rst
├── configuration.rst
├── error-messages.rst
└── glossary.rst
```

---

## Tutorial Strategy

Tutorials are the entry point for new users.

### Tutorial Principles

1. **Start from Zero:** Assume no prior knowledge
2. **Provide Early Wins:** Users should see results within 5 minutes
3. **Build Incrementally:** Each tutorial builds on previous ones
4. **Include Exercises:** Active learning beats passive reading
5. **Show, Don't Just Tell:** Working code examples are essential

### Tutorial Series Structure

**Series 1: Getting Started (Complete Beginner)**

1. **Installation and Setup (10 min)**
2. **Your First Network (15 min)**
3. **Running Inference (20 min)**
4. **Working with Real Data (25 min)**

**Series 2: Domain Tutorials (Domain Beginner)**

5. **Epidemiology Basics (30 min)**
6. **Finance Basics (30 min)**

**Series 3: Intermediate Techniques (Some Experience)**

7. **Custom Topologies (30 min)**
8. **Parameter Learning (40 min)**
9. **Handling Uncertainty (35 min)**

---

## How-To Guide Strategy

How-to guides help users accomplish specific tasks efficiently.

### How-To Guide Principles

1. **Problem-First Organization:** Users come with problems, not interest in features
2. **Assumption of Basic Knowledge:** Can assume users have completed tutorials
3. **Focus on Results:** Get to working code quickly
4. **Handle Edge Cases:** Address common variations and gotchas
5. **Include Verification:** Show how to verify the solution worked

### How-To Guide Categories

**Inference Tasks:**
- How to run exact inference
- How to run approximate inference on large networks
- How to choose an inference algorithm

**Network Tasks:**
- How to create a network from a pandas DataFrame
- How to import a NetworkX graph
- How to work with temporal networks

**Domain Tasks:**
- How to set up an SEIR model
- How to run a financial stress test
- How to model opinion dynamics

---

## Explanation Strategy

Explanations deepen understanding of concepts, theory, and design.

### Explanation Principles

1. **Answer "Why," Not Just "How":** Explain reasoning behind design decisions
2. **Connect to Fundamentals:** Link Lutufi concepts to broader theory
3. **Acknowledge Complexity:** Don't oversimplify genuinely complex topics
4. **Use Analogies:** Make abstract concepts concrete
5. **Build Mental Models:** Help users develop accurate intuitions

### Explanation Topics

**Core Concepts:**
- Probabilistic graphical models
- Bayesian networks
- Markov random fields
- Factor graphs
- Inference and learning

**Network Theory:**
- Graph theory fundamentals
- Network topologies
- Centrality and importance
- Community structure

**Algorithms:**
- Belief propagation
- Gibbs sampling
- Variational inference
- Structure learning

**Design Philosophy:**
- Why "Anticipated Integration"?
- API design principles
- Performance trade-offs

---

## Reference Strategy

Reference documentation provides precise, complete information.

### API Documentation Standards

- NumPy-style docstrings
- Type hints
- Parameter descriptions
- Return value descriptions
- Exception documentation
- Usage examples
- Cross-references

### Glossary

Maintain a comprehensive glossary of terms used throughout the documentation.

---

## Writing Style

Clear, concrete, helpful, accessible but precise.

### Be Clear

- Use simple language
- Short sentences
- Active voice
- Concrete examples

### Be Concrete

- Specific examples
- Real numbers
- Actual code
- Tangible outcomes

### Be Helpful

- Anticipate questions
- Provide context
- Explain relevance
- Offer next steps

### Be Accessible but Precise

- Define technical terms
- Avoid unnecessary jargon
- Use analogies
- Maintain precision where it matters

---

## Code Examples in Documentation

Every concept should have code. Copy-paste runnable examples.

### Principles

1. **Runnable:** Examples should execute without modification
2. **Complete:** Include all imports and setup
3. **Short:** Focus on the concept being demonstrated
4. **Realistic:** Use realistic scenarios, not foo/bar
5. **Tested:** Examples run in CI to prevent breakage

### Code Block Standards

```python
import lutufi as lt
import numpy as np

# Create a simple network
model = lt.ProbabilisticNetwork()
model.add_node('rain', distribution='bernoulli', p=0.2)
model.add_node('sprinkler', distribution='conditional',
               parents=['rain'], cpd=[[0.4, 0.6], [0.01, 0.99]])

# Run inference
result = model.infer(query=['sprinkler'], evidence={'rain': 0})
print(f"P(Sprinkler | No Rain) = {result}")
```

---

## Visual Documentation

Diagrams, screenshots, concept visualizations.

### Types of Visuals

**Architecture Diagrams:**
- System architecture
- Component relationships
- Data flow

**Concept Visualizations:**
- Graph structures
- Probability distributions
- Algorithm flow

**Screenshots:**
- Jupyter notebook outputs
- Visualization results
- Tool interfaces

### Tools

- **Diagrams:** draw.io, Lucidchart, TikZ
- **Plots:** Matplotlib, Seaborn examples
- **Network Visualizations:** NetworkX plots

---

## Documentation Maintenance

Keeping docs in sync with code, versioning docs.

### Synchronization Strategy

1. **Version Documentation:** Docs versioned with code
2. **Automated API Docs:** Generated from docstrings
3. **Example Testing:** All examples run in CI
4. **Review Process:** Documentation reviewed with code changes

### Versioning

- Docs match code version
- Version switcher on documentation site
- Archive old versions
- Migration guides between versions

---

## User Feedback on Docs

Gathering feedback, improving based on user pain points.

### Feedback Channels

- GitHub Issues (documentation label)
- Documentation surveys
- User interviews
- Analytics (popular pages, search terms)
- Support questions analysis

### Metrics

- Time on page
- Search queries
- "Was this helpful?" ratings
- Issue reports
- Common questions

---

## Documentation Tools

Sphinx, MkDocs, JupyterBook, API doc generation.

### Recommended Stack

**Primary: Sphinx with extensions**
- Core documentation
- API reference generation
- Multiple output formats

**Extensions:**
- sphinx-autodoc: API documentation
- sphinx-gallery: Example gallery
- nbsphinx: Jupyter notebook integration
- sphinx-rtd-theme: ReadTheDocs theme

**Hosting:**
- ReadTheDocs (free for open source)
- GitHub Pages
- Self-hosted

### Alternatives

**MkDocs:**
- Simpler than Sphinx
- Good for narrative docs
- Less powerful for API docs

**JupyterBook:**
- Excellent for tutorial content
- Native notebook support
- Good for educational materials

---

## Measuring Documentation Quality

User testing, analytics, help requests.

### Quantitative Metrics

- **Coverage:** Percentage of APIs documented
- **Freshness:** Time since last update
- **Completeness:** Required sections present
- **Test Pass Rate:** Doctests and examples passing
- **Page Load Time:** Performance

### Qualitative Metrics

- **User Satisfaction:** Survey ratings
- **Task Completion:** Can users complete tasks?
- **Error Reduction:** Fewer support questions
- **Adoption Rate:** Correlation with documentation quality

### Testing

**Usability Testing:**
- Give users tasks
- Observe where they struggle
- Iterate on pain points

**Hallway Testing:**
- Ask colleagues to try documentation
- Gather quick feedback
- Fix obvious issues

---

## Comparison to Other Libraries

What makes Lutufi docs different/better?

### Current Landscape

**pgmpy:**
- Basic documentation
- API reference
- Limited tutorials
- Room for improvement

**NetworkX:**
- Excellent tutorials
- Good API docs
- Strong example gallery
- Gold standard to match

**graph-tool:**
- Academic documentation style
- Comprehensive but dense
- Assumes high expertise
- Could be more accessible

### Lutufi's Differentiation

1. **Unified Approach:** One place for both Bayesian and network analysis
2. **Domain Focus:** Domain-specific tutorials and guides
3. **Pedagogical Progression:** Structured learning path
4. **Practical Focus:** Real-world examples, not just theory
5. **Accessibility:** Clear explanations without sacrificing precision

### Documentation Goals

**Match NetworkX:**
- Tutorial quality
- Example richness
- Community engagement

**Exceed pgmpy:**
- More comprehensive
- Better organized
- More accessible

**Differ from graph-tool:**
- More accessible entry point
- Better pedagogical structure
- Clearer practical guidance

---

## Conclusion

Documentation is a strategic asset for Lutufi. This philosophy provides:

1. **Clear structure** via the Diátaxis framework
2. **Comprehensive coverage** across all documentation types
3. **Quality standards** for writing and maintenance
4. **User focus** through feedback and testing
5. **Competitive differentiation** from other libraries

Investing in documentation yields returns in adoption, credibility, and community growth. Lutufi documentation should be as carefully crafted as the code itself.
