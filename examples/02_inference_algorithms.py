"""
Inference algorithms: exact vs. approximate.

Lutufi ships several inference backends, all reachable through the same
InferenceEngine interface:

  - "variable_elimination" (alias "exact"): exact, recomputed per query.
  - "junction_tree": exact, compiles a junction tree once and reuses it
    for repeated queries against the same evidence/model.
  - "lbp": loopy belief propagation (approximate, fast on large/loopy graphs).
  - "mcmc": Gibbs sampling (approximate, asymptotically exact).
  - "variational": mean-field variational inference (approximate).
  - "auto" (default): picks an algorithm automatically based on the
    network's size/treewidth.

This example builds a slightly larger network (a small "Alarm"-style
network) and runs the same query through several algorithms so you can
compare their outputs and diagnostics.
"""

import lutufi
from lutufi.inference import InferenceEngine, InferenceOptions, JunctionTreeEngine


def build_alarm_network() -> lutufi.BayesianNetwork:
    builder = (
        lutufi.BayesianNetwork.builder()
        .add_variable("Burglary", domain=["T", "F"])
        .add_variable("Earthquake", domain=["T", "F"])
        .add_variable("Alarm", domain=["T", "F"])
        .add_variable("JohnCalls", domain=["T", "F"])
        .add_variable("MaryCalls", domain=["T", "F"])
        .add_edge("Burglary", "Alarm")
        .add_edge("Earthquake", "Alarm")
        .add_edge("Alarm", "JohnCalls")
        .add_edge("Alarm", "MaryCalls")
    )

    builder.set_cpd("Burglary", [0.001, 0.999])
    builder.set_cpd("Earthquake", [0.002, 0.998])

    # P(Alarm | Burglary, Earthquake): columns are
    # (B=T,E=T), (B=T,E=F), (B=F,E=T), (B=F,E=F)
    builder.set_cpd("Alarm", [
        [0.95, 0.94, 0.29, 0.001],  # Alarm=T
        [0.05, 0.06, 0.71, 0.999],  # Alarm=F
    ])

    # P(JohnCalls | Alarm): columns are Alarm=T, Alarm=F
    builder.set_cpd("JohnCalls", [
        [0.90, 0.05],  # JohnCalls=T
        [0.10, 0.95],  # JohnCalls=F
    ])

    # P(MaryCalls | Alarm): columns are Alarm=T, Alarm=F
    builder.set_cpd("MaryCalls", [
        [0.70, 0.01],  # MaryCalls=T
        [0.30, 0.99],  # MaryCalls=F
    ])

    return builder.build()


def main():
    bn = build_alarm_network()
    evidence = {"JohnCalls": "T", "MaryCalls": "T"}

    print("Query: P(Burglary | JohnCalls=T, MaryCalls=T)\n")

    # --- Exact inference via variable elimination ---
    ve = InferenceEngine(bn, algorithm="variable_elimination")
    result = ve.query(["Burglary"], evidence=evidence)
    print(f"variable_elimination -> {result['Burglary']}  (algorithm={result.algorithm})")

    # --- Exact inference via a compiled junction tree ---
    jt = InferenceEngine(bn, algorithm="junction_tree")
    result = jt.query(["Burglary"], evidence=evidence)
    print(f"junction_tree         -> {result['Burglary']}  (algorithm={result.algorithm})")

    # The junction tree can also be built directly to inspect treewidth,
    # which is useful for deciding whether exact inference is tractable.
    jt_engine = JunctionTreeEngine(bn)
    print(f"junction tree treewidth: {jt_engine.treewidth}")

    # --- Approximate: loopy belief propagation ---
    lbp = InferenceEngine(bn, algorithm="lbp")
    lbp.set_options(InferenceOptions(max_iterations=50, tolerance=1e-4, damping=0.5))
    result = lbp.query(["Burglary"], evidence=evidence)
    print(f"lbp                    -> {result['Burglary']}  "
          f"(converged={result.diagnostics['converged']}, "
          f"iterations={result.diagnostics['iterations']})")

    # --- Approximate: Gibbs sampling / MCMC ---
    mcmc = InferenceEngine(bn, algorithm="mcmc")
    mcmc.set_options(InferenceOptions(n_samples=5000, burn_in=500, seed=42))
    result = mcmc.query(["Burglary"], evidence=evidence)
    print(f"mcmc                   -> {result['Burglary']}  "
          f"(n_samples={result.diagnostics['n_samples']})")

    # --- Approximate: mean-field variational inference ---
    vi = InferenceEngine(bn, algorithm="variational")
    vi.set_options(InferenceOptions(max_iterations=50, tolerance=1e-6))
    result = vi.query(["Burglary"], evidence=evidence)
    print(f"variational            -> {result['Burglary']}  "
          f"(converged={result.diagnostics['converged']}, "
          f"elbo={result.diagnostics['elbo']:.4f})")

    # --- "auto": let Lutufi pick based on the network's treewidth ---
    auto = InferenceEngine(bn)  # algorithm="auto" is the default
    result = auto.query(["Burglary"], evidence=evidence)
    print(f"auto                   -> {result['Burglary']}  (algorithm={result.algorithm})")

    # --- MAP / MPE queries ---
    # mode="map": most probable joint assignment for the *queried* variables.
    map_result = ve.query(["Alarm", "Burglary"], evidence=evidence, mode="map")
    print("\nMost probable (Alarm, Burglary) given both calls:", map_result.most_probable())

    # mode="mpe": most probable explanation for *all* unobserved variables.
    mpe_result = ve.query(bn.nodes(), evidence=evidence, mode="mpe")
    print("Most probable full explanation:", mpe_result.most_probable())


if __name__ == "__main__":
    main()
