use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use crate::core::{
    assignment::Assignment,
    inference::{junction_tree::JunctionTreeEngine, variable_elimination::{VariableEliminationEngine, EliminationHeuristic, InferenceMode}},
};
use crate::ffi::models::PyBayesianNetwork;

#[pyclass(name = "_RustVariableEliminationEngine")]
pub struct PyVariableEliminationEngine {
    // We need to store the model or a reference to it.
    // For simplicity in FFI, we might just store the network and create the engine when needed.
}

#[pyclass(name = "_RustJunctionTreeEngine")]
pub struct PyJunctionTreeEngine {
    engine: Option<JunctionTreeEngine>,
}

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
        let mut rust_evidence = Assignment::new();
        if let Some(map) = evidence {
            for (name, val) in map {
                let var_id = engine.model.id_of(&name).map_err(|e: crate::core::error::LutufiError| PyValueError::new_err(e.to_string()))?;
                rust_evidence.set(var_id, val);
            }
        }

        let var_refs: Vec<&str> = variables.iter().map(|s| s.as_str()).collect();
        let result_factor = engine.query(&var_refs, &rust_evidence).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let mut result = HashMap::new();
        let scope = result_factor.scope();
        let values: Vec<f64> = (0..scope.num_entries()).map(|i| result_factor.log_value_at(i).exp()).collect();
        result.insert("values".to_string(), values.into_py(py));

        let mut var_names = Vec::new();
        for id in scope.variable_ids() {
            if let Some(var) = engine.model.variables.get(id) {
                var_names.push(var.name().to_string());
            } else {
                var_names.push(format!("Unknown_{}", id));
            }
        }
        result.insert("variables".to_string(), var_names.into_py(py));
        Ok(result)
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
        heuristic: Option<PyObject>, // Can be String or Vec<String>
    ) -> PyResult<HashMap<String, PyObject>> {
        let h = if let Some(h_obj) = heuristic {
            let h_any = h_obj.bind(py);
            if let Ok(s) = h_any.extract::<String>() {
                match s.as_str() {
                    "min_degree" => EliminationHeuristic::MinDegree,
                    "min_fill" => EliminationHeuristic::MinFill,
                    other => return Err(PyValueError::new_err(format!("Unknown heuristic: {}", other))),
                }
            } else if let Ok(v) = h_any.extract::<Vec<String>>() {
                let mut ids = Vec::new();
                for name in v {
                    let id = model.inner.id_of(&name).map_err(|e: crate::core::error::LutufiError| PyValueError::new_err(e.to_string()))?;
                    ids.push(id);
                }
                EliminationHeuristic::UserSpecified(ids)
            } else {
                return Err(PyValueError::new_err("Heuristic must be a string or a list of variable names"));
            }
        } else {
            EliminationHeuristic::MinFill // Default
        };

        // Create Assignment from HashMap
        let mut rust_evidence = Assignment::new();
        for (name, val) in evidence {
            let var_id = model.inner.id_of(&name).map_err(|e: crate::core::error::LutufiError| PyValueError::new_err(e.to_string()))?;
            rust_evidence.set(var_id, val);
        }

        let engine = VariableEliminationEngine::new(&model.inner);
        let var_refs: Vec<&str> = variables.iter().map(|s| s.as_str()).collect();
        
        let result_factor = engine.query(&var_refs, &rust_evidence, h)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        
        let mut result = HashMap::new();
        let scope = result_factor.scope();

        let values: Vec<f64> = (0..scope.num_entries())
            .map(|i| result_factor.log_value_at(i).exp())
            .collect();
        
        result.insert("values".to_string(), values.into_py(py));
        
        let mut var_names = Vec::new();
        for id in scope.variable_ids() {
            if let Some(var) = model.inner.variables.get(id) {
                var_names.push(var.name().to_string());
            } else {
                var_names.push(format!("Unknown_{}", id));
            }
        }
        result.insert("variables".to_string(), var_names.into_py(py));
        
        Ok(result)
    }

    #[pyo3(signature = (model, variables, evidence, heuristic=None, mode="map"))]
    pub fn query_map(
        &self,
        py: Python<'_>,
        model: &PyBayesianNetwork,
        variables: Vec<String>,
        evidence: HashMap<String, String>,
        heuristic: Option<PyObject>,
        mode: String,
    ) -> PyResult<HashMap<String, PyObject>> {
        let h = Self::parse_heuristic(py, model, heuristic)?;
        let m = match mode.as_str() {
            "map" => InferenceMode::Map,
            "mpe" => InferenceMode::Mpe,
            other => return Err(PyValueError::new_err(format!("Unknown mode: {}", other))),
        };

        let mut rust_evidence = Assignment::new();
        for (name, val) in evidence {
            let var_id = model.inner.id_of(&name).map_err(|e: crate::core::error::LutufiError| PyValueError::new_err(e.to_string()))?;
            rust_evidence.set(var_id, val);
        }

        let engine = VariableEliminationEngine::new(&model.inner);
        let var_refs: Vec<&str> = variables.iter().map(|s| s.as_str()).collect();
        let result_factor = engine.query_map(&var_refs, &rust_evidence, h, m)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        let mut result = HashMap::new();
        let scope = result_factor.scope();

        let values: Vec<f64> = (0..scope.num_entries())
            .map(|i| result_factor.log_value_at(i).exp())
            .collect();

        result.insert("values".to_string(), values.into_py(py));
        let mut var_names = Vec::new();
        for id in scope.variable_ids() {
            if let Some(var) = model.inner.variables.get(id) {
                var_names.push(var.name().to_string());
            } else {
                var_names.push(format!("Unknown_{}", id));
            }
        }
        result.insert("variables".to_string(), var_names.into_py(py));
        Ok(result)
    }

    #[pyo3(signature = (model, heuristic=None))]
    pub fn estimate_treewidth(
        &self,
        py: Python<'_>,
        model: &PyBayesianNetwork,
        heuristic: Option<PyObject>,
    ) -> PyResult<usize> {
        let h = Self::parse_heuristic(py, model, heuristic)?;
        let engine = VariableEliminationEngine::new(&model.inner);
        engine.estimate_treewidth(h).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn parse_heuristic(
        py: Python<'_>,
        model: &PyBayesianNetwork,
        heuristic: Option<PyObject>,
    ) -> PyResult<EliminationHeuristic> {
        if let Some(h_obj) = heuristic {
            let h_any = h_obj.bind(py);
            if let Ok(s) = h_any.extract::<String>() {
                return match s.as_str() {
                    "min_degree" => Ok(EliminationHeuristic::MinDegree),
                    "min_fill" => Ok(EliminationHeuristic::MinFill),
                    other => Err(PyValueError::new_err(format!("Unknown heuristic: {}", other))),
                };
            }
            if let Ok(v) = h_any.extract::<Vec<String>>() {
                let mut ids = Vec::new();
                for name in v {
                    let id = model.inner.id_of(&name).map_err(|e: crate::core::error::LutufiError| PyValueError::new_err(e.to_string()))?;
                    ids.push(id);
                }
                return Ok(EliminationHeuristic::UserSpecified(ids));
            }
            return Err(PyValueError::new_err("Heuristic must be a string or a list of variable names"));
        }
        Ok(EliminationHeuristic::MinFill)
    }
}
