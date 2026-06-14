use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use crate::core::{
    assignment::Assignment,
    backend::get_backend,
    error::{LutufiError, LutufiResult},
    variable::VariableId,
};
use super::scope::Scope;
use super::utils::{multi_index_from_flat, project_indices, log_sum_exp};

/// Trait representing a factor in a probabilistic graphical model.
pub trait Factor {
    /// Returns the scope of the factor.
    fn scope(&self) -> &Scope;
    /// Evaluates the factor at the given full assignment, returning the probability.
    fn evaluate(&self, assignment: &Assignment) -> LutufiResult<f64>;
    /// Marginalizes the factor by summing out the specified variables.
    fn marginalize(&self, variables: &[VariableId]) -> LutufiResult<Box<dyn Factor>>;
    /// Multiplies this factor with another factor.
    fn multiply(&self, other: &dyn Factor) -> LutufiResult<Box<dyn Factor>>;
    /// Fixes variables to their assigned values and returns a reduced factor.
    fn reduce(&self, assignment: &Assignment) -> LutufiResult<Box<dyn Factor>>;
    /// Normalizes the factor in-place so its entries sum to 1.
    fn normalize(&mut self);
    /// Returns the log-value at the given flat index.
    fn log_value_at(&self, index: usize) -> f64;
    /// Downcasts to `Any` for dynamic dispatch support.
    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A factor stored as a table of log-probabilities, either dense or sparse.
pub enum TabularFactor {
    /// Dense storage: a vector of log-probabilities for every assignment.
    Dense {
        /// The scope (variables) of this dense factor.
        scope: Scope,
        /// Log-probability values for every assignment in row-major order.
        log_table: Vec<f64>,
    },
    /// Sparse storage: a map from flat index to log-probability for non-zero entries.
    Sparse {
        /// The scope (variables) of this sparse factor.
        scope: Scope,
        /// Log-probabilities for non-zero entries, keyed by flat index.
        log_entries: HashMap<usize, f64>,
    },
}

impl TabularFactor {
    /// Creates a `TabularFactor` from probability values, automatically choosing dense or sparse storage.
    pub fn from_values(scope: Scope, values: Vec<f64>) -> LutufiResult<Self> {
        if values.len() != scope.num_entries() {
            return Err(LutufiError::CptWrongShape {
                variable: "Factor".to_string(),
                expected_shape: format!("{}", scope.num_entries()),
                actual_shape: format!("{}", values.len()),
            });
        }

        let non_zeros = values.iter().filter(|&&v| v > 1e-12).count();
        let density = non_zeros as f64 / values.len() as f64;

        if density < 0.3 {
            let mut log_entries = HashMap::new();
            for (i, &v) in values.iter().enumerate() {
                if v > 1e-12 {
                    log_entries.insert(i, v.ln());
                }
            }
            Ok(TabularFactor::Sparse { scope, log_entries })
        } else {
            let log_table = values.iter().map(|&v| if v < 1e-12 { f64::NEG_INFINITY } else { v.ln() }).collect();
            Ok(TabularFactor::Dense { scope, log_table })
        }
    }

    /// Creates an identity factor (all ones, i.e. log-value 0) over the given scope.
    pub fn identity(scope: Scope) -> LutufiResult<Self> {
        let log_table = vec![0.0; scope.num_entries()];
        Ok(TabularFactor::Dense { scope, log_table })
    }

    /// Creates a dense `TabularFactor` directly from log-probability values.
    pub fn from_log_values(scope: Scope, log_values: Vec<f64>) -> LutufiResult<Self> {
        if log_values.len() != scope.num_entries() {
            return Err(LutufiError::CptWrongShape {
                variable: "Factor".to_string(),
                expected_shape: format!("{}", scope.num_entries()),
                actual_shape: format!("{}", log_values.len()),
            });
        }
        Ok(TabularFactor::Dense { scope, log_table: log_values })
    }

    /// Returns the log-probability at the given flat index.
    pub fn log_value_at(&self, index: usize) -> f64 {
        match self {
            TabularFactor::Dense { log_table, .. } => log_table[index],
            TabularFactor::Sparse { log_entries, .. } => *log_entries.get(&index).unwrap_or(&f64::NEG_INFINITY),
        }
    }

    /// Returns the probability at the given flat index.
    pub fn value_at(&self, index: usize) -> f64 {
        self.log_value_at(index).exp()
    }

