use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    models::bayesian_network::BayesianNetwork,
    variable::VariableId,
    assignment::Assignment,
    graph::DirectedVariableGraph,
};

/// A Structural Causal Model (SCM) or Causal Bayesian Network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalModel {
    pub network: BayesianNetwork,
    bidirected_edges: HashSet<(VariableId, VariableId)>,
}

impl CausalModel {
    /// Create a new causal model wrapping a Bayesian network.
    pub fn new(mut network: BayesianNetwork) -> Self {
        network.mark_as_causal();
        CausalModel {
            network,
            bidirected_edges: HashSet::new(),
        }
    }

    /// Check that the model is causal, raising NonCausalModel error if not.
    fn ensure_causal(&self, operation: &str) -> LutufiResult<()> {
        if !self.network.is_causal() {
            Err(LutufiError::NonCausalModel {
                variable: operation.to_string(),
                value: String::new(),
            })
        } else {
            Ok(())
        }
    }

    /// Mark a hidden confounder between two variables using a bidirected edge.
    pub fn mark_hidden_confounder(&mut self, var1: &str, var2: &str) -> LutufiResult<()> {
        let id1 = self.network.id_of(var1)?;
        let id2 = self.network.id_of(var2)?;
        let pair = if id1 < id2 { (id1, id2) } else { (id2, id1) };
        self.bidirected_edges.insert(pair);
        Ok(())
    }

    /// Apply the do-operator: returns mutilated model with incoming edges removed.
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

    /// Check the back-door criterion.
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

    /// Check the front-door criterion.
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

    /// Compute the front-door adjustment: P(Y=y | do(X=x)) using a mediator Z.
    /// Formula: P(y | do(x)) = Σ_z P(z | x) Σ_x' P(y | x', z) P(x')
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
            return Err(LutufiError::InternalError {
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

        // Resolve domain values for iteration
        let t_values: Vec<String> = match t_domain {
            crate::core::domain::Domain::Discrete { states } => states.clone(),
            crate::core::domain::Domain::Binary => vec!["false".to_string(), "true".to_string()],
            crate::core::domain::Domain::Continuous { .. } => {
                return Err(LutufiError::InternalError { message: "Front-door adjustment requires discrete domains".to_string() });
            }
        };
        let z_values: Vec<String> = match z_domain {
            crate::core::domain::Domain::Discrete { states } => states.clone(),
            crate::core::domain::Domain::Binary => vec!["false".to_string(), "true".to_string()],
            crate::core::domain::Domain::Continuous { .. } => {
                return Err(LutufiError::InternalError { message: "Front-door adjustment requires discrete domains".to_string() });
            }
        };

        // Formula: P(y | do(x)) = Σ_z P(z | x) Σ_x' P(y | x', z) P(x')

        // Get P(x') marginal for all x'
        let mut p_x_marginal = Vec::new();
        for xv in &t_values {
            p_x_marginal.push(
                self.network.query(&[treatment], &Assignment::new(), crate::core::inference::Algorithm::Auto)?
                    .marginal_prob(treatment, xv).unwrap_or(0.0)
            );
        }

        let t_idx = t_values.iter().position(|v| v == treatment_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: treatment_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_values),
        })?;

