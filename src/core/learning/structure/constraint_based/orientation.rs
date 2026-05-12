use std::collections::HashMap;
use crate::core::error::LutufiResult;
use super::types::{EdgeOrientation, SkeletonResult, VStructureResult};

pub struct VStructureOrientator;

impl VStructureOrientator {
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
