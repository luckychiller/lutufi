use std::collections::{HashMap, HashSet};

use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::TabularFactor,
    models::bayesian_network::BayesianNetwork,
    variable::VariableId,
};

/// Manages evidence for efficient incremental re-inference.
pub struct EvidenceManager {
    model: BayesianNetwork,
    current_evidence: Assignment,
    cached_jt: Option<super::super::junction_tree::JunctionTreeEngine>,
    dirty_variables: HashSet<VariableId>,
}

impl EvidenceManager {
    /// Create a new evidence manager for the given model.
    pub fn new(model: BayesianNetwork) -> Self {
        EvidenceManager {
            model,
            current_evidence: Assignment::new(),
            cached_jt: None,
            dirty_variables: HashSet::new(),
        }
    }

    /// Get a reference to the underlying model.
    pub fn model(&self) -> &BayesianNetwork { &self.model }
    /// Get the current evidence assignment.
    pub fn evidence(&self) -> &Assignment { &self.current_evidence }

    /// Set evidence for a variable by name and value.
    pub fn set_evidence(&mut self, variable: &str, value: &str) -> LutufiResult<()> {
        let var_id = self.model.id_of(variable)?;
        let var = self.model.variable(variable)?;
        var.validate_value(value)?;
        let state_idx = var.domain().index_of(value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: value.to_string(), variable: variable.to_string(), valid_values: format!("{:?}", var.domain()),
        })?;
        self.current_evidence.set_discrete(var_id, state_idx)?;
        self.dirty_variables.insert(var_id);
        Ok(())
    }

    /// Remove evidence for a variable.
    pub fn remove_evidence(&mut self, variable: &str) -> LutufiResult<()> {
        let var_id = self.model.id_of(variable)?;
        self.current_evidence.unset(&var_id);
        self.dirty_variables.insert(var_id);
        Ok(())
    }

    /// Clear all evidence.
    pub fn clear_evidence(&mut self) {
        self.current_evidence = Assignment::new();
        self.dirty_variables.clear();
        self.cached_jt = None;
    }

    /// Compile the evidence into the junction tree for inference.
    pub fn compile(&mut self) -> LutufiResult<()> {
        let jt = super::super::junction_tree::JunctionTreeEngine::new(&self.model)?;
        self.cached_jt = Some(jt);
        self.dirty_variables.clear();
        Ok(())
    }

    /// Run inference on the compiled model to compute marginals for query variables.
    pub fn infer(&self, query_variables: &[&str]) -> LutufiResult<HashMap<String, TabularFactor>> {
        if let Some(ref jt) = self.cached_jt {
            let factor = jt.query(query_variables, &self.current_evidence);
            match factor {
                Ok(f) => Ok(self.extract_marginals(&f, query_variables)),
                Err(e) => Err(e),
            }
        } else {
            let result = crate::core::inference::InferenceEngine::query(
                &self.model, query_variables, &self.current_evidence,
                crate::core::inference::Algorithm::Auto,
            )?;
            let mut map = HashMap::new();
            for (var_name, dist) in &result.distributions {
                if query_variables.contains(&var_name.as_str()) {
                    map.insert(var_name.clone(), dist.clone());
                }
            }
            Ok(map)
        }
    }

    /// Get a map of variable names to their evidence values.
    pub fn evidence_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (var_id, val_str) in self.current_evidence.iter() {
            if let Some(var) = self.model.variables().get(var_id) {
                map.insert(var.name().to_string(), val_str.clone());
            }
        }
        map
    }

    /// Get the number of evidence assignments.
    pub fn evidence_count(&self) -> usize { self.current_evidence.len() }

    fn extract_marginals(&self, factor: &TabularFactor, query: &[&str]) -> HashMap<String, TabularFactor> {
        let mut result = HashMap::new();
        let scope = factor.scope();
        for (_i, &var_id) in scope.variable_ids().iter().enumerate() {
            if let Some(var) = self.model.variables().get(&var_id) {
                if query.contains(&var.name()) {
                    let mut marginal = factor.clone();
                    let others: Vec<VariableId> = scope.variable_ids().iter()
                        .filter(|&&id| id != var_id).copied().collect();
                    if !others.is_empty() {
                        if let Ok(m) = marginal.marginalize(&others) {
                            marginal = m;
                        }
                    }
                    marginal.normalize();
                    result.insert(var.name().to_string(), marginal);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Domain;

    #[test]
    fn test_evidence_manager_add_remove() {
        let mut model = BayesianNetwork::new();
        model.add_variable("X", Domain::binary()).unwrap();
        let mut mgr = EvidenceManager::new(model);
        assert_eq!(mgr.evidence_count(), 0);
        mgr.set_evidence("X", "true").unwrap();
        assert_eq!(mgr.evidence_count(), 1);
        mgr.clear_evidence();
        assert_eq!(mgr.evidence_count(), 0);
    }
}
