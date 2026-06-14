use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::core::domain::{Domain, BINARY_STATES};

/// Python wrapper for Domain.
///
/// Python usage:
///   domain = Domain.discrete(["low", "medium", "high"])
///   domain = Domain.binary()
///   domain = Domain.continuous(lower=0.0, upper=1.0)
#[pyclass(name = "Domain")]
#[derive(Clone)]
pub struct PyDomain {
    pub(crate) inner: Domain,
}

#[pymethods]
impl PyDomain {
    /// Create a discrete domain from a list of state names.
    #[staticmethod]
    pub fn discrete(states: Vec<String>) -> PyResult<Self> {
        Domain::discrete(states)
            .map(|d| PyDomain { inner: d })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Create a binary domain with states ["false", "true"].
    #[staticmethod]
    pub fn binary() -> Self {
        PyDomain { inner: Domain::binary() }
    }

    /// Create a continuous real-valued domain.
    #[staticmethod]
    #[pyo3(signature = (lower=None, upper=None))]
    pub fn continuous(lower: Option<f64>, upper: Option<f64>) -> Self {
        PyDomain { inner: Domain::continuous(lower, upper) }
    }

    /// Number of states (None for continuous domains).
    #[getter]
    pub fn size(&self) -> Option<usize> {
        self.inner.size()
    }

    /// Whether this domain is discrete (finite number of states).
    #[getter]
    pub fn is_discrete(&self) -> bool {
        self.inner.is_discrete()
    }

    /// The state names for discrete domains. None for continuous.
    #[getter]
    pub fn states(&self) -> Option<Vec<String>> {
        match &self.inner {
            Domain::Discrete { states } => Some(states.clone()),
            Domain::Binary => Some(BINARY_STATES.iter().map(|s| s.to_string()).collect()),
            Domain::Continuous { .. } => None,
        }
    }

    /// Check if a value is valid for this domain.
    pub fn contains(&self, value: &str) -> bool {
        self.inner.contains(value)
    }

    /// Return a string representation of this Domain.
    pub fn __repr__(&self) -> String {
        match &self.inner {
            Domain::Discrete { states } => format!("Domain.discrete({:?})", states),
            Domain::Binary => "Domain.binary()".to_string(),
            Domain::Continuous { lower, upper } => {
                format!("Domain.continuous(lower={:?}, upper={:?})", lower, upper)
            }
        }
    }

    /// Return a human-readable string for this Domain.
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}
