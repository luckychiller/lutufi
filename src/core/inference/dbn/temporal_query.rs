use std::collections::HashMap;

use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::TabularFactor,
    models::dynamic_bayesian_network::DynamicBayesianNetwork,
};

use super::{DBNInference, DBNInferenceEngine, DBNInferenceOptions};

/// Mode of temporal inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalMode {
    Filter,
    Smooth,
    Predict,
}

/// Unified temporal query API for DBNs.
pub struct TemporalQueryEngine {
    dbn: DynamicBayesianNetwork,
    options: DBNInferenceOptions,
}

impl TemporalQueryEngine {
    /// Create a new temporal query engine.
    pub fn new(dbn: DynamicBayesianNetwork) -> Self {
        TemporalQueryEngine { dbn, options: DBNInferenceOptions::default() }
    }

    /// Create a new temporal query engine with custom options.
    pub fn new_with_options(dbn: DynamicBayesianNetwork, options: DBNInferenceOptions) -> Self {
        TemporalQueryEngine { dbn, options }
    }

    /// Get a reference to the underlying DBN.
    pub fn dbn(&self) -> &DynamicBayesianNetwork { &self.dbn }
    /// Get a mutable reference to the underlying DBN.
    pub fn dbn_mut(&mut self) -> &mut DynamicBayesianNetwork { &mut self.dbn }

    /// Run a temporal query with the specified mode.
    pub fn query(
        &self,
        variables: &[&str],
        evidence_sequence: &[HashMap<String, String>],
        time: usize,
        mode: TemporalMode,
    ) -> LutufiResult<HashMap<String, TabularFactor>> {
        let inference = DBNInferenceEngine::new_with_options(self.dbn.clone(), self.options.clone());

        let result = match mode {
            TemporalMode::Filter => inference.filter(evidence_sequence)?,
            TemporalMode::Smooth => inference.smooth(evidence_sequence)?,
            TemporalMode::Predict => {
                let horizon = if time >= evidence_sequence.len() {
                    time - evidence_sequence.len() + 1
                } else { 0 };
                inference.predict(evidence_sequence, horizon)?
            }
        };

        if time >= result.marginals.len() {
            return Err(LutufiError::InternalError {
                message: format!("Time step {} out of range (max {})", time, result.marginals.len() - 1),
            });
        }

        let mut query_result = HashMap::new();
        let beliefs = &result.marginals[time];
        for &var_name in variables {
            let var_id = self.dbn.prior().id_of(var_name)?;
            let factor = beliefs.get(&var_id).ok_or_else(|| LutufiError::VariableNotFound {
                name: var_name.to_string(),
                available: format!("{:?}", self.dbn.prior().nodes()),
            })?;
            query_result.insert(var_name.to_string(), factor.clone());
        }

        Ok(query_result)
    }

    /// Run a filtering query to compute marginals at the current time given evidence up to that time.
    pub fn query_filter(&self, variables: &[&str], evidence_sequence: &[HashMap<String, String>], time: usize) -> LutufiResult<HashMap<String, TabularFactor>> {
        self.query(variables, evidence_sequence, time, TemporalMode::Filter)
    }

    /// Run a smoothing query to compute marginals at a past time given all evidence.
    pub fn query_smooth(&self, variables: &[&str], evidence_sequence: &[HashMap<String, String>], time: usize) -> LutufiResult<HashMap<String, TabularFactor>> {
        self.query(variables, evidence_sequence, time, TemporalMode::Smooth)
    }

    /// Run a prediction query to forecast future states beyond the last evidence time.
    pub fn query_predict(&self, variables: &[&str], evidence_sequence: &[HashMap<String, String>], horizon: usize) -> LutufiResult<HashMap<String, TabularFactor>> {
        let time = evidence_sequence.len() + horizon - 1;
        self.query(variables, evidence_sequence, time, TemporalMode::Predict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Domain;

    #[test]
    fn test_temporal_query_engine_empty() {
        let dbn = DynamicBayesianNetwork::new();
        let engine = TemporalQueryEngine::new(dbn);
        assert!(engine.dbn().prior().nodes().is_empty());
    }
}
