"""Lutufi - High-performance network analysis library with probabilistic reasoning.

This package provides Python bindings for the Lutufi Rust core, offering:
- Network/graph representation and manipulation
- Probabilistic inference on network structures
- Learning network parameters from data
- I/O operations for network formats

Example:
    >>> import lutufi
    >>> print(lutufi.__version__)
    '0.1.0'
"""

try:
    from lutufi._lutufi import (
        __version__,
        models,
        inference,
        learning,
        io,
    )
except ImportError:
    # Fallback when the Rust extension is not built
    __version__ = "0.1.0"
    models = None
    inference = None
    learning = None
    io = None

from lutufi.models import NetworkModel
from lutufi.inference import InferenceEngine
from lutufi.learning import LearningEngine
from lutufi.io import read_graph, write_graph

__all__ = [
    "__version__",
    "models",
    "inference",
    "learning",
    "io",
    "NetworkModel",
    "InferenceEngine",
    "LearningEngine",
    "read_graph",
    "write_graph",
]