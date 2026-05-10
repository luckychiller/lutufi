use serde::{Deserialize, Serialize};

use crate::core::{
    domain::Domain,
    error::{LutufiError, LutufiResult},
    models::{
        bayesian_network::BayesianNetwork,
        dynamic_bayesian_network::DynamicBayesianNetwork,
    },
};

use super::transition_model::TransitionModel;

/// A Dynamic Bayesian Network with non-stationary dynamics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonStationaryDBN {
    prior: BayesianNetwork,
    transition: TransitionModel,
}

impl NonStationaryDBN {
    /// Create a NonStationaryDBN with a fixed transition model.
    pub fn new(prior: BayesianNetwork, transition: BayesianNetwork) -> Self {
        NonStationaryDBN { prior, transition: TransitionModel::Fixed(transition) }
    }

    /// Create a NonStationaryDBN with time-varying transitions.
    pub fn new_time_varying(prior: BayesianNetwork, transitions: Vec<BayesianNetwork>) -> Self {
        NonStationaryDBN { prior, transition: TransitionModel::TimeVarying(transitions) }
    }

    /// Create a NonStationaryDBN with regime-switching transitions.
    pub fn new_regime_switching(
        prior: BayesianNetwork,
        regimes: Vec<BayesianNetwork>,
        regime_transition: Vec<Vec<f64>>,
        initial_regime: Vec<f64>,
    ) -> LutufiResult<Self> {
        let n = regimes.len();
        if n == 0 {
            return Err(LutufiError::InternalError { message: "At least one regime is required".to_string() });
        }
        if regime_transition.len() != n || regime_transition.iter().any(|r| r.len() != n) {
            return Err(LutufiError::InternalError { message: format!("Regime transition matrix must be {}x{}", n, n) });
        }
        if initial_regime.len() != n {
            return Err(LutufiError::InternalError { message: format!("Initial regime must have length {}", n) });
        }

        let log_trans = regime_transition.iter()
            .map(|r| r.iter().map(|&v| if v > 0.0 { v.ln() } else { f64::NEG_INFINITY }).collect())
            .collect();
        let log_init = initial_regime.iter().map(|&v| if v > 0.0 { v.ln() } else { f64::NEG_INFINITY }).collect();

        Ok(NonStationaryDBN {
            prior,
            transition: TransitionModel::RegimeSwitching {
                regimes,
                regime_transition: log_trans,
                initial_regime: log_init,
            },
        })
    }

    /// Get the prior (initial time-slice) network.
    pub fn prior(&self) -> &BayesianNetwork { &self.prior }
    /// Get a mutable reference to the prior network.
    pub fn prior_mut(&mut self) -> &mut BayesianNetwork { &mut self.prior }
    /// Get the transition model.
    pub fn transition(&self) -> &TransitionModel { &self.transition }
    /// Get a mutable reference to the transition model.
    pub fn transition_mut(&mut self) -> &mut TransitionModel { &mut self.transition }

    /// Get transition network at a time step.
    pub fn transition_at(&self, time_step: usize) -> Option<&BayesianNetwork> {
        self.transition.get(time_step)
    }

    /// Add a variable to both prior and transition.
    pub fn add_variable(&mut self, name: &str, domain: Domain) -> LutufiResult<()> {
        self.prior.add_variable(name, domain.clone())?;
        self.transition.add_variable(name, domain)?;
        Ok(())
    }

    /// Number of regimes (1 for non-regime-switching).
    pub fn num_regimes(&self) -> usize { self.transition.num_models() }

    /// Validate the model.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        errors.extend(self.prior.validate().into_iter().map(|e| format!("Prior: {}", e)));
        errors.extend(self.transition.validate());
        errors
    }

    /// Check whether the model is valid (no validation errors).
    pub fn is_valid(&self) -> bool { self.validate().is_empty() }

    /// Get all variable names from the prior network.
    pub fn nodes(&self) -> Vec<&str> { self.prior.nodes() }

    /// Compute stationary regime distribution for ergodic models.
    pub fn stationary_regime_distribution(&self) -> Option<Vec<f64>> {
        match &self.transition {
            TransitionModel::RegimeSwitching { regime_transition, .. } => {
                let n = regime_transition.len();
                if n == 0 { return None; }
                let mut pi = vec![1.0 / n as f64; n];
                for _ in 0..1000 {
                    let mut new_pi = vec![0.0; n];
                    for i in 0..n {
                        for j in 0..n {
                            new_pi[i] += pi[j] * regime_transition[j][i].exp();
                        }
                    }
                    let diff: f64 = pi.iter().zip(new_pi.iter()).map(|(a, b)| (a - b).abs()).sum();
                    pi = new_pi;
                    if diff < 1e-12 { break; }
                }
                let sum: f64 = pi.iter().sum();
                if sum > 0.0 { Some(pi.iter().map(|&v| v / sum).collect()) } else { None }
            }
            _ => None,
        }
    }

    /// Convert to a standard DynamicBayesianNetwork.
    pub fn to_standard_dbn(&self) -> DynamicBayesianNetwork {
        let mut dbn = DynamicBayesianNetwork::new();
        for name in self.prior.nodes() {
            if let Ok(var) = self.prior.variable(name) {
                let _ = dbn.add_variable(name, var.domain().clone());
            }
        }
        if let TransitionModel::Fixed(bn) = &self.transition {
            for (from, to) in bn.edges() {
                let _ = dbn.add_intraslice_edge(from, to);
            }
        }
        dbn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_stationary_dbn_creation() {
        let prior = BayesianNetwork::new();
        let transition = BayesianNetwork::new();
        let ns_dbn = NonStationaryDBN::new(prior, transition);
        assert!(ns_dbn.prior().nodes().is_empty());
    }

    #[test]
    fn test_regime_switching_creation() {
        let prior = BayesianNetwork::new();
        let ns_dbn = NonStationaryDBN::new_regime_switching(
            prior,
            vec![BayesianNetwork::new(), BayesianNetwork::new()],
            vec![vec![0.9, 0.1], vec![0.2, 0.8]],
            vec![0.5, 0.5],
        ).unwrap();
        assert_eq!(ns_dbn.num_regimes(), 2);
    }

    #[test]
    fn test_stationary_distribution() {
        let ns_dbn = NonStationaryDBN::new_regime_switching(
            BayesianNetwork::new(),
            vec![BayesianNetwork::new(), BayesianNetwork::new()],
            vec![vec![0.9, 0.1], vec![0.2, 0.8]],
            vec![0.5, 0.5],
        ).unwrap();
        assert!(ns_dbn.stationary_regime_distribution().is_some());
    }

    #[test]
    fn test_validate_empty() {
        let ns_dbn = NonStationaryDBN::new(BayesianNetwork::new(), BayesianNetwork::new());
        assert!(ns_dbn.validate().is_empty());
    }
}
