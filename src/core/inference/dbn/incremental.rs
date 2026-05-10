use std::collections::HashMap;
use std::time::Instant;

use crate::core::{
    error::LutufiResult,
    factor::TabularFactor,
    models::bayesian_network::BayesianNetwork,
    variable::VariableId,
};

use super::evidence_manager::EvidenceManager;

/// Options for incremental inference updates.
#[derive(Debug, Clone)]
pub struct IncrementalUpdateOptions {
    pub use_cache: bool,
    pub full_recompile_threshold: f64,
}

impl Default for IncrementalUpdateOptions {
    fn default() -> Self {
        IncrementalUpdateOptions { use_cache: true, full_recompile_threshold: 0.3 }
    }
}

/// Context for incremental inference on evolving networks.
pub struct IncrementalInferenceContext {
    model: BayesianNetwork,
    evidence_manager: EvidenceManager,
    options: IncrementalUpdateOptions,
    last_recompilation: Instant,
    updates_since_recompile: usize,
}

impl IncrementalInferenceContext {
    /// Create a new incremental inference context.
    pub fn new(model: BayesianNetwork) -> Self {
        let evidence_manager = EvidenceManager::new(model.clone());
        IncrementalInferenceContext {
            model,
            evidence_manager,
            options: IncrementalUpdateOptions::default(),
            last_recompilation: Instant::now(),
            updates_since_recompile: 0,
        }
    }

    /// Add a new variable node to the model.
    pub fn add_node(&mut self, name: &str, domain: crate::core::domain::Domain) -> LutufiResult<()> {
        self.model.add_variable(name, domain)?;
        self.updates_since_recompile += 1;
        Ok(())
    }

    /// Add a directed edge between two variables.
    pub fn add_edge(&mut self, from: &str, to: &str) -> LutufiResult<()> {
        self.model.add_edge(from, to)?;
        self.updates_since_recompile += 1;
        Ok(())
    }

    /// Remove a directed edge between two variables.
    pub fn remove_edge(&mut self, from: &str, to: &str) -> LutufiResult<()> {
        self.model.remove_edge(from, to)?;
        self.updates_since_recompile += 1;
        Ok(())
    }

    /// Set the CPD for a variable.
    pub fn set_cpd(&mut self, variable: &str, cpd: crate::core::factor::ConditionalProbabilityTable) -> LutufiResult<()> {
        self.model.set_cpd(variable, cpd)?;
        self.updates_since_recompile += 1;
        Ok(())
    }

    /// Set evidence for a variable.
    pub fn set_evidence(&mut self, variable: &str, value: &str) -> LutufiResult<()> {
        self.evidence_manager.set_evidence(variable, value)
    }

    /// Remove evidence for a variable.
    pub fn remove_evidence(&mut self, variable: &str) -> LutufiResult<()> {
        self.evidence_manager.remove_evidence(variable)
    }

    /// Run inference to compute marginals for the given query variables.
    pub fn infer(&mut self, query_variables: &[&str]) -> LutufiResult<HashMap<String, TabularFactor>> {
        if self.options.use_cache && self.should_recompile() {
            self.evidence_manager.compile()?;
            self.updates_since_recompile = 0;
            self.last_recompilation = Instant::now();
        }
        self.evidence_manager.infer(query_variables)
    }

    fn should_recompile(&self) -> bool {
        if !self.options.use_cache { return true; }
        self.updates_since_recompile > 10
    }

    /// Get a reference to the underlying model.
    pub fn model(&self) -> &BayesianNetwork { &self.model }
    /// Get a reference to the evidence manager.
    pub fn evidence_manager(&self) -> &EvidenceManager { &self.evidence_manager }
}

/// Compute ESS from log importance weights.
pub fn effective_sample_size(log_weights: &[f64]) -> f64 {
    if log_weights.is_empty() { return 0.0; }
    let max_log = log_weights.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let weights: Vec<f64> = log_weights.iter().map(|w| (w - max_log).exp()).collect();
    let sum_w: f64 = weights.iter().sum();
    let sum_w_sq: f64 = weights.iter().map(|w| w * w).sum();
    if sum_w_sq <= 0.0 { 0.0 } else { (sum_w * sum_w) / sum_w_sq }
}

/// Compute particle filter weights.
pub fn particle_filter_weights(
    particles: &[Vec<HashMap<VariableId, usize>>],
    observation: &HashMap<VariableId, usize>,
    log_weight: &[f64],
) -> Vec<f64> {
    let n = particles.len();
    let mut new_weights = vec![f64::NEG_INFINITY; n];
    for i in 0..n {
        let empty_map = HashMap::new();
        let last_state = particles[i].last().unwrap_or(&empty_map);
        let mut log_likelihood = 0.0;
        for (var_id, &obs_val) in observation {
            if let Some(&state_val) = last_state.get(var_id) {
                if state_val != obs_val {
                    log_likelihood += f64::NEG_INFINITY;
                }
            }
        }
        new_weights[i] = log_weight[i] + log_likelihood;
    }
    let max_log = new_weights.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let weights: Vec<f64> = new_weights.iter().map(|w| (w - max_log).exp()).collect();
    let sum: f64 = weights.iter().sum();
    if sum > 0.0 { weights.iter().map(|w| w / sum).collect() } else { vec![1.0 / n as f64; n] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_sample_size() {
        let weights = vec![0.0, -1.0, -2.0, -3.0];
        let ess = effective_sample_size(&weights);
        assert!(ess > 0.0);
        assert!(ess <= weights.len() as f64);
    }

    #[test]
    fn test_incremental_context() {
        let model = BayesianNetwork::new();
        let mut ctx = IncrementalInferenceContext::new(model);
        ctx.add_node("A", crate::core::domain::Domain::binary()).unwrap();
        ctx.add_node("B", crate::core::domain::Domain::binary()).unwrap();
        assert_eq!(ctx.model().nodes().len(), 2);
    }
}
