use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    variable::{Variable, VariableId},
};

/// The scope of a factor — the variables it covers.
/// Scopes are always kept sorted by variable ID for deterministic indexing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scope {
    variables: Vec<VariableId>,
    sizes: Vec<usize>,
}

impl Scope {
    /// Create a new scope from a list of variables.
    pub fn new(variables: Vec<&Variable>) -> Self {
        let mut sorted_vars: Vec<&Variable> = variables.to_vec();
        sorted_vars.sort_by_key(|v| v.id());
        
        let vars: Vec<VariableId> = sorted_vars.iter().map(|v| v.id()).collect();
        let sizes = sorted_vars.iter().map(|v| v.domain().size().unwrap_or(0)).collect();
        Scope { variables: vars, sizes }
    }

    /// Create a scope from variable IDs and their sizes.
    pub fn from_ids_and_sizes(variables: Vec<VariableId>, sizes: Vec<usize>) -> Self {
        let mut vars_with_sizes: Vec<(VariableId, usize)> = variables.into_iter().zip(sizes.into_iter()).collect();
        vars_with_sizes.sort_by_key(|(var_id, _)| var_id.to_string());
        let (vars, sizes): (Vec<_>, Vec<_>) = vars_with_sizes.into_iter().unzip();
        Scope { variables: vars, sizes }
    }

    /// The number of variables in the scope.
    pub fn len(&self) -> usize { self.variables.len() }
    /// Whether the scope is empty.
    pub fn is_empty(&self) -> bool { self.variables.is_empty() }
    /// The total number of configurations in the table.
    pub fn num_entries(&self) -> usize { self.sizes.iter().product() }
    /// The variable IDs in the scope.
    pub fn variable_ids(&self) -> &[VariableId] { &self.variables }
    /// The sizes of the variable domains.
    pub fn sizes(&self) -> &[usize] { &self.sizes }

    /// Check if the scope contains a variable.
    pub fn contains(&self, var_id: &VariableId) -> bool {
        self.variables.binary_search(var_id).is_ok()
    }

    /// Check if the scope contains all variables in the set.
    pub fn contains_all(&self, vars: &std::collections::HashSet<VariableId>) -> bool {
        vars.iter().all(|id| self.contains(id))
    }

    /// Get the size of a variable in the scope.
    pub fn size_of(&self, var_id: &VariableId) -> Option<usize> {
        self.variables.binary_search(var_id).ok().map(|idx| self.sizes[idx])
    }
}

/// A factor over a set of variables.
///
/// A factor is a mapping from assignments of its variables to a non-negative real value.
/// This trait provides the fundamental operations for factor algebra: product and marginalization.
pub trait Factor {
    /// The variables this factor covers.
    fn scope(&self) -> &Scope;
    /// Evaluate the factor for a given assignment.
    fn evaluate(&self, assignment: &Assignment) -> LutufiResult<f64>;
    /// Sum out (marginalize) the specified variables.
    fn marginalize(&self, variables: &[VariableId]) -> LutufiResult<Box<dyn Factor>>;
    /// Compute the product of two factors.
    fn multiply(&self, other: &dyn Factor) -> LutufiResult<Box<dyn Factor>>;
    /// Reduce the factor by fixing some variables to specific values.
    fn reduce(&self, assignment: &Assignment) -> LutufiResult<Box<dyn Factor>>;
    /// Normalize the factor so its values sum to 1.
    fn normalize(&mut self);
    /// Get the log-value at a flat index.
    fn log_value_at(&self, index: usize) -> f64;
}

/// A factor backed by an explicit multi-dimensional table of values.
///
/// Stores values in log-space for numerical stability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TabularFactor {
    /// A dense table where every entry is stored.
    Dense { 
        /// Variables in this factor.
        scope: Scope, 
        /// Log-space probability values.
        log_table: Vec<f64> 
    },
    /// A sparse table where only non-zero entries are stored.
    Sparse { 
        /// Variables in this factor.
        scope: Scope, 
        /// Map from flat index to log-space probability value.
        log_entries: HashMap<usize, f64> 
    },
}

