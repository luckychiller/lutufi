use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use crate::core::{
    assignment::Assignment,
    error::LutufiError,
    inference::{
        junction_tree::JunctionTreeEngine,
        variable_elimination::{VariableEliminationEngine, EliminationHeuristic, InferenceMode},
        lbp::{LBPEngine, LBPOptions},
        mcmc::{MCMCEngine, MCMCOptions},
        variational::{VariationalEngine, VariationalOptions},
    },
    models::factor_graph::FactorGraph,
};
use crate::ffi::models::PyBayesianNetwork;

#[pyclass(name = "_RustVariableEliminationEngine")]
pub struct PyVariableEliminationEngine {}

#[pyclass(name = "_RustJunctionTreeEngine")]
pub struct PyJunctionTreeEngine {
    engine: Option<JunctionTreeEngine>,
}

#[pyclass(name = "_RustLBPEngine")]
pub struct PyLBPEngine {}

#[pyclass(name = "_RustMCMCEngine")]
pub struct PyMCMCEngine {}

#[pyclass(name = "_RustVariationalEngine")]
pub struct PyVariationalEngine {}

#[pymethods]
impl PyJunctionTreeEngine {
    #[new]
    pub fn new(model: &PyBayesianNetwork) -> PyResult<Self> {
        let engine = JunctionTreeEngine::new(&model.inner).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyJunctionTreeEngine { engine: Some(engine) })
    }

    pub fn treewidth(&self) -> PyResult<usize> {
        self.engine.as_ref().map(|engine| engine.treewidth()).ok_or_else(|| PyValueError::new_err("Engine not initialized"))
    }

    #[pyo3(signature = (variables, evidence=None))]
    pub fn query(
        &self,
        py: Python<'_>,
        variables: Vec<String>,
        evidence: Option<HashMap<String, String>>,
    ) -> PyResult<HashMap<String, PyObject>> {
        let engine = self.engine.as_ref().ok_or_else(|| PyValueError::new_err("Engine not initialized"))?;

        // Heavy computation (junction tree propagation) runs with the GIL released
        // so other Python threads / the asyncio event loop are not blocked.
        let result_factor = py.allow_threads(|| -> Result<_, LutufiError> {
            let mut rust_evidence = Assignment::new();
            if let Some(map) = evidence {
                for (name, val) in map {
                    let var_id = engine.model().id_of(&name)?;
                    rust_evidence.set(var_id, val);
                }
            }

            let var_refs: Vec<&str> = variables.iter().map(|s| s.as_str()).collect();
            engine.query(&var_refs, &rust_evidence)
        }).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let mut result = HashMap::new();
        let scope = result_factor.scope();
        let values: Vec<f64> = (0..scope.num_entries()).map(|i| result_factor.log_value_at(i).exp()).collect();
        result.insert("values".to_string(), values.into_py(py));

        let mut var_names = Vec::new();
        for id in scope.variable_ids() {
            if let Some(var) = engine.model().variables().get(id) {
                var_names.push(var.name().to_string());
            } else {
                var_names.push(format!("Unknown_{}", id));
            }
        }
        result.insert("variables".to_string(), var_names.into_py(py));
        Ok(result)
    }
}

impl PyVariableEliminationEngine {
    fn parse_heuristic(py: Python<'_>, _model: &PyBayesianNetwork, heuristic: Option<PyObject>) -> PyResult<EliminationHeuristic> {
        if let Some(h_obj) = heuristic {
            let h_any = h_obj.bind(py);
            if let Ok(s) = h_any.extract::<String>() {
                return match s.as_str() {
                    "min_degree" => Ok(EliminationHeuristic::MinDegree),
                    "min_fill" => Ok(EliminationHeuristic::MinFill),
                    other => Err(PyValueError::new_err(format!("Unknown heuristic: {}", other))),
                };
            }
        }
        Ok(EliminationHeuristic::MinFill)
    }

