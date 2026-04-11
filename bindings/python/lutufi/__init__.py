"""Lutufi - High-performance network analysis library with probabilistic reasoning.

This package provides Python bindings for the Lutufi Rust core, offering:
- Network/graph representation and manipulation
- Probabilistic inference on network structures
- Learning network parameters from data
- I/O operations for network formats

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
    # Fallback when the Rust extension is not built
    __version__ = "0.1.0-dev"
    models = None
    inference = None
    learning = None
    io = None
    Variable = None
    Domain = None

from lutufi.inference import InferenceEngine, QueryResult, JunctionTreeEngine, LutufiHighTreewidthWarning
from lutufi.learning import LearningEngine
from lutufi.io import read_graph, write_graph
from lutufi.models import BayesianNetwork, LutufiError, LutufiValidationError, LutufiNonCausalError

__all__ = [
    "__version__",
    "models",
    "inference",
    "learning",
    "io",
    "Variable",
    "Domain",
    "BayesianNetwork",
    "LutufiError",
    "LutufiValidationError",
    "LutufiNonCausalError",
    "InferenceEngine",
    "LearningEngine",
    "QueryResult",
    "JunctionTreeEngine",
    "LutufiHighTreewidthWarning",
    "read_graph",
    "write_graph",
]
