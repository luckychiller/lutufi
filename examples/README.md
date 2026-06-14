# Lutufi Examples

These scripts demonstrate the Lutufi Python API end to end. Run any of
them directly with `python examples/<name>.py` (after `maturin develop
--release` has built the native extension).

## Core API walkthroughs

| File | Demonstrates |
| --- | --- |
| [01_quickstart.py](01_quickstart.py) | Building a Bayesian network, the CPT convention, basic queries, "explaining away" |
| [02_inference_algorithms.py](02_inference_algorithms.py) | Exact (variable elimination, junction tree) and approximate (LBP, MCMC, variational) inference, treewidth, MAP/MPE queries |
| [03_learning_from_data.py](03_learning_from_data.py) | `from_dataframe()`, `ParameterEstimator`/`fit()`, `StructureLearner`/`learn_structure()` |
| [04_networkx_and_io.py](04_networkx_and_io.py) | Converting to/from NetworkX, reading/writing GraphML and other graph formats |
| [05_causal_reasoning.py](05_causal_reasoning.py) | `d_separated()`, `markov_blanket()`, `mark_as_causal()` |
| [06_markov_random_fields.py](06_markov_random_fields.py) | Building and inspecting undirected `MarkovRandomField` models |
| [07_dynamic_bayesian_networks.py](07_dynamic_bayesian_networks.py) | Intra-slice and inter-slice edges in a `DynamicBayesianNetwork` |
| [08_editing_and_error_handling.py](08_editing_and_error_handling.py) | The `edit()` atomic context manager and the `LutufiError` exception hierarchy |
| [09_visualization.py](09_visualization.py) | `plot()`, `plot_cpd()`, and `QueryResult.plot()` |

## Domain examples

| File | Scenario |
| --- | --- |
| [epidemiology/sir_model.py](epidemiology/sir_model.py) | A simple SIR-style disease-spread model over two time steps |
| [finance/interbank_risk.py](finance/interbank_risk.py) | Systemic risk contagion across a small interbank lending network |
| [intelligence/dark_network.py](intelligence/dark_network.py) | Inferring a hidden actor's activity from indirect observations |

## CPT convention

Across all examples, `set_cpd(variable, values)` follows one convention:

- **Rows** correspond to `variable`'s own states, in domain order.
- **Columns** correspond to parent configurations — the cartesian product
  of parent domains, with the *first*-added parent varying slowest.
- For a variable with no parents, pass a flat list (a single column).
- **Every column must sum to 1.0.**

`model.cpd(variable)` returns the same data flattened in
`(parent_config, state)` order; reshape with
`.reshape(-1, len(model.get_states(variable)))` to get a 2D table where
rows are parent configurations and columns are states.
