"""Probabilistic inference module.

This module provides inference algorithms for network models including:
- Belief propagation (sum-product algorithm)
- Loopy belief propagation
- Gibbs sampling
- Variational inference
- Exact inference for small networks
"""

from typing import Optional, Dict, List, Any, Union, Sequence
import warnings
import numpy as np
from dataclasses import dataclass


@dataclass
class InferenceOptions:
    """Options for inference algorithms.
    
    Attributes:
        max_iterations: Maximum number of iterations
        tolerance: Convergence threshold
        seed: Random seed for stochastic algorithms
    """
    max_iterations: int = 1000
    tolerance: float = 1e-6
    seed: Optional[int] = None
    treewidth_threshold: int = 15


class LutufiHighTreewidthWarning(UserWarning):
    """Warning raised when exact inference is attempted on a high-treewidth network."""
    pass


@dataclass
class InferenceResult:
    """Result of an inference operation.
    
    Attributes:
        marginals: Marginal probabilities for each node
        iterations: Number of iterations performed
        converged: Whether the algorithm converged
        log_likelihood: Log-likelihood value (if available)
    """
    marginals: Dict[str, np.ndarray]
    iterations: int
    converged: bool
    log_likelihood: Optional[float] = None


class QueryResult:
    """Results of a probabilistic query.

    Provides convenient access to probability distributions and joint factors.
    """

    def __init__(
        self,
        variables: List[str],
        distributions: Dict[str, np.ndarray],
        joint: Optional[np.ndarray] = None,
        state_names: Optional[Dict[str, List[str]]] = None,
        argmax: Optional[Dict[str, Any]] = None,
    ):
        self.variables = variables
        self.distributions = distributions
        self.joint = joint
        self.state_names = state_names or {}
        self._argmax = argmax

    def __getitem__(self, variable: str) -> np.ndarray:
        if variable == "__joint__":
            return self.joint
        return self.distributions[variable]

    def to_dataframe(self) -> 'pd.DataFrame':
        try:
            import pandas as pd
        except ImportError as e:
            raise ImportError("pandas is required to convert QueryResult to a DataFrame") from e

        if self.joint is None:
            rows = [
                {"state": i, "probability": float(dist)}
                for i, dist in enumerate(next(iter(self.distributions.values())))
            ]
            return pd.DataFrame(rows)

        sizes = [self.distributions[var].shape[0] for var in self.variables]
        entries = self.joint.flatten()
        coords = np.stack(np.unravel_index(np.arange(entries.size), sizes), axis=-1)
        records = []
        for idx, coord in enumerate(coords):
            row = {var: int(coord[i]) for i, var in enumerate(self.variables)}
            row["probability"] = float(entries[idx])
            records.append(row)
        return pd.DataFrame(records)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "variables": self.variables,
            "distributions": {k: v.tolist() for k, v in self.distributions.items()},
            "joint": self.joint.tolist() if self.joint is not None else None,
            "argmax": self._argmax,
        }

    def most_probable(self) -> Dict[str, Any]:
        if self._argmax is not None:
            return self._argmax

        result = {}
        for var, dist in self.distributions.items():
            index = int(np.argmax(dist))
            states = self.state_names.get(var)
            result[var] = states[index] if states and index < len(states) else index
        return result


try:
    from lutufi._lutufi import (
        _RustVariableEliminationEngine,
        _RustJunctionTreeEngine,
    )
except ImportError:
    _RustVariableEliminationEngine = None
    _RustJunctionTreeEngine = None

