use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use crate::core::{
    learning::{
        ParameterEstimator, ParameterLearningMethod, LegacyParameterLearningOptions,
        ScoreBasedLearner, ScoreBasedOptions,
        ConstraintBasedLearner, ConstraintBasedOptions,
    },
};
use crate::ffi::models::PyBayesianNetwork;

/// Estimate parameters (CPTs) of a Bayesian network from data.
#[pyclass(name = "_RustParameterEstimator")]
pub struct PyParameterEstimator {
    inner: ParameterEstimator,
}

/// Learn the structure (graph) of a Bayesian network from data.
///
/// Supports score-based (hill climbing, GES) and constraint-based (PC, FCI) algorithms.
#[pyclass(name = "_RustStructureLearner")]
pub struct PyStructureLearner {
    score_based: ScoreBasedLearner,
    constraint_based: ConstraintBasedLearner,
}

#[pymethods]
impl PyParameterEstimator {
    /// Create a parameter estimator with the given method ("mle" or "bayesian").
    #[new]
    #[pyo3(signature = (method="mle", alpha=0.5, max_iterations=100))]
    pub fn new(method: &str, alpha: f64, max_iterations: usize) -> PyResult<Self> {
        let m = match method {
            "mle" => ParameterLearningMethod::MLE,
            "bayesian" => ParameterLearningMethod::Bayesian,
            _ => return Err(PyValueError::new_err("Invalid method")),
        };
        let options = LegacyParameterLearningOptions {
            method: m,
            alpha,
            max_iterations,
            tolerance: 1e-4,
        };
        Ok(PyParameterEstimator { inner: ParameterEstimator::new(options) })
    }

    /// Fit the CPTs of a Bayesian network to the provided data.
    pub fn fit(
        &self,
        model: &mut PyBayesianNetwork,
        data: Vec<HashMap<String, String>>,
    ) -> PyResult<()> {
        self.inner.fit(&mut model.inner, &data)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymethods]
impl PyStructureLearner {
    /// Create a new structure learner.
    #[new]
    pub fn new() -> Self {
        Self {
            score_based: ScoreBasedLearner::new(ScoreBasedOptions::default()),
            constraint_based: ConstraintBasedLearner::new(ConstraintBasedOptions::default()),
        }
    }

    /// Learn a Bayesian network structure from data using the specified algorithm ("hc", "ges", "pc", "fci").
    #[pyo3(signature = (data, method="hc"))]
    pub fn learn_structure(
        &self,
        data: Vec<HashMap<String, String>>,
        method: &str,
    ) -> PyResult<PyBayesianNetwork> {
        let build_py_bn = |bn: crate::core::models::bayesian_network::BayesianNetwork| -> PyBayesianNetwork {
            PyBayesianNetwork { inner: bn }
        };

        match method {
            "hc" => {
                let bn = self.score_based.hill_climbing(&data, &[], &[])
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                Ok(build_py_bn(bn))
            },
            "ges" => {
                let bn = self.score_based.ges(&data)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                Ok(build_py_bn(bn))
            },
            "pc" => {
                let bn = self.constraint_based.pc_algorithm(&data)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                Ok(build_py_bn(bn))
            },
            "fci" => {
                let result = self.constraint_based.fci_algorithm(&data)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                Ok(build_py_bn(result.network))
            },
            _ => Err(PyValueError::new_err("Invalid structure learning method")),
        }
    }
}
