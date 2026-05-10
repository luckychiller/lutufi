"""Installation verification tests.

These tests verify basic functionality works after installation.
They should run in a clean environment (not necessarily from source).
"""

import importlib


class TestPackageImport:
    """Verify the package imports correctly."""

    def test_import_lutufi(self):
        """The lutufi package imports without error."""
        import lutufi
        assert lutufi is not None

    def test_version_string(self):
        """__version__ is a non-empty string."""
        import lutufi
        assert isinstance(lutufi.__version__, str)
        assert len(lutufi.__version__) > 0

    def test_module_attributes(self):
        """All expected top-level attrs are accessible."""
        import lutufi
        for attr in ["__version__", "BayesianNetwork", "MarkovRandomField",
                      "DynamicBayesianNetwork", "InferenceEngine",
                      "QueryResult", "ParameterEstimator", "StructureLearner",
                      "read_graph", "write_graph"]:
            assert hasattr(lutufi, attr), f"Missing top-level attr: {attr}"

    def test_submodule_imports(self):
        """All submodules import correctly."""
        for module_name in ["lutufi.models", "lutufi.inference",
                            "lutufi.learning", "lutufi.io"]:
            mod = importlib.import_module(module_name)
            assert mod is not None


class TestBasicFunctionality:
    """Minimal functional tests that don't require compiled Rust extension."""

    def test_bayesian_network_builder_api(self):
        """The builder API constructs a network."""
        from lutufi import BayesianNetwork
        bn = (
            BayesianNetwork.builder()
            .add_variable("A", domain=["0", "1"])
            .add_variable("B", domain=["0", "1"])
            .add_edge("A", "B")
            .set_cpd("A", [0.5, 0.5])
            .set_cpd("B", [[0.9, 0.1], [0.2, 0.8]])
            .build()
        )
        assert bn is not None
        assert len(bn.nodes()) == 2
        assert len(bn.edges()) == 1

    def test_model_inspection(self):
        """Model inspection methods return correct values."""
        from lutufi import BayesianNetwork
        bn = (
            BayesianNetwork.builder()
            .add_variable("X", domain=["0", "1", "2"])
            .add_variable("Y", domain=["0", "1"])
            .add_edge("X", "Y")
            .set_cpd("X", [0.3, 0.4, 0.3])
            .set_cpd("Y", [[0.9, 0.1, 0.2], [0.1, 0.9, 0.8]])
            .build()
        )
        assert len(bn.nodes()) == 2
        assert bn.get_states("X") == ["0", "1", "2"]
        assert bn.get_states("Y") == ["0", "1"]

    def test_model_validation(self):
        """A properly built model passes validation."""
        from lutufi import BayesianNetwork
        bn = (
            BayesianNetwork.builder()
            .add_variable("A", domain=["0", "1"])
            .add_variable("B", domain=["0", "1"])
            .add_edge("A", "B")
            .set_cpd("A", [0.5, 0.5])
            .set_cpd("B", [[0.9, 0.1], [0.2, 0.8]])
            .build()
        )
        assert bn.is_valid()

    def test_model_str_repr(self):
        """__str__ and __repr__ are informative."""
        from lutufi import BayesianNetwork
        bn = (
            BayesianNetwork.builder()
            .add_variable("A", domain=["0", "1"])
            .add_variable("B", domain=["0", "1"])
            .add_edge("A", "B")
            .set_cpd("A", [0.5, 0.5])
            .set_cpd("B", [[0.9, 0.1], [0.2, 0.8]])
            .build()
        )
        assert "BayesianNetwork" in repr(bn)
        assert "A" in str(bn)

    def test_markov_random_field_creation(self):
        """MarkovRandomField can be created and inspected."""
        from lutufi import MarkovRandomField
        mrf = MarkovRandomField(name="test")
        mrf.add_variable("A", ["0", "1"])
        mrf.add_variable("B", ["0", "1"])
        mrf.add_edge("A", "B")
        assert len(mrf.nodes()) == 2
        assert len(mrf.edges()) == 1

    def test_dynamic_bayesian_network_creation(self):
        """DynamicBayesianNetwork can be created."""
        from lutufi import DynamicBayesianNetwork
        dbn = DynamicBayesianNetwork(name="test_dbn")
        dbn.add_variable("X", ["0", "1"])
        dbn.add_variable("Y", ["0", "1"])
        dbn.add_intraslice_edge("X", "Y")
        dbn.add_interslice_edge("X", "X")
        assert dbn is not None

    def test_error_classes_importable(self):
        """All error classes are accessible."""
        from lutufi import (
            LutufiError, LutufiValidationError, LutufiNonCausalError,
            LutufiCyclicGraphError, LutufiNotIdentifiableError,
            LutufiNumericalError, LutufiResourceLimitError,
            LutufiSerializationError, LutufiMissingDataError,
            LutufiConvergenceWarning, LutufiHighTreewidthWarning,
        )
        assert issubclass(LutufiError, Exception)
        assert issubclass(LutufiCyclicGraphError, LutufiError)
        assert issubclass(LutufiConvergenceWarning, UserWarning)

    def test_inference_classes_importable(self):
        """All inference classes are accessible."""
        from lutufi import (
            InferenceEngine, QueryResult, LazyQueryResult,
            JunctionTreeEngine, BeliefPropagation,
            LoopyBeliefPropagation, GibbsSampler, VariationalInference,
        )
        assert InferenceEngine is not None

    def test_learning_classes_importable(self):
        """All learning classes are accessible."""
        from lutufi import ParameterEstimator, StructureLearner, fit, learn_structure
        assert ParameterEstimator is not None

    def test_io_functions_importable(self):
        """All IO functions are accessible."""
        from lutufi import read_graph, write_graph
        from lutufi.io import GraphReader, GraphWriter, GraphIOError
        assert read_graph is not None
