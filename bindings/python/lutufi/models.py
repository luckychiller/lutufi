"""
High-level Python API for Lutufi probabilistic models.

Provides a fluent builder pattern over the Rust core, adding Python-idiomatic
conveniences:
  - Method chaining (builder pattern)
  - Pandas DataFrame import/export
  - NetworkX round-trip conversion
  - Matplotlib visualization
  - Atomic edit context manager
  - Informative __repr__ and __str__

Nothing in this file implements probabilistic logic. All mathematics
lives in the Rust core. This file exists purely for ergonomics.
"""

from __future__ import annotations

from typing import Dict, List, Optional, Tuple, Union, Any, Iterator
from contextlib import contextmanager
from pathlib import Path
import warnings

import numpy as np

try:
    from lutufi._lutufi import (
        _RustBayesianNetwork,
        _RustMarkovRandomField,
        _RustDynamicBayesianNetwork,
        ValidationResult,
    )
except ImportError:
    raise ImportError(
        "Lutufi native extension not found.\n"
        "Run 'maturin develop' from the repository root to build it.\n"
        "See INSTALL.md for platform-specific instructions."
    )


# ─── Error Classes ────────────────────────────────────────────────────────────

class LutufiError(ValueError):
    """Base class for all Lutufi errors raised from the Python layer.

    Subclasses ``ValueError`` so that existing ``except ValueError`` call
    sites continue to work even as specific operations start raising the
    more informative subclasses below.
    """
    pass


class LutufiValidationError(LutufiError):
    """
    Raised when a model fails validation before inference.

    Attributes
    ----------
    errors : list of str
        The specific validation failures.
    """
    def __init__(self, errors: List[str]):
        self.errors = errors
        formatted = "\n  - ".join(errors)
        super().__init__(
            f"Model validation failed with {len(errors)} error(s):\n  - {formatted}\n\n"
            f"Fix these errors before running inference."
        )


class LutufiNonCausalError(LutufiError):
    """
    Raised when a causal operation is called on a non-causal model.

    Attributes
    ----------
    operation : str
        The causal operation that was attempted.
    """
    def __init__(self, operation: str):
        super().__init__(
            f"Cannot call '{operation}' on a non-causal model.\n\n"
            f"If your edges represent direct causal mechanisms (not just "
            f"statistical associations), call model.mark_as_causal() first.\n"
            f"Warning: Only mark a model as causal if you have domain knowledge "
            f"or have used a causal discovery algorithm to establish this. "
            f"Correlation is not causation."
        )


class LutufiCyclicGraphError(LutufiError):
    """
    Raised when adding an edge would create a cycle in a directed acyclic graph.

    Attributes
    ----------
    from_node : str
        The source node of the edge that would create the cycle.
    to_node : str
        The target node of the edge that would create the cycle.
    cycle : str
        A description of the cycle path detected.
    """
    def __init__(self, from_node: str, to_node: str, cycle: str = ""):
        self.from_node = from_node
        self.to_node = to_node
        self.cycle = cycle
        msg = (
            f"Cannot add edge {from_node} -> {to_node}: this would create a cycle "
            f"in the directed graph."
        )
        if cycle:
            msg += f"\nCycle detected: {cycle}"
        msg += (
            f"\n\nBayesian networks must be directed acyclic graphs (DAGs). "
            f"Remove the edge or restructure the model to eliminate the cycle."
        )
        super().__init__(msg)


class LutufiNotIdentifiableError(LutufiError):
    """
    Raised when a causal effect is not identifiable from observational data.

    Attributes
    ----------
    target : str
        The target variable of the query.
    intervention : str
        The intervention variable.
    hedge : str
        Explanation of why the effect is not identifiable.
    """
    def __init__(self, target: str, intervention: str, hedge: str = ""):
        self.target = target
        self.intervention = intervention
        self.hedge = hedge
        msg = (
            f"Causal effect P({target} | do({intervention})) is not identifiable "
            f"from observational data given the current graph structure."
        )
        if hedge:
            msg += f"\nEvidence: {hedge}"
        msg += (
            f"\n\nThis means that multiple causal structures compatible with the "
            f"data would produce different causal effect estimates. Consider: "
            f"(1) adding more domain knowledge to refine the graph, "
            f"(2) using a different set of assumptions, or "
            f"(3) collecting interventional data (experimental or quasi-experimental)."
        )
        super().__init__(msg)


class LutufiNumericalError(LutufiError):
    """
    Raised when a numerical computation fails (NaN, Inf, underflow).

    Attributes
    ----------
    location : str
        Where the error occurred (factor, variable, operation).
    message : str
        Details about the numerical issue.
    """
    def __init__(self, location: str = "", message: str = ""):
        self.location = location
        self.message = message
        details = f" at {location}" if location else ""
        more = f": {message}" if message else ": An unexpected numerical condition was encountered."
        super().__init__(f"Numerical error{details}{more}")


