use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::core::{domain::Domain, variable::Variable};
use crate::ffi::domain::PyDomain;

/// Python wrapper for Variable.
///
/// Python usage:
///   v = Variable("income", domain=["low", "medium", "high"])
///   print(v.name)       # "income"
///   print(v.domain)     # Domain.discrete(["low", "medium", "high"])
#[pyclass(name = "Variable")]
#[derive(Clone)]
pub struct PyVariable {
    pub(crate) inner: Variable,
}

#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature = (name, domain))]
    pub fn new(name: &str, domain: Vec<String>) -> PyResult<Self> {
        let d = Domain::discrete(domain)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyVariable {
            inner: Variable::new(name, d),
        })
    }

    /// The human-readable name of this variable.
    #[getter]
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    /// The domain of this variable.
    #[getter]
    pub fn domain(&self) -> PyDomain {
        PyDomain { inner: self.inner.domain().clone() }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Variable({:?}, domain={:?})",
            self.inner.name(),
            self.inner.domain().states()
        )
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}
