use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use crate::core::{
    assignment::Assignment,
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

    /// Apply Pearl's do-operator: remove incoming edges to each intervened
    /// variable and fix it to the given value, returning the mutilated
    /// interventional network. Requires mark_as_causal() to have been called.
    pub fn do_operator(&self, intervention: HashMap<String, String>) -> PyResult<Self> {
        self.require_causal("do_operator")?;
        let cm = crate::core::models::CausalModel::new(self.inner.clone());
        let assign = self.assignment_from_map(&intervention)?;
        cm.do_operator(&assign)
            .map(|inner| PyBayesianNetwork { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Compute P(targets | do(interventions)): marginal distributions of the
    /// target variables in the mutilated network. Returns a dict mapping each
    /// target name to its marginal probability vector (in state order).
    pub fn causal_query(
        &self,
        py: Python<'_>,
        targets: Vec<String>,
        interventions: HashMap<String, String>,
    ) -> PyResult<HashMap<String, PyObject>> {
        self.require_causal("causal_query")?;
        let assign = self.assignment_from_map(&interventions)?;
        let result = py.allow_threads(|| {
            let cm = crate::core::models::CausalModel::new(self.inner.clone());
            let target_refs: Vec<&str> = targets.iter().map(|s| s.as_str()).collect();
            cm.causal_query(&target_refs, &assign)
        }).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Self::pack_marginals(py, &targets, &result)
    }

    /// Counterfactual query: condition on `observed` evidence, apply
    /// `intervention`, and return marginals of the `query` variables.
    pub fn counterfactual(
        &self,
        py: Python<'_>,
        observed: HashMap<String, String>,
        intervention: HashMap<String, String>,
        query: Vec<String>,
    ) -> PyResult<HashMap<String, PyObject>> {
        self.require_causal("counterfactual")?;
        let obs = self.assignment_from_map(&observed)?;
        let interv = self.assignment_from_map(&intervention)?;
        let result = py.allow_threads(|| {
            let cm = crate::core::models::CausalModel::new(self.inner.clone());
            let query_refs: Vec<&str> = query.iter().map(|s| s.as_str()).collect();
            cm.counterfactual(&obs, &interv, &query_refs)
        }).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Self::pack_marginals(py, &query, &result)
    }

    /// Probability of necessity: P(outcome would not have occurred without
    /// the treatment, given it occurred under the treatment).
    pub fn probability_of_necessity(
        &self,
        py: Python<'_>,
        outcome: &str,
        outcome_value: &str,
        treatment: &str,
        treatment_value: &str,
        reference_value: &str,
    ) -> PyResult<f64> {
        self.require_causal("probability_of_necessity")?;
        py.allow_threads(|| {
            let cm = crate::core::models::CausalModel::new(self.inner.clone());
            cm.probability_of_necessity(outcome, outcome_value, treatment, treatment_value, reference_value)
        }).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Probability of sufficiency: P(outcome would have occurred under the
    /// treatment, given it did not occur under the reference condition).
    pub fn probability_of_sufficiency(
        &self,
        py: Python<'_>,
        outcome: &str,
        outcome_value: &str,
        treatment: &str,
        treatment_value: &str,
        reference_value: &str,
    ) -> PyResult<f64> {
        self.require_causal("probability_of_sufficiency")?;
        py.allow_threads(|| {
            let cm = crate::core::models::CausalModel::new(self.inner.clone());
            cm.probability_of_sufficiency(outcome, outcome_value, treatment, treatment_value, reference_value)
        }).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Run the ID algorithm: is P(targets | do(interventions)) identifiable
    /// from observational data given this graph? Returns (identifiable,
    /// formula_or_reason).
    pub fn identify(&self, targets: Vec<String>, interventions: Vec<String>) -> PyResult<(bool, String)> {
        self.require_causal("identify")?;
        let cm = crate::core::models::CausalModel::new(self.inner.clone());
        let target_refs: Vec<&str> = targets.iter().map(|s| s.as_str()).collect();
        let interv_refs: Vec<&str> = interventions.iter().map(|s| s.as_str()).collect();
        match cm.identify(&target_refs, &interv_refs).map_err(|e| PyValueError::new_err(e.to_string()))? {
            crate::core::models::causal_model::types::IdentificationResult::Identifiable(f) => Ok((true, f.formula)),
            crate::core::models::causal_model::types::IdentificationResult::NotIdentifiable(reason) => Ok((false, reason)),
        }
    }

    /// Serialize this network (structure + CPDs) to an LMF JSON file at `path`.
    pub fn save(&self, path: &str) -> PyResult<()> {
        self.inner.save_lmf(path).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Load a Bayesian network (structure + CPDs) from an LMF JSON file at `path`.
    #[staticmethod]
    pub fn load(path: &str) -> PyResult<Self> {
        BayesianNetwork::load_lmf(path)
            .map(|inner| PyBayesianNetwork { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Serialize this network (structure + CPDs) to an LMF JSON string.
    pub fn to_lmf_json(&self) -> PyResult<String> {
        let doc = crate::core::io::lmf::LmfDocument::from_bayesian_network(&self.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        serde_json::to_string_pretty(&doc)
            .map_err(|e| PyValueError::new_err(format!("JSON serialization failed: {}", e)))
    }

    /// Deserialize a Bayesian network (structure + CPDs) from an LMF JSON string.
    #[staticmethod]
    pub fn from_lmf_json(json: &str) -> PyResult<Self> {
        BayesianNetwork::load_lmf_from_str(json)
            .map(|inner| PyBayesianNetwork { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

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
    /// Enforce the mark_as_causal() contract before any causal operation.
    /// (CausalModel::new force-marks its network, so without this check the
    /// safety gate documented on mark_as_causal() would be silently skipped.)
    fn require_causal(&self, operation: &str) -> PyResult<()> {
        if self.inner.is_causal() {
            Ok(())
        } else {
            Err(PyValueError::new_err(format!(
                "Cannot call '{}' on a non-causal model. Call mark_as_causal() first.",
                operation
            )))
        }
    }

    /// Build an Assignment from a Python dict of {variable_name: state_value}.
    fn assignment_from_map(&self, map: &HashMap<String, String>) -> PyResult<Assignment> {
        let mut assign = Assignment::new();
        for (name, val) in map {
            let id = self.inner.id_of(name).map_err(|e| PyValueError::new_err(e.to_string()))?;
            assign.set(id, val.as_str());
        }
        Ok(assign)
    }

    /// Pack an InferenceResult's per-variable marginals into {name: Vec<f64>}.
    fn pack_marginals(
        py: Python<'_>,
        names: &[String],
        result: &crate::core::inference::InferenceResult,
    ) -> PyResult<HashMap<String, PyObject>> {
        let mut out = HashMap::new();
        for name in names {
            let factor = result.distributions.get(name).ok_or_else(|| {
                PyValueError::new_err(format!("No marginal returned for '{}'", name))
            })?;
            let values: Vec<f64> = (0..factor.scope().num_entries())
                .map(|i| factor.value_at(i))
                .collect();
            out.insert(name.clone(), values.into_py(py));
        }
        Ok(out)
    }

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
