import pytest
from lutufi import BayesianNetwork


def build_asia_network() -> BayesianNetwork:
    """
    Construct the Asia network (Lauritzen & Spiegelhalter, 1988)
    using the Lutufi Python builder API.

    This is the canonical 8-node medical Bayesian network used as
    a benchmark throughout the PGM literature.
    """
    return (
        BayesianNetwork.builder()

        # ── Variables ───────────────────────────────────────────────────
        .add_variable("Asia",         domain=["F", "T"])
        .add_variable("Tuberculosis", domain=["F", "T"])
        .add_variable("Smoking",      domain=["F", "T"])
        .add_variable("LungCancer",   domain=["F", "T"])
        .add_variable("Bronchitis",   domain=["F", "T"])
        .add_variable("TbOrCa",       domain=["F", "T"])
        .add_variable("XRay",         domain=["F", "T"])
        .add_variable("Dyspnoea",     domain=["F", "T"])

        # ── Structure ───────────────────────────────────────────────────
        .add_edge("Asia",         "Tuberculosis")
        .add_edge("Smoking",      "LungCancer")
        .add_edge("Smoking",      "Bronchitis")
        .add_edge("Tuberculosis", "TbOrCa")
        .add_edge("LungCancer",   "TbOrCa")
        .add_edge("TbOrCa",       "XRay")
        .add_edge("TbOrCa",       "Dyspnoea")
        .add_edge("Bronchitis",   "Dyspnoea")

        # ── Parameters ──────────────────────────────────────────────────

        # P(Asia): prevalence of recent Asia travel
        .set_cpd("Asia",     [0.99, 0.01])

        # P(Smoking): smoking prevalence
        .set_cpd("Smoking",  [0.50, 0.50])

        # P(Tuberculosis | Asia)
        #              Asia=F  Asia=T
        # Tb=F          0.99    0.95
        # Tb=T          0.01    0.05
        .set_cpd("Tuberculosis", [[0.99, 0.95],
                                   [0.01, 0.05]])

        # P(LungCancer | Smoking)
        #              Smoking=F  Smoking=T
        # LC=F           0.99       0.90
        # LC=T           0.01       0.10
        .set_cpd("LungCancer", [[0.99, 0.90],
                                 [0.01, 0.10]])

        # P(Bronchitis | Smoking)
        #              Smoking=F  Smoking=T
        # Br=F           0.70       0.40
        # Br=T           0.30       0.60
        .set_cpd("Bronchitis", [[0.70, 0.40],
                                 [0.30, 0.60]])

        # P(TbOrCa | Tuberculosis, LungCancer) — deterministic OR
        #              Tb=F,LC=F  Tb=F,LC=T  Tb=T,LC=F  Tb=T,LC=T
        # TbOrCa=F       1.0        0.0        0.0        0.0
        # TbOrCa=T       0.0        1.0        1.0        1.0
        .set_cpd("TbOrCa", [[1.0, 0.0, 0.0, 0.0],
                             [0.0, 1.0, 1.0, 1.0]])

        # P(XRay | TbOrCa)
        #              TbOrCa=F  TbOrCa=T
        # XRay=F         0.95      0.02
        # XRay=T         0.05      0.98
        .set_cpd("XRay", [[0.95, 0.02],
                           [0.05, 0.98]])

        # P(Dyspnoea | TbOrCa, Bronchitis)
        #              TbOrCa=F,Br=F  TbOrCa=F,Br=T  TbOrCa=T,Br=F  TbOrCa=T,Br=T
        # Dy=F            0.90           0.20           0.30           0.10
        # Dy=T            0.10           0.80           0.70           0.90
        .set_cpd("Dyspnoea", [[0.90, 0.20, 0.30, 0.10],
                               [0.10, 0.80, 0.70, 0.90]])

        .build()
    )


# ── Tests ──────────────────────────────────────────────────────────────────────

def test_asia_network_builds_without_error():
    """The Asia network can be constructed without raising any exception."""
    model = build_asia_network()
    assert model is not None


def test_asia_network_has_correct_node_count():
    model = build_asia_network()
    assert len(model.nodes()) == 8, (
        f"Expected 8 nodes, got {len(model.nodes())}: {model.nodes()}"
    )


def test_asia_network_has_correct_edge_count():
    model = build_asia_network()
    assert len(model.edges()) == 8, (
        f"Expected 8 edges, got {len(model.edges())}: {model.edges()}"
    )


