use serde::{Deserialize, Serialize};

use crate::core::models::bayesian_network::BayesianNetwork;

/// A transition model that can change over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionModel {
    /// Fixed transition (standard DBN behavior).
    Fixed(BayesianNetwork),
    /// Time-varying transition, one model per time step.
    TimeVarying(Vec<BayesianNetwork>),
    /// Regime-switching: a set of transitions selected by a regime variable.
    RegimeSwitching {
        /// One Bayesian network per regime representing transition dynamics.
        regimes: Vec<BayesianNetwork>,
        /// Regime transition probability matrix (from regime i to regime j).
        regime_transition: Vec<Vec<f64>>,
        /// Initial probability distribution over regimes.
        initial_regime: Vec<f64>,
    },
}

impl TransitionModel {
    /// Number of transition networks available.
    pub fn num_models(&self) -> usize {
        match self {
            TransitionModel::Fixed(_) => 1,
            TransitionModel::TimeVarying(transitions) => transitions.len(),
            TransitionModel::RegimeSwitching { regimes, .. } => regimes.len(),
        }
    }

    /// Get the transition network at a specific time step or index.
    pub fn get(&self, index: usize) -> Option<&BayesianNetwork> {
        match self {
            TransitionModel::Fixed(bn) => Some(bn),
            TransitionModel::TimeVarying(transitions) => transitions.get(index),
            TransitionModel::RegimeSwitching { regimes, .. } => {
                if regimes.is_empty() { None } else { Some(&regimes[index % regimes.len()]) }
            }
        }
    }

    /// Get the transition network at a specific time step or index (mutable).
    pub fn get_mut(&mut self, index: usize) -> Option<&mut BayesianNetwork> {
        match self {
            TransitionModel::Fixed(bn) => Some(bn),
            TransitionModel::TimeVarying(transitions) => transitions.get_mut(index),
            TransitionModel::RegimeSwitching { regimes, .. } => {
                if regimes.is_empty() {
                    None
                } else {
                    let idx = index % regimes.len();
                    Some(&mut regimes[idx])
                }
            }
        }
    }

    /// Add a variable to all transition networks.
    pub fn add_variable(&mut self, name: &str, domain: crate::core::domain::Domain) -> crate::core::error::LutufiResult<()> {
        match self {
            TransitionModel::Fixed(bn) => { bn.add_variable(name, domain)?; }
            TransitionModel::TimeVarying(transitions) => {
                for bn in transitions.iter_mut() { bn.add_variable(name, domain.clone())?; }
            }
            TransitionModel::RegimeSwitching { regimes, .. } => {
                for bn in regimes.iter_mut() { bn.add_variable(name, domain.clone())?; }
            }
        }
        Ok(())
    }

    /// Validate all transition networks.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        match self {
            TransitionModel::Fixed(bn) => {
                errors.extend(bn.validate().into_iter().map(|e| format!("Fixed transition: {}", e)));
            }
            TransitionModel::TimeVarying(transitions) => {
                for (i, bn) in transitions.iter().enumerate() {
                    errors.extend(bn.validate().into_iter().map(|e| format!("Transition[{}]: {}", i, e)));
                }
            }
            TransitionModel::RegimeSwitching { regimes, .. } => {
                for (i, bn) in regimes.iter().enumerate() {
                    errors.extend(bn.validate().into_iter().map(|e| format!("Regime[{}]: {}", i, e)));
                }
            }
        }
        errors
    }
}
