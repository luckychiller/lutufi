use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::ConditionalProbabilityTable,
    graph::DirectedVariableGraph,
    variable::{Variable, VariableId},
};

/// A Bayesian Network — a directed acyclic graph (DAG) where each node
/// is a random variable with a conditional probability table.
///
/// # Invariants maintained at all times:
/// 1. The graph is acyclic. Adding a cycle raises an error immediately.
/// 2. CPT parent sets match the graph parent sets.
/// 3. All CPT columns sum to 1 within 1e-6 tolerance.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BayesianNetwork {
    /// The underlying graph structure.
    pub graph: DirectedVariableGraph,
    /// Map from variable ID to the Variable object.
    pub variables: HashMap<VariableId, Variable>,
    /// Map from variable name to its unique ID.
    name_to_id: HashMap<String, VariableId>,
    /// Map from variable ID to its Conditional Probability Table.
    cpds: HashMap<VariableId, ConditionalProbabilityTable>,
    /// Whether this model is marked as a structural causal model.
    is_causal: bool,
}

impl BayesianNetwork {
    /// Create a new, empty Bayesian Network.
    pub fn new() -> Self {
        BayesianNetwork {
            graph: DirectedVariableGraph::new(),
            variables: HashMap::new(),
            name_to_id: HashMap::new(),
            cpds: HashMap::new(),
            is_causal: false,
        }
    }

    /// Add a new variable to the network.
    ///
    /// # Errors
    /// Returns `LutufiError::VariableAlreadyExists` if a variable with this name already exists.
    pub fn add_variable(
        &mut self,
        name: &str,
        domain: crate::core::domain::Domain,
    ) -> LutufiResult<&Variable> {
        if self.name_to_id.contains_key(name) {
            return Err(LutufiError::VariableAlreadyExists {
                name: name.to_string(),
            });
        }

        let var = Variable::new(name, domain);
        let id = var.id();
        self.graph.add_node(id);
        self.name_to_id.insert(name.to_string(), id);
        self.variables.insert(id, var);

        Ok(self.variables.get(&id).unwrap())
    }

    /// Remove a variable and all its edges from the network.
    ///
    /// # Errors
    /// Returns `LutufiError::VariableNotFound` if the variable doesn't exist.
    pub fn remove_variable(&mut self, name: &str) -> LutufiResult<()> {
        let id = self.id_of(name)?;
        self.graph.remove_node(&id);
        self.variables.remove(&id);
        self.name_to_id.remove(name);
        self.cpds.remove(&id);
        Ok(())
    }

    /// Add a directed edge from `from_name` to `to_name`.
    ///
    /// # Errors
    /// - `LutufiError::VariableNotFound` if either variable doesn't exist.
    /// - `LutufiError::CyclicGraph` if the edge would create a cycle.
    pub fn add_edge(&mut self, from_name: &str, to_name: &str) -> LutufiResult<()> {
        let from_id = self.id_of(from_name)?;
        let to_id = self.id_of(to_name)?;
        self.graph.add_edge(&from_id, &to_id, from_name, to_name)
    }

    /// Remove an edge from the network.
    ///
    /// # Errors
    /// Returns `LutufiError::VariableNotFound` if either variable doesn't exist.
    pub fn remove_edge(&mut self, from_name: &str, to_name: &str) -> LutufiResult<()> {
        let from_id = self.id_of(from_name)?;
        let to_id = self.id_of(to_name)?;
        self.graph.remove_edge(&from_id, &to_id);
        Ok(())
    }

