"""
Learning: parameter estimation and structure learning from data.

This example generates synthetic observations from a known "ground truth"
network, then shows three ways to recover a model from data:

  1. BayesianNetwork.from_dataframe(): given a known structure, learn CPTs
     directly from a DataFrame in one call.
  2. ParameterEstimator / fit(): learn CPTs for a model whose structure
     (variables + edges) is already defined.
  3. StructureLearner / learn_structure(): learn the graph structure itself
     from data (via hill-climbing), then fit parameters on top of it.
"""

import numpy as np
import pandas as pd

import lutufi
from lutufi.inference import InferenceEngine
from lutufi.learning import fit, learn_structure


def generate_data(n_samples: int = 2000, seed: int = 0) -> pd.DataFrame:
    """Sample from a small ground-truth network: Rain -> Umbrella, Rain -> Traffic."""
    rng = np.random.default_rng(seed)

    rain = rng.binomial(1, 0.3, n_samples)
    # Umbrella is much more likely when it rains.
    p_umbrella = np.where(rain == 1, 0.9, 0.1)
    umbrella = rng.binomial(1, p_umbrella)
    # Traffic is somewhat more likely when it rains.
    p_traffic = np.where(rain == 1, 0.6, 0.2)
    traffic = rng.binomial(1, p_traffic)

    df = pd.DataFrame({
        "Rain": rain.astype(str),
        "Umbrella": umbrella.astype(str),
        "Traffic": traffic.astype(str),
    })
    return df


def main():
    data = generate_data()
    print(f"Generated {len(data)} samples")
    print(data.head(), "\n")

    structure = [("Rain", "Umbrella"), ("Rain", "Traffic")]

    # --- 1. Learn CPTs given a known structure, in one call ---
    bn = lutufi.BayesianNetwork.from_dataframe(data, structure=structure, estimator="bayesian")
    print("from_dataframe(): learned P(Umbrella | Rain):")
    # cpd() returns a flat array ordered (parent_config, state); reshape for
    # a readable table where rows are Rain configurations (0, 1) and
    # columns are Umbrella states (0, 1).
    n_states = len(bn.get_states("Umbrella"))
    print(bn.cpd("Umbrella").reshape(-1, n_states))

    engine = InferenceEngine(bn)
    result = engine.query(["Umbrella"], evidence={"Rain": "1"})
    print("P(Umbrella | Rain=1) =", dict(zip(bn.get_states("Umbrella"), result["Umbrella"])), "\n")

    # --- 2. Build the structure yourself, then fit() parameters ---
    builder = lutufi.BayesianNetwork.builder()
    for var in ["Rain", "Umbrella", "Traffic"]:
        builder.add_variable(var, domain=["0", "1"])
    builder.add_edge("Rain", "Umbrella")
    builder.add_edge("Rain", "Traffic")

    # set_cpd() must be called before build() validates the model, even
    # though fit() will overwrite these placeholder values.
    builder.set_cpd("Rain", [0.5, 0.5])
    builder.set_cpd("Umbrella", [[0.5, 0.5], [0.5, 0.5]])
    builder.set_cpd("Traffic", [[0.5, 0.5], [0.5, 0.5]])
    bn2 = builder.build()

    fit(bn2, data, method="mle")
    print("fit(): learned P(Traffic | Rain):")
    print(bn2.cpd("Traffic"), "\n")

    # --- 3. Learn the structure itself, then fit parameters ---
    learned = learn_structure(data, method="hc")
    print("learn_structure(): recovered edges =", learned.edges())

    fit(learned, data, method="bayesian")
    print("is_valid after fit():", learned.is_valid())

    learned_engine = InferenceEngine(learned)
    if "Rain" in learned.nodes() and "Umbrella" in learned.nodes():
        result = learned_engine.query(["Umbrella"])
        print("P(Umbrella) from learned model =",
              dict(zip(learned.get_states("Umbrella"), result["Umbrella"])))


if __name__ == "__main__":
    main()
