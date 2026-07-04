use std::collections::HashSet;
use crate::core::error::{LutufiError, LutufiResult};
use crate::core::variable::VariableId;
use super::types::{CausalModel, IdentificationResult, IdentificationFormula, IdResult};

impl CausalModel {
    /// Identifies the causal effect `P(targets | do(interventions))` using back-door, front-door,
    /// and recursive ID algorithms.
    pub fn identify(&self, targets: &[&str], interventions: &[&str]) -> LutufiResult<IdentificationResult> {
        self.ensure_causal("identify")?;
        if targets.is_empty() || interventions.is_empty() {
            return Err(LutufiError::InternalError {
                message: "Targets and interventions cannot be empty".to_string(),
            });
        }

        let x_name = interventions[0];
        let y_name = targets[0];

        let mk_formula = |formula: String| -> IdentificationResult {
            IdentificationResult::Identifiable(IdentificationFormula {
                formula,
                targets: targets.iter().map(|s| s.to_string()).collect(),
                interventions: interventions.iter().map(|s| s.to_string()).collect(),
            })
        };

        if self.satisfies_backdoor(x_name, y_name, &[])? {
            return Ok(mk_formula(format!("P({} | {})", y_name, x_name)));
        }

        let x_id = self.network.id_of(x_name)?;
        let parents: Vec<String> = self.network.graph.parents(&x_id).iter().filter_map(|id| {
            self.network.variables().get(id).map(|v| v.name().to_string())
        }).collect();
        let parent_refs: Vec<&str> = parents.iter().map(|s| s.as_str()).collect();

        if self.satisfies_backdoor(x_name, y_name, &parent_refs)? {
            let parents_fmt = parents.join(", ");
            return Ok(mk_formula(format!(
                "sum_{{{}}} P({} | {}, {}) P({})",
                parents_fmt, y_name, x_name, parents_fmt, parents_fmt
            )));
        }

        let children = self.network.graph.children(&x_id);
        for child_id in children {
            let child_name = match self.network.variables().get(&child_id) {
                Some(v) => v.name(),
                None => continue,
            };
            if self.satisfies_frontdoor(x_name, y_name, &[child_name])? {
                return Ok(mk_formula(format!("Front-door adjustment via {}", child_name)));
            }
        }

        let target_ids: HashSet<VariableId> = targets.iter().filter_map(|n| self.network.id_of(n).ok()).collect();
        let intervention_ids: HashSet<VariableId> = interventions.iter().filter_map(|n| self.network.id_of(n).ok()).collect();

        let result = self.id_recursive(&target_ids, &intervention_ids, 0)?;
        match result {
            IdResult::Formula(s) => Ok(mk_formula(s)),
            IdResult::Fail(hedge) => {
                let target_str = targets.join(", ");
                let intervention_str = interventions.join(", ");
                Err(LutufiError::NotIdentifiable {
                    target: target_str,
                    intervention: intervention_str,
                    hedge,
                })
            }
        }
    }

    /// Identifies the conditional causal effect `P(targets | conditions, do(interventions))`.
    pub fn identify_conditional(
        &self,
        targets: &[&str],
        interventions: &[&str],
        conditions: &[&str],
    ) -> LutufiResult<IdentificationResult> {
        if conditions.is_empty() {
            return self.identify(targets, interventions);
        }
        let all_targets: Vec<&str> = targets.iter().chain(conditions).copied().collect();
        let joint_result = self.identify(&all_targets, interventions)?;
        match joint_result {
            IdentificationResult::Identifiable(_) => {
                let cond_names = conditions.join(", ");
                let target_names = targets.join(", ");
                Ok(IdentificationResult::Identifiable(IdentificationFormula {
                    formula: format!("P({} | {}, do(...)) identified. Use P({}, {} | do(...)) / P({} | do(...))",
                        target_names, cond_names, target_names, cond_names, cond_names),
                    targets: targets.iter().map(|s| s.to_string()).collect(),
                    interventions: interventions.iter().map(|s| s.to_string()).collect(),
                }))
            }
            IdentificationResult::NotIdentifiable(reason) => {
                Err(LutufiError::NotIdentifiable {
                    target: targets.join(", "),
                    intervention: interventions.join(", "),
                    hedge: format!("Conditional query not identifiable: {}", reason),
                })
            }
        }
    }

