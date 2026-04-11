//! FFI module for Python bindings (PyO3)
//!
//! This module provides Python bindings for the Lutufi Rust core using PyO3.

use pyo3::prelude::*;

pub mod variable;
pub mod domain;
pub mod models;
pub mod inference;

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
    Ok(())
}