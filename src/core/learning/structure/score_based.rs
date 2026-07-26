//! Score-based structure learning.
//!
//! Implements BIC, BDeu scoring and hill climbing search.

use std::collections::HashMap;
use rayon::prelude::*;
use crate::core::{
    error::{LutufiError, LutufiResult},
    models::bayesian_network::BayesianNetwork,
    variable::{Variable, VariableId},
};

/// Type of score to use for structure learning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreType {
    /// Bayesian Information Criterion.
    BIC,
    /// Bayesian Dirichlet equivalent uniform score.
    BDeu,
}

/// Options for score-based learning.
#[derive(Debug, Clone)]
pub struct ScoreBasedOptions {
    /// Score function (BIC or BDeu).
    pub score_type: ScoreType,
    /// Equivalent sample size for BDeu score.
    pub equivalent_sample_size: f64,
    /// Number of random restarts for hill climbing.
    pub n_restarts: usize,
    /// Maximum number of local modifications.
    pub max_iter: usize,
}

impl Default for ScoreBasedOptions {
    fn default() -> Self {
        Self {
            score_type: ScoreType::BIC,
            equivalent_sample_size: 10.0,
            n_restarts: 5,
            max_iter: 1000,
        }
    }
}

/// Learner for score-based structure discovery.
pub struct ScoreBasedLearner {
    options: ScoreBasedOptions,
}

impl ScoreBasedLearner {
    /// Create a new ScoreBasedLearner.
    pub fn new(options: ScoreBasedOptions) -> Self {
        Self { options }
    }

