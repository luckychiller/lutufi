use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use crate::core::{
    domain::Domain,
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    variable::Variable,
};

/// Result of model validation.
///
/// Contains whether the model is valid and a list of error messages.
#[pyclass(name = "ValidationResult")]
pub struct PyValidationResult {
    /// Whether the model passed validation.
    #[pyo3(get)]
    pub is_valid: bool,
    /// List of validation error messages (empty if valid).
    #[pyo3(get)]
    pub errors: Vec<String>,
}

#[pymethods]
impl PyValidationResult {
    /// Return a string representation of this ValidationResult.
    pub fn __repr__(&self) -> String {
        if self.is_valid { "ValidationResult(valid=True)".to_string() }
        else { format!("ValidationResult(valid=False, errors={:?})", self.errors) }
    }
    /// Return True if the model is valid, False otherwise.
    pub fn __bool__(&self) -> bool { self.is_valid }
}

/// A Bayesian network model.
///
/// Represents a directed acyclic graph (DAG) where nodes are random variables
/// and edges represent conditional dependencies.
#[pyclass(name = "_RustBayesianNetwork")]
pub struct PyBayesianNetwork {
    pub(crate) inner: BayesianNetwork,
}

#[pymethods]
impl PyBayesianNetwork {
    /// Create an empty Bayesian network.
    #[new]
    pub fn new() -> Self {
        PyBayesianNetwork {
            inner: BayesianNetwork::new(),
        }
    }

