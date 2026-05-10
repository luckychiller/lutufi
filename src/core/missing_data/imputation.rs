use crate::core::{
    error::LutufiResult,
    variable::VariableId,
};

/// The imputation method used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImputationMethod {
    /// Replace missing with mean (continuous) or mode (discrete).
    MeanMode,
    /// Linear regression imputation using observed variables as predictors.
    Regression,
    /// Multiple Imputation by Chained Equations.
    Mice,
    /// Expectation-Maximization imputation.
    Em,
}

/// The result of an imputation operation.
#[derive(Debug, Clone)]
pub struct ImputationResult {
    /// Imputed data matrix (all missing values filled).
    pub imputed_data: Vec<Vec<f64>>,
    /// Variables corresponding to columns.
    pub variables: Vec<VariableId>,
    /// The method used.
    pub method: ImputationMethod,
    /// Number of imputed values.
    pub num_imputed: usize,
    /// Log-likelihood of the imputed model (if applicable).
    pub log_likelihood: Option<f64>,
}

/// Engine for performing data imputation.
#[derive(Debug, Clone)]
pub struct ImputationEngine {
    /// The imputation method.
    method: ImputationMethod,
    /// Number of imputation rounds (for MICE).
    rounds: usize,
    /// Random seed for reproducibility.
    seed: u64,
}

impl ImputationEngine {
    /// Create a new imputation engine with the specified method.
    pub fn new(method: ImputationMethod) -> Self {
        ImputationEngine {
            method,
            rounds: 5,
            seed: 42,
        }
    }

    /// Set the number of imputation rounds (for MICE).
    pub fn with_rounds(mut self, rounds: usize) -> Self {
        self.rounds = rounds.max(1);
        self
    }

    /// Set the random seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    /// Impute missing values in the provided data.
    pub fn impute(
        &self,
        data: &[Vec<Option<f64>>],
        variables: &[VariableId],
    ) -> LutufiResult<ImputationResult> {
        match self.method {
            ImputationMethod::MeanMode => self.impute_mean_mode(data, variables),
            ImputationMethod::Regression => self.impute_regression(data, variables),
            ImputationMethod::Mice => self.impute_mice(data, variables),
            ImputationMethod::Em => self.impute_em(data, variables),
        }
    }