    /// Learn the DAG structure using Hill Climbing search.
    pub fn hill_climbing(
        &self,
        data: &[HashMap<String, String>],
        forbidden_edges: &[(String, String)],
        required_edges: &[(String, String)],
    ) -> LutufiResult<BayesianNetwork> {
        if data.is_empty() {
            return Err(LutufiError::InternalError { message: "Cannot learn structure from empty data".to_string() });
        }

        let node_names: Vec<String> = data[0].keys().cloned().collect();
        let mut best_overall_model = BayesianNetwork::new();
        let mut best_overall_score = f64::NEG_INFINITY;

        for _restart in 0..self.options.n_restarts {
            let mut current_model = BayesianNetwork::new();
            // 1. Initialize variables from data
            for name in &node_names {
                let mut states = std::collections::HashSet::new();
                for row in data {
                    if let Some(val) = row.get(name) {
                        states.insert(val.clone());
                    }
                }
                let mut sorted_states: Vec<_> = states.into_iter().collect();
                sorted_states.sort();
                current_model.add_variable(name, crate::core::domain::Domain::discrete(sorted_states)?)?;
            }

            // 2. Initialize with required edges
            for (from, to) in required_edges {
                current_model.add_edge(from, to)?;
            }

            // 3. Cache local scores
            let mut local_scores = HashMap::new();
            for node in current_model.nodes() {
                local_scores.insert(node.to_string(), self.local_score(&current_model, node, data)?);
            }
            let mut current_score: f64 = local_scores.values().sum();

            // 4. Search loop
            for _iter in 0..self.options.max_iter {
                let mut best_delta = 0.0;
                let mut best_op = None;

                let nodes: Vec<String> = current_model.nodes().iter().map(|&s| s.to_string()).collect();

                // Phase A (serial, cheap): enumerate the legal moves. Legality
                // depends on acyclicity, which is a property of the whole graph,
                // so it is checked here against the single shared model.
                //
                // Each candidate carries the hypothetical parent sets its score
                // depends on, so Phase B needs no access to a mutable graph.
                let mut candidates: Vec<Candidate> = Vec::new();
                for u_name in &nodes {
                    for v_name in &nodes {
                        if u_name == v_name { continue; }

                        let u_id = current_model.id_of(u_name)?;
                        let v_id = current_model.id_of(v_name)?;
                        let edge_exists = current_model.graph.edges().contains(&(u_id, v_id));

                        // Case 1: add U -> V.
                        if !edge_exists
                            && !forbidden_edges.contains(&(u_name.clone(), v_name.clone()))
                            && !self.would_create_cycle(&current_model, u_name, v_name)?
                        {
                            let mut v_parents = current_model.graph.parents(&v_id);
                            v_parents.push(u_id);
                            candidates.push(Candidate {
                                op: StructureOp::Add(u_name.clone(), v_name.clone()),
                                rescored: vec![(v_name.clone(), v_parents)],
                            });
                        }

                        // Case 2: remove U -> V.
                        if edge_exists
                            && !required_edges.contains(&(u_name.clone(), v_name.clone()))
                        {
                            let v_parents: Vec<VariableId> = current_model.graph
                                .parents(&v_id).into_iter().filter(|p| *p != u_id).collect();
                            candidates.push(Candidate {
                                op: StructureOp::Remove(u_name.clone(), v_name.clone()),
                                rescored: vec![(v_name.clone(), v_parents)],
                            });
                        }

                        // Case 3: reverse U -> V into V -> U. Acyclicity must be
                        // judged with the original edge already gone, so the
                        // graph is temporarily mutated and restored here — still
                        // serial, still cheap, and no scoring happens inside.
                        if edge_exists
                            && !required_edges.contains(&(u_name.clone(), v_name.clone()))
                            && !forbidden_edges.contains(&(v_name.clone(), u_name.clone()))
                        {
                            current_model.graph.remove_edge(&u_id, &v_id);
                            let reversal_is_acyclic =
                                !self.would_create_cycle(&current_model, v_name, u_name)?;
                            current_model.graph.add_edge(&u_id, &v_id, u_name, v_name)?;

                            if reversal_is_acyclic {
                                let mut u_parents = current_model.graph.parents(&u_id);
                                u_parents.push(v_id);
                                let v_parents: Vec<VariableId> = current_model.graph
                                    .parents(&v_id).into_iter().filter(|p| *p != u_id).collect();
                                candidates.push(Candidate {
                                    op: StructureOp::Reverse(u_name.clone(), v_name.clone()),
                                    rescored: vec![
                                        (u_name.clone(), u_parents),
                                        (v_name.clone(), v_parents),
                                    ],
                                });
                            }
                        }
                    }
                }

                // Phase B (parallel, expensive): score every candidate. Each
                // scoring pass scans the dataset, which is what dominates the
                // search; candidates are independent given the immutable model.
                let scored: Vec<LutufiResult<f64>> = candidates
                    .par_iter()
                    .map(|candidate| {
                        let mut delta = 0.0;
                        for (node, parents) in &candidate.rescored {
                            let old = local_scores[node];
                            let new = self.local_score_with_parents(
                                &current_model, node, parents, data)?;
                            delta += new - old;
                        }
                        Ok(delta)
                    })
                    .collect();

                // Phase C (serial): pick the best. Ties resolve to the earliest
                // candidate in enumeration order, so the search path does not
                // depend on thread scheduling.
                for (candidate, delta) in candidates.into_iter().zip(scored) {
                    let delta = delta?;
                    if delta > best_delta {
                        best_delta = delta;
                        best_op = Some(candidate.op);
                    }
                }

                if let Some(op) = best_op {
                    match op {
                        StructureOp::Add(u, v) => {
                            current_model.add_edge(&u, &v)?;
                            local_scores.insert(v.clone(), self.local_score(&current_model, &v, data)?);
                        }
                        StructureOp::Remove(u, v) => {
                            let u_id = current_model.id_of(&u)?;
                            let v_id = current_model.id_of(&v)?;
                            current_model.graph.remove_edge(&u_id, &v_id);
                            local_scores.insert(v.clone(), self.local_score(&current_model, &v, data)?);
                        }
                        StructureOp::Reverse(u, v) => {
                            let u_id = current_model.id_of(&u)?;
                            let v_id = current_model.id_of(&v)?;
                            current_model.graph.remove_edge(&u_id, &v_id);
                            current_model.add_edge(&v, &u)?;
                            local_scores.insert(u.clone(), self.local_score(&current_model, &u, data)?);
                            local_scores.insert(v.clone(), self.local_score(&current_model, &v, data)?);
                        }
                    }
                    current_score += best_delta;
                } else {
                    break; // No improvement
                }
            }

            if current_score > best_overall_score {
                best_overall_score = current_score;
                best_overall_model = current_model;
            }
        }

        Ok(best_overall_model)
    }