class LutufiResourceLimitError(LutufiError):
    """
    Raised when a computation exceeds configured resource limits.

    Attributes
    ----------
    resource : str
        The resource that was exceeded (memory, time, nodes, etc.).
    limit : str
        The limit that was exceeded.
    """
    def __init__(self, resource: str, limit: str, detail: str = ""):
        self.resource = resource
        self.limit = limit
        msg = f"Resource limit exceeded: {resource} ({limit})"
        if detail:
            msg += f"\n{detail}"
        msg += (
            f"\n\nTo increase this limit, adjust the ResourceBudget configuration. "
            f"See the documentation for details."
        )
        super().__init__(msg)


class LutufiSerializationError(LutufiError):
    """
    Raised when model serialization or deserialization fails.

    Attributes
    ----------
    path : str
        The file path involved.
    reason : str
        The cause of the failure.
    """
    def __init__(self, path: str = "", reason: str = ""):
        self.path = path
        self.reason = reason
        loc = f" for {path}" if path else ""
        super().__init__(f"Serialization error{loc}: {reason}")


class LutufiMissingDataError(LutufiError):
    """
    Raised when missing data handling encounters an issue.

    Attributes
    ----------
    message : str
        Description of the missing data issue.
    """
    def __init__(self, message: str):
        super().__init__(f"Missing data error: {message}")


class LutufiConvergenceWarning(UserWarning):
    """
    Warning raised when an iterative algorithm fails to converge.

    Attributes
    ----------
    algorithm : str
        Name of the algorithm.
    iterations : int
        Number of iterations performed.
    residual : float
        Final convergence residual.
    """
    def __init__(self, algorithm: str, iterations: int, residual: float):
        self.algorithm = algorithm
        self.iterations = iterations
        self.residual = residual
        super().__init__(
            f"{algorithm} did not converge after {iterations} iterations. "
            f"Final residual: {residual:.6e}. "
            f"Consider increasing max_iterations or adjusting the algorithm parameters."
        )


class LutufiHighTreewidthWarning(UserWarning):
    """Warning raised when exact inference is attempted on a high-treewidth network."""
    pass


# ─── Rust Error Translation ────────────────────────────────────────────────
#
# The Rust core (src/core/error.rs) defines a rich `LutufiError` enum, but
# the PyO3 bindings currently surface every variant as a generic
# `ValueError` (via `PyValueError::new_err(e.to_string())`). The message
# text still follows the enum's `#[error("...")]` templates, so we can
# recognize those templates here and re-raise the matching
# `LutufiXxxError` subclass, giving callers a catchable, documented
# exception type without requiring changes to the compiled extension.

import re as _re

_CYCLE_RE = _re.compile(
    r"^Adding edge (.+?) -> (.+?) would create a cycle: (.*)$", _re.DOTALL
)
_RESOURCE_LIMIT_RE = _re.compile(
    r"^Resource limit exceeded: (\S+) \(limit: (.+?), requested: (.+?)\)\.\s*(.*)$",
    _re.DOTALL,
)


def _translate_error(exc: ValueError) -> Exception:
    """Translate a generic ``ValueError`` from the Rust core into the
    matching ``LutufiError`` subclass, based on its message text.

    Returns `exc` unchanged if the message doesn't match a known
    `LutufiError` template (e.g. errors that don't yet have a dedicated
    Python exception class).
    """
    msg = str(exc)

    m = _CYCLE_RE.match(msg)
    if m:
        from_node, to_node, cycle = m.groups()
        return LutufiCyclicGraphError(from_node, to_node, cycle)

    m = _RESOURCE_LIMIT_RE.match(msg)
    if m:
        resource, limit, requested, detail = m.groups()
        return LutufiResourceLimitError(resource, f"{limit}, requested {requested}", detail)

    if msg.startswith("Numerical underflow") or msg.startswith(
        "Attempted to compute log-probability"
    ):
        return LutufiNumericalError(message=msg)

    return exc


def _call_rust(fn, *args, **kwargs):
    """Call a Rust FFI method, translating a generic ``ValueError`` (if
    any) into the matching ``LutufiError`` subclass before re-raising."""
    try:
        return fn(*args, **kwargs)
    except ValueError as exc:
        translated = _translate_error(exc)
        if translated is exc:
            raise
        raise translated from exc


# ─── Main BayesianNetwork Class ───────────────────────────────────────────────

class NetworkModel:
    """Base class for all Lutufi network models."""
    def __init__(self, name: str = "unnamed"):
        self.name = name

    def nodes(self) -> List[str]:
        raise NotImplementedError()

    def edges(self) -> List[Tuple[str, str]]:
        raise NotImplementedError()

    def is_valid(self) -> bool:
        raise NotImplementedError()

    def to_networkx(self):
        """Convert this model to a NetworkX graph.

        Note: probabilistic information (CPTs, factors) is lost in conversion
        since NetworkX has no native CPT storage.
        """
        import networkx as nx
        G = nx.DiGraph() if isinstance(self, BayesianNetwork) else nx.Graph()
        G.add_nodes_from(self.nodes())
        G.add_edges_from(self.edges())
        return G

    @contextmanager
    def edit(self) -> Iterator["NetworkModel"]:
        """Atomic edit context manager.

        Changes made within the context are committed only if no exception occurs.
        If any step fails, all changes are rolled back.

        Example:
            >>> with model.edit() as m:
            ...     m.add_edge("A", "B")
            ...     m.set_cpd("B", [[0.9, 0.2], [0.1, 0.8]])
        """
        # Snapshot the current state
        import copy
        snapshot = _RustBayesianNetwork() if isinstance(self, BayesianNetwork) else None
        if snapshot is not None and hasattr(self, '_model'):
            snapshot = copy.deepcopy(self._model)

        try:
            yield self
            # Validate after modifications
            if hasattr(self, '_model') and not self._model.is_valid():
                errors = self._model.validate().errors
                raise LutufiValidationError(errors)
        except Exception:
            # Rollback: restore from snapshot
            if snapshot is not None and hasattr(self, '_model'):
                self._model = snapshot
            raise