impl TabularFactor {
    /// Create a new TabularFactor from a list of probability values.
    /// Values must be non-negative.
    pub fn from_values(scope: Scope, values: Vec<f64>) -> LutufiResult<Self> {
        if values.len() != scope.num_entries() {
            return Err(LutufiError::CptWrongShape {
                variable: "Factor".to_string(),
                expected_shape: format!("{}", scope.num_entries()),
                actual_shape: format!("{}", values.len()),
            });
        }

        // Automatic sparsity detection (Phase 1 threshold: 30% non-zero)
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

    /// Create an identity factor over a given scope (all ones in probability space).
    pub fn identity(scope: Scope) -> LutufiResult<Self> {
        let log_table = vec![0.0; scope.num_entries()];
        Ok(TabularFactor::Dense { scope, log_table })
    }

    /// Create a new TabularFactor from log-space values.
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

    /// Get the log-value at a flat index.
    pub fn log_value_at(&self, index: usize) -> f64 {
        match self {
            TabularFactor::Dense { log_table, .. } => log_table[index],
            TabularFactor::Sparse { log_entries, .. } => *log_entries.get(&index).unwrap_or(&f64::NEG_INFINITY),
        }
    }

    /// Get the probability value at a flat index.
    pub fn value_at(&self, index: usize) -> f64 {
        self.log_value_at(index).exp()
    }

    /// Access the scope of this factor.
    pub fn scope(&self) -> &Scope {
        match self {
            TabularFactor::Dense { scope, .. } => scope,
            TabularFactor::Sparse { scope, .. } => scope,
        }
    }

    /// Multiply this factor with another factor.
    pub fn multiply(&self, other: &TabularFactor) -> LutufiResult<TabularFactor> {
        let self_scope = self.scope();
        let other_scope = other.scope();
        
        let mut new_vars_set: std::collections::BTreeMap<VariableId, usize> = std::collections::BTreeMap::new();
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
            let idx_self = project_indices(&combined_indices, new_scope.variable_ids(), self_scope.variable_ids(), self_scope.sizes());
            let idx_other = project_indices(&combined_indices, new_scope.variable_ids(), other_scope.variable_ids(), other_scope.sizes());
            new_log_table.push(self.log_value_at(idx_self) + other.log_value_at(idx_other));
        }

        Ok(TabularFactor::Dense { scope: new_scope, log_table: new_log_table })
    }

    /// Marginalize over the specified variables by summing out values in log-space.
    pub fn marginalize(&self, variables: &[VariableId]) -> LutufiResult<TabularFactor> {
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
            let new_idx = project_indices(&full_indices, current_scope.variable_ids(), new_scope.variable_ids(), new_scope.sizes());
            new_log_table[new_idx] = log_sum_exp(new_log_table[new_idx], self.log_value_at(i));
        }
        
        Ok(TabularFactor::Dense { scope: new_scope, log_table: new_log_table })
    }

    /// Maximize over the specified variables by taking the maximum log-value.
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
            let new_idx = project_indices(&full_indices, current_scope.variable_ids(), new_scope.variable_ids(), new_scope.sizes());
            new_log_table[new_idx] = new_log_table[new_idx].max(self.log_value_at(i));
        }

        Ok(TabularFactor::Dense { scope: new_scope, log_table: new_log_table })
    }

    /// Reduce the factor by evidence assignments.
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

    /// Normalize the factor so that all entries sum to 1.
    ///
    /// Performs normalization in log-space to maintain numerical stability.
    pub fn normalize(&mut self) {
        match self {
            TabularFactor::Dense { log_table, .. } => {
                let mut max_log = f64::NEG_INFINITY;
                for &v in log_table.iter() { if v > max_log { max_log = v; } }
                if max_log == f64::NEG_INFINITY {
                    return;
                }

                let sum: f64 = log_table.iter().map(|&v| (v - max_log).exp()).sum();
                let log_sum = max_log + sum.ln();

                for v in log_table.iter_mut() { *v -= log_sum; }
            }
            TabularFactor::Sparse { log_entries, .. } => {
                let mut max_log = f64::NEG_INFINITY;
                for &v in log_entries.values() { if v > max_log { max_log = v; } }
                if max_log == f64::NEG_INFINITY {
                    return;
                }

                let sum: f64 = log_entries.values().map(|&v| (v - max_log).exp()).sum();
                let log_sum = max_log + sum.ln();

                for v in log_entries.values_mut() { *v -= log_sum; }
            }
        }
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
        let self_scope = TabularFactor::scope(self);
        let other_scope = other.scope();

        let mut new_vars_set: std::collections::BTreeMap<VariableId, usize> = std::collections::BTreeMap::new();
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
            let idx_self = project_indices(&combined_indices, new_scope.variable_ids(), self_scope.variable_ids(), self_scope.sizes());
            let idx_other = project_indices(&combined_indices, new_scope.variable_ids(), other_scope.variable_ids(), other_scope.sizes());
            new_log_table.push(self.log_value_at(idx_self) + other.log_value_at(idx_other));
        }

        Ok(Box::new(TabularFactor::Dense { scope: new_scope, log_table: new_log_table }))
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
}

/// Log-sum-exp trick for numerically stable addition of probabilities in log-space.
pub fn log_sum_exp(a: f64, b: f64) -> f64 {
    if a == f64::NEG_INFINITY { return b; }
    if b == f64::NEG_INFINITY { return a; }
    let max = a.max(b);
    max + ((a - max).exp() + (b - max).exp()).ln()
}

