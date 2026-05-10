"""Probabilistic inference module.

This module provides inference algorithms for network models including:
- Variable elimination (exact)
- Junction tree (exact, compiled)
- Loopy belief propagation (approximate)
- Gibbs sampling / MCMC (approximate)
- Variational inference (approximate)
- Unified InferenceEngine with automatic algorithm selection
- Lazy and async query support
"""

from typing import Optional, Dict, List, Any, Union, Callable
import warnings
import numpy as np
from dataclasses import dataclass, field


# ─── Inference Options ────────────────────────────────────────────────────────

@dataclass
class InferenceOptions:
    """Options for inference algorithms.

    Attributes:
        max_iterations: Maximum number of iterations for iterative algorithms.
        tolerance: Convergence threshold.
        seed: Random seed for stochastic algorithms.
        treewidth_threshold: Threshold for raising a high-treewidth warning.
        damping: Damping factor for LBP (0 = no damping, 1 = full damping).
        n_samples: Number of samples for MCMC.
        burn_in: Number of initial samples to discard for MCMC.
        n_chains: Number of parallel MCMC chains (for Gelman-Rubin diagnostic).
        n_restarts: Number of random restarts for variational inference.
    """
    max_iterations: int = 1000
    tolerance: float = 1e-6
    seed: Optional[int] = None
    treewidth_threshold: int = 15
    damping: float = 0.5
    n_samples: int = 1000
    burn_in: int = 100
    n_chains: int = 4
    n_restarts: int = 5


class LutufiHighTreewidthWarning(UserWarning):
    """Warning raised when exact inference is attempted on a high-treewidth network."""
    pass


# ─── Inference Result Classes ─────────────────────────────────────────────────

@dataclass
class InferenceMetadata:
    """Metadata about an inference operation.

    Attributes:
        algorithm: Name of the algorithm used.
        converged: Whether the algorithm converged.
        iterations: Number of iterations performed.
        residual: Final convergence residual (if applicable).
        elbo: Final ELBO value (variational inference).
        n_samples: Number of samples (MCMC).
        treewidth: Estimated treewidth (junction tree).
        warnings: List of warnings generated during inference.
    """
    algorithm: str = ""
    converged: bool = True
    iterations: int = 0
    residual: Optional[float] = None
    elbo: Optional[float] = None
    n_samples: Optional[int] = None
    treewidth: Optional[int] = None
    warnings: List[str] = field(default_factory=list)


class InferenceResult:
    """Result of an inference operation.

    Provides marginal distributions, algorithm metadata,
    and automatic algorithm selection transparency.

    Attributes:
        marginals: Marginal probabilities for each queried variable.
        metadata: Information about the inference algorithm and its quality.
        variables: List of queried variable names.
    """

    def __init__(
        self,
        variables: List[str],
        marginals: Dict[str, np.ndarray],
        metadata: Optional[InferenceMetadata] = None,
    ):
        self.variables = variables
        self.marginals = marginals
        self.metadata = metadata or InferenceMetadata()

    def __getitem__(self, variable: str) -> np.ndarray:
        return self.marginals[variable]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "variables": self.variables,
            "marginals": {k: v.tolist() for k, v in self.marginals.items()},
            "metadata": {
                "algorithm": self.metadata.algorithm,
                "converged": self.metadata.converged,
                "iterations": self.metadata.iterations,
            },
        }


