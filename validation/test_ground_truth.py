"""Ground-truth validation tests.

These tests compute exact probabilities for small, hand-specified networks
using a plain-Python brute-force enumeration (independent of the Rust
implementation), and check that every inference algorithm exposed by
``InferenceEngine`` (variable elimination, junction tree, loopy belief
propagation, MCMC/Gibbs sampling, and variational inference) agrees with
those ground-truth numbers to an appropriate tolerance.

Exact algorithms (variable elimination, junction tree) must match the
brute-force enumeration to numerical precision. Loopy belief propagation is
exact on tree-structured networks, so it is also checked to high precision
on the chain network. MCMC and variational inference are approximate, so
they are checked with looser tolerances (and, for variational inference,
only sanity-checked as a valid probability distribution).
"""

import itertools

import numpy as np
import pytest

from lutufi import BayesianNetwork, InferenceEngine


# ─── Brute-force exact inference (independent of lutufi) ──────────────────────

def brute_force_marginals(variables, parents, cpds, evidence=None):
    """Compute exact marginals for a binary Bayesian network by enumeration.

    Args:
        variables: Variable names in topological order. All variables are
            assumed binary with states 0 and 1.
        parents: Mapping from variable name to a list of parent names, in
            the same order used by ``cpds``.
        cpds: Mapping from variable name to a function that takes a tuple of
            0/1 parent values (in ``parents[var]`` order) and returns
            ``[P(var=0 | parents), P(var=1 | parents)]``.
        evidence: Optional mapping from variable name to an observed 0/1
            value.

    Returns:
        Mapping from each non-evidence variable to its marginal
        ``[P(var=0 | evidence), P(var=1 | evidence)]``.
    """
    evidence = evidence or {}
    joint = {}
    total = 0.0
    for bits in itertools.product((0, 1), repeat=len(variables)):
        assignment = dict(zip(variables, bits))
        if any(assignment[k] != v for k, v in evidence.items()):
            continue
        p = 1.0
        for var in variables:
            parent_vals = tuple(assignment[parent] for parent in parents[var])
            p *= cpds[var](parent_vals)[assignment[var]]
        joint[bits] = p
        total += p

    marginals = {}
    for i, var in enumerate(variables):
        if var in evidence:
            continue
        probs = [0.0, 0.0]
        for bits, p in joint.items():
            probs[bits[i]] += p
        marginals[var] = [p / total for p in probs]
    return marginals


# ─── Sprinkler network (Russell & Norvig AIMA, "explaining away") ──────────────
#
#   Cloudy --> Sprinkler --> WetGrass
#     |                          ^
#     `------> Rain -------------'

SPRINKLER_VARIABLES = ["Cloudy", "Sprinkler", "Rain", "WetGrass"]
SPRINKLER_PARENTS = {
    "Cloudy": [],
    "Sprinkler": ["Cloudy"],
    "Rain": ["Cloudy"],
    "WetGrass": ["Sprinkler", "Rain"],
}
SPRINKLER_CPDS = {
    "Cloudy": lambda _: [0.5, 0.5],
    "Sprinkler": lambda c: [0.5, 0.5] if c[0] == 0 else [0.9, 0.1],
    "Rain": lambda c: [0.8, 0.2] if c[0] == 0 else [0.2, 0.8],
    "WetGrass": lambda sr: {
        (0, 0): [1.0, 0.0],
        (0, 1): [0.1, 0.9],
        (1, 0): [0.1, 0.9],
        (1, 1): [0.01, 0.99],
    }[sr],
}


def build_sprinkler_network() -> BayesianNetwork:
    builder = BayesianNetwork.builder()
    for name in SPRINKLER_VARIABLES:
        builder.add_variable(name, domain=["F", "T"])

    builder.set_cpd("Cloudy", [0.5, 0.5])

    builder.add_edge("Cloudy", "Sprinkler")
    builder.set_cpd("Sprinkler", [[0.5, 0.9],
                                   [0.5, 0.1]])

    builder.add_edge("Cloudy", "Rain")
    builder.set_cpd("Rain", [[0.8, 0.2],
                              [0.2, 0.8]])

    builder.add_edge("Sprinkler", "WetGrass")
    builder.add_edge("Rain", "WetGrass")
    builder.set_cpd("WetGrass", [[1.0, 0.1, 0.1, 0.01],
                                  [0.0, 0.9, 0.9, 0.99]])

    return builder.build()


