use std::collections::HashMap;
use super::types::{EdgeOrientation, SkeletonResult};

/// Applies Meek's orientation propagation rules to extend a partial orientation
/// into a maximally oriented graph.
pub struct MeeksRuleApplier;

impl MeeksRuleApplier {
    /// Iteratively applies Meek's rules (R1–R4) to orient additional edges
    /// until no further orientations can be inferred.
    pub fn apply(
        orientations: &mut HashMap<(String, String), EdgeOrientation>,
        skeleton: &SkeletonResult,
    ) {
        loop {
            let mut changed = false;
            let current_edges: Vec<_> = orientations.keys().cloned().collect();

            for (a, b) in current_edges {
                if Self::is_oriented(orientations, &a, &b) == Some(true) {
                    for c in skeleton.adjacency.get(&b).unwrap_or(&std::collections::HashSet::new()).clone() {
                        if c != a && !skeleton.adjacency.get(&a).unwrap_or(&std::collections::HashSet::new()).contains(&c) {
                            if Self::is_undirected(orientations, &b, &c) {
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
