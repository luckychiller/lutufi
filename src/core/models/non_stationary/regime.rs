use std::collections::HashMap;

use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::log_sum_exp,
};

use super::NonStationaryDBN;
use super::transition_model::TransitionModel;

/// Compute cumulative regime posterior probabilities from observations.
pub fn regime_posterior(dbn: &NonStationaryDBN, observations: &[HashMap<String, String>]) -> LutufiResult<Vec<Vec<f64>>> {
    match dbn.transition() {
        TransitionModel::RegimeSwitching { regime_transition, initial_regime, .. } => {
            let n_regimes = regime_transition.len();
            let t = observations.len();
            if t == 0 {
                return Err(LutufiError::InternalError { message: "Observations must not be empty".to_string() });
            }

            let mut alpha = vec![vec![f64::NEG_INFINITY; n_regimes]; t];
            for r in 0..n_regimes { alpha[0][r] = initial_regime[r]; }

            for step in 1..t {
                for r in 0..n_regimes {
                    let mut sum = f64::NEG_INFINITY;
                    for r_prev in 0..n_regimes {
                        sum = log_sum_exp(sum, alpha[step - 1][r_prev] + regime_transition[r_prev][r]);
                    }
                    alpha[step][r] = sum;
                }
            }

            let mut posteriors = Vec::new();
            for step in 0..t {
                let mut row = vec![0.0; n_regimes];
                let mut norm = f64::NEG_INFINITY;
                for r in 0..n_regimes { norm = log_sum_exp(norm, alpha[step][r]); }
                if norm > f64::NEG_INFINITY {
                    for r in 0..n_regimes { row[r] = (alpha[step][r] - norm).exp(); }
                }
                posteriors.push(row);
            }

            Ok(posteriors)
        }
        _ => Err(LutufiError::InternalError { message: "Not a regime-switching DBN".to_string() }),
    }
}

/// Compute regime forecast probabilities k steps ahead.
pub fn regime_forecast(dbn: &NonStationaryDBN, current_posterior: &[f64], horizon: usize) -> Option<Vec<Vec<f64>>> {
    match dbn.transition() {
        TransitionModel::RegimeSwitching { regime_transition, .. } => {
            let n = regime_transition.len();
            if current_posterior.len() != n { return None; }

            let mut forecasts = Vec::with_capacity(horizon);
            let mut p = current_posterior.to_vec();

            for _ in 0..horizon {
                let mut next = vec![0.0; n];
                for i in 0..n {
                    for j in 0..n {
                        next[j] += p[i] * regime_transition[i][j].exp();
                    }
                }
                p = next;
                forecasts.push(p.clone());
            }

            Some(forecasts)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::bayesian_network::BayesianNetwork;
    use crate::core::models::non_stationary::NonStationaryDBN;

    #[test]
    fn test_regime_posterior_non_regime() {
        let dbn = NonStationaryDBN::new(BayesianNetwork::new(), BayesianNetwork::new());
        let obs = vec![HashMap::new()];
        assert!(regime_posterior(&dbn, &obs).is_err());
    }

    #[test]
    fn test_regime_forecast() {
        let ns_dbn = NonStationaryDBN::new_regime_switching(
            BayesianNetwork::new(),
            vec![BayesianNetwork::new(), BayesianNetwork::new()],
            vec![vec![0.9, 0.1], vec![0.2, 0.8]],
            vec![0.5, 0.5],
        ).unwrap();
        let forecasts = regime_forecast(&ns_dbn, &[1.0, 0.0], 3);
        assert!(forecasts.is_some());
        assert_eq!(forecasts.unwrap().len(), 3);
    }
}