        let mut sum_over_z = 0.0;
        for z_idx in 0..z_values.len() {
            let zv = &z_values[z_idx];

            // P(Z=z | X=x)
            let mut ev_x = Assignment::new();
            ev_x.set_discrete(t_id, t_idx)?;
            let p_z_given_x = self.network.query(&[mediator], &ev_x, crate::core::inference::Algorithm::Auto)?
                .marginal_prob(mediator, zv).unwrap_or(0.0);

            if p_z_given_x < 1e-15 { continue; }

            // Σ_x' P(Y=y | X=x', Z=z) * P(X=x')
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

    fn has_directed_path(&self, graph: &DirectedVariableGraph, from: &VariableId, to: &VariableId) -> bool {
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

    /// C-components connected by bidirected edges.
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
                    for (u, v) in &self.bidirected_edges {
                        if *u == current && unvisited.contains(v) { stack.push(*v); }
                        else if *v == current && unvisited.contains(u) { stack.push(*u); }
                    }
                }
            }
            components.push(component);
        }
        components
    }

    /// C-components of a subgraph defined by removing outgoing edges from X.
    pub fn c_components_in_subgraph(&self, remove_outgoing: &HashSet<VariableId>) -> Vec<HashSet<VariableId>> {
        let mut components = Vec::new();
        let mut unvisited: HashSet<VariableId> = self.network.variables().keys().copied().collect();

        let mut bi_in_subgraph: HashSet<(VariableId, VariableId)> = HashSet::new();
        for &(u, v) in &self.bidirected_edges {
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

    /// Ancestors of a set in a graph with certain edges removed.
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

    /// Compute variables in a subgraph of G restricted to a set of variables.
    #[allow(dead_code)]
    fn subgraph_over(&self, ids: &HashSet<VariableId>) -> DirectedVariableGraph {
        let mut sub = DirectedVariableGraph::new();
        for &id in ids {
            sub.add_node(id);
        }
        for &id in ids {
            for child in self.network.graph.children(&id) {
                if ids.contains(&child) {
                    if let (Some(from_name), Some(to_name)) = (
                        self.network.registry().variable(&id).map(|v| v.name().to_string()),
                        self.network.registry().variable(&child).map(|v| v.name().to_string()),
                    ) {
                        let _ = sub.add_edge(&id, &child, &from_name, &to_name);
                    }
                }
            }
        }
        sub
    }

    /// Get all variable IDs.
    fn all_var_ids(&self) -> HashSet<VariableId> {
        self.network.variables().keys().copied().collect()
    }

    /// Check if X has any ancestors in G_X̅ (graph with incoming edges to intervention vars removed).
    #[allow(dead_code)]
    fn ancestors_of_intervention(&self, interventions: &HashSet<VariableId>) -> HashSet<VariableId> {
        self.ancestors_in_mutilated(interventions, interventions)
    }

    /// Full ID algorithm (Shpitser & Pearl 2006).
    /// Identifies P(Y | do(X)) and returns a formula or a non-identifiability proof.
    pub fn identify(&self, targets: &[&str], interventions: &[&str]) -> LutufiResult<IdentificationResult> {
        self.ensure_causal("identify")?;
        if targets.is_empty() || interventions.is_empty() {
            return Err(LutufiError::InternalError {
                message: "Targets and interventions cannot be empty".to_string(),
            });
        }

        // Quick check: try back-door and front-door first
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

        // Full ID algorithm via c-component decomposition
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

    /// Recursive ID algorithm.
    fn id_recursive(&self, targets: &HashSet<VariableId>, interventions: &HashSet<VariableId>, depth: usize) -> LutufiResult<IdResult> {
        let _indent = "  ".repeat(depth);
        let all_vars = self.all_var_ids();

        // Step 1: If no interventions, sum over non-targets
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

        // Step 2: Remove variables that are not ancestors of targets in G_X̅
        let remove_incoming = interventions.clone();
        let ancestors = self.ancestors_in_mutilated(targets, &remove_incoming);
        let non_ancestors: HashSet<VariableId> = all_vars.difference(&ancestors).copied().collect();
        if !non_ancestors.is_empty() {
            let new_interventions: HashSet<VariableId> = interventions.intersection(&ancestors).copied().collect();
            return self.id_recursive(targets, &new_interventions, depth + 1);
        }

        // Step 3: Find c-components of G_X (graph with outgoing edges from interventions removed)
        let components = self.c_components_in_subgraph(interventions);

        // Step 4: If more than one c-component, decompose
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

        // Step 5: Single c-component case
        if let Some(component) = components.first() {
            let c_set: HashSet<VariableId> = component.iter().copied().collect();

            // Check for hedge: if component is subset of interventions, non-identifiable
            if c_set.iter().all(|v| interventions.contains(v)) {
                let var_names: Vec<String> = c_set.iter()
                    .filter_map(|id| self.network.variables().get(id).map(|v| v.name().to_string()))
                    .collect();
                return Ok(IdResult::Fail(format!(
                    "Hedge structure detected: c-component {{{}}} is fully intervened. This forms a hedge proving non-identifiability.",
                    var_names.join(", ")
                )));
            }

            // Compute ancestors of targets in G_X̅ restricted to c-component
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

    /// Conditional identification: P(Y | do(X), Z).
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

    /// Perform an interventional query: P(Y | do(X=x)).
    pub fn causal_query(
        &self,
        targets: &[&str],
        interventions: &Assignment,
    ) -> LutufiResult<crate::core::inference::InferenceResult> {
        self.ensure_causal("causal_query")?;
        let mutilated = self.do_operator(interventions)?;
        mutilated.query(targets, &Assignment::new(), crate::core::inference::Algorithm::Auto)
    }

    /// Perform counterfactual inference: P(Y_{X=x} | E=e).
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

    /// Probability of Necessity: PN = P(Y_{X=x'} = y | X=x, Y=y)
    /// The probability that the outcome Y would not have occurred if X had been x',
    /// given that X was x and Y was y.
    pub fn probability_of_necessity(
        &self,
        outcome: &str,
        outcome_value: &str,
        treatment: &str,
        treatment_value: &str,
        reference_value: &str,
    ) -> LutufiResult<f64> {
        self.ensure_causal("probability_of_necessity")?;
        // PN = (P(Y=y | X=x) - P(Y=y | do(X=x'))) / P(Y=y | X=x)
        // under monotonicity assumption.

        // Compute P(Y=y | X=x) via observational inference
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

        let obs_result = self.network.query(&[outcome], &obs_evidence, crate::core::inference::Algorithm::Auto)?;
        let p_y_given_x = obs_result.marginal_prob(outcome, outcome_value)?;

        if p_y_given_x <= 1e-15 {
            return Ok(0.0);
        }

        // Compute P(Y=y | do(X=x')) via mutilated inference
        let mut ref_intervention = Assignment::new();
        let _ref_idx = t_var.domain().index_of(reference_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: reference_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_var.domain()),
        })?;
        ref_intervention.set(t_id, reference_value);

        let mutilated = self.do_operator(&ref_intervention)?;
        let mutilated_result = mutilated.query(&[outcome], &Assignment::new(), crate::core::inference::Algorithm::Auto)?;
        let p_y_do_ref = mutilated_result.marginal_prob(outcome, outcome_value)?;

        let pn = (p_y_given_x - p_y_do_ref) / p_y_given_x;
        Ok(pn.max(0.0).min(1.0))
    }

    /// Probability of Sufficiency: PS = P(Y_{X=x} = y | X=x', Y=y')
    /// The probability that setting X to x would have resulted in Y=y,
    /// given that X was x' and Y was y'.
    pub fn probability_of_sufficiency(
        &self,
        outcome: &str,
        outcome_value: &str,
        treatment: &str,
        treatment_value: &str,
        reference_value: &str,
    ) -> LutufiResult<f64> {
        self.ensure_causal("probability_of_sufficiency")?;
        // PS = (P(Y=y | do(X=x)) - P(Y=y | X=x')) / P(Y=y' | X=x')
        // under monotonicity assumption.

        // Compute P(Y=y' | X=x') via observational inference
        let t_id = self.network.id_of(treatment)?;
        let t_var = self.network.registry().variable(&t_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: treatment.to_string(), available: "".to_string(),
        })?;
        let _ref_idx = t_var.domain().index_of(reference_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: reference_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_var.domain()),
        })?;

        let mut ref_evidence = Assignment::new();
        ref_evidence.set(t_id, reference_value);
        let ref_result = self.network.query(&[outcome], &ref_evidence, crate::core::inference::Algorithm::Auto)?;
        let p_y_given_ref = ref_result.marginal_prob(outcome, outcome_value)?;
        let p_not_y_given_ref = 1.0 - p_y_given_ref;

        if p_not_y_given_ref <= 1e-15 {
            return Ok(0.0);
        }

        // Compute P(Y=y | do(X=x)) via mutilated inference
        let _treat_idx = t_var.domain().index_of(treatment_value).ok_or_else(|| LutufiError::ValueNotInDomain {
            value: treatment_value.to_string(), variable: treatment.to_string(),
            valid_values: format!("{:?}", t_var.domain()),
        })?;
        let mut treatment_intervention = Assignment::new();
        treatment_intervention.set(t_id, treatment_value);

        let mutilated = self.do_operator(&treatment_intervention)?;
        let mutilated_result = mutilated.query(&[outcome], &Assignment::new(), crate::core::inference::Algorithm::Auto)?;
        let p_y_do_treat = mutilated_result.marginal_prob(outcome, outcome_value)?;

        let ps = (p_y_do_treat - p_y_given_ref) / p_not_y_given_ref;
        Ok(ps.max(0.0).min(1.0))
    }
}

