"""
Quickstart: the classic "Sprinkler" Bayesian network.

This example builds the textbook Cloudy / Sprinkler / Rain / WetGrass
network, runs basic inference, and demonstrates the canonical "explaining
away" pattern: observing WetGrass=True raises both P(Rain) and
P(Sprinkler), but observing that it also Rained lowers the probability
that the Sprinkler was on (since Rain alone explains the wet grass).

Network structure:

    Cloudy --> Sprinkler --> WetGrass
      |                          ^
      +----------> Rain ---------+

CPT convention used throughout Lutufi:
  - Rows correspond to the *child* variable's states (in domain order).
  - Columns correspond to parent configurations. For a variable with no
    parents, the CPT is a flat list (one root "configuration").
  - For variables with multiple parents, parent configurations are the
    cartesian product of parent domains, with the *first* parent (the
    one whose edge was added first) varying slowest.
  - Every column must sum to 1.0 (it is a probability distribution over
    the child's states given that parent configuration).
"""

import lutufi
from lutufi.inference import InferenceEngine


def build_sprinkler_network() -> lutufi.BayesianNetwork:
    builder = (
        lutufi.BayesianNetwork.builder()
        .add_variable("Cloudy", domain=["T", "F"])
        .add_variable("Sprinkler", domain=["T", "F"])
        .add_variable("Rain", domain=["T", "F"])
        .add_variable("WetGrass", domain=["T", "F"])
        .add_edge("Cloudy", "Sprinkler")
        .add_edge("Cloudy", "Rain")
        .add_edge("Sprinkler", "WetGrass")
        .add_edge("Rain", "WetGrass")
    )

    # P(Cloudy)
    builder.set_cpd("Cloudy", [0.5, 0.5])

    # P(Sprinkler | Cloudy): columns are Cloudy=T, Cloudy=F
    builder.set_cpd("Sprinkler", [
        [0.1, 0.5],  # Sprinkler=T
        [0.9, 0.5],  # Sprinkler=F
    ])

    # P(Rain | Cloudy): columns are Cloudy=T, Cloudy=F
    builder.set_cpd("Rain", [
        [0.8, 0.2],  # Rain=T
        [0.2, 0.8],  # Rain=F
    ])

    # P(WetGrass | Sprinkler, Rain): columns are
    # (S=T,R=T), (S=T,R=F), (S=F,R=T), (S=F,R=F)
    builder.set_cpd("WetGrass", [
        [0.99, 0.90, 0.90, 0.00],  # WetGrass=T
        [0.01, 0.10, 0.10, 1.00],  # WetGrass=F
    ])

    return builder.build()


def main():
    bn = build_sprinkler_network()
    print(bn)
    print("Topological order:", bn.topological_order())
    print()

    engine = InferenceEngine(bn)

    # Prior probability of wet grass.
    prior = engine.query(["WetGrass"])
    print("P(WetGrass)            =", dict(zip(bn.get_states("WetGrass"), prior["WetGrass"])))

    # Observing wet grass raises belief in both Rain and Sprinkler.
    posterior = engine.query(["Rain", "Sprinkler"], evidence={"WetGrass": "T"})
    print("P(Rain | WetGrass=T)    =", dict(zip(bn.get_states("Rain"), posterior["Rain"])))
    print("P(Sprinkler | WetGrass=T) =", dict(zip(bn.get_states("Sprinkler"), posterior["Sprinkler"])))

    # "Explaining away": once we also know it rained, the Sprinkler is
    # less likely to have been the cause of the wet grass.
    explained = engine.query(["Sprinkler"], evidence={"WetGrass": "T", "Rain": "T"})
    print("P(Sprinkler | WetGrass=T, Rain=T) =",
          dict(zip(bn.get_states("Sprinkler"), explained["Sprinkler"])))

    # most_probable() gives a quick MAP-style summary of a query result.
    print("\nMost probable explanation given WetGrass=T:")
    full = engine.query(["Cloudy", "Sprinkler", "Rain"], evidence={"WetGrass": "T"})
    print(full.most_probable())


if __name__ == "__main__":
    main()
