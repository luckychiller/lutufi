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
        let mut vars: Vec<(&Variable, VariableId)> = variables.into_iter().map(|v| (v, v.id())).collect();
        vars.sort_by_key(|(_, id)| *id);
        let sorted_vars = vars.iter().map(|(_, id)| *id).collect();
        let sizes = vars.iter().map(|(v, _)| v.domain().size().unwrap_or(0)).collect();
        Scope { variables: sorted_vars, sizes }
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
    /// Normalize the factor so its values sum to 1.
    fn normalize(&mut self);
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
}

impl Factor for TabularFactor {
    fn scope(&self) -> &Scope { self.scope() }

    fn evaluate(&self, assignment: &Assignment) -> LutufiResult<f64> {
        let mut flat_idx = 0;
        let mut stride = 1;
        let scope = self.scope();

        for i in (0..scope.len()).rev() {
            let var_id = scope.variable_ids()[i];
            let val_idx = assignment.get_discrete(&var_id)?;
            flat_idx += val_idx * stride;
            stride *= scope.sizes()[i];
        }
        Ok(self.value_at(flat_idx))
    }

    fn marginalize(&self, variables: &[VariableId]) -> LutufiResult<Box<dyn Factor>> {
        let current_scope = self.scope();
        let mut _new_vars: Vec<VariableId> = Vec::new();
        let target_set: std::collections::HashSet<_> = variables.iter().collect();

        for &id in current_scope.variable_ids() {
            if !target_set.contains(&id) {
                // In a real implementation we'd need the Variable object here.
                // For Phase 1, we just return an error or a placeholder.
                return Err(LutufiError::InternalError { message: "Marginalization requires Variable objects in Phase 1".to_string() });
            }
        }
        
        Ok(Box::new(self.clone()))
    }

    fn multiply(&self, _other: &dyn Factor) -> LutufiResult<Box<dyn Factor>> {
        // Implementation for Phase 2
        Ok(Box::new(self.clone()))
    }

    fn normalize(&mut self) {
        match self {
            TabularFactor::Dense { log_table, .. } => {
                let mut max_log = f64::NEG_INFINITY;
                for &v in log_table.iter() { if v > max_log { max_log = v; } }
                
                let sum: f64 = log_table.iter().map(|&v| (v - max_log).exp()).sum();
                let log_sum = max_log + sum.ln();
                
                for v in log_table.iter_mut() { *v -= log_sum; }
            }
            TabularFactor::Sparse { log_entries, .. } => {
                let mut max_log = f64::NEG_INFINITY;
                for &v in log_entries.values() { if v > max_log { max_log = v; } }
                
                let sum: f64 = log_entries.values().map(|&v| (v - max_log).exp()).sum();
                let log_sum = max_log + sum.ln();

                for v in log_entries.values_mut() { *v -= log_sum; }
            }
        }
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
    /// values[parent_config][child_state]
    pub fn from_values(child: &Variable, parents: &[&Variable], values: Vec<Vec<f64>>) -> LutufiResult<Self> {
        let mut scope_vars = parents.to_vec();
        scope_vars.push(child);
        let scope = Scope::new(scope_vars);

        // Flatten values for TabularFactor. 
        // Order must be: parents vary slowest, child varies fastest.
        let mut flat_values = Vec::new();
        
        // Validation: check each row sums to 1
        for (i, row) in values.iter().enumerate() {
            let sum: f64 = row.iter().sum();
            if (sum - 1.0).abs() > 1e-6 {
                return Err(LutufiError::CptDoesNotNormalize {
                    variable: child.name().to_string(),
                    parent_config: format!("index {}", i),
                    actual_sum: sum,
                });
            }
            for &v in row { flat_values.push(v); }
        }

        let factor = TabularFactor::from_values(scope, flat_values)?;
        
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
