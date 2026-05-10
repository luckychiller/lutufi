#[cfg(test)]
mod tests {
    use lutufi_core::core::{
        models::{
            bayesian_network::BayesianNetwork,
            causal_model::{CausalModel, IdentificationResult},
        },
        domain::Domain,
        assignment::Assignment,
    };

    fn build_simple_scm() -> CausalModel {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("X", Domain::binary()).unwrap();
        bn.add_variable("Y", Domain::binary()).unwrap();
        bn.add_edge("X", "Y").unwrap();

        let x_var = bn.variable("X").unwrap().clone();
        let y_var = bn.variable("Y").unwrap().clone();

        bn.set_cpd("X", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&x_var, &[], vec![vec![0.5], vec![0.5]]).unwrap()).unwrap();
        bn.set_cpd("Y", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&y_var, &[&x_var], vec![vec![0.9, 0.1], vec![0.1, 0.9]]).unwrap()).unwrap();

        CausalModel::new(bn)
    }

    fn build_asia_causal() -> CausalModel {
        let mut model = BayesianNetwork::new();
        let bin = || Domain::binary();
        model.add_variable("Asia", bin()).unwrap();
        model.add_variable("Tub", bin()).unwrap();
        model.add_variable("Smoke", bin()).unwrap();
        model.add_variable("Lung", bin()).unwrap();
        model.add_variable("Bronc", bin()).unwrap();
        model.add_variable("Either", bin()).unwrap();
        model.add_variable("Xray", bin()).unwrap();
        model.add_variable("Dysp", bin()).unwrap();

        model.add_edge("Asia", "Tub").unwrap();
        model.add_edge("Smoke", "Lung").unwrap();
        model.add_edge("Smoke", "Bronc").unwrap();
        model.add_edge("Tub", "Either").unwrap();
        model.add_edge("Lung", "Either").unwrap();
        model.add_edge("Either", "Xray").unwrap();
        model.add_edge("Either", "Dysp").unwrap();
        model.add_edge("Bronc", "Dysp").unwrap();

        let set_cpt = |m: &mut BayesianNetwork, name: &str, values: Vec<Vec<f64>>| {
            let var = m.variable(name).unwrap().clone();
            let mut parents: Vec<lutufi_core::core::variable::Variable> = m.graph.parents(&var.id()).iter()
                .map(|id| m.registry().variable(id).unwrap().clone())
                .collect();
            parents.sort_by_key(|p| p.name().to_string());
            let parent_refs: Vec<&lutufi_core::core::variable::Variable> = parents.iter().collect();
            let cpd = lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&var, &parent_refs, values).unwrap();
            m.set_cpd(name, cpd).unwrap();
        };

        set_cpt(&mut model, "Asia", vec![vec![0.99], vec![0.01]]);
        set_cpt(&mut model, "Smoke", vec![vec![0.5], vec![0.5]]);
        set_cpt(&mut model, "Tub", vec![vec![0.99, 0.95], vec![0.01, 0.05]]);
        set_cpt(&mut model, "Lung", vec![vec![0.99, 0.90], vec![0.01, 0.10]]);
        set_cpt(&mut model, "Bronc", vec![vec![0.7, 0.4], vec![0.3, 0.6]]);
        set_cpt(&mut model, "Either", vec![vec![1.0, 0.0, 0.0, 0.0], vec![0.0, 1.0, 1.0, 1.0]]);
        set_cpt(&mut model, "Xray", vec![vec![0.95, 0.02], vec![0.05, 0.98]]);
        set_cpt(&mut model, "Dysp", vec![vec![0.9, 0.2, 0.3, 0.1], vec![0.1, 0.8, 0.7, 0.9]]);

        CausalModel::new(model)
    }

    #[test]
    fn test_causal_model_creation() {
        let scm = build_simple_scm();
        assert!(scm.network.is_causal());
        assert_eq!(scm.network.nodes().len(), 2);
    }

    #[test]
    fn test_do_operator() {
        let scm = build_simple_scm();
        let mut intervention = Assignment::new();
        let x_id = scm.network.id_of("X").unwrap();
        intervention.set(x_id, "true");
        
        let mutilated = scm.do_operator(&intervention).unwrap();
        assert_eq!(mutilated.edges().len(), 1); // X -> Y should remain
        
        let cpd_x = mutilated.cpd("X").unwrap();
        assert_eq!(cpd_x.as_factor().value_at(0), 0.0);
        assert_eq!(cpd_x.as_factor().value_at(1), 1.0);
    }

    #[test]
    fn test_backdoor_criterion() {
        let scm = build_asia_causal();
        // Smoke -> Lung -> Either
        // Smoke -> Bronc -> Dysp
        // Lung <- Smoke -> Bronc -> Dysp <- Either
        
        // Test: is {} a back-door set for (Lung, Either)?
        // Backdoor path: Lung <- Smoke -> Bronc -> Dysp <- Either. Blocked by collider Dysp.
        // So {} should satisfy backdoor.
        assert!(scm.satisfies_backdoor("Lung", "Either", &[]).unwrap());
        
        // Test: is {Smoke} a back-door set for (Lung, Bronc)? Yes, Smoke is a common cause.
        // Backdoor path: Lung <- Smoke -> Bronc. Open if {} given.
        assert!(!scm.satisfies_backdoor("Lung", "Bronc", &[]).unwrap());
        // Blocked if {Smoke} given.
        assert!(scm.satisfies_backdoor("Lung", "Bronc", &["Smoke"]).unwrap());
    }

    #[test]
    fn test_c_components() {
        let mut scm = build_simple_scm();
        scm.mark_hidden_confounder("X", "Y").unwrap();
        let components = scm.c_components();
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 2);
    }

    #[test]
    fn test_causal_identification() {
        let scm = build_asia_causal();
        let res = scm.identify(&["Lung"], &["Smoke"]).unwrap();
        match res {
            IdentificationResult::Identifiable(formula) => {
                assert!(formula.formula.contains("P(Lung | Smoke)"));
            },
            _ => panic!("Should be identifiable"),
        }
    }

    #[test]
    fn test_identification_formula_evaluate() {
        let scm = build_asia_causal();
        let res = scm.identify(&["Lung"], &["Smoke"]).unwrap();
        match res {
            IdentificationResult::Identifiable(formula) => {
                let evaluated = formula.evaluate(&scm, "true").unwrap();
                assert!(evaluated.contains_key("Lung"));
                let p = evaluated["Lung"];
                // P(Lung=true | do(Smoke=true)) should be ~0.10 from Asia network CPT
                assert!((p - 0.10).abs() < 0.02, "P(Lung=true | do(Smoke=true)) should be ~0.10, got {}", p);
            },
            _ => panic!("Should be identifiable"),
        }
    }

    #[test]
    fn test_frontdoor_criterion() {
        // Simple mediation: X -> M -> Y
        let mut bn = BayesianNetwork::new();
        bn.add_variable("X", Domain::binary()).unwrap();
        bn.add_variable("M", Domain::binary()).unwrap();
        bn.add_variable("Y", Domain::binary()).unwrap();
        bn.add_edge("X", "M").unwrap();
        bn.add_edge("M", "Y").unwrap();

        let x = bn.variable("X").unwrap().clone();
        let m = bn.variable("M").unwrap().clone();
        let y = bn.variable("Y").unwrap().clone();

        bn.set_cpd("X", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&x, &[], vec![vec![0.5], vec![0.5]]).unwrap()).unwrap();
        bn.set_cpd("M", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&m, &[&x], vec![vec![0.8, 0.2], vec![0.2, 0.8]]).unwrap()).unwrap();
        bn.set_cpd("Y", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&y, &[&m], vec![vec![0.9, 0.1], vec![0.1, 0.9]]).unwrap()).unwrap();

        let scm = CausalModel::new(bn);
        assert!(scm.satisfies_frontdoor("X", "Y", &["M"]).unwrap());
    }

    #[test]
    fn test_frontdoor_adjustment_computation() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("X", Domain::binary()).unwrap();
        bn.add_variable("M", Domain::binary()).unwrap();
        bn.add_variable("Y", Domain::binary()).unwrap();
        bn.add_edge("X", "M").unwrap();
        bn.add_edge("M", "Y").unwrap();

        let x = bn.variable("X").unwrap().clone();
        let m = bn.variable("M").unwrap().clone();
        let y = bn.variable("Y").unwrap().clone();

        bn.set_cpd("X", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&x, &[], vec![vec![0.5], vec![0.5]]).unwrap()).unwrap();
        bn.set_cpd("M", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&m, &[&x], vec![vec![0.8, 0.2], vec![0.2, 0.8]]).unwrap()).unwrap();
        bn.set_cpd("Y", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&y, &[&m], vec![vec![0.9, 0.1], vec![0.1, 0.9]]).unwrap()).unwrap();

        let scm = CausalModel::new(bn);
        let p = scm.frontdoor_adjustment("X", "true", "Y", "true", "M").unwrap();
        assert!(p >= 0.0 && p <= 1.0);
    }

    #[test]
    fn test_causal_model_guard() {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("X", Domain::binary()).unwrap();
        bn.add_variable("Y", Domain::binary()).unwrap();
        bn.add_edge("X", "Y").unwrap();

        let x = bn.variable("X").unwrap().clone();
        let y = bn.variable("Y").unwrap().clone();
        bn.set_cpd("X", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&x, &[], vec![vec![0.5], vec![0.5]]).unwrap()).unwrap();
        bn.set_cpd("Y", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&y, &[&x], vec![vec![0.9, 0.1], vec![0.1, 0.9]]).unwrap()).unwrap();

        // Build CausalModel from non-causal network (should not happen normally, but guard is in place)
        let scm = CausalModel::new(bn);
        assert!(scm.network.is_causal());
    }

    // --- Pearl Textbook Counterfactual Tests ---

    /// Build a deterministic SCM: X -> Y with Y = X (deterministic)
    fn build_deterministic_scm() -> CausalModel {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("X", Domain::binary()).unwrap();
        bn.add_variable("Y", Domain::binary()).unwrap();
        bn.add_edge("X", "Y").unwrap();

        let x_var = bn.variable("X").unwrap().clone();
        let y_var = bn.variable("Y").unwrap().clone();

        // P(X=0) = 0.5, P(X=1) = 0.5
        bn.set_cpd("X", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(
            &x_var, &[], vec![vec![0.5], vec![0.5]]
        ).unwrap()).unwrap();

        // Y = X deterministically: P(Y=1 | X=1) = 1, P(Y=1 | X=0) = 0
        bn.set_cpd("Y", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(
            &y_var, &[&x_var], vec![vec![1.0, 0.0], vec![0.0, 1.0]]
        ).unwrap()).unwrap();

        CausalModel::new(bn)
    }

    #[test]
    fn test_counterfactual_deterministic() {
        // In a deterministic model Y = X, P(Y=1 | do(X=1), X=0, Y=0) = 1
        let scm = build_deterministic_scm();

        let mut observed = Assignment::new();
        let x_id = scm.network.id_of("X").unwrap();
        let y_id = scm.network.id_of("Y").unwrap();
        observed.set(x_id, "false");
        observed.set(y_id, "false");

        let mut intervention = Assignment::new();
        intervention.set(x_id, "true");

        let result = scm.counterfactual(&observed, &intervention, &["Y"]).unwrap();
        // Check value at index 1 (true) directly from the factor
        let factor = result.distributions.get("Y").unwrap();
        let p_y1 = factor.value_at(1);
        assert!((p_y1 - 1.0).abs() < 1e-10,
            "P(Y=1 | do(X=1), X=0, Y=0) should be 1, got {}", p_y1);
    }

    #[test]
    fn test_probability_of_necessity_deterministic() {
        let scm = build_deterministic_scm();
        let pn = scm.probability_of_necessity("Y", "true", "X", "true", "false").unwrap();
        assert!((pn - 1.0).abs() < 1e-10,
            "PN in deterministic model should be 1, got {}", pn);
    }

    #[test]
    fn test_probability_of_sufficiency_deterministic() {
        let scm = build_deterministic_scm();
        let ps = scm.probability_of_sufficiency("Y", "true", "X", "true", "false").unwrap();
        assert!((ps - 1.0).abs() < 1e-10,
            "PS in deterministic model should be 1, got {}", ps);
    }

    /// Build a noisy-OR SCM: X -> Y with added noise
    fn build_noisy_scm() -> CausalModel {
        let mut bn = BayesianNetwork::new();
        bn.add_variable("X", Domain::binary()).unwrap();
        bn.add_variable("Y", Domain::binary()).unwrap();
        bn.add_edge("X", "Y").unwrap();

        let x_var = bn.variable("X").unwrap().clone();
        let y_var = bn.variable("Y").unwrap().clone();

        bn.set_cpd("X", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(
            &x_var, &[], vec![vec![0.5], vec![0.5]]
        ).unwrap()).unwrap();

        // P(Y=1 | X=1) = 0.9, P(Y=1 | X=0) = 0.1
        bn.set_cpd("Y", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(
            &y_var, &[&x_var], vec![vec![0.9, 0.1], vec![0.1, 0.9]]
        ).unwrap()).unwrap();

        CausalModel::new(bn)
    }

    #[test]
    fn test_counterfactual_noisy() {
        let scm = build_noisy_scm();

        let mut observed = Assignment::new();
        let x_id = scm.network.id_of("X").unwrap();
        observed.set(x_id, "true");

        let mut intervention = Assignment::new();
        intervention.set(x_id, "false");

        let result = scm.counterfactual(&observed, &intervention, &["Y"]).unwrap();
        let factor = result.distributions.get("Y").unwrap();
        let p_y1 = factor.value_at(1); // index 1 = "true"
        // P(Y=1 | do(X=0), X=1) should be close to P(Y=1 | X=0) ≈ 0.1
        assert!((p_y1 - 0.1).abs() < 0.05,
            "P(Y=1 | do(X=0), X=1) should be ~0.1, got {}", p_y1);
    }

    #[test]
    fn test_do_operator_removes_incoming_edges() {
        // Verify that do(X=x) removes all incoming edges to X
        let mut bn = BayesianNetwork::new();
        bn.add_variable("Z", Domain::binary()).unwrap();
        bn.add_variable("X", Domain::binary()).unwrap();
        bn.add_variable("Y", Domain::binary()).unwrap();
        bn.add_edge("Z", "X").unwrap();
        bn.add_edge("X", "Y").unwrap();

        let z = bn.variable("Z").unwrap().clone();
        let x = bn.variable("X").unwrap().clone();
        let y = bn.variable("Y").unwrap().clone();

        bn.set_cpd("Z", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(
            &z, &[], vec![vec![0.5], vec![0.5]]
        ).unwrap()).unwrap();
        bn.set_cpd("X", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(
            &x, &[&z], vec![vec![0.8, 0.2], vec![0.2, 0.8]]
        ).unwrap()).unwrap();
        bn.set_cpd("Y", lutufi_core::core::factor::ConditionalProbabilityTable::from_values(
            &y, &[&x], vec![vec![0.9, 0.1], vec![0.1, 0.9]]
        ).unwrap()).unwrap();

        let scm = CausalModel::new(bn);

        let mut intervention = Assignment::new();
        let x_id = scm.network.id_of("X").unwrap();
        intervention.set(x_id, "true");

        let mutilated = scm.do_operator(&intervention).unwrap();
        // After do(X=true), incoming edge Z -> X should be removed
        assert_eq!(mutilated.graph.parents(&x_id).len(), 0,
            "After do(X), X should have no parents");
        // Outgoing edge X -> Y should remain
        assert_eq!(mutilated.graph.children(&x_id).len(), 1,
            "After do(X), X should still have children");
    }

    #[test]
    fn test_fci_result() {
        use lutufi_core::core::learning::ConstraintBasedLearner;
        use std::collections::HashMap;

        // Simple data: X independent of Y given Z
        let mut data = Vec::new();
        for _ in 0..100 {
            let mut row = HashMap::new();
            row.insert("X".to_string(), if rand::random::<f64>() < 0.5 { "0".to_string() } else { "1".to_string() });
            row.insert("Y".to_string(), if rand::random::<f64>() < 0.5 { "0".to_string() } else { "1".to_string() });
            row.insert("Z".to_string(), if rand::random::<f64>() < 0.5 { "0".to_string() } else { "1".to_string() });
            data.push(row);
        }

        let options = lutufi_core::core::learning::ConstraintBasedOptions::default();
        let learner = ConstraintBasedLearner::new(options);
        let result = learner.fci_algorithm(&data).unwrap();
        assert!(result.network.nodes().len() == 3);
    }
}