    /// Set the conditional probability table for a variable.
    ///
    /// The CPT's parent set must match the variable's current parents in the graph.
    ///
    /// # Errors
    /// - `LutufiError::VariableNotFound` if the variable doesn't exist.
    /// - `LutufiError::CptParentMismatch` if the CPT's parents don't match the graph.
    pub fn set_cpd(
        &mut self,
        variable_name: &str,
        cpd: ConditionalProbabilityTable,
    ) -> LutufiResult<()> {
        let var_id = self.id_of(variable_name)?;

        // Validate parent match
        let graph_parents: Vec<VariableId> = self.graph.parents(&var_id);
        let cpt_parents: Vec<VariableId> = cpd.parent_ids().to_vec();

        // Sort both for order-independent comparison
        let mut sorted_graph = graph_parents.clone();
        let mut sorted_cpt = cpt_parents.clone();
        sorted_graph.sort_by_key(|id| id.to_string());
        sorted_cpt.sort_by_key(|id| id.to_string());

        if sorted_graph != sorted_cpt {
            let graph_names: Vec<String> = graph_parents.iter()
                .filter_map(|id| self.variables.get(id))
                .map(|v| v.name().to_string())
                .collect();
            let cpt_names: Vec<String> = cpt_parents.iter()
                .filter_map(|id| self.variables.get(id))
                .map(|v| v.name().to_string())
                .collect();

            return Err(LutufiError::CptParentMismatch {
                variable: variable_name.to_string(),
                cpt_parents: cpt_names,
                graph_parents: graph_names,
            });
        }

        self.cpds.insert(var_id, cpd);
        Ok(())
    }

    /// Get the CPT for a variable.
    ///
    /// # Errors
    /// Returns `LutufiError::MissingCpt` if no CPT has been set for this variable.
    pub fn cpd(&self, variable_name: &str) -> LutufiResult<&ConditionalProbabilityTable> {
        let id = self.id_of(variable_name)?;
        self.cpds.get(&id).ok_or_else(|| LutufiError::MissingCpt {
            variable: variable_name.to_string(),
        })
    }

    /// Iterate over all CPTs in the network.
    pub fn cpd_iter(&self) -> impl Iterator<Item = &ConditionalProbabilityTable> {
        self.cpds.values()
    }

    /// All variable names in topological order.
    ///
    /// # Errors
    /// Returns `LutufiError::InternalError` if topological sort fails.
    pub fn topological_order(&self) -> LutufiResult<Vec<&str>> {
        let order = self.graph.topological_order()?;
        Ok(order.iter()
            .filter_map(|id| self.variables.get(id))
            .map(|v| v.name())
            .collect())
    }

    /// The Markov blanket of a variable (parents, children, co-parents).
    ///
    /// # Errors
    /// Returns `LutufiError::VariableNotFound` if the variable doesn't exist.
    pub fn markov_blanket(&self, variable_name: &str) -> LutufiResult<Vec<&str>> {
        let id = self.id_of(variable_name)?;
        let blanket_ids = self.graph.markov_blanket(&id);
        Ok(blanket_ids.iter()
            .filter_map(|id| self.variables.get(id))
            .map(|v| v.name())
            .collect())
    }

    /// Test whether two variables are d-separated given a set of observed variables.
    ///
    /// Uses the Bayes Ball algorithm, which is O(V + E).
    pub fn d_separated(&self, a: &str, b: &str, given: &[&str]) -> LutufiResult<bool> {
        let a_id = self.id_of(a)?;
        let b_id = self.id_of(b)?;

        let given_ids: Result<HashSet<VariableId>, LutufiError> = given.iter()
            .map(|&name| self.id_of(name))
            .collect();
        let given_ids = given_ids?;

        if a_id == b_id {
            return Ok(!given_ids.contains(&a_id));
        }

        // Precompute which nodes have observed descendants
        let mut has_observed_descendant = HashSet::new();
        let mut stack: Vec<VariableId> = given_ids.iter().cloned().collect();
        while let Some(current) = stack.pop() {
            if has_observed_descendant.insert(current) {
                for parent in self.graph.parents(&current) {
                    stack.push(parent);
                }
            }
        }

        #[derive(Hash, PartialEq, Eq, Clone, Copy)]
        enum Direction { FromChild, FromParent }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Initial balls from 'a' as if they came from its (imaginary) children
        queue.push_back((a_id, Direction::FromChild));

        while let Some((u, dir)) = queue.pop_front() {
            if visited.contains(&(u, dir)) {
                continue;
            }
            visited.insert((u, dir));

            if u == b_id {
                return Ok(false); // Found an active path
            }

            let is_observed = given_ids.contains(&u);

            match dir {
                Direction::FromChild => {
                    if is_observed {
                        // Observed node blocks ball from children
                    } else {
                        // Non-observed node passes ball to parents and children
                        for p in self.graph.parents(&u) {
                            queue.push_back((p, Direction::FromChild));
                        }
                        for c in self.graph.children(&u) {
                            queue.push_back((c, Direction::FromParent));
                        }
                    }
                }
                Direction::FromParent => {
                    if is_observed {
                        // Observed node reflects ball back to parents
                        for p in self.graph.parents(&u) {
                            queue.push_back((p, Direction::FromChild));
                        }
                    } else {
                        // Non-observed node passes ball to children
                        for c in self.graph.children(&u) {
                            queue.push_back((c, Direction::FromParent));
                        }
                    }
                }
            }
        }

        Ok(true)
    }