def test_asia_network_is_valid_for_inference():
    """All CPTs are set — the model should pass validation."""
    model = build_asia_network()
    result = model.validate()
    assert result.is_valid, (
        f"Asia network should be valid but got errors: {result.errors}"
    )


def test_asia_network_topological_order_respects_edges():
    """Every parent appears before its child in topological order."""
    model = build_asia_network()
    order = model.topological_order()
    edges = model.edges()

    for parent, child in edges:
        parent_idx = order.index(parent)
        child_idx = order.index(child)
        assert parent_idx < child_idx, (
            f"'{parent}' (pos {parent_idx}) should come before "
            f"'{child}' (pos {child_idx}) in topological order.\n"
            f"Full order: {order}"
        )


def test_asia_network_markov_blanket_dyspnoea():
    """Dyspnoea's Markov blanket contains TbOrCa and Bronchitis."""
    model = build_asia_network()
    blanket = set(model.markov_blanket("Dyspnoea"))
    assert "TbOrCa" in blanket, f"TbOrCa should be in blanket, got: {blanket}"
    assert "Bronchitis" in blanket, f"Bronchitis should be in blanket, got: {blanket}"
    assert len(blanket) == 2, f"Expected 2 nodes in blanket, got: {blanket}"


def test_cycle_detection_gives_informative_error():
    """Adding a cycle raises ValueError with a helpful message."""
    builder = BayesianNetwork.builder()
    builder.add_variable("A", domain=["F", "T"])
    builder.add_variable("B", domain=["F", "T"])
    builder.add_edge("A", "B")

    try:
        builder.add_edge("B", "A")  # creates cycle
        assert False, "Should have raised ValueError"
    except ValueError as e:
        msg = str(e)
        assert "cycle" in msg.lower(), f"Error should mention cycle: {msg}"


def test_cpt_normalization_error_is_informative():
    """A CPT that doesn't sum to 1 gives a clear error message."""
    builder = BayesianNetwork.builder()
    builder.add_variable("A", domain=["F", "T"])

    try:
        builder.set_cpd("A", [0.3, 0.8])  # sums to 1.1
        assert False, "Should have raised ValueError"
    except ValueError as e:
        msg = str(e)
        assert "sum" in msg.lower() or "1.1" in msg or "normaliz" in msg.lower(), (
            f"Error message should explain the normalization issue: {msg}"
        )


def test_variable_not_found_error_lists_available():
    """Accessing a non-existent variable lists available variable names."""
    model = build_asia_network()
    try:
        model.markov_blanket("NonExistentVariable")
        assert False, "Should have raised ValueError"
    except ValueError as e:
        msg = str(e)
        # The error should name some of the actual variables
        assert any(v in msg for v in ["Asia", "Smoking", "XRay"]), (
            f"Error should list available variables: {msg}"
        )


def test_networkx_roundtrip():
    """NetworkX export and import preserves graph structure."""
    pytest.importorskip("networkx")

    original = build_asia_network()
    G = original.to_networkx()
    recovered = BayesianNetwork.from_networkx(G)

    assert set(original.nodes()) == set(recovered.nodes()), (
        f"Node sets differ after NetworkX round-trip.\n"
        f"Original: {sorted(original.nodes())}\n"
        f"Recovered: {sorted(recovered.nodes())}"
    )
    assert set(original.edges()) == set(recovered.edges()), (
        f"Edge sets differ after NetworkX round-trip.\n"
        f"Original: {sorted(original.edges())}\n"
        f"Recovered: {sorted(recovered.edges())}"
    )


def test_model_repr_is_informative():
    """__repr__ contains enough information to understand the model at a glance."""
    model = build_asia_network()
    r = repr(model)
    assert "8" in r, f"repr should mention node count: {r}"
    assert "valid" in r.lower(), f"repr should mention validity: {r}"


def test_model_str_is_human_readable():
    """__str__ produces a multi-line human-readable summary."""
    model = build_asia_network()
    s = str(model)
    # Should be multi-line
    assert "\n" in s, "str output should be multi-line"
    # Should mention all variables
    for var in ["Asia", "Smoking", "Dyspnoea"]:
        assert var in s, f"str should mention variable '{var}'"


if __name__ == "__main__":
    # Allow running directly as a script for quick manual testing
    print("Building Asia network...")
    model = build_asia_network()
    print(model)
    print(f"\nTopological order: {model.topological_order()}")
    print(f"Markov blanket of Dyspnoea: {model.markov_blanket('Dyspnoea')}")
    print("\n✓ All manual checks passed.")
