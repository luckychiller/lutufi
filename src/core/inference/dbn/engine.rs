use std::collections::HashMap;
use std::time::Instant;

use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    models::dynamic_bayesian_network::DynamicBayesianNetwork,
    variable::VariableId,
};

use super::types::{DBNInferenceOptions, DBNInferenceResult};

/// Interface for all DBN inference engines.
pub trait DBNInference: Send + Sync {
    /// Run the forward algorithm (filtering): P(X_t | E_{1:t}).
    fn filter(&self, evidence_sequence: &[HashMap<String, String>]) -> LutufiResult<DBNInferenceResult>;
    /// Run smoothing: P(X_t | E_{1:T}) using forward-backward.
    fn smooth(&self, evidence_sequence: &[HashMap<String, String>]) -> LutufiResult<DBNInferenceResult>;
    /// Predict k steps ahead: P(X_{t+k} | E_{1:t}).
    fn predict(&self, evidence_sequence: &[HashMap<String, String>], horizon: usize) -> LutufiResult<DBNInferenceResult>;
}

/// Primary DBN inference engine implementing filtering, smoothing, and prediction.
pub struct DBNInferenceEngine {
    dbn: DynamicBayesianNetwork,
    options: DBNInferenceOptions,
}

impl DBNInferenceEngine {
    /// Create a new DBN inference engine with default options.
    pub fn new(dbn: DynamicBayesianNetwork) -> Self {
        DBNInferenceEngine { dbn, options: DBNInferenceOptions::default() }
    }

    /// Create a new DBN inference engine with custom options.
    pub fn new_with_options(dbn: DynamicBayesianNetwork, options: DBNInferenceOptions) -> Self {
        DBNInferenceEngine { dbn, options }
    }

    /// Get a reference to the underlying DBN.
    pub fn dbn(&self) -> &DynamicBayesianNetwork { &self.dbn }

    fn validate_evidence_step(&self, evidence: &HashMap<String, String>) -> LutufiResult<()> {
        for (name, _) in evidence {
            if self.dbn.prior().id_of(name).is_err() {
                return Err(LutufiError::VariableNotFound {
                    name: name.clone(),
                    available: format!("{:?}", self.dbn.prior().nodes()),
                });
            }
        }
        Ok(())
    }

    fn apply_transition(&self, current_beliefs: &HashMap<VariableId, TabularFactor>) -> LutufiResult<HashMap<VariableId, TabularFactor>> {
        let mut next_beliefs = HashMap::new();
        let var_ids: Vec<VariableId> = self.dbn.prior().variables().keys().copied().collect();

        for &var_id in &var_ids {
            let var = self.dbn.prior().variables().get(&var_id).ok_or_else(|| LutufiError::InternalError {
                message: "Variable not found".to_string(),
            })?;
            let domain_size = var.domain().size().unwrap_or(2);

            if let Ok(cpd) = self.dbn.transition().cpd(&format!("{}_t1", var.name())) {
                let transition_factor = cpd.as_factor();
                let parent_ids = cpd.parent_ids().to_vec();

                let mut evidence = Assignment::new();
                for &pid in &parent_ids {
                    if let Some(belief) = current_beliefs.get(&pid) {
                        let scope = belief.scope();
                        if scope.len() == 1 {
                            for i in 0..scope.num_entries() {
                                if belief.value_at(i) > 0.5 {
                                    evidence.set_discrete(pid, i).ok();
                                    break;
                                }
                            }
                        }
                    }
                }

                let reduced = transition_factor.reduce(&evidence)?;
                let mut marginalized = reduced;
                if marginalized.scope().len() > 1 {
                    let vars_to_sum: Vec<VariableId> = marginalized.scope().variable_ids()
                        .iter().filter(|&&id| id != var_id).copied().collect();
                    marginalized = marginalized.marginalize(&vars_to_sum)?;
                }
                marginalized.normalize();
                next_beliefs.insert(var_id, marginalized);
            } else {
                let scope = Scope::from_ids_and_sizes(vec![var_id], vec![domain_size]);
                next_beliefs.insert(var_id, TabularFactor::identity(scope)?);
            }
        }

        Ok(next_beliefs)
    }

    fn uniform_beliefs(&self) -> LutufiResult<HashMap<VariableId, TabularFactor>> {
        let mut beliefs = HashMap::new();
        let var_ids: Vec<VariableId> = self.dbn.prior().variables().keys().copied().collect();
        for &var_id in &var_ids {
            let var = self.dbn.prior().variables().get(&var_id).ok_or_else(|| LutufiError::InternalError {
                message: "Variable not found".to_string(),
            })?;
            let domain_size = var.domain().size().unwrap_or(2);
            let uniform = vec![1.0 / domain_size as f64; domain_size];
            let scope = Scope::from_ids_and_sizes(vec![var_id], vec![domain_size]);
            beliefs.insert(var_id, TabularFactor::from_values(scope, uniform)?);
        }
        Ok(beliefs)
    }
}

impl DBNInference for DBNInferenceEngine {
    fn filter(&self, evidence_sequence: &[HashMap<String, String>]) -> LutufiResult<DBNInferenceResult> {
        let start_time = Instant::now();

        if evidence_sequence.is_empty() {
            return Err(LutufiError::InternalError { message: "Evidence sequence must contain at least one time step".to_string() });
        }
        if evidence_sequence.len() > self.options.max_time_steps {
            return Err(LutufiError::InternalError { message: format!("Evidence sequence length {} exceeds maximum {}", evidence_sequence.len(), self.options.max_time_steps) });
        }
        for (t, evidence) in evidence_sequence.iter().enumerate() {
            self.validate_evidence_step(evidence).map_err(|e| LutufiError::InternalError {
                message: format!("Invalid evidence at time step {}: {}", t, e),
            })?;
        }

        let _var_ids: Vec<VariableId> = self.dbn.prior().variables().keys().copied().collect();
        let mut current_beliefs = self.uniform_beliefs()?;
        let mut marginals = Vec::new();
        let mut log_evidence = 0.0;

        for (t, evidence) in evidence_sequence.iter().enumerate() {
            let mut evidence_assign = Assignment::new();
            for (name, value) in evidence {
                let var_id = self.dbn.prior().id_of(name)?;
                let var = self.dbn.prior().variable(name)?;
                let state_idx = var.domain().index_of(value).ok_or_else(|| LutufiError::ValueNotInDomain {
                    value: value.clone(), variable: name.clone(), valid_values: format!("{:?}", var.domain()),
                })?;
                evidence_assign.set_discrete(var_id, state_idx)?;
            }

            if t > 0 {
                current_beliefs = self.apply_transition(&current_beliefs)?;
            }

            for (var_id, belief) in current_beliefs.iter_mut() {
                *belief = belief.reduce(&evidence_assign)?;
                if belief.scope().is_empty() {
                    let var = self.dbn.prior().variables().get(var_id).ok_or_else(|| LutufiError::InternalError { message: "Variable not found".to_string() })?;
                    let domain_size = var.domain().size().unwrap_or(2);
                    *belief = TabularFactor::identity(Scope::from_ids_and_sizes(vec![*var_id], vec![domain_size]))?;
                }
                belief.normalize();
            }

            marginals.push(current_beliefs.clone());

            for (name, value) in evidence {
                if let Ok(var_id) = self.dbn.prior().id_of(name) {
                    if let Some(belief) = current_beliefs.get(&var_id) {
                        let var = self.dbn.prior().variable(name)?;
                        if let Some(state_idx) = var.domain().index_of(value) {
                            if state_idx < belief.scope().num_entries() {
                                log_evidence += belief.log_value_at(state_idx);
                            }
                        }
                    }
                }
            }
        }

        Ok(DBNInferenceResult {
            time_steps: (0..marginals.len()).collect(),
            marginals,
            log_evidence,
            algorithm: "DBNFiltering".to_string(),
            computation_time: start_time.elapsed(),
            warnings: vec![],
        })
    }

