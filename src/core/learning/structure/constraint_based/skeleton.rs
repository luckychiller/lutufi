use std::collections::{HashMap, HashSet};
use crate::core::error::LutufiResult;
use super::types::SkeletonResult;

pub struct SkeletonDiscovery {
    alpha: f64,
}

impl SkeletonDiscovery {
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }

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