    /// All variable names in this network.
    pub fn nodes(&self) -> Vec<&str> {
        self.variables.values().map(|v| v.name()).collect()
    }

    /// All edges as (from_name, to_name) pairs.
    pub fn edges(&self) -> Vec<(&str, &str)> {
        self.graph.edges().iter()
            .filter_map(|(from_id, to_id)| {
                let from = self.variables.get(from_id)?.name();
                let to = self.variables.get(to_id)?.name();
                Some((from, to))
            })
            .collect()
    }

    /// Get a variable by name.
    ///
    /// # Errors
    /// Returns `LutufiError::VariableNotFound` if the variable doesn't exist.
    pub fn variable(&self, name: &str) -> LutufiResult<&Variable> {
        let id = self.id_of(name)?;
        Ok(self.variables.get(&id).unwrap())
    }

    /// Check whether this network is valid for inference.
    ///
    /// Returns a list of validation errors. An empty list means the network is valid.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for var in self.variables.values() {
            if !self.cpds.contains_key(&var.id()) {
                errors.push(format!(
                    "Variable '{}' has no CPT. Call set_cpd() before running inference.",
                    var.name()
                ));
            }
        }

        errors
    }

    /// Whether this network is currently valid for inference.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Mark this network as a structural causal model.
    /// Required before calling any causal inference operations (do-calculus).
    ///
    /// Only mark as causal if you are confident that edges represent direct
    /// causal mechanisms, not just statistical dependencies.
    pub fn mark_as_causal(&mut self) {
        self.is_causal = true;
    }

    /// Whether this network has been marked as causal.
    pub fn is_causal(&self) -> bool {
        self.is_causal
    }

    /// Perform a query on the network.
    pub fn query(
        &self,
        variables: &[&str],
        evidence: &crate::core::assignment::Assignment,
        algorithm: crate::core::inference::Algorithm,
    ) -> LutufiResult<crate::core::inference::InferenceResult> {
        crate::core::inference::InferenceEngine::query(self, variables, evidence, algorithm)
    }

    /// Convert this network to a Factor Graph representation.
    ///
    /// # Errors
    /// Returns `LutufiError` if conversion fails.
    pub fn to_factor_graph(&self) -> LutufiResult<crate::core::models::factor_graph::FactorGraph> {
        crate::core::models::factor_graph::FactorGraph::from_bayesian_network(self)
    }

    // ── Internal helpers ───────────────────────────────────────────────

    /// Get the unique ID of a variable by its name.
    ///
    /// # Errors
    /// Returns `LutufiError::VariableNotFound` if the variable doesn't exist.
    pub fn id_of(&self, name: &str) -> LutufiResult<VariableId> {
        self.name_to_id.get(name).copied().ok_or_else(|| {
            let available = self.name_to_id.keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            LutufiError::VariableNotFound {
                name: name.to_string(),
                available,
            }
        })
    }
}

impl std::fmt::Display for BayesianNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BayesianNetwork({} variables, {} edges, valid={})",
            self.variables.len(),
            self.graph.edges().len(),
            self.is_valid()
        )
    }
}
