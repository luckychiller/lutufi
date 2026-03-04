# PyMC and Pyro Comparative Analysis

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Overview](#overview)
2. [PyMC](#pymc)
3. [Pyro](#pyro)
4. [NumPyro](#numpyro)
5. [Architecture](#architecture)
6. [Features](#features)
7. [Strengths](#strengths)
8. [Weaknesses](#weaknesses)
9. [The Probabilistic Programming Approach](#the-probabilistic-programming-approach)
10. [Documentation](#documentation)
11. [Community](#community)
12. [Comparison to Lutufi](#comparison-to-lutufi)
13. [Lessons for Lutufi](#lessons-for-lutufi)
14. [Conclusion](#conclusion)
15. [References](#references)

---

## Overview

### What are PyMC and Pyro?

PyMC and Pyro represent the modern approach to probabilistic modeling through probabilistic programming languages (PPLs). Unlike traditional Bayesian network libraries that explicitly define graph structures, PPLs allow users to specify probabilistic models using programming language constructs, with automatic inference capabilities.

**PyMC** (formerly PyMC3, now PyMC v5+) is a Python library for probabilistic programming that emphasizes user-friendliness and integrates with the scientific Python ecosystem. It traces its lineage to the original PyMC (2003) and underwent a complete rewrite for PyMC3 (2015) using Theano, then migrated to Aesara/PyTensor for modern versions.

**Pyro** is a universal probabilistic programming language developed by Uber AI Labs, built on PyTorch. It emphasizes scalability, deep probabilistic modeling, and integration with modern deep learning workflows.

**NumPyro** is a lightweight version of Pyro built on JAX, emphasizing performance, functional programming, and hardware acceleration.

### Position in the ML/Probabilistic Modeling Landscape

These libraries occupy a crucial position at the intersection of:
- **Bayesian Statistics**: Principled uncertainty quantification
- **Deep Learning**: Scalable gradient-based optimization
- **Probabilistic Graphical Models**: Structured probabilistic reasoning
- **Scientific Computing**: Reproducible research and analysis

They represent the state-of-the-art for Bayesian modeling in Python, with PyMC targeting statisticians and data scientists while Pyro targets machine learning researchers and practitioners.

### The Shift to Probabilistic Programming

Traditional PGM libraries (pgmpy, bnlearn) require explicit graph specification. PPLs enable:
- **Programmatic Model Specification**: Models are Python code, not graph structures
- **Automatic Inference**: Users specify models; the system handles inference
- **Composability**: Models compose like functions
- **Gradient-Based Inference**: Automatic differentiation for efficient inference

This paradigm shift has democratized Bayesian modeling while creating new challenges for domains requiring explicit graph structure.

---

## PyMC

### History and Evolution

**PyMC 1.x-2.x (2003-2015)**: Original implementation by Christopher Fonnesbeck, using Fortran extensions. Functional but limited by its implementation technology.

**PyMC3 (2015)**: Complete rewrite using Theano as the computational backend:
- Revolutionized usability through Theano's automatic differentiation
- Introduced modern API with `with pm.Model()` context managers
- Enabled GPU acceleration for inference
- Massive adoption in the scientific community

**PyMC v4+ (2022-present)**: Migration from Theano to Aesara, then to PyTensor:
- Addressed Theano's end-of-life
- Maintained API compatibility while modernizing backend
- PyMC v5 (2023+) further refined the PyTensor integration

### PyMC Architecture

**PyTensor Backend**: PyMC v5+ uses PyTensor (a fork of Theano) for:
- Symbolic computation graphs
- Automatic differentiation
- GPU/CPU execution
- Computational graph optimization

**Model Specification**:
```python
import pymc as pm

with pm.Model() as model:
    # Priors
    alpha = pm.Normal('alpha', mu=0, sigma=1)
    beta = pm.Normal('beta', mu=0, sigma=1)
    
    # Likelihood
    mu = alpha + beta * x
    obs = pm.Normal('obs', mu=mu, sigma=1, observed=y)
    
    # Inference
    idata = pm.sample(1000)
```

**Key Components**:
- **Distributions**: Comprehensive library of probability distributions
- **Samplers**: NUTS, Metropolis, Slice, and variational inference
- **Diagnostics**: Built-in convergence diagnostics and model checking
- **Visualization**: Arviz integration for visualization

### PyMC Ecosystem Integration

**ArviZ**: Comprehensive visualization and diagnostics
**Bambi**: High-level interface for mixed effects models
**PyMC-Marketing**: Marketing mix modeling
**PyMC-Experimental**: Cutting-edge features

---

## Pyro

### History and Development

Pyro was developed by Uber AI Labs (now part of the Pyro team at Broad Institute and community) starting in 2017. It emerged from the need for:
- Scalable variational inference
- Deep probabilistic models
- Integration with PyTorch's dynamic computation graphs

### Pyro Architecture

**PyTorch Foundation**: Built directly on PyTorch:
- Dynamic computation graphs
- GPU acceleration
- Deep learning integration
- Production deployment capabilities

**Universal PPL**: Supports:
- Stochastic primitives (`pyro.sample`)
- Effect handlers for inference
- Custom inference algorithms
- Deep generative models

**Model Specification**:
```python
import pyro
import pyro.distributions as dist
from pyro.infer import SVI, Trace_ELBO
from pyro.optim import Adam

def model(data):
    alpha = pyro.sample('alpha', dist.Normal(0, 1))
    beta = pyro.sample('beta', dist.Normal(0, 1))
    
    with pyro.plate('data', len(data)):
        pyro.sample('obs', dist.Normal(alpha + beta * x, 1), obs=data)

def guide(data):
    alpha_loc = pyro.param('alpha_loc', torch.tensor(0.))
    alpha_scale = pyro.param('alpha_scale', torch.tensor(1.))
    pyro.sample('alpha', dist.Normal(alpha_loc, alpha_scale))
    # ... similar for beta

# Inference
svi = SVI(model, guide, Adam({'lr': 0.01}), loss=Trace_ELBO())
```

### Pyro Design Philosophy

**Universal**: Can represent any computable probabilistic model
**Scalable**: Designed for large-scale inference
**Minimal**: Core language is small but powerful
**Extensible**: Effect handlers enable custom inference

---

## NumPyro

### JAX-Based Implementation

NumPyro is a NumPy-backed Pyro using JAX for:
- Functional programming paradigm
- JIT compilation
- Hardware acceleration (GPU/TPU)
- Automatic differentiation

### NumPyro Advantages

**Performance**: JAX's JIT compilation provides significant speedups
**Functionality**: Pure functions enable powerful abstractions
**Scalability**: Excellent for high-dimensional models
**Integration**: Works with JAX ecosystem (Optax, Flax, etc.)

### NumPyro Model Example

```python
import numpyro
import numpyro.distributions as dist
from numpyro.infer import MCMC, NUTS

def model(data):
    alpha = numpyro.sample('alpha', dist.Normal(0, 1))
    beta = numpyro.sample('beta', dist.Normal(0, 1))
    
    with numpyro.plate('data', len(data)):
        numpyro.sample('obs', dist.Normal(alpha + beta * x, 1), obs=data)

# Inference
nuts_kernel = NUTS(model)
mcmc = MCMC(nuts_kernel, num_samples=1000, num_warmup=500)
mcmc.run(rng_key, data)
```

---

## Architecture

### Computational Graph Backends

All three libraries use computational graphs as their foundation:

**PyTensor (PyMC)**:
- Static computation graphs
- Ahead-of-time compilation
- Graph optimization
- Python-like syntax with Theano heritage

**PyTorch (Pyro)**:
- Dynamic computation graphs
- Define-by-run execution
- Eager execution mode
- Industry-standard deep learning framework

**JAX (NumPyro)**:
- Functional computation graphs
- JIT compilation to XLA
- Composable transformations
- Google-backed research tool

### Automatic Differentiation

**AD as Foundation**: All three libraries rely on automatic differentiation for:
- Gradient-based inference (HMC, NUTS)
- Variational inference optimization
- Deep learning integration
- Sensitivity analysis

**Backend Differences**:
- PyTensor: Static graph AD
- PyTorch: Dynamic graph AD
- JAX: Functional transformation AD

### Variational Inference

All three provide variational inference capabilities:

**PyMC**: ADVI (Automatic Differentiation Variational Inference)
**Pyro**: Stochastic Variational Inference (SVI) with flexible guides
**NumPyro**: Variational inference with JAX optimization

### Inference Abstractions

**PyMC Approach**:
- Declarative inference specification
- Automatic algorithm selection (NUTS default)
- Integrated diagnostics

**Pyro Approach**:
- Explicit inference specification
- Custom guide functions
- Effect handlers for inference control

**NumPyro Approach**:
- Similar to PyMC but with JAX
- Explicit inference control
- JIT compilation for speed

---

## Features

### Hamiltonian Monte Carlo

All three libraries implement HMC and its variants:

**NUTS (No-U-Turn Sampler)**: 
- Default sampler for continuous variables
- Automatic tuning of step size and path length
- Excellent convergence properties
- Implementation across all three libraries

**HMC**: 
- Available for specialized cases
- Manual tuning options

### ADVI and Variational Methods

**Automatic Differentiation Variational Inference**:
- Fast approximate inference
- Scalable to large datasets
- Mean-field and full-rank approximations

**Custom Variational Families** (Pyro especially):
- Normalizing flows
- Implicit variational distributions
- Neural network guides

### Composable Inference

**Pyro's Effect Handlers**: Enable sophisticated inference composition:
- Condition: Fix values for conditioning
- Replay: Reuse samples
- Block: Prevent sampling at certain sites
- Scale: Adjust log-probabilities

**PyMC's Composability**: Built-in composition through model structure

### Deep Probabilistic Models

**Pyro Strength**: Deep generative models
- Variational autoencoders (VAEs)
- Deep Gaussian processes
- Neural network-based likelihoods
- Attention-based models

**PyMC Integration**: Can use PyTensor for neural networks, though less mature than PyTorch

**NumPyro Integration**: Works with Flax/Haiku for neural networks

### Model Primitives

**Common Distributions**:
- Continuous: Normal, Cauchy, Student-T, Gamma, Beta, etc.
- Discrete: Bernoulli, Categorical, Poisson, Binomial, etc.
- Multivariate: Multivariate Normal, Dirichlet, LKJ, etc.
- Advanced: Gaussian processes, mixture models

**Control Flow**:
- Conditional statements
- Loops and recursion
- Dynamic model structures

**Plates (Vectorization)**:
- Efficient handling of independent observations
- Broadcasting semantics
- Memory-efficient implementation

---

## Strengths

### Flexibility in Model Specification

PPLs enable unprecedented modeling flexibility:

**Programmatic Models**: Models are Python programs, not static graphs:
```python
def flexible_model(data, model_type):
    if model_type == 'linear':
        mu = alpha + beta * x
    elif model_type == 'polynomial':
        mu = alpha + beta1 * x + beta2 * x**2
    # ... more complex logic
```

**Dynamic Structure**: Model structure can depend on data:
```python
def adaptive_model(data):
    k = pm.DiscreteUniform('k', 1, 10)
    if k > 5:
        # Complex model
    else:
        # Simple model
```

**Composable Components**: Models compose like functions:
```python
def component_a(...):
    return pm.sample(...)

def component_b(input_from_a, ...):
    return pm.sample(...)

def full_model(...):
    a = component_a(...)
    b = component_b(a, ...)
```

### Modern ML Integration

**Deep Learning**: Seamless integration with neural networks
**Hardware Acceleration**: GPU/TPU support throughout
**Scalability**: Handle large datasets and complex models
**Production Deployment**: Export to production systems (especially PyTorch/Pyro)

### Scalability Through Hardware Acceleration

**GPU Support**: All three libraries support GPU computation
**Vectorization**: Efficient batch processing
**Distributed Computing**: Support for distributed inference (especially Pyro)
**Large Models**: Can handle models with millions of parameters

### Automatic Inference

Users specify models, not inference algorithms:
```python
with pm.Model() as model:
    # Just specify the model
    ...
    # Inference is automatic
    idata = pm.sample()  # NUTS selected automatically
```

This democratizes Bayesian modeling by removing the need to manually implement inference algorithms.

### Comprehensive Diagnostics

**Convergence Diagnostics**: R-hat, ESS, MCSE
**Visualization**: Trace plots, posterior plots, pair plots
**Model Comparison**: LOO, WAIC for model selection
**Posterior Predictive**: Built-in posterior predictive checking

### Active Development and Ecosystem

**Rapid Innovation**: Constant development of new features
**Research Integration**: New research quickly integrated
**Community Contributions**: Active open-source communities
**Industry Adoption**: Used at major tech companies

---

## Weaknesses

### No Explicit Graph Structure

The fundamental limitation for network analysis:

**Hidden Structure**: Models have implicit dependency structures but no explicit graph representation:
```python
# The dependencies exist but aren't explicitly represented
with pm.Model() as model:
    a = pm.Normal('a', 0, 1)
    b = pm.Normal('b', a, 1)  # b depends on a, but no graph object
```

**No Graph Algorithms**: Cannot run graph algorithms (shortest path, centrality, etc.) on the model structure.

**No Structure Learning**: Cannot learn structure from data; model structure must be specified programmatically.

**No Network Analysis**: No integration with network analysis concepts.

### No Structure Learning

PPLs do not provide structure learning capabilities:

**Manual Structure Specification**: Users must write the model structure in code.

**No Constraint-Based Learning**: No PC algorithm or similar.

**No Score-Based Learning**: No hill-climbing or similar structure search.

**No Structure Priors**: Cannot express uncertainty about structure itself (though some research directions exist).

### Not Designed for Relational/Network Data

**IID Assumption**: Most PPL models assume independent observations:
```python
with pm.Model() as model:
    with pm.plate('data', N):  # Assumes independence
        obs = pm.Normal('obs', mu, sigma, observed=y)
```

**No Native Relational Modeling**: While possible to write, PPLs don't provide primitives for:
- Entity-relationship modeling
- Network autocorrelation
- Relational probabilistic models

**Limited Graph Neural Network Integration**: Though improving, not as seamless as specialized libraries.

### Steep Learning Curve for Non-ML Users

**Technical Requirements**: Using PPLs effectively requires:
- Understanding of Bayesian statistics
- Knowledge of MCMC and variational inference
- Programming proficiency
- Understanding of the backend (PyTorch/JAX)

**Debugging Complexity**: Debugging probabilistic programs is harder than deterministic code:
- Shape issues in sampling
- Convergence problems
- Model misspecification
- Numerical stability

**Conceptual Shift**: The PPL paradigm requires a different mindset from traditional statistical modeling.

### Performance for Discrete Models

**Continuous Bias**: HMC/NUTS work best for continuous variables; discrete variables require:
- Marginalization (computationally expensive)
- Compound steps (less efficient)
- Approximations

**Graph Size Limitations**: While PPLs scale well in parameter count, they don't scale well in graph complexity for discrete models.

### Limited Causal Inference

While PPLs can express causal models, they don't provide:
- Do-calculus operations
- Identification algorithms
- Causal discovery
- Counterfactual reasoning utilities

Users must implement causal operations manually.

---

## The Probabilistic Programming Approach

### vs Explicit PGM Approaches

| Aspect | Explicit PGMs (pgmpy/bnlearn) | Probabilistic Programming (PyMC/Pyro) |
|--------|------------------------------|--------------------------------------|
| Model Specification | Graph structure + CPDs | Python code |
| Structure Learning | Core capability | Not supported |
| Inference | Specialized algorithms | Automatic/General-purpose |
| Flexibility | Limited to graph structure | Unlimited (Turing-complete) |
| Network Analysis | Limited/none | None |
| Causal Inference | Limited (pgmpy) / None (bnlearn) | Manual implementation |
| Scalability | Limited | High |
| Learning Curve | Moderate | Steep |

### When to Use PPLs vs PGMs

**Use PPLs When**:
- Model structure is known and fixed
- Complex functional relationships exist
- Deep learning integration needed
- Large-scale inference required
- Flexibility is paramount

**Use PGMs When**:
- Structure needs to be learned from data
- Graph-theoretic properties matter
- Interpretability of structure is important
- Network analysis integration needed
- Discrete variables dominate

### Hybrid Approaches

Some research and applications combine both:
- Learn structure with PGM tools
- Implement learned structure in PPL for inference
- Use PPL for complex distributions within known structure

---

## Documentation

### Tutorials and Examples

**PyMC Documentation**:
- Comprehensive tutorials from beginner to advanced
- Example gallery with real-world applications
- Case studies across domains
- API reference with examples

**Pyro Documentation**:
- Tutorial series covering basics to advanced topics
- Examples organized by technique
- Research paper implementations
- API documentation

**NumPyro Documentation**:
- Similar structure to Pyro
- JAX-specific tutorials
- Performance optimization guides

### API Documentation

All three provide:
- Complete API reference
- Parameter descriptions
- Return value documentation
- Usage examples
- Cross-references

### Educational Resources

**Books**:
- "Bayesian Analysis with Python" (Martin, PyMC)
- "Statistical Rethinking" (McElreath, PyMC examples)
- "Pattern Recognition and Machine Learning" (Bishop, concepts)

**Courses**:
- Multiple online courses use PyMC/Pyro
- University courses increasingly adopting PPLs

### Documentation Gaps

**Network Applications**: Limited documentation on network/relational modeling
**Causal Modeling**: No dedicated causal inference documentation
**Structure Learning**: Not applicable, but no guidance on structure specification best practices
**Debugging**: Limited guidance on debugging complex models

---

## Community

### Active Development

**PyMC**:
- Active core development team
- Regular releases
- Strong NumFOCUS backing
- Corporate sponsorship

**Pyro**:
- Uber AI Labs origin, now community/Broad Institute
- Active development
- Integration with PyTorch ecosystem
- Research-oriented community

**NumPyro**:
- JAX ecosystem integration
- Active development
- Research-focused
- Growing adoption

### Modern Tooling

**Package Ecosystem**: Rich ecosystems around each library
**Integration**: CI/CD, cloud deployment, experiment tracking
**IDE Support**: Good support in Jupyter, VS Code
**Visualization**: ArviZ, TensorBoard, custom tools

### Community Characteristics

**PyMC**: Statistics-focused, academic and industry mix
**Pyro**: ML research-focused, industry-heavy
**NumPyro**: Research-focused, JAX ecosystem

---

## Comparison to Lutufi

### PPLs are General, Lutufi is Specialized for Networks

**PPL Scope**: General-purpose probabilistic modeling
**Lutufi Scope**: Specialized for network-based probabilistic and causal analysis

### Different Abstraction Levels

**PPLs**: Low-level primitives (sample, observe)
**Lutufi**: High-level network concepts (nodes, edges, paths, communities, interventions)

### Complementary Capabilities

**PPLs Excel At**:
- Complex functional relationships
- Large-scale inference
- Deep learning integration
- General probabilistic modeling

**Lutufi Excels At**:
- Structure learning
- Network analysis integration
- Causal inference on networks
- Graph-theoretic reasoning
- Probabilistic reasoning on learned structures

### Integration Potential

Lutufi could integrate with PPLs for:
- Complex conditional distributions within network structures
- Scalable inference for large networks
- Deep learning-based factors

```python
# Hypothetical integration
from lutufi import CausalNetwork
import pymc as pm

network = CausalNetwork(edges)

# Use PyMC for complex node distributions
with pm.Model() as node_model:
    complex_dist = pm.SomeComplexDistribution(...)
    network.set_cpd('node_a', complex_dist)

# Use Lutufi for network-level inference
result = network.query(...)
```

---

## Lessons for Lutufi

### Modern API Design

**1. Context Managers**: PyMC's `with pm.Model()` pattern provides clean scoping. Lutufi uses similar patterns where appropriate.

**2. Composability**: Design components that compose naturally.

**3. Automatic Inference**: Where possible, automate inference algorithm selection.

**4. Integration with Ecosystem**: Work seamlessly with pandas, NumPy, scikit-learn.

### Hardware Acceleration

**1. Performance Matters**: PPLs demonstrate the importance of performance for adoption.

**2. Hardware Abstraction**: Users shouldn't need to think about hardware; it should just work.

**3. JIT Compilation**: NumPyro's JAX-based approach shows the power of JIT for probabilistic models.

### Differentiable Programming

**1. Gradients Enable Inference**: Automatic differentiation is foundational to modern inference.

**2. Neural Network Factors**: Support for neural network-based conditional distributions enables hybrid models.

**3. End-to-End Differentiability**: Where possible, maintain differentiability for optimization.

### User Experience

**1. Progressive Disclosure**: Simple for beginners, powerful for experts.

**2. Clear Errors**: Helpful error messages guide users.

**3. Diagnostics**: Built-in diagnostics and visualization aid model development.

### What to Avoid

**1. Hidden Structure**: PPLs' lack of explicit graph structure limits network analysis. Lutufi maintains explicit graph representation.

**2. No Structure Learning**: PPLs cannot learn structure from data. Lutufi makes structure learning central.

**3. Steep Learning Curve**: PPLs require significant statistical background. Lutufi provides higher-level abstractions.

**4. Network Blindness**: PPLs ignore network structure. Lutufi makes networks first-class.

---

## Conclusion

PyMC, Pyro, and NumPyro represent the cutting edge of probabilistic programming, demonstrating how automatic differentiation, modern computational frameworks, and user-friendly APIs can democratize Bayesian modeling. These libraries have fundamentally changed how practitioners approach probabilistic inference, enabling complex models that would have been impractical with traditional methods.

Their strengths—flexibility, scalability, hardware acceleration, and modern ML integration—set benchmarks for any modern probabilistic library. The paradigm shift from explicit graph specification to programmatic model definition has opened new possibilities while creating new challenges.

However, these libraries' fundamental limitations for network analysis—lack of explicit graph structure, no structure learning, limited relational modeling, and no causal inference utilities—create significant opportunities for Lutufi. The PPL approach and explicit PGM approach serve different purposes and can be complementary.

Lutufi occupies a distinct position: specialized for network-based probabilistic and causal analysis while learning from PPLs' modern approaches to inference, hardware acceleration, and API design. Rather than competing directly, Lutufi can potentially integrate with PPLs, using them for complex distributions within network structures while providing the graph-theoretic and causal capabilities that PPLs lack.

The lessons from these libraries—particularly around API design, performance optimization, and user experience—directly inform Lutufi's development. By combining the accessibility and power of modern PPLs with explicit network structure, structure learning, and causal inference, Lutufi addresses a gap that neither traditional PGM libraries nor modern PPLs can fill alone.

For users of PyMC, Pyro, or NumPyro, Lutufi offers specialized capabilities for network analysis that complement their existing tools. For new users requiring network-based probabilistic and causal analysis, Lutufi provides a purpose-built solution that doesn't require shoehorning network problems into general-purpose frameworks.

---

## References

1. Salvatier, J., Wiecki, T. V., & Fonnesbeck, C. (2016). Probabilistic programming in Python using PyMC3. *PeerJ Computer Science*, 2, e55.

2. Bingham, E., Chen, J. P., Jankowiak, M., Obermeyer, F., Pradhan, N., Karaletsos, T., ... & Goodman, N. D. (2019). Pyro: Deep universal probabilistic programming. *The Journal of Machine Learning Research*, 20(1), 973-978.

3. Phan, D., Pradhan, N., & Jankowiak, M. (2019). Composable effects for flexible and accelerated probabilistic programming in NumPyro. *arXiv preprint arXiv:1912.11554*.

4. Martin, O. A., Kumar, R., & Lao, J. (2024). *Bayesian Modeling and Computation in Python*. Chapman and Hall/CRC.

5. McElreath, R. (2020). *Statistical Rethinking: A Bayesian Course with Examples in R and Stan* (2nd ed.). Chapman and Hall/CRC.

6. PyMC Documentation. https://www.pymc.io/

7. Pyro Documentation. https://pyro.ai/

8. NumPyro Documentation. https://num.pyro.ai/

9. Hoffman, M. D., Gelman, A., et al. (2014). The No-U-Turn sampler: adaptively setting path lengths in Hamiltonian Monte Carlo. *Journal of Machine Learning Research*, 15(1), 1593-1623.

10. Kucukelbir, A., Tran, D., Ranganath, R., Gelman, A., & Blei, D. M. (2017). Automatic differentiation variational inference. *The Journal of Machine Learning Research*, 18(1), 430-474.

---

*Document part of the Lutufi Project — Unifying Bayesian networks with social and economic network analysis. For more information, see the project repository or contact the author.*
