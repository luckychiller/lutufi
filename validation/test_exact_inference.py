import pytest
import numpy as np
from lutufi import BayesianNetwork, InferenceEngine, JunctionTreeEngine, QueryResult, LutufiHighTreewidthWarning
from validation.test_asia_construction import build_asia_network


def build_simple_or_network():
    builder = BayesianNetwork.builder()
    builder.add_variable("A", domain=["F", "T"])
    builder.add_variable("B", domain=["F", "T"])
    builder.add_variable("C", domain=["F", "T"])

    builder.set_cpd("A", [0.6, 0.4])
    builder.set_cpd("B", [0.7, 0.3])
    builder.add_edge("A", "C")
    builder.add_edge("B", "C")
    builder.set_cpd("C", [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 1.0, 1.0]])
    return builder.build()


def test_query_result_formats_dataframes_and_dicts():
    model = build_simple_or_network()
    engine = InferenceEngine(model)
    result = engine.query(variables=["C"])
    assert isinstance(result, QueryResult)
    # C = A OR B, with P(A=T)=0.4, P(B=T)=0.3, so P(C=F) = P(A=F)*P(B=F) = 0.6*0.7 = 0.42.
    assert np.allclose(result["C"], [0.42, 0.58], atol=1e-10)
    df = result.to_dataframe()
    assert "probability" in df.columns
    data = result.to_dict()
    assert data["variables"] == ["C"]
    np.testing.assert_allclose(data["distributions"]["C"], [0.42, 0.58], atol=1e-10)


def test_junction_tree_matches_variable_elimination_for_asia():
    model = build_asia_network()
    ve = InferenceEngine(model, algorithm="variable_elimination")
    jt = JunctionTreeEngine(model)

    for variable in ["Asia", "Tuberculosis", "Smoking", "LungCancer", "Bronchitis"]:
        ve_res = ve.query(variables=[variable])
        jt_res = jt.query(variables=[variable])
        assert isinstance(jt_res, QueryResult)
        np.testing.assert_allclose(jt_res[variable], ve_res[variable], atol=1e-10)

    evidence = {"Asia": "1"}
    ve_res = ve.query(variables=["Tuberculosis"], evidence=evidence)
    jt_res = jt.query(variables=["Tuberculosis"], evidence=evidence)
    np.testing.assert_allclose(jt_res["Tuberculosis"], ve_res["Tuberculosis"], atol=1e-10)


def test_treewidth_warning_fires_for_strict_threshold():
    model = build_asia_network()
    with pytest.warns(LutufiHighTreewidthWarning):
        JunctionTreeEngine(model, treewidth_threshold=0)


def test_map_query_returns_map_assignment_with_correct_state_labels():
    model = build_simple_or_network()
    engine = InferenceEngine(model)
    result = engine.map_query(["A", "B"], evidence={"C": "1"})
    assert isinstance(result, QueryResult)
    # argmax_{A,B} P(A,B|C=T): joint(A,B,C=T) = P(A)P(B) except (F,F) which gives C=F.
    # (F,T)=0.18, (T,F)=0.28, (T,T)=0.12 -> (T,F) is the MAP assignment.
    assert result.most_probable() == {"A": "T", "B": "F"}


def test_mpe_query_returns_mpe_assignment():
    model = build_simple_or_network()
    engine = InferenceEngine(model)
    result = engine.mpe_query(evidence={"C": "0"})
    assert result.most_probable() == {"A": "F", "B": "F", "C": "F"}


def test_d_separation_chain_fork_collider():
    # Chain structure A -> B -> C
    chain = BayesianNetwork.builder()
    chain.add_variable("A", domain=["F", "T"])
    chain.add_variable("B", domain=["F", "T"])
    chain.add_variable("C", domain=["F", "T"])
    chain.add_edge("A", "B")
    chain.add_edge("B", "C")
    chain.set_cpd("A", [0.5, 0.5])
    chain.set_cpd("B", [[0.5, 0.5], [0.5, 0.5]])
    chain.set_cpd("C", [[0.5, 0.5], [0.5, 0.5]])
    net = chain.build()

    assert net.d_separated("A", "C", ["B"])
    assert not net.d_separated("A", "C", [])

    # Fork structure B -> A, B -> C
    fork = BayesianNetwork.builder()
    fork.add_variable("A", domain=["F", "T"])
    fork.add_variable("B", domain=["F", "T"])
    fork.add_variable("C", domain=["F", "T"])
    fork.add_edge("B", "A")
    fork.add_edge("B", "C")
    fork.set_cpd("B", [0.5, 0.5])
    fork.set_cpd("A", [[0.5, 0.5], [0.5, 0.5]])
    fork.set_cpd("C", [[0.5, 0.5], [0.5, 0.5]])
    net = fork.build()

    assert net.d_separated("A", "C", ["B"])
    assert not net.d_separated("A", "C", [])

    # Collider structure A -> B <- C
    collider = BayesianNetwork.builder()
    collider.add_variable("A", domain=["F", "T"])
    collider.add_variable("B", domain=["F", "T"])
    collider.add_variable("C", domain=["F", "T"])
    collider.add_edge("A", "B")
    collider.add_edge("C", "B")
    collider.set_cpd("A", [0.5, 0.5])
    collider.set_cpd("C", [0.5, 0.5])
    collider.set_cpd("B", [[0.5, 0.5, 0.5, 0.5], [0.5, 0.5, 0.5, 0.5]])
    net = collider.build()

    assert net.d_separated("A", "C", [])
    assert not net.d_separated("A", "C", ["B"])
