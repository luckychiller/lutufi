//! FFI module for Python bindings (PyO3)
//!
//! This module provides Python bindings for the Lutufi Rust core using PyO3.

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