    /// Returns the log-probability at the given assignment.
    pub fn log_value_at_assignment(&self, assignment: &Assignment) -> LutufiResult<f64> {
        let scope = self.scope();
        let mut flat_idx = 0;
        let mut stride = 1;

        for i in (0..scope.len()).rev() {
            let var_id = scope.variable_ids()[i];
            let val_idx = assignment.get_discrete(&var_id)?;
            flat_idx += val_idx * stride;
            stride *= scope.sizes()[i];
        }

        Ok(self.log_value_at(flat_idx))
    }

    /// Returns a reference to the factor's scope.
    pub fn scope(&self) -> &Scope {
        match self {
            TabularFactor::Dense { scope, .. } => scope,
            TabularFactor::Sparse { scope, .. } => scope,
        }
    }

    /// Multiplies this factor with another `TabularFactor` using the configured backend.
    pub fn multiply(&self, other: &TabularFactor) -> LutufiResult<TabularFactor> {
        get_backend().multiply(self, other)
    }

    /// Computes the product of two factors without using the backend.
    pub fn multiply_internal(&self, other: &TabularFactor) -> LutufiResult<TabularFactor> {
        let self_scope = self.scope();
        let other_scope = other.scope();

        let mut new_vars_set: BTreeMap<VariableId, usize> = BTreeMap::new();
        for (i, &var_id) in self_scope.variable_ids().iter().enumerate() {
            new_vars_set.insert(var_id, self_scope.sizes()[i]);
        }
        for (i, &var_id) in other_scope.variable_ids().iter().enumerate() {
            if let Some(&existing_size) = new_vars_set.get(&var_id) {
                if existing_size != other_scope.sizes()[i] {
                    return Err(LutufiError::InternalError {
                        message: format!("Variable size mismatch during factor product: {} has sizes {} and {}",
                                       var_id, existing_size, other_scope.sizes()[i])
                    });
                }
            } else {
                new_vars_set.insert(var_id, other_scope.sizes()[i]);
            }
        }

        let new_vars: Vec<VariableId> = new_vars_set.keys().cloned().collect();
        let new_sizes: Vec<usize> = new_vars_set.values().cloned().collect();
        let new_scope = Scope { variables: new_vars, sizes: new_sizes };
        let mut new_log_table = Vec::with_capacity(new_scope.num_entries());

        for i in 0..new_scope.num_entries() {
            let combined_indices = multi_index_from_flat(i, new_scope.sizes());
            let idx_self = project_indices(&combined_indices, new_scope.variable_ids(), self_scope.variable_ids(), self_scope.sizes())?;
            let idx_other = project_indices(&combined_indices, new_scope.variable_ids(), other_scope.variable_ids(), other_scope.sizes())?;
            new_log_table.push(self.log_value_at(idx_self) + other.log_value_at(idx_other));
        }

        Ok(TabularFactor::Dense { scope: new_scope, log_table: new_log_table })
    }

    /// Marginalizes variables using the configured backend.
    pub fn marginalize(&self, variables: &[VariableId]) -> LutufiResult<TabularFactor> {
        get_backend().marginalize(self, variables)
    }

    /// Marginalizes variables without using the backend.
    pub fn marginalize_internal(&self, variables: &[VariableId]) -> LutufiResult<TabularFactor> {
        let current_scope = self.scope();
        let target_set: std::collections::HashSet<_> = variables.iter().collect();

        let mut remaining_vars = Vec::new();
        let mut remaining_sizes = Vec::new();

        for (i, &var_id) in current_scope.variable_ids().iter().enumerate() {
            if !target_set.contains(&var_id) {
                remaining_vars.push(var_id);
                remaining_sizes.push(current_scope.sizes()[i]);
            }
        }

        if remaining_vars.len() == current_scope.len() {
            return Ok(self.clone());
        }

        let new_scope = Scope { variables: remaining_vars, sizes: remaining_sizes };
        let mut new_log_table = vec![f64::NEG_INFINITY; new_scope.num_entries()];

        for i in 0..current_scope.num_entries() {
            let full_indices = multi_index_from_flat(i, current_scope.sizes());
            let new_idx = project_indices(&full_indices, current_scope.variable_ids(), new_scope.variable_ids(), new_scope.sizes())?;
            new_log_table[new_idx] = log_sum_exp(new_log_table[new_idx], self.log_value_at(i));
        }

        Ok(TabularFactor::Dense { scope: new_scope, log_table: new_log_table })
    }