    /// Add a discrete variable to the network.
    pub fn add_variable(&mut self, name: &str, domain: Vec<String>) -> PyResult<()> {
        let d = Domain::discrete(domain).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.add_variable(name, d).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Add a directed edge between two variables.
    pub fn add_edge(&mut self, from_name: &str, to_name: &str) -> PyResult<()> {
        self.inner.add_edge(from_name, to_name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Remove a directed edge between two variables.
    pub fn remove_edge(&mut self, from_name: &str, to_name: &str) -> PyResult<()> {
        self.inner.remove_edge(from_name, to_name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Set the conditional probability table (CPT) for a variable.
    pub fn set_cpd(&mut self, variable_name: &str, values: &Bound<'_, PyAny>) -> PyResult<()> {
        let normalized = self.normalize_cpd_input(variable_name, values)?;
        let child = self.inner.variable(variable_name).map_err(|e| PyValueError::new_err(e.to_string()))?.clone();
        let parent_vars = self.collect_parent_variables(variable_name)?;
        let parent_refs: Vec<&Variable> = parent_vars.iter().collect();
        let cpd = ConditionalProbabilityTable::from_values(&child, &parent_refs, normalized).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.set_cpd(variable_name, cpd).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Get the conditional probability table (CPT) for a variable.
    pub fn get_cpd(&self, variable_name: &str) -> PyResult<HashMap<String, Vec<f64>>> {
        self.inner.cpd(variable_name).map(|cpd| {
            let mut result = HashMap::new();
            let factor = cpd.as_factor();
            let values: Vec<f64> = (0..factor.scope().num_entries()).map(|i| factor.value_at(i)).collect();
            result.insert("values".to_string(), values);
            result
        }).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Return the list of variable names in the network.
    pub fn nodes(&self) -> Vec<String> { self.inner.nodes().iter().map(|&s| s.to_string()).collect() }
    /// Return the list of directed edges as (parent, child) pairs.
    pub fn edges(&self) -> Vec<(String, String)> { self.inner.edges().iter().map(|(a, b)| (a.to_string(), b.to_string())).collect() }
    /// Return a topological ordering of the variables.
    pub fn topological_order(&self) -> PyResult<Vec<String>> {
        self.inner.topological_order().map(|ns| ns.iter().map(|&s| s.to_string()).collect()).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    /// Return the Markov blanket of a variable (parents, children, spouses).
    pub fn markov_blanket(&self, variable_name: &str) -> PyResult<Vec<String>> {
        self.inner.markov_blanket(variable_name).map(|ns| ns.iter().map(|&s| s.to_string()).collect()).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    /// Validate the network structure and parameters.
    pub fn validate(&self) -> PyValidationResult {
        let errors = self.inner.validate();
        PyValidationResult { is_valid: errors.is_empty(), errors }
    }
    /// Check whether the network is valid (structure + parameters).
    pub fn is_valid(&self) -> bool { self.inner.is_valid() }
    /// Mark this network as a causal model.
    pub fn mark_as_causal(&mut self) { self.inner.mark_as_causal(); }
    /// Check if two nodes are d-separated given a set of evidence variables.
    pub fn d_separated(&self, a: &str, b: &str, given: Vec<String>) -> PyResult<bool> {
        let given_refs: Vec<&str> = given.iter().map(|s| s.as_str()).collect();
        self.inner.d_separated(a, b, &given_refs).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    /// Return the state names for a discrete variable.
    pub fn get_states(&self, variable_name: &str) -> PyResult<Vec<String>> {
        let id = self.inner.id_of(variable_name).map_err(|e: crate::core::error::LutufiError| PyValueError::new_err(e.to_string()))?;
        let var = self.inner.variables().get(&id).ok_or_else(|| PyValueError::new_err("Var not found"))?;
        match var.domain() {
            Domain::Discrete { states } => Ok(states.clone()),
            Domain::Binary => Ok(vec!["false".to_string(), "true".to_string()]),
            _ => Err(PyValueError::new_err("Variable domain is not discrete")),
        }
    }
    /// Return whether this network is marked as a causal model.
    pub fn is_causal(&self) -> bool { self.inner.is_causal() }

    /// Return a shallow copy of this Bayesian network.
    pub fn __copy__(&self) -> Self {
        PyBayesianNetwork { inner: self.inner.clone() }
    }

    /// Return a deep copy of this Bayesian network.
    pub fn __deepcopy__(&self, _memo: &Bound<'_, PyAny>) -> Self {
        PyBayesianNetwork { inner: self.inner.clone() }
    }
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
            let var: &Variable = self.inner.variables().get(&id).ok_or_else(|| PyValueError::new_err("Parent mismatch"))?;
            parents.push(var.clone());
        }
        Ok(parents)
    }
}

/// Result of a query against an inference engine.
#[pyclass(name = "QueryResult")]
pub struct PyQueryResult {}

/// An undirected graphical model (Markov random field).
#[pyclass(name = "_RustMarkovRandomField")]
pub struct PyMarkovRandomField {
    inner: crate::core::models::markov_random_field::MarkovRandomField,
}

#[pymethods]
impl PyMarkovRandomField {
    /// Create an empty Markov random field.
    #[new] pub fn new() -> Self { PyMarkovRandomField { inner: crate::core::models::markov_random_field::MarkovRandomField::new() } }
    /// Add a discrete variable to the MRF.
    pub fn add_variable(&mut self, name: &str, domain: Vec<String>) -> PyResult<()> {
        let d = Domain::discrete(domain).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.add_variable(name, d).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }
    /// Add an undirected edge between two variables.
    pub fn add_edge(&mut self, name1: &str, name2: &str) -> PyResult<()> {
        self.inner.add_edge(name1, name2).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    /// Return the list of variable names in the MRF.
    pub fn nodes(&self) -> Vec<String> { self.inner.nodes().into_iter().map(|s| s.to_string()).collect() }
    /// Return the list of undirected edges as (node1, node2) pairs.
    pub fn edges(&self) -> Vec<(String, String)> { self.inner.edges().into_iter().map(|(a, b)| (a.to_string(), b.to_string())).collect() }
}

/// A dynamic Bayesian network (DBN) for time-series modeling.
#[pyclass(name = "_RustDynamicBayesianNetwork")]
pub struct PyDynamicBayesianNetwork {
    inner: crate::core::models::dynamic_bayesian_network::DynamicBayesianNetwork,
}

#[pymethods]
impl PyDynamicBayesianNetwork {
    /// Create an empty dynamic Bayesian network.
    #[new] pub fn new() -> Self { PyDynamicBayesianNetwork { inner: crate::core::models::dynamic_bayesian_network::DynamicBayesianNetwork::new() } }
    /// Add a discrete variable to the DBN.
    pub fn add_variable(&mut self, name: &str, domain: Vec<String>) -> PyResult<()> {
        let d = Domain::discrete(domain).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.add_variable(name, d).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }
    /// Add an edge within the same time slice.
    pub fn add_intraslice_edge(&mut self, from: &str, to: &str) -> PyResult<()> {
        self.inner.add_intraslice_edge(from, to).map_err(|e| PyValueError::new_err(e.to_string()))
    }
    /// Add an edge from the previous time slice to the current time slice.
    pub fn add_interslice_edge(&mut self, from: &str, to: &str) -> PyResult<()> {
        self.inner.add_interslice_edge(from, to).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
