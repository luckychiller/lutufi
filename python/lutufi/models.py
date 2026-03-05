"""Network models module.

This module provides classes and functions for working with network models
including Bayesian networks, Markov random fields, and exponential random
graph models.
"""

from typing import Optional, Dict, Any, List
import numpy as np


class NetworkModel:
    """Base class for all network models.
    
    This class provides common functionality for network models and serves
    as the foundation for specific model implementations.
    
    Attributes:
        name: Model name
        metadata: Model metadata dictionary
    
    Example:
        >>> model = NetworkModel(name="my_model")
        >>> print(model.name)
        'my_model'
    """
    
    def __init__(self, name: str = "unnamed", metadata: Optional[Dict[str, Any]] = None):
        """Initialize a network model.
        
        Args:
            name: Model name
            metadata: Optional metadata dictionary
        """
        self._name = name
        self._metadata = metadata or {}
        self._node_count = 0
        self._edge_count = 0
    
    @property
    def name(self) -> str:
        """Get the model name."""
        return self._name
    
    @name.setter
    def name(self, value: str) -> None:
        """Set the model name."""
        self._name = value
    
    @property
    def metadata(self) -> Dict[str, Any]:
        """Get model metadata."""
        return self._metadata
    
    @property
    def node_count(self) -> int:
        """Get the number of nodes in the model."""
        return self._node_count
    
    @property
    def edge_count(self) -> int:
        """Get the number of edges in the model."""
        return self._edge_count
    
    def validate(self) -> bool:
        """Validate the model structure.
        
        Returns:
            True if the model is valid, False otherwise
        """
        return True
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert the model to a dictionary representation.
        
        Returns:
            Dictionary representation of the model
        """
        return {
            "name": self._name,
            "metadata": self._metadata,
            "node_count": self._node_count,
            "edge_count": self._edge_count,
        }


class BayesianNetwork(NetworkModel):
    """Bayesian Network model.
    
    A Bayesian network is a probabilistic graphical model that represents
    a set of random variables and their conditional dependencies via a
    directed acyclic graph.
    
    Attributes:
        nodes: List of node names
        edges: List of (parent, child) tuples
        cpds: Conditional probability distributions
    """
    
    def __init__(self, name: str = "bayesian_network"):
        """Initialize a Bayesian network.
        
        Args:
            name: Network name
        """
        super().__init__(name)
        self._nodes: List[str] = []
        self._edges: List[tuple] = []
        self._cpds: Dict[str, np.ndarray] = {}
    
    def add_node(self, name: str) -> None:
        """Add a node to the network.
        
        Args:
            name: Node name
        """
        if name not in self._nodes:
            self._nodes.append(name)
            self._node_count = len(self._nodes)
    
    def add_edge(self, parent: str, child: str) -> None:
        """Add an edge to the network.
        
        Args:
            parent: Parent node name
            child: Child node name
        """
        self._edges.append((parent, child))
        self._edge_count = len(self._edges)


class MarkovRandomField(NetworkModel):
    """Markov Random Field model.
    
    A Markov random field is an undirected graphical model that represents
    dependencies between random variables using an undirected graph.
    """
    
    def __init__(self, name: str = "markov_random_field"):
        """Initialize a Markov random field.
        
        Args:
            name: Field name
        """
        super().__init__(name)
        self._nodes: List[str] = []
        self._edges: List[tuple] = []
        self._factors: Dict[tuple, np.ndarray] = {}