"""
Markov Random Fields (MRFs): undirected graphical models.

Unlike a BayesianNetwork (a DAG with directed CPTs), a MarkovRandomField
is an undirected graph over random variables. This example builds a small
grid-shaped MRF (useful for image segmentation / spatial models, where
each node is a pixel/region and edges encode "neighboring values tend to
agree"), and shows the structural API: building, inspecting, converting
to/from NetworkX, and visualizing.

Note: as of this version, Lutufi's MRF support covers structure
(variables + undirected edges); potential-table storage and dedicated MRF
inference are not yet exposed through the Python API. For probabilistic
inference today, model your problem as a BayesianNetwork (see
01_quickstart.py and 02_inference_algorithms.py).
"""

import lutufi


def main():
    # A 2x2 grid of binary variables, each connected to its neighbors.
    #
    #   X00 -- X01
    #    |      |
    #   X10 -- X11
    mrf = lutufi.MarkovRandomField()
    for name in ["X00", "X01", "X10", "X11"]:
        mrf.add_variable(name, domain=["0", "1"])

    mrf.add_edge("X00", "X01")
    mrf.add_edge("X00", "X10")
    mrf.add_edge("X01", "X11")
    mrf.add_edge("X10", "X11")

    print("Nodes:", mrf.nodes())
    print("Edges:", mrf.edges())
    print("is_valid:", mrf.is_valid())

    # --- Convert to NetworkX for structural analysis ---
    G = mrf.to_networkx()
    import networkx as nx
    print("\nIs the grid bipartite?", nx.is_bipartite(G))
    print("Degree of each node:", dict(G.degree()))

    # --- Build an MRF back from a NetworkX graph ---
    rebuilt = lutufi.MarkovRandomField.from_networkx(G)
    print("\nRebuilt MRF nodes:", rebuilt.nodes())
    print("Rebuilt MRF edges:", rebuilt.edges())


if __name__ == "__main__":
    main()
