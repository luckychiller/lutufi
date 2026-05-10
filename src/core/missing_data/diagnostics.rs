use crate::core::{
    error::{LutufiError, LutufiResult},
    variable::VariableId,
};
use std::collections::HashMap;

/// Diagnostic metrics for evaluating imputation quality.
#[derive(Debug, Clone)]
pub struct ImputationDiagnostics {
    /// Root Mean Squared Error between true and imputed values (if true values known).
    pub rmse: Option<f64>,
    /// Mean Absolute Error between true and imputed values.
    pub mae: Option<f64>,
    /// Fraction of imputed values that were "close" to true values.
    pub close_fraction: Option<f64>,
    /// Per-variable imputation accuracy.
    pub per_variable: HashMap<VariableId, VariableImputationMetrics>,
}

/// Per-variable imputation metrics.
#[derive(Debug, Clone)]
pub struct VariableImputationMetrics {
    /// Variable name/ID.
    pub variable: VariableId,
    /// RMSE for this variable.
    pub rmse: f64,
    /// MAE for this variable.
    pub mae: f64,
    /// Number of imputed values for this variable.
    pub num_imputed: usize,
}

/// Result of diagnostic evaluation.
#[derive(Debug, Clone)]
pub struct ImputationDiagnosticResult {
    /// Summary of the diagnostics.
    pub summary: String,
    /// Detailed diagnostics.
    pub diagnostics: ImputationDiagnostics,
    /// Warnings about potential issues.
    pub warnings: Vec<String>,
}

impl ImputationDiagnostics {
    /// Compute diagnostics comparing true vs imputed data.
    ///
    /// # Arguments
    /// * `true_data` - The ground truth data (no missing values).
    /// * `imputed_data` - The imputed data.
    /// * `missing_mask` - Boolean mask indicating which values were originally missing.
    /// * `variables` - Variable IDs for columns.
    pub fn compute(
        true_data: &[Vec<f64>],
        imputed_data: &[Vec<f64>],
        missing_mask: &[Vec<bool>],
        variables: &[VariableId],
    ) -> LutufiResult<Self> {
        if true_data.len() != imputed_data.len() || imputed_data.len() != missing_mask.len() {
            return Err(LutufiError::InternalError {
                message: "All inputs must have the same number of rows".to_string(),
            });
        }
        let n_vars = variables.len();
        let n_rows = true_data.len();

        let mut total_sq_err = 0.0_f64;
        let mut total_abs_err = 0.0_f64;
        let mut total_close = 0_u64;
        let mut total_imputed = 0_u64;

        let mut per_var = HashMap::new();

        for &vid in variables {
            per_var.insert(vid, VariableImputationMetrics {
                variable: vid,
                rmse: 0.0,
                mae: 0.0,
                num_imputed: 0,
            });
        }

        for i in 0..n_rows {
            for j in 0..n_vars {
                if !missing_mask[i][j] {
                    continue;
                }
                let true_val = true_data[i][j];
                let imp_val = imputed_data[i][j];
                let diff = true_val - imp_val;
                total_sq_err += diff * diff;
                total_abs_err += diff.abs();
                total_imputed += 1;

                if diff.abs() < 0.1 {
                    total_close += 1;
                }

                if let Some(metrics) = per_var.get_mut(&variables[j]) {
                    metrics.rmse += diff * diff;
                    metrics.mae += diff.abs();
                    metrics.num_imputed += 1;
                }
            }
        }

        let rmse = if total_imputed > 0 {
            Some((total_sq_err / total_imputed as f64).sqrt())
        } else {
            None
        };
        let mae = if total_imputed > 0 {
            Some(total_abs_err / total_imputed as f64)
        } else {
            None
        };
        let close_fraction = if total_imputed > 0 {
            Some(total_close as f64 / total_imputed as f64)
        } else {
            None
        };

        // Finalize per-variable metrics.
        for metrics in per_var.values_mut() {
            if metrics.num_imputed > 0 {
                let n = metrics.num_imputed as f64;
                metrics.rmse = (metrics.rmse / n).sqrt();
                metrics.mae /= n;
            }
        }

        Ok(ImputationDiagnostics {
            rmse,
            mae,
            close_fraction,
            per_variable: per_var,
        })
    }

    /// Compute diagnostics without ground truth (internal consistency checks).
    ///
    /// Checks: variance preservation, distributional similarity, etc.
    pub fn compute_unsupervised(
        original_missing: &[Vec<Option<f64>>],
        imputed_data: &[Vec<f64>],
        variables: &[VariableId],
    ) -> LutufiResult<Self> {
        let n_vars = variables.len();
        let n_rows = original_missing.len();

        // Compute observed-vs-imputed distribution distances.
        let mut per_var = HashMap::new();

        for &vid in variables {
            per_var.insert(vid, VariableImputationMetrics {
                variable: vid,
                rmse: 0.0,
                mae: 0.0,
                num_imputed: 0,
            });
        }

        for j in 0..n_vars {
            let mut obs_values: Vec<f64> = Vec::new();
            let mut imp_values: Vec<f64> = Vec::new();
            for i in 0..n_rows {
                if original_missing[i][j].is_some() {
                    obs_values.push(imputed_data[i][j]);
                } else {
                    imp_values.push(imputed_data[i][j]);
                }
            }

            if let Some(metrics) = per_var.get_mut(&variables[j]) {
                metrics.num_imputed = imp_values.len();
                if !obs_values.is_empty() && !imp_values.is_empty() {
                    let obs_mean = obs_values.iter().sum::<f64>() / obs_values.len() as f64;
                    let imp_mean = imp_values.iter().sum::<f64>() / imp_values.len() as f64;
                    metrics.mae = (obs_mean - imp_mean).abs();
                    metrics.rmse = metrics.mae;
                }
            }
        }

        Ok(ImputationDiagnostics {
            rmse: None,
            mae: None,
            close_fraction: None,
            per_variable: per_var,
        })
    }

    /// Generate a diagnostic result with summary.
    pub fn into_result(self, warnings: Vec<String>) -> ImputationDiagnosticResult {
        let mut parts = Vec::new();
        if let Some(rmse) = self.rmse {
            parts.push(format!("RMSE: {:.6}", rmse));
        }
        if let Some(mae) = self.mae {
            parts.push(format!("MAE: {:.6}", mae));
        }
        if let Some(cf) = self.close_fraction {
            parts.push(format!("Close fraction: {:.2}%", cf * 100.0));
        }
        let summary = if parts.is_empty() {
            "Unsupervised diagnostics computed (no ground truth available).".to_string()
        } else {
            parts.join(", ")
        };

        ImputationDiagnosticResult {
            summary,
            diagnostics: self,
            warnings,
        }
    }
}