    fn pack_result(py: Python<'_>, model: &crate::core::models::bayesian_network::BayesianNetwork, factor: crate::core::factor::TabularFactor) -> HashMap<String, PyObject> {
        let mut result = HashMap::new();
        let scope = factor.scope();
        let values: Vec<f64> = (0..scope.num_entries()).map(|i| factor.log_value_at(i).exp()).collect();
        result.insert("values".to_string(), values.into_py(py));
        let mut var_names = Vec::new();
        for id in scope.variable_ids() {
            if let Some(var) = model.variables().get(id) {
                var_names.push(var.name().to_string());
            } else {
                var_names.push(format!("Unknown_{}", id));
            }
        }
        result.insert("variables".to_string(), var_names.into_py(py));
        result
    }
}

#[pymethods]
impl PyVariableEliminationEngine {
    #[new]
    pub fn new() -> Self {
        PyVariableEliminationEngine {}
    }

    #[pyo3(signature = (model, variables, evidence, heuristic=None))]
    pub fn query(
        &self,
        py: Python<'_>,
        model: &PyBayesianNetwork,
        variables: Vec<String>,
        evidence: HashMap<String, String>,
        heuristic: Option<PyObject>,
    ) -> PyResult<HashMap<String, PyObject>> {
        // Parsing the heuristic requires the GIL (it inspects a Python object).
        let h = Self::parse_heuristic(py, model, heuristic)?;

        // The actual elimination is the expensive part, so run it with the GIL released.
        let result_factor = py.allow_threads(|| -> Result<_, LutufiError> {
            let mut rust_evidence = Assignment::new();
            for (name, val) in evidence {
                let var_id = model.inner.id_of(&name)?;
                rust_evidence.set(var_id, val);
            }

            let engine = VariableEliminationEngine::new(&model.inner);
            let var_refs: Vec<&str> = variables.iter().map(|s| s.as_str()).collect();
            engine.query(&var_refs, &rust_evidence, h)
        }).map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(Self::pack_result(py, &model.inner, result_factor))
    }

    #[pyo3(signature = (model, variables, evidence, heuristic=None, mode="map"))]
    pub fn query_map(
        &self,
        py: Python<'_>,
        model: &PyBayesianNetwork,
        variables: Vec<String>,
        evidence: HashMap<String, String>,
        heuristic: Option<PyObject>,
        mode: &str,
    ) -> PyResult<HashMap<String, PyObject>> {
        let h = Self::parse_heuristic(py, model, heuristic)?;
        let m = match mode {
            "map" => InferenceMode::Map,
            "mpe" => InferenceMode::Mpe,
            other => return Err(PyValueError::new_err(format!("Unknown mode: {}", other))),
        };

        let result_factor = py.allow_threads(|| -> Result<_, LutufiError> {
            let mut rust_evidence = Assignment::new();
            for (name, val) in evidence {
                let var_id = model.inner.id_of(&name)?;
                rust_evidence.set(var_id, val);
            }

            let engine = VariableEliminationEngine::new(&model.inner);
            let var_refs: Vec<&str> = variables.iter().map(|s| s.as_str()).collect();
            engine.query_map(&var_refs, &rust_evidence, h, m)
        }).map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(Self::pack_result(py, &model.inner, result_factor))
    }
}

#[pymethods]
impl PyLBPEngine {
    #[new] pub fn new() -> Self { PyLBPEngine {} }

    #[pyo3(signature = (model, variables, evidence, max_iterations=1000, tolerance=1e-6, damping=0.5))]
    pub fn query(
        &self,
        py: Python<'_>,
        model: &PyBayesianNetwork,
        variables: Vec<String>,
        evidence: HashMap<String, String>,
        max_iterations: usize,
        tolerance: f64,
        damping: f64,
    ) -> PyResult<HashMap<String, PyObject>> {
        // Building the factor graph and running loopy belief propagation are both
        // potentially expensive; do all of it with the GIL released.
        let res = py.allow_threads(|| -> Result<_, LutufiError> {
            let fg = FactorGraph::from_bayesian_network(&model.inner)?;
            let options = LBPOptions { max_iterations, tolerance, damping };
            let mut engine = LBPEngine::new(fg, options);

            let mut rust_evidence = Assignment::new();
            for (name, val) in evidence {
                let id = model.inner.id_of(&name)?;
                rust_evidence.set(id, val);
            }

            engine.run(&rust_evidence)
        }).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let mut py_res = HashMap::new();
        py_res.insert("converged".to_string(), res.converged.into_py(py));
        py_res.insert("iterations".to_string(), res.iterations.into_py(py));
        py_res.insert("residual".to_string(), res.residual.into_py(py));

        let mut marginals = HashMap::new();
        for name in variables {
            let id = model.inner.id_of(&name).map_err(|e: LutufiError| PyValueError::new_err(e.to_string()))?;
            let factor = res.beliefs.get(&id).ok_or_else(|| PyValueError::new_err("Missing marginal"))?;
            let values: Vec<f64> = (0..factor.scope().num_entries()).map(|i| factor.value_at(i)).collect();
            marginals.insert(name, values.into_py(py));
        }
        py_res.insert("marginals".to_string(), marginals.into_py(py));
        Ok(py_res)
    }
}

