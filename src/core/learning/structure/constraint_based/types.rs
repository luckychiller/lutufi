use std::collections::{HashMap, HashSet};
use crate::core::models::bayesian_network::BayesianNetwork;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndependenceTestType {
    ChiSquare,
    GTest,
    FisherZ,
}

#[derive(Debug, Clone)]
pub struct ConstraintBasedOptions {
    pub test_type: IndependenceTestType,
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

pub struct SkeletonResult {
    pub adjacency: HashMap<String, HashSet<String>>,
    pub separation_sets: HashMap<(String, String), Vec<String>>,
}

pub struct VStructureResult {
    pub orientations: HashMap<(String, String), EdgeOrientation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeOrientation {
    Undirected,
    Directed,
    ReverseDirected,
}

pub(crate) struct FciSkeleton {
    pub(crate) adjacency: HashMap<String, HashSet<String>>,
    pub(crate) sep_sets: HashMap<(String, String), Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagEdgeMark {
    Arrow,
    Tail,
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

#[derive(Debug, Clone)]
pub struct PagGraph {
    adjacency: HashMap<String, HashMap<String, (PagEdgeMark, PagEdgeMark)>>,
}

impl PagGraph {
    pub fn new(node_names: &[String]) -> Self {
        let mut adjacency = HashMap::new();
        for name in node_names {
            adjacency.insert(name.clone(), HashMap::new());
        }
        PagGraph { adjacency }
    }

    pub fn set_edge(&mut self, u: &str, v: &str, mark_u: PagEdgeMark, mark_v: PagEdgeMark) {
        self.adjacency.get_mut(u).and_then(|m| m.insert(v.to_string(), (mark_u, mark_v)));
        self.adjacency.get_mut(v).and_then(|m| m.insert(u.to_string(), (mark_v, mark_u)));
    }

    pub fn get_edge_marks(&self, u: &str, v: &str) -> (PagEdgeMark, PagEdgeMark) {
        self.adjacency.get(u)
            .and_then(|m| m.get(v))
            .copied()
            .unwrap_or((PagEdgeMark::Tail, PagEdgeMark::Tail))
    }

    pub fn has_edge(&self, u: &str, v: &str) -> bool {
        self.adjacency.get(u).and_then(|m| m.get(v)).is_some()
    }

    pub fn neighbors(&self, u: &str) -> Vec<String> {
        self.adjacency.get(u)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }

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

#[derive(Debug, Clone)]
pub struct FciResult {
    pub pag: PagGraph,
    pub network: BayesianNetwork,
}

impl FciResult {
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
