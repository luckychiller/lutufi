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
from dataclasses import dataclass, field


@dataclass
class InferenceOptions:
    """Options for inference algorithms.
    
    Attributes:
        max_iterations: Maximum number of iterations
        tolerance: Convergence threshold
        seed: Optional[int]: Random seed for stochastic algorithms
        treewidth_threshold: int: Threshold for raising a high-treewidth warning
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
    
    Attributes:
        variables (List[str]): Names of the queried variables.
        distributions (Dict[str, np.ndarray]): Marginal distributions for each variable.
        joint (Optional[np.ndarray]): The joint distribution if multiple variables were queried.
        algorithm (str): The name of the algorithm used for the query.
        diagnostics (Dict[str, Any]): Algorithm-specific diagnostic information.
    """

    def __init__(
        self,
        variables: List[str],
        distributions: Dict[str, np.ndarray],
        joint: Optional[np.ndarray] = None,
        state_names: Optional[Dict[str, List[str]]] = None,
        argmax: Optional[Dict[str, Any]] = None,
        algorithm: Optional[str] = None,
        diagnostics: Optional[Dict[str, Any]] = None,
    ):
        self.variables = variables
        self.distributions = distributions
        self.joint = joint
        self.state_names = state_names or {}
        self._argmax = argmax
        self.algorithm = algorithm
        self.diagnostics = diagnostics or {}

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
            # For single variable query
            var_name = self.variables[0]
            dist = self.distributions[var_name]
            states = self.state_names.get(var_name, [str(i) for i in range(len(dist))])
            rows = [
                {"state": states[i], "probability": float(dist[i])}
                for i in range(len(dist))
            ]
            return pd.DataFrame(rows)

        sizes = [self.distributions[var].shape[0] for var in self.variables]
        entries = self.joint.flatten()
        coords = np.stack(np.unravel_index(np.arange(entries.size), sizes), axis=-1)
        records = []
        for idx, coord in enumerate(coords):
            row = {var: self.state_names.get(var, [str(i) for i in range(sizes[i])])[int(coord[i])] 
                   for i, var in enumerate(self.variables)}
            row["probability"] = float(entries[idx])
            records.append(row)
        return pd.DataFrame(records)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "variables": self.variables,
            "distributions": {k: v.tolist() for k, v in self.distributions.items()},
            "joint": self.joint.tolist() if self.joint is not None else None,
            "argmax": self.most_probable(),
            "algorithm": self.algorithm,
            "diagnostics": self.diagnostics,
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
        _RustLBPEngine,
        _RustMCMCEngine,
        _RustVariationalEngine,
    )
except ImportError:
    _RustVariableEliminationEngine = None
    _RustJunctionTreeEngine = None
    _RustLBPEngine = None
    _RustMCMCEngine = None
    _RustVariationalEngine = None