class QueryResult:
    """Results of a probabilistic query.

    Provides convenient access to probability distributions and joint factors.

    Attributes:
        variables: Names of the queried variables.
        distributions: Marginal distributions for each variable.
        joint: The joint distribution if multiple variables were queried.
        algorithm: The name of the algorithm used for the query.
        diagnostics: Algorithm-specific diagnostic information.
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

    def to_dataframe(self) -> "pd.DataFrame":
        """Convert the result to a tidy pandas DataFrame.

        Returns:
            A DataFrame with columns for each variable and a 'probability' column.

        Example:
            >>> result.to_dataframe()
               state  probability
            0   low     0.3
            1  high     0.7
        """
        try:
            import pandas as pd
        except ImportError as e:
            raise ImportError(
                "pandas is required to convert QueryResult to a DataFrame"
            ) from e

        if self.joint is None:
            var_name = self.variables[0]
            dist = self.distributions[var_name]
            states = self.state_names.get(
                var_name, [str(i) for i in range(len(dist))]
            )
            rows = [
                {"state": states[i], "probability": float(dist[i])}
                for i in range(len(dist))
            ]
            return pd.DataFrame(rows)

        sizes = [self.distributions[var].shape[0] for var in self.variables]
        entries = self.joint.flatten()
        coords = np.stack(
            np.unravel_index(np.arange(entries.size), sizes), axis=-1
        )
        records = []
        for idx, coord in enumerate(coords):
            row = {
                var: self.state_names.get(var, [str(i) for i in range(sizes[i])])[
                    int(coord[i])
                ]
                for i, var in enumerate(self.variables)
            }
            row["probability"] = float(entries[idx])
            records.append(row)
        return pd.DataFrame(records)

    def to_dict(self) -> Dict[str, Any]:
        """Convert the result to a plain Python dict.

        Returns:
            A dict with variables, distributions, most probable state, and metadata.
        """
        return {
            "variables": self.variables,
            "distributions": {k: v.tolist() for k, v in self.distributions.items()},
            "joint": self.joint.tolist() if self.joint is not None else None,
            "argmax": self.most_probable(),
            "algorithm": self.algorithm,
            "diagnostics": self.diagnostics,
        }

    def most_probable(self) -> Dict[str, Any]:
        """Get the most probable state for each queried variable.

        Returns:
            A dict mapping variable names to their most probable states.
        """
        if self._argmax is not None:
            return self._argmax

        result = {}
        for var, dist in self.distributions.items():
            index = int(np.argmax(dist))
            states = self.state_names.get(var)
            result[var] = (
                states[index] if states and index < len(states) else index
            )
        return result

    def plot(self, **kwargs):
        """Visualize the query result as a bar chart.

        Args:
            **kwargs: Additional keyword arguments for matplotlib.pyplot.

        Returns:
            A matplotlib Figure and Axes (fig, ax).
        """
        import matplotlib.pyplot as plt

        n_vars = len(self.variables)
        fig, axes = plt.subplots(
            1, n_vars, figsize=kwargs.pop("figsize", (5 * n_vars, 4)),
            squeeze=False
        )

        for i, var in enumerate(self.variables):
            ax = axes[0, i]
            dist = self.distributions[var]
            states = self.state_names.get(
                var, [str(i) for i in range(len(dist))]
            )
            colors = kwargs.pop("colors", ["steelblue"] * len(states))

            ax.bar(states, dist, color=colors, edgecolor="black", linewidth=1.2)
            ax.set_xlabel(kwargs.pop("xlabel", "State"), fontsize=11)
            ax.set_ylabel(kwargs.pop("ylabel", "Probability"), fontsize=11)
            ax.set_title(f"P({var})", fontsize=13, fontweight="bold")
            ax.set_ylim(0, 1.05)
            ax.grid(axis="y", alpha=0.3)

            for j, p in enumerate(dist):
                ax.text(j, p + 0.02, f"{p:.3f}", ha="center", fontsize=9)

        plt.tight_layout()
        return fig, axes


class LazyQueryResult:
    """A lazily evaluated query result.

    Computation is deferred until the result is actually accessed
    (via .distributions, .to_dataframe(), etc.).
    This enables query optimization when multiple queries are submitted together.

    Example:
        >>> result = engine.query(["A"], {"B": "1"})  # returns LazyQueryResult
        >>> # No computation yet
        >>> df = result.to_dataframe()  # Computation triggered here
    """

    def __init__(self, compute_fn: Callable[[], QueryResult]):
        self._compute_fn = compute_fn
        self._result: Optional[QueryResult] = None

    def _compute(self) -> QueryResult:
        if self._result is None:
            self._result = self._compute_fn()
        return self._result

    @property
    def distributions(self) -> Dict[str, np.ndarray]:
        return self._compute().distributions

    @property
    def joint(self) -> Optional[np.ndarray]:
        return self._compute().joint

    @property
    def variables(self) -> List[str]:
        return self._compute().variables

    @property
    def algorithm(self) -> Optional[str]:
        return self._compute().algorithm

    @property
    def diagnostics(self) -> Dict[str, Any]:
        return self._compute().diagnostics

    def __getitem__(self, variable: str) -> np.ndarray:
        return self._compute()[variable]

    def to_dataframe(self) -> "pd.DataFrame":
        return self._compute().to_dataframe()

    def to_dict(self) -> Dict[str, Any]:
        return self._compute().to_dict()

    def most_probable(self) -> Dict[str, Any]:
        return self._compute().most_probable()

    def plot(self, **kwargs):
        return self._compute().plot(**kwargs)


# ─── Rust Engine Imports ──────────────────────────────────────────────────────

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


# ─── Unified Inference Engine ─────────────────────────────────────────────────

class InferenceEngine:
    """Unified inference engine with automatic algorithm selection.

    Selects the best inference algorithm automatically based on network
    characteristics. The user may override with an explicit algorithm choice.

    Parameters
    ----------
    model : BayesianNetwork
        The model to perform inference on.
    algorithm : str, optional
        Inference algorithm to use. One of:
        - "auto" (default): automatically selects based on network profile
        - "exact" or "variable_elimination": variable elimination
        - "junction_tree": junction tree (compiled for repeated queries)
        - "lbp": loopy belief propagation
        - "mcmc": Gibbs sampling / MCMC
        - "variational": mean field variational inference

    Example:
        >>> engine = InferenceEngine(model, algorithm="auto")
        >>> result = engine.query(["Disease"], evidence={"Fever": "high"})
        >>> print(result.distributions["Disease"])
    """

    def __init__(self, model: Any, algorithm: str = "auto"):
        """Initialize the inference engine.

        Args:
            model: Network model to perform inference on.
            algorithm: Inference algorithm to use.
        """
        self._model = model
        self._algorithm = algorithm
        self._evidence: Dict[str, str] = {}
        self._options = InferenceOptions()

        self._rust_ve = (
            _RustVariableEliminationEngine() if _RustVariableEliminationEngine else None
        )
        self._rust_jt = None
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

    def _get_jt_engine(self):
        """Lazily initialize and return the Junction Tree engine."""
        if self._rust_jt is None and _RustJunctionTreeEngine is not None:
            self._rust_jt = _RustJunctionTreeEngine(self._model._model)
        return self._rust_jt

    def set_options(self, options: InferenceOptions) -> None:
        """Set inference options.

        Args:
            options: Inference options.
        """
        self._options = options

    def set_evidence(self, node: str, value: Any) -> None:
        """Set evidence (observed value) for a node.

        Args:
            node: Node name.
            value: Observed value (state name or index).
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
        lazy: bool = False,
        **kwargs,
    ) -> QueryResult:
        """Query the marginal probabilities for specific variables.

        Args:
            variables: List of variable names to query.
            evidence: Optional dict mapping variable names to observed values.
            algorithm: Optional algorithm name to override the default.
            lazy: If True, return a LazyQueryResult (computation deferred).
            **kwargs: Algorithm-specific parameters (e.g., damping for LBP,
                     n_samples for MCMC).

        Returns:
            A QueryResult containing marginals and diagnostics.
            If lazy=True, returns a LazyQueryResult.
        """
        self.clear_evidence()
        if evidence:
            for node, value in evidence.items():
                self.set_evidence(node, value)

        alg = algorithm or self._algorithm

        if lazy:
            return LazyQueryResult(lambda: self._query_impl(variables, alg, **kwargs))
        return self._query_impl(variables, alg, **kwargs)

    def _query_impl(
        self, variables: List[str], alg: str, **kwargs
    ) -> QueryResult:
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
        """Automatically select the best inference algorithm."""
        num_nodes = len(self._model.nodes())
        if num_nodes <= 20:
            return "exact"

        jt = self._get_jt_engine()
        if jt:
            tw = jt.treewidth()
            if tw <= self._options.treewidth_threshold:
                return "junction_tree"

        return "lbp"

    def _build_evidence_map(self) -> Dict[str, str]:
        return dict(self._evidence)

    def _query_ve(
        self, variables: List[str], heuristic: Any
    ) -> QueryResult:
        if not self._rust_ve:
            raise RuntimeError("Native extension not loaded")
        raw = self._rust_ve.query(
            self._model._model, variables, self._build_evidence_map(), heuristic
        )
        return self._build_query_result(raw, variables, "variable_elimination")

    def _query_jt(self, variables: List[str]) -> QueryResult:
        jt = self._get_jt_engine()
        if not jt:
            raise RuntimeError("Native extension not loaded")
        raw = jt.query(variables, self._build_evidence_map())
        if jt.treewidth() > self._options.treewidth_threshold:
            warnings.warn(
                f"High treewidth: {jt.treewidth()}",
                LutufiHighTreewidthWarning,
            )
        return self._build_query_result(raw, variables, "junction_tree")

    def _query_lbp(self, variables: List[str], **kwargs) -> QueryResult:
        if not self._rust_lbp:
            raise RuntimeError("Native extension not loaded")
        res = self._rust_lbp.query(
            self._model._model,
            variables,
            self._build_evidence_map(),
            max_iterations=kwargs.get("max_iterations", self._options.max_iterations),
            tolerance=kwargs.get("tolerance", self._options.tolerance),
            damping=kwargs.get("damping", self._options.damping),
        )
        if not res["converged"]:
            warnings.warn(
                f"LBP did not converge (residual: {res['residual']})",
                UserWarning,
            )

        return QueryResult(
            variables=variables,
            distributions={
                v: np.array(d) for v, d in res["marginals"].items()
            },
            state_names={v: self._model.get_states(v) for v in variables},
            algorithm="lbp",
            diagnostics={
                "converged": res["converged"],
                "iterations": res["iterations"],
                "residual": res["residual"],
            },
        )

    def _query_mcmc(self, variables: List[str], **kwargs) -> QueryResult:
        if not self._rust_mcmc:
            raise RuntimeError("Native extension not loaded")
        res = self._rust_mcmc.query(
            self._model._model,
            variables,
            self._build_evidence_map(),
            n_samples=kwargs.get("n_samples", self._options.n_samples),
            burn_in=kwargs.get("burn_in", self._options.burn_in),
        )
        return QueryResult(
            variables=variables,
            distributions={
                v: np.array(d) for v, d in res["marginals"].items()
            },
            state_names={v: self._model.get_states(v) for v in variables},
            algorithm="mcmc",
            diagnostics={"n_samples": res["n_samples"]},
        )

    def _query_vi(self, variables: List[str], **kwargs) -> QueryResult:
        if not self._rust_vi:
            raise RuntimeError("Native extension not loaded")
        res = self._rust_vi.query(
            self._model._model,
            variables,
            self._build_evidence_map(),
            max_iterations=kwargs.get("max_iterations", self._options.max_iterations),
            tolerance=kwargs.get("tolerance", self._options.tolerance),
        )
        return QueryResult(
            variables=variables,
            distributions={
                v: np.array(d) for v, d in res["marginals"].items()
            },
            state_names={v: self._model.get_states(v) for v in variables},
            algorithm="variational",
            diagnostics={
                "converged": res["converged"],
                "elbo": res["elbo"],
            },
        )

    def _build_query_result(
        self,
        raw_result: Dict[str, Any],
        variables: List[str],
        alg: str,
    ) -> QueryResult:
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
            algorithm=alg,
        )

    def map_query(
        self,
        variables: List[str],
        evidence: Optional[Dict[str, Any]] = None,
    ) -> QueryResult:
        """Compute the MAP (Maximum A Posteriori) assignment.

        Finds the most probable joint assignment to the query variables
        given the evidence.

        Args:
            variables: List of variable names to query.
            evidence: Optional evidence dict.

        Returns:
            A QueryResult with the MAP assignment.
        """
        return self.query(
            variables, evidence, algorithm="variable_elimination", mode="map"
        )

    def mpe_query(
        self,
        evidence: Optional[Dict[str, Any]] = None,
    ) -> QueryResult:
        """Compute the MPE (Most Probable Explanation).

        Finds the most probable assignment to ALL unobserved variables
        given the evidence.

        Args:
            evidence: Optional evidence dict.

        Returns:
            A QueryResult with the MPE assignment.
        """
        return self.query(
            list(self._model.nodes()),
            evidence,
            algorithm="variable_elimination",
            mode="mpe",
        )

    async def query_async(
        self,
        variables: List[str],
        evidence: Optional[Dict[str, Any]] = None,
        algorithm: Optional[str] = None,
        **kwargs,
    ) -> QueryResult:
        """Async version of query().

        Runs inference in a thread pool to avoid blocking the event loop.

        Args:
            variables: List of variable names to query.
            evidence: Optional evidence dict.
            algorithm: Optional algorithm name.
            **kwargs: Algorithm-specific parameters.

        Returns:
            A QueryResult.
        """
        import asyncio
        loop = asyncio.get_event_loop()
        return await loop.run_in_executor(
            None, self.query, variables, evidence, algorithm
        )


