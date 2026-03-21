//! FFI module for Python bindings (PyO3)
//!
//! This module provides Python bindings for the Lutufi Rust core using PyO3.

use pyo3::prelude::*;

pub mod domain;
pub mod variable;

/// Python module initialization
#[pymodule]
fn _lutufi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Add classes
    m.add_class::<domain::PyDomain>()?;
    m.add_class::<variable::PyVariable>()?;
    
    // Add submodules
    let models_mod = PyModule::new_bound(m.py(), "models")?;
    models_init(&models_mod)?;
    m.add_submodule(&models_mod)?;

    let inference_mod = PyModule::new_bound(m.py(), "inference")?;
    inference_init(&inference_mod)?;
    m.add_submodule(&inference_mod)?;

    let learning_mod = PyModule::new_bound(m.py(), "learning")?;
    learning_init(&learning_mod)?;
    m.add_submodule(&learning_mod)?;

    let io_mod = PyModule::new_bound(m.py(), "io")?;
    io_init(&io_mod)?;
    m.add_submodule(&io_mod)?;
    
    Ok(())
}

fn models_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__doc__", "Network models module")?;
    Ok(())
}

fn inference_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__doc__", "Probabilistic inference module")?;
    Ok(())
}

fn learning_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__doc__", "Learning module")?;
    Ok(())
}

fn io_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__doc__", "I/O operations module")?;
    Ok(())
}
