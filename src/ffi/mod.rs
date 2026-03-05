//! FFI module for Python bindings (PyO3)
//!
//! This module provides Python bindings for the Lutufi Rust core using PyO3.

use pyo3::prelude::*;

/// Python module initialization
#[pymodule]
fn _lutufi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    
    // Add submodules
    m.add_wrapped(pyo3::wrap_pymodule!(models))?;
    m.add_wrapped(pyo3::wrap_pymodule!(inference))?;
    m.add_wrapped(pyo3::wrap_pymodule!(learning))?;
    m.add_wrapped(pyo3::wrap_pymodule!(io))?;
    
    Ok(())
}

/// Models submodule
#[pymodule]
fn models(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__doc__", "Network models module")?;
    Ok(())
}

/// Inference submodule
#[pymodule]
fn inference(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__doc__", "Probabilistic inference module")?;
    Ok(())
}

/// Learning submodule
#[pymodule]
fn learning(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__doc__", "Learning module")?;
    Ok(())
}

/// I/O submodule
#[pymodule]
fn io(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__doc__", "I/O operations module")?;
    Ok(())
}