class BayesianNetwork(NetworkModel):
    """
    A Bayesian Network — a directed acyclic graph where each node is a random
    variable with a conditional probability table (CPT).

    Parameters
    ----------
    name : str, optional
        A human-readable name for the network (default: "unnamed").
    """

    def __init__(self, _rust_model: _RustBayesianNetwork, name: str = "unnamed"):
        super().__init__(name=name)
        self._model = _rust_model

    def get_states(self, variable_name: str) -> List[str]:
        """Get the possible states for a variable.

        Args:
            variable_name: Name of the variable.

        Returns:
            List of state names.
        """
        return self._model.get_states(variable_name)

    def save(self, path: Union[str, Path]) -> None:
        """Serialize this network (structure + CPDs) to Lutufi's native LMF
        format at `path`.

        Unlike `to_networkx()` / `lutufi.io.write_graph()`, this preserves
        full model fidelity — variables, edges, domains, and fitted CPDs —
        so the exact model can be reloaded with `BayesianNetwork.load()`.

        Args:
            path: Destination file path (conventionally ``*.lmf``).

        Example:
            >>> model.save("trained_model.lmf")
            >>> reloaded = BayesianNetwork.load("trained_model.lmf")
        """
        try:
            self._model.save(str(path))
        except ValueError as exc:
            raise LutufiSerializationError(path=str(path), reason=str(exc)) from exc

    @classmethod
    def load(cls, path: Union[str, Path]) -> "BayesianNetwork":
        """Load a Bayesian network (structure + CPDs) previously written by
        `save()` from Lutufi's native LMF format at `path`.

        Args:
            path: Path to an LMF file written by `save()`.

        Returns:
            The reconstructed BayesianNetwork, with CPDs intact.
        """
        try:
            rust_model = _RustBayesianNetwork.load(str(path))
        except ValueError as exc:
            raise LutufiSerializationError(path=str(path), reason=str(exc)) from exc
        return cls(rust_model)

    def to_lmf_json(self) -> str:
        """Serialize this network (structure + CPDs) to an LMF JSON string,
        without touching the filesystem.

        Useful for embedding a trained model directly in another artifact
        (e.g. a config blob, a database row, or bundled alongside other
        model weights) instead of shipping a separate ``.lmf`` file.

        Returns:
            The LMF document as a JSON string.
        """
        try:
            return self._model.to_lmf_json()
        except ValueError as exc:
            raise LutufiSerializationError(reason=str(exc)) from exc

    @classmethod
    def from_lmf_json(cls, json: str) -> "BayesianNetwork":
        """Reconstruct a BayesianNetwork from an LMF JSON string produced by
        `to_lmf_json()`.

        Args:
            json: An LMF document as a JSON string.

        Returns:
            The reconstructed BayesianNetwork, with CPDs intact.
        """
        try:
            rust_model = _RustBayesianNetwork.from_lmf_json(json)
        except ValueError as exc:
            raise LutufiSerializationError(reason=str(exc)) from exc
        return cls(rust_model)

    @classmethod
    def builder(cls) -> "BayesianNetworkBuilder":
        """Create a fluent builder for constructing a BayesianNetwork.

        Example:
            >>> bn = (BayesianNetwork.builder()
            ...     .add_variable("A", domain=["0", "1"])
            ...     .add_variable("B", domain=["0", "1"])
            ...     .add_edge("A", "B")
            ...     .set_cpd("A", [0.3, 0.7])
            ...     .set_cpd("B", [[0.9, 0.2], [0.1, 0.8]])
            ...     .build())
        """
        return BayesianNetworkBuilder()

    @classmethod
    def from_dataframe(
        cls,
        data,
        structure: List[Tuple[str, str]],
        state_names: Optional[Dict[str, List[str]]] = None,
        estimator: str = "mle",
        prior_counts: float = 0.5,
    ) -> "BayesianNetwork":
        """Construct a BayesianNetwork from a pandas DataFrame.

        Learns CPTs from the data given the specified graph structure.

        Args:
            data: A pandas DataFrame with one column per variable.
            structure: List of (parent, child) edges defining the DAG structure.
            state_names: Optional dict mapping variable names to their domain states.
                If not provided, states are inferred from the unique values in each column.
            estimator: Parameter estimation method ("mle" or "bayesian").
            prior_counts: Pseudocount for smoothing (default 0.5).

        Returns:
            A new BayesianNetwork with learned parameters.
        """
        try:
            import pandas as pd
        except ImportError:
            raise ImportError("pandas is required for from_dataframe().")
        if not isinstance(data, pd.DataFrame):
            raise TypeError("data must be a pandas DataFrame")

        builder = cls.builder()

        variables = set()
        for u, v in structure:
            variables.add(u)
            variables.add(v)

        for var in variables:
            if state_names and var in state_names:
                builder.add_variable(var, state_names[var])
            else:
                states = sorted(data[var].dropna().unique().astype(str).tolist())
                builder.add_variable(var, states)

        for parent, child in structure:
            builder.add_edge(parent, child)

        from lutufi.learning import ParameterEstimator
        estimator_obj = ParameterEstimator(method=estimator, alpha=prior_counts)
        # CPTs are learned from data below, so skip the build()-time
        # validation (which would fail on a model with no CPTs yet).
        bn = BayesianNetwork(builder._rust)
        estimator_obj.fit(bn, data)
        if not bn.is_valid():
            errors = bn.validate().errors
            if errors:
                raise LutufiValidationError(errors)
        return bn

    @classmethod
    def from_networkx(cls, G, cpds: Optional[Dict] = None,
                      state_names: Optional[Dict[str, List[str]]] = None) -> "BayesianNetwork":
        """Construct a BayesianNetwork from a NetworkX DiGraph.

        Args:
            G: A NetworkX DiGraph (directed graph). Node attributes may include
               a 'domain' key with a list of state names.
            cpds: Optional dict mapping variable names to CPT values.
            state_names: Optional dict mapping variable names to domain states.

        Returns:
            A new BayesianNetwork.
        """
        try:
            import networkx as nx
        except ImportError:
            raise ImportError("networkx is required for from_networkx().")

        builder = cls.builder()

        domains: Dict[str, List[str]] = {}
        for node in G.nodes():
            domain = state_names.get(str(node)) if state_names else None
            if domain is None:
                domain = G.nodes[node].get("domain", ["0", "1"])
            domain = list(domain)
            domains[str(node)] = domain
            builder.add_variable(str(node), domain)

        parents_of: Dict[str, List[str]] = {str(node): [] for node in G.nodes()}
        for u, v in G.edges():
            builder.add_edge(str(u), str(v))
            parents_of[str(v)].append(str(u))

        # NetworkX graphs carry no CPT information, so fill in uniform
        # CPDs for any variable that wasn't given an explicit one. This
        # keeps the resulting model structurally faithful while still
        # passing build()'s validation.
        cpds = cpds or {}
        for var_name, domain in domains.items():
            if var_name in cpds:
                continue
            k = len(domain)
            num_configs = 1
            for parent in parents_of[var_name]:
                num_configs *= len(domains[parent])
            uniform = 1.0 / k
            if num_configs == 1:
                builder.set_cpd(var_name, [uniform] * k)
            else:
                builder.set_cpd(var_name, [[uniform] * num_configs for _ in range(k)])

        for var_name, values in cpds.items():
            builder.set_cpd(var_name, values)

        return builder.build()

    def nodes(self) -> List[str]:
        """Get all variable names in the network.

        Returns:
            A list of variable name strings.
        """
        return self._model.nodes()

    def edges(self) -> List[Tuple[str, str]]:
        """Get all directed edges in the network.

        Returns:
            A list of (parent, child) tuples.
        """
        return self._model.edges()

    def cpd(self, variable_name: str) -> np.ndarray:
        """Get the CPT for a variable as a numpy array.

        Args:
            variable_name: Name of the variable.

        Returns:
            A numpy array containing the CPT values.
        """
        raw = self._model.get_cpd(variable_name)
        return np.array(raw["values"])

    def markov_blanket(self, variable_name: str) -> List[str]:
        """Get the Markov blanket of a variable.

        The Markov blanket includes the variable's parents, children,
        and co-parents (other parents of its children).

        Args:
            variable_name: Name of the variable.

        Returns:
            List of variable names in the Markov blanket.
        """
        return self._model.markov_blanket(variable_name)

    def topological_order(self) -> List[str]:
        """Get a topological ordering of the network variables.

        Returns:
            Variables ordered such that parents appear before children.
        """
        return self._model.topological_order()

    def is_valid(self) -> bool:
        """Check if the model is valid for inference.

        A valid model must:
        - Be a DAG (no cycles)
        - Have all CPTs assigned
        - Have matching parent sets between CPTs and graph

        Returns:
            True if the model is valid, False otherwise.
        """
        return self._model.is_valid()

    def validate(self) -> "ValidationResult":
        """Validate the model and return detailed error information.

        Returns:
            A ValidationResult with is_valid and errors attributes.
        """
        return self._model.validate()

    def mark_as_causal(self) -> "BayesianNetwork":
        """Explicitly mark this model as causal.

        Required before calling causal operations like do(), identify(),
        or counterfactual().

        Returns:
            self for method chaining.
        """
        self._model.mark_as_causal()
        return self

    @property
    def is_causal(self) -> bool:
        """Whether this model has been marked as causal."""
        return self._model.is_causal()

    # ─── Causal operations (require mark_as_causal()) ────────────────────

    def do(self, intervention: Dict[str, str]) -> "BayesianNetwork":
        """Apply Pearl's do-operator and return the interventional network.

        Removes all incoming edges to each intervened variable and fixes it
        to the given value (graph mutilation). The returned network answers
        interventional queries: querying it is querying P(· | do(...)).

        Args:
            intervention: Mapping of variable name -> forced state value.

        Returns:
            A new BayesianNetwork representing the mutilated model.

        Example:
            >>> model.mark_as_causal()
            >>> intervened = model.do({"Sprinkler": "T"})
        """
        rust = _call_rust(self._model.do_operator, intervention)
        return BayesianNetwork(rust, name=f"{self.name}|do")

    def causal_query(
        self, targets: List[str], interventions: Dict[str, str]
    ) -> Dict[str, List[float]]:
        """Compute P(targets | do(interventions)).

        Unlike conditioning, this answers "what would happen if we *forced*
        these variables to these values" — correlation vs. causation.

        Args:
            targets: Variables whose interventional marginals to compute.
            interventions: Mapping of variable name -> forced state value.

        Returns:
            Dict mapping each target name to its marginal probability list
            (aligned with get_states(target)).
        """
        return _call_rust(self._model.causal_query, targets, interventions)

    def counterfactual(
        self,
        observed: Dict[str, str],
        intervention: Dict[str, str],
        query: List[str],
    ) -> Dict[str, List[float]]:
        """Counterfactual query: given `observed` evidence, what would the
        `query` variables look like under `intervention`?

        Returns:
            Dict mapping each query variable to its marginal probability list.
        """
        return _call_rust(self._model.counterfactual, observed, intervention, query)

    def probability_of_necessity(
        self, outcome: str, outcome_value: str, treatment: str,
        treatment_value: str, reference_value: str,
    ) -> float:
        """Pearl's probability of necessity (PN): the probability the outcome
        would NOT have occurred had the treatment been `reference_value`
        instead, given that it did occur under `treatment_value`.
        """
        return _call_rust(
            self._model.probability_of_necessity,
            outcome, outcome_value, treatment, treatment_value, reference_value,
        )

    def probability_of_sufficiency(
        self, outcome: str, outcome_value: str, treatment: str,
        treatment_value: str, reference_value: str,
    ) -> float:
        """Pearl's probability of sufficiency (PS): the probability the
        outcome WOULD have occurred under `treatment_value`, given that it
        did not occur under `reference_value`.
        """
        return _call_rust(
            self._model.probability_of_sufficiency,
            outcome, outcome_value, treatment, treatment_value, reference_value,
        )

    def identify(
        self, targets: List[str], interventions: List[str]
    ) -> Tuple[bool, str]:
        """Run the ID algorithm: is P(targets | do(interventions))
        identifiable from observational data given this graph structure?

        Returns:
            (identifiable, detail) — `detail` is the identification formula
            when identifiable, or the reason (hedge) when not.
        """
        return _call_rust(self._model.identify, targets, interventions)

    def d_separated(self, a: str, b: str, given: Optional[List[str]] = None) -> bool:
        """Test whether two variables are d-separated given a set of observed variables.

        Args:
            a: Name of the first variable.
            b: Name of the second variable.
            given: Optional list of names of observed (conditioning) variables.

        Returns:
            True if a and b are d-separated, False otherwise.

        Example:
            >>> model.d_separated("Rain", "Sprinkler", given=["Cloudy"])
            True
        """
        return self._model.d_separated(a, b, given or [])

    def to_networkx(self):
        """Convert this BayesianNetwork to a NetworkX DiGraph.

        Note: CPT data is not preserved in the conversion, since NetworkX
        has no native CPT storage. Use model.save() for full fidelity.

        Returns:
            A NetworkX DiGraph with the same structure.
        """
        import networkx as nx
        G = nx.DiGraph()
        G.add_nodes_from(self.nodes())
        G.add_edges_from(self.edges())
        return G

    @contextmanager
    def edit(self) -> Iterator["BayesianNetwork"]:
        """Atomic edit context manager.

        Changes made within the context are committed atomically.
        If any step fails or validation fails, all changes are rolled back.

        Example:
            >>> with model.edit() as m:
            ...     m.add_edge("A", "B")
            ...     m.set_cpd("B", [[0.9, 0.2], [0.1, 0.8]])
        """
        import copy
        snapshot = copy.deepcopy(self._model) if hasattr(self, '_model') else None
        try:
            yield self
            if not self._model.is_valid():
                errors = self._model.validate().errors
                raise LutufiValidationError(errors)
        except Exception:
            if snapshot is not None:
                self._model = snapshot
            raise

    def plot(self, **kwargs):
        """Visualize the network structure.

        Draws the graph with node labels. Node size is proportional to
        the number of states, and edge direction is indicated by arrows.

        Args:
            **kwargs: Additional keyword arguments passed to
                networkx.draw_networkx.

        Returns:
            A matplotlib Figure and Axes (fig, ax).
        """
        import matplotlib.pyplot as plt
        import networkx as nx

        G = self.to_networkx()
        fig, ax = plt.subplots(figsize=kwargs.pop("figsize", (10, 8)))
        pos = kwargs.pop("pos", nx.spring_layout(G, seed=42))

        nx.draw_networkx(
            G, pos=pos, ax=ax, with_labels=True,
            node_color="lightblue", node_size=kwargs.pop("node_size", 1500),
            arrowsize=kwargs.pop("arrowsize", 20),
            font_size=kwargs.pop("font_size", 10),
            **kwargs
        )
        ax.set_title(kwargs.pop("title", f"Bayesian Network: {self.name}"), fontsize=14)
        ax.axis("off")
        plt.tight_layout()
        return fig, ax

    def plot_cpd(self, variable_name: str, **kwargs):
        """Visualize the CPT of a variable as a heatmap.

        Args:
            variable_name: Name of the variable to visualize.
            **kwargs: Additional keyword arguments passed to
                matplotlib.pyplot.subplots or imshow.

        Returns:
            A matplotlib Figure and Axes (fig, ax).
        """
        import matplotlib.pyplot as plt

        cpd_array = self.cpd(variable_name)
        states = self.get_states(variable_name)

        # cpd_array is a flat array in (parent_config, state) order; reshape
        # so each row is one parent configuration and each column is a
        # state of `variable_name`.
        fig, ax = plt.subplots(figsize=kwargs.pop("figsize", (8, 4)))
        im = ax.imshow(cpd_array.reshape(-1, len(states)), aspect="auto", cmap="YlOrRd")

        ax.set_xticks(range(len(states)))
        ax.set_xticklabels(states, fontsize=10)
        ax.set_xlabel(f"States of {variable_name}")
        ax.set_ylabel("Parent configurations")
        ax.set_title(f"CPT: P({variable_name} | Parents)", fontsize=12)

        plt.colorbar(im, ax=ax, label="Probability")
        plt.tight_layout()
        return fig, ax

    def __repr__(self) -> str:
        return (
            f"BayesianNetwork(nodes={len(self.nodes())}, "
            f"edges={len(self.edges())}, "
            f"valid={self.is_valid()})"
        )

    def __str__(self) -> str:
        lines = [
            f"Bayesian Network: {self.name}",
            f"  Nodes: {len(self.nodes())}",
            f"  Edges: {len(self.edges())}",
            f"  Causal: {self.is_causal}",
            f"  Valid for inference: {self.is_valid()}",
        ]
        if self.nodes():
            lines.append("  Variables:")
            for node in sorted(self.nodes()):
                lines.append(f"    - {node}  (states: {self.get_states(node)})")
        return "\n".join(lines)

    def __len__(self) -> int:
        return len(self.nodes())

    def __eq__(self, other) -> bool:
        if not isinstance(other, BayesianNetwork):
            return NotImplemented
        return (
            sorted(self.nodes()) == sorted(other.nodes())
            and sorted(self.edges()) == sorted(other.edges())
        )


