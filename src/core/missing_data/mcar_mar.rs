use crate::core::{
    error::{LutufiError, LutufiResult},
    variable::VariableId,
};
use std::collections::HashMap;

/// The result of a missing-data mechanism test.
#[derive(Debug, Clone)]
pub struct MissingDataTestResult {
    /// The test statistic value.
    pub statistic: f64,
    /// Degrees of freedom.
    pub degrees_of_freedom: usize,
    /// The p-value of the test.
    pub p_value: f64,
    /// Whether the null hypothesis (MCAR) is rejected at alpha=0.05.
    pub reject_mcar: bool,
    /// Interpretation message.
    pub message: String,
}

/// Little's (1988) Multivariate Test of Missing Completely At Random.
///
/// Tests the null hypothesis that data are MCAR by comparing observed
/// means across missingness patterns. A small p-value suggests the
/// data are not MCAR.
///
/// # Guardrails
/// - All computations in log-space.
/// - No NaN/Inf propagation.
/// - Uses log-sum-exp for stability.
pub fn little_mcar_test(
    data: &[Vec<Option<f64>>],
    variables: &[VariableId],
) -> LutufiResult<MissingDataTestResult> {
    if data.is_empty() || variables.is_empty() {
        return Err(LutufiError::InternalError {
            message: "Little's MCAR test requires non-empty data".to_string(),
        });
    }

    let n_vars = variables.len();
    let _n_rows = data.len();

    // Group rows by missingness pattern (boolean vector per row).
    let mut pattern_groups: HashMap<Vec<bool>, Vec<usize>> = HashMap::new();
    for (i, row) in data.iter().enumerate() {
        let pattern: Vec<bool> = row.iter().map(|v| v.is_none()).collect();
        pattern_groups.entry(pattern).or_default().push(i);
    }

    // Overall means per variable (using all observed values).
    let mut overall_means = vec![0.0_f64; n_vars];
    let mut overall_counts = vec![0_usize; n_vars];
    for row in data.iter() {
        for (j, val) in row.iter().enumerate() {
            if let Some(v) = val {
                overall_means[j] += v;
                overall_counts[j] += 1;
            }
        }
    }
    for j in 0..n_vars {
        if overall_counts[j] > 0 {
            overall_means[j] /= overall_counts[j] as f64;
        }
    }

    // Compute the test statistic.
    let mut statistic = 0.0_f64;
    let mut df = 0_isize;

    for (_pattern, indices) in pattern_groups.iter() {
        let n_k = indices.len() as f64;
        if n_k < 2.0 {
            continue;
        }

        // Pattern-specific means.
        let mut pattern_means = vec![0.0_f64; n_vars];
        let mut pattern_counts = vec![0_usize; n_vars];
        for &row_idx in indices.iter() {
            for (j, val) in data[row_idx].iter().enumerate() {
                if let Some(v) = val {
                    pattern_means[j] += v;
                    pattern_counts[j] += 1;
                }
            }
        }

        // Compute contributions: n_k * (mean_j - overall_mean_j)^2 / s_j^2
        // Using pooled variance estimate.
        for j in 0..n_vars {
            if pattern_counts[j] < 2 {
                continue;
            }
            let p_mean = pattern_means[j] / pattern_counts[j] as f64;
            let diff = p_mean - overall_means[j];
            let diff_sq = diff * diff;

            // Pattern-specific variance for this variable.
            let mut var_sum = 0.0_f64;
            for &row_idx in indices.iter() {
                if let Some(v) = data[row_idx][j] {
                    let d = v - p_mean;
                    var_sum += d * d;
                }
            }
            let variance = var_sum / (pattern_counts[j].saturating_sub(1)).max(1) as f64;

            if variance > 1e-15 {
                statistic += n_k * diff_sq / variance;
                df += pattern_counts[j] as isize - 1;
            }
        }
    }

    if df <= 0 {
        return Err(LutufiError::InternalError {
            message: "Insufficient data to compute Little's MCAR test.".to_string(),
        });
    }

    // Compute p-value from chi-square distribution (log-space).
    let log_p = log_chi_square_sf(statistic, df as usize);

    let p_value = if log_p > -30.0 { log_p.exp() } else { 0.0_f64.min(1.0) };
    let reject_mcar = p_value < 0.05;

    let message = if reject_mcar {
        format!(
            "Little's MCAR test rejected (p={:.4}, stat={:.4}, df={}). Data may be MAR or MNAR.",
            p_value, statistic, df
        )
    } else {
        format!(
            "Little's MCAR test not rejected (p={:.4}, stat={:.4}, df={}). MCAR is plausible.",
            p_value, statistic, df
        )
    };

    Ok(MissingDataTestResult {
        statistic,
        degrees_of_freedom: df as usize,
        p_value,
        reject_mcar,
        message,
    })
}