    /// Learn the DAG structure using Greedy Equivalence Search (GES).
    /// 
    /// This implementation uses a two-phase greedy search on the space of DAGs:
    /// Phase 1 (Forward): Greedily add edges that most improve the score.
    /// Phase 2 (Backward): Greedily remove edges that most improve the score.
    pub fn ges(
        &self,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<BayesianNetwork> {
        if data.is_empty() {
            return Err(LutufiError::InternalError { message: "Cannot learn structure from empty data".to_string() });
        }

        let node_names: Vec<String> = data[0].keys().cloned().collect();
        let mut current_model = BayesianNetwork::new();
        for name in &node_names {
            let mut states = std::collections::HashSet::new();
            for row in data {
                if let Some(val) = row.get(name) {
                    states.insert(val.clone());
                }
            }
            let mut sorted_states: Vec<_> = states.into_iter().collect();
            sorted_states.sort();
            current_model.add_variable(name, crate::core::domain::Domain::discrete(sorted_states)?)?;
        }

        let mut local_scores = HashMap::new();
        for node in current_model.nodes() {
            local_scores.insert(node.to_string(), self.local_score(&current_model, node, data)?);
        }

        // Phase 1: Forward Search (Add Edoc)
        loop {
            let mut best_delta = 0.0;
            let mut best_edge = None;
            let nodes: Vec<String> = current_model.nodes().iter().map(|&s| s.to_string()).collect();

            for u in &nodes {
                for v in &nodes {
                    if u == v { continue; }
                    let u_id = current_model.id_of(u)?;
                    let v_id = current_model.id_of(v)?;

                    if !current_model.graph.edges().contains(&(u_id, v_id))
                        && !self.would_create_cycle(&current_model, u, v)? {
                            let old_v_score = local_scores[v];
                            current_model.graph.add_edge(&u_id, &v_id, u, v)?;
                            let new_v_score = self.local_score(&current_model, v, data)?;
                            current_model.graph.remove_edge(&u_id, &v_id); // Backtrack

                            let delta = new_v_score - old_v_score;
                            if delta > best_delta {
                                best_delta = delta;
                                best_edge = Some((u.clone(), v.clone()));
                            }
                        }
                }
            }

            if let Some((u, v)) = best_edge {
                current_model.add_edge(&u, &v)?;
                local_scores.insert(v.clone(), self.local_score(&current_model, &v, data)?);
            } else {
                break;
            }
        }

        // Phase 2: Backward Search (Remove Edges)
        loop {
            let mut best_delta = 0.0;
            let mut best_edge = None;
            let edges = current_model.graph.edges().to_vec();

            for (u_id, v_id) in edges {
                let u_name = current_model.variables().get(&u_id)
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Variable {:?} not found in model", u_id),
                    })?.name().to_string();
                let v_name = current_model.variables().get(&v_id)
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Variable {:?} not found in model", v_id),
                    })?.name().to_string();

                let old_v_score = local_scores[&v_name];
                current_model.graph.remove_edge(&u_id, &v_id);
                let new_v_score = self.local_score(&current_model, &v_name, data)?;
                current_model.graph.add_edge(&u_id, &v_id, &u_name, &v_name)?;