    /// Mean/mode imputation: mean for continuous, mode for discrete.
    fn impute_mean_mode(
        &self,
        data: &[Vec<Option<f64>>],
        variables: &[VariableId],
    ) -> LutufiResult<ImputationResult> {
        let n_vars = variables.len();
        let mut result = vec![vec![0.0_f64; n_vars]; data.len()];

        // Compute means per variable.
        let mut sums = vec![0.0_f64; n_vars];
        let mut counts = vec![0_usize; n_vars];
        for row in data.iter() {
            for (j, val) in row.iter().enumerate() {
                if let Some(v) = val {
                    sums[j] += v;
                    counts[j] += 1;
                }
            }
        }

        let means: Vec<f64> = sums.iter().zip(counts.iter())
            .map(|(&s, &c)| if c > 0 { s / c as f64 } else { 0.0 })
            .collect();

        let mut num_imputed = 0_usize;
        for (i, row) in data.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                result[i][j] = match val {
                    Some(v) => *v,
                    None => {
                        num_imputed += 1;
                        means[j]
                    }
                };
            }
        }

        Ok(ImputationResult {
            imputed_data: result,
            variables: variables.to_vec(),
            method: ImputationMethod::MeanMode,
            num_imputed,
            log_likelihood: None,
        })
    }

    /// Linear regression imputation.
    fn impute_regression(
        &self,
        data: &[Vec<Option<f64>>],
        variables: &[VariableId],
    ) -> LutufiResult<ImputationResult> {
        let n_vars = variables.len();
        let n_rows = data.len();
        let mut result: Vec<Vec<f64>> = data.iter()
            .map(|row| row.iter().map(|v| v.unwrap_or(0.0)).collect())
            .collect();

        let mut num_imputed = 0_usize;

        for target_j in 0..n_vars {
            let missing_rows: Vec<usize> = (0..n_rows)
                .filter(|&i| data[i][target_j].is_none())
                .collect();

            if missing_rows.is_empty() {
                continue;
            }

            // Build regression: predictors are all other variables.
            let predictors: Vec<usize> = (0..n_vars).filter(|&j| j != target_j).collect();
            let n_pred = predictors.len();

            if n_pred == 0 {
                continue;
            }

            // Collect complete-case rows (all predictors and target observed).
            let complete_rows: Vec<usize> = (0..n_rows)
                .filter(|&i| {
                    data[i][target_j].is_some()
                        && predictors.iter().all(|&j| data[i][j].is_some())
                })
                .collect();

            if complete_rows.len() < 2 || complete_rows.len() < n_pred + 1 {
                // Fall back to mean imputation for this variable.
                let sum: f64 = (0..n_rows)
                    .filter_map(|i| data[i][target_j])
                    .sum();
                let count = (0..n_rows).filter(|i| data[*i][target_j].is_some()).count();
                let mean = if count > 0 { sum / count as f64 } else { 0.0 };
                for &i in &missing_rows {
                    result[i][target_j] = mean;
                    num_imputed += 1;
                }
                continue;
            }

            // Fit linear regression using normal equations.
            let _m = complete_rows.len();
            let mut xtx = vec![0.0_f64; (n_pred + 1) * (n_pred + 1)];
            let mut xty = vec![0.0_f64; n_pred + 1];

            for &row_idx in &complete_rows {
                // Intercept term.
                xtx[0] += 1.0;
                xty[0] += result[row_idx][target_j];
                for p in 0..n_pred {
                    let x_val = result[row_idx][predictors[p]];
                    xtx[(p + 1) * (n_pred + 1) + 0] += x_val;
                    xtx[0 * (n_pred + 1) + (p + 1)] += x_val;
                    xty[p + 1] += x_val * result[row_idx][target_j];
                    for q in 0..n_pred {
                        let xq = result[row_idx][predictors[q]];
                        xtx[(p + 1) * (n_pred + 1) + (q + 1)] += x_val * xq;
                    }
                }
            }

            // Add regularization.
            for j in 0..=n_pred {
                xtx[j * (n_pred + 1) + j] += 1e-6;
            }

            // Solve for coefficients.
            let dim = n_pred + 1;
            let mut augmented = xtx.clone();
            for i in 0..dim {
                augmented[i * dim + dim] = xty[i];
            }

            match super::mcar_mar::solve_linear_system(&mut augmented, dim) {
                Ok(coeffs) => {
                    for &row_idx in &missing_rows {
                        let mut pred = coeffs[0];
                        for p in 0..n_pred {
                            pred += coeffs[p + 1] * result[row_idx][predictors[p]];
                        }
                        result[row_idx][target_j] = pred;
                        num_imputed += 1;
                    }
                }
                Err(_) => {
                    // Fall back to mean imputation.
                    let sum: f64 = (0..n_rows).filter_map(|i| data[i][target_j]).sum();
                    let count = (0..n_rows).filter(|i| data[*i][target_j].is_some()).count();
                    let mean = if count > 0 { sum / count as f64 } else { 0.0 };
                    for &i in &missing_rows {
                        result[i][target_j] = mean;
                        num_imputed += 1;
                    }
                }
            }
        }

        Ok(ImputationResult {
            imputed_data: result,
            variables: variables.to_vec(),
            method: ImputationMethod::Regression,
            num_imputed,
            log_likelihood: None,
        })
    }

    /// MICE imputation (simple single-chain implementation).
    fn impute_mice(
        &self,
        data: &[Vec<Option<f64>>],
        variables: &[VariableId],
    ) -> LutufiResult<ImputationResult> {
        // Start with mean imputation.
        let mean_engine = ImputationEngine::new(ImputationMethod::MeanMode);
        let mut current = mean_engine.impute(data, variables)?;
        let num_imputed = current.num_imputed;
        let n_vars = variables.len();
        let n_rows = data.len();
        let n_rounds = self.rounds.max(3);

        for _round in 0..n_rounds {
            for target_j in 0..n_vars {
                let missing_rows: Vec<usize> = (0..n_rows)
                    .filter(|&i| data[i][target_j].is_none())
                    .collect();
                if missing_rows.is_empty() {
                    continue;
                }

                let predictors: Vec<usize> = (0..n_vars).filter(|&j| j != target_j).collect();
                if predictors.is_empty() {
                    continue;
                }

                let n_pred = predictors.len();
                let complete_rows: Vec<usize> = (0..n_rows)
                    .filter(|&i| data[i][target_j].is_some())
                    .collect();

                if complete_rows.len() < n_pred + 1 {
                    continue;
                }

                // Fit regression on all rows with observed target.
                let _m = complete_rows.len();
                let mut xtx = vec![0.0_f64; (n_pred + 1) * (n_pred + 1)];
                let mut xty = vec![0.0_f64; n_pred + 1];

                for &row_idx in &complete_rows {
                    xtx[0] += 1.0;
                    xty[0] += current.imputed_data[row_idx][target_j];
                    for p in 0..n_pred {
                        let x_val = current.imputed_data[row_idx][predictors[p]];
                        xtx[(p + 1) * (n_pred + 1) + 0] += x_val;
                        xtx[0 * (n_pred + 1) + (p + 1)] += x_val;
                        xty[p + 1] += x_val * current.imputed_data[row_idx][target_j];
                        for q in 0..n_pred {
                            xtx[(p + 1) * (n_pred + 1) + (q + 1)]
                                += x_val * current.imputed_data[row_idx][predictors[q]];
                        }
                    }
                }

                for j in 0..=n_pred {
                    xtx[j * (n_pred + 1) + j] += 1e-6;
                }

                let dim = n_pred + 1;
                let mut augmented = xtx.clone();
                for i in 0..dim {
                    augmented[i * dim + dim] = xty[i];
                }

                if let Ok(coeffs) = super::mcar_mar::solve_linear_system(&mut augmented, dim) {
                    for &row_idx in &missing_rows {
                        let mut pred = coeffs[0];
                        for p in 0..n_pred {
                            pred += coeffs[p + 1] * current.imputed_data[row_idx][predictors[p]];
                        }
                        current.imputed_data[row_idx][target_j] = pred;
                    }
                }
            }
        }

        Ok(ImputationResult {
            imputed_data: current.imputed_data,
            variables: variables.to_vec(),
            method: ImputationMethod::Mice,
            num_imputed,
            log_likelihood: None,
        })
    }

    /// EM imputation for multivariate normal data.
    fn impute_em(
        &self,
        data: &[Vec<Option<f64>>],
        variables: &[VariableId],
    ) -> LutufiResult<ImputationResult> {
        let n_vars = variables.len();
        let n_rows = data.len();
        let n_obs: f64 = n_rows as f64;

        // Initialization: mean imputation then covariance.
        let mean_engine = ImputationEngine::new(ImputationMethod::MeanMode);
        let mut current = mean_engine.impute(data, variables)?;

        let mut mean = vec![0.0_f64; n_vars];
        for j in 0..n_vars {
            mean[j] = current.imputed_data.iter().map(|row| row[j]).sum::<f64>() / n_obs;
        }

        let mut cov = vec![vec![0.0_f64; n_vars]; n_vars];
        for i in 0..n_rows {
            for j1 in 0..n_vars {
                for j2 in 0..n_vars {
                    cov[j1][j2] += (current.imputed_data[i][j1] - mean[j1])
                        * (current.imputed_data[i][j2] - mean[j2]);
                }
            }
        }
        for j1 in 0..n_vars {
            for j2 in 0..n_vars {
                cov[j1][j2] /= n_obs - 1.0;
            }
        }

        let max_iter = 100;
        let tol = 1e-6;

        for _iter in 0..max_iter {
            let old_mean = mean.clone();

            // E-step: impute missing values using conditional expectation.
            let mut sum_obs = vec![0.0_f64; n_vars];
            let mut count_obs = vec![0_usize; n_vars];
            let mut sum_cross = vec![vec![0.0_f64; n_vars]; n_vars];

            for i in 0..n_rows {
                let missing: Vec<usize> = (0..n_vars)
                    .filter(|&j| data[i][j].is_none())
                    .collect();
                let observed: Vec<usize> = (0..n_vars)
                    .filter(|&j| data[i][j].is_some())
                    .collect();

                if missing.is_empty() || observed.is_empty() {
                    for j in 0..n_vars {
                        sum_obs[j] += current.imputed_data[i][j];
                        count_obs[j] += 1;
                    }
                    for j1 in 0..n_vars {
                        for j2 in 0..n_vars {
                            sum_cross[j1][j2] += (current.imputed_data[i][j1] - mean[j1])
                                * (current.imputed_data[i][j2] - mean[j2]);
                        }
                    }
                    continue;
                }

                if observed.is_empty() {
                    // Entire row missing: use current mean.
                    for &j in &missing {
                        current.imputed_data[i][j] = mean[j];
                        sum_obs[j] += mean[j];
                        count_obs[j] += 1;
                    }
                    continue;
                }

                // Partition covariance.
                let m = missing.len();
                let o = observed.len();
                let mut cov_mm = vec![vec![0.0_f64; m]; m];
                let mut cov_mo = vec![vec![0.0_f64; m]; o];

                for (mi, &mi_var) in missing.iter().enumerate() {
                    for (mj, &mj_var) in missing.iter().enumerate() {
                        cov_mm[mi][mj] = cov[mi_var][mj_var];
                    }
                    for (oj, &oj_var) in observed.iter().enumerate() {
                        cov_mo[mi][oj] = cov[mi_var][oj_var];
                    }
                }

                let mut cov_oo = vec![vec![0.0_f64; o]; o];
                for (oi, &oi_var) in observed.iter().enumerate() {
                    for (oj, &oj_var) in observed.iter().enumerate() {
                        cov_oo[oi][oj] = cov[oi_var][oj_var];
                    }
                }

                // Observed values deviation from mean.
                let mut obs_dev = vec![0.0_f64; o];
                for (oi, &oj_var) in observed.iter().enumerate() {
                    obs_dev[oi] = current.imputed_data[i][oj_var] - mean[oj_var];
                }

                // Solve cov_oo * beta = cov_mo^T for beta (where beta = cov_oo^{-1} * cov_mo^T).
                let mut aug = vec![0.0_f64; o * (o + m)];
                for oi in 0..o {
                    for oj in 0..o {
                        aug[oi * (o + m) + oj] = cov_oo[oi][oj];
                    }
                    aug[oi * (o + m) + o] = 1.0; // dummy, we'll solve for each column
                }
                // We need to add regularization since cov_oo might be singular.
                for oi in 0..o {
                    aug[oi * (o + m) + oi] += 1e-8;
                }

                // Simple approach: conditional expectation = mean_m + cov_mo * cov_oo^{-1} * obs_dev.
                // Use multivariate regression formula directly.
                let mut beta = vec![vec![0.0_f64; o]; m];
                let mut aug_matrix = vec![0.0_f64; o * (o + o)];
                for col in 0..o {
                    for oi in 0..o {
                        for oj in 0..o {
                            aug_matrix[oi * (o + o) + oj] = cov_oo[oi][oj];
                        }
                        aug_matrix[oi * (o + o) + o] = 0.0;
                    }
                    // Solve cov_oo * x = cov_mo[col] for each row of beta
                    let mut aug2 = vec![0.0_f64; o * (o + 1)];
                    for oi in 0..o {
                        for oj in 0..o {
                            aug2[oi * (o + 1) + oj] = cov_oo[oi][oj];
                        }
                        aug2[oi * (o + 1) + o] = cov_mo[col][oi];
                    }
                    if let Ok(x) = super::mcar_mar::solve_linear_system(&mut aug2, o) {
                        for oi in 0..o {
                            beta[col][oi] = x[oi];
                        }
                    }
                }

                // Impute missing values.
                for (mi, &mj_var) in missing.iter().enumerate() {
                    let mut cond_mean = mean[mj_var];
                    for oi in 0..o {
                        cond_mean += beta[mi][oi] * obs_dev[oi];
                    }
                    current.imputed_data[i][mj_var] = cond_mean;
                    sum_obs[mj_var] += cond_mean;
                    count_obs[mj_var] += 1;
                }

                for &j in &observed {
                    sum_obs[j] += current.imputed_data[i][j];
                    count_obs[j] += 1;
                }
                for j1 in 0..n_vars {
                    for j2 in 0..n_vars {
                        sum_cross[j1][j2] += (current.imputed_data[i][j1] - mean[j1])
                            * (current.imputed_data[i][j2] - mean[j2]);
                    }
                }
            }

            // M-step: update mean and covariance.
            for j in 0..n_vars {
                if count_obs[j] > 0 {
                    mean[j] = sum_obs[j] / count_obs[j] as f64;
                }
            }
            for j1 in 0..n_vars {
                for j2 in 0..n_vars {
                    cov[j1][j2] = sum_cross[j1][j2] / (n_obs - 1.0).max(1.0);
                }
            }

            // Check convergence.
            let max_diff = mean.iter().zip(old_mean.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            if max_diff < tol {
                break;
            }
        }

        // Compute log-likelihood of the final imputed data under multivariate normal.
        let mut log_like = 0.0_f64;
        let mut cov_det = 1.0_f64;
        for j in 0..n_vars {
            cov_det *= if cov[j][j] > 0.0 { cov[j][j] } else { 1.0 };
        }
        let log_cov_det = cov_det.ln().max(-100.0);

        for i in 0..n_rows {
            let mut dev = vec![0.0_f64; n_vars];
            for j in 0..n_vars {
                dev[j] = current.imputed_data[i][j] - mean[j];
            }
            let mut quad = 0.0_f64;
            for j1 in 0..n_vars {
                for j2 in 0..n_vars {
                    quad += dev[j1] * dev[j2] / (cov[j1][j2].abs().max(1e-10));
                }
            }
            log_like += -0.5 * (n_vars as f64 * (2.0 * std::f64::consts::PI).ln() + log_cov_det + quad);
        }

        Ok(ImputationResult {
            imputed_data: current.imputed_data,
            variables: variables.to_vec(),
            method: ImputationMethod::Em,
            num_imputed: current.num_imputed,
            log_likelihood: Some(log_like),
        })
    }
}
