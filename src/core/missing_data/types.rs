use crate::core::{
    error::{LutufiError, LutufiResult},
    variable::VariableId,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// The mechanism governing missing data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissingDataMechanism {
    /// Missing Completely At Random: missingness independent of both observed and unobserved data.
    MCAR,
    /// Missing At Random: missingness depends only on observed data.
    MAR,
    /// Missing Not At Random: missingness depends on unobserved data.
    MNAR,
}

/// Describes which values are missing in a dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDataPattern {
    /// Variable IDs mapped to the set of row indices where they are missing.
    pub missing_by_variable: HashMap<VariableId, HashSet<usize>>,
    /// Total number of rows in the dataset.
    pub total_rows: usize,
    /// Total number of variables.
    pub total_variables: usize,
}

impl MissingDataPattern {
    /// Create a new pattern from per-variable missing indices.
    pub fn new(
        missing_by_variable: HashMap<VariableId, HashSet<usize>>,
        total_rows: usize,
        total_variables: usize,
    ) -> Self {
        MissingDataPattern { missing_by_variable, total_rows, total_variables }
    }

    /// Overall fraction of missing entries.
    pub fn missing_fraction(&self) -> f64 {
        let total_cells = self.total_rows * self.total_variables;
        if total_cells == 0 {
            return 0.0;
        }
        let missing_count: usize = self.missing_by_variable.values().map(|s| s.len()).sum();
        missing_count as f64 / total_cells as f64
    }

    /// Fraction of missing entries for a specific variable.
    pub fn variable_missing_fraction(&self, var_id: &VariableId) -> f64 {
        let count = self.missing_by_variable.get(var_id).map_or(0, |s| s.len());
        if self.total_rows == 0 { 0.0 } else { count as f64 / self.total_rows as f64 }
    }

    /// Row indices that are complete (no missing values).
    pub fn complete_rows(&self) -> HashSet<usize> {
        let all_rows: HashSet<usize> = (0..self.total_rows).collect();
        let missing_rows: HashSet<usize> = self.missing_by_variable
            .values()
            .flat_map(|s| s.iter().copied())
            .collect();
        all_rows.difference(&missing_rows).copied().collect()
    }

    /// Check if a specific row-variable cell is missing.
    pub fn is_missing(&self, var_id: &VariableId, row: usize) -> bool {
        self.missing_by_variable.get(var_id).map_or(false, |s| s.contains(&row))
    }
}

/// Summary statistics for missing data patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSummary {
    /// Number of distinct missingness patterns.
    pub num_patterns: usize,
    /// Size of the largest pattern group.
    pub largest_pattern_size: usize,
    /// Fraction of rows that are complete.
    pub complete_fraction: f64,
    /// Per-variable missing fractions.
    pub variable_fractions: Vec<(VariableId, f64)>,
    /// Overall missing fraction.
    pub overall_fraction: f64,
}

/// A container for data with missing values.
///
/// Rows are stored as `Vec<Option<f64>>` where `None` denotes a missing value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingValues {
    /// Variable IDs corresponding to columns.
    pub variables: Vec<VariableId>,
    /// Data matrix: rows × columns of optional values.
    pub data: Vec<Vec<Option<f64>>>,
}

impl MissingValues {
    /// Create a new missing-values container.
    pub fn new(variables: Vec<VariableId>, data: Vec<Vec<Option<f64>>>) -> LutufiResult<Self> {
        if variables.is_empty() {
            return Err(LutufiError::InternalError {
                message: "MissingValues must have at least one variable".to_string(),
            });
        }
        let ncols = variables.len();
        for (i, row) in data.iter().enumerate() {
            if row.len() != ncols {
                return Err(LutufiError::InternalError {
                    message: format!("Row {i} has {} columns, expected {ncols}", row.len()),
                });
            }
        }
        Ok(MissingValues { variables, data })
    }

    /// Number of rows.
    pub fn nrows(&self) -> usize { self.data.len() }
    /// Number of columns.
    pub fn ncols(&self) -> usize { self.variables.len() }

    /// Detect the missing data pattern.
    pub fn pattern(&self) -> MissingDataPattern {
        let mut missing_by_variable: HashMap<VariableId, HashSet<usize>> = HashMap::new();
        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                if val.is_none() {
                    let var_id = self.variables[col_idx];
                    missing_by_variable.entry(var_id).or_default().insert(row_idx);
                }
            }
        }
        MissingDataPattern::new(missing_by_variable, self.nrows(), self.ncols())
    }

    /// Create a summary of missing data patterns.
    pub fn summarize(&self) -> PatternSummary {
        let pattern = self.pattern();
        let complete = pattern.complete_rows();
        let n = self.nrows();

        let variable_fractions: Vec<(VariableId, f64)> = self.variables
            .iter()
            .map(|&vid| (vid, pattern.variable_missing_fraction(&vid)))
            .collect();

        let unique_patterns: HashSet<Vec<bool>> = self.data.iter().map(|row| {
            row.iter().map(|v| v.is_none()).collect()
        }).collect();

        PatternSummary {
            num_patterns: unique_patterns.len(),
            largest_pattern_size: 0,
            complete_fraction: complete.len() as f64 / n.max(1) as f64,
            variable_fractions,
            overall_fraction: pattern.missing_fraction(),
        }
    }
}

/// A boolean mask indicating which entries are missing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingMask {
    /// Variables corresponding to columns.
    pub variables: Vec<VariableId>,
    /// Boolean matrix: `true` means the value is missing.
    pub mask: Vec<Vec<bool>>,
}

impl MissingMask {
    /// Create a mask from a MissingValues instance.
    pub fn from_values(values: &MissingValues) -> Self {
        let mask = values.data.iter()
            .map(|row| row.iter().map(|v| v.is_none()).collect())
            .collect();
        MissingMask { variables: values.variables.clone(), mask }
    }

    /// Number of rows in the mask.
    pub fn nrows(&self) -> usize { self.mask.len() }
    /// Number of columns.
    pub fn ncols(&self) -> usize { self.variables.len() }

    /// The proportion of entries marked as missing.
    pub fn missing_fraction(&self) -> f64 {
        if self.mask.is_empty() || self.variables.is_empty() {
            return 0.0;
        }
        let total = (self.nrows() * self.ncols()) as f64;
        let missing: usize = self.mask.iter().map(|row| row.iter().filter(|&&m| m).count()).sum();
        missing as f64 / total
    }
}
