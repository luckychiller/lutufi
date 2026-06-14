"""
NetworkX interoperability and graph file I/O.

Lutufi models can round-trip through NetworkX, which unlocks the wider
graph-analysis ecosystem (centrality measures, layouts, custom plotting,
etc.) as well as file formats like GraphML, GEXF, GML, and edge lists.

Note: NetworkX graphs have no concept of conditional probability tables,
so a round trip through NetworkX/graph files preserves *structure*
(nodes, edges, domains) but not learned CPDs. If you need full fidelity
(structure + CPDs), keep working with the BayesianNetwork object directly.
"""

import tempfile
from pathlib import Path

import lutufi
from lutufi.io import read_graph, write_graph


def main():
    bn = (
        lutufi.BayesianNetwork.builder()
        .add_variable("A", domain=["0", "1"])
        .add_variable("B", domain=["0", "1"])
        .add_variable("C", domain=["low", "medium", "high"])
        .add_edge("A", "B")
        .add_edge("A", "C")
        .add_edge("B", "C")
        .set_cpd("A", [0.4, 0.6])
        .set_cpd("B", [[0.7, 0.2], [0.3, 0.8]])
        .set_cpd("C", [
            [0.1, 0.2, 0.3, 0.4],  # C=low
            [0.3, 0.3, 0.3, 0.3],  # C=medium
            [0.6, 0.5, 0.4, 0.3],  # C=high
        ])
        .build()
    )

    # --- Convert to NetworkX for structural analysis ---
    G = bn.to_networkx()
    print("Nodes:", list(G.nodes()))
    print("Edges:", list(G.edges()))

    import networkx as nx
    print("Topological generations:", [list(gen) for gen in nx.topological_generations(G)])
    print("In-degree of C:", G.in_degree("C"))

    # --- Round-trip through a graph file (GraphML) ---
    with tempfile.TemporaryDirectory() as tmpdir:
        path = Path(tmpdir) / "network.graphml"
        write_graph(bn, path)
        print(f"\nWrote network structure to {path.name}")

        G2 = read_graph(path)
        print("Reloaded nodes:", sorted(G2.nodes()))
        print("Reloaded edges:", sorted(G2.edges()))

        # Rebuild a Lutufi model from the reloaded graph. Since GraphML
        # doesn't carry CPDs, from_networkx() fills in uniform CPDs for any
        # variable without an explicit entry in `cpds` so the model is
        # immediately valid for inference.
        domains = {"A": ["0", "1"], "B": ["0", "1"], "C": ["low", "medium", "high"]}
        rebuilt = lutufi.BayesianNetwork.from_networkx(G2, state_names=domains)
        print("\nRebuilt model is valid:", rebuilt.is_valid())
        print("Rebuilt P(A) (uniform, since CPDs aren't stored in GraphML):", rebuilt.cpd("A"))


if __name__ == "__main__":
    main()