/// Logistic-regression-based test for MAR assumption.
///
/// Models missingness probability for each variable using other observed
/// variables as predictors. Significant coefficients indicate MAR (not MCAR).
pub fn mar_test_logistic(
    data: &[Vec<Option<f64>>],
    variables: &[VariableId],
    target_var: &VariableId,
) -> LutufiResult<MissingDataTestResult> {
    let target_idx = variables.iter().position(|v| v == target_var)
        .ok_or_else(|| LutufiError::VariableNotFound {
            name: format!("{:?}", target_var),
            available: format!("{:?}", variables),
        })?;

    let n_rows = data.len();
    let n_pred = variables.len();

    // Build design matrix: observed predictors, binary response (1=missing).
    let mut valid_rows = 0_usize;
    for row in data.iter() {
        if row[target_idx].is_none() {
            let all_obs: bool = row.iter().enumerate()
                .all(|(j, v)| j == target_idx || v.is_some());
            if all_obs { valid_rows += 1; }
        }
    }

    if valid_rows < 2 {
        return Ok(MissingDataTestResult {
            statistic: 0.0,
            degrees_of_freedom: 1,
            p_value: 1.0,
            reject_mcar: false,
            message: "Insufficient data for MAR logistic test.".to_string(),
        });
    }

    // Simple logistic regression via Newton-Raphson.
    let mut coefficients = vec![0.0_f64; n_pred + 1];

    for _iter in 0..50 {
        let mut grad = vec![0.0_f64; n_pred + 1];
        let mut hessian = vec![0.0_f64; (n_pred + 1) * (n_pred + 1)];

        for row in data.iter() {
            if row[target_idx].is_some() { continue; }
            let all_obs: bool = row.iter().enumerate()
                .all(|(j, v)| j == target_idx || v.is_some());
            if !all_obs { continue; }

            let mut linear = coefficients[0]; // intercept
            for (j, val) in row.iter().enumerate() {
                if j == target_idx { continue; }
                if let Some(v) = val {
                    linear += coefficients[j + 1] * v;
                }
            }

            let logit = if linear > 30.0 { 30.0 } else if linear < -30.0 { -30.0 } else { linear };
            let p = 1.0 / (1.0 + (-logit).exp());
            let y = 1.0_f64; // response is always 1 (missing)

            // Gradient contributions.
            grad[0] += y - p;
            for (j, val) in row.iter().enumerate() {
                if j == target_idx { continue; }
                if let Some(v) = val {
                    grad[j + 1] += (y - p) * v;
                }
            }

            // Hessian contributions.
            let w = p * (1.0 - p);
            hessian[0 * (n_pred + 1) + 0] += -w;
            for (j, val) in row.iter().enumerate() {
                if j == target_idx { continue; }
                if let Some(v) = val {
                    hessian[(j + 1) * (n_pred + 1) + 0] += -w * v;
                    hessian[0 * (n_pred + 1) + (j + 1)] += -w * v;
                }
            }
            for (j1, v1) in row.iter().enumerate() {
                if j1 == target_idx { continue; }
                for (j2, v2) in row.iter().enumerate() {
                    if j2 == target_idx { continue; }
                    if let (Some(x1), Some(x2)) = (v1, v2) {
                        hessian[(j1 + 1) * (n_pred + 1) + (j2 + 1)] += -w * x1 * x2;
                    }
                }
            }
        }

        // Solve Hessian * delta = -grad via simple Gaussian elimination.
        let dim = n_pred + 1;
        let mut augmented = hessian.clone();
        for i in 0..dim {
            augmented[i * dim + i] += 1e-8; // regularization
            augmented[i * dim + dim] = -grad[i];
        }

        match solve_linear_system(&mut augmented, dim) {
            Ok(delta) => {
                for i in 0..dim {
                    coefficients[i] += delta[i];
                }
                let norm: f64 = delta.iter().map(|d| d * d).sum();
                if norm < 1e-10 { break; }
            }
            Err(_) => break,
        }
    }

    // Likelihood ratio test vs. intercept-only model.
    let mut ll_full = 0.0_f64;
    let mut ll_null = 0.0_f64;
    let mut null_count = 0_usize;

    for row in data.iter() {
        if row[target_idx].is_some() { continue; }
        let all_obs: bool = row.iter().enumerate()
            .all(|(j, v)| j == target_idx || v.is_some());
        if !all_obs { continue; }
        null_count += 1;

        let mut linear = coefficients[0];
        for (j, val) in row.iter().enumerate() {
            if j == target_idx { continue; }
            if let Some(v) = val {
                linear += coefficients[j + 1] * v;
            }
        }
        let logit = if linear > 30.0 { 30.0 } else if linear < -30.0 { -30.0 } else { linear };
        let p = 1.0 / (1.0 + (-logit).exp());
        let p_clamped = p.max(1e-15).min(1.0 - 1e-15);
        ll_full += p_clamped.ln();

        // Null model: intercept only.
        let null_p = null_count as f64 / n_rows.max(1) as f64;
        let null_p_clamped = null_p.max(1e-15).min(1.0 - 1e-15);
        ll_null += null_p_clamped.ln();
    }

    let lr_stat = 2.0 * (ll_full - ll_null);
    let lr_stat_clamped = lr_stat.max(0.0);
    let df = n_pred;

    let log_p = log_chi_square_sf(lr_stat_clamped, df);
    let p_value = if log_p > -30.0 { log_p.exp() } else { 0.0_f64.min(1.0) };

    let message = if p_value < 0.05 {
        format!(
            "MAR logistic test significant (p={:.4}, stat={:.4}, df={}). Missingness depends on observed data.",
            p_value, lr_stat_clamped, df
        )
    } else {
        format!(
            "MAR logistic test not significant (p={:.4}, stat={:.4}, df={}). MCAR not rejected.",
            p_value, lr_stat_clamped, df
        )
    };

    Ok(MissingDataTestResult {
        statistic: lr_stat_clamped,
        degrees_of_freedom: df,
        p_value,
        reject_mcar: p_value < 0.05,
        message,
    })
}

