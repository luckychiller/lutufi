"""Lutufi - High-performance network analysis library with probabilistic reasoning.

This package provides Python bindings for the Lutufi Rust core, offering:
- Network/graph representation and manipulation
- Probabilistic inference on network structures
- Learning network parameters from data
- I/O operations for network formats
- Causal inference via do-calculus

Example:
    >>> import lutufi
    >>> print(lutufi.__version__)
    '0.1.0-dev'
"""

try:
    from lutufi._lutufi import (
        __version__,
        models,
        inference,
        learning,
        io,
        Variable,
        Domain,
    )
except ImportError:
    __version__ = "0.1.0-dev"
    models = None
    inference = None
    learning = None
    io = None
    Variable = None
    Domain = None

from lutufi.inference import (
    InferenceEngine,
    QueryResult,
    LazyQueryResult,
    InferenceResult,
    InferenceMetadata,
    InferenceOptions,
    JunctionTreeEngine,
    LutufiHighTreewidthWarning,
    BeliefPropagation,
    LoopyBeliefPropagation,
    GibbsSampler,
    VariationalInference,
)
from lutufi.learning import ParameterEstimator, StructureLearner, fit, learn_structure
from lutufi.io import read_graph, write_graph
from lutufi.models import (
    BayesianNetwork,
    MarkovRandomField,
    DynamicBayesianNetwork,
    BayesianNetworkBuilder,
    LutufiError,
    LutufiValidationError,
    LutufiNonCausalError,
    LutufiCyclicGraphError,
    LutufiNotIdentifiableError,
    LutufiNumericalError,
    LutufiResourceLimitError,
    LutufiSerializationError,
    LutufiMissingDataError,
    LutufiConvergenceWarning,
    NetworkModel,
)

__all__ = [
    "__version__",
    "Variable",
    "Domain",
    "BayesianNetwork",
    "MarkovRandomField",
    "DynamicBayesianNetwork",
    "BayesianNetworkBuilder",
    "NetworkModel",
    "LutufiError",
    "LutufiValidationError",
    "LutufiNonCausalError",
    "LutufiCyclicGraphError",
    "LutufiNotIdentifiableError",
    "LutufiNumericalError",
    "LutufiResourceLimitError",
    "LutufiSerializationError",
    "LutufiMissingDataError",
    "LutufiConvergenceWarning",
    "LutufiHighTreewidthWarning",
    "InferenceEngine",
    "InferenceResult",
    "InferenceMetadata",
    "InferenceOptions",
    "QueryResult",
    "LazyQueryResult",
    "JunctionTreeEngine",
    "BeliefPropagation",
    "LoopyBeliefPropagation",
    "GibbsSampler",
    "VariationalInference",
    "ParameterEstimator",
    "StructureLearner",
    "fit",
    "learn_structure",
    "read_graph",
    "write_graph",
]
