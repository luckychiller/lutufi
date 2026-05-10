//! Constraint-based structure learning.
//!
//! Implements PC and FCI algorithms for causal discovery.

use std::collections::{HashMap, HashSet};
use crate::core::{
    error::{LutufiError, LutufiResult},
    learning::data_processor::DataProcessor,
    models::bayesian_network::BayesianNetwork,
};

/// Type of conditional independence test to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndependenceTestType {
    /// Chi-square test for discrete data.
    ChiSquare,
    /// G-test (likelihood ratio) for discrete data.
    GTest,
    /// Fisher's Z-test for continuous data (Gaussian assumption).
    FisherZ,
}

/// Options for constraint-based learning.
#[derive(Debug, Clone)]
pub struct ConstraintBasedOptions {
    /// Independence test type.
    pub test_type: IndependenceTestType,
    /// Significance level for independence tests.
    pub alpha: f64,
}

impl Default for ConstraintBasedOptions {
    fn default() -> Self {
        Self {
            test_type: IndependenceTestType::ChiSquare,
            alpha: 0.05,
        }
    }
}

/// Result of skeleton discovery phase.
pub struct SkeletonResult {
    /// Adjacency map (undirected edges).
    pub adjacency: HashMap<String, HashSet<String>>,
    /// Separation sets for independence relationships.
    pub separation_sets: HashMap<(String, String), Vec<String>>,
}

/// Result of v-structure orientation phase.
pub struct VStructureResult {
    /// Edge orientations.
    pub orientations: HashMap<(String, String), EdgeOrientation>,
}

/// Edge orientation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeOrientation {
    /// Undirected edge
    Undirected,
    /// Directed edge u -> v
    Directed,
    /// Reverse directed edge v -> u
    ReverseDirected,
}

/// Phase 1: Skeleton Discovery
///
/// Discovers the skeleton (undirected graph) using conditional independence tests.
pub struct SkeletonDiscovery {
    alpha: f64,
}

