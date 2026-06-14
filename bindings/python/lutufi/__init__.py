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
    '1.0.0'
"""

try:
    from lutufi._lutufi import (
        __version__,
        Variable,
        Domain,
    )
except ImportError as e:
    raise ImportError(
        "Lutufi native extension ('_lutufi') failed to import.\n"
        "Run 'maturin develop --release' from the repository root to build it,\n"
        "or install a prebuilt wheel with 'pip install lutufi'.\n"
        "See INSTALL.md for platform-specific instructions."
    ) from e

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