                let delta = new_v_score - old_v_score;
                if delta > best_delta {
                    best_delta = delta;
                    best_edge = Some((u_id, v_id));
                }
            }

            if let Some((u_id, v_id)) = best_edge {
                current_model.graph.remove_edge(&u_id, &v_id);
                let v_name = current_model.variables().get(&v_id)
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Variable {:?} not found in model", v_id),
                    })?.name().to_string();
                local_scores.insert(v_name.clone(), self.local_score(&current_model, &v_name, data)?);
            } else {
                break;
            }
        }

        Ok(current_model)
    }

    /// Compute the total score for a Bayesian Network.
    pub fn total_score(
        &self,
        model: &BayesianNetwork,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<f64> {
        let mut score = 0.0;
        for node in model.nodes() {
            score += self.local_score(model, node, data)?;
        }
        Ok(score)
    }

    /// Compute the local score for a node given its parents.
    pub fn local_score(
        &self,
        model: &BayesianNetwork,
        node_name: &str,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<f64> {
        let child_id = model.id_of(node_name)?;
        let parents = model.graph.parents(&child_id);
        self.local_score_with_parents(model, node_name, &parents, data)
    }

    /// Score a node against a *hypothetical* parent set.
    ///
    /// Scoring a candidate edge does not require the edge to exist in the graph —
    /// only that the parent set be known. Taking parents explicitly lets many
    /// candidate moves be scored concurrently from one immutable model, instead
    /// of each one mutating the shared graph and undoing it afterwards.
    pub fn local_score_with_parents(
        &self,
        model: &BayesianNetwork,
        node_name: &str,
        parents: &[VariableId],
        data: &[HashMap<String, String>],
    ) -> LutufiResult<f64> {
        let child_id = model.id_of(node_name)?;
        let parents = parents.to_vec();

        let counts = self.get_counts(model, node_name, &parents, data)?;
        let n_total = data.len() as f64;
        
        match self.options.score_type {
            ScoreType::BIC => {
                let mut log_l = 0.0;
                let child_var = model.variables().get(&child_id)
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Child variable {:?} not found", child_id),
                    })?;
                let r = child_var.domain().size()
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Child variable {} has no discrete domain size", child_var.name()),
                    })?;
                let q = counts.len() / r;
                
                for j in 0..q {
                    let mut n_j = 0.0;
                    for i in 0..r {
                        n_j += counts[j * r + i];
                    }
                    if n_j > 0.0 {
                        for i in 0..r {
                            let n_ij = counts[j * r + i];
                            if n_ij > 0.0 {
                                log_l += n_ij * (n_ij / n_j).ln();
                            }
                        }
                    }
                }
                
                let k = ((r - 1) * q) as f64;
                Ok(log_l - (k / 2.0) * n_total.ln())
            }
            ScoreType::BDeu => {
                let child_var = model.variables().get(&child_id)
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Child variable {:?} not found", child_id),
                    })?;
                let r = child_var.domain().size()
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Child variable {} has no discrete domain size", child_var.name()),
                    })?;
                let q = counts.len() / r;
                let alpha = self.options.equivalent_sample_size;
                let alpha_ij = alpha / (q as f64);
                let alpha_ijk = alpha / ((q * r) as f64);
                
                let mut score = 0.0;
                for j in 0..q {
                    let mut n_j = 0.0;
                    for i in 0..r {
                        n_j += counts[j * r + i];
                    }
                    
                    score += statrs::function::gamma::ln_gamma(alpha_ij) - statrs::function::gamma::ln_gamma(alpha_ij + n_j);
                    for i in 0..r {
                        let n_ij = counts[j * r + i];
                        score += statrs::function::gamma::ln_gamma(alpha_ijk + n_ij) - statrs::function::gamma::ln_gamma(alpha_ijk);
                    }
                }
                Ok(score)
            }
        }
    }

    /// Helper to compute joint counts for a node and its parents.
    fn get_counts(
        &self,
        model: &BayesianNetwork,
        node_name: &str,
        parents: &[VariableId],
        data: &[HashMap<String, String>],
    ) -> LutufiResult<Vec<f64>> {
        let child_id = model.id_of(node_name)?;
        let child_var = model.variables().get(&child_id)
            .ok_or_else(|| LutufiError::InternalError {
                message: format!("Child variable {:?} not found", child_id),
            })?;
        
        let mut sorted_parents = parents.to_vec();
        sorted_parents.sort_by_key(|p| {
            let p_var = model.variables().get(p)
                .ok_or_else(|| LutufiError::InternalError {
                    message: format!("Parent variable {:?} not found in model", p),
                }).map(|v| v.name().to_string());
            p_var.unwrap_or_default()
        });
        
        let mut ordered_vars: Vec<&Variable> = sorted_parents.iter().map(|id| {
            model.variables().get(id)
                .ok_or_else(|| LutufiError::InternalError {
                    message: format!("Variable {:?} not found in model for ordering", id),
                })
        }).collect::<LutufiResult<Vec<_>>>()?;
        ordered_vars.push(child_var);
        
        let scope = crate::core::factor::Scope::new(ordered_vars.clone());
        let mut counts = vec![0.0; scope.num_entries()];
        
        let parent_sizes: Vec<usize> = sorted_parents.iter().map(|id| {
            model.variables().get(id)
                .ok_or_else(|| LutufiError::InternalError {
                    message: format!("Parent variable {:?} not found", id),
                }).and_then(|v| v.domain().size()
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Parent variable {} has no discrete domain size", v.name()),
                    }))
        }).collect::<LutufiResult<Vec<_>>>()?;
        let child_domain_size = child_var.domain().size()
            .ok_or_else(|| LutufiError::InternalError {
                message: format!("Child variable {} has no discrete domain size", child_var.name()),
            })?;

        for row in data {
            let mut missing = false;
            let mut p_indices = Vec::with_capacity(sorted_parents.len());
            for &p_id in &sorted_parents {
                let p_var = model.variables().get(&p_id)
                    .ok_or_else(|| LutufiError::InternalError {
                        message: format!("Parent variable {:?} not found", p_id),
                    })?;
                if let Some(val) = row.get(p_var.name()) {
                    if let Some(idx) = p_var.domain().index_of(val) {
                        p_indices.push(idx);
                    } else { missing = true; break; }
                } else { missing = true; break; }
            }
            if missing { continue; }
            
            if let Some(val) = row.get(node_name) {
                if let Some(c_idx) = child_var.domain().index_of(val) {
                    let mut p_config = 0;
                    let mut p_stride = 1;
                    for i in (0..p_indices.len()).rev() {
                        p_config += p_indices[i] * p_stride;
                        p_stride *= parent_sizes[i];
                    }
                    
                    let flat_idx = p_config * child_domain_size + c_idx;
                    counts[flat_idx] += 1.0;
                }
            }
        }
        
        Ok(counts)
    }

    fn would_create_cycle(&self, model: &BayesianNetwork, from: &str, to: &str) -> LutufiResult<bool> {
        let to_id = model.id_of(to)?;
        let from_id = model.id_of(from)?;
        
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![to_id];
        
        while let Some(current) = stack.pop() {
            if current == from_id { return Ok(true); }
            if visited.insert(current) {
                for child in model.graph.children(&current) {
                    stack.push(child);
                }
            }
        }
        Ok(false)
    }
}

enum StructureOp {
    Add(String, String),
    Remove(String, String),
    Reverse(String, String),
}

/// A legal hill-climbing move together with everything needed to score it.
///
/// `rescored` lists the nodes whose local score the move changes, each paired
/// with the parent set it would have afterwards. Carrying the hypothetical
/// parents here is what lets scoring run against an immutable model.
struct Candidate {
    op: StructureOp,
    rescored: Vec<(String, Vec<VariableId>)>,
}