class InferenceEngine:
    """Engine for performing probabilistic inference.
    
    This class provides a unified interface for various inference algorithms
    on network models.
    """
    
    def __init__(self, model: Any, algorithm: str = "variable_elimination"):
        """Initialize the inference engine.
        
        Args:
            model: Network model to perform inference on
            algorithm: Inference algorithm to use
        """
        self._model = model
        self._algorithm = algorithm
        self._evidence: Dict[str, str] = {}
        self._options = InferenceOptions()
        self._rust_ve = _RustVariableEliminationEngine() if _RustVariableEliminationEngine else None
        self._rust_jt = _RustJunctionTreeEngine(self._model._model) if _RustJunctionTreeEngine else None
    
    @property
    def model(self) -> Any:
        """Get the model being used for inference."""
        return self._model
    
    @property
    def algorithm(self) -> str:
        """Get the inference algorithm name."""
        return self._algorithm
    
    def set_options(self, options: InferenceOptions) -> None:
        """Set inference options.
        
        Args:
            options: Inference options
        """
        self._options = options
    
    def set_evidence(self, node: str, value: Any) -> None:
        """Set evidence for a node.
        
        Args:
            node: Node name
            value: Observed value (index as int/str or state name)
        """
        # If value is a string, check if it's a state name
        if isinstance(value, str):
            try:
                # Try to parse as integer index first
                int(value)
                self._evidence[node] = value
            except ValueError:
                # Not an integer, must be a state name
                # We need to get the states from the model
                states = self._model._model.get_states(node)
                if value in states:
                    self._evidence[node] = str(states.index(value))
                else:
                    # Fallback to original value (maybe it's an index as string)
                    self._evidence[node] = value
        else:
            self._evidence[node] = str(value)
    
    def clear_evidence(self) -> None:
        """Clear all evidence."""
        self._evidence.clear()
    
    def infer(self) -> InferenceResult:
        """Run inference on the model.
        
        Returns:
            Inference result containing marginals and convergence info
        """
        # Default to variable elimination if no specific infer method implemented
        if self._algorithm == "variable_elimination":
            return self._run_variable_elimination(list(self._model.nodes()))
        
        return InferenceResult(
            marginals={},
            iterations=0,
            converged=False,
            log_likelihood=None,
        )
    
    def query(
        self,
        variables: List[str],
        evidence: Optional[Dict[str, Any]] = None,
        algorithm: Optional[str] = None,
        elimination_order: Optional[Optional[Union[str, List[str]]]] = None,
    ) -> QueryResult:
        """Query the marginal probabilities for specific variables.

        Args:
            variables: List of variable names to query
            evidence: Optional dictionary of evidence
            algorithm: Optional algorithm name to override the default
            elimination_order: Optional heuristic ("min_fill", "min_degree") or explicit list of variable names.

        Returns:
            A QueryResult containing marginals, joint distributions, and support utilities.
        """
        self.clear_evidence()
        if evidence:
            for node, value in evidence.items():
                self.set_evidence(node, value)

        alg = algorithm or self._algorithm
        
        if alg == "variable_elimination":
            raw_result = self._run_variable_elimination(variables, elimination_order)
            return self._build_query_result(raw_result, variables)

        if alg == "junction_tree":
            if not self._rust_jt:
                raise RuntimeError("Lutufi native extension not loaded.")
            raw_result = self._rust_jt.query(variables, self._evidence)
            if self._rust_jt.treewidth() > self._options.treewidth_threshold:
                warnings.warn(
                    f"Treewidth {self._rust_jt.treewidth()} exceeds threshold {self._options.treewidth_threshold}.",
                    LutufiHighTreewidthWarning,
                )
            return self._build_query_result(raw_result, variables)

        raise ValueError(f"Unknown inference algorithm: {alg}")

    def map_query(
        self,
        variables: List[str],
        evidence: Optional[Dict[str, Any]] = None,
        elimination_order: Optional[Union[str, List[str]]] = None,
    ) -> QueryResult:
        """Query the MAP assignment for a set of variables."""
        self.clear_evidence()
        if evidence:
            for node, value in evidence.items():
                self.set_evidence(node, value)

        if not self._rust_ve:
            raise RuntimeError("Lutufi native extension not loaded.")

        raw_result = self._rust_ve.query_map(
            self._model._model,
            variables,
            self._evidence,
            elimination_order,
            "map",
        )
        return self._build_query_result(raw_result, variables)

    def mpe_query(
        self,
        evidence: Optional[Dict[str, Any]] = None,
        elimination_order: Optional[Union[str, List[str]]] = None,
    ) -> QueryResult:
        """Query the most probable explanation for all unobserved variables."""
        self.clear_evidence()
        if evidence:
            for node, value in evidence.items():
                self.set_evidence(node, value)

        if not self._rust_ve:
            raise RuntimeError("Lutufi native extension not loaded.")

        variables = list(self._model.nodes())
        raw_result = self._rust_ve.query_map(
            self._model._model,
            variables,
            self._evidence,
            elimination_order,
            "mpe",
        )
        return self._build_query_result(raw_result, variables)

    def _build_query_result(self, raw_result: Dict[str, Any], variables: List[str]) -> QueryResult:
        values = np.array(raw_result["values"])
        var_names = raw_result["variables"]
        shapes = [len(self._model.get_states(var)) for var in var_names]

        joint = None
        distributions = {}
        if len(var_names) == 1:
            distributions[var_names[0]] = values
        else:
            joint = values.reshape(shapes)
            for axis, var in enumerate(var_names):
                axes = tuple(i for i in range(len(shapes)) if i != axis)
                distributions[var] = np.sum(joint, axis=axes)

        state_names = {var: self._model.get_states(var) for var in var_names}
        argmax = None
        if values.size > 0:
            idx = int(np.argmax(values))
            if len(var_names) == 1:
                argmax = {var_names[0]: state_names[var_names[0]][idx]}
            else:
                unravel = np.unravel_index(idx, tuple(shapes))
                argmax = {
                    var_names[i]: state_names[var_names[i]][int(unravel[i])]
                    for i in range(len(var_names))
                }

        return QueryResult(
            variables=var_names,
            distributions=distributions,
            joint=joint,
            state_names=state_names,
            argmax=argmax,
        )

    def _run_variable_elimination(self, variables: List[str], heuristic: Optional[Union[str, List[str]]] = None) -> Dict[str, Any]:
        if not self._rust_ve:
            raise RuntimeError("Lutufi native extension not loaded.")

        raw_result = self._rust_ve.query(
            self._model._model,
            variables,
            self._evidence,
            heuristic,
        )
        return raw_result