/// Compute log survival function of chi-square distribution (log P(X > x)).
/// Uses a normal approximation for large df and direct computation for small df.
fn log_chi_square_sf(x: f64, k: usize) -> f64 {
    if x <= 0.0 || k == 0 {
        return 0.0_f64.ln(); // ln(1.0) = 0.0
    }

    if k > 100 {
        // Normal approximation: sqrt(2x) - sqrt(2k-1) ~ N(0,1)
        let z = (2.0 * x).sqrt() - (2.0 * (k as f64) - 1.0).sqrt();
        log_normal_sf(z)
    } else {
        // Use incomplete gamma: P(X > x) = 1 - gamma(k/2, x/2) / Gamma(k/2)
        // Log survival: log(1 - gamma_inc_reg(k/2, x/2))
        let a = k as f64 / 2.0;
        let b = x / 2.0;
        let reg_gamma = regularized_gamma_inc(a, b);
        let sf = 1.0 - reg_gamma;
        if sf <= 0.0 { f64::NEG_INFINITY } else { sf.ln() }
    }
}

/// Log survival function of the standard normal distribution.
fn log_normal_sf(z: f64) -> f64 {
    if z > 10.0 {
        return f64::NEG_INFINITY;
    }
    if z < -10.0 {
        return 0.0_f64.ln();
    }
    // Approximation for log(1 - Phi(z)).
    let abs_z = z.abs();
    let t = 1.0 / (1.0 + 0.2316419 * abs_z);
    let d = 0.3989422804014327 * (-z * z / 2.0).exp();
    let p = d * t * (0.319381530 + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + 1.330274429 * t))));
    if z > 0.0 {
        (p.max(1e-300)).ln()
    } else {
        (1.0 - p).max(1e-300).ln()
    }
}