    fn smooth(&self, evidence_sequence: &[HashMap<String, String>]) -> LutufiResult<DBNInferenceResult> {
        let start_time = Instant::now();
        let t = evidence_sequence.len();
        if t == 0 {
            return Err(LutufiError::InternalError { message: "Evidence sequence must contain at least one time step".to_string() });
        }

        let filter_result = self.filter(evidence_sequence)?;
        let mut smoothed = filter_result.marginals.clone();
        let var_ids: Vec<VariableId> = self.dbn.prior().variables().keys().copied().collect();

        if t > 1 {
            let mut backwards_messages = vec![HashMap::new(); t];

            for &var_id in &var_ids {
                let var = self.dbn.prior().variables().get(&var_id).ok_or_else(|| LutufiError::InternalError { message: "Variable not found".to_string() })?;
                let domain_size = var.domain().size().unwrap_or(2);
                backwards_messages[t - 1].insert(var_id, TabularFactor::identity(Scope::from_ids_and_sizes(vec![var_id], vec![domain_size]))?);
            }

            for i in (0..t - 1).rev() {
                let mut new_backward = HashMap::new();
                for &var_id in &var_ids {
                    let domain_size = self.dbn.prior().variables().get(&var_id).map(|v| v.domain().size().unwrap_or(2)).unwrap_or(2);
                    let mut back_msg = TabularFactor::identity(Scope::from_ids_and_sizes(vec![var_id], vec![domain_size]))?;
                    if let Some(next_back) = backwards_messages[i + 1].get(&var_id) {
                        back_msg = back_msg.multiply(next_back)?;
                    }
                    if !back_msg.scope().is_empty() {
                        let mut normalized = back_msg;
                        normalized.normalize();
                        new_backward.insert(var_id, normalized);
                    } else {
                        new_backward.insert(var_id, back_msg);
                    }
                }
                backwards_messages[i] = new_backward;
            }

            for i in 0..t {
                for &var_id in &var_ids {
                    if let (Some(filtered_belief), Some(back_msg)) = (smoothed[i].get(&var_id), backwards_messages[i].get(&var_id)) {
                        let combined = filtered_belief.multiply(back_msg)?;
                        if !combined.scope().is_empty() {
                            let mut norm = combined;
                            norm.normalize();
                            smoothed[i].insert(var_id, norm);
                        }
                    }
                }
            }
        }

        Ok(DBNInferenceResult {
            time_steps: (0..smoothed.len()).collect(),
            marginals: smoothed,
            log_evidence: filter_result.log_evidence,
            algorithm: "DBNSmoothing".to_string(),
            computation_time: start_time.elapsed(),
            warnings: filter_result.warnings,
        })
    }

    fn predict(&self, evidence_sequence: &[HashMap<String, String>], horizon: usize) -> LutufiResult<DBNInferenceResult> {
        let start_time = Instant::now();
        let filter_result = self.filter(evidence_sequence)?;
        let last_beliefs = filter_result.marginals.last().ok_or_else(|| LutufiError::InternalError {
            message: "No filtering result available for prediction".to_string(),
        })?;

        let mut predicted = Vec::new();
        let mut current = last_beliefs.clone();

        for _ in 1..=horizon {
            current = self.apply_transition(&current)?;
            for (_, belief) in current.iter_mut() { belief.normalize(); }
            predicted.push(current.clone());
        }

        let mut marginals = filter_result.marginals;
        marginals.extend(predicted);

        Ok(DBNInferenceResult {
            time_steps: (0..marginals.len()).collect(),
            marginals,
            log_evidence: filter_result.log_evidence,
            algorithm: "DBNPrediction".to_string(),
            computation_time: start_time.elapsed(),
            warnings: filter_result.warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Domain;

    #[test]
    fn test_filter_empty_evidence() {
        let dbn = DynamicBayesianNetwork::new();
        let engine = DBNInferenceEngine::new(dbn);
        assert!(engine.filter(&[]).is_err());
    }

    #[test]
    fn test_filter_single_step() {
        let mut dbn = DynamicBayesianNetwork::new();
        dbn.add_variable("X", Domain::binary()).unwrap();
        let engine = DBNInferenceEngine::new(dbn);
        let evidence = vec![std::collections::HashMap::from([("X".to_string(), "true".to_string())])];
        let result = engine.filter(&evidence).unwrap();
        assert_eq!(result.marginals.len(), 1);
    }
}
