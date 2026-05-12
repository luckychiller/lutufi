use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor, multi_index_from_flat},
    variable::{Variable, VariableId},
};

/// A Noisy-OR parameterization for binary variables with many parents.
///
/// Instead of storing O(2^k) entries for k parents, Noisy-OR stores
/// only k+1 parameters (the leak probability and one inhibition probability
/// per parent). This enables modeling of large parent sets that would
/// be intractable with full CPT tables.
///
/// Mathematical form:
/// P(child = false | parents) = leak * Π_i inhibition_i^{parent_i = true}
/// P(child = true | parents) = 1 - P(child = false | parents)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoisyOrFactor {
    child_id: VariableId,
    parent_ids: Vec<VariableId>,
    /// Probability that the child is true even if all parents are false (leak).
    leak: f64,
    /// Inhibition probabilities: P(child = false | parent_i = true, all others false).
    inhibitions: Vec<f64>,
    /// Log-space precomputations.
    log_leak: f64,
    log_inhibitions: Vec<f64>,
    /// Parent names (for error messages).
    parent_names: Vec<String>,
    child_name: String,
}

impl NoisyOrFactor {
    /// Create a new Noisy-OR factor.
    ///
    /// `child` is the binary child variable, `parents` are its binary parents,
    /// `leak` is the probability the child is true when all parents are false,
    /// and `inhibitions[i]` is P(child = false | parent_i = true, all others false).
    pub fn new(
        child: &Variable,
        parents: &[&Variable],
        leak: f64,
        inhibitions: Vec<f64>,
    ) -> LutufiResult<Self> {
        if !(0.0..=1.0).contains(&leak) {
            return Err(LutufiError::InternalError {
                message: format!("Noisy-OR leak probability must be in [0,1], got {}", leak),
            });
        }
        if inhibitions.len() != parents.len() {
            return Err(LutufiError::InternalError {
                message: format!(
                    "Noisy-OR requires {} inhibition probabilities for {} parents, got {}",
                    parents.len(), parents.len(), inhibitions.len()
                ),
            });
        }
        for (i, &inh) in inhibitions.iter().enumerate() {
            if !(0.0..=1.0).contains(&inh) {
                return Err(LutufiError::InternalError {
                    message: format!(
                        "Noisy-OR inhibition probability for parent {} must be in [0,1], got {}",
                        parents[i].name(), inh
                    ),
                });
            }
        }

        let log_leak = if leak < 1e-300 { f64::NEG_INFINITY } else { leak.ln() };
        let log_inhibitions: Vec<f64> = inhibitions.iter()
            .map(|&v| if v < 1e-300 { f64::NEG_INFINITY } else { v.ln() })
            .collect();

        Ok(NoisyOrFactor {
            child_id: child.id(),
            parent_ids: parents.iter().map(|v| v.id()).collect(),
            leak,
            inhibitions,
            log_leak,
            log_inhibitions,
            parent_names: parents.iter().map(|v| v.name().to_string()).collect(),
            child_name: child.name().to_string(),
        })
    }

    /// Returns the child variable ID.
    pub fn child_id(&self) -> VariableId { self.child_id }
    /// Returns the parent variable IDs.
    pub fn parent_ids(&self) -> &[VariableId] { &self.parent_ids }
    /// Returns the leak probability.
    pub fn leak(&self) -> f64 { self.leak }
    /// Returns the inhibition probabilities.
    pub fn inhibitions(&self) -> &[f64] { &self.inhibitions }

    /// Compute P(child = false | parent_config) where parent_config is a
    /// vector of booleans indicating which parents are true.
    pub fn prob_child_false(&self, parent_is_true: &[bool]) -> f64 {
        let mut log_prob = self.log_leak;
        for (i, &is_true) in parent_is_true.iter().enumerate() {
            if is_true && i < self.log_inhibitions.len() {
                log_prob += self.log_inhibitions[i];
            }
        }
        if log_prob.is_infinite() && log_prob.is_sign_negative() {
            0.0
        } else {
            log_prob.exp().min(1.0)
        }
    }

    /// Compute P(child = true | parent_config).
    pub fn prob_child_true(&self, parent_is_true: &[bool]) -> f64 {
        1.0 - self.prob_child_false(parent_is_true)
    }

    /// Evaluate for a specific flat index of the full scope.
    pub fn evaluate(&self, _flat_index: usize, num_parent_configs: usize) -> Vec<f64> {
        let child_size = 2;
        let mut result = vec![0.0; child_size * num_parent_configs];
        let parent_domain_size = 2;

        for parent_config in 0..num_parent_configs {
            let mut parent_is_true = Vec::with_capacity(self.parent_ids.len());
            let mut temp = parent_config;
            for _ in 0..self.parent_ids.len() {
                parent_is_true.push(temp % parent_domain_size == 1);
                temp /= parent_domain_size;
            }

            let p_false = self.prob_child_false(&parent_is_true);
            let p_true = 1.0 - p_false;
            // child false first (index 0), then child true (index 1)
            let base = parent_config * child_size;
            result[base] = p_false;
            result[base + 1] = p_true;
        }

        result
    }

