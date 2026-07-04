# Changelog

All notable changes to Lutufi are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-07-04

First stable release.

### Added

- **Core models**: `BayesianNetwork` (with fluent builder), `MarkovRandomField`,
  and two-slice `DynamicBayesianNetwork`.
- **Exact inference**: variable elimination and junction tree propagation, with
  automatic algorithm selection based on estimated treewidth.
- **Approximate inference**: loopy belief propagation, Gibbs sampling (MCMC)
  with multi-chain Gelman–Rubin diagnostics, and mean-field variational
  inference.
- **Log-space computation** throughout the factor algebra for numerical
  stability on large and deep networks.
- **Learning**: parameter estimation (MLE and Bayesian with pseudocounts) and
  constraint-based / score-based structure learning, including
  `BayesianNetwork.from_dataframe()` for learning directly from pandas.
- **Structural & causal analysis**: d-separation queries, Markov blankets,
  explicit `mark_as_causal()` semantics, and do-calculus / ID-algorithm
  machinery in the Rust core.
- **I/O**: BIF, XMLBIF, UAI, and the native LMF format (with format
  versioning and migration), plus NetworkX round-trips and
  GraphML/GEXF/GML/Pajek support via `lutufi.io`.
- **Python API**: typed (`py.typed` + stubs), documented exception hierarchy
  (`LutufiError` and subclasses), atomic `edit()` context manager,
  matplotlib visualization helpers, and lazy/async query support.
- **Packaging**: abi3 wheels (one wheel per platform covers Python 3.9+) for
  Linux, macOS, and Windows on x86_64 and arm64; the GIL is released during
  heavy Rust computations.

### Fixed

- BIF, XMLBIF, UAI, and LMF importers constructed conditional probability
  tables transposed (rows/columns swapped), which corrupted or silently
  dropped CPTs on import. Importers now use the documented convention (one
  row per child state, one column per parent configuration) and propagate
  errors instead of skipping invalid tables.
- UAI export now uses a deterministic (topological) variable ordering, making
  exports stable and round-trippable.