impl SkeletonDiscovery {
    /// Create a new skeleton discovery phase.
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }

    /// Execute skeleton discovery algorithm.
    pub fn discover(
        &self,
        data: &[HashMap<String, String>],
        node_names: &[String],
    ) -> LutufiResult<SkeletonResult> {
        let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
        for u in node_names {
            let mut neighbors = HashSet::new();
            for v in node_names {
                if u != v {
                    neighbors.insert(v.clone());
                }
            }
            adjacency.insert(u.clone(), neighbors);
        }

        let mut separation_sets: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut k = 0;

        loop {
            let mut edges_to_remove = Vec::new();
            let mut any_neighbor_size_ge_k = false;

            for u in node_names {
                let neighbors: Vec<String> = adjacency[u].iter().cloned().collect();
                if neighbors.len() >= k {
                    any_neighbor_size_ge_k = true;
                    for v in &neighbors {
                        if u >= v {
                            continue;
                        }

                        let others: Vec<String> = adjacency[u]
                            .iter()
                            .filter(|&n| n != v)
                            .cloned()
                            .collect();
                        if others.len() >= k {
                            for z_set in self.get_subsets(&others, k) {
                                if self.is_independent(u, v, &z_set, data)? {
                                    edges_to_remove.push((u.clone(), v.clone()));
                                    separation_sets.insert((u.clone(), v.clone()), z_set.clone());
                                    separation_sets.insert((v.clone(), u.clone()), z_set);
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

            if !any_neighbor_size_ge_k || k > 5 {
                break;
            }
            k += 1;
        }

        Ok(SkeletonResult {
            adjacency,
            separation_sets,
        })
    }

    fn get_subsets(&self, items: &[String], k: usize) -> Vec<Vec<String>> {
        if k == 0 { return vec![vec![]]; }
        if items.is_empty() { return vec![]; }

        let mut result = Vec::new();
        let head = items[0].clone();
        let tail = &items[1..];

        for mut subset in self.get_subsets(&tail.iter().map(|s| s.clone()).collect::<Vec<_>>(), k - 1) {
            subset.insert(0, head.clone());
            result.push(subset);
        }

        result.extend(self.get_subsets(
            &tail.iter().map(|s| s.clone()).collect::<Vec<_>>(),
            k,
        ));
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
        let (_, _, p_value) = crate::core::learning::data_processor::DataProcessor::chi_square_test(
            data, x, y, &z_refs,
        )?;
        Ok(p_value > self.alpha)
    }
}

/// Phase 2: V-Structure Orientation
pub struct VStructureOrientator;

impl VStructureOrientator {
    /// Orient v-structures in the skeleton.
    pub fn orient(
        skeleton: &SkeletonResult,
        node_names: &[String],
    ) -> LutufiResult<VStructureResult> {
        let mut orientations: HashMap<(String, String), EdgeOrientation> = HashMap::new();

        for u in node_names {
            for v in node_names {
                if u < v && skeleton.adjacency[u].contains(v) {
                    orientations.insert((u.clone(), v.clone()), EdgeOrientation::Undirected);
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
                        if let Some(sep) = skeleton.separation_sets.get(&(u.clone(), v.clone())) {
                            if !sep.contains(z) {
                                Self::set_orientation(&mut orientations, u, z, EdgeOrientation::Directed);
                                Self::set_orientation(&mut orientations, v, z, EdgeOrientation::Directed);
                            }
                        }
                    }
                }
            }
        }

        Ok(VStructureResult { orientations })
    }

    fn set_orientation(
        orientations: &mut HashMap<(String, String), EdgeOrientation>,
        from: &str,
        to: &str,
        orientation: EdgeOrientation,
    ) {
        let key = if from < to {
            (from.to_string(), to.to_string())
        } else {
            (to.to_string(), from.to_string())
        };
        orientations.insert(key, orientation);
    }
}

/// Phase 3: Meek's Rules Application
pub struct MeeksRuleApplier;

impl MeeksRuleApplier {
    /// Apply Meek's rules to propagate orientations.
    pub fn apply(
        orientations: &mut HashMap<(String, String), EdgeOrientation>,
        skeleton: &SkeletonResult,
    ) {
        loop {
            let mut changed = false;
            let current_edges: Vec<_> = orientations.keys().cloned().collect();

            for (a, b) in current_edges {
                if Self::is_oriented(&orientations, &a, &b) == Some(true) {
                    for c in skeleton.adjacency.get(&b).unwrap_or(&HashSet::new()).clone() {
                        if c != a && !skeleton.adjacency.get(&a).unwrap_or(&HashSet::new()).contains(&c) {
                            if Self::is_undirected(&orientations, &b, &c) {
                                Self::set_orientation(orientations, &b, &c, EdgeOrientation::Directed);
                                changed = true;
                            }
                        }
                    }
                }
            }

            if !changed { break; }
        }
    }

    fn is_oriented(
        orientations: &HashMap<(String, String), EdgeOrientation>,
        u: &str,
        v: &str,
    ) -> Option<bool> {
        let key = if u < v {
            (u.to_string(), v.to_string())
        } else {
            (v.to_string(), u.to_string())
        };
        orientations.get(&key).map(|o| {
            if u < v {
                matches!(o, EdgeOrientation::Directed)
            } else {
                matches!(o, EdgeOrientation::ReverseDirected)
            }
        })
    }

    fn is_undirected(
        orientations: &HashMap<(String, String), EdgeOrientation>,
        u: &str,
        v: &str,
    ) -> bool {
        let key = if u < v {
            (u.to_string(), v.to_string())
        } else {
            (v.to_string(), u.to_string())
        };
        orientations
            .get(&key)
            .map(|o| matches!(o, EdgeOrientation::Undirected))
            .unwrap_or(false)
    }

    fn set_orientation(
        orientations: &mut HashMap<(String, String), EdgeOrientation>,
        from: &str,
        to: &str,
        orientation: EdgeOrientation,
    ) {
        let key = if from < to {
            (from.to_string(), to.to_string())
        } else {
            (to.to_string(), from.to_string())
        };
        orientations.insert(key, orientation);
    }
}

/// Phase 4: Network Builder
pub struct ConstrainedNetworkBuilder;

impl ConstrainedNetworkBuilder {
    /// Build final Bayesian Network from orientations.
    pub fn build(
        node_names: &[String],
        orientations: &HashMap<(String, String), EdgeOrientation>,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<BayesianNetwork> {
        let mut model = BayesianNetwork::new();

        for name in node_names {
            let mut states = HashSet::new();
            for row in data {
                if let Some(val) = row.get(name) {
                    states.insert(val.clone());
                }
            }
            let mut sorted_states: Vec<_> = states.into_iter().collect();
            sorted_states.sort();
            model.add_variable(
                name,
                crate::core::domain::Domain::discrete(sorted_states)?,
            )?;
        }

        for ((u, v), orientation) in orientations {
            match orientation {
                EdgeOrientation::Directed => {
                    model.add_edge(u, v)?;
                }
                EdgeOrientation::ReverseDirected => {
                    model.add_edge(v, u)?;
                }
                EdgeOrientation::Undirected => {
                    if model.add_edge(u, v).is_err() {
                        model.add_edge(v, u)?;
                    }
                }
            }
        }

        Ok(model)
    }
}

/// Learner for constraint-based structure discovery.
pub struct ConstraintBasedLearner {
    options: ConstraintBasedOptions,
}

impl ConstraintBasedLearner {
    /// Create a new ConstraintBasedLearner.
    pub fn new(options: ConstraintBasedOptions) -> Self {
        Self { options }
    }

    /// Learn the structure using the PC algorithm.
    pub fn pc_algorithm(
        &self,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<BayesianNetwork> {
        if data.is_empty() {
            return Err(LutufiError::InternalError { message: "Cannot learn structure from empty data".to_string() });
        }

        let node_names = crate::core::learning::data_processor::DataProcessor::extract_variables(data);
        
        let skeleton_discovery = SkeletonDiscovery::new(self.options.alpha);
        let skeleton = skeleton_discovery.discover(data, &node_names)?;

        let v_structure_result = VStructureOrientator::orient(&skeleton, &node_names)?;
        let mut orientations = v_structure_result.orientations;

        MeeksRuleApplier::apply(&mut orientations, &skeleton);

        ConstrainedNetworkBuilder::build(&node_names, &orientations, data)
    }

    /// Learn the structure using the Fast Causal Inference (FCI) algorithm.
    /// FCI extends PC by allowing for latent confounders and produces a
    /// Partial Ancestral Graph (PAG).
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

        // Phase 1: FCI skeleton discovery (extended PC skeleton)
        let skeleton = self.fci_skeleton_discovery(data, &node_names)?;

        // Phase 2: Orient v-structures
        let mut pag = self.fci_orient_vstructures(&skeleton, &node_names, data)?;

        // Phase 3: FCI orientation rules (R1-R4)
        self.fci_apply_rules(&mut pag, &node_names);

        // Build Bayesian network from oriented edges
        let model = FciResult::pag_to_bayesian_network(&node_names, &pag, data)?;

        Ok(FciResult { pag, network: model })
    }

    /// FCI skeleton discovery - records all separating sets for later orientation.
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

                        // FCI uses adj(X) ∪ adj(Y) \ {X,Y} as candidate conditioning sets
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

    /// Orient v-structures from the FCI skeleton.
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

        // Orient v-structures: A - C - B with C not in SepSet(A,B)
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

    /// Apply FCI orientation rules R1-R4 to propagate edge marks.
    fn fci_apply_rules(&self, pag: &mut PagGraph, _node_names: &[String]) {
        loop {
            let mut changed = false;
            let edges: Vec<(String, String)> = pag.edges();

            // R1: If A o-> B o-* C and A, C not adjacent → A o-> B -> C
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

            // R2: If A -> B o-> C and A, C not adjacent → A -> B -> C
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

            // R3: If A o-> B <-o C and A, C not adjacent → A o-> B *-> C
            // Actually we already oriented v-structures, so this may not fire.
            // But check for: A o-* B <-* C and A, C not adjacent
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

            // R4: Propagate tail marks: if A -> B -o C and A o-* C and A, C adjacent
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

/// Skeleton result for the FCI algorithm.
struct FciSkeleton {
    adjacency: HashMap<String, HashSet<String>>,
    sep_sets: HashMap<(String, String), Vec<String>>,
}

/// Edge mark type for Partial Ancestral Graphs (PAGs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagEdgeMark {
    /// Arrow head (>)
    Arrow,
    /// Tail (-)
    Tail,
    /// Circle (o)
    Circle,
}

impl std::fmt::Display for PagEdgeMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PagEdgeMark::Arrow => write!(f, ">"),
            PagEdgeMark::Tail => write!(f, "-"),
            PagEdgeMark::Circle => write!(f, "o"),
        }
    }
}

/// A Partial Ancestral Graph (PAG) representation.
#[derive(Debug, Clone)]
pub struct PagGraph {
    adjacency: HashMap<String, HashMap<String, (PagEdgeMark, PagEdgeMark)>>,
}

impl PagGraph {
    /// Create a new empty PAG over the given nodes.
    pub fn new(node_names: &[String]) -> Self {
        let mut adjacency = HashMap::new();
        for name in node_names {
            adjacency.insert(name.clone(), HashMap::new());
        }
        PagGraph { adjacency }
    }

    /// Set edge marks between u and v (marks are from u's perspective, v's perspective).
    pub fn set_edge(&mut self, u: &str, v: &str, mark_u: PagEdgeMark, mark_v: PagEdgeMark) {
        self.adjacency.get_mut(u).and_then(|m| m.insert(v.to_string(), (mark_u, mark_v)));
        self.adjacency.get_mut(v).and_then(|m| m.insert(u.to_string(), (mark_v, mark_u)));
    }

    /// Get edge marks between u and v.
    pub fn get_edge_marks(&self, u: &str, v: &str) -> (PagEdgeMark, PagEdgeMark) {
        self.adjacency.get(u)
            .and_then(|m| m.get(v))
            .copied()
            .unwrap_or((PagEdgeMark::Tail, PagEdgeMark::Tail))
    }

    /// Check if an edge exists between u and v.
    pub fn has_edge(&self, u: &str, v: &str) -> bool {
        self.adjacency.get(u).and_then(|m| m.get(v)).is_some()
    }

    /// Get all neighbors of a node.
    pub fn neighbors(&self, u: &str) -> Vec<String> {
        self.adjacency.get(u)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all edges in the PAG.
    pub fn edges(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        for (u, neighbors) in &self.adjacency {
            for v in neighbors.keys() {
                if u < v {
                    result.push((u.clone(), v.clone()));
                }
            }
        }
        result
    }
}

/// Result of the FCI algorithm containing both the PAG and a converted BayesianNetwork.
#[derive(Debug, Clone)]
pub struct FciResult {
    /// The Partial Ancestral Graph (PAG).
    pub pag: PagGraph,
    /// A BayesianNetwork approximation (fully oriented edges only).
    pub network: BayesianNetwork,
}

impl FciResult {
    /// Convert a PAG to a BayesianNetwork using only definitively oriented edges.
    pub fn pag_to_bayesian_network(
        node_names: &[String],
        pag: &PagGraph,
        data: &[HashMap<String, String>],
    ) -> LutufiResult<BayesianNetwork> {
        let mut model = BayesianNetwork::new();
        model.mark_as_causal();

        for name in node_names {
            let mut states = HashSet::new();
            for row in data {
                if let Some(val) = row.get(name) {
                    states.insert(val.clone());
                }
            }
            let mut sorted_states: Vec<_> = states.into_iter().collect();
            sorted_states.sort();
            model.add_variable(name, crate::core::domain::Domain::discrete(sorted_states)?)?;
        }

        // Add edges that are definitively oriented (Arrow-Tail = directed)
        for (u, v) in pag.edges() {
            let (mark_u, mark_v) = pag.get_edge_marks(&u, &v);
            match (mark_u, mark_v) {
                (PagEdgeMark::Arrow, PagEdgeMark::Tail) => {
                    let _ = model.add_edge(&v, &u);
                }
                (PagEdgeMark::Tail, PagEdgeMark::Arrow) => {
                    let _ = model.add_edge(&u, &v);
                }
                _ => {}
            }
        }

        Ok(model)
    }
}

impl std::fmt::Display for FciResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FCIResult {{")?;
        writeln!(f, "  Directed edges (certain):")?;
        for edge in self.network.edges() {
            writeln!(f, "    {} -> {}", edge.0, edge.1)?;
        }
        writeln!(f, "  PAG edges:")?;
        for (u, v) in self.pag.edges() {
            let (mu, mv) = self.pag.get_edge_marks(&u, &v);
            writeln!(f, "    {} {} {} {}", u, mu, mv, v)?;
        }
        write!(f, "}}")
    }
}
