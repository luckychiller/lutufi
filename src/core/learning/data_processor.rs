//! Common Data Processing Utilities
//!
//! This module extracts common data processing logic used across learning modules,
//! following the DRY (Don't Repeat Yourself) principle and improving maintainability.

use crate::core::{
    error::{LutufiError, LutufiResult},
};
use std::collections::{HashMap, HashSet};

/// Utility for common data processing operations.
pub struct DataProcessor;

impl DataProcessor {
    /// Extract unique variable names from data.
    pub fn extract_variables(data: &[HashMap<String, String>]) -> Vec<String> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut vars = HashSet::new();
        for row in data {
            vars.extend(row.keys().cloned());
        }

        let mut vars: Vec<_> = vars.into_iter().collect();
        vars.sort();
        vars
    }

    /// Validate that data contains specified variables.
    pub fn validate_variables(
        data: &[HashMap<String, String>],
        required_vars: &[&str],
    ) -> LutufiResult<()> {
        if data.is_empty() {
            return Err(LutufiError::InternalError {
                message: "Cannot process empty dataset".to_string(),
            });
        }

        let data_vars: HashSet<_> = Self::extract_variables(data).into_iter().collect();
        let required_set: HashSet<_> = required_vars.iter().map(|s| s.to_string()).collect();

        let missing: Vec<_> = required_set
            .difference(&data_vars)
            .cloned()
            .collect();

        if !missing.is_empty() {
            return Err(LutufiError::InternalError {
                message: format!("Missing variables in data: {:?}", missing),
            });
        }

        Ok(())
    }

    /// Count complete cases (rows with no missing values).
    pub fn count_complete_cases(
        data: &[HashMap<String, String>],
        variables: &[&str],
    ) -> usize {
        data.iter()
            .filter(|row| variables.iter().all(|v| row.contains_key(*v)))
            .count()
    }

    /// Extract values for a specific variable, filtering out missing values.
    pub fn extract_column(
        data: &[HashMap<String, String>],
        variable: &str,
    ) -> Vec<String> {
        data.iter()
            .filter_map(|row| row.get(variable).cloned())
            .collect()
    }

    /// Get unique values for a variable (for discrete domains).
    pub fn get_unique_values(
        data: &[HashMap<String, String>],
        variable: &str,
    ) -> Vec<String> {
        let values: HashSet<_> = Self::extract_column(data, variable)
            .into_iter()
            .collect();
        let mut sorted: Vec<_> = values.into_iter().collect();
        sorted.sort();
        sorted
    }

    /// Group data rows by value of a variable.
    pub fn group_by_variable<'a>(
        data: &'a [HashMap<String, String>],
        variable: &str,
    ) -> HashMap<String, Vec<&'a HashMap<String, String>>> {
        let mut groups: HashMap<String, Vec<_>> = HashMap::new();
        for row in data {
            if let Some(value) = row.get(variable) {
                groups
                    .entry(value.clone())
                    .or_insert_with(Vec::new)
                    .push(row);
            }
        }
        groups
    }

    /// Filter data rows where a variable has a specific value.
    pub fn filter_by_value<'a>(
        data: &'a [HashMap<String, String>],
        variable: &str,
        value: &str,
    ) -> Vec<&'a HashMap<String, String>> {
        data.iter()
            .filter(|row| row.get(variable).map(|v| v == value).unwrap_or(false))
            .collect()
    }

    /// Get statistics about missing data.
    pub fn missing_data_stats(
        data: &[HashMap<String, String>],
        variables: &[&str],
    ) -> HashMap<String, (usize, f64)> {
        let total_rows = data.len() as f64;
        let mut stats = HashMap::new();

        for &var in variables {
            let missing_count = data
                .iter()
                .filter(|row| !row.contains_key(var))
                .count();
            let missing_percent = (missing_count as f64 / total_rows) * 100.0;
            stats.insert(var.to_string(), (missing_count, missing_percent));
        }

        stats
    }

    /// Perform a Chi-square independence test for discrete data.
    /// Returns (chi_square_value, degrees_of_freedom, p_value).
    pub fn chi_square_test(
        data: &[HashMap<String, String>],
        x: &str,
        y: &str,
        z: &[&str],
    ) -> LutufiResult<(f64, usize, f64)> {
        use statrs::distribution::{ChiSquared, ContinuousCDF};

        let x_states = Self::get_unique_values(data, x);
        let y_states = Self::get_unique_values(data, y);
        let z_states_list: Vec<Vec<String>> = z.iter().map(|&name| Self::get_unique_values(data, name)).collect();

        let r = x_states.len();
        let c = y_states.len();
        let q: usize = z_states_list.iter().map(|s| s.len()).product();
        if r < 2 || c < 2 { return Ok((0.0, 0, 1.0)); }

        let mut counts = vec![0.0; r * c * q];
        let z_sizes: Vec<usize> = z_states_list.iter().map(|s| s.len()).collect();

        for row in data {
            let x_val = if let Some(v) = row.get(x) { v } else { continue; };
            let y_val = if let Some(v) = row.get(y) { v } else { continue; };
            
            let x_idx = x_states.iter().position(|s| s == x_val).ok_or_else(|| LutufiError::InternalError {
                message: format!("Value '{}' not found in x_states for variable '{}'", x_val, x),
            })?;
            let y_idx = y_states.iter().position(|s| s == y_val).ok_or_else(|| LutufiError::InternalError {
                message: format!("Value '{}' not found in y_states for variable '{}'", y_val, y),
            })?;
            
            let mut z_idx = 0;
            let mut z_stride = 1;
            let mut missing_z = false;
            for i in (0..z.len()).rev() {
                let z_val = if let Some(v) = row.get(z[i]) { v } else { missing_z = true; break; };
                let val_idx = z_states_list[i].iter().position(|s| s == z_val).ok_or_else(|| LutufiError::InternalError {
                    message: format!("Value '{}' not found in z_states for variable '{}'", z_val, z[i]),
                })?;
                z_idx += val_idx * z_stride;
                z_stride *= z_sizes[i];
            }
            if missing_z { continue; }

            let idx = z_idx * (r * c) + y_idx * r + x_idx;
            counts[idx] += 1.0;
        }

        let mut total_chi2 = 0.0;
        let mut total_df = 0;

        for z_idx in 0..q {
            let mut n_ij = vec![vec![0.0; c]; r];
            let mut n_i_dot = vec![0.0; r];
            let mut n_dot_j = vec![0.0; c];
            let mut n_dot_dot = 0.0;

            for i in 0..r {
                for j in 0..c {
                    let count = counts[z_idx * (r * c) + j * r + i];
                    n_ij[i][j] = count;
                    n_i_dot[i] += count;
                    n_dot_j[j] += count;
                    n_dot_dot += count;
                }
            }

            if n_dot_dot > 0.0 {
                let mut df = 0;
                let non_zero_rows = n_i_dot.iter().filter(|&&v| v > 0.0).count();
                let non_zero_cols = n_dot_j.iter().filter(|&&v| v > 0.0).count();
                if non_zero_rows > 1 && non_zero_cols > 1 {
                    df = (non_zero_rows - 1) * (non_zero_cols - 1);
                }
                total_df += df;

                for i in 0..r {
                    for j in 0..c {
                        let expected = (n_i_dot[i] * n_dot_j[j]) / n_dot_dot;
                        if expected > 0.0 {
                            let diff = n_ij[i][j] - expected;
                            total_chi2 += (diff * diff) / expected;
                        }
                    }
                }
            }
        }

        let p_value = if total_df > 0 {
            let chi = ChiSquared::new(total_df as f64).map_err(|e| crate::core::error::LutufiError::InternalError { message: format!("Statrs error: {}", e) })?;
            1.0 - chi.cdf(total_chi2)
        } else {
            1.0
        };

        Ok((total_chi2, total_df, p_value))
    }

    /// Perform a G-test (likelihood ratio) independence test.
    /// Returns (g_value, degrees_of_freedom, p_value).
    pub fn g_test(
        data: &[HashMap<String, String>],
        x: &str,
        y: &str,
        z: &[&str],
    ) -> LutufiResult<(f64, usize, f64)> {
        use statrs::distribution::{ChiSquared, ContinuousCDF};

        // Implementation similar to chi-square but using 2 * sum(O * ln(O/E))
        let x_states = Self::get_unique_values(data, x);
        let y_states = Self::get_unique_values(data, y);
        let z_states_list: Vec<Vec<String>> = z.iter().map(|&name| Self::get_unique_values(data, name)).collect();

        let r = x_states.len();
        let c = y_states.len();
        let q: usize = z_states_list.iter().map(|s| s.len()).product();
        if r < 2 || c < 2 { return Ok((0.0, 0, 1.0)); }

        let mut counts = vec![0.0; r * c * q];
        let z_sizes: Vec<usize> = z_states_list.iter().map(|s| s.len()).collect();

        for row in data {
            let x_val = if let Some(v) = row.get(x) { v } else { continue; };
            let y_val = if let Some(v) = row.get(y) { v } else { continue; };
            
            let x_idx = x_states.iter().position(|s| s == x_val).ok_or_else(|| LutufiError::InternalError {
                message: format!("Value '{}' not found in x_states for variable '{}'", x_val, x),
            })?;
            let y_idx = y_states.iter().position(|s| s == y_val).ok_or_else(|| LutufiError::InternalError {
                message: format!("Value '{}' not found in y_states for variable '{}'", y_val, y),
            })?;
            
            let mut z_idx = 0;
            let mut z_stride = 1;
            let mut missing_z = false;
            for i in (0..z.len()).rev() {
                let z_val = if let Some(v) = row.get(z[i]) { v } else { missing_z = true; break; };
                let val_idx = z_states_list[i].iter().position(|s| s == z_val).ok_or_else(|| LutufiError::InternalError {
                    message: format!("Value '{}' not found in z_states for variable '{}'", z_val, z[i]),
                })?;
                z_idx += val_idx * z_stride;
                z_stride *= z_sizes[i];
            }
            if missing_z { continue; }

            let idx = z_idx * (r * c) + y_idx * r + x_idx;
            counts[idx] += 1.0;
        }

        let mut total_g = 0.0;
        let mut total_df = 0;

        for z_idx in 0..q {
            let mut n_ij = vec![vec![0.0; c]; r];
            let mut n_i_dot = vec![0.0; r];
            let mut n_dot_j = vec![0.0; c];
            let mut n_dot_dot = 0.0;

            for i in 0..r {
                for j in 0..c {
                    let count = counts[z_idx * (r * c) + j * r + i];
                    n_ij[i][j] = count;
                    n_i_dot[i] += count;
                    n_dot_j[j] += count;
                    n_dot_dot += count;
                }
            }

            if n_dot_dot > 0.0 {
                let mut df = 0;
                let non_zero_rows = n_i_dot.iter().filter(|&&v| v > 0.0).count();
                let non_zero_cols = n_dot_j.iter().filter(|&&v| v > 0.0).count();
                if non_zero_rows > 1 && non_zero_cols > 1 {
                    df = (non_zero_rows - 1) * (non_zero_cols - 1);
                }
                total_df += df;

                for i in 0..r {
                    for j in 0..c {
                        let observed = n_ij[i][j];
                        let expected = (n_i_dot[i] * n_dot_j[j]) / n_dot_dot;
                        if observed > 0.0 && expected > 0.0 {
                            let ratio: f64 = observed / expected;
                            total_g += 2.0 * observed * ratio.ln();
                        }
                    }
                }
            }
        }

        let p_value = if total_df > 0 {
            let chi = ChiSquared::new(total_df as f64).map_err(|e| crate::core::error::LutufiError::InternalError { message: format!("Statrs error: {}", e) })?;
            1.0 - chi.cdf(total_g)
        } else {
            1.0
        };

        Ok((total_g, total_df, p_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_variables() {
        let mut row1 = HashMap::new();
        row1.insert("A".to_string(), "1".to_string());
        row1.insert("B".to_string(), "2".to_string());

        let data = vec![row1];
        let vars = DataProcessor::extract_variables(&data);
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_extract_column() {
        let mut row1 = HashMap::new();
        row1.insert("X".to_string(), "value1".to_string());
        let mut row2 = HashMap::new();
        row2.insert("X".to_string(), "value2".to_string());

        let data = vec![row1, row2];
        let col = DataProcessor::extract_column(&data, "X");
        assert_eq!(col.len(), 2);
    }

    #[test]
    fn test_group_by_variable() {
        let mut row1 = HashMap::new();
        row1.insert("group".to_string(), "A".to_string());
        let mut row2 = HashMap::new();
        row2.insert("group".to_string(), "B".to_string());
        let mut row3 = HashMap::new();
        row3.insert("group".to_string(), "A".to_string());

        let data = vec![row1, row2, row3];
        let groups = DataProcessor::group_by_variable(&data, "group");
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.get("A").unwrap().len(), 2);
    }
}