class MarkovRandomField(NetworkModel):
    """
    A Markov Random Field (MRF) — an undirected graphical model where
    potentials are defined over cliques.

    Parameters
    ----------
    _rust_model : _RustMarkovRandomField, optional
        Internal Rust model. Creates a new one if not provided.
    name : str, optional
        A human-readable name for the network (default: "unnamed").
    """
    def __init__(self, _rust_model: Optional[_RustMarkovRandomField] = None,
                 name: str = "unnamed"):
        super().__init__(name=name)
        self._model = _rust_model or _RustMarkovRandomField()

    def add_variable(self, name: str, domain: List[str]) -> "MarkovRandomField":
        """Add a variable to the MRF.

        Args:
            name: Variable name.
            domain: List of state names for the variable.

        Returns:
            self for method chaining.
        """
        self._model.add_variable(name, domain)
        return self

    def add_edge(self, name1: str, name2: str) -> "MarkovRandomField":
        """Add an undirected edge between two variables.

        Args:
            name1: First variable name.
            name2: Second variable name.

        Returns:
            self for method chaining.
        """
        _call_rust(self._model.add_edge, name1, name2)
        return self

    def nodes(self) -> List[str]:
        """Get all variable names in the MRF.

        Returns:
            A list of variable name strings.
        """
        return self._model.nodes()

    def edges(self) -> List[Tuple[str, str]]:
        """Get all undirected edges.

        Returns:
            A list of (node1, node2) tuples.
        """
        return self._model.edges()

    def is_valid(self) -> bool:
        """Check if the MRF is valid.

        Returns:
            True if the MRF has nodes and is properly constructed.
        """
        return len(self.nodes()) > 0

    @classmethod
    def from_networkx(cls, G, potentials: Optional[Dict] = None,
                      state_names: Optional[Dict[str, List[str]]] = None) -> "MarkovRandomField":
        """Construct an MRF from a NetworkX Graph.

        Args:
            G: A NetworkX Graph (undirected).
            potentials: Optional dict mapping edge or clique identifiers to potentials.
            state_names: Optional dict mapping variable names to domain states.

        Returns:
            A new MarkovRandomField.
        """
        try:
            import networkx as nx
        except ImportError:
            raise ImportError("networkx is required for from_networkx().")

        mrf = cls()
        for node in G.nodes():
            domain = state_names.get(str(node)) if state_names else G.nodes[node].get("domain", ["0", "1"])
            mrf.add_variable(str(node), list(domain))
        for u, v in G.edges():
            mrf.add_edge(str(u), str(v))
        return mrf

    def to_networkx(self):
        """Convert this MRF to a NetworkX Graph.

        Note: potential data is not preserved in conversion.

        Returns:
            A NetworkX Graph with the same structure.
        """
        import networkx as nx
        G = nx.Graph()
        G.add_nodes_from(self.nodes())
        G.add_edges_from(self.edges())
        return G

    def plot(self, **kwargs):
        """Visualize the MRF structure.

        Args:
            **kwargs: Additional keyword arguments for networkx.draw_networkx.

        Returns:
            A matplotlib Figure and Axes (fig, ax).
        """
        import matplotlib.pyplot as plt
        import networkx as nx

        G = self.to_networkx()
        fig, ax = plt.subplots(figsize=kwargs.pop("figsize", (10, 8)))
        pos = kwargs.pop("pos", nx.spring_layout(G, seed=42))

        nx.draw_networkx(
            G, pos=pos, ax=ax, with_labels=True,
            node_color="lightgreen", node_size=kwargs.pop("node_size", 1500),
            font_size=kwargs.pop("font_size", 10),
            **kwargs
        )
        ax.set_title(kwargs.pop("title", f"Markov Random Field: {self.name}"), fontsize=14)
        ax.axis("off")
        plt.tight_layout()
        return fig, ax

    def __repr__(self) -> str:
        return f"MarkovRandomField(nodes={len(self.nodes())}, edges={len(self.edges())})"

    def __str__(self) -> str:
        lines = [
            f"Markov Random Field: {self.name}",
            f"  Nodes: {len(self.nodes())}",
            f"  Edges: {len(self.edges())}",
        ]
        return "\n".join(lines)

    def __len__(self) -> int:
        return len(self.nodes())


