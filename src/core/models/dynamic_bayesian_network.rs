use serde::{Deserialize, Serialize};
use crate::core::{
    error::LutufiResult,
    models::bayesian_network::BayesianNetwork,
};

/// A Dynamic Bayesian Network (DBN) — a two-slice temporal model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicBayesianNetwork {
    prior: BayesianNetwork,
    transition: BayesianNetwork,
}

impl DynamicBayesianNetwork {
    /// Create a new, empty Dynamic Bayesian Network.
    pub fn new() -> Self {
        DynamicBayesianNetwork {
            prior: BayesianNetwork::new(),
            transition: BayesianNetwork::new(),
        }
    }

    /// Add a variable to the DBN.
    pub fn add_variable(&mut self, name: &str, domain: crate::core::domain::Domain) -> LutufiResult<()> {
        self.prior.add_variable(name, domain.clone())?;
        self.transition.add_variable(&format!("{}_t", name), domain.clone())?;
        self.transition.add_variable(&format!("{}_t1", name), domain)?;
        Ok(())
    }

    /// Add an edge within a single time slice.
    pub fn add_intraslice_edge(&mut self, from: &str, to: &str) -> LutufiResult<()> {
        self.prior.add_edge(from, to)?;
        self.transition.add_edge(&format!("{}_t", from), &format!("{}_t", to))?;
        self.transition.add_edge(&format!("{}_t1", from), &format!("{}_t1", to))?;
        Ok(())
    }

    /// Add an edge between consecutive time slices (t -> t+1).
    pub fn add_interslice_edge(&mut self, from: &str, to: &str) -> LutufiResult<()> {
        self.transition.add_edge(&format!("{}_t", from), &format!("{}_t1", to))
    }

    /// Access the prior network (P(X_0)).
    pub fn prior(&self) -> &BayesianNetwork {
        &self.prior
    }

    /// Access the transition network (P(X_t | X_{t-1})).
    pub fn transition(&self) -> &BayesianNetwork {
        &self.transition
    }

    /// Access the prior network mutably.
    pub fn prior_mut(&mut self) -> &mut BayesianNetwork {
        &mut self.prior
    }

    /// Access the transition network mutably.
    pub fn transition_mut(&mut self) -> &mut BayesianNetwork {
        &mut self.transition
    }

    /// Unroll the DBN for T time slices into a static Bayesian Network.
    pub fn unroll(&self, _t: usize) -> LutufiResult<BayesianNetwork> {
        let unrolled = BayesianNetwork::new();
        // Simplified implementation for Phase 1
        Ok(unrolled)
    }
}