    /// Max-marginalizes the factor by taking the max over the specified variables.
    pub fn max_marginalize(&self, variables: &[VariableId]) -> LutufiResult<TabularFactor> {
        let current_scope = self.scope();
        let target_set: std::collections::HashSet<_> = variables.iter().collect();

        let mut remaining_vars = Vec::new();
        let mut remaining_sizes = Vec::new();

        for (i, &var_id) in current_scope.variable_ids().iter().enumerate() {
            if !target_set.contains(&var_id) {
                remaining_vars.push(var_id);
                remaining_sizes.push(current_scope.sizes()[i]);
            }
        }

        if remaining_vars.len() == current_scope.len() {
            return Ok(self.clone());
        }

        let new_scope = Scope { variables: remaining_vars, sizes: remaining_sizes };
        let mut new_log_table = vec![f64::NEG_INFINITY; new_scope.num_entries()];

        for i in 0..current_scope.num_entries() {
            let full_indices = multi_index_from_flat(i, current_scope.sizes());
            let new_idx = project_indices(&full_indices, current_scope.variable_ids(), new_scope.variable_ids(), new_scope.sizes())?;
            new_log_table[new_idx] = new_log_table[new_idx].max(self.log_value_at(i));
        }

        Ok(TabularFactor::Dense { scope: new_scope, log_table: new_log_table })
    }

    /// Fixes variables to their assigned values and returns a reduced factor.
    pub fn reduce(&self, assignment: &Assignment) -> LutufiResult<TabularFactor> {
        let current_scope = self.scope();
        let mut remaining_vars = Vec::new();
        let mut remaining_sizes = Vec::new();
        let mut fixed_values = Vec::new();

        for (i, &var_id) in current_scope.variable_ids().iter().enumerate() {
            if let Some(val_idx) = assignment.get_discrete(&var_id).ok() {
                fixed_values.push((i, val_idx));
            } else {
                remaining_vars.push(var_id);
                remaining_sizes.push(current_scope.sizes()[i]);
            }
        }

        if fixed_values.is_empty() {
            return Ok(self.clone());
        }

        let new_scope = Scope { variables: remaining_vars, sizes: remaining_sizes };
        let mut new_log_table = Vec::with_capacity(new_scope.num_entries());

        for i in 0..new_scope.num_entries() {
            let rem_indices = multi_index_from_flat(i, new_scope.sizes());
            let mut full_indices = vec![0; current_scope.len()];

            let mut rem_idx = 0;
            let mut fixed_idx = 0;
            for j in 0..current_scope.len() {
                if fixed_idx < fixed_values.len() && fixed_values[fixed_idx].0 == j {
                    full_indices[j] = fixed_values[fixed_idx].1;
                    fixed_idx += 1;
                } else {
                    full_indices[j] = rem_indices[rem_idx];
                    rem_idx += 1;
                }
            }

            let mut flat_idx = 0;
            let mut stride = 1;
            for j in (0..current_scope.len()).rev() {
                flat_idx += full_indices[j] * stride;
                stride *= current_scope.sizes()[j];
            }
            new_log_table.push(self.log_value_at(flat_idx));
        }

        Ok(TabularFactor::Dense { scope: new_scope, log_table: new_log_table })
    }

    /// Normalizes the factor in-place using the configured backend.
    pub fn normalize(&mut self) {
        get_backend().normalize(self);
    }

    /// Normalizes the factor in-place without using the backend.
    pub fn normalize_internal(&mut self) {
        match self {
            TabularFactor::Dense { log_table, .. } => {
                let mut max_log = f64::NEG_INFINITY;
                for &v in log_table.iter() { if v > max_log { max_log = v; } }
                if max_log.is_infinite() && max_log.is_sign_negative() {
                    return;
                }

                let sum: f64 = log_table.iter().map(|&v| (v - max_log).exp()).sum();
                let log_sum = max_log + sum.ln();

                for v in log_table.iter_mut() { *v -= log_sum; }
            }
            TabularFactor::Sparse { log_entries, .. } => {
                let mut max_log = f64::NEG_INFINITY;
                for &v in log_entries.values() { if v > max_log { max_log = v; } }
                if max_log.is_infinite() && max_log.is_sign_negative() {
                    return;
                }

                let sum: f64 = log_entries.values().map(|&v| (v - max_log).exp()).sum();
                let log_sum = max_log + sum.ln();

                for v in log_entries.values_mut() { *v -= log_sum; }
            }
        }
    }