class DynamicBayesianNetwork(NetworkModel):
    """
    A Dynamic Bayesian Network (DBN) — a two-slice temporal model.

    Stores a prior network (time 0) and a transition network (time t to t+1).
    Variables are referred to as name_t (time slice 0) and name_t1 (time slice 1).

    Parameters
    ----------
    _rust_model : _RustDynamicBayesianNetwork, optional
        Internal Rust model. Creates a new one if not provided.
    name : str, optional
        A human-readable name for the network (default: "unnamed").
    """
    def __init__(self, _rust_model: Optional[_RustDynamicBayesianNetwork] = None,
                 name: str = "unnamed"):
        super().__init__(name=name)
        self._model = _rust_model or _RustDynamicBayesianNetwork()
        # The native _RustDynamicBayesianNetwork does not expose nodes()/edges()
        # accessors, so track structure on the Python side as it is built up.
        self._node_names: List[str] = []
        self._intra_edges: List[Tuple[str, str]] = []
        self._inter_edges: List[Tuple[str, str]] = []

    def add_variable(self, name: str, domain: List[str]) -> "DynamicBayesianNetwork":
        """Add a variable to the DBN.

        Args:
            name: Variable name.
            domain: List of state names for the variable.

        Returns:
            self for method chaining.
        """
        self._model.add_variable(name, domain)
        self._node_names.append(name)
        return self

    def add_intraslice_edge(self, from_node: str, to_node: str) -> "DynamicBayesianNetwork":
        """Add an edge within the same time slice.

        Args:
            from_node: Source variable name.
            to_node: Target variable name.

        Returns:
            self for method chaining.

        Raises:
            LutufiCyclicGraphError: If the edge would create a cycle.
        """
        _call_rust(self._model.add_intraslice_edge, from_node, to_node)
        self._intra_edges.append((from_node, to_node))
        return self

    def add_interslice_edge(self, from_node: str, to_node: str) -> "DynamicBayesianNetwork":
        """Add a temporal edge from time t to time t+1.

        Args:
            from_node: Variable name at time t (e.g., "X_t").
            to_node: Variable name at time t+1 (e.g., "X_t1").

        Returns:
            self for method chaining.
        """
        _call_rust(self._model.add_interslice_edge, from_node, to_node)
        self._inter_edges.append((from_node, to_node))
        return self

    def nodes(self) -> List[str]:
        """Get all variable names in the DBN."""
        return list(self._node_names)

    def edges(self) -> List[Tuple[str, str]]:
        """Get all edges in the DBN (both intra-slice and inter-slice)."""
        return self._intra_edges + self._inter_edges

    def is_valid(self) -> bool:
        """Check if the DBN is valid.

        Returns:
            True if the DBN has at least one variable.
        """
        return len(self._node_names) > 0

    @classmethod
    def from_networkx(cls, G, intra_edges: Optional[List[Tuple[str, str]]] = None,
                      inter_edges: Optional[List[Tuple[str, str]]] = None,
                      state_names: Optional[Dict[str, List[str]]] = None) -> "DynamicBayesianNetwork":
        """Construct a DBN from a NetworkX graph.

        Args:
            G: A NetworkX DiGraph.
            intra_edges: Intra-slice edges.
            inter_edges: Inter-slice (temporal) edges.
            state_names: Optional dict mapping variable names to domain states.

        Returns:
            A new DynamicBayesianNetwork.
        """
        try:
            import networkx as nx
        except ImportError:
            raise ImportError("networkx is required for from_networkx().")

        dbn = cls()
        for node in G.nodes():
            domain = state_names.get(str(node)) if state_names else G.nodes[node].get("domain", ["0", "1"])
            dbn.add_variable(str(node), list(domain))

        if intra_edges:
            for u, v in intra_edges:
                dbn.add_intraslice_edge(str(u), str(v))
        if inter_edges:
            for u, v in inter_edges:
                dbn.add_interslice_edge(str(u), str(v))
        return dbn

    def to_networkx(self):
        """Convert this DBN to a NetworkX DiGraph.

        Note: temporal edge type information is not preserved.

        Returns:
            A NetworkX DiGraph with all edges.
        """
        import networkx as nx
        G = nx.DiGraph()
        G.add_nodes_from(self.nodes())
        G.add_edges_from(self.edges())
        return G

    def __repr__(self) -> str:
        return "DynamicBayesianNetwork()"

    def __str__(self) -> str:
        return f"Dynamic Bayesian Network: {self.name}"

    def __len__(self) -> int:
        return len(self.nodes())


