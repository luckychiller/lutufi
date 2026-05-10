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

#[pyclass(name = "_RustParameterEstimator")]
pub struct PyParameterEstimator {
    inner: ParameterEstimator,
}

#[pyclass(name = "_RustStructureLearner")]
pub struct PyStructureLearner {
    score_based: ScoreBasedLearner,
    constraint_based: ConstraintBasedLearner,
}

#[pymethods]
impl PyParameterEstimator {
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
    #[new]
    pub fn new() -> Self {
        Self {
            score_based: ScoreBasedLearner::new(ScoreBasedOptions::default()),
            constraint_based: ConstraintBasedLearner::new(ConstraintBasedOptions::default()),
        }
    }

    #[pyo3(signature = (data, method="hc"))]
    pub fn learn_structure(
        &self,
        data: Vec<HashMap<String, String>>,
        method: &str,
    ) -> PyResult<PyBayesianNetwork> {
        let build_py_bn = |bn: crate::core::models::bayesian_network::BayesianNetwork| -> PyBayesianNetwork {
            let mut registry = std::collections::HashMap::new();
            for (_, var) in bn.variables() {
                registry.insert(var.name().to_string(), var.clone());
            }
            PyBayesianNetwork { inner: bn, variable_registry: registry }
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
                let bn = self.constraint_based.fci_algorithm(&data)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                Ok(build_py_bn(bn))
            },
            _ => Err(PyValueError::new_err("Invalid structure learning method")),
        }
    }
}