@pytest.mark.parametrize("algorithm", ["variable_elimination", "junction_tree"])
def test_sprinkler_prior_marginals_match_brute_force(algorithm):
    """Exact algorithms reproduce brute-force priors for every variable."""
    model = build_sprinkler_network()
    engine = InferenceEngine(model, algorithm=algorithm)
    ground_truth = brute_force_marginals(SPRINKLER_VARIABLES, SPRINKLER_PARENTS, SPRINKLER_CPDS)

    for var in SPRINKLER_VARIABLES:
        result = engine.query([var])
        np.testing.assert_allclose(result[var], ground_truth[var], atol=1e-9)


@pytest.mark.parametrize("algorithm", ["variable_elimination", "junction_tree"])
def test_sprinkler_explaining_away_matches_brute_force(algorithm):
    """Observing WetGrass=T should raise P(Rain=T) more than P(Sprinkler=T)
    (the classic "explaining away" pattern), and both must match the
    brute-force conditional marginals exactly.
    """
    model = build_sprinkler_network()
    engine = InferenceEngine(model, algorithm=algorithm)
    evidence = {"WetGrass": "1"}
    ground_truth = brute_force_marginals(
        SPRINKLER_VARIABLES, SPRINKLER_PARENTS, SPRINKLER_CPDS, evidence={"WetGrass": 1}
    )

    for var in ("Cloudy", "Sprinkler", "Rain"):
        result = engine.query([var], evidence=evidence)
        np.testing.assert_allclose(result[var], ground_truth[var], atol=1e-9)

    # Explaining away: rain explains the wet grass better than the sprinkler does.
    assert ground_truth["Rain"][1] > ground_truth["Sprinkler"][1]


# ─── Chain network (tree-structured, exact for LBP) ────────────────────────────
#
#   A --> B --> C

CHAIN_VARIABLES = ["A", "B", "C"]
CHAIN_PARENTS = {"A": [], "B": ["A"], "C": ["B"]}
CHAIN_CPDS = {
    "A": lambda _: [0.3, 0.7],
    "B": lambda a: [0.8, 0.2] if a[0] == 0 else [0.4, 0.6],
    "C": lambda b: [0.9, 0.1] if b[0] == 0 else [0.1, 0.9],
}


def build_chain_network() -> BayesianNetwork:
    builder = BayesianNetwork.builder()
    for name in CHAIN_VARIABLES:
        builder.add_variable(name, domain=["0", "1"])

    builder.set_cpd("A", [0.3, 0.7])

    builder.add_edge("A", "B")
    builder.set_cpd("B", [[0.8, 0.4],
                           [0.2, 0.6]])

    builder.add_edge("B", "C")
    builder.set_cpd("C", [[0.9, 0.1],
                           [0.1, 0.9]])

    return builder.build()


def test_chain_lbp_matches_brute_force_exactly():
    """Loopy belief propagation is exact on a tree, so it should match the
    brute-force marginals to high precision, both with and without evidence.
    """
    model = build_chain_network()
    engine = InferenceEngine(model, algorithm="lbp")

    ground_truth = brute_force_marginals(CHAIN_VARIABLES, CHAIN_PARENTS, CHAIN_CPDS)
    for var in CHAIN_VARIABLES:
        result = engine.query([var])
        np.testing.assert_allclose(result[var], ground_truth[var], atol=1e-6)

    evidence = {"A": "1"}
    ground_truth_ev = brute_force_marginals(CHAIN_VARIABLES, CHAIN_PARENTS, CHAIN_CPDS, evidence={"A": 1})
    for var in ("B", "C"):
        result = engine.query([var], evidence=evidence)
        np.testing.assert_allclose(result[var], ground_truth_ev[var], atol=1e-6)


def test_chain_mcmc_approximates_brute_force():
    """Gibbs sampling should approximate the brute-force marginals reasonably
    well given enough samples.
    """
    model = build_chain_network()
    engine = InferenceEngine(model, algorithm="mcmc")
    ground_truth = brute_force_marginals(CHAIN_VARIABLES, CHAIN_PARENTS, CHAIN_CPDS)

    for var in CHAIN_VARIABLES:
        result = engine.query([var], n_samples=4000, burn_in=500)
        np.testing.assert_allclose(result[var], ground_truth[var], atol=0.08)


def test_chain_variational_returns_valid_distribution():
    """Mean-field variational inference is approximate and not guaranteed to
    match exact marginals, but it must always return valid (normalized,
    non-negative) probability distributions and report convergence
    diagnostics.
    """
    model = build_chain_network()
    engine = InferenceEngine(model, algorithm="variational")

    for var in CHAIN_VARIABLES:
        result = engine.query([var])
        probs = np.asarray(result[var])
        assert probs.shape == (2,)
        assert np.all(probs >= 0.0)
        np.testing.assert_allclose(probs.sum(), 1.0, atol=1e-6)
        assert "converged" in result.diagnostics
        assert "elbo" in result.diagnostics
