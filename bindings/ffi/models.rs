use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use crate::core::{
    domain::Domain,
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    variable::Variable,
};

#[pyclass(name = "ValidationResult")]
pub struct PyValidationResult {
    #[pyo3(get)]
    pub is_valid: bool,
    #[pyo3(get)]
    pub errors: Vec<String>,
}

#[pymethods]
impl PyValidationResult {
    pub fn __repr__(&self) -> String {
        if self.is_valid { "ValidationResult(valid=True)".to_string() }
        else { format!("ValidationResult(valid=False, errors={:?})", self.errors) }
    }
    pub fn __bool__(&self) -> bool { self.is_valid }
}

#[pyclass(name = "_RustBayesianNetwork")]
pub struct PyBayesianNetwork {
    pub(crate) inner: BayesianNetwork,
    pub(crate) variable_registry: HashMap<String, Variable>,
}

#[pymethods]
impl PyBayesianNetwork {
    #[new]
    pub fn new() -> Self {
        PyBayesianNetwork {
            inner: BayesianNetwork::new(),
            variable_registry: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, name: &str, domain: Vec<String>) -> PyResult<()> {
        let d = Domain::discrete(domain).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let var = Variable::new(name, d.clone());
        self.variable_registry.insert(name.to_string(), var);
        self.inner.add_variable(name, d).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    pub fn add_edge(&mut self, from_name: &str, to_name: &str) -> PyResult<()> {
        self.inner.add_edge(from_name, to_name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn remove_edge(&mut self, from_name: &str, to_name: &str) -> PyResult<()> {
        self.inner.remove_edge(from_name, to_name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn set_cpd(&mut self, variable_name: &str, values: &Bound<'_, PyAny>) -> PyResult<()> {
        let normalized = self.normalize_cpd_input(variable_name, values)?;
        let child = self.variable_registry.get(variable_name).ok_or_else(|| PyValueError::new_err(format!("Var {} not found", variable_name)))?.clone();
        let parent_vars = self.collect_parent_variables(variable_name)?;
        let parent_refs: Vec<&Variable> = parent_vars.iter().collect();
        let cpd = ConditionalProbabilityTable::from_values(&child, &parent_refs, normalized).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.set_cpd(variable_name, cpd).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_cpd(&self, variable_name: &str) -> PyResult<HashMap<String, Vec<f64>>> {
        self.inner.cpd(variable_name).map(|cpd| {
            let mut result = HashMap::new();
            let factor = cpd.as_factor();
            let values: Vec<f64> = (0..factor.scope().num_entries()).map(|i| factor.value_at(i)).collect();
            result.insert("values".to_string(), values);
            result
        }).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn nodes(&self) -> Vec<String> { self.inner.nodes().iter().map(|&s| s.to_string()).collect() }
    pub fn edges(&self) -> Vec<(String, String)> { self.inner.edges().iter().map(|(a, b)| (a.to_string(), b.to_string())).collect() }
    pub fn topological_order(&self) -> PyResult<Vec<String>> {
        self.inner.topological_order().map(|ns| ns.iter().map(|&s| s.to_string()).collect()).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    pub fn markov_blanket(&self, variable_name: &str) -> PyResult<Vec<String>> {
        self.inner.markov_blanket(variable_name).map(|ns| ns.iter().map(|&s| s.to_string()).collect()).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    pub fn validate(&self) -> PyValidationResult {
        let errors = self.inner.validate();
        PyValidationResult { is_valid: errors.is_empty(), errors }
    }
    pub fn is_valid(&self) -> bool { self.inner.is_valid() }
    pub fn mark_as_causal(&mut self) { self.inner.mark_as_causal(); }
    pub fn d_separated(&self, a: &str, b: &str, given: Vec<String>) -> PyResult<bool> {
        let given_refs: Vec<&str> = given.iter().map(|s| s.as_str()).collect();
        self.inner.d_separated(a, b, &given_refs).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    pub fn get_states(&self, variable_name: &str) -> PyResult<Vec<String>> {
        let id = self.inner.id_of(variable_name).map_err(|e: crate::core::error::LutufiError| PyValueError::new_err(e.to_string()))?;
        let var = self.inner.variables.get(&id).ok_or_else(|| PyValueError::new_err("Var not found"))?;
        match var.domain() {
            Domain::Discrete { states } => Ok(states.clone()),
            Domain::Binary => Ok(vec!["false".to_string(), "true".to_string()]),
            _ => Err(PyValueError::new_err("Variable domain is not discrete")),
        }
    }
    pub fn is_causal(&self) -> bool { self.inner.is_causal() }
}

impl PyBayesianNetwork {
    fn normalize_cpd_input(&self, variable_name: &str, values: &Bound<'_, PyAny>) -> PyResult<Vec<Vec<f64>>> {
        if let Ok(nested) = values.extract::<Vec<Vec<f64>>>() { return Ok(nested); }
        if let Ok(flat) = values.extract::<Vec<f64>>() { return Ok(flat.into_iter().map(|v| vec![v]).collect()); }
        Err(PyValueError::new_err(format!("Invalid CPT values for {}", variable_name)))
    }

    fn collect_parent_variables(&self, child_name: &str) -> PyResult<Vec<Variable>> {
        let child_id = self.inner.id_of(child_name).map_err(|e: crate::core::error::LutufiError| PyValueError::new_err(e.to_string()))?;
        let parent_ids = self.inner.graph.parents(&child_id);
        let mut parents = Vec::new();
        for id in parent_ids {
            let var = self.inner.variables.get(&id).ok_or_else(|| PyValueError::new_err("Parent mismatch"))?.clone();
            parents.push(var);
        }
        Ok(parents)
    }
}

#[pyclass(name = "QueryResult")]
pub struct PyQueryResult {}

#[pyclass(name = "_RustMarkovRandomField")]
pub struct PyMarkovRandomField {
    inner: crate::core::models::markov_random_field::MarkovRandomField,
}

#[pymethods]
impl PyMarkovRandomField {
    #[new] pub fn new() -> Self { PyMarkovRandomField { inner: crate::core::models::markov_random_field::MarkovRandomField::new() } }
    pub fn add_variable(&mut self, name: &str, domain: Vec<String>) -> PyResult<()> {
        let d = Domain::discrete(domain).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.add_variable(name, d).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }
    pub fn add_edge(&mut self, name1: &str, name2: &str) -> PyResult<()> {
        self.inner.add_edge(name1, name2).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    pub fn nodes(&self) -> Vec<String> { self.inner.nodes().into_iter().map(|s| s.to_string()).collect() }
    pub fn edges(&self) -> Vec<(String, String)> { self.inner.edges().into_iter().map(|(a, b)| (a.to_string(), b.to_string())).collect() }
}

#[pyclass(name = "_RustDynamicBayesianNetwork")]
pub struct PyDynamicBayesianNetwork {
    inner: crate::core::models::dynamic_bayesian_network::DynamicBayesianNetwork,
}

#[pymethods]
impl PyDynamicBayesianNetwork {
    #[new] pub fn new() -> Self { PyDynamicBayesianNetwork { inner: crate::core::models::dynamic_bayesian_network::DynamicBayesianNetwork::new() } }
    pub fn add_variable(&mut self, name: &str, domain: Vec<String>) -> PyResult<()> {
        let d = Domain::discrete(domain).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.add_variable(name, d).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }
    pub fn add_intraslice_edge(&mut self, from: &str, to: &str) -> PyResult<()> {
        self.inner.add_intraslice_edge(from, to).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    pub fn add_interslice_edge(&mut self, from: &str, to: &str) -> PyResult<()> {
        self.inner.add_interslice_edge(from, to).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