#[pymethods]
impl PyMCMCEngine {
    #[new] pub fn new() -> Self { PyMCMCEngine {} }

    #[pyo3(signature = (model, variables, evidence, n_samples=1000, burn_in=100))]
    pub fn query(
        &self,
        py: Python<'_>,
        model: &PyBayesianNetwork,
        variables: Vec<String>,
        evidence: HashMap<String, String>,
        n_samples: usize,
        burn_in: usize,
    ) -> PyResult<HashMap<String, PyObject>> {
        // Sampling can take a long time on large networks; release the GIL for it.
        let res = py.allow_threads(|| -> Result<_, LutufiError> {
            let fg = FactorGraph::from_bayesian_network(&model.inner)?;
            let options = MCMCOptions { n_samples, burn_in, ..Default::default() };
            let engine = MCMCEngine::new(fg, options);

            let mut rust_evidence = Assignment::new();
            for (name, val) in evidence {
                let id = model.inner.id_of(&name)?;
                rust_evidence.set(id, val);
            }

            engine.run(&rust_evidence)
        }).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let mut py_res = HashMap::new();
        py_res.insert("n_samples".to_string(), res.n_samples.into_py(py));

        let mut marginals = HashMap::new();
        for name in variables {
            let id = model.inner.id_of(&name).map_err(|e: LutufiError| PyValueError::new_err(e.to_string()))?;
            let factor = res.marginals.get(&id).ok_or_else(|| PyValueError::new_err("Missing marginal"))?;
            let values: Vec<f64> = (0..factor.scope().num_entries()).map(|i| factor.value_at(i)).collect();
            marginals.insert(name, values.into_py(py));
        }
        py_res.insert("marginals".to_string(), marginals.into_py(py));
        Ok(py_res)
    }
}

#[pymethods]
impl PyVariationalEngine {
    #[new] pub fn new() -> Self { PyVariationalEngine {} }

    #[pyo3(signature = (model, variables, evidence, max_iterations=100, tolerance=1e-4))]
    pub fn query(
        &self,
        py: Python<'_>,
        model: &PyBayesianNetwork,
        variables: Vec<String>,
        evidence: HashMap<String, String>,
        max_iterations: usize,
        tolerance: f64,
    ) -> PyResult<HashMap<String, PyObject>> {
        // Mean-field optimization runs with the GIL released.
        let res = py.allow_threads(|| -> Result<_, LutufiError> {
            let fg = FactorGraph::from_bayesian_network(&model.inner)?;
            let options = VariationalOptions { max_iterations, tolerance, ..Default::default() };
            let engine = VariationalEngine::new(fg, options);

            let mut rust_evidence = Assignment::new();
            for (name, val) in evidence {
                let id = model.inner.id_of(&name)?;
                rust_evidence.set(id, val);
            }

            engine.run_with_evidence(&rust_evidence)
        }).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let mut py_res = HashMap::new();
        py_res.insert("converged".to_string(), res.converged.into_py(py));
        py_res.insert("elbo".to_string(), res.elbo.into_py(py));

        let mut marginals = HashMap::new();
        for name in variables {
            let id = model.inner.id_of(&name).map_err(|e: LutufiError| PyValueError::new_err(e.to_string()))?;
            let factor = res.marginals.get(&id).ok_or_else(|| PyValueError::new_err("Missing marginal"))?;
            let values: Vec<f64> = (0..factor.scope().num_entries()).map(|i| factor.value_at(i)).collect();
            marginals.insert(name, values.into_py(py));
        }
        py_res.insert("marginals".to_string(), marginals.into_py(py));
        Ok(py_res)
    }
}