    /// Computes the C-components (connected components via bidirected edges) of the full graph.
    pub fn c_components(&self) -> Vec<HashSet<VariableId>> {
        let mut components = Vec::new();
        let mut unvisited: HashSet<VariableId> = self.network.variables().keys().copied().collect();
        while !unvisited.is_empty() {
            let start = *unvisited.iter().next().unwrap_or_else(|| unreachable!());
            let mut component = HashSet::new();
            let mut stack = vec![start];
            while let Some(current) = stack.pop() {
                if component.insert(current) {
                    unvisited.remove(&current);
                    for (u, v) in self.bidirected_edges() {
                        if *u == current && unvisited.contains(v) { stack.push(*v); }
                        else if *v == current && unvisited.contains(u) { stack.push(*u); }
                    }
                }
            }
            components.push(component);
        }
        components
    }

    /// Computes C-components in a subgraph where outgoing edges from variables in `remove_outgoing` are removed.
    pub fn c_components_in_subgraph(&self, remove_outgoing: &HashSet<VariableId>) -> Vec<HashSet<VariableId>> {
        let mut components = Vec::new();
        let mut unvisited: HashSet<VariableId> = self.network.variables().keys().copied().collect();

        let mut bi_in_subgraph: HashSet<(VariableId, VariableId)> = HashSet::new();
        for &(u, v) in self.bidirected_edges() {
            if unvisited.contains(&u) && unvisited.contains(&v) {
                bi_in_subgraph.insert((u, v));
            }
        }
        for var in remove_outgoing {
            let children = self.network.graph.children(var);
            for child in children {
                bi_in_subgraph.retain(|&(u, v)| {
                    !((u == *var && v == child) || (u == child && v == *var))
                });
            }
        }

        while !unvisited.is_empty() {
            let start = *unvisited.iter().next().unwrap_or_else(|| unreachable!());
            let mut component = HashSet::new();
            let mut stack = vec![start];
            while let Some(current) = stack.pop() {
                if component.insert(current) {
                    unvisited.remove(&current);
                    for &(u, v) in &bi_in_subgraph {
                        if u == current && unvisited.contains(&v) { stack.push(v); }
                        else if v == current && unvisited.contains(&u) { stack.push(u); }
                    }
                }
            }
            components.push(component);
        }
        components
    }

    fn ancestors_in_mutilated(&self, vars: &HashSet<VariableId>, remove_incoming: &HashSet<VariableId>) -> HashSet<VariableId> {
        let mut ancestors = vars.clone();
        let mut stack: Vec<VariableId> = vars.iter().copied().collect();
        while let Some(current) = stack.pop() {
            if !remove_incoming.contains(&current) {
                for parent in self.network.graph.parents(&current) {
                    if ancestors.insert(parent) {
                        stack.push(parent);
                    }
                }
            }
        }
        ancestors
    }

    fn all_var_ids(&self) -> HashSet<VariableId> {
        self.network.variables().keys().copied().collect()
    }

