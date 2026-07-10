use crate::core::assignment::Assignment;
use crate::core::error::{LutufiError, LutufiResult};
use crate::core::models::bayesian_network::BayesianNetwork;
use super::types::CausalModel;

impl CausalModel {
    /// Applies the do-operator by mutilating the graph (removing incoming edges to intervened nodes)
    /// and fixing their values, returning the interventional Bayesian network.
    pub fn do_operator(&self, intervention: &Assignment) -> LutufiResult<BayesianNetwork> {
        self.ensure_causal("do_operator")?;
        let mut mutilated = self.network.clone();
        for (var_id, value) in intervention.iter() {
            let var_name = mutilated.registry().variable(&var_id)
                .ok_or_else(|| LutufiError::VariableNotFound { name: var_id.to_string(), available: "".to_string() })?
                .name().to_string();

            let parent_ids = mutilated.graph.parents(&var_id);
            let parents: Vec<String> = parent_ids.iter().filter_map(|id| {
                mutilated.registry().variable(id).map(|v| v.name().to_string())
            }).collect();

            for parent_name in parents {
                mutilated.remove_edge(&parent_name, &var_name)?;
            }

            let var = mutilated.registry().variable(&var_id).ok_or_else(|| LutufiError::VariableNotFound {
                name: var_id.to_string(), available: "".to_string(),
            })?;
            let domain_size = var.domain().size().ok_or_else(|| LutufiError::InternalError {
                message: "Intervened variable must have a discrete domain".to_string(),
            })?;
            let val_idx = var.domain().index_of(value).ok_or_else(|| LutufiError::ValueNotInDomain {
                value: value.clone(), variable: var_name.clone(),
                valid_values: format!("{:?}", var.domain()),
            })?;

            let mut point_values = vec![vec![0.0]; domain_size];
            point_values[val_idx][0] = 1.0;
            let cpd = crate::core::factor::ConditionalProbabilityTable::from_values(var, &[], point_values)?;
            mutilated.set_cpd(&var_name, cpd)?;
        }
        Ok(mutilated)
    }

    /// Performs a causal query by applying the do-operator and then querying the mutilated network.
    pub fn causal_query(
        &self,
        targets: &[&str],
        interventions: &Assignment,
    ) -> LutufiResult<crate::core::inference::InferenceResult> {
        self.ensure_causal("causal_query")?;
        let mutilated = self.do_operator(interventions)?;
        mutilated.query(targets, &Assignment::new(), crate::core::inference::Algorithm::Auto)
    }

    /// Computes a counterfactual query: given observed evidence, applies an intervention and queries.
    pub fn counterfactual(
        &self,
        observed: &Assignment,
        intervention: &Assignment,
        query: &[&str],
    ) -> LutufiResult<crate::core::inference::InferenceResult> {
        self.ensure_causal("counterfactual")?;
        let mutilated = self.do_operator(intervention)?;
        mutilated.query(query, observed, crate::core::inference::Algorithm::Auto)
    }

    /// Computes the probability of necessity (PN): the probability that the outcome would not have occurred
    /// had the treatment been absent, given that it did occur under the actual treatment.
    pub fn probability_of_necessity(
        &self,
        outcome: &str,
        outcome_value: &str,
        treatment: &str,
        treatment_value: &str,
        reference_value: &str,
    ) -> LutufiResult<f64> {
        self.ensure_causal("probability_of_necessity")?;

        let mut obs_evidence = Assignment::new();
        let t_id = self.network.id_of(treatment)?;
        let t_var = self.network.registry().variable(&t_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: treatment.to_string(), available: "".to_string(),
        })?;
        let _t_idx = t_var.domain().index_of(treatment_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: treatment_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_var.domain()),
        })?;
        obs_evidence.set(t_id, treatment_value);

        // marginal_prob resolves states by index (factor scopes carry no
        // state names), so translate the outcome's state name to its domain
        // index up front. Passing the raw name only works for "true"/"false".
        let outcome_idx = self.outcome_state_index(outcome, outcome_value)?;

        let obs_result = self.network.query(&[outcome], &obs_evidence, crate::core::inference::Algorithm::Auto)?;
        let p_y_given_x = obs_result.marginal_prob(outcome, &outcome_idx.to_string())?;

        if p_y_given_x <= 1e-15 {
            return Ok(0.0);
        }

        let mut ref_intervention = Assignment::new();
        let _ref_idx = t_var.domain().index_of(reference_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: reference_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_var.domain()),
        })?;
        ref_intervention.set(t_id, reference_value);

        let mutilated = self.do_operator(&ref_intervention)?;
        let mutilated_result = mutilated.query(&[outcome], &Assignment::new(), crate::core::inference::Algorithm::Auto)?;
        let p_y_do_ref = mutilated_result.marginal_prob(outcome, &outcome_idx.to_string())?;

        let pn = (p_y_given_x - p_y_do_ref) / p_y_given_x;
        Ok(pn.max(0.0).min(1.0))
    }

    /// Resolve an outcome variable's state name to its domain index, for use
    /// with `InferenceResult::marginal_prob` (which resolves by index).
    fn outcome_state_index(&self, outcome: &str, outcome_value: &str) -> LutufiResult<usize> {
        let o_id = self.network.id_of(outcome)?;
        let o_var = self.network.registry().variable(&o_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: outcome.to_string(), available: "".to_string(),
        })?;
        o_var.domain().index_of(outcome_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: outcome_value.to_string(), variable: outcome.to_string(),
            valid_values: format!("{:?}", o_var.domain()),
        })
    }

    /// Computes the probability of sufficiency (PS): the probability that the outcome would have occurred
    /// had the treatment been present, given that it did not occur under the reference condition.
    pub fn probability_of_sufficiency(
        &self,
        outcome: &str,
        outcome_value: &str,
        treatment: &str,
        treatment_value: &str,
        reference_value: &str,
    ) -> LutufiResult<f64> {
        self.ensure_causal("probability_of_sufficiency")?;

        let t_id = self.network.id_of(treatment)?;
        let t_var = self.network.registry().variable(&t_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: treatment.to_string(), available: "".to_string(),
        })?;
        let _ref_idx = t_var.domain().index_of(reference_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: reference_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_var.domain()),
        })?;

        let outcome_idx = self.outcome_state_index(outcome, outcome_value)?;

        let mut ref_evidence = Assignment::new();
        ref_evidence.set(t_id, reference_value);
        let ref_result = self.network.query(&[outcome], &ref_evidence, crate::core::inference::Algorithm::Auto)?;
        let p_y_given_ref = ref_result.marginal_prob(outcome, &outcome_idx.to_string())?;
        let p_not_y_given_ref = 1.0 - p_y_given_ref;

        if p_not_y_given_ref <= 1e-15 {
            return Ok(0.0);
        }

        let _treat_idx = t_var.domain().index_of(treatment_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: treatment_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_var.domain()),
        })?;
        let mut treatment_intervention = Assignment::new();
        treatment_intervention.set(t_id, treatment_value);

        let mutilated = self.do_operator(&treatment_intervention)?;
        let mutilated_result = mutilated.query(&[outcome], &Assignment::new(), crate::core::inference::Algorithm::Auto)?;
        let p_y_do_treat = mutilated_result.marginal_prob(outcome, &outcome_idx.to_string())?;

        let ps = (p_y_do_treat - p_y_given_ref) / p_not_y_given_ref;
        Ok(ps.max(0.0).min(1.0))
    }
}