    /// Convert to a full TabularFactor (for testing and compatibility).
    pub fn to_tabular_factor(&self, child: &Variable, parents: &[&Variable]) -> LutufiResult<TabularFactor> {
        let mut scope_vars: Vec<&Variable> = parents.to_vec();
        scope_vars.push(child);

        let scope = Scope::new(scope_vars.clone());
        let num_parent_configs = if parents.is_empty() {
            1
        } else {
            parents.iter().map(|v| v.domain().size().unwrap_or(2)).product()
        };

        let raw_values = self.evaluate(0, num_parent_configs);
        let mut values = vec![0.0; raw_values.len()];

        // Reorder from (parent_config, child_state) to scope order
        let original_sizes: Vec<usize> = scope_vars.iter()
            .map(|v| v.domain().size().unwrap_or(2))
            .collect();

        for i in 0..raw_values.len() {
            let sorted_indices = multi_index_from_flat(i, &original_sizes);
            let sorted_scope = Scope::new(scope_vars.clone());
            let sorted_idx = crate::core::factor::project_indices(
                &sorted_indices,
                &sorted_scope.variable_ids().to_vec(),
                scope.variable_ids(),
                scope.sizes(),
            )?;
            values[sorted_idx] = raw_values[i];
        }

        TabularFactor::from_values(scope, values)
    }

    /// Memory saved compared to a full CPT (in bytes).
    pub fn memory_savings(&self, domain_sizes: &[usize]) -> usize {
        let full_size: usize = domain_sizes.iter().product::<usize>() * std::mem::size_of::<f64>();
        let noisy_or_size = (2 + self.parent_ids.len()) * std::mem::size_of::<f64>()
            + self.parent_ids.len() * std::mem::size_of::<VariableId>();
        full_size.saturating_sub(noisy_or_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noisy_or_basic() {
        let child = Variable::new("Fever", crate::core::domain::Domain::binary());
        let p1 = Variable::new("Cold", crate::core::domain::Domain::binary());
        let p2 = Variable::new("Flu", crate::core::domain::Domain::binary());

        let no = NoisyOrFactor::new(&child, &[&p1, &p2], 0.1, vec![0.4, 0.3]).unwrap();

        assert!((no.leak() - 0.1).abs() < 1e-10);
        assert_eq!(no.inhibitions().len(), 2);

        // All parents false: P(false) = leak = 0.1
        let p_false = no.prob_child_false(&[false, false]);
        assert!((p_false - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_noisy_or_parameter_validation() {
        let child = Variable::new("X", crate::core::domain::Domain::binary());
        let parent = Variable::new("Y", crate::core::domain::Domain::binary());

        // Invalid leak
        assert!(NoisyOrFactor::new(&child, &[&parent], -0.1, vec![0.5]).is_err());
        // Invalid inhibition
        assert!(NoisyOrFactor::new(&child, &[&parent], 0.1, vec![1.5]).is_err());
        // Wrong number of inhibitions
        assert!(NoisyOrFactor::new(&child, &[&parent], 0.1, vec![0.5, 0.3]).is_err());
    }

    #[test]
    fn test_noisy_or_vs_full_cpt() {
        let child = Variable::new("Child", crate::core::domain::Domain::binary());
        let parent = Variable::new("Parent", crate::core::domain::Domain::binary());

        let no = NoisyOrFactor::new(&child, &[&parent], 0.0, vec![0.0]).unwrap();
        let tf = no.to_tabular_factor(&child, &[&parent]).unwrap();

        // With leak=0 and inhibition=0: P(child=true|parent=true) = 1
        // P(child=false|parent=false) = 0... wait, leak=0 means P(child=true) = 0 when all false
        // Let's compute: P(false|all false) = leak = 0. So P(true|all false) = 1
        // P(false|parent=true) = leak * inhibition = 0*0 = 0. So P(true|parent=true) = 1
        assert!((tf.value_at(0) - 0.0).abs() < 1e-10); // child=false, parent=false
        assert!((tf.value_at(1) - 1.0).abs() < 1e-10); // child=true, parent=false
        assert!((tf.value_at(0) - 0.0).abs() < 1e-10); // child=false, parent=true
        assert!((tf.value_at(1) - 1.0).abs() < 1e-10); // child=true, parent=true
    }

    #[test]
    fn test_memory_savings() {
        let child = Variable::new("Y", crate::core::domain::Domain::binary());
        let p1 = Variable::new("A", crate::core::domain::Domain::binary());
        let p2 = Variable::new("B", crate::core::domain::Domain::binary());
        let p3 = Variable::new("C", crate::core::domain::Domain::binary());

        let no = NoisyOrFactor::new(&child, &[&p1, &p2, &p3], 0.1, vec![0.5, 0.5, 0.5]).unwrap();
        let savings = no.memory_savings(&[2, 2, 2, 2]);
        assert!(savings > 0);
    }
}
