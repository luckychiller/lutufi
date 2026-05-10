"""Tests for Phase 8 Sprint 8.1 — Core Python API Finalization.

Covers: error message library, context managers, lazy evaluation,
async inference, and API consistency.
"""

import pytest
import warnings
import numpy as np
from lutufi import (
    BayesianNetwork,
    MarkovRandomField,
    DynamicBayesianNetwork,
    InferenceEngine,
    LazyQueryResult,
    LutufiCyclicGraphError,
    LutufiNotIdentifiableError,
    LutufiNumericalError,
    LutufiResourceLimitError,
    LutufiSerializationError,
    LutufiMissingDataError,
    LutufiConvergenceWarning,
    LutufiHighTreewidthWarning,
)
from validation.test_asia_construction import build_asia_network


# ─── Error Message Library ────────────────────────────────────────────────────

class TestErrorMessageLibrary:
    def test_cyclic_graph_error_message(self):
        """CyclicGraphError mentions the edge and cycle."""
        builder = BayesianNetwork.builder()
        builder.add_variable("A", domain=["0", "1"])
        builder.add_variable("B", domain=["0", "1"])
        builder.add_variable("C", domain=["0", "1"])
        builder.add_edge("A", "B")
        builder.add_edge("B", "C")
        try:
            builder.add_edge("C", "A")
            pytest.fail("Expected an error for cycle")
        except ValueError as e:
            msg = str(e).lower()
            assert "cycle" in msg

    def test_not_identifiable_error_message(self):
        """NotIdentifiableError has clear context."""
        err = LutufiNotIdentifiableError(
            target="Y", intervention="X", hedge="Hedge structure found"
        )
        msg = str(err)
        assert "Y" in msg and "X" in msg
        assert "hedge" in msg.lower()

    def test_numerical_error_message(self):
        """NumericalError includes location."""
        err = LutufiNumericalError(location="factor.rs:120", message="NaN detected")
        msg = str(err)
        assert "factor.rs" in msg
        assert "NaN" in msg

    def test_resource_limit_error_message(self):
        """ResourceLimitError names the resource and limit."""
        err = LutufiResourceLimitError(
            resource="memory",
            limit="2GB",
            detail="Junction tree requires 4.2GB"
        )
        msg = str(err)
        assert "memory" in msg
        assert "2GB" in msg
        assert "4.2GB" in msg

    def test_serialization_error_message(self):
        """SerializationError includes path and reason."""
        err = LutufiSerializationError(
            path="model.lmf", reason="Invalid header magic bytes"
        )
        msg = str(err)
        assert "model.lmf" in msg
        assert "magic" in msg.lower()

    def test_missing_data_error_message(self):
        """MissingDataError has a descriptive message."""
        err = LutufiMissingDataError("MAR test failed: insufficient data")
        msg = str(err)
        assert "MAR" in msg

    def test_convergence_warning_message(self):
        """ConvergenceWarning mentions algorithm and iterations."""
        with pytest.warns(LutufiConvergenceWarning) as record:
            LutufiConvergenceWarning("LBP", 1000, 1e-3)
            warnings.warn(
                LutufiConvergenceWarning("LBP", 1000, 1e-3)
            )
        msg = str(record[0].message)
        assert "LBP" in msg
        assert "1000" in msg


# ─── Context Managers ─────────────────────────────────────────────────────────

class TestContextManagers:
    def test_edit_context_manager_modifies_model(self):
        """Changes made in edit() context persist after exit."""
        builder = BayesianNetwork.builder()
        builder.add_variable("A", domain=["0", "1"])
        builder.add_variable("B", domain=["0", "1"])
        model = builder.build()

        with model.edit() as m:
            m._model.add_edge("A", "B")

        assert ("A", "B") in model.edges()

    def test_edit_context_manager_rolls_back_on_error(self):
        """If an exception occurs in edit(), changes are rolled back."""
        builder = BayesianNetwork.builder()
        builder.add_variable("A", domain=["0", "1"])
        builder.add_variable("B", domain=["0", "1"])
        model = builder.build()
        initial_edges = model.edges()

        class SimulatedError(Exception):
            pass

        try:
            with model.edit() as m:
                m._model.add_edge("A", "B")
                raise SimulatedError("oops")
        except SimulatedError:
            pass

        assert model.edges() == initial_edges, (
            "Edges should be rolled back after exception"
        )


# ─── Lazy Evaluation ──────────────────────────────────────────────────────────

class TestLazyEvaluation:
    def test_lazy_query_returns_lazy_result(self):
        """query(lazy=True) returns a LazyQueryResult, not a QueryResult."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia"], lazy=True)
        assert isinstance(result, LazyQueryResult)

    def test_lazy_query_computes_on_access(self):
        """LazyQueryResult computes when distributions is accessed."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia"], lazy=True)
        dist = result.distributions
        assert "Asia" in dist
        np.testing.assert_allclose(dist["Asia"], [0.99, 0.01], atol=1e-10)

    def test_lazy_query_caches_result(self):
        """LazyQueryResult caches the computed result."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia"], lazy=True)
        dist1 = result.distributions
        dist2 = result.distributions
        assert dist1 is dist2  # same cached object

    def test_lazy_query_to_dataframe(self):
        """LazyQueryResult.to_dataframe() works."""
        import pandas as pd
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia"], lazy=True)
        df = result.to_dataframe()
        assert isinstance(df, pd.DataFrame)
        assert "probability" in df.columns

    def test_lazy_query_most_probable(self):
        """LazyQueryResult.most_probable() works."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(["Asia"], lazy=True)
        mp = result.most_probable()
        assert "Asia" in mp


# ─── Async Inference ──────────────────────────────────────────────────────────

@pytest.mark.asyncio
class TestAsyncInference:
    async def test_async_query_returns_query_result(self):
        """query_async returns a QueryResult."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = await engine.query_async(["Asia"])
        assert "Asia" in result.distributions

    async def test_async_query_correct_values(self):
        """Async query produces correct marginals."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = await engine.query_async(
            ["Asia"], evidence={"Smoking": "1"}
        )
        np.testing.assert_allclose(result["Asia"], [0.99, 0.01], atol=1e-10)


# ─── API Consistency ──────────────────────────────────────────────────────────

class TestAPIConsistency:
    def test_inference_engine_parameter_order(self):
        """InferenceEngine takes (model, algorithm) consistently."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="junction_tree")
        assert engine.algorithm == "junction_tree"

    def test_query_parameter_order(self):
        """query takes (variables, evidence, algorithm) consistently."""
        model = build_asia_network()
        engine = InferenceEngine(model, algorithm="variable_elimination")
        result = engine.query(
            variables=["Asia"],
            evidence={"Smoking": "1"}
        )
        assert result is not None

    def test_junction_tree_engine_treewidth_property(self):
        """JunctionTreeEngine exposes treewidth."""
        import warnings
        model = build_asia_network()
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            jt = InferenceEngine(model, algorithm="junction_tree")
            jt.query(["Asia"])
