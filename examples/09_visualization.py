"""
Visualization: plotting network structure, CPTs, and query results.

Lutufi provides Matplotlib-based plotting helpers:

  - model.plot(): draw the network's graph structure (via NetworkX).
  - model.plot_cpd(variable): draw a heatmap of a variable's CPT.
  - query_result.plot(): bar charts of marginal distributions.

This example uses Matplotlib's non-interactive "Agg" backend and saves
figures to an `output/` directory next to this script, so it runs
headlessly (e.g. in CI or over SSH). To view the plots interactively
instead, remove the `matplotlib.use("Agg")` call and replace
`fig.savefig(...)` with `plt.show()`.
"""

from pathlib import Path

import matplotlib
matplotlib.use("Agg")

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
    out_dir = Path(__file__).parent / "output"
    out_dir.mkdir(exist_ok=True)

    # --- Network structure ---
    fig, ax = bn.plot()
    fig.savefig(out_dir / "network_structure.png", dpi=100)
    print(f"Saved {out_dir / 'network_structure.png'}")

    # --- CPT heatmap ---
    fig, ax = bn.plot_cpd("WetGrass")
    fig.savefig(out_dir / "wetgrass_cpd.png", dpi=100)
    print(f"Saved {out_dir / 'wetgrass_cpd.png'}")

    # --- Query result bar chart ---
    engine = InferenceEngine(bn)
    result = engine.query(["Rain", "Sprinkler"], evidence={"WetGrass": "T"})
    fig, axes = result.plot()
    fig.savefig(out_dir / "posterior_given_wetgrass.png", dpi=100)
    print(f"Saved {out_dir / 'posterior_given_wetgrass.png'}")


if __name__ == "__main__":
    main()
