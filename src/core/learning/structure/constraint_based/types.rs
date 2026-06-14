use std::collections::{HashMap, HashSet};
use crate::core::models::bayesian_network::BayesianNetwork;

/// Statistical test used for conditional independence testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndependenceTestType {
    /// Chi-squared test of independence.
    ChiSquare,
    /// G-test (likelihood-ratio) of independence.
    GTest,
    /// Fisher's z-test (for continuous data approximated as categorical).
    FisherZ,
}

/// Configuration options for constraint-based structure learning algorithms.
#[derive(Debug, Clone)]
pub struct ConstraintBasedOptions {
    /// The independence test to use (e.g. ChiSquare, GTest, FisherZ).
    pub test_type: IndependenceTestType,
    /// Significance level (p-value threshold) for independence tests.
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

/// The result of skeleton discovery in constraint-based structure learning.
///
/// Contains the undirected adjacency graph and separation sets used during
/// conditional independence testing.
pub struct SkeletonResult {
    /// Undirected adjacency: maps each node to its set of neighbors.
    pub adjacency: HashMap<String, HashSet<String>>,
    /// Maps unordered variable pairs to the conditioning set that made them
    /// conditionally independent.
    pub separation_sets: HashMap<(String, String), Vec<String>>,
}

/// The result of v-structure orientation phase.
///
/// Contains the edge orientations discovered from collider detection.
pub struct VStructureResult {
    /// Maps unordered variable pairs to their determined orientation.
    pub orientations: HashMap<(String, String), EdgeOrientation>,
}

/// Orientation assigned to an undirected edge during v-structure discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeOrientation {
    /// No direction has been assigned to the edge.
    Undirected,
    /// Edge is oriented from the first node to the second (a -> b).
    Directed,
    /// Edge is oriented from the second node to the first (b -> a).
    ReverseDirected,
}

pub(crate) struct FciSkeleton {
    pub(crate) adjacency: HashMap<String, HashSet<String>>,
    pub(crate) sep_sets: HashMap<(String, String), Vec<String>>,
}

/// An endpoint mark on a PAG edge (arrow, tail, or circle).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagEdgeMark {
    /// Arrow mark ("->") indicating a directed edge endpoint.
    Arrow,
    /// Tail mark ("--") indicating a tail (undirected) edge endpoint.
    Tail,
    /// Circle mark ("o-") indicating an uncertain or unknown edge endpoint.
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

/// A Partial Ancestral Graph (PAG) representing equivalence classes of
/// causal graphs with latent confounders.
#[derive(Debug, Clone)]
pub struct PagGraph {
    adjacency: HashMap<String, HashMap<String, (PagEdgeMark, PagEdgeMark)>>,
}

impl PagGraph {
    /// Creates a new empty PAG with the given node names (no edges).
    pub fn new(node_names: &[String]) -> Self {
        let mut adjacency = HashMap::new();
        for name in node_names {
            adjacency.insert(name.clone(), HashMap::new());
        }
        PagGraph { adjacency }
    }

    /// Sets a bidirectional edge between `u` and `v` with the given endpoint marks.
    pub fn set_edge(&mut self, u: &str, v: &str, mark_u: PagEdgeMark, mark_v: PagEdgeMark) {
        self.adjacency.get_mut(u).and_then(|m| m.insert(v.to_string(), (mark_u, mark_v)));
        self.adjacency.get_mut(v).and_then(|m| m.insert(u.to_string(), (mark_v, mark_u)));
    }

    /// Returns the edge marks for edge `(u, v)`, defaulting to (Tail, Tail) if absent.
    pub fn get_edge_marks(&self, u: &str, v: &str) -> (PagEdgeMark, PagEdgeMark) {
        self.adjacency.get(u)
            .and_then(|m| m.get(v))
            .copied()
            .unwrap_or((PagEdgeMark::Tail, PagEdgeMark::Tail))
    }

    /// Returns `true` if there is an edge between `u` and `v`.
    pub fn has_edge(&self, u: &str, v: &str) -> bool {
        self.adjacency.get(u).and_then(|m| m.get(v)).is_some()
    }

    /// Returns the list of nodes adjacent to `u`.
    pub fn neighbors(&self, u: &str) -> Vec<String> {
        self.adjacency.get(u)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Returns all undirected edge pairs `(u, v)` with `u < v` in the graph.
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

/// The result of the FCI (Fast Causal Inference) algorithm.
///
/// Contains the learned Partial Ancestral Graph (PAG) and a derived Bayesian
/// network built from the certain directed edges.
#[derive(Debug, Clone)]
pub struct FciResult {
    /// The learned Partial Ancestral Graph.
    pub pag: PagGraph,
    /// A Bayesian network derived from the PAG's certain directed edges.
    pub network: BayesianNetwork,
}

impl FciResult {
    /// Converts a PAG into a Bayesian network by extracting directed edges
    /// (Arrow–Tail or Tail–Arrow) and building a variable model from the data.
    pub fn pag_to_bayesian_network(
        node_names: &[String],
        pag: &PagGraph,
        data: &[HashMap<String, String>],
    ) -> crate::core::error::LutufiResult<BayesianNetwork> {
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
