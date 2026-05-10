use crate::core::{
    error::LutufiResult,
    variable::VariableId,
};
use std::collections::HashMap;

/// A parameter for sensitivity analysis.
#[derive(Debug, Clone)]
pub struct SensitivityParameter {
    /// The variable being perturbed.
    pub variable: VariableId,
    /// The type of perturbation.
    pub parameter_type: SensitivityParameterType,
    /// Range of values to explore.
    pub values: Vec<f64>,
}

/// Type of sensitivity parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensitivityParameterType {
    /// Delta adjustment: shift imputed values by an amount.
    Delta,
    /// Lambda: weight given to MAR-based imputation vs. MNAR adjustment.
    Lambda,
    /// Sigma multiplier: scale the imputation variance.
    Sigma,
    /// Correlation between outcome and missingness (for selection models).
    Correlation,
}

/// Result of a sensitivity analysis.
#[derive(Debug, Clone)]
pub struct SensitivityResult {
    /// Parameter values explored.
    pub parameter_values: Vec<f64>,
    /// Outcome metric at each parameter value (e.g., mean estimate).
    pub outcomes: Vec<f64>,
    /// Upper confidence bound at each parameter value.
    pub upper_bound: Vec<f64>,
    /// Lower confidence bound at each parameter value.
    pub lower_bound: Vec<f64>,
    /// The parameter type analyzed.
    pub parameter_type: SensitivityParameterType,
    /// The variable analyzed.
    pub variable: VariableId,
    /// Summary of the sensitivity analysis.
    pub summary: String,
}

/// Engine for sensitivity analysis of missing data assumptions.
#[derive(Debug, Clone)]
pub struct SensitivityAnalysis {
    /// Original (imputed) data.
    imputed_data: Vec<Vec<f64>>,
    /// Missing mask.
    missing_mask: Vec<Vec<bool>>,
    /// Variable IDs.
    variables: Vec<VariableId>,
    /// Baseline estimates (e.g., means after MAR imputation).
    baseline_estimates: HashMap<VariableId, f64>,
}

impl SensitivityAnalysis {
    /// Create a new sensitivity analysis.
    pub fn new(
        imputed_data: Vec<Vec<f64>>,
        missing_mask: Vec<Vec<bool>>,
        variables: Vec<VariableId>,
    ) -> Self {
        let baseline_estimates = Self::compute_means(&imputed_data, &variables);
        SensitivityAnalysis {
            imputed_data,
            missing_mask,
            variables,
            baseline_estimates,
        }
    }

    /// Compute per-variable means.
    fn compute_means(data: &[Vec<f64>], variables: &[VariableId]) -> HashMap<VariableId, f64> {
        let n_vars = variables.len();
        let n_rows = data.len();
        let mut means = HashMap::new();
        for j in 0..n_vars {
            let sum: f64 = data.iter().map(|row| row[j]).sum();
            means.insert(variables[j], sum / n_rows.max(1) as f64);
        }
        means
    }

    /// Run a sensitivity analysis over a parameter grid.
    ///
    /// For each parameter value, re-estimates the mean of the target variable
    /// by applying the specified perturbation to imputed values.
    pub fn analyze(
        &self,
        target_variable: &VariableId,
        parameter: &SensitivityParameter,
    ) -> LutufiResult<SensitivityResult> {
        let var_idx = self.variables.iter().position(|v| v == target_variable)
            .ok_or_else(|| crate::core::error::LutufiError::VariableNotFound {
                name: format!("{:?}", target_variable),
                available: format!("{:?}", self.variables),
            })?;

        let mut outcomes = Vec::new();
        let mut upper_bounds = Vec::new();
        let mut lower_bounds = Vec::new();

        let baseline = self.baseline_estimates.get(target_variable).copied().unwrap_or(0.0);

        for &param_val in &parameter.values {
            let mut adjusted_values = Vec::new();
            let mut _count = 0_usize;

            for i in 0..self.imputed_data.len() {
                if self.missing_mask[i][var_idx] {
                    let orig = self.imputed_data[i][var_idx];
                    let adjusted = match parameter.parameter_type {
                        SensitivityParameterType::Delta => {
                            orig + param_val
                        }
                        SensitivityParameterType::Lambda => {
                            // Lambda: 0 = MAR (baseline), 1 = full MNAR adjustment.
                            let mar_estimate = orig;
                            let mnar_adjustment = param_val * orig;
                            mar_estimate * (1.0 - param_val.abs().min(1.0))
                                + mnar_adjustment * param_val.abs().min(1.0)
                        }
                        SensitivityParameterType::Sigma => {
                            // Scale variance: multiply deviation from mean.
                            let dev = orig - baseline;
                            baseline + dev * param_val.max(0.0)
                        }
                        SensitivityParameterType::Correlation => {
                            // Adjust based on correlation with missingness propensity.
                            let delta = param_val * (orig - baseline);
                            orig + delta
                        }
                    };
                    adjusted_values.push(adjusted);
                    _count += 1;
                }
            }

            let new_mean = if !adjusted_values.is_empty() {
                adjusted_values.iter().sum::<f64>() / adjusted_values.len() as f64
            } else {
                baseline
            };

            // Bootstrap-style confidence (simplified: use 1.96 * SEM).
            let variance: f64 = if adjusted_values.len() > 1 {
                let mean = new_mean;
                adjusted_values.iter().map(|v| (v - mean).powi(2)).sum::<f64>()
                    / (adjusted_values.len() - 1) as f64
            } else {
                0.0
            };
            let se = (variance / adjusted_values.len().max(1) as f64).sqrt();
            let ci = 1.96 * se;

            outcomes.push(new_mean);
            upper_bounds.push(new_mean + ci);
            lower_bounds.push(new_mean - ci);
        }

        let summary = self.generate_summary(
            target_variable,
            parameter,
            &outcomes,
            baseline,
        );

        Ok(SensitivityResult {
            parameter_values: parameter.values.clone(),
            outcomes,
            upper_bound: upper_bounds,
            lower_bound: lower_bounds,
            parameter_type: parameter.parameter_type,
            variable: *target_variable,
            summary,
        })
    }

    /// Generate a textual summary of the sensitivity analysis.
    fn generate_summary(
        &self,
        target: &VariableId,
        param: &SensitivityParameter,
        outcomes: &[f64],
        baseline: f64,
    ) -> String {
        if outcomes.is_empty() {
            return "No sensitivity outcomes computed.".to_string();
        }
        let min_out = outcomes.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_out = outcomes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max_out - min_out;

        format!(
            "Sensitivity analysis for variable {:?}: parameter {:?} range [{:.4}, {:.4}], \
             baseline={:.4}, outcome range [{:.4}, {:.4}] (span={:.4})",
            target, param.parameter_type,
            param.values.first().copied().unwrap_or(0.0),
            param.values.last().copied().unwrap_or(0.0),
            baseline, min_out, max_out, range,
        )
    }
}