    fn multiply_generic(&self, other: &dyn Factor) -> LutufiResult<Box<dyn Factor>> {
        let self_scope = self.scope();
        let other_scope = other.scope();

        let mut new_vars_set: BTreeMap<VariableId, usize> = BTreeMap::new();
        for (i, &var_id) in self_scope.variable_ids().iter().enumerate() {
            new_vars_set.insert(var_id, self_scope.sizes()[i]);
        }
        for (i, &var_id) in other_scope.variable_ids().iter().enumerate() {
            if let Some(&existing_size) = new_vars_set.get(&var_id) {
                if existing_size != other_scope.sizes()[i] {
                    return Err(LutufiError::InternalError {
                        message: format!("Variable size mismatch during factor product: {} has sizes {} and {}",
                                       var_id, existing_size, other_scope.sizes()[i])
                    });
                }
            } else {
                new_vars_set.insert(var_id, other_scope.sizes()[i]);
            }
        }

        let new_vars: Vec<VariableId> = new_vars_set.keys().cloned().collect();
        let new_sizes: Vec<usize> = new_vars_set.values().cloned().collect();
        let new_scope = Scope { variables: new_vars, sizes: new_sizes };
        let mut new_log_table = Vec::with_capacity(new_scope.num_entries());

        for i in 0..new_scope.num_entries() {
            let combined_indices = multi_index_from_flat(i, new_scope.sizes());
            let idx_self = project_indices(&combined_indices, new_scope.variable_ids(), self_scope.variable_ids(), self_scope.sizes())?;
            let idx_other = project_indices(&combined_indices, new_scope.variable_ids(), other_scope.variable_ids(), other_scope.sizes())?;
            new_log_table.push(self.log_value_at(idx_self) + other.log_value_at(idx_other));
        }

        Ok(Box::new(TabularFactor::Dense { scope: new_scope, log_table: new_log_table }))
    }
}

impl Factor for TabularFactor {
    fn scope(&self) -> &Scope {
        TabularFactor::scope(self)
    }

    fn evaluate(&self, assignment: &Assignment) -> LutufiResult<f64> {
        let scope = TabularFactor::scope(self);
        let mut flat_idx = 0;
        let mut stride = 1;

        for i in (0..scope.len()).rev() {
            let var_id = scope.variable_ids()[i];
            let val_idx = assignment.get_discrete(&var_id)?;
            flat_idx += val_idx * stride;
            stride *= scope.sizes()[i];
        }
        Ok(self.value_at(flat_idx))
    }

    fn marginalize(&self, variables: &[VariableId]) -> LutufiResult<Box<dyn Factor>> {
        Ok(Box::new(TabularFactor::marginalize(self, variables)?))
    }

    fn multiply(&self, other: &dyn Factor) -> LutufiResult<Box<dyn Factor>> {
        if let Some(other_tabular) = other.as_any().downcast_ref::<TabularFactor>() {
             Ok(Box::new(TabularFactor::multiply(self, other_tabular)?))
        } else {
            self.multiply_generic(other)
        }
    }

    fn reduce(&self, assignment: &Assignment) -> LutufiResult<Box<dyn Factor>> {
        Ok(Box::new(TabularFactor::reduce(self, assignment)?))
    }

    fn normalize(&mut self) {
        TabularFactor::normalize(self);
    }

    fn log_value_at(&self, index: usize) -> f64 {
        TabularFactor::log_value_at(self, index)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A factor representing a potential function (an unnormalized factor).
pub struct PotentialFunction {
    factor: TabularFactor,
}

impl PotentialFunction {
    /// Creates a new `PotentialFunction` with the given scope and probability values.
    pub fn new(scope: Scope, values: Vec<f64>) -> LutufiResult<Self> {
        Ok(PotentialFunction { factor: TabularFactor::from_values(scope, values)? })
    }

    /// Returns a reference to the underlying `TabularFactor`.
    pub fn as_factor(&self) -> &TabularFactor { &self.factor }
}
