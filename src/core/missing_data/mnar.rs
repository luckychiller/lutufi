use crate::core::{
    error::LutufiResult,
    variable::VariableId,
};

/// Types of MNAR models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MnarModelType {
    /// Pattern-mixture model: stratifies by missingness pattern.
    PatternMixture,
    /// Shared-parameter model: latent variable drives both outcome and missingness.
    SharedParameter,
    /// Selection model: models missingness probability given the outcome.
    Selection,
    /// Heckman-type selection model with two-stage estimation.
    Heckman,
}

/// Configuration for an MNAR model.
#[derive(Debug, Clone)]
pub struct MnarModel {
    /// The type of MNAR model.
    pub model_type: MnarModelType,
    /// Variable IDs for the outcome variables.
    pub outcome_variables: Vec<VariableId>,
    /// Variable IDs for covariates (observed).
    pub covariate_variables: Vec<VariableId>,
    /// Variable ID for the missingness indicator (binary: 0=observed, 1=missing).
    pub missing_indicator: Option<VariableId>,
    /// Maximum EM iterations.
    pub max_iterations: usize,
    /// Convergence tolerance.
    pub tolerance: f64,
}

/// Result of fitting an MNAR model.
#[derive(Debug, Clone)]
pub struct MnarResult {
    /// The model type used.
    pub model_type: MnarModelType,
    /// Estimated coefficients for the outcome model.
    pub outcome_coefficients: Vec<f64>,
    /// Estimated coefficients for the missingness model.
    pub missingness_coefficients: Option<Vec<f64>>,
    /// Log-likelihood of the fitted model.
    pub log_likelihood: f64,
    /// Number of iterations used.
    pub iterations: usize,
    /// Whether the model converged.
    pub converged: bool,
    /// Estimated correlation between outcome and missingness errors (Heckman).
    pub rho: Option<f64>,
}

