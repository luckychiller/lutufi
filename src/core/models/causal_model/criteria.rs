use std::collections::HashSet;
use crate::core::assignment::Assignment;
use crate::core::error::LutufiResult;
use crate::core::variable::VariableId;
use super::types::CausalModel;

impl CausalModel {
    pub fn satisfies_backdoor(&self, x: &str, y: &str, z: &[&str]) -> LutufiResult<bool> {
        let x_id = self.network.id_of(x)?;
        let descendants = self.network.graph.descendants(&x_id);
        for &z_name in z {
            let z_id = self.network.id_of(z_name)?;
            if descendants.contains(&z_id) { return Ok(false); }
        }
        let mut mutilated_graph = self.network.graph.clone();
        for child_id in mutilated_graph.children(&x_id) {
            mutilated_graph.remove_edge(&x_id, &child_id);
        }
        let mut temp_network = self.network.clone();
        temp_network.graph = mutilated_graph;
        temp_network.d_separated(x, y, z)
    }

    pub fn satisfies_frontdoor(&self, x: &str, y: &str, z: &[&str]) -> LutufiResult<bool> {
        let x_id = self.network.id_of(x)?;
        let y_id = self.network.id_of(y)?;

        let mut no_out_z = self.network.graph.clone();
        for &z_name in z {
            let z_id = self.network.id_of(z_name)?;
            for child_id in no_out_z.children(&z_id) {
                no_out_z.remove_edge(&z_id, &child_id);
            }
        }
        if self.has_directed_path(&no_out_z, &x_id, &y_id) { return Ok(false); }

        let mut g_x_underbar = self.network.graph.clone();
        for child_id in g_x_underbar.children(&x_id) {
            g_x_underbar.remove_edge(&x_id, &child_id);
        }
        let mut temp_net_x = self.network.clone();
        temp_net_x.graph = g_x_underbar;
        for &z_name in z {
            if !temp_net_x.d_separated(x, z_name, &[])? { return Ok(false); }
        }

        if z.is_empty() { return Ok(true); }
        let mut g_z_underbar = self.network.graph.clone();
        for &z_name in z {
            let z_id = self.network.id_of(z_name)?;
            for child_id in g_z_underbar.children(&z_id) {
                g_z_underbar.remove_edge(&z_id, &child_id);
            }
        }
        let mut temp_net_z = self.network.clone();
        temp_net_z.graph = g_z_underbar;
        temp_net_z.d_separated(z[0], y, &[x])
    }

    pub fn frontdoor_adjustment(
        &self,
        treatment: &str,
        treatment_value: &str,
        outcome: &str,
        outcome_value: &str,
        mediator: &str,
    ) -> LutufiResult<f64> {
        self.ensure_causal("frontdoor_adjustment")?;
        if !self.satisfies_frontdoor(treatment, outcome, &[mediator])? {
            return Err(crate::core::error::LutufiError::InternalError {
                message: format!(
                    "Front-door criterion not satisfied: '{}' is not a valid mediator between '{}' and '{}'",
                    mediator, treatment, outcome
                ),
            });
        }

        let t_id = self.network.id_of(treatment)?;
        let t_var = self.network.variable(treatment)?;
        let z_id = self.network.id_of(mediator)?;
        let z_var = self.network.variable(mediator)?;
        let t_domain = t_var.domain();
        let z_domain = z_var.domain();

        let t_values: Vec<String> = match t_domain {
            crate::core::domain::Domain::Discrete { states } => states.clone(),
            crate::core::domain::Domain::Binary => vec!["false".to_string(), "true".to_string()],
            crate::core::domain::Domain::Continuous { .. } => {
                return Err(crate::core::error::LutufiError::InternalError { message: "Front-door adjustment requires discrete domains".to_string() });
            }
        };
        let z_values: Vec<String> = match z_domain {
            crate::core::domain::Domain::Discrete { states } => states.clone(),
            crate::core::domain::Domain::Binary => vec!["false".to_string(), "true".to_string()],
            crate::core::domain::Domain::Continuous { .. } => {
                return Err(crate::core::error::LutufiError::InternalError { message: "Front-door adjustment requires discrete domains".to_string() });
            }
        };

        let mut p_x_marginal = Vec::new();
        for xv in &t_values {
            p_x_marginal.push(
                self.network.query(&[treatment], &Assignment::new(), crate::core::inference::Algorithm::Auto)?
                    .marginal_prob(treatment, xv).unwrap_or(0.0)
            );
        }

        let t_idx = t_values.iter().position(|v| v == treatment_value).ok_or_else(|| crate::core::error::LutufiError::ValueNotInDomain {
            value: treatment_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_values),
        })?;

        let mut sum_over_z = 0.0;
        for z_idx in 0..z_values.len() {
            let zv = &z_values[z_idx];

            let mut ev_x = Assignment::new();
            ev_x.set_discrete(t_id, t_idx)?;
            let p_z_given_x = self.network.query(&[mediator], &ev_x, crate::core::inference::Algorithm::Auto)?
                .marginal_prob(mediator, zv).unwrap_or(0.0);

            if p_z_given_x < 1e-15 { continue; }

            let mut sum_over_x = 0.0;
            for xi in 0..t_values.len() {
                let mut cond_ev = Assignment::new();
                cond_ev.set_discrete(t_id, xi)?;
                cond_ev.set_discrete(z_id, z_idx)?;
                let p_y_given_xz = self.network.query(&[outcome], &cond_ev, crate::core::inference::Algorithm::Auto)?
                    .marginal_prob(outcome, outcome_value).unwrap_or(0.0);
                sum_over_x += p_y_given_xz * p_x_marginal[xi];
            }

            sum_over_z += p_z_given_x * sum_over_x;
        }

        Ok(sum_over_z)
    }

    pub(crate) fn has_directed_path(&self, graph: &crate::core::graph::DirectedVariableGraph, from: &VariableId, to: &VariableId) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![*from];
        while let Some(current) = stack.pop() {
            if current == *to { return true; }
            if visited.insert(current) {
                for child in graph.children(&current) {
                    stack.push(child);
                }
            }
        }
        false
    }
}
