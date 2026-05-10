use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::ConditionalProbabilityTable,
    graph::DirectedVariableGraph,
    variable::{Variable, VariableId},
    models::registry::NetworkRegistry,
};

/// A Bayesian Network — a directed acyclic graph (DAG) where each node
/// is a random variable with a conditional probability table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BayesianNetwork {
    /// The underlying graph structure.
    pub graph: DirectedVariableGraph,
    /// Registry for variables and identifiers.
    registry: NetworkRegistry,
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
            registry: NetworkRegistry::new(),
            cpds: HashMap::new(),
            is_causal: false,
        }
    }

    /// Get a builder for constructing a Bayesian Network.
    pub fn builder() -> crate::core::models::bayesian_network_builder::BayesianNetworkBuilder {
        crate::core::models::bayesian_network_builder::BayesianNetworkBuilder::new()
    }

    /// Add a new variable to the network.
    pub fn add_variable(
        &mut self,
        name: &str,
        domain: crate::core::domain::Domain,
    ) -> LutufiResult<&Variable> {
        let var = self.registry.add_variable(name, domain)?;
        self.graph.add_node(var.id());
        Ok(var)
    }

    /// Remove a variable and all its edges from the network.
    pub fn remove_variable(&mut self, name: &str) -> LutufiResult<()> {
        let id = self.registry.remove_variable(name)?;
        self.graph.remove_node(&id);
        self.cpds.remove(&id);
        Ok(())
    }

    /// Add a directed edge from `from_name` to `to_name`.
    pub fn add_edge(&mut self, from_name: &str, to_name: &str) -> LutufiResult<()> {
        let from_id = self.registry.id_of(from_name)?;
        let to_id = self.registry.id_of(to_name)?;
        self.graph.add_edge(&from_id, &to_id, from_name, to_name)
    }

    /// Remove an edge from the network.
    pub fn remove_edge(&mut self, from_name: &str, to_name: &str) -> LutufiResult<()> {
        let from_id = self.registry.id_of(from_name)?;
        let to_id = self.registry.id_of(to_name)?;
        self.graph.remove_edge(&from_id, &to_id);
        Ok(())
    }

    /// Set the conditional probability table for a variable.
    pub fn set_cpd(
        &mut self,
        variable_name: &str,
        cpd: ConditionalProbabilityTable,
    ) -> LutufiResult<()> {
        let var_id = self.registry.id_of(variable_name)?;

        // Validate parent match
        let graph_parents: Vec<VariableId> = self.graph.parents(&var_id);
        let cpt_parents: Vec<VariableId> = cpd.parent_ids().to_vec();

        let mut sorted_graph = graph_parents.clone();
        let mut sorted_cpt = cpt_parents.clone();
        sorted_graph.sort_by_key(|id| id.to_string());
        sorted_cpt.sort_by_key(|id| id.to_string());

        if sorted_graph != sorted_cpt {
            let graph_names: Vec<String> = graph_parents.iter()
                .filter_map(|id| self.registry.variable(id))
                .map(|v| v.name().to_string())
                .collect();
            let cpt_names: Vec<String> = cpt_parents.iter()
                .filter_map(|id| self.registry.variable(id))
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
    pub fn cpd(&self, variable_name: &str) -> LutufiResult<&ConditionalProbabilityTable> {
        let id = self.registry.id_of(variable_name)?;
        self.cpds.get(&id).ok_or_else(|| LutufiError::MissingCpt {
            variable: variable_name.to_string(),
        })
    }

    /// Access the network's variable registry.
    pub fn registry(&self) -> &NetworkRegistry {
        &self.registry
    }

    /// Access all variables in the network.
    pub fn variables(&self) -> &HashMap<VariableId, Variable> {
        self.registry.variables()
    }

    /// Access all CPDs in the network.
    pub fn cpds(&self) -> &HashMap<VariableId, ConditionalProbabilityTable> {
        &self.cpds
    }

    /// Get a variable by name.
    pub fn variable(&self, name: &str) -> LutufiResult<&Variable> {
        self.registry.variable_by_name(name)
    }

    /// Get the unique ID of a variable by its name.
    pub fn id_of(&self, name: &str) -> LutufiResult<VariableId> {
        self.registry.id_of(name)
    }

    /// All variable names in this network.
    pub fn nodes(&self) -> Vec<&str> {
        self.registry.nodes()
    }

    /// All edges as (from_name, to_name) pairs.
    pub fn edges(&self) -> Vec<(&str, &str)> {
        self.graph.edges().iter()
            .filter_map(|(from_id, to_id)| {
                let from = self.registry.variable(from_id)?.name();
                let to = self.registry.variable(to_id)?.name();
                Some((from, to))
            })
            .collect()
    }

    /// Iterate over all CPTs in the network.
    pub fn cpd_iter(&self) -> impl Iterator<Item = &ConditionalProbabilityTable> {
        self.cpds.values()
    }

    /// All variable names in topological order.
    pub fn topological_order(&self) -> LutufiResult<Vec<&str>> {
        let order = self.graph.topological_order()?;
        Ok(order.iter()
            .filter_map(|id| self.registry.variable(id))
            .map(|v| v.name())
            .collect())
    }

    /// The Markov blanket of a variable (parents, children, co-parents).
    pub fn markov_blanket(&self, variable_name: &str) -> LutufiResult<Vec<&str>> {
        let id = self.registry.id_of(variable_name)?;
        let blanket_ids = self.graph.markov_blanket(&id);
        Ok(blanket_ids.iter()
            .filter_map(|id| self.registry.variable(id))
            .map(|v| v.name())
            .collect())
    }

    /// Test whether two variables are d-separated given a set of observed variables.
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
        queue.push_back((a_id, Direction::FromChild));

        while let Some((u, dir)) = queue.pop_front() {
            if visited.contains(&(u, dir)) { continue; }
            visited.insert((u, dir));
            if u == b_id { return Ok(false); }

            let is_observed = given_ids.contains(&u);
            match dir {
                Direction::FromChild => {
                    if !is_observed {
                        for p in self.graph.parents(&u) { queue.push_back((p, Direction::FromChild)); }
                        for c in self.graph.children(&u) { queue.push_back((c, Direction::FromParent)); }
                    }
                }
                Direction::FromParent => {
                    if is_observed {
                        for p in self.graph.parents(&u) { queue.push_back((p, Direction::FromChild)); }
                    } else {
                        for c in self.graph.children(&u) { queue.push_back((c, Direction::FromParent)); }
                    }
                }
            }
        }
        Ok(true)
    }

    /// Check whether this network is valid for inference.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        for var in self.registry.variables().values() {
            if !self.cpds.contains_key(&var.id()) {
                errors.push(format!("Variable '{}' has no CPT.", var.name()));
            }
        }
        errors
    }

    /// Whether this network is currently valid for inference.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Mark this network as a structural causal model.
    pub fn mark_as_causal(&mut self) {
        self.is_causal = true;
    }

    /// Whether this network has been marked as causal.
    pub fn is_causal(&self) -> bool {
        self.is_causal
    }

    /// Fit the model parameters to a dataset.
    pub fn fit(
        &mut self,
        data: &[HashMap<String, String>],
        options: crate::core::learning::LegacyParameterLearningOptions,
    ) -> LutufiResult<()> {
        crate::core::learning::ParameterEstimator::new(options).fit(self, data)
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

    /// Generate a single sample from the network using forward sampling.
    pub fn sample(&self) -> LutufiResult<crate::core::assignment::Assignment> {
        crate::core::models::sampler::Sampler::forward_sample(self)
    }

    /// Convert this network to a Factor Graph representation.
    pub fn to_factor_graph(&self) -> LutufiResult<crate::core::models::factor_graph::FactorGraph> {
        crate::core::models::factor_graph::FactorGraph::from_bayesian_network(self)
    }

    /// Learn the DAG structure from data using score-based or constraint-based learning.
    ///
    /// # Arguments
    /// * `data` - A slice of rows, each a HashMap mapping variable names to values.
    /// * `method` - The structure learning method to use.
    /// * `forbidden_edges` - Edges that must not appear in the learned structure.
    /// * `required_edges` - Edges that must appear in the learned structure.
    ///
    /// # Returns
    /// A new BayesianNetwork with the learned structure (no parameter estimates).
    pub fn learn_structure(
        data: &[HashMap<String, String>],
        method: StructureLearningMethod,
        forbidden_edges: &[(String, String)],
        required_edges: &[(String, String)],
    ) -> LutufiResult<Self> {
        match method {
            StructureLearningMethod::HillClimbing(options) => {
                let learner = crate::core::learning::ScoreBasedLearner::new(options);
                learner.hill_climbing(data, forbidden_edges, required_edges)
            }
            StructureLearningMethod::GES(options) => {
                let learner = crate::core::learning::ScoreBasedLearner::new(options);
                learner.ges(data)
            }
            StructureLearningMethod::PC(options) => {
                let learner = crate::core::learning::ConstraintBasedLearner::new(options);
                learner.pc_algorithm(data)
            }
            StructureLearningMethod::FCI(options) => {
                let learner = crate::core::learning::ConstraintBasedLearner::new(options);
                let result = learner.fci_algorithm(data)?;
                Ok(result.network)
            }
        }
    }
}

/// Method for structure learning.
#[derive(Debug, Clone)]
pub enum StructureLearningMethod {
    /// Hill climbing search with BIC/BDeu scoring.
    HillClimbing(crate::core::learning::ScoreBasedOptions),
    /// Greedy Equivalence Search.
    GES(crate::core::learning::ScoreBasedOptions),
    /// PC algorithm (constraint-based).
    PC(crate::core::learning::ConstraintBasedOptions),
    /// FCI algorithm (constraint-based, handles latent confounders).
    FCI(crate::core::learning::ConstraintBasedOptions),
}

impl std::fmt::Display for BayesianNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BayesianNetwork({} variables, {} edges, valid={})",
            self.registry.len(),
            self.graph.edges().len(),
            self.is_valid()
        )
    }
}
