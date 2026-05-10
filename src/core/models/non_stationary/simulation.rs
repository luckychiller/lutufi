use rand::Rng;

use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    models::sampler::Sampler,
};

use super::NonStationaryDBN;

/// Simulate T steps forward from the prior.
pub fn simulate(dbn: &NonStationaryDBN, t: usize, n_samples: usize) -> LutufiResult<Vec<Vec<Assignment>>> {
    if t == 0 || n_samples == 0 {
        return Err(LutufiError::InternalError { message: "T and n_samples must both be positive".to_string() });
    }

    let mut all_trajectories = Vec::with_capacity(n_samples);
    let mut rng = rand::thread_rng();

    for _ in 0..n_samples {
        let mut trajectory = Vec::with_capacity(t);
        trajectory.push(Sampler::forward_sample(dbn.prior())?);

        for step in 1..t {
            if let Some(bn) = dbn.transition_at(step - 1) {
                trajectory.push(Sampler::forward_sample(bn)?);
            } else {
                let last = trajectory.last().ok_or_else(|| LutufiError::InternalError {
                    message: "Simulation: empty trajectory".to_string(),
                })?.clone();
                trajectory.push(last);
            }
        }

        // Handle regime-switching for regime-aware simulation
        if let crate::core::models::non_stationary::transition_model::TransitionModel::RegimeSwitching {
            regimes, regime_transition, initial_regime
        } = dbn.transition()
        {
            let n_regimes = regimes.len();
            if n_regimes > 0 {
                let regime_probs: Vec<f64> = initial_regime.iter()
                    .map(|&v| if v > f64::NEG_INFINITY { v.exp() } else { 0.0 }).collect();
                let mut regime = if regime_probs.is_empty() { 0 } else {
                    let cumsum: Vec<f64> = regime_probs.iter().scan(0.0, |s, &x| { *s += x; Some(*s) }).collect();
                    let u: f64 = rng.gen();
                    cumsum.iter().position(|&c| u <= c).unwrap_or(0)
                };

                let mut new_trajectory = Vec::with_capacity(t);
                new_trajectory.push(trajectory[0].clone());

                for _step in 1..t {
                    if let Some(bn) = regimes.get(regime) {
                        new_trajectory.push(Sampler::forward_sample(bn)?);
                    }
                    let trans_probs: Vec<f64> = regime_transition.get(regime)
                        .map(|row| row.iter().map(|&v| if v > f64::NEG_INFINITY { v.exp() } else { 0.0 }).collect())
                        .unwrap_or_default();
                    if !trans_probs.is_empty() {
                        let cumsum: Vec<f64> = trans_probs.iter().scan(0.0, |s, &x| { *s += x; Some(*s) }).collect();
                        let u: f64 = rng.gen();
                        regime = cumsum.iter().position(|&c| u <= c).unwrap_or(0);
                    }
                }
                all_trajectories.push(new_trajectory);
                continue;
            }
        }

        all_trajectories.push(trajectory);
    }

    Ok(all_trajectories)
}

/// Compute the log-likelihood of a trajectory under a DBN.
pub fn trajectory_log_likelihood(dbn: &NonStationaryDBN, trajectory: &[Assignment]) -> LutufiResult<f64> {
    if trajectory.is_empty() {
        return Err(LutufiError::InternalError { message: "Trajectory must not be empty".to_string() });
    }

    let mut log_likelihood = 0.0;
    let prior = dbn.prior();
    let ordered_names = prior.topological_order()?;
    let first_state = &trajectory[0];

    for &name in &ordered_names {
        if let Ok(var) = prior.variable(name) {
            if let Some(value) = first_state.get(&var.id()) {
                if let Some(state_idx) = var.domain().index_of(value) {
                    if let Ok(cpd) = prior.cpd(name) {
                        if let Ok(reduced) = cpd.as_factor().reduce(first_state) {
                            let scope = reduced.scope();
                            if !scope.is_empty() && state_idx < scope.num_entries() {
                                log_likelihood += reduced.log_value_at(state_idx);
                            }
                        }
                    }
                }
            }
        }
    }

    for t in 1..trajectory.len() {
        if let Some(bn) = dbn.transition_at(t - 1) {
            let t1_names = bn.topological_order()?;
            for name in &t1_names {
                if let Ok(var) = bn.variable(&format!("{}_t1", name)) {
                    let cur_state = &trajectory[t];
                    let prev_state = &trajectory[t - 1];
                    if let Some(value) = cur_state.get(&var.id()) {
                        if let Some(state_idx) = var.domain().index_of(value) {
                            let mut combined = Assignment::new();
                            combined.merge(prev_state);
                            if let Ok(cpd) = bn.cpd(&format!("{}_t1", name)) {
                                if let Ok(reduced) = cpd.as_factor().reduce(&combined) {
                                    let scope = reduced.scope();
                                    if !scope.is_empty() && state_idx < scope.num_entries() {
                                        log_likelihood += reduced.log_value_at(state_idx);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(log_likelihood)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::bayesian_network::BayesianNetwork;

    #[test]
    fn test_simulate_errors() {
        let dbn = NonStationaryDBN::new(BayesianNetwork::new(), BayesianNetwork::new());
        assert!(simulate(&dbn, 0, 10).is_err());
        assert!(simulate(&dbn, 10, 0).is_err());
    }
}
