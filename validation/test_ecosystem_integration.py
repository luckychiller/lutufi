"""Tests for Phase 8 Sprint 8.2 — Ecosystem Integration.

Covers: numpy array inputs/outputs, pandas integration,
NetworkX round-trip for all model types, matplotlib visualization.
"""

import pytest
import numpy as np
from lutufi import (
    BayesianNetwork,
    MarkovRandomField,
    DynamicBayesianNetwork,
    InferenceEngine,
    QueryResult,
)
from validation.test_asia_construction import build_asia_network


# ─── NumPy Array Integration ──────────────────────────────────────────────────

class TestNumPyIntegration:
    def test_cpd_returns_numpy_array(self):
        """cpd() returns a numpy array, not a list."""
        model = build_asia_network()
        cpd = model.cpd("Asia")
        assert isinstance(cpd, np.ndarray)

    def test_query_result_values_are_numpy_arrays(self):
        """Query result distributions are numpy arrays."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia"])
        assert isinstance(result["Asia"], np.ndarray)

    def test_multi_variable_query_joint(self):
        """Query with multiple variables returns joint distribution."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia", "Smoking"])
        assert len(result.distributions) == 2
        assert "Asia" in result.distributions
        assert "Smoking" in result.distributions


# ─── Pandas Integration ───────────────────────────────────────────────────────

class TestPandasIntegration:
    def test_query_result_to_dataframe(self):
        """Single-variable query result converts to tidy DataFrame."""
        import pandas as pd
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia"])
        df = result.to_dataframe()
        assert isinstance(df, pd.DataFrame)
        assert list(df.columns) == ["state", "probability"]
        assert len(df) == 2  # F and T

    def test_query_result_to_dataframe_multi(self):
        """Multi-variable query result converts to DataFrame."""
        import pandas as pd
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia", "Smoking"])
        df = result.to_dataframe()
        assert isinstance(df, pd.DataFrame)
        assert "Asia" in df.columns
        assert "Smoking" in df.columns
        assert "probability" in df.columns

    def test_from_dataframe_with_structure(self):
        """from_dataframe creates a model from pandas data."""
        import pandas as pd
        np.random.seed(42)
        n = 1000
        data = pd.DataFrame({
            "A": np.random.choice(["0", "1"], n),
            "B": np.random.choice(["0", "1"], n),
            "C": np.random.choice(["0", "1"], n),
        })
        structure = [("A", "C"), ("B", "C")]
        model = BayesianNetwork.from_dataframe(
            data, structure=structure,
            state_names={"A": ["0", "1"], "B": ["0", "1"], "C": ["0", "1"]},
        )
        assert isinstance(model, BayesianNetwork)
        assert len(model.nodes()) == 3
        assert len(model.edges()) == 2


# ─── NetworkX Round-Trip ──────────────────────────────────────────────────────

class TestNetworkXRoundTrip:
    def test_bayesian_network_roundtrip(self):
        """BayesianNetwork to/from NetworkX preserves structure."""
        import networkx as nx
        original = build_asia_network()
        G = original.to_networkx()
        assert isinstance(G, nx.DiGraph)
        recovered = BayesianNetwork.from_networkx(G)
        assert set(original.nodes()) == set(recovered.nodes())
        assert set(original.edges()) == set(recovered.edges())

    def test_markov_random_field_roundtrip(self):
        """MarkovRandomField to/from NetworkX preserves structure."""
        import networkx as nx
        mrf = MarkovRandomField(name="test")
        mrf.add_variable("A", ["0", "1"])
        mrf.add_variable("B", ["0", "1"])
        mrf.add_variable("C", ["0", "1"])
        mrf.add_edge("A", "B")
        mrf.add_edge("B", "C")

        G = mrf.to_networkx()
        assert isinstance(G, nx.Graph)
        assert set(G.nodes()) == {"A", "B", "C"}
        # Edge endpoint order is not meaningful for an undirected graph,
        # so compare as unordered pairs.
        assert {frozenset(e) for e in G.edges()} == {frozenset(("A", "B")), frozenset(("B", "C"))}

        recovered = MarkovRandomField.from_networkx(G)
        assert set(recovered.nodes()) == set(mrf.nodes())
        assert {frozenset(e) for e in recovered.edges()} == {frozenset(e) for e in mrf.edges()}

    def test_bayesian_network_from_networkx_with_domains(self):
        """from_networkx reads node attributes for domains."""
        import networkx as nx
        G = nx.DiGraph()
        G.add_node("A", domain=["low", "high"])
        G.add_node("B", domain=["0", "1"])
        G.add_edge("A", "B")
        bn = BayesianNetwork.from_networkx(G)
        assert bn.get_states("A") == ["low", "high"]
        assert bn.get_states("B") == ["0", "1"]


# ─── Matplotlib Visualization ─────────────────────────────────────────────────

class TestVisualization:
    def test_bayesian_network_plot_returns_figure(self):
        """model.plot() returns a matplotlib Figure."""
        import matplotlib
        matplotlib.use("Agg")  # non-interactive backend
        import matplotlib.pyplot as plt

        model = build_asia_network()
        fig, ax = model.plot()
        assert fig is not None
        plt.close(fig)

    def test_bayesian_network_plot_cpd_returns_figure(self):
        """model.plot_cpd() returns a matplotlib Figure."""
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt

        model = build_asia_network()
        fig, ax = model.plot_cpd("Asia")
        assert fig is not None
        plt.close(fig)

    def test_query_result_plot_returns_figure(self):
        """result.plot() returns a matplotlib Figure."""
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt

        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia"])
        fig, axes = result.plot()
        assert fig is not None
        plt.close(fig)


# ─── I/O Integration ──────────────────────────────────────────────────────────

class TestIOIntegration:
    def test_read_graph_graphml(self, tmp_path):
        """read_graph reads GraphML through NetworkX."""
        import networkx as nx
        from lutufi.io import read_graph, write_graph

        G = nx.DiGraph()
        G.add_node("A", domain=["0", "1"])
        G.add_node("B", domain=["0", "1"])
        G.add_edge("A", "B")

        path = tmp_path / "test.graphml"
        write_graph(G, str(path))
        loaded = read_graph(str(path))
        assert set(loaded.nodes()) == {"A", "B"}
        assert ("A", "B") in list(loaded.edges())

    def test_write_graph_from_model(self, tmp_path):
        """write_graph accepts a Lutufi model."""
        import networkx as nx
        from lutufi.io import write_graph

        model = build_asia_network()
        path = tmp_path / "asia.graphml"
        write_graph(model, str(path))
        loaded = nx.read_graphml(str(path))
        assert len(loaded.nodes()) == 8
        assert len(loaded.edges()) == 8

    def test_graph_reader_writer_classes(self, tmp_path):
        """GraphReader and GraphWriter class-based API works."""
        import networkx as nx
        from lutufi.io import GraphReader, GraphWriter

        G = nx.DiGraph()
        G.add_edge("A", "B")
        path = tmp_path / "test.graphml"

        writer = GraphWriter(format="graphml")
        writer.write(G, str(path))

        reader = GraphReader(format="graphml")
        loaded = reader.read(str(path))
        assert ("A", "B") in list(loaded.edges())