    fn id_recursive(&self, targets: &HashSet<VariableId>, interventions: &HashSet<VariableId>, depth: usize) -> LutufiResult<IdResult> {
        let _indent = "  ".repeat(depth);
        let all_vars = self.all_var_ids();

        if interventions.is_empty() {
            let non_targets: Vec<String> = all_vars.difference(targets)
                .filter_map(|id| self.network.variables().get(id).map(|v| v.name().to_string()))
                .collect();
            if non_targets.is_empty() {
                return Ok(IdResult::Formula("P(V)".to_string()));
            }
            return Ok(IdResult::Formula(
                format!("sum_{{{}}} P(V)", non_targets.join(", "))
            ));
        }

        let remove_incoming = interventions.clone();
        let ancestors = self.ancestors_in_mutilated(targets, &remove_incoming);
        let non_ancestors: HashSet<VariableId> = all_vars.difference(&ancestors).copied().collect();
        if !non_ancestors.is_empty() {
            let new_interventions: HashSet<VariableId> = interventions.intersection(&ancestors).copied().collect();
            return self.id_recursive(targets, &new_interventions, depth + 1);
        }

        let components = self.c_components_in_subgraph(interventions);

        if components.len() > 1 {
            let mut formulas = Vec::new();
            for component in &components {
                let comp_targets: HashSet<VariableId> = targets.intersection(component).copied().collect();
                let comp_interventions: HashSet<VariableId> = interventions.intersection(component).copied().collect();
                if comp_targets.is_empty() && comp_interventions.is_empty() {
                    continue;
                }
                let sub_result = self.id_recursive(&comp_targets, &comp_interventions, depth + 1)?;
                match sub_result {
                    IdResult::Formula(f) => formulas.push(f),
                    IdResult::Fail(_) => return Ok(sub_result),
                }
            }
            if formulas.is_empty() {
                return Ok(IdResult::Formula("1".to_string()));
            }
            return Ok(IdResult::Formula(formulas.join(" * ")));
        }

        if let Some(component) = components.first() {
            let c_set: HashSet<VariableId> = component.iter().copied().collect();

            if c_set.iter().all(|v| interventions.contains(v)) {
                let var_names: Vec<String> = c_set.iter()
                    .filter_map(|id| self.network.variables().get(id).map(|v| v.name().to_string()))
                    .collect();
                return Ok(IdResult::Fail(format!(
                    "Hedge structure detected: c-component {{{}}} is fully intervened. This forms a hedge proving non-identifiability.",
                    var_names.join(", ")
                )));
            }

            let ancestors_in_comp = self.ancestors_in_mutilated(targets, interventions);
            let s_set: HashSet<VariableId> = c_set.intersection(&ancestors_in_comp).copied().collect();

            if s_set.is_empty() {
                return Ok(IdResult::Formula("1".to_string()));
            }

            if s_set.len() == targets.len() && s_set.iter().all(|v| targets.contains(v)) {
                let cpd_parts: Vec<String> = s_set.iter()
                    .filter_map(|id| {
                        let var = self.network.variables().get(id)?;
                        let parents: Vec<String> = self.network.graph.parents(id)
                            .iter().filter_map(|p| self.network.variables().get(p).map(|v| v.name().to_string()))
                            .collect();
                        if parents.is_empty() {
                            Some(format!("P({})", var.name()))
                        } else {
                            Some(format!("P({} | {})", var.name(), parents.join(", ")))
                        }
                    })
                    .collect();
                return Ok(IdResult::Formula(cpd_parts.join(" * ")));
            }

            let marginalize: HashSet<VariableId> = s_set.difference(targets).copied().collect();
            let marginalize_str: Vec<String> = marginalize.iter()
                .filter_map(|id| self.network.variables().get(id).map(|v| v.name().to_string()))
                .collect();

            let cpd_parts: Vec<String> = s_set.iter()
                .filter_map(|id| {
                    let var = self.network.variables().get(id)?;
                    let parents: Vec<String> = self.network.graph.parents(id)
                        .iter().filter_map(|p| self.network.variables().get(p).map(|v| v.name().to_string()))
                        .collect();
                    if parents.is_empty() {
                        Some(format!("P({})", var.name()))
                    } else {
                        Some(format!("P({} | {})", var.name(), parents.join(", ")))
                    }
                })
                .collect();

            let numerator = if marginalize.is_empty() {
                cpd_parts.join(" * ")
            } else {
                format!("sum_{{{}}} {}",
                    marginalize_str.join(", "),
                    cpd_parts.join(" * ")
                )
            };

            let s_interventions: HashSet<VariableId> = interventions.intersection(&s_set).copied().collect();
            if s_interventions.is_empty() {
                return Ok(IdResult::Formula(numerator));
            }

            let denominator = self.id_recursive(&s_set, &s_interventions, depth + 1)?;
            match denominator {
                IdResult::Formula(den) => Ok(IdResult::Formula(format!("({}) / ({})", numerator, den))),
                IdResult::Fail(_) => Ok(denominator),
            }
        } else {
            Ok(IdResult::Formula("1".to_string()))
        }
    }
}