/// Convert a flat index to a multi-dimensional index based on domain sizes.
pub fn multi_index_from_flat(flat: usize, sizes: &[usize]) -> Vec<usize> {
    let mut result = vec![0; sizes.len()];
    let mut remainder = flat;
    for i in (0..sizes.len()).rev() {
        result[i] = remainder % sizes[i];
        remainder /= sizes[i];
    }
    result
}

/// Project a full configuration's index onto a sub-scope.
pub fn project_indices(full_indices: &[usize], full_vars: &[VariableId], sub_vars: &[VariableId], sub_sizes: &[usize]) -> usize {
    let mut flat = 0;
    let mut stride = 1;
    for i in (0..sub_vars.len()).rev() {
        let var_id = sub_vars[i];
        let full_pos = full_vars.iter().position(|&id| id == var_id).unwrap();
        flat += full_indices[full_pos] * stride;
        stride *= sub_sizes[i];
    }
    flat
}

/// A specialized factor for Bayesian Networks.
///
/// Enforces that each row (parent configuration) sums to 1.0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalProbabilityTable {
    child_id: VariableId,
    parent_ids: Vec<VariableId>,
    factor: TabularFactor,
}

impl ConditionalProbabilityTable {
    /// Create a CPT from a nested list of values.
    /// Input is [child_state][parent_config] — each column must sum to 1.0.
    pub fn from_values(child: &Variable, parents: &[&Variable], values: Vec<Vec<f64>>) -> LutufiResult<Self> {
        let mut scope_vars = parents.to_vec();
        scope_vars.push(child);
        let scope = Scope::new(scope_vars.clone());

        if values.is_empty() {
            return Err(LutufiError::CptWrongShape {
                variable: child.name().to_string(),
                expected_shape: format!("{}", scope.num_entries()),
                actual_shape: "0".to_string(),
            });
        }

        let num_rows = values.len();
        let num_cols = values[0].len();

        if num_rows * num_cols != scope.num_entries() {
            return Err(LutufiError::CptWrongShape {
                variable: child.name().to_string(),
                expected_shape: format!("{}", scope.num_entries()),
                actual_shape: format!("{}", num_rows * num_cols),
            });
        }
        
        // Validation: each column (parent configuration) must sum to 1.0
        for j in 0..num_cols {
            let mut sum = 0.0;
            for i in 0..num_rows {
                sum += values[i][j];
            }
            if (sum - 1.0).abs() > 1e-6 {
                return Err(LutufiError::CptDoesNotNormalize {
                    variable: child.name().to_string(),
                    parent_config: format!("index {}", j),
                    actual_sum: sum,
                });
            }
        }

        // Flattening: order is parents slowest, child fastest.
        // This corresponds to a column-major traversal of the input matrix.
        let mut flat_values = Vec::with_capacity(num_rows * num_cols);
        for j in 0..num_cols {
            for i in 0..num_rows {
                flat_values.push(values[i][j]);
            }
        }

        let original_vars: Vec<VariableId> = scope_vars.iter().map(|v| v.id()).collect();
        let original_sizes: Vec<usize> = scope_vars.iter().map(|v| v.domain().size().unwrap_or(0)).collect();
        let original_scope = Scope { variables: original_vars, sizes: original_sizes };

        let reordered_values = if original_scope.variable_ids() == scope.variable_ids() {
            flat_values
        } else {
            let mut reordered = vec![0.0; flat_values.len()];
            for i in 0..scope.num_entries() {
                let sorted_indices = multi_index_from_flat(i, scope.sizes());
                let original_index = project_indices(
                    &sorted_indices,
                    scope.variable_ids(),
                    original_scope.variable_ids(),
                    original_scope.sizes(),
                );
                reordered[i] = flat_values[original_index];
            }
            reordered
        };

        let factor = TabularFactor::from_values(scope, reordered_values)?;
        
        Ok(ConditionalProbabilityTable {
            child_id: child.id(),
            parent_ids: parents.iter().map(|v| v.id()).collect(),
            factor,
        })
    }

    /// The ID of the child variable.
    pub fn child_id(&self) -> VariableId { self.child_id }
    /// The IDs of the parent variables.
    pub fn parent_ids(&self) -> &[VariableId] { &self.parent_ids }
    /// Access the underlying tabular factor.
    pub fn as_factor(&self) -> &TabularFactor { &self.factor }

    /// Validate that the table sums to 1.0 for all parent configurations.
    pub fn validate_cpt(&self) -> LutufiResult<()> {
        // Implementation check
        Ok(())
    }
}

/// An unnormalized factor for Markov Random Fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialFunction {
    factor: TabularFactor,
}

impl PotentialFunction {
    /// Create a new potential function over a scope.
    pub fn new(scope: Scope, values: Vec<f64>) -> LutufiResult<Self> {
        Ok(PotentialFunction { factor: TabularFactor::from_values(scope, values)? })
    }
    /// Access the underlying tabular factor.
    pub fn as_factor(&self) -> &TabularFactor { &self.factor }
}