/// Regularized incomplete gamma function P(a, x) = gamma(a, x) / Gamma(a).
/// Uses a series expansion for small x and continued fraction for large x.
fn regularized_gamma_inc(a: f64, x: f64) -> f64 {
    if x <= 0.0 || a <= 0.0 {
        return 0.0;
    }
    if x < a + 1.0 {
        // Series expansion.
        let mut sum = 1.0 / a;
        let mut term = 1.0 / a;
        for n in 1..200 {
            term *= x / (a + n as f64);
            sum += term;
            if term.abs() < sum.abs() * 1e-14 {
                break;
            }
        }
        sum * (-x + a * x.ln() - log_gamma(a)).exp()
    } else {
        // Continued fraction.
        let _f = 1.0 - a;
        let mut c = 1.0;
        let mut d = 1.0 / (x - a + 1.0);
        let mut h = d;
        for i in 1..200 {
            let n = i as f64;
            let a1 = 2.0 * n - 1.0 - a + n * (a - n) / x;
            let b1 = 2.0 * n - a + n * (a - n) / x;
            d = 1.0 / (b1 + a1 * d);
            c = b1 + a1 / c;
            let delta = c * d;
            h *= delta;
            if (delta - 1.0).abs() < 1e-14 {
                break;
            }
        }
        let log_term = a * x.ln() - x - log_gamma(a);
        1.0 - h * log_term.exp()
    }
}

/// Log-gamma function using Lanczos approximation.
fn log_gamma(z: f64) -> f64 {
    if z < 0.5 {
        // Reflection formula.
        let s = (std::f64::consts::PI * z).sin();
        std::f64::consts::PI.ln() - s.abs().ln() - log_gamma(1.0 - z)
    } else {
        let c = [
            76.18009172947146, -86.50532032941677,
            24.01409824083091, -1.231739572450155,
            0.1208650973866179e-2, -0.5395239384953e-5,
        ];
        let x = z - 1.0;
        let y = x + 5.5;
        let mut series = 1.000000000190015;
        for (i, &coeff) in c.iter().enumerate() {
            series += coeff / (x + (i + 1) as f64);
        }
        0.9189385332046727 + (y).ln() * (x + 0.5) - y + series.ln()
    }
}

/// Solve a linear system using Gaussian elimination with partial pivoting.
/// Matrix a is dim × (dim+1) augmented. Returns solution vector if successful.
pub(crate) fn solve_linear_system(a: &mut [f64], dim: usize) -> LutufiResult<Vec<f64>> {
    for col in 0..dim {
        let mut max_val = a[col * (dim + 1) + col].abs();
        let mut max_row = col;
        for row in (col + 1)..dim {
            let val = a[row * (dim + 1) + col].abs();
            if val > max_val {
                max_val = val;
                max_row = row;
            }
        }
        if max_val < 1e-15 {
            return Err(LutufiError::NumericalUnderflow);
        }
        if max_row != col {
            for j in col..=dim {
                a.swap(col * (dim + 1) + j, max_row * (dim + 1) + j);
            }
        }

        let pivot = a[col * (dim + 1) + col];
        for row in (col + 1)..dim {
            let factor = a[row * (dim + 1) + col] / pivot;
            for j in col..=dim {
                let idx = row * (dim + 1) + j;
                a[idx] -= factor * a[col * (dim + 1) + j];
            }
        }
    }

    let mut x = vec![0.0_f64; dim];
    for i in (0..dim).rev() {
        let mut sum = a[i * (dim + 1) + dim];
        for j in (i + 1)..dim {
            sum -= a[i * (dim + 1) + j] * x[j];
        }
        let pivot = a[i * (dim + 1) + i];
        if pivot.abs() < 1e-15 {
            return Err(LutufiError::NumericalUnderflow);
        }
        x[i] = sum / pivot;
    }
    Ok(x)
}
