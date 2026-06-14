use std::collections::{HashMap, HashSet};
use crate::core::error::{LutufiError, LutufiResult};
use crate::core::learning::data_processor::DataProcessor;
use super::types::{
    ConstraintBasedOptions, FciResult, FciSkeleton, PagEdgeMark, PagGraph,
};
use super::skeleton::SkeletonDiscovery;
use super::orientation::VStructureOrientator;
use super::meeks::MeeksRuleApplier;
use super::network::ConstrainedNetworkBuilder;

/// Runs constraint-based causal structure learning algorithms (PC, FCI).
///
/// Uses conditional independence tests to discover the structure of a
/// Bayesian network from observational data.
pub struct ConstraintBasedLearner {
    pub(crate) options: ConstraintBasedOptions,
}

impl ConstraintBasedLearner {
    /// Creates a new learner with the given configuration options.
    pub fn new(options: ConstraintBasedOptions) -> Self {
        Self { options }
    }

    /// Runs the PC (Peter–Clark) algorithm to learn a Bayesian network.
    ///
    /// Steps: skeleton discovery, v-structure orientation, Meek's rule
    /// propagation, then network construction.
    pub fn pc_algorithm(
        &self,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<crate::core::models::bayesian_network::BayesianNetwork> {
        if data.is_empty() {
            return Err(LutufiError::InternalError { message: "Cannot learn structure from empty data".to_string() });
        }

        let node_names = DataProcessor::extract_variables(data);

        let skeleton_discovery = SkeletonDiscovery::new(self.options.alpha);
        let skeleton = skeleton_discovery.discover(data, &node_names)?;

        let v_structure_result = VStructureOrientator::orient(&skeleton, &node_names)?;
        let mut orientations = v_structure_result.orientations;

        MeeksRuleApplier::apply(&mut orientations, &skeleton);

        ConstrainedNetworkBuilder::build(&node_names, &orientations, data)
    }

    /// Runs the FCI (Fast Causal Inference) algorithm to learn a Partial
    /// Ancestral Graph (PAG) and a derived Bayesian network.
    ///
    /// Steps: FCI skeleton discovery, v-structure orientation in a PAG, rule
    /// application, then network construction.
    pub fn fci_algorithm(
        &self,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<FciResult> {
        if data.is_empty() {
            return Err(LutufiError::InternalError {
                message: "Cannot learn structure from empty data".to_string(),
            });
        }

        let node_names = DataProcessor::extract_variables(data);

        let skeleton = self.fci_skeleton_discovery(data, &node_names)?;

        let mut pag = self.fci_orient_vstructures(&skeleton, &node_names, data)?;

        self.fci_apply_rules(&mut pag, &node_names);

        let model = FciResult::pag_to_bayesian_network(&node_names, &pag, data)?;

        Ok(FciResult { pag, network: model })
    }

    fn fci_skeleton_discovery(
        &self,
        data: &[HashMap<String, String>],
        node_names: &[String],
    ) -> LutufiResult<FciSkeleton> {
        let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
        for u in node_names {
            let mut neighbors = HashSet::new();
            for v in node_names {
                if u != v { neighbors.insert(v.clone()); }
            }
            adjacency.insert(u.clone(), neighbors);
        }

        let mut sep_sets: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut k = 0;

        loop {
            let mut edges_to_remove = Vec::new();
            let mut any_neighbor_size_ge_k = false;

            for u in node_names {
                let neighbors: Vec<String> = adjacency[u].iter().cloned().collect();
                if neighbors.len() >= k {
                    any_neighbor_size_ge_k = true;
                    for v in &neighbors {
                        if u >= v { continue; }

                        let mut candidate_set: Vec<String> = adjacency[u]
                            .union(&adjacency[v])
                            .cloned()
                            .collect();
                        candidate_set.retain(|n| n != u && n != v);

                        if candidate_set.len() >= k {
                            for z_set in self.get_subsets_fci(&candidate_set, k) {
                                let indep = self.is_independent(u, v, &z_set, data)?;
                                if indep {
                                    edges_to_remove.push((u.clone(), v.clone()));
                                    sep_sets.insert((u.clone(), v.clone()), z_set.clone());
                                    sep_sets.insert((v.clone(), u.clone()), z_set);
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            for (u, v) in edges_to_remove {
                if let Some(neighbors) = adjacency.get_mut(&u) {
                    neighbors.remove(&v);
                }
                if let Some(neighbors) = adjacency.get_mut(&v) {
                    neighbors.remove(&u);
                }
            }

            if !any_neighbor_size_ge_k || k > 5 { break; }
            k += 1;
        }

        Ok(FciSkeleton { adjacency, sep_sets })
    }

    fn fci_orient_vstructures(
        &self,
        skeleton: &FciSkeleton,
        node_names: &[String],
        _data: &[HashMap<String, String>],
    ) -> LutufiResult<PagGraph> {
        let mut pag = PagGraph::new(node_names);
        for u in node_names {
            for v in node_names {
                if u < v && skeleton.adjacency[u].contains(v) {
                    pag.set_edge(u, v, PagEdgeMark::Circle, PagEdgeMark::Circle);
                }
            }
        }

        for z in node_names {
            let neighbors: Vec<String> = skeleton.adjacency[z].iter().cloned().collect();
            for i in 0..neighbors.len() {
                for j in i + 1..neighbors.len() {
                    let u = &neighbors[i];
                    let v = &neighbors[j];
                    if !skeleton.adjacency[u].contains(v) {
                        if let Some(sep) = skeleton.sep_sets.get(&(u.clone(), v.clone())) {
                            if !sep.contains(z) {
                                pag.set_edge(u, z, PagEdgeMark::Arrow, PagEdgeMark::Tail);
                                pag.set_edge(v, z, PagEdgeMark::Arrow, PagEdgeMark::Tail);
                            }
                        }
                    }
                }
            }
        }

        Ok(pag)
    }

    fn fci_apply_rules(&self, pag: &mut PagGraph, _node_names: &[String]) {
        loop {
            let mut changed = false;
            let edges: Vec<(String, String)> = pag.edges();

            for (a, c) in &edges {
                if a == c { continue; }
                if !pag.has_edge(a, c) { continue; }
                let (mark_a_c, _) = pag.get_edge_marks(a, c);
                if mark_a_c != PagEdgeMark::Circle { continue; }

                for b in pag.neighbors(a) {
                    if pag.has_edge(a, &b) && pag.has_edge(&b, c) {
                        let (mark_a_b, _mark_b_a) = pag.get_edge_marks(a, &b);
                        let (mark_b_c, _mark_c_b) = pag.get_edge_marks(&b, c);
                        let a_adj_c = pag.has_edge(a, c);

                        if mark_a_b == PagEdgeMark::Arrow && !a_adj_c {
                            if mark_b_c != PagEdgeMark::Tail {
                                pag.set_edge(&b, c, PagEdgeMark::Arrow, PagEdgeMark::Tail);
                                changed = true;
                            }
                        }
                    }
                }
            }

            for (a, c) in &edges {
                if a == c || !pag.has_edge(a, c) { continue; }
                for b in pag.neighbors(a) {
                    if !pag.has_edge(a, &b) || !pag.has_edge(&b, c) { continue; }
                    let (mark_a_b, mark_b_a) = pag.get_edge_marks(a, &b);
                    let (mark_b_c, _mark_c_b) = pag.get_edge_marks(&b, c);

                    if mark_a_b == PagEdgeMark::Arrow && mark_b_a == PagEdgeMark::Tail
                        && mark_b_c == PagEdgeMark::Circle
                        && !pag.has_edge(a, c)
                    {
                        pag.set_edge(&b, c, PagEdgeMark::Arrow, PagEdgeMark::Tail);
                        changed = true;
                    }
                }
            }

            for (a, c) in &edges {
                if a == c || !pag.has_edge(a, c) { continue; }
                for b in pag.neighbors(a) {
                    if !pag.has_edge(a, &b) || !pag.has_edge(&b, c) { continue; }
                    let (mark_a_b, mark_b_a) = pag.get_edge_marks(a, &b);
                    let (mark_b_c, _mark_c_b) = pag.get_edge_marks(&b, c);

                    if mark_b_a == PagEdgeMark::Arrow && mark_b_c == PagEdgeMark::Arrow
                        && !pag.has_edge(a, c)
                    {
                        if mark_a_b == PagEdgeMark::Circle {
                            pag.set_edge(a, &b, PagEdgeMark::Arrow, PagEdgeMark::Arrow);
                            changed = true;
                        }
                    }
                }
            }

            for (a, c) in &edges {
                if a == c || !pag.has_edge(a, c) { continue; }
                for b in pag.neighbors(a) {
                    if !pag.has_edge(a, &b) || !pag.has_edge(&b, c) { continue; }
                    let (mark_a_b, mark_b_a) = pag.get_edge_marks(a, &b);
                    let (_mark_b_c, mark_c_b) = pag.get_edge_marks(&b, c);

                    if mark_a_b == PagEdgeMark::Arrow && mark_b_a == PagEdgeMark::Tail
                        && mark_c_b == PagEdgeMark::Circle
                    {
                        pag.set_edge(c, &b, PagEdgeMark::Circle, PagEdgeMark::Tail);
                        changed = true;
                    }
                }
            }

            if !changed { break; }
        }
    }

    fn get_subsets_fci(&self, items: &[String], k: usize) -> Vec<Vec<String>> {
        if k == 0 { return vec![vec![]]; }
        if items.is_empty() { return vec![]; }

        let mut result = Vec::new();
        let head = items[0].clone();
        let tail: Vec<String> = items[1..].to_vec();

        for mut subset in self.get_subsets_fci(&tail, k - 1) {
            subset.insert(0, head.clone());
            result.push(subset);
        }

        result.extend(self.get_subsets_fci(&tail, k));
        result
    }

    fn is_independent(
        &self,
        x: &str,
        y: &str,
        z: &[String],
        data: &[HashMap<String, String>],
    ) -> LutufiResult<bool> {
        let z_refs: Vec<&str> = z.iter().map(|s| s.as_str()).collect();
        let (_, _, p_value) = DataProcessor::chi_square_test(data, x, y, &z_refs)?;
        Ok(p_value > self.options.alpha)
    }
}