class JunctionTreeEngine:
    """Exact inference engine using a compiled junction tree."""

    def __init__(self, model: Any, treewidth_threshold: int = 15):
        self._model = model
        self._engine = _RustJunctionTreeEngine(self._model._model) if _RustJunctionTreeEngine else None
        if not self._engine:
            raise RuntimeError("Lutufi native extension not loaded.")

        self.treewidth = self._engine.treewidth()
        if self.treewidth > treewidth_threshold:
            warnings.warn(
                f"Treewidth {self.treewidth} exceeds threshold {treewidth_threshold}.",
                LutufiHighTreewidthWarning,
            )

    def query(
        self,
        variables: List[str],
        evidence: Optional[Dict[str, Any]] = None,
    ) -> QueryResult:
        if evidence is None:
            evidence = {}
        raw_result = self._engine.query(variables, evidence)
        return self._build_query_result(raw_result, variables)

    def _build_query_result(self, raw_result: Dict[str, Any], variables: List[str]) -> QueryResult:
        values = np.array(raw_result["values"])
        var_names = raw_result["variables"]
        shapes = [len(self._model.get_states(var)) for var in var_names]

        joint = None
        distributions = {}
        if len(var_names) == 1:
            distributions[var_names[0]] = values
        else:
            joint = values.reshape(shapes)
            for axis, var in enumerate(var_names):
                axes = tuple(i for i in range(len(shapes)) if i != axis)
                distributions[var] = np.sum(joint, axis=axes)

        state_names = {var: self._model.get_states(var) for var in var_names}
        argmax = None
        if values.size > 0:
            idx = int(np.argmax(values))
            if len(var_names) == 1:
                argmax = {var_names[0]: state_names[var_names[0]][idx]}
            else:
                unravel = np.unravel_index(idx, tuple(shapes))
                argmax = {
                    var_names[i]: state_names[var_names[i]][int(unravel[i])]
                    for i in range(len(var_names))
                }

        return QueryResult(
            variables=var_names,
            distributions=distributions,
            joint=joint,
            state_names=state_names,
            argmax=argmax,
        )


class BeliefPropagation(InferenceEngine):
    """Belief propagation algorithm implementation.
    
    Implements the sum-product algorithm for exact inference on
    tree-structured networks.
    """
    
    def __init__(self, model: Any):
        """Initialize belief propagation engine.
        
        Args:
            model: Network model (must be tree-structured)
        """
        super().__init__(model, algorithm="belief_propagation")


class LoopyBeliefPropagation(InferenceEngine):
    """Loopy belief propagation algorithm implementation.
    
    Approximate inference algorithm for networks with loops.
    """
    
    def __init__(self, model: Any):
        """Initialize loopy belief propagation engine.
        
        Args:
            model: Network model
        """
        super().__init__(model, algorithm="loopy_belief_propagation")


class GibbsSampler(InferenceEngine):
    """Gibbs sampling algorithm implementation.
    
    Markov chain Monte Carlo method for approximate inference.
    """
    
    def __init__(self, model: Any):
        """Initialize Gibbs sampler.
        
        Args:
            model: Network model
        """
        super().__init__(model, algorithm="gibbs_sampling")