//! FFI module for Python bindings (PyO3)
//!
//! This module provides Python bindings for the Lutufi Rust core using PyO3.

// The `#[pymethods]` expansion wraps every fallible return in an `Into<PyErr>`
// conversion. When the method already returns `PyResult`, that conversion is a
// no-op and clippy flags it — pointing at the generated return type rather than
// at anything we wrote. The lint is correct about the generated code and
// unactionable for us, so it is silenced here (scoped to the binding layer) and
// nowhere else.
#![allow(clippy::useless_conversion)]
// Python-constructed types are built via `#[new]`, which PyO3 requires. A Rust
// `Default` impl on the wrapper would be unused API surface.
#![allow(clippy::new_without_default)]
// Binding signatures mirror the Python API, where keyword arguments are the
// idiomatic way to pass inference options. Collapsing them into a struct would
// make the Python side worse to use in order to satisfy a Rust-side heuristic.
#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;

/// Variable type for probabilistic graphical models.
pub mod variable;
/// Domain types (discrete, binary, continuous) for variables.
pub mod domain;
/// Model types: BayesianNetwork, MarkovRandomField, DynamicBayesianNetwork.
pub mod models;
/// Inference engines: variable elimination, junction tree, LBP, MCMC, variational.
pub mod inference;
/// Parameter and structure learning algorithms.
pub mod learning;

/// Register all Python-exposed types and functions.
/// Called once when the Python extension module is loaded.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<variable::PyVariable>()?;
    m.add_class::<domain::PyDomain>()?;
    m.add_class::<models::PyBayesianNetwork>()?;
    m.add_class::<models::PyQueryResult>()?;
    m.add_class::<models::PyMarkovRandomField>()?;
    m.add_class::<models::PyDynamicBayesianNetwork>()?;
    m.add_class::<models::PyValidationResult>()?;
    m.add_class::<inference::PyVariableEliminationEngine>()?;
    m.add_class::<inference::PyJunctionTreeEngine>()?;
    m.add_class::<inference::PyLBPEngine>()?;
    m.add_class::<inference::PyMCMCEngine>()?;
    m.add_class::<inference::PyVariationalEngine>()?;
    m.add_class::<learning::PyParameterEstimator>()?;
    m.add_class::<learning::PyStructureLearner>()?;
    Ok(())
}