/// Result of a causal identification query.
#[derive(Debug, Clone)]
pub enum IdentificationResult {
    /// The causal effect is identifiable.
    Identifiable(IdentificationFormula),
    /// The causal effect is NOT identifiable.
    NotIdentifiable(String),
}

/// A specific identification formula that can be evaluated against data.
#[derive(Debug, Clone)]
pub struct IdentificationFormula {
    /// Human-readable formula string.
    pub formula: String,
    /// Target variables (outcomes).
    pub targets: Vec<String>,
    /// Intervention variables.
    pub interventions: Vec<String>,
}

impl IdentificationFormula {
    /// Evaluate the identified causal effect from observational data.
    /// Uses the do-operator on the causal model to compute P(Y | do(X)).
    pub fn evaluate(
        &self,
        model: &CausalModel,
        outcome_value: &str,
    ) -> LutufiResult<std::collections::HashMap<String, f64>> {
        let mut results = std::collections::HashMap::new();

        // Build intervention assignment for all intervention variables
        // using actual state names (not indices) for do_operator compatibility
        let mut intervention_assign = Assignment::new();
        for intervention in &self.interventions {
            let var = model.network.variable(intervention)?;
            let var_id = var.id();
            // Use the outcome_value as the intervention value if it's valid for this var
            if var.domain().contains(outcome_value) {
                intervention_assign.set(var_id, outcome_value);
            } else {
                // Use first state by default
                let states = match var.domain() {
                    crate::core::domain::Domain::Discrete { states } => states.clone(),
                    crate::core::domain::Domain::Binary => vec!["false".to_string(), "true".to_string()],
                    crate::core::domain::Domain::Continuous { .. } => return Err(LutufiError::InternalError {
                        message: "Cannot evaluate with continuous domains".to_string(),
                    }),
                };
                if let Some(first) = states.first() {
                    intervention_assign.set(var_id, first.clone());
                }
            }
        }

        let mutilated = model.do_operator(&intervention_assign)?;
        let query_vars: Vec<&str> = self.targets.iter().map(|s| s.as_str()).collect();

        for target in &self.targets {
            let result = mutilated.query(
                &query_vars,
                &Assignment::new(),
                crate::core::inference::Algorithm::Auto,
            )?;
            results.insert(
                target.clone(),
                result.marginal_prob(target, outcome_value).unwrap_or(0.0),
            );
        }

        Ok(results)
    }
}

/// Internal recursive ID algorithm result.
enum IdResult {
    Formula(String),
    Fail(String),
}