# ─── Builder ──────────────────────────────────────────────────────────────────

class BayesianNetworkBuilder:
    """Fluent builder for constructing a BayesianNetwork.

    Example:
        >>> bn = (BayesianNetwork.builder()
        ...     .add_variable("A", domain=["0", "1"])
        ...     .add_variable("B", domain=["0", "1"])
        ...     .add_edge("A", "B")
        ...     .set_cpd("A", [0.3, 0.7])
        ...     .set_cpd("B", [[0.9, 0.2], [0.1, 0.8]])
        ...     .build())
    """
    def __init__(self):
        self._rust = _RustBayesianNetwork()

    def add_variable(self, name: str, domain: List[str]) -> "BayesianNetworkBuilder":
        """Add a variable to the network.

        Args:
            name: Variable name.
            domain: List of state names.

        Returns:
            self for method chaining.
        """
        self._rust.add_variable(name, domain)
        return self

    def add_edge(self, parent: str, child: str) -> "BayesianNetworkBuilder":
        """Add a directed edge from parent to child.

        Args:
            parent: Name of the parent variable.
            child: Name of the child variable.

        Returns:
            self for method chaining.

        Raises:
            LutufiCyclicGraphError: If the edge would create a cycle.
        """
        _call_rust(self._rust.add_edge, parent, child)
        return self

    def set_cpd(self, variable: str, values: Union[List[float], List[List[float]]]) -> "BayesianNetworkBuilder":
        """Set the conditional probability table for a variable.

        Args:
            variable: Name of the variable.
            values: Flat list of probabilities (for root nodes) or a nested
                list with one row per child state and one column per parent
                configuration. Each column (the distribution over the child's
                states for a fixed parent configuration) must sum to 1.

        Returns:
            self for method chaining.
        """
        self._rust.set_cpd(variable, values)
        return self

    def build(self) -> BayesianNetwork:
        """Build and return the BayesianNetwork.

        Returns:
            A fully constructed BayesianNetwork.

        Raises:
            LutufiValidationError: If the network is invalid.
        """
        bn = BayesianNetwork(self._rust)
        if not bn.is_valid():
            errors = bn.validate().errors
            if errors:
                raise LutufiValidationError(errors)
        return bn