class InferenceEngine:
    """Engine for performing probabilistic inference.
    
    This class provides a unified interface for various inference algorithms
    on network models.
    """
    
    def __init__(self, model: Any, algorithm: str = "auto"):
        """Initialize the inference engine.
        
        Args:
            model: Network model to perform inference on
            algorithm: Inference algorithm to use ('auto', 'exact', 'lbp', 'mcmc', 'variational')
        """
        self._model = model
        self._algorithm = algorithm
        self._evidence: Dict[str, str] = {}
        self._options = InferenceOptions()
        
        # Initialize Rust engines
        self._rust_ve = _RustVariableEliminationEngine() if _RustVariableEliminationEngine else None
        self._rust_jt = _RustJunctionTreeEngine(self._model._model) if _RustJunctionTreeEngine else None
        self._rust_lbp = _RustLBPEngine() if _RustLBPEngine else None
        self._rust_mcmc = _RustMCMCEngine() if _RustMCMCEngine else None
        self._rust_vi = _RustVariationalEngine() if _RustVariationalEngine else None
    
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
        if isinstance(value, str):
            try:
                int(value)
                self._evidence[node] = value
            except ValueError:
                states = self._model.get_states(node)
                if value in states:
                    self._evidence[node] = str(states.index(value))
                else:
                    self._evidence[node] = value
        else:
            self._evidence[node] = str(value)
    
    def clear_evidence(self) -> None:
        """Clear all evidence."""
        self._evidence.clear()

    def query(
        self,
        variables: List[str],
        evidence: Optional[Dict[str, Any]] = None,
        algorithm: Optional[str] = None,
        **kwargs
    ) -> QueryResult:
        """Query the marginal probabilities for specific variables.

        Args:
            variables: List of variable names to query
            evidence: Optional dictionary of evidence
            algorithm: Optional algorithm name to override the default
            **kwargs: Algorithm-specific parameters (e.g., damping for LBP, n_samples for MCMC)

        Returns:
            A QueryResult containing marginals and diagnostics.
        """
        self.clear_evidence()
        if evidence:
            for node, value in evidence.items():
                self.set_evidence(node, value)

        alg = algorithm or self._algorithm
        
        if alg == "auto":
            alg = self._select_algorithm(variables)

        if alg in ("exact", "variable_elimination"):
            return self._query_ve(variables, kwargs.get("elimination_order"))
        
        if alg == "junction_tree":
            return self._query_jt(variables)

        if alg == "lbp":
            return self._query_lbp(variables, **kwargs)

        if alg == "mcmc":
            return self._query_mcmc(variables, **kwargs)

        if alg == "variational":
            return self._query_vi(variables, **kwargs)

        raise ValueError(f"Unknown inference algorithm: {alg}")

    def _select_algorithm(self, variables: List[str]) -> str:
        num_nodes = len(self._model.nodes())
        if num_nodes <= 20:
            return "exact"
        
        if self._rust_jt:
            tw = self._rust_jt.treewidth()
            if tw <= 15:
                return "junction_tree"
        
        return "lbp"

    def _query_ve(self, variables: List[str], heuristic: Any) -> QueryResult:
        if not self._rust_ve: raise RuntimeError("Native extension not loaded")
        raw = self._rust_ve.query(self._model._model, variables, self._evidence, heuristic)
        return self._build_query_result(raw, variables, "variable_elimination")

    def _query_jt(self, variables: List[str]) -> QueryResult:
        if not self._rust_jt: raise RuntimeError("Native extension not loaded")
        raw = self._rust_jt.query(variables, self._evidence)
        if self._rust_jt.treewidth() > self._options.treewidth_threshold:
            warnings.warn(f"High treewidth: {self._rust_jt.treewidth()}", LutufiHighTreewidthWarning)
        return self._build_query_result(raw, variables, "junction_tree")

    def _query_lbp(self, variables: List[str], **kwargs) -> QueryResult:
        if not self._rust_lbp: raise RuntimeError("Native extension not loaded")
        res = self._rust_lbp.query(
            self._model._model, variables, self._evidence,
            max_iterations=kwargs.get("max_iterations", self._options.max_iterations),
            tolerance=kwargs.get("tolerance", self._options.tolerance),
            damping=kwargs.get("damping", 0.5)
        )
        if not res["converged"]:
            warnings.warn(f"LBP did not converge (residual: {res['residual']})", UserWarning)
        
        return QueryResult(
            variables=variables,
            distributions={v: np.array(d) for v, d in res["marginals"].items()},
            state_names={v: self._model.get_states(v) for v in variables},
            algorithm="lbp",
            diagnostics={"converged": res["converged"], "iterations": res["iterations"], "residual": res["residual"]}
        )

    def _query_mcmc(self, variables: List[str], **kwargs) -> QueryResult:
        if not self._rust_mcmc: raise RuntimeError("Native extension not loaded")
        res = self._rust_mcmc.query(
            self._model._model, variables, self._evidence,
            n_samples=kwargs.get("n_samples", 1000),
            burn_in=kwargs.get("burn_in", 100)
        )
        return QueryResult(
            variables=variables,
            distributions={v: np.array(d) for v, d in res["marginals"].items()},
            state_names={v: self._model.get_states(v) for v in variables},
            algorithm="mcmc",
            diagnostics={"n_samples": res["n_samples"]}
        )

    def _query_vi(self, variables: List[str], **kwargs) -> QueryResult:
        if not self._rust_vi: raise RuntimeError("Native extension not loaded")
        res = self._rust_vi.query(
            self._model._model, variables, self._evidence,
            max_iterations=kwargs.get("max_iterations", 100),
            tolerance=kwargs.get("tolerance", 1e-4)
        )
        return QueryResult(
            variables=variables,
            distributions={v: np.array(d) for v, d in res["marginals"].items()},
            state_names={v: self._model.get_states(v) for v in variables},
            algorithm="variational",
            diagnostics={"converged": res["converged"], "elbo": res["elbo"]}
        )

    def _build_query_result(self, raw_result: Dict[str, Any], variables: List[str], alg: str) -> QueryResult:
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
        
        # Calculate argmax for convenient access
        idx = int(np.argmax(values))
        if len(var_names) == 1:
            argmax = {var_names[0]: state_names[var_names[0]][idx]}
        else:
            unravel = np.unravel_index(idx, tuple(shapes))
            argmax = {var_names[i]: state_names[var_names[i]][int(unravel[i])] for i in range(len(var_names))}

        return QueryResult(
            variables=var_names,
            distributions=distributions,
            joint=joint,
            state_names=state_names,
            argmax=argmax,
            algorithm=alg
        )

    def map_query(self, variables: List[str], evidence: Optional[Dict[str, Any]] = None) -> QueryResult:
        """Compute the MAP assignment."""
        return self.query(variables, evidence, algorithm="variable_elimination", mode="map")

    def mpe_query(self, evidence: Optional[Dict[str, Any]] = None) -> QueryResult:
        """Compute the MPE assignment."""
        return self.query(list(self._model.nodes()), evidence, algorithm="variable_elimination", mode="mpe")


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
        
        # Convert evidence to string map for Rust
        rust_evidence = {}
        for node, value in evidence.items():
            if isinstance(value, str):
                states = self._model.get_states(node)
                if value in states:
                    rust_evidence[node] = str(states.index(value))
                else:
                    rust_evidence[node] = value
            else:
                rust_evidence[node] = str(value)

        raw_result = self._engine.query(variables, rust_evidence)
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
        
        idx = int(np.argmax(values))
        if len(var_names) == 1:
            argmax = {var_names[0]: state_names[var_names[0]][idx]}
        else:
            unravel = np.unravel_index(idx, tuple(shapes))
            argmax = {var_names[i]: state_names[var_names[i]][int(unravel[i])] for i in range(len(var_names))}

        return QueryResult(
            variables=var_names,
            distributions=distributions,
            joint=joint,
            state_names=state_names,
            argmax=argmax,
            algorithm="junction_tree",
            diagnostics={"treewidth": self.treewidth}
        )


class BeliefPropagation(InferenceEngine):
    def __init__(self, model: Any):
        super().__init__(model, algorithm="junction_tree")


class LoopyBeliefPropagation(InferenceEngine):
    def __init__(self, model: Any):
        super().__init__(model, algorithm="lbp")


class GibbsSampler(InferenceEngine):
    def __init__(self, model: Any):
        super().__init__(model, algorithm="mcmc")
