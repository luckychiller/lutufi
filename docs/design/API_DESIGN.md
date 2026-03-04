# Lutufi API Design Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [API Design Philosophy](#api-design-philosophy)
3. [Naming Conventions](#naming-conventions)
4. [Module Structure](#module-structure)
5. [Core Classes and Interfaces](#core-classes-and-interfaces)
6. [Constructing Models](#constructing-models)
7. [Running Inference](#running-inference)
8. [Learning from Data](#learning-from-data)
9. [Causal Queries](#causal-queries)
10. [Network Analysis Integration](#network-analysis-integration)
11. [DataFrame Integration](#dataframe-integration)
12. [Visualization API](#visualization-api)
13. [Configuration and Options](#configuration-and-options)
14. [Error Handling in the API](#error-handling-in-the-api)
15. [Type Hints and Documentation](#type-hints-and-documentation)
16. [Example Workflows](#example-workflows)
17. [API Stability and Deprecation](#api-stability-and-deprecation)
18. [Key References](#key-references)

---

## Executive Summary

This document defines the Application Programming Interface (API) for the Lutufi library. The API is designed to provide a Pythonic, intuitive interface to sophisticated probabilistic modeling capabilities while maintaining performance and extensibility.

The design prioritizes:
- **Clarity**: Functions and parameters have clear, descriptive names
- **Consistency**: Patterns are applied uniformly across the API
- **Composability**: Operations can be chained and combined
- **Discoverability**: APIs are organized logically and documented comprehensively
- **Safety**: Invalid operations fail fast with informative messages

---

## API Design Philosophy

### Explicit Over Implicit

Lutufi APIs favor explicit specification over implicit behavior:

**Explicit:**
```python
# User clearly specifies the algorithm
result = model.infer_marginal(
    variables=['Disease'],
    evidence={'Symptom': 'fever'},
    algorithm='variable_elimination',
    elimination_order=['Age', 'Gender', 'Location']
)
```

**Not Implicit:**
```python
# Avoid: What algorithm is used? What is the elimination order?
result = model.query('Disease', Symptom='fever')
```

Benefits of explicitness:
- Code is self-documenting
- Results are reproducible
- Users understand the computational cost
- Debugging is easier when behavior is predictable

### Fail Fast

Invalid inputs are detected and rejected immediately:

```python
# Raises ValueError immediately with clear message
model.add_edge('A', 'A')  # Error: Self-loops not allowed in Bayesian networks

# Raises ValueError with diagnostic information
model.set_cpd('B', [[0.3, 0.5], [0.2, 0.8]])  # Error: Probabilities sum to 0.9, expected 1.0
```

Validation occurs at multiple levels:
1. **Type validation**: Ensures arguments have correct Python types
2. **Value validation**: Ensures values are in valid ranges
3. **Semantic validation**: Ensures operations are valid for the model state
4. **Consistency validation**: Ensures related values are mutually consistent

### Sensible Defaults

While explicit is preferred, sensible defaults reduce boilerplate:

```python
# Uses default inference algorithm based on model characteristics
result = model.query(variables=['X'], evidence={'Y': 1})

# Default: BIC scoring with hill climbing search
model = lf.BayesianNetwork.fit(data)

# Default: Uniform prior for parameter learning
model.fit_parameters(data, prior='uniform')
```

Default selection criteria:
- Default algorithms chosen based on expected performance for typical cases
- Default parameters derived from literature recommendations
- Defaults can be overridden for specific use cases
- Default behavior is documented and stable

### Pythonic Where Appropriate

The API follows Python conventions while respecting domain-specific needs:

**Pythonic Patterns:**
```python
# Iteration
for node in model.nodes():
    print(node)

# Context managers
with lf.InferenceSession(model) as session:
    result = session.query(...)

# Duck typing
model.add_nodes_from(['A', 'B', 'C'])  # Accepts any iterable

# Properties
if model.is_dag:
    print("Model is a valid DAG")
```

**Domain-Specific Where Necessary:**
```python
# Mathematical notation preserved for clarity
result = model.do_calculus.intervention({'Treatment': 1})

# Probabilistic terminology used correctly
marginal = model.marginal_distribution(['Outcome'])
```

---

## Naming Conventions

### Function Names

**Actions (Verbs):**
- `fit()`: Learn parameters or structure from data
- `predict()`: Generate predictions from model
- `query()`: Execute probabilistic query
- `sample()`: Generate samples from distribution
- `transform()`: Convert data or model representation

**Queries (Noun Phrases):**
- `get_cpd()`: Retrieve conditional probability distribution
- `get_parents()`: Retrieve parent nodes
- `get_moral_graph()`: Retrieve moralized graph
- `is_d_separated()`: Test d-separation

**Boolean Predicates:**
- `is_dag`: Property indicating directed acyclic graph
- `has_cycle()`: Method testing for cycles
- `is_valid()`: Method testing model validity
- `can_infer_exactly()`: Method testing inference feasibility

### Class Names

**Model Classes:**
- `BayesianNetwork`: Directed graphical model with CPDs
- `MarkovRandomField`: Undirected graphical model with factors
- `DynamicBayesianNetwork`: Time-sliced Bayesian network
- `FactorGraph`: Bipartite factor graph representation

**Engine Classes:**
- `InferenceEngine`: Base class for inference algorithms
- `VariableElimination`: Exact inference via variable elimination
- `BeliefPropagation`: Approximate inference via message passing
- `GibbsSampler`: MCMC sampling for inference

**Learning Classes:**
- `StructureLearner`: Base class for structure learning
- `HillClimbingSearch`: Score-based structure learning
- `PCAlgorithm`: Constraint-based structure learning
- `ParameterLearner`: Base class for parameter learning

### Parameter Names

**Common Parameters:**
- `variables`: List of variable names (always plural for list)
- `evidence`: Dictionary mapping variables to observed values
- `algorithm`: String naming algorithm to use
- `max_iterations`: Upper bound on iterative algorithms
- `tolerance`: Convergence threshold
- `random_state`: Seed for reproducibility
- `verbose`: Boolean controlling output verbosity
- `n_jobs`: Number of parallel jobs (-1 for all cores)

**Domain-Specific Parameters:**
- `cpd`: Conditional probability distribution
- `elimination_order`: Order for variable elimination
- `scoring_method`: Function for structure scoring
- `independence_test`: Test for constraint-based learning
- `significance_level`: Alpha for statistical tests

### Consistent Terminology

| Concept | Term | Avoid |
|---------|------|-------|
| Random variable | `variable` | node, vertex |
| Variable value | `value` or `state` | level, category |
| Network structure | `structure` or `graph` | topology, skeleton |
| Conditional probability | `cpd` | CPT, table, distribution |
| Factor potential | `factor` | potential, clique potential |
| Evidence/observation | `evidence` | observation, data point |
| Query target | `target` or `variables` | query_var, interest |
| Parent nodes | `parents` | predecessors, sources |
| Child nodes | `children` | successors, sinks |

---

## Module Structure

### Top-Level Organization

```
lutufi/
├── __init__.py              # Core imports and version
├── models/                  # Model representations
│   ├── __init__.py
│   ├── bayesian_network.py
│   ├── markov_random_field.py
│   ├── dynamic_bayesian_network.py
│   └── factor_graph.py
├── inference/               # Inference algorithms
│   ├── __init__.py
│   ├── exact/               # Exact inference
│   │   ├── __init__.py
│   │   ├── variable_elimination.py
│   │   └── junction_tree.py
│   ├── approximate/         # Approximate inference
│   │   ├── __init__.py
│   │   ├── belief_propagation.py
│   │   ├── gibbs_sampling.py
│   │   └── variational.py
│   └── engines.py           # Engine factory and base classes
├── learning/                # Learning algorithms
│   ├── __init__.py
│   ├── structure/           # Structure learning
│   │   ├── __init__.py
│   │   ├── score_based.py
│   │   ├── constraint_based.py
│   │   └── hybrid.py
│   └── parameter/           # Parameter learning
│       ├── __init__.py
│       ├── mle.py
│       ├── bayesian.py
│       └── em.py
├── networks/                # Network analysis integration
│   ├── __init__.py
│   ├── conversion.py
│   ├── measures.py
│   └── dynamics.py
├── causal/                  # Causal inference
│   ├── __init__.py
│   ├── do_calculus.py
│   ├── identification.py
│   └── counterfactual.py
├── viz/                     # Visualization
│   ├── __init__.py
│   ├── network_plot.py
│   ├── inference_plot.py
│   └── learning_plot.py
├── data/                    # Data I/O
│   ├── __init__.py
│   ├── readers.py
│   ├── writers.py
│   └── adapters.py
└── utils/                   # Utilities
    ├── __init__.py
    ├── validation.py
    ├── parallel.py
    └── logging.py
```

### Module Exports

**Core Module (`lutufi`):**
```python
# lutufi/__init__.py

from .models import BayesianNetwork, MarkovRandomField, DynamicBayesianNetwork
from .inference import InferenceEngine, VariableElimination, BeliefPropagation
from .learning import fit_structure, fit_parameters
from .networks import to_networkx, from_networkx
from .causal import do, counterfactual
from .viz import plot_network, plot_inference

__version__ = "0.1.0"
```

**Inference Module:**
```python
# lutufi/inference/__init__.py

from .engines import InferenceEngine, InferenceResult
from .exact import VariableElimination, JunctionTree
from .approximate import BeliefPropagation, GibbsSampler, VariationalInference

__all__ = [
    'InferenceEngine',
    'InferenceResult',
    'VariableElimination',
    'JunctionTree',
    'BeliefPropagation',
    'GibbsSampler',
    'VariationalInference',
]
```

---

## Core Classes and Interfaces

### FactorGraph

The canonical internal representation.

```python
class FactorGraph:
    """
    A factor graph representing the joint distribution over variables.
    
    A factor graph is a bipartite graph consisting of variable nodes and
    factor nodes. Each factor represents a function over a subset of variables.
    
    Parameters
    ----------
    variables : list of Variable
        The variables in the factor graph.
    factors : list of Factor
        The factors defining the joint distribution.
    
    Attributes
    ----------
    variables : dict
        Mapping from variable names to Variable objects.
    factors : list
        List of Factor objects.
    """
    
    def __init__(self, variables: List[Variable], factors: List[Factor]):
        """Initialize a factor graph."""
        pass
    
    def add_factor(self, factor: Factor) -> None:
        """Add a factor to the graph."""
        pass
    
    def remove_factor(self, factor: Factor) -> None:
        """Remove a factor from the graph."""
        pass
    
    def get_neighbors(self, variable: Variable) -> List[Factor]:
        """Get factors connected to a variable."""
        pass
    
    def marginalize(self, variables: List[str]) -> 'FactorGraph':
        """Return factor graph with variables marginalized out."""
        pass
    
    def multiply(self, other: 'FactorGraph') -> 'FactorGraph':
        """Multiply two factor graphs."""
        pass
```

### BayesianNetwork

Primary user-facing model class.

```python
class BayesianNetwork:
    """
    A Bayesian Network representing a joint probability distribution.
    
    A Bayesian network is a directed acyclic graph where nodes represent
    random variables and edges represent direct dependencies. Each node
    has an associated conditional probability distribution (CPD).
    
    Parameters
    ----------
    structure : nx.DiGraph, optional
        The network structure as a directed graph.
    
    Attributes
    ----------
    nodes : list
        List of node names.
    edges : list
        List of directed edges (parent, child).
    cpds : dict
        Mapping from node names to CPD objects.
    
    Examples
    --------
    >>> from lutufi import BayesianNetwork
    >>> model = BayesianNetwork()
    >>> model.add_node('A', states=['low', 'high'])
    >>> model.add_edge('A', 'B')
    >>> model.set_cpd('A', [0.3, 0.7])
    >>> model.set_cpd('B', [[0.9, 0.1], [0.2, 0.8]])
    """
    
    def __init__(self, structure: Optional[nx.DiGraph] = None):
        """Initialize a Bayesian network."""
        pass
    
    # Structure manipulation
    def add_node(self, node: str, states: Optional[List[str]] = None,
                 domain: Optional[Domain] = None) -> None:
        """Add a node to the network."""
        pass
    
    def add_edge(self, u: str, v: str) -> None:
        """Add a directed edge from u to v."""
        pass
    
    def remove_edge(self, u: str, v: str) -> None:
        """Remove a directed edge."""
        pass
    
    def get_parents(self, node: str) -> List[str]:
        """Get the parents of a node."""
        pass
    
    def get_children(self, node: str) -> List[str]:
        """Get the children of a node."""
        pass
    
    def get_ancestors(self, node: str) -> Set[str]:
        """Get all ancestors of a node."""
        pass
    
    def get_descendants(self, node: str) -> Set[str]:
        """Get all descendants of a node."""
        pass
    
    # CPD management
    def set_cpd(self, node: str, cpd: Union[np.ndarray, CPD, List]) -> None:
        """Set the conditional probability distribution for a node."""
        pass
    
    def get_cpd(self, node: str) -> CPD:
        """Get the conditional probability distribution for a node."""
        pass
    
    def check_model(self) -> bool:
        """Check if the model is valid (DAG, consistent CPDs)."""
        pass
    
    # Inference
    def query(self, variables: List[str],
              evidence: Optional[Dict[str, Any]] = None,
              algorithm: Optional[str] = None,
              **kwargs) -> 'QueryResult':
        """
        Query the model for marginal or conditional distributions.
        
        Parameters
        ----------
        variables : list of str
            Variables to query.
        evidence : dict, optional
            Observed values {variable: value}.
        algorithm : str, optional
            Inference algorithm to use. If None, automatically selected.
        **kwargs
            Additional algorithm-specific parameters.
        
        Returns
        -------
        QueryResult
            Query results with probabilities.
        
        Examples
        --------
        >>> result = model.query(['Disease'], evidence={'Symptom': 'fever'})
        >>> print(result['Disease'])
        Disease      present    absent
        probability    0.75      0.25
        """
        pass
    
    def map_query(self, variables: List[str],
                  evidence: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """
        Maximum a posteriori query.
        
        Returns the most probable assignment to the query variables.
        """
        pass
    
    def predict(self, data: pd.DataFrame,
                variables: Optional[List[str]] = None) -> pd.DataFrame:
        """
        Predict values for variables given observed data.
        
        Parameters
        ----------
        data : pd.DataFrame
            Observed data (may contain missing values).
        variables : list of str, optional
            Variables to predict. If None, predict all non-evidence variables.
        
        Returns
        -------
        pd.DataFrame
            Predicted values.
        """
        pass
    
    def sample(self, n_samples: int = 100,
               evidence: Optional[Dict[str, Any]] = None) -> pd.DataFrame:
        """
        Generate samples from the model.
        
        Parameters
        ----------
        n_samples : int
            Number of samples to generate.
        evidence : dict, optional
            Conditioning evidence.
        
        Returns
        -------
        pd.DataFrame
            Generated samples.
        """
        pass
    
    # Learning
    @classmethod
    def fit(cls, data: pd.DataFrame,
            structure_algorithm: str = 'hc',
            parameter_algorithm: str = 'mle',
            **kwargs) -> 'BayesianNetwork':
        """
        Learn a Bayesian network from data.
        
        Parameters
        ----------
        data : pd.DataFrame
            Training data.
        structure_algorithm : str
            Algorithm for structure learning ('hc', 'pc', 'exhaustive').
        parameter_algorithm : str
            Algorithm for parameter learning ('mle', 'bayesian').
        **kwargs
            Additional algorithm-specific parameters.
        
        Returns
        -------
        BayesianNetwork
            Learned model.
        """
        pass
    
    def fit_parameters(self, data: pd.DataFrame,
                       algorithm: str = 'mle',
                       prior: Optional[str] = None,
                       **kwargs) -> None:
        """Learn parameters from data given the current structure."""
        pass
    
    # Validation
    def is_dag(self) -> bool:
        """Check if the structure is a directed acyclic graph."""
        pass
    
    def is_d_separated(self, x: str, y: str,
                       observed: Optional[List[str]] = None) -> bool:
        """Check if x and y are d-separated given observed variables."""
        pass
    
    # Conversion
    def to_factor_graph(self) -> FactorGraph:
        """Convert to factor graph representation."""
        pass
    
    def to_networkx(self) -> nx.DiGraph:
        """Convert to NetworkX directed graph."""
        pass
    
    @classmethod
    def from_networkx(cls, graph: nx.DiGraph) -> 'BayesianNetwork':
        """Create from NetworkX directed graph."""
        pass
    
    # Serialization
    def save(self, filename: str, format: Optional[str] = None) -> None:
        """Save model to file."""
        pass
    
    @classmethod
    def load(cls, filename: str) -> 'BayesianNetwork':
        """Load model from file."""
        pass
```

### MarkovRandomField

```python
class MarkovRandomField:
    """
    A Markov Random Field (undirected graphical model).
    
    Represents a joint distribution as a product of factors over cliques.
    
    Parameters
    ----------
    structure : nx.Graph, optional
        The underlying undirected graph structure.
    
    Examples
    --------
    >>> from lutufi import MarkovRandomField
    >>> model = MarkovRandomField()
    >>> model.add_nodes_from(['A', 'B', 'C'])
    >>> model.add_edges_from([('A', 'B'), ('B', 'C')])
    >>> model.add_factor(['A', 'B'], phi_AB)
    """
    
    def __init__(self, structure: Optional[nx.Graph] = None):
        """Initialize a Markov random field."""
        pass
    
    def add_factor(self, variables: List[str], values: np.ndarray) -> None:
        """Add a factor over the specified variables."""
        pass
    
    def get_factors(self, variable: Optional[str] = None) -> List[Factor]:
        """Get factors, optionally filtered by variable."""
        pass
    
    def query(self, variables: List[str],
              evidence: Optional[Dict[str, Any]] = None,
              **kwargs) -> 'QueryResult':
        """Query for marginal distributions."""
        pass
```

### DynamicBayesianNetwork

```python
class DynamicBayesianNetwork:
    """
    A Dynamic Bayesian Network for temporal modeling.
    
    Represents a probability distribution over time series using a
    two-slice temporal Bayes net (2TBN) structure.
    
    Parameters
    ----------
    initial_model : BayesianNetwork
        Model for the initial time slice.
    transition_model : BayesianNetwork
        Two-slice model for transitions.
    
    Examples
    --------
    >>> dbn = DynamicBayesianNetwork(initial_model, transition_model)
    >>> # Inference over 5 time steps
    >>> result = dbn.query(['State'], time_slice=5)
    """
    
    def __init__(self, initial_model: BayesianNetwork,
                 transition_model: BayesianNetwork):
        """Initialize a dynamic Bayesian network."""
        pass
    
    def unroll(self, n_time_slices: int) -> BayesianNetwork:
        """Unroll the DBN into a static Bayesian network."""
        pass
    
    def forward_inference(self, evidence: List[Dict[str, Any]]) -> List['QueryResult']:
        """Forward filtering through time series evidence."""
        pass
    
    def smoothing(self, evidence: List[Dict[str, Any]],
                  t: int) -> 'QueryResult':
        """Compute smoothed distribution at time t given all evidence."""
        pass
```

### Variable

```python
class Variable:
    """
    A random variable in a probabilistic model.
    
    Parameters
    ----------
    name : str
        Unique identifier for the variable.
    domain : Domain
        The set of possible values.
    
    Examples
    --------
    >>> var = Variable('Weather', DiscreteDomain(['sunny', 'rainy', 'cloudy']))
    >>> print(var.domain)
    ['sunny', 'rainy', 'cloudy']
    """
    
    def __init__(self, name: str, domain: 'Domain'):
        """Initialize a variable."""
        pass
    
    @property
    def cardinality(self) -> int:
        """Number of possible values (for discrete variables)."""
        pass
    
    def is_discrete(self) -> bool:
        """Check if variable is discrete."""
        pass
    
    def is_continuous(self) -> bool:
        """Check if variable is continuous."""
        pass
```

### CPD (Conditional Probability Distribution)

```python
class CPD:
    """
    Base class for conditional probability distributions.
    
    A CPD represents P(X | Parents(X)) for a variable X.
    
    Parameters
    ----------
    variable : str
        The child variable.
    parents : list of str
        The parent variables.
    
    Examples
    --------
    >>> cpd = TabularCPD('B', ['A'], values=[[0.9, 0.2], [0.1, 0.8]])
    >>> print(cpd)
    P(B | A)
    A=0    A=1
    B=0    0.9    0.2
    B=1    0.1    0.8
    """
    
    def __init__(self, variable: str, parents: List[str]):
        """Initialize a CPD."""
        pass
    
    def get_values(self, parent_assignment: Optional[Dict[str, Any]] = None) -> np.ndarray:
        """Get probability values, optionally conditioned on parent values."""
        pass
    
    def sample(self, parent_values: Optional[Dict[str, Any]] = None,
               n_samples: int = 1) -> np.ndarray:
        """Sample from the conditional distribution."""
        pass
    
    def copy(self) -> 'CPD':
        """Create a copy of the CPD."""
        pass


class TabularCPD(CPD):
    """
    Tabular conditional probability distribution.
    
    Stores probabilities in a multidimensional array where dimensions
    correspond to variable and its parents.
    
    Parameters
    ----------
    variable : str
        The child variable.
    variable_card : int
        Number of states for the variable.
    values : array-like
        Probability table with shape (variable_card, parent_card_1 * ...).
    evidence : list of str, optional
        Parent variable names.
    evidence_card : list of int, optional
        Number of states for each parent.
    
    Examples
    --------
    >>> cpd = TabularCPD(
    ...     'Grade', 3,
    ...     [[0.3, 0.05, 0.9, 0.5],
    ...      [0.4, 0.25, 0.08, 0.3],
    ...      [0.3, 0.7, 0.02, 0.2]],
    ...     evidence=['Difficulty', 'Intelligence'],
    ...     evidence_card=[2, 2]
    ... )
    """
    
    def __init__(self, variable: str, variable_card: int,
                 values: np.ndarray,
                 evidence: Optional[List[str]] = None,
                 evidence_card: Optional[List[int]] = None):
        """Initialize a tabular CPD."""
        pass
    
    def normalize(self) -> None:
        """Normalize the CPD so probabilities sum to 1."""
        pass
    
    def marginalize(self, variables: List[str]) -> 'TabularCPD':
        """Marginalize out the specified variables."""
        pass


class GaussianCPD(CPD):
    """
    Gaussian conditional probability distribution for continuous variables.
    
    Represents P(X | Parents) as a Gaussian with mean being a linear
    function of parents and constant variance.
    
    Parameters
    ----------
    variable : str
        The child variable.
    beta : array-like
        Linear coefficients for parents.
    intercept : float
        Intercept term.
    variance : float
        Conditional variance.
    parents : list of str
        Parent variable names.
    
    Examples
    --------
    >>> cpd = GaussianCPD(
    ...     'Y',
    ...     beta=[0.5, -0.3],
    ...     intercept=1.0,
    ...     variance=0.25,
    ...     parents=['X1', 'X2']
    ... )
    """
    
    def __init__(self, variable: str, beta: np.ndarray,
                 intercept: float, variance: float,
                 parents: List[str]):
        """Initialize a Gaussian CPD."""
        pass
```

### InferenceEngine

```python
class InferenceEngine(ABC):
    """
    Abstract base class for inference engines.
    
    Inference engines compute probabilistic queries on models.
    
    Parameters
    ----------
    model : BayesianNetwork or MarkovRandomField
        The model to perform inference on.
    
    Examples
    --------
    >>> engine = VariableElimination(model)
    >>> result = engine.query(['A'], evidence={'B': 1})
    """
    
    def __init__(self, model: Union[BayesianNetwork, MarkovRandomField]):
        """Initialize inference engine."""
        pass
    
    @abstractmethod
    def query(self, variables: List[str],
              evidence: Optional[Dict[str, Any]] = None,
              **kwargs) -> 'QueryResult':
        """
        Execute a probabilistic query.
        
        Parameters
        ----------
        variables : list of str
            Variables to query.
        evidence : dict, optional
            Observed values.
        **kwargs
            Algorithm-specific parameters.
        
        Returns
        -------
        QueryResult
            Query results.
        """
        pass
    
    @abstractmethod
    def map_query(self, variables: List[str],
                  evidence: Optional[Dict[str, Any]] = None,
                  **kwargs) -> Dict[str, Any]:
        """Maximum a posteriori query."""
        pass


class VariableElimination(InferenceEngine):
    """
    Exact inference by variable elimination.
    
    Eliminates variables one by one by summing them out from factors.
    Exact but exponential in treewidth.
    
    Parameters
    ----------
    model : BayesianNetwork or MarkovRandomField
        The model to perform inference on.
    
    Examples
    --------
    >>> engine = VariableElimination(model)
    >>> # With custom elimination order
    >>> result = engine.query(['A'], elimination_order=['B', 'C', 'D'])
    """
    
    def query(self, variables: List[str],
              evidence: Optional[Dict[str, Any]] = None,
              elimination_order: Optional[List[str]] = None,
              **kwargs) -> 'QueryResult':
        """
        Query with variable elimination.
        
        Parameters
        ----------
        elimination_order : list of str, optional
            Order for eliminating variables. If None, automatically determined.
        """
        pass


class BeliefPropagation(InferenceEngine):
    """
    Approximate inference by belief propagation.
    
    Uses message passing on a factor graph. Exact for trees,
    approximate (loopy BP) for graphs with cycles.
    
    Parameters
    ----------
    model : BayesianNetwork or MarkovRandomField
        The model to perform inference on.
    max_iterations : int
        Maximum number of message passing iterations.
    tolerance : float
        Convergence threshold.
    
    Examples
    --------
    >>> engine = BeliefPropagation(model, max_iterations=100, tolerance=1e-6)
    >>> result = engine.query(['A'])
    """
    
    def __init__(self, model: Union[BayesianNetwork, MarkovRandomField],
                 max_iterations: int = 100,
                 tolerance: float = 1e-6,
                 damping: float = 0.0):
        """Initialize belief propagation engine."""
        pass
    
    def query(self, variables: List[str],
              evidence: Optional[Dict[str, Any]] = None,
              **kwargs) -> 'QueryResult':
        """Query with belief propagation."""
        pass
    
    @property
    def converged(self) -> bool:
        """Whether the algorithm converged."""
        pass
    
    @property
    def num_iterations(self) -> int:
        """Number of iterations performed."""
        pass
```

### Query and QueryResult

```python
class Query:
    """
    Represents a probabilistic query.
    
    Parameters
    ----------
    variables : list of str
        Variables to query.
    evidence : dict, optional
        Observed values.
    query_type : str
        Type of query ('marginal', 'map', 'mpe').
    
    Examples
    --------
    >>> query = Query(['Disease'], evidence={'Symptom': 'fever'})
    >>> engine = InferenceEngine(model)
    >>> result = engine.execute(query)
    """
    
    def __init__(self, variables: List[str],
                 evidence: Optional[Dict[str, Any]] = None,
                 query_type: str = 'marginal'):
        """Initialize a query."""
        pass


class QueryResult:
    """
    Results of a probabilistic query.
    
    Provides convenient access to probability distributions with
    support for multiple output formats.
    
    Attributes
    ----------
    variables : list of str
        Query variables.
    distributions : dict
        Probability distributions for each variable or joint.
    
    Examples
    --------
    >>> result = model.query(['A', 'B'])
    >>> print(result['A'])  # Marginal for A
    >>> print(result.joint)  # Joint distribution
    >>> df = result.to_dataframe()  # Convert to pandas DataFrame
    """
    
    def __init__(self, variables: List[str],
                 distributions: Dict[str, np.ndarray],
                 joint: Optional[np.ndarray] = None):
        """Initialize query result."""
        pass
    
    def __getitem__(self, variable: str) -> np.ndarray:
        """Get marginal distribution for a variable."""
        pass
    
    def to_dataframe(self) -> pd.DataFrame:
        """Convert results to pandas DataFrame."""
        pass
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert results to dictionary."""
        pass
    
    @property
    def joint(self) -> Optional[np.ndarray]:
        """Joint probability distribution over query variables."""
        pass
```

---

## Causal vs Statistical Graph API

### Design Rationale

Lutufi makes an explicit distinction between **statistical** Bayesian networks (representing conditional dependencies) and **causal** Bayesian networks (representing structural causal models). This distinction is enforced at the API level to prevent accidental causal reasoning on models that do not support it.

### The mark_as_causal() Method

All probabilistic model classes provide a `mark_as_causal()` method that explicitly designates the model as representing a structural causal model:

```python
model = BayesianNetwork(...)
model.mark_as_causal()  # Explicit designation as SCM
```

**What marking does:**
- Records that the graph structure represents causal (not just statistical) relationships
- Enables causal query methods (do-operator, counterfactuals)
- Validates that the model meets causal requirements (e.g., no cycles in causal interpretation)
- Freezes structural modifications (causal graphs require careful handling)

**What marking does NOT do:**
- Change the underlying graph structure
- Automatically verify that edges actually represent causation (this remains the user's responsibility)
- Make the model causal by fiat—incorrect causal assumptions still produce incorrect answers

### Causal Operations Require Causal Models

The do-operator and other causal inference methods raise `LutufiNonCausalModelError` on non-causal models:

```python
# Valid causal workflow
model = BayesianNetwork(...)
model.mark_as_causal()  # Explicit designation
result = model.do("X", x=1).query("Y")  # Valid - model is causal

# Invalid - raises error
model = BayesianNetwork(...)  # Statistical only
result = model.do("X", x=1).query("Y")
# Raises: LutufiNonCausalModelError

# Alternative for statistical models
result = model.observational_query("Y", evidence={"X": 1})  # Valid - no causation claimed
```

### Error Handling: LutufiNonCausalModelError

```python
class LutufiNonCausalModelError(LutufiError):
    """Raised when causal operations are attempted on non-causal models."""
    
    def __init__(self, operation: str, suggestion: str = None):
        message = (
            f"Causal operation '{operation}' requires a model designated as causal. "
            f"Call model.mark_as_causal() after verifying your graph represents "
            f"causal structure, or use observational_query() for statistical queries."
        )
        if suggestion:
            message += f" {suggestion}"
        super().__init__(message)
```

### API Contract

| Model Type | mark_as_causal() Required | do() Supported | query() Behavior |
|------------|---------------------------|----------------|------------------|
| Statistical BN | No (default) | No (raises) | P(Y \| X=x) - conditional probability |
| Causal BN | Yes | Yes | P(Y \| do(X=x)) - interventional probability |

This API design prevents the common error of interpreting conditional probabilities causally—a mistake that has led to flawed research conclusions and policy recommendations.

---

## Constructing Models

### Fluent API Design

Models can be constructed using a fluent, chainable API:

```python
model = (BayesianNetwork.builder()
    .add_variable('Difficulty', domain=['easy', 'hard'])
    .add_variable('Intelligence', domain=['low', 'high'])
    .add_variable('Grade', domain=['A', 'B', 'C'])
    .add_variable('SAT', domain=['low', 'high'])
    .add_variable('Letter', domain=['strong', 'weak'])
    .add_edge('Difficulty', 'Grade')
    .add_edge('Intelligence', 'Grade')
    .add_edge('Intelligence', 'SAT')
    .add_edge('Grade', 'Letter')
    .set_cpd('Difficulty', [0.6, 0.4])
    .set_cpd('Intelligence', [0.7, 0.3])
    .set_cpd('Grade', [
        # D=easy, I=low   D=easy, I=high  D=hard, I=low  D=hard, I=high
        [0.30,            0.90,           0.05,          0.50],      # A
        [0.40,            0.08,           0.25,          0.30],      # B
        [0.30,            0.02,           0.70,          0.20]       # C
    ])
    .set_cpd('SAT', [[0.95, 0.20], [0.05, 0.80]])
    .set_cpd('Letter', [[0.10, 0.60], [0.90, 0.40]])
    .build())
```

### Builder Pattern Implementation

```python
class BayesianNetworkBuilder:
    """Fluent builder for Bayesian networks."""
    
    def __init__(self):
        self._nodes = {}
        self._edges = []
        self._cpds = {}
    
    def add_variable(self, name: str, domain: List[str]) -> 'BayesianNetworkBuilder':
        """Add a variable with the specified domain."""
        self._nodes[name] = domain
        return self
    
    def add_variables(self, **variables: List[str]) -> 'BayesianNetworkBuilder':
        """Add multiple variables."""
        for name, domain in variables.items():
            self._nodes[name] = domain
        return self
    
    def add_edge(self, parent: str, child: str) -> 'BayesianNetworkBuilder':
        """Add a directed edge."""
        self._edges.append((parent, child))
        return self
    
    def add_edges_from(self, edges: List[Tuple[str, str]]) -> 'BayesianNetworkBuilder':
        """Add multiple edges."""
        self._edges.extend(edges)
        return self
    
    def set_cpd(self, variable: str, values: Union[List, np.ndarray]) -> 'BayesianNetworkBuilder':
        """Set CPD for a variable."""
        self._cpds[variable] = np.array(values)
        return self
    
    def build(self) -> BayesianNetwork:
        """Construct the Bayesian network."""
        model = BayesianNetwork()
        for name, domain in self._nodes.items():
            model.add_node(name, states=domain)
        for parent, child in self._edges:
            model.add_edge(parent, child)
        for variable, values in self._cpds.items():
            model.set_cpd(variable, values)
        return model
```

### Importing from NetworkX

```python
import networkx as nx

# Create NetworkX graph
nx_graph = nx.DiGraph()
nx_graph.add_edges_from([
    ('A', 'B'),
    ('A', 'C'),
    ('B', 'D'),
    ('C', 'D')
])

# Convert to Bayesian network
model = BayesianNetwork.from_networkx(nx_graph)

# Access NetworkX attributes preserved in conversion
print(model.to_networkx().nodes['A'])
```

### Importing from pandas

```python
import pandas as pd

# DataFrame with model specification
structure_df = pd.DataFrame({
    'variable': ['A', 'B', 'C', 'D'],
    'parents': [[], ['A'], ['A'], ['B', 'C']],
    'states': [['0', '1'], ['0', '1'], ['0', '1'], ['0', '1']]
})

# Create model from DataFrame
model = BayesianNetwork.from_dataframe(structure_df)
```

---

## Running Inference

### The Inference API

**Basic Query:**
```python
# Simple marginal query
result = model.query(variables=['Disease'])

# Conditional query with evidence
result = model.query(
    variables=['Disease'],
    evidence={'Symptom1': 'fever', 'Symptom2': 'cough'}
)

# Multiple query variables (joint distribution)
result = model.query(variables=['Disease', 'Treatment'])
```

**With Specific Algorithm:**
```python
# Explicit algorithm selection
result = model.query(
    variables=['Disease'],
    evidence={'Symptom': 'fever'},
    algorithm='variable_elimination',
    elimination_order=['Age', 'Gender', 'Location']
)

# Approximate inference
result = model.query(
    variables=['Disease'],
    evidence={'Symptom': 'fever'},
    algorithm='belief_propagation',
    max_iterations=100,
    tolerance=1e-6
)

# Sampling-based inference
result = model.query(
    variables=['Disease'],
    evidence={'Symptom': 'fever'},
    algorithm='gibbs_sampling',
    n_samples=10000,
    burn_in=1000
)
```

### Async Inference Options

For long-running inference tasks:

```python
import asyncio

async def run_inference():
    # Async query returns a coroutine
    result = await model.query_async(
        variables=['Disease'],
        evidence={'Symptom': 'fever'},
        timeout=300  # 5 minute timeout
    )
    return result

# Or with callback
def on_complete(result):
    print(f"Inference complete: {result}")

def on_error(error):
    print(f"Inference failed: {error}")

model.query_async(
    variables=['Disease'],
    evidence={'Symptom': 'fever'},
    callback=on_complete,
    error_callback=on_error
)
```

### MAP Inference

```python
# Most probable explanation
map_result = model.map_query(variables=['Disease', 'Treatment'])
# Returns: {'Disease': 'present', 'Treatment': 'antibiotics'}

# MAP with evidence
map_result = model.map_query(
    variables=['Disease'],
    evidence={'Symptom': 'fever'}
)

# All MAP assignments (if multiple)
map_results = model.map_query(
    variables=['Disease'],
    evidence={'Symptom': 'fever'},
    return_all=True
)
```

### Working with Results

```python
result = model.query(variables=['Grade', 'Letter'])

# Access individual marginals
grade_dist = result['Grade']
print(grade_dist)
# Grade
# A    0.362
# B    0.288
# C    0.350

# Access joint distribution
joint = result.joint
print(joint.shape)  # (3, 2) for Grade x Letter

# Convert to DataFrame
df = result.to_dataframe()
print(df)
#   Grade  Letter  probability
# 0     A  strong        0.289
# 1     A    weak        0.073
# ...

# Query specific values
prob_a_strong = result.get_probability(Grade='A', Letter='strong')
```

---

## Learning from Data

### Structure Learning API

```python
# Basic structure learning
from lutufi.learning.structure import HillClimbingSearch, PCAlgorithm

# Score-based learning
hc = HillClimbingSearch(
    scoring_method='bic',  # or 'bdeu', 'aic', 'mdl'
    max_indegree=4,        # Limit parent set size
    epsilon=1e-4,          # Score improvement threshold
    max_iter=1000,
    random_state=42
)
best_model = hc.fit(data)

# Constraint-based learning
pc = PCAlgorithm(
    independence_test='chi2',  # or 'g2', 'fisherz'
    significance_level=0.05,
    max_cond_vars=5
)
skeleton = pc.fit(data)

# Hybrid learning
from lutufi.learning.structure import MMHC
mmhc = MMHC(
    scoring_method='bic',
    independence_test='chi2'
)
model = mmhc.fit(data)

# With constraints
from lutufi.learning.structure import ConstraintBasedLearner

learner = ConstraintBasedLearner(
    required_edges=[('A', 'B')],      # Must include
    forbidden_edges=[('B', 'A')],     # Must exclude
    fixed_edges=[('C', 'D')]          # Pre-specified
)
model = learner.fit(data)
```

### Parameter Learning API

```python
# Maximum Likelihood Estimation
model.fit_parameters(data, algorithm='mle')

# Bayesian estimation with Dirichlet prior
model.fit_parameters(
    data,
    algorithm='bayesian',
    prior='bdeu',          # or 'uniform', 'k2'
    equivalent_sample_size=10
)

# EM for incomplete data
model.fit_parameters(
    data,
    algorithm='em',
    max_iterations=100,
    tolerance=1e-3
)

# Per-CPD learning with custom prior
cpd = model.get_cpd('Grade')
cpd.fit(
    data,
    algorithm='bayesian',
    prior=DirichletPrior(alpha=[[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]])
)
```

### The fit() Pattern

Consistent with scikit-learn conventions:

```python
from lutufi import BayesianNetwork

# Learn both structure and parameters
model = BayesianNetwork.fit(
    data,
    structure_algorithm='hc',
    structure_params={'scoring_method': 'bic'},
    parameter_algorithm='mle',
    parameter_params={}
)

# With cross-validation
from lutufi.learning import cross_validate_structure

best_model = cross_validate_structure(
    data,
    algorithms=['hc', 'pc'],
    cv=5,
    scoring='log_likelihood'
)
```

---

## Causal Queries

### The do() Operator API

```python
from lutufi.causal import do

# Interventional query: P(Y | do(X=1))
intervention_result = do(model).intervention(
    interventions={'Treatment': 1},
    query_variables=['Outcome']
)

# Multiple interventions
result = do(model).intervention(
    interventions={'Treatment': 1, 'Dosage': 'high'},
    query_variables=['Recovery'],
    evidence={'Age': 65}
)

# Using the causal module directly
from lutufi import causal

result = causal.query(
    model,
    query='P(Outcome | do(Treatment=1))',
    method='adjustment'  # or 'frontdoor', 'instrumental'
)
```

### Interventional Queries

```python
# Backdoor adjustment
result = model.causal.query(
    variables=['Outcome'],
    interventions={'Treatment': 1},
    adjustment_set=['Age', 'Gender'],
    method='backdoor'
)

# Frontdoor criterion
result = model.causal.query(
    variables=['Outcome'],
    interventions={'Treatment': 1},
    frontdoor_variables=['Mediator'],
    method='frontdoor'
)

# Instrumental variables
result = model.causal.query(
    variables=['Outcome'],
    interventions={'Treatment': 1},
    instrument='Z',
    method='iv'
)
```

### Counterfactual API

```python
from lutufi.causal.counterfactual import counterfactual

# Three steps of counterfactual reasoning
# 1. Abduction: Infer exogenous variables from evidence
# 2. Action: Modify model with intervention
# 3. Prediction: Compute counterfactual

cf_result = counterfactual(
    model,
    factual_evidence={'Treatment': 0, 'Outcome': 'sick'},
    intervention={'Treatment': 1},
    query_variable='Outcome'
)
# Returns: Probability of 'healthy' if Treatment had been 1
# given that with Treatment=0, Outcome was 'sick'

# Batch counterfactuals
cf_results = counterfactual_batch(
    model,
    factual_data=df,  # DataFrame with actual observations
    intervention={'Treatment': 1},
    query_variable='Outcome'
)
```

### Causal Effect Estimation

```python
# Average Treatment Effect (ATE)
ate = model.causal.ate(
    treatment='Treatment',
    outcome='Outcome',
    adjustment_set=['Age', 'Gender']
)

# Conditional Average Treatment Effect (CATE)
cate = model.causal.cate(
    treatment='Treatment',
    outcome='Outcome',
    condition={'Age': 65},
    adjustment_set=['Gender']
)

# Causal effect identification
is_identifiable, formula = model.causal.identify_effect(
    treatment='Treatment',
    outcome='Outcome'
)
if is_identifiable:
    print(f"Identified: {formula}")
```

---

## Network Analysis Integration

### NetworkX Compatibility

```python
import networkx as nx
from lutufi import BayesianNetwork

# Convert to NetworkX for analysis
model = BayesianNetwork.fit(data)
nx_graph = model.to_networkx()

# Use NetworkX algorithms
centrality = nx.betweenness_centrality(nx_graph)
clustering = nx.average_clustering(nx_graph.to_undirected())
paths = nx.all_simple_paths(nx_graph, source='A', target='D')

# Convert back after modification
nx_graph.add_edge('E', 'F')
model_updated = BayesianNetwork.from_networkx(nx_graph)
```

### Converting Between Representations

```python
# Bayesian Network to Markov Random Field (moralization)
mrf = model.to_markov_random_field()

# MRF to BN (chordal graph and orientation)
from lutufi.networks.conversion import mrf_to_bn
model = mrf_to_bn(mrf, ordering=['A', 'B', 'C', 'D'])

# Factor graph conversions
fg = model.to_factor_graph()
bn_restored = fg.to_bayesian_network()
```

### Network Measures on Probabilistic Networks

```python
# Probabilistic centrality measures
from lutufi.networks.measures import (
    probabilistic_betweenness,
    information_centrality,
    causal_influence
)

# Influence considering probability flow
influence = probabilistic_betweenness(model, evidence={'Source': 1})

# Mutual information between nodes
from lutufi.networks.measures import mutual_information_graph
mi_graph = mutual_information_graph(model, data)

# Causal strength of edges
causal_strength = model.get_edge_strength(method='mutual_information')
```

---

## DataFrame Integration

### pandas DataFrame Input/Output

```python
import pandas as pd

# DataFrame input for learning
data = pd.DataFrame({
    'Difficulty': ['easy', 'hard', 'easy', 'hard'],
    'Intelligence': ['high', 'low', 'high', 'high'],
    'Grade': ['A', 'C', 'B', 'A'],
    'SAT': ['high', 'low', 'high', 'high']
})

model = BayesianNetwork.fit(data)

# DataFrame output from queries
result = model.query(['Grade'])
result_df = result.to_dataframe()
print(result_df)
#   Grade  probability
# 0     A        0.362
# 1     B        0.288
# 2     C        0.350

# DataFrame output from sampling
samples = model.sample(n_samples=1000)
print(samples.head())
#   Difficulty Intelligence Grade   SAT
# 0        easy         high     A  high
# 1        hard          low     C   low
```

### Automatic Type Inference

```python
# Automatic domain detection
data = pd.DataFrame({
    'Age': [25, 30, 35, 40, 45],           # Continuous
    'Gender': ['M', 'F', 'M', 'F', 'M'],   # Categorical (string)
    'Score': [85, 90, 78, 92, 88]         # Continuous
})

model = BayesianNetwork.fit(data)
# Automatically determines:
# - Age: Continuous variable
# - Gender: Discrete with domain ['F', 'M']
# - Score: Continuous variable
```

### Column Mapping

```python
# Explicit column type specification
model = BayesianNetwork.fit(
    data,
    column_types={
        'Age': 'continuous',
        'Income': 'continuous',
        'Category': 'categorical'
    },
    discrete_domains={
        'Category': ['A', 'B', 'C']
    }
)

# Rename columns in output
result = model.query(['Outcome'])
result_df = result.to_dataframe(
    column_map={'Outcome': 'Predicted_Outcome'}
)
```

---

## Visualization API

### Programmatic Visualization

```python
from lutufi import viz

# Basic network plot
viz.plot_network(model)

# With styling
viz.plot_network(
    model,
    node_color='lightblue',
    node_size=2000,
    edge_color='gray',
    layout='spring',
    with_labels=True,
    label_font_size=10
)

# Highlight specific nodes/edges
viz.plot_network(
    model,
    highlight_nodes=['Disease'],
    highlight_edges=[('Symptom', 'Disease')],
    highlight_color='red'
)
```

### Matplotlib Integration

```python
import matplotlib.pyplot as plt
from lutufi.viz import plot_network, plot_inference_results

fig, axes = plt.subplots(1, 2, figsize=(14, 6))

# Plot network structure
plot_network(model, ax=axes[0], title='Network Structure')

# Plot inference results
result = model.query(['Grade'])
plot_inference_results(result, ax=axes[1], title='Query Results')

plt.tight_layout()
plt.savefig('analysis.png', dpi=300)
```

### Interactive Options

```python
# Interactive plotly visualization
viz.plot_network_interactive(
    model,
    hover_info=['probability', 'states'],
    layout='hierarchical'
)

# Jupyter widget
from lutufi.viz import NetworkWidget

widget = NetworkWidget(model)
widget.show()
# Allows: zoom, pan, click nodes for details

# Animation for dynamic networks
viz.animate_dbn(
    dbn_model,
    time_steps=range(10),
    interval=500  # milliseconds
)
```

### Learning Visualization

```python
from lutufi.viz import plot_learning_curve, plot_structure_comparison

# Plot score during structure learning
plot_learning_curve(learner.history)

# Compare learned structures
plot_structure_comparison(
    learned_model,
    true_model,
    comparison='edges'  # or 'skeleton', 'v-structures'
)
```

---

## Configuration and Options

### Global Settings

```python
import lutufi as lf

# Global configuration
lf.config.set({
    'inference.default_algorithm': 'variable_elimination',
    'inference.max_memory_gb': 8,
    'learning.random_state': 42,
    'logging.level': 'INFO',
    'parallel.n_jobs': -1,  # Use all cores
    'numerical.tolerance': 1e-9
})

# Or load from file
lf.config.load('lutufi_config.yaml')
```

### Per-Model Settings

```python
model = BayesianNetwork()
model.config = {
    'inference_timeout': 300,
    'cache_enabled': True,
    'cache_size_mb': 512
}
```

### Configuration Files

```yaml
# lutufi_config.yaml
inference:
  default_algorithm: variable_elimination
  max_memory_gb: 16
  cache:
    enabled: true
    size_mb: 1024

learning:
  random_state: 42
  structure:
    default_algorithm: hill_climbing
    max_indegree: 6
  parameter:
    default_prior: bdeu
    equivalent_sample_size: 10

logging:
  level: INFO
  format: "%(asctime)s - %(name)s - %(levelname)s - %(message)s"

parallel:
  n_jobs: -1
  backend: loky
```

---

## Error Handling in the API

### Exception Hierarchy

```python
# Base exception
class LutufiError(Exception):
    """Base class for all Lutufi exceptions."""
    pass

# Model errors
class ModelError(LutufiError):
    """Errors related to model structure or validity."""
    pass

class StructureError(ModelError):
    """Invalid graph structure."""
    pass

class CPDError(ModelError):
    """Invalid or inconsistent CPD."""
    pass

# Inference errors
class InferenceError(LutufiError):
    """Errors during inference."""
    pass

class ConvergenceError(InferenceError):
    """Algorithm failed to converge."""
    pass

class InsufficientMemoryError(InferenceError):
    """Not enough memory for inference."""
    pass

# Learning errors
class LearningError(LutufiError):
    """Errors during learning."""
    pass

class StructureLearningError(LearningError):
    """Structure learning failed."""
    pass

class ParameterLearningError(LearningError):
    """Parameter learning failed."""
    pass
```

### Error Messages

```python
try:
    model.add_edge('A', 'A')
except StructureError as e:
    print(e)
    # Output: "Cannot add self-loop to 'A': Bayesian networks must be DAGs. "
    #         "Consider using a MarkovRandomField for cyclic dependencies."

try:
    model.query(['X'], algorithm='exact')
except InferenceError as e:
    print(e)
    # Output: "Exact inference intractable for this model (treewidth: 47). "
    #         "Try: algorithm='belief_propagation' or reduce model complexity."

try:
    model.set_cpd('Grade', [[0.3, 0.5], [0.4, 0.8]])
except CPDError as e:
    print(e)
    # Output: "CPD for 'Grade' has inconsistent probabilities: "
    #         "Column 0 sums to 0.7, Column 1 sums to 1.3. "
    #         "Use normalize=True to automatically normalize."
```

### Recovery Suggestions

```python
try:
    result = model.query(variables=['X', 'Y', 'Z'], evidence={'A': 1})
except InsufficientMemoryError as e:
    print(f"Error: {e}")
    print("Suggestions:")
    for suggestion in e.suggestions:
        print(f"  - {suggestion}")
    # Suggestions:
    #   - Use approximate inference: algorithm='belief_propagation'
    #   - Query fewer variables at once
    #   - Use a machine with more RAM
    #   - Enable memory-mapped files: lutufi.config.set({'memory.mapped': True})
```

---

## Error Messages Section

Lutufi provides specific, actionable error messages for common mistakes. Each error includes:
- Clear explanation of what went wrong
- The specific values or constraints violated
- Suggested fixes or alternatives
- Links to relevant documentation where applicable

### 1. Cyclic Graph to BayesianNetwork

**Error:**
```
LutufiCyclicGraphError: BayesianNetwork requires an acyclic graph.
Detected cycle: A → B → C → A.
Consider using MarkovRandomField or DynamicBayesianNetwork for cyclic structures.
```

**Context:** Attempted to create a Bayesian network from a graph containing cycles.

**Resolution:**
```python
# Option 1: Use MarkovRandomField for undirected/cyclic structures
model = MarkovRandomField(cyclic_graph)

# Option 2: Use DynamicBayesianNetwork for temporal cycles
model = DynamicBayesianNetwork(unrolled_temporal_graph)

# Option 3: Break the cycle with domain knowledge
acyclic_graph = break_cycle(graph, edge_to_remove=('C', 'A'))
model = BayesianNetwork(acyclic_graph)
```

### 2. CPT Rows Don't Sum to One

**Error:**
```
LutufiCPTError: CPD for 'Grade' has invalid probabilities.
Row for parents (Difficulty=easy, Intelligence=high) sums to 0.9, expected 1.0.
Values provided: [0.3, 0.4, 0.2]
Hint: Use normalize=True to automatically normalize, or check your probability values.
```

**Context:** Conditional probability table row does not sum to 1.0 within tolerance.

**Resolution:**
```python
# Option 1: Auto-normalize
cpd = TabularCPD('Grade', 3, values=[[0.3, 0.4, 0.2]],
                 evidence=['Difficulty', 'Intelligence'],
                 normalize=True)

# Option 2: Manual fix
import numpy as np
values = np.array([[0.3, 0.4, 0.2]])
values = values / values.sum(axis=1, keepdims=True)
```

### 3. Interventional Query on Non-Causal Model

**Error:**
```
LutufiNonCausalModelError: Causal operations (do-operator) require a model designated as causal.
Call model.mark_as_causal() after verifying your graph represents causal structure,
or use observational_query() for statistical queries.
See: https://docs.lutufi.org/causal/required-designation.html
```

**Context:** Attempted causal query on statistical-only model.

**Resolution:**
```python
# Option 1: Designate as causal (if appropriate)
model.mark_as_causal()
result = model.do("Treatment", x=1).query("Outcome")

# Option 2: Use observational query
result = model.observational_query("Outcome", evidence={"Treatment": 1})
```

### 4. Querying Non-Existent Variable

**Error:**
```
LutufiVariableError: Variable 'Symptom' not found in model.
Available variables: ['Disease', 'Test_Result', 'Treatment_Response']
Did you mean: 'Symptom_Level'? (Similarity: 0.85)
To add missing variables: model.add_node('Symptom', states=['present', 'absent'])
```

**Context:** Query referenced a variable not in the model, with typo suggestion.

**Resolution:**
```python
# Check available variables
print(model.nodes())

# Add the missing variable
model.add_node('Symptom', states=['present', 'absent'])
model.set_cpd('Symptom', ...)
```

### 5. Evidence Out of Domain

**Error:**
```
LutufiDomainError: Evidence 'blue' for variable 'Color' is not in the domain.
Valid values: ['red', 'green', 'yellow']
Character distance analysis: 'blue' → did you mean 'red'? (Levenshtein distance: 4)
```

**Context:** Observed value not in variable's defined domain.

**Resolution:**
```python
# Check the domain
print(model.get_cpds('Color').state_names)
# Output: ['red', 'green', 'yellow']

# Use valid evidence
result = model.query(['Disease'], evidence={'Color': 'red'})
```

### 6. Inference Algorithm Mismatch

**Error:**
```
LutufiInferenceError: Exact inference intractable for this model (treewidth: 47, max recommended: 25).
Estimated memory required: 48 GB
Estimated time: >2 hours

Suggested alternatives:
  1. Use approximate inference: algorithm='loopy_belief_propagation'
  2. Reduce query variables (currently querying 8 variables)
  3. Enable variable elimination heuristics: elimination_order='min_fill'
  4. Use sampling-based inference: algorithm='gibbs_sampling', n_samples=10000
```

**Context:** Model complexity exceeds exact inference capabilities.

**Resolution:**
```python
# Use approximate inference
result = model.query(['Outcome'],
                    evidence={'Symptom': 'fever'},
                    algorithm='loopy_belief_propagation',
                    max_iterations=100)
```

### 7. Missing Parameter Specification

**Error:**
```
LutufiParameterError: Model contains 3 nodes without CPDs: ['Smoking', 'Cancer', 'Outcome'].
Nodes with CPDs: ['Age', 'Gender']
To complete the model:
  model.set_cpd('Smoking', [0.3, 0.7])
  model.set_cpd('Cancer', [[0.1, 0.05], [0.9, 0.95]])
  model.set_cpd('Outcome', ...)
```

**Context:** Model incomplete—some nodes lack probability tables.

**Resolution:**
```python
# Add missing CPDs
model.set_cpd('Smoking', [0.3, 0.7])  # Prior
model.set_cpd('Cancer', [[0.1, 0.05], [0.9, 0.95]])  # Conditional
model.check_model()  # Verify completeness
```

### 8. Invalid Edge Direction

**Error:**
```
LutufiStructureError: Edge direction A → B creates a cycle.
Current path exists: B → C → D → A.
Adding A → B would create: B → C → D → A → B (cycle detected).

To model cyclic dependencies, consider:
  1. Use MarkovRandomField for undirected relationships
  2. Use DynamicBayesianNetwork for temporal feedback
  3. Re-examine causal direction (may need expert input)
```

**Context:** Edge addition would create a cycle in a DAG.

### 9. Incompatible Model Conversion

**Error:**
```
LutufiConversionError: Cannot convert MarkovRandomField to BayesianNetwork.
MRF contains symmetric edges that cannot be directed without introducing cycles:
  Undirected edges: [(A, B), (B, C), (C, A)]

Conversion options:
  1. Moralization (BN → MRF) is always possible: bn.to_markov_random_field()
  2. Chordal graph + perfect elimination ordering required for MRF → BN
  3. Use FactorGraph as intermediate representation: mrf.to_factor_graph()
```

**Context:** Attempted invalid model conversion.

### 10. Convergence Failure in Approximate Inference

**Error:**
```
LutufiConvergenceError: Loopy belief propagation failed to converge after 100 iterations.
Final residual: 0.0012 (tolerance: 0.0001)

Diagnostic: Network contains near-deterministic relationships that may cause oscillations.

Solutions:
  1. Increase damping: damping=0.5 (current: 0.0)
  2. Increase max_iterations: max_iterations=500
  3. Use generalized belief propagation for improved convergence
  4. Switch to sampling: algorithm='gibbs_sampling'
  5. Check for deterministic nodes: model.has_deterministic_relationships()
```

**Context:** Iterative inference algorithm failed to converge.

### 11. Memory Exhaustion

**Error:**
```
LutufiMemoryError: Inference would require 124 GB of memory.
Available system memory: 16 GB.

Memory breakdown:
  Junction tree creation: ~89 GB
  Factor storage: ~25 GB
  Message passing: ~10 GB

Mitigation strategies:
  1. Use approximate inference (memory: ~2 GB)
  2. Enable memory-mapped factors: config.memory.mapped = True
  3. Batch query processing for large evidence sets
  4. Reduce precision: config.numerical.float_precision = 'float32'
```

**Context:** Model too large for available memory.

### 12. File Format Error

**Error:**
```
LutufiIOError: Failed to parse model file 'model.bif'.
Parse error at line 127: Expected 'probability' keyword, found 'probabiltiy'.
Position: 127:5 (character 4831)

Common BIF syntax errors:
  - Missing semicolon at end of probability block
  - Undeclared variable referenced
  - Probability values not summing to 1.0

See BIF specification: https://www.cs.washington.edu/dm/bnlearn/bif.htm
```

**Context:** Model file parsing failed with specific location.

---

## Type Hints and Documentation

### Python Type Hints

```python
from typing import List, Dict, Optional, Union, Tuple, Any
import numpy as np
import pandas as pd

class BayesianNetwork:
    def query(
        self,
        variables: List[str],
        evidence: Optional[Dict[str, Any]] = None,
        algorithm: Optional[str] = None,
        **kwargs: Any
    ) -> 'QueryResult':
        """
        Execute a probabilistic query.
        
        Parameters
        ----------
        variables : list of str
            Variables to query for marginal distributions.
        evidence : dict, optional
            Observed values mapping variable names to values.
        algorithm : str, optional
            Inference algorithm. Options: 'variable_elimination',
            'belief_propagation', 'gibbs_sampling'. Default: auto-select.
        **kwargs
            Additional algorithm-specific parameters.
        
        Returns
        -------
        QueryResult
            Query results containing probability distributions.
        
        Raises
        ------
        ValueError
            If variables not in model or invalid evidence.
        InferenceError
            If inference fails or is intractable.
        
        Examples
        --------
        >>> model = BayesianNetwork()
        >>> # ... build model ...
        >>> result = model.query(['Disease'], evidence={'Symptom': 'fever'})
        >>> print(result['Disease'])
        """
        pass
```

### Docstring Standards

Following NumPy docstring conventions:

```python
def fit_structure(
    data: pd.DataFrame,
    algorithm: str = 'hc',
    scoring_method: str = 'bic',
    **kwargs: Any
) -> BayesianNetwork:
    """
    Learn network structure from data.
    
    This function performs structure learning using the specified
    algorithm to discover the dependency structure among variables.
    
    Parameters
    ----------
    data : pd.DataFrame
        Training data with one row per observation and one column
        per variable.
    algorithm : str, default='hc'
        Structure learning algorithm:
        - 'hc': Hill climbing (score-based)
        - 'pc': PC algorithm (constraint-based)
        - 'mmhc': Max-Min Hill Climbing (hybrid)
    scoring_method : str, default='bic'
        Scoring function for score-based algorithms:
        - 'bic': Bayesian Information Criterion
        - 'bdeu': Bayesian Dirichlet equivalent uniform
        - 'aic': Akaike Information Criterion
    **kwargs
        Algorithm-specific parameters. See individual algorithm
        documentation.
    
    Returns
    -------
    BayesianNetwork
        Learned model with fitted structure (parameters not learned).
    
    See Also
    --------
    fit_parameters : Learn parameters given structure.
    BayesianNetwork.fit : Learn both structure and parameters.
    
    References
    ----------
    .. [1] Koller & Friedman, "Probabilistic Graphical Models",
           Chapter 18: Structure Learning.
    .. [2] Tsamardinos et al., "The max-min hill-climbing Bayesian
           network structure learning algorithm", Machine Learning, 2006.
    
    Examples
    --------
    >>> import pandas as pd
    >>> from lutufi import fit_structure
    >>> data = pd.read_csv('data.csv')
    >>> model = fit_structure(data, algorithm='hc')
    >>> print(model.edges())
    """
    pass
```

### API Documentation Generation

Documentation is generated using Sphinx with:
- **autodoc**: Automatic API documentation from docstrings
- **napoleon**: Support for NumPy-style docstrings
- **typehints**: Automatic inclusion of type hints
- **examples**: Executable examples using doctest

---

## Example Workflows

### Complete Example: Medical Diagnosis

```python
import lutufi as lf
import pandas as pd
import numpy as np

# Step 1: Build the model
model = lf.BayesianNetwork()

# Add variables
model.add_node('Smoking', states=['yes', 'no'])
model.add_node('LungCancer', states=['present', 'absent'])
model.add_node('Bronchitis', states=['present', 'absent'])
model.add_node('Cough', states=['severe', 'mild', 'none'])
model.add_node('Fatigue', states=['yes', 'no'])
model.add_node('ChestXRay', states=['positive', 'negative'])

# Add structure
model.add_edges_from([
    ('Smoking', 'LungCancer'),
    ('Smoking', 'Bronchitis'),
    ('LungCancer', 'Cough'),
    ('LungCancer', 'Fatigue'),
    ('LungCancer', 'ChestXRay'),
    ('Bronchitis', 'Cough'),
    ('Bronchitis', 'Fatigue')
])

# Set CPDs
model.set_cpd('Smoking', [0.3, 0.7])
model.set_cpd('LungCancer', [
    [0.1, 0.01],  # P(LC=yes | Smoking=yes), P(LC=yes | Smoking=no)
    [0.9, 0.99]
])
model.set_cpd('Bronchitis', [
    [0.6, 0.3],
    [0.4, 0.7]
])
# ... set remaining CPDs

# Step 2: Validate
assert model.check_model()

# Step 3: Query
# Diagnostic query
result = model.query(
    variables=['LungCancer', 'Bronchitis'],
    evidence={
        'Smoking': 'yes',
        'Cough': 'severe',
        'ChestXRay': 'positive'
    }
)
print(result)

# Prognostic query (with intervention)
from lutufi.causal import do
result = do(model).intervention(
    interventions={'Smoking': 'no'},
    query_variables=['LungCancer'],
    evidence={'Cough': 'severe'}
)
print(f"P(LungCancer | do(Smoking=no), Cough=severe): {result}")

# Step 4: Visualize
from lutufi import viz
import matplotlib.pyplot as plt

fig, axes = plt.subplots(1, 2, figsize=(14, 6))
viz.plot_network(model, ax=axes[0])
result.plot(ax=axes[1])
plt.savefig('diagnosis_analysis.png')
```

### Complete Example: Social Network Analysis

```python
import lutufi as lf
import networkx as nx
import pandas as pd

# Load social network data
nx_graph = nx.karate_club_graph()

# Convert to Markov Random Field for influence modeling
mrf = lf.MarkovRandomField.from_networkx(nx_graph)

# Add factors for influence propagation
# Each node has a binary state: active/inactive
for node in mrf.nodes():
    # Individual activation potential
    mrf.add_factor([node], [0.3, 0.7])  # Prior: 30% active

for edge in mrf.edges():
    # Pairwise interaction: active nodes activate neighbors
    mrf.add_factor(
        list(edge),
        [[0.9, 0.5],   # Both inactive, one active
         [0.5, 0.1]]   # Both active
    )

# Infer influence spread
result = mrf.query(
    variables=['0', '1', '2'],
    evidence={'33': 'active'}  # Club instructor is active
)

# Analyze influence paths
from lutufi.networks.measures import probabilistic_betweenness
influence = probabilistic_betweenness(mrf, evidence={'33': 'active'})
top_influencers = sorted(influence.items(), key=lambda x: x[1], reverse=True)[:5]

print("Top influencers:", top_influencers)
```

### Complete Example: Learning from Data

```python
import lutufi as lf
import pandas as pd
from sklearn.model_selection import train_test_split

# Load data
data = pd.read_csv('sensor_data.csv')
train_data, test_data = train_test_split(data, test_size=0.2)

# Learn structure
print("Learning structure...")
model = lf.BayesianNetwork.fit(
    train_data,
    structure_algorithm='mmhc',
    structure_params={'scoring_method': 'bic'},
    parameter_algorithm='bayesian',
    parameter_params={'prior': 'bdeu', 'equivalent_sample_size': 10}
)

print(f"Learned {len(model.edges())} edges")
print(f"Learned structure: {list(model.edges())}")

# Evaluate
from lutufi.learning import log_likelihood

train_ll = log_likelihood(model, train_data)
test_ll = log_likelihood(model, test_data)
print(f"Train log-likelihood: {train_ll:.2f}")
print(f"Test log-likelihood: {test_ll:.2f}")

# Predict missing values
predictions = model.predict(test_data, variables=['Sensor_Fault'])
accuracy = (predictions == test_data['Sensor_Fault']).mean()
print(f"Prediction accuracy: {accuracy:.2%}")

# Save model
model.save('sensor_model.lutufi')
```

### Complete Example: Causal Inference

```python
import lutufi as lf
from lutufi.causal import do, counterfactual
import pandas as pd

# Load observational data
obs_data = pd.read_csv('treatment_study.csv')

# Learn causal model
print("Learning causal structure...")
model = lf.BayesianNetwork.fit(obs_data)

# Estimate Average Treatment Effect (ATE)
ate = model.causal.ate(
    treatment='Treatment',
    outcome='Recovery',
    adjustment_set=model.get_parents('Treatment')
)
print(f"Average Treatment Effect: {ate:.3f}")

# Counterfactual: What if patient 42 had received treatment?
patient_42 = obs_data.iloc[42]
print(f"Actual outcome: {patient_42['Recovery']}")

cf_result = counterfactual(
    model,
    factual_evidence=patient_42.to_dict(),
    intervention={'Treatment': 1},
    query_variable='Recovery'
)
print(f"Counterfactual P(Recovery=1 | do(Treatment=1)): {cf_result:.3f}")

# Policy evaluation
print("\nEvaluating treatment policies...")
for threshold in [0.3, 0.5, 0.7]:
    # Treat only if predicted risk > threshold
    policy_effect = model.causal.evaluate_policy(
        treatment='Treatment',
        outcome='Recovery',
        policy=lambda x: x['Risk'] > threshold
    )
    print(f"  Risk threshold {threshold}: Effect = {policy_effect:.3f}")
```

---

## API Stability and Deprecation

### Deprecation Policy

**Version Lifecycle:**
1. **Experimental (0.x)**: API may change without deprecation
2. **Stable (1.x+)**: Deprecation required before removal

**Deprecation Timeline:**
```
Version N:    Feature marked deprecated, warning issued
Version N+1:  Feature still works, warning continues
Version N+2:  Feature removed, migration guide provided
```

### Deprecation Examples

```python
# In version 0.5.0
@deprecated(
    version='0.5.0',
    removal='0.7.0',
    alternative='query()',
    message='Use query() instead. Will be removed in 0.7.0.'
)
def infer_marginal(*args, **kwargs):
    return query(*args, **kwargs)

# In version 0.6.0 (still works with warning)
model.infer_marginal(['A'])  # DeprecationWarning: Use query() instead

# In version 0.7.0
model.infer_marginal(['A'])  # AttributeError: 'BayesianNetwork' has no attribute 'infer_marginal'
```

### Migration Guides

**When APIs change, migration guides include:**
- Before/after code examples
- Rationale for the change
- Automated migration scripts when possible
- Timeline for support

```markdown
# Migration Guide: 0.6.0 → 0.7.0

## Renamed Methods

### `infer_marginal()` → `query()`

**Before:**
```python
result = model.infer_marginal(['A'], evidence={'B': 1})
```

**After:**
```python
result = model.query(['A'], evidence={'B': 1})
```

## Changed Parameters

### `fit_structure()` scoring_method

**Before:**
```python
model = fit_structure(data, score='bic')
```

**After:**
```python
model = fit_structure(data, scoring_method='bic')
```

## Removed Features

### `Model.validate()` (use `Model.check_model()`)
```

### Version Compatibility

```python
import lutufi as lf

# Check version
print(lf.__version__)  # '0.7.0'

# Version compatibility helpers
if lf.version_at_least('1.0.0'):
    # Use new API
    result = model.query(...)
else:
    # Use legacy API
    result = model.infer_marginal(...)
```

---

## Key References

### Well-Designed Library APIs

1. **scikit-learn API Design**
   - Consistent `fit()`, `predict()`, `transform()` pattern
   - Estimator base classes and mixins
   - https://scikit-learn.org/stable/developers/develop.html

2. **NetworkX API Design**
   - Graph classes with dict-like node/edge access
   - Algorithm organization by function
   - https://networkx.org/documentation/stable/

3. **PyTorch API Design**
   - Module composition and parameter management
   - Device-agnostic tensor operations
   - https://pytorch.org/docs/stable/

4. **pandas API Design**
   - DataFrame as primary data structure
   - Method chaining with consistent return types
   - https://pandas.pydata.org/docs/

5. **Stan API Design** (cmdstanpy)
   - Separation of model compilation and execution
   - Data dictionary interface
   - https://mc-stan.org/users/interfaces/cmdstanpy

6. **pgmpy API Reference** (Python PGM library)
   - Probabilistic graphical model API patterns
   - Similar problem domain
   - https://pgmpy.org/

### API Design Principles

7. **McKinney, W. (2010).** "Data Structures for Statistical Computing in Python." *Proceedings of the 9th Python in Science Conference.*
   - DataFrame design principles

8. **Beyer, M. (2018).** "API Design Patterns." *O'Reilly Media.*
   - General API design patterns

9. **Google (2023).** "Google API Design Guide." https://cloud.google.com/apis/design
   - Resource-oriented design
   - Standard methods and fields

10. **Microsoft (2023).** "Azure REST API Guidelines." https://github.com/microsoft/api-guidelines
    - REST API design (applicable concepts)

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial draft |
| 1.0 | 2026-03-03 | Wasswa Lutufi Sebbanja | Complete API design document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