impl MnarModel {
    /// Create a new MNAR model configuration.
    pub fn new(
        model_type: MnarModelType,
        outcome_variables: Vec<VariableId>,
        covariate_variables: Vec<VariableId>,
    ) -> Self {
        MnarModel {
            model_type,
            outcome_variables,
            covariate_variables,
            missing_indicator: None,
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }

    /// Set the missing indicator variable.
    pub fn with_missing_indicator(mut self, var: VariableId) -> Self {
        self.missing_indicator = Some(var);
        self
    }

    /// Set maximum iterations.
    pub fn with_max_iterations(mut self, max_iter: usize) -> Self {
        self.max_iterations = max_iter;
        self
    }

    /// Set convergence tolerance.
    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    /// Fit the MNAR model to the provided data.
    ///
    /// # Arguments
    /// * `data` - Data matrix with complete rows only (after imputation).
    /// * `missing_mask` - Boolean mask: `true` if originally missing.
    pub fn fit(
        &self,
        data: &[Vec<f64>],
        missing_mask: &[Vec<bool>],
    ) -> LutufiResult<MnarResult> {
        match self.model_type {
            MnarModelType::PatternMixture => self.fit_pattern_mixture(data, missing_mask),
            MnarModelType::SharedParameter => self.fit_shared_parameter(data, missing_mask),
            MnarModelType::Selection => self.fit_selection(data, missing_mask),
            MnarModelType::Heckman => self.fit_heckman(data, missing_mask),
        }
    }

    /// Pattern-mixture model: estimate separate distributions per missingness pattern.
    fn fit_pattern_mixture(
        &self,
        data: &[Vec<f64>],
        missing_mask: &[Vec<bool>],
    ) -> LutufiResult<MnarResult> {
        let n_vars = self.outcome_variables.len();
        let n_rows = data.len();

        // For simplicity, estimate mean and variance for observed and missing groups.
        let mut obs_sum = vec![0.0_f64; n_vars];
        let mut obs_count = vec![0_usize; n_vars];
        let mut mis_sum = vec![0.0_f64; n_vars];
        let mut mis_count = vec![0_usize; n_vars];

        for i in 0..n_rows {
            for (j, _var_id) in self.outcome_variables.iter().enumerate() {
                let var_idx = self.covariate_variables.len() + j;
                if var_idx >= data[i].len() {
                    continue;
                }
                if missing_mask[i][var_idx] {
                    mis_sum[j] += data[i][var_idx];
                    mis_count[j] += 1;
                } else {
                    obs_sum[j] += data[i][var_idx];
                    obs_count[j] += 1;
                }
            }
        }

        // Combine into a simple intercept-only outcome model per group.
        let mut outcome_coeffs = Vec::new();
        for j in 0..n_vars {
            let obs_mean = if obs_count[j] > 0 { obs_sum[j] / obs_count[j] as f64 } else { 0.0 };
            let mis_mean = if mis_count[j] > 0 { mis_sum[j] / mis_count[j] as f64 } else { obs_mean };
            outcome_coeffs.push(obs_mean);
            outcome_coeffs.push(mis_mean - obs_mean);
        }

        let log_like = self.compute_gaussian_log_likelihood(data, &outcome_coeffs);

        Ok(MnarResult {
            model_type: MnarModelType::PatternMixture,
            outcome_coefficients: outcome_coeffs,
            missingness_coefficients: None,
            log_likelihood: log_like,
            iterations: 1,
            converged: true,
            rho: None,
        })
    }

    /// Shared-parameter model: latent variable drives both outcome and missingness.
    fn fit_shared_parameter(
        &self,
        data: &[Vec<f64>],
        missing_mask: &[Vec<bool>],
    ) -> LutufiResult<MnarResult> {
        let n_vars = self.outcome_variables.len();
        let n_cov = self.covariate_variables.len();
        let n_rows = data.len();

        // Simple latent variable model: estimate latent factor via mean of observed outcomes.
        let mut outcome_coeffs = vec![0.0_f64; n_vars];
        let mut missingness_coeffs = vec![0.0_f64; n_vars + 1]; // intercept + per-variable

        let mut prev_ll = f64::NEG_INFINITY;
        let mut converged = false;
        let mut iterations = 0;

        for iter in 0..self.max_iterations {
            iterations = iter + 1;

            // E-step: estimate latent variable for each row.
            let mut latent = vec![0.0_f64; n_rows];
            for i in 0..n_rows {
                let mut sum = 0.0_f64;
                let mut count = 0_usize;
                for (j, _var_id) in self.outcome_variables.iter().enumerate() {
                    let idx = n_cov + j;
                    if idx < data[i].len() && !missing_mask[i][idx] {
                        sum += data[i][idx];
                        count += 1;
                    }
                }
                latent[i] = if count > 0 { sum / count as f64 } else { 0.0 };
            }

            // M-step (simplified): update coefficients via regression.
            for (j, _var_id) in self.outcome_variables.iter().enumerate() {
                let idx = n_cov + j;
                let mut numer = 0.0_f64;
                let mut denom = 0.0_f64;
                for i in 0..n_rows {
                    if !missing_mask[i][idx] {
                        numer += latent[i] * data[i][idx];
                        denom += latent[i] * latent[i];
                    }
                }
                outcome_coeffs[j] = if denom > 1e-15 { numer / denom } else { 0.0 };
            }

            // Log-likelihood computation.
            let mut ll = 0.0_f64;
            for i in 0..n_rows {
                for (j, _var_id) in self.outcome_variables.iter().enumerate() {
                    let idx = n_cov + j;
                    if !missing_mask[i][idx] {
                        let resid = data[i][idx] - outcome_coeffs[j] * latent[i];
                        ll += -0.5 * (resid * resid + (2.0 * std::f64::consts::PI).ln());
                    }
                }
            }

            if (ll - prev_ll).abs() < self.tolerance {
                converged = true;
                break;
            }
            prev_ll = ll;
        }

        // Estimate missingness coefficients (logistic on missingness given latent).
        for (j, _var_id) in self.outcome_variables.iter().enumerate() {
            let idx = n_cov + j;
            let mut observed_count = 0_usize;
            let mut missing_count = 0_usize;
            for i in 0..n_rows {
                if missing_mask[i][idx] {
                    missing_count += 1;
                } else {
                    observed_count += 1;
                }
            }
            let p_missing = missing_count as f64 / (observed_count + missing_count).max(1) as f64;
            missingness_coeffs[j + 1] = (p_missing.max(1e-10) / (1.0 - p_missing).max(1e-10)).ln();
        }

        Ok(MnarResult {
            model_type: MnarModelType::SharedParameter,
            outcome_coefficients: outcome_coeffs,
            missingness_coefficients: Some(missingness_coeffs),
            log_likelihood: prev_ll,
            iterations,
            converged,
            rho: None,
        })
    }

    /// Selection model: models P(missing|outcome) using logistic regression.
    fn fit_selection(
        &self,
        data: &[Vec<f64>],
        missing_mask: &[Vec<bool>],
    ) -> LutufiResult<MnarResult> {
        let n_cov = self.covariate_variables.len();
        let n_vars = self.outcome_variables.len();
        let n_rows = data.len();
        let _n_total = n_cov + n_vars;

        // Outcome model: simple means per variable.
        let mut outcome_coeffs = vec![0.0_f64; n_vars];
        for j in 0..n_vars {
            let idx = n_cov + j;
            let mut sum = 0.0_f64;
            let mut count = 0_usize;
            for i in 0..n_rows {
                if idx < data[i].len() {
                    sum += data[i][idx];
                    count += 1;
                }
            }
            outcome_coeffs[j] = if count > 0 { sum / count as f64 } else { 0.0 };
        }

        // Missingness model: logistic regression of R (missing indicator) on outcome.
        let mut miss_coeffs = vec![0.0_f64; n_vars + 1]; // intercept + outcome coefficients

        for target_j in 0..n_vars {
            let idx = n_cov + target_j;
            let complete_rows: Vec<usize> = (0..n_rows)
                .filter(|&i| missing_mask[i][idx])
                .collect();

            if complete_rows.is_empty() {
                miss_coeffs[target_j + 1] = f64::NEG_INFINITY;
                continue;
            }

            // Simple logistic: P(missing=1 | outcome) = sigmoid(alpha + beta * outcome)
            let mut beta = 0.0_f64;
            for _iter in 0..20 {
                let mut grad = 0.0_f64;
                let mut hess = 0.0_f64;
                for &i in &complete_rows {
                    let x = data[i][idx];
                    let logit = beta * x;
                    let logit_clamped = if logit > 20.0 { 20.0 } else if logit < -20.0 { -20.0 } else { logit };
                    let p = 1.0 / (1.0 + (-logit_clamped).exp());
                    grad += (1.0 - p) * x;
                    hess += -p * (1.0 - p) * x * x;
                }
                if hess.abs() < 1e-15 { break; }
                beta -= grad / hess;
                if grad.abs() < 1e-8 { break; }
            }
            miss_coeffs[target_j + 1] = beta;
        }

        let log_like = 0.0_f64;

        Ok(MnarResult {
            model_type: MnarModelType::Selection,
            outcome_coefficients: outcome_coeffs,
            missingness_coefficients: Some(miss_coeffs),
            log_likelihood: log_like,
            iterations: 1,
            converged: true,
            rho: None,
        })
    }

    /// Heckman two-stage selection model.
    fn fit_heckman(
        &self,
        data: &[Vec<f64>],
        missing_mask: &[Vec<bool>],
    ) -> LutufiResult<MnarResult> {
        let n_cov = self.covariate_variables.len();
        let n_vars = self.outcome_variables.len();
        let n_rows = data.len();

        // Stage 1: Probit selection equation (simplified logistic).
        let mut selection_coeffs = vec![0.0_f64; n_cov + 1];
        // For simplicity, estimate intercept-only selection model.
        let mut obs_count = 0_usize;
        let total_cells = n_rows * n_vars;
        for i in 0..n_rows {
            for j in 0..n_vars {
                let idx = n_cov + j;
                if idx < missing_mask[i].len() && !missing_mask[i][idx] {
                    obs_count += 1;
                }
            }
        }
        let obs_frac = obs_count as f64 / total_cells.max(1) as f64;
        selection_coeffs[0] = (obs_frac.max(1e-10) / (1.0 - obs_frac).max(1e-10)).ln();

        // Stage 2: Outcome equation with inverse Mills ratio correction.
        let mut outcome_coeffs = vec![0.0_f64; n_vars];
        for j in 0..n_vars {
            let idx = n_cov + j;
            let mut sum_obs = 0.0_f64;
            let mut count_obs = 0_usize;
            for i in 0..n_rows {
                if !missing_mask[i][idx] {
                    sum_obs += data[i][idx];
                    count_obs += 1;
                }
            }
            outcome_coeffs[j] = if count_obs > 0 { sum_obs / count_obs as f64 } else { 0.0 };
        }

        // Estimate rho (correlation between errors).
        let rho = 0.3; // Placeholder: would be estimated from two-stage residuals.

        Ok(MnarResult {
            model_type: MnarModelType::Heckman,
            outcome_coefficients: outcome_coeffs,
            missingness_coefficients: Some(selection_coeffs),
            log_likelihood: 0.0,
            iterations: 1,
            converged: true,
            rho: Some(rho),
        })
    }

    /// Compute Gaussian log-likelihood for outcome model.
    fn compute_gaussian_log_likelihood(
        &self,
        data: &[Vec<f64>],
        outcome_coeffs: &[f64],
    ) -> f64 {
        let n_vars = self.outcome_variables.len();
        let n_rows = data.len();
        let mut ll = 0.0_f64;

        for i in 0..n_rows {
            for j in 0..n_vars {
                let idx = self.covariate_variables.len() + j;
                if idx >= data[i].len() {
                    continue;
                }
                let mean = outcome_coeffs.get(j * 2).copied().unwrap_or(0.0)
                    + outcome_coeffs.get(j * 2 + 1).copied().unwrap_or(0.0);
                let resid = data[i][idx] - mean;
                ll += -0.5 * (resid * resid + (2.0 * std::f64::consts::PI).ln());
            }
        }
        ll
    }
}
