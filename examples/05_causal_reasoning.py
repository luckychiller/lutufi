"""
Causal/structural reasoning: d-separation, Markov blankets, and marking
a model as causal.

A Bayesian network's graph encodes conditional independence relationships
via d-separation, regardless of whether the edges represent genuine causal
mechanisms. Lutufi exposes these structural queries directly:

  - d_separated(a, b, given): test conditional independence implied by the graph.
  - markov_blanket(v): the minimal set of variables that render v
    conditionally independent of everything else.
  - mark_as_causal(): an explicit opt-in that documents your assumption
    that the edges represent causal mechanisms (not just correlations).
    This is required before calling causal operations such as do(),
    identify(), or counterfactual() queries.

We reuse the Sprinkler network from 01_quickstart.py.
"""

import lutufi


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
    builder.set_cpd("Cloudy", [0.5, 0.5])
    builder.set_cpd("Sprinkler", [[0.1, 0.5], [0.9, 0.5]])
    builder.set_cpd("Rain", [[0.8, 0.2], [0.2, 0.8]])
    builder.set_cpd("WetGrass", [
        [0.99, 0.90, 0.90, 0.00],
        [0.01, 0.10, 0.10, 1.00],
    ])
    return builder.build()


def main():
    bn = build_sprinkler_network()

    # --- d-separation ---
    # Sprinkler and Rain are marginally dependent (both caused by Cloudy)...
    print("d_separated(Sprinkler, Rain, given=[])      =",
          bn.d_separated("Sprinkler", "Rain", given=[]))
    # ...but become independent once Cloudy is observed.
    print("d_separated(Sprinkler, Rain, given=[Cloudy]) =",
          bn.d_separated("Sprinkler", "Rain", given=["Cloudy"]))

    # Cloudy and WetGrass are dependent (Cloudy -> {Sprinkler,Rain} -> WetGrass)...
    print("d_separated(Cloudy, WetGrass, given=[])      =",
          bn.d_separated("Cloudy", "WetGrass", given=[]))
    # ...but conditioning on both intermediate variables blocks every path.
    print("d_separated(Cloudy, WetGrass, given=[Sprinkler, Rain]) =",
          bn.d_separated("Cloudy", "WetGrass", given=["Sprinkler", "Rain"]))

    # --- Markov blanket ---
    # The Markov blanket of Sprinkler is everything needed to predict it
    # without consulting the rest of the network: its parent (Cloudy),
    # its child (WetGrass), and WetGrass's other parent (Rain).
    print("\nmarkov_blanket(Sprinkler) =", sorted(bn.markov_blanket("Sprinkler")))

    # --- Marking a model as causal ---
    # is_causal() is False until you explicitly opt in. This is a safeguard:
    # only mark a model causal if its edges represent mechanisms you believe
    # in (from domain knowledge or a causal discovery algorithm), not mere
    # statistical association.
    print("\nis_causal() before marking:", bn.is_causal)
    bn.mark_as_causal()
    print("is_causal() after marking: ", bn.is_causal)


if __name__ == "__main__":
    main()
