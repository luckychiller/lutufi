"""
Dynamic Bayesian Networks (DBNs): modeling processes over time.

A DynamicBayesianNetwork represents a two-slice ("2-TBN") template: a
prior network for time 0, plus a transition model relating time t to
time t+1. Variables at time t are conventionally suffixed "_t" and their
time-(t+1) counterparts "_t1".

This example builds a minimal weather model:

    Weather_t --> Weather_t1   (the weather tends to persist)
    Weather_t --> Umbrella_t   (intra-slice: umbrella depends on weather)

Note: as of this version, the Python API for DBNs covers structure
(variables, intra-slice edges, inter-slice edges); CPDs and rollout/
inference over time slices are not yet exposed. For inference today,
"unroll" the process manually as a regular BayesianNetwork (see
examples/epidemiology/sir_model.py for a worked example of this pattern).
"""

import lutufi


def main():
    dbn = lutufi.DynamicBayesianNetwork()

    dbn.add_variable("Weather_t", domain=["Sunny", "Rainy"])
    dbn.add_variable("Umbrella_t", domain=["Yes", "No"])
    dbn.add_variable("Weather_t1", domain=["Sunny", "Rainy"])
    dbn.add_variable("Umbrella_t1", domain=["Yes", "No"])

    # Intra-slice: within a single time step, weather influences umbrella use.
    dbn.add_intraslice_edge("Weather_t", "Umbrella_t")
    dbn.add_intraslice_edge("Weather_t1", "Umbrella_t1")

    # Inter-slice: weather at time t influences weather at time t+1.
    dbn.add_interslice_edge("Weather_t", "Weather_t1")

    print("Nodes:", dbn.nodes())
    print("Edges:", dbn.edges())
    print("is_valid:", dbn.is_valid())

    # --- Convert to NetworkX ---
    G = dbn.to_networkx()
    print("\nNetworkX nodes:", list(G.nodes()))
    print("NetworkX edges:", list(G.edges()))


if __name__ == "__main__":
    main()