# ─── Specialized Engine Classes ───────────────────────────────────────────────

class JunctionTreeEngine:
    """Exact inference engine using a compiled junction tree.

    Compiles the model into a junction tree once, then answers arbitrary
    marginal queries in O(clique_size) time. Compilation is expensive but
    subsequent queries are fast.

    Parameters
    ----------
    model : BayesianNetwork
        The model to compile.
    treewidth_threshold : int, optional
        Threshold for high-treewidth warning (default 15).
    """

    def __init__(self, model: Any, treewidth_threshold: int = 15):
        self._model = model
        self._engine = (
            _RustJunctionTreeEngine(self._model._model)
            if _RustJunctionTreeEngine
            else None
        )
        if not self._engine:
            raise RuntimeError("Lutufi native extension not loaded.")

        self.treewidth = self._engine.treewidth()
        if self.treewidth > treewidth_threshold:
            warnings.warn(
                f"Treewidth {self.treewidth} exceeds threshold {treewidth_threshold}. "
                f"Consider using an approximate inference algorithm for this network.",
                LutufiHighTreewidthWarning,
            )

    def query(
        self,
        variables: List[str],
        evidence: Optional[Dict[str, Any]] = None,
    ) -> QueryResult:
        """Run a marginal query on the compiled junction tree.

        Args:
            variables: Variables to query.
            evidence: Observed values.

        Returns:
            A QueryResult with marginals.
        """
        if evidence is None:
            evidence = {}

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

    def _build_query_result(
        self, raw_result: Dict[str, Any], variables: List[str]
    ) -> QueryResult:
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
            algorithm="junction_tree",
            diagnostics={"treewidth": self.treewidth},
        )


class BeliefPropagation(InferenceEngine):
    """Exact inference via junction tree (convenience class).

    Alias for InferenceEngine with algorithm="junction_tree".
    """
    def __init__(self, model: Any):
        super().__init__(model, algorithm="junction_tree")


class LoopyBeliefPropagation(InferenceEngine):
    """Approximate inference via loopy belief propagation.

    Alias for InferenceEngine with algorithm="lbp".
    """
    def __init__(self, model: Any):
        super().__init__(model, algorithm="lbp")


class GibbsSampler(InferenceEngine):
    """Approximate inference via Gibbs sampling (MCMC).

    Alias for InferenceEngine with algorithm="mcmc".
    """
    def __init__(self, model: Any):
        super().__init__(model, algorithm="mcmc")


class VariationalInference(InferenceEngine):
    """Approximate inference via mean field variational inference.

    Alias for InferenceEngine with algorithm="variational".
    """
    def __init__(self, model: Any):
        super().__init__(model, algorithm="variational")
