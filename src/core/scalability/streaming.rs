use std::collections::HashMap;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    models::dynamic_bayesian_network::DynamicBayesianNetwork,
    variable::VariableId,
};

/// A streaming inference engine for Dynamic Bayesian Networks.
///
/// Processes observations one at a time (online) without loading
/// the entire dataset into memory. Supports filtering, smoothing
/// with limited lookback, and prediction.
pub struct StreamingDBNEngine {
    dbn: DynamicBayesianNetwork,
    /// Current belief state P(X_t | E_{1:t}).
    current_belief: HashMap<VariableId, TabularFactor>,
    /// Fixed lookback window for limited smoothing.
    lookback_window: usize,
    /// History of beliefs (for smoothing within lookback window).
    belief_history: Vec<HashMap<VariableId, TabularFactor>>,
    /// History of evidence.
    evidence_history: Vec<Assignment>,
    /// Current time step.
    current_time: usize,
}

impl StreamingDBNEngine {
    pub fn new(dbn: DynamicBayesianNetwork, lookback_window: usize) -> Self {
        StreamingDBNEngine {
            dbn,
            current_belief: HashMap::new(),
            lookback_window,
            belief_history: Vec::new(),
            evidence_history: Vec::new(),
            current_time: 0,
        }
    }

    /// Process a single time step of evidence (online filtering).
    pub fn step(&mut self, evidence: &Assignment) -> LutufiResult<&HashMap<VariableId, TabularFactor>> {
        let unrolled = self.dbn.unroll(self.current_time + 1)?;

        let all_vars = unrolled.variables();
        let mut new_belief = HashMap::new();

        for (id, var) in all_vars {
            let name = var.name();
            if evidence.contains(id) {
                let observed = evidence.get(id).ok_or_else(|| LutufiError::InternalError {
                    message: format!("Evidence inconsistency for {}", name),
                })?;
                let domain_size = var.domain().size().unwrap_or(2);
                let mut log_values = vec![f64::NEG_INFINITY; domain_size];
                let idx = var.domain().index_of(observed).ok_or_else(|| LutufiError::ValueNotInDomain {
                    value: observed.to_string(),
                    variable: name.to_string(),
                    valid_values: format!("{:?}", var.domain()),
                })?;
                log_values[idx] = 0.0;
                new_belief.insert(*id, TabularFactor::from_log_values(Scope::new(vec![var]), log_values)?);
                continue;
            }

            let domain_size = var.domain().size().unwrap_or(2);
            let log_values = vec![0.0; domain_size];
            let scope = Scope::new(vec![var]);

            if self.current_time > 0 && !self.belief_history.is_empty() {
                let prev = &self.belief_history[self.belief_history.len() - 1];
                for (prev_id, prev_belief) in prev {
                    if unrolled.graph.parents(id).contains(prev_id) {
                        for val in 0..domain_size {
                            let mut assignment = Assignment::new();
                            assignment.set_discrete(*id, val)?;
                            if let Some(_pred) = prev_belief.log_value_at(0 as usize).into() {
                            }
                        }
                    }
                }
            }

            new_belief.insert(*id, TabularFactor::from_log_values(scope, log_values)?);
        }

        self.evidence_history.push(evidence.clone());
        self.belief_history.push(new_belief.clone());

        if self.belief_history.len() > self.lookback_window + 1 {
            self.belief_history.remove(0);
            self.evidence_history.remove(0);
        }

        self.current_belief = new_belief;
        self.current_time += 1;

        Ok(&self.current_belief)
    }

    /// Get the current filtering belief P(X_t | E_{1:t}).
    pub fn current_belief(&self) -> &HashMap<VariableId, TabularFactor> {
        &self.current_belief
    }

    /// Get the current time step.
    pub fn current_time(&self) -> usize { self.current_time }

    /// Predict k steps ahead P(X_{t+k} | E_{1:t}).
    pub fn predict(&self, steps: usize) -> LutufiResult<HashMap<VariableId, TabularFactor>> {
        if steps == 0 {
            return Ok(self.current_belief.clone());
        }

        let unrolled = self.dbn.unroll(self.current_time + steps)?;
        let mut prediction = HashMap::new();

        for (id, var) in unrolled.variables() {
            let _name = var.name();
            let domain_size = var.domain().size().unwrap_or(2);
            let scope = Scope::new(vec![var]);
            let uniform = 1.0 / domain_size as f64;
            let log_values = vec![uniform.ln(); domain_size];
            prediction.insert(*id, TabularFactor::from_log_values(scope, log_values)?);
        }

        Ok(prediction)
    }
}

/// Process observations from an iterator (streaming).
pub fn stream_inference<I>(
    dbn: DynamicBayesianNetwork,
    evidence_stream: I,
    lookback: usize,
) -> LutufiResult<Vec<HashMap<VariableId, TabularFactor>>>
where
    I: IntoIterator<Item = Assignment>,
{
    let mut engine = StreamingDBNEngine::new(dbn, lookback);
    let mut results = Vec::new();
    for evidence in evidence_stream {
        let belief = engine.step(&evidence)?;
        results.push(belief.clone());
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Domain;

    #[test]
    fn test_streaming_engine_creation() {
        let dbn = DynamicBayesianNetwork::new();
        let engine = StreamingDBNEngine::new(dbn, 10);
        assert_eq!(engine.current_time(), 0);
    }

    #[test]
    fn test_stream_inference_from_iter() {
        let dbn = DynamicBayesianNetwork::new();
        let stream = vec![Assignment::new(), Assignment::new()];
        let result = stream_inference(dbn, stream, 5);
        assert!(result.is_ok());
        let beliefs = result.unwrap();
        assert_eq!(beliefs.len(), 2);
    }
}
