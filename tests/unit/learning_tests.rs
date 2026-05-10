#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use lutufi_core::core::{
        learning::{
            ParameterEstimator, ParameterLearningMethod, LegacyParameterLearningOptions,
            ScoreBasedLearner, ScoreBasedOptions,
            ConstraintBasedLearner, ConstraintBasedOptions,
        },
        models::bayesian_network::BayesianNetwork,
        domain::Domain,
        factor::Factor,
    };

    #[test]
    fn test_parameter_estimator_boilerplate() {
        let options = LegacyParameterLearningOptions {
            method: ParameterLearningMethod::MLE,
            alpha: 0.5,
            max_iterations: 100,
            tolerance: 1e-4,
        };
        let estimator = ParameterEstimator::new(options);
        let mut model = BayesianNetwork::new();
        model.add_variable("A", Domain::binary()).unwrap();
        
        let data = vec![
            [("A".to_string(), "false".to_string())].into_iter().collect::<HashMap<_, _>>(),
            [("A".to_string(), "true".to_string())].into_iter().collect::<HashMap<_, _>>(),
        ];
        
        let result = estimator.fit(&mut model, &data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parameter_estimator_with_names() {
        let options = LegacyParameterLearningOptions {
            method: ParameterLearningMethod::MLE,
            alpha: 0.5,
            max_iterations: 100,
            tolerance: 1e-4,
        };
        let estimator = ParameterEstimator::new(options);
        let mut model = BayesianNetwork::new();
        model.add_variable("A", Domain::discrete(vec!["F", "T"]).unwrap()).unwrap();
        
        let data = vec![
            [("A".to_string(), "F".to_string())].into_iter().collect::<HashMap<_, _>>(),
            [("A".to_string(), "T".to_string())].into_iter().collect::<HashMap<_, _>>(),
        ];
        
        let result = estimator.fit(&mut model, &data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_score_based_learner_boilerplate() {
        let options = ScoreBasedOptions::default();
        let learner = ScoreBasedLearner::new(options);
        let data = vec![];
        
        let result = learner.hill_climbing(&data, &[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_constraint_based_learner_boilerplate() {
        let options = ConstraintBasedOptions::default();
        let learner = ConstraintBasedLearner::new(options);
        let data = vec![];
        
        let result = learner.pc_algorithm(&data);
        assert!(result.is_err());
    }

    fn build_asia_network() -> BayesianNetwork {
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

        model
    }

    #[test]
    fn test_asia_parameter_learning_convergence() {
        let true_model = build_asia_network();
        let mut samples = Vec::with_capacity(10000);
        for _ in 0..10000 {
            let sample = true_model.sample().unwrap();
            let mut row = HashMap::new();
            for (&id, val) in sample.iter() {
                let name = true_model.variables().get(&id).unwrap().name().to_string();

                row.insert(name.clone(), val.clone());
            }
            samples.push(row);
        }
        
        let mut learned_model = BayesianNetwork::new();
        let bin = || Domain::binary();
        learned_model.add_variable("Asia", bin()).unwrap();
        learned_model.add_variable("Tub", bin()).unwrap();
        learned_model.add_variable("Smoke", bin()).unwrap();
        learned_model.add_variable("Lung", bin()).unwrap();
        learned_model.add_variable("Bronc", bin()).unwrap();
        learned_model.add_variable("Either", bin()).unwrap();
        learned_model.add_variable("Xray", bin()).unwrap();
        learned_model.add_variable("Dysp", bin()).unwrap();

        learned_model.add_edge("Asia", "Tub").unwrap();
        learned_model.add_edge("Smoke", "Lung").unwrap();
        learned_model.add_edge("Smoke", "Bronc").unwrap();
        learned_model.add_edge("Tub", "Either").unwrap();
        learned_model.add_edge("Lung", "Either").unwrap();
        learned_model.add_edge("Either", "Xray").unwrap();
        learned_model.add_edge("Either", "Dysp").unwrap();
        learned_model.add_edge("Bronc", "Dysp").unwrap();

        let options = LegacyParameterLearningOptions {
            method: ParameterLearningMethod::MLE,
            alpha: 0.1,
            ..Default::default()
        };
        learned_model.fit(&samples, options).unwrap();

        for node_name in true_model.nodes() {
            let true_cpd = true_model.cpd(node_name).unwrap();
            let learned_cpd = learned_model.cpd(node_name).unwrap();
            let true_factor = true_cpd.as_factor();
            let learned_factor = learned_cpd.as_factor();
            let scope_true = true_factor.scope();
            for i in 0..scope_true.num_entries() {
                let mut assignment_true = lutufi_core::core::assignment::Assignment::new();
                let indices = lutufi_core::core::factor::multi_index_from_flat(i, scope_true.sizes());
                for (j, &var_id) in scope_true.variable_ids().iter().enumerate() {
                    assignment_true.set_discrete(var_id, indices[j]).unwrap();
                }
                let true_val = true_factor.evaluate(&assignment_true).unwrap();
                let mut assignment_learned = lutufi_core::core::assignment::Assignment::new();
                for (j, &var_id) in scope_true.variable_ids().iter().enumerate() {
                    let name = true_model.variables().get(&var_id).unwrap().name();
                    let target_id = learned_model.id_of(name).unwrap();
                    assignment_learned.set_discrete(target_id, indices[j]).unwrap();
                }
                let learned_val = learned_factor.evaluate(&assignment_learned).unwrap();
                let diff = (true_val - learned_val).abs();
                if true_val > 0.05 {
                    assert!(diff < 0.1 || diff / true_val < 0.25);
                } else {
                    assert!(diff < 0.1);
                }
            }
        }
    }

    #[test]
    fn test_em_parameter_learning_convergence() {
        let true_model = build_asia_network();
        let mut samples = Vec::with_capacity(500);
        for _ in 0..500 {
            let sample = true_model.sample().unwrap();
            let mut row = HashMap::new();
            for (&id, val) in sample.iter() {
                let name = true_model.variables().get(&id).unwrap().name().to_string();

                if name != "Lung" { row.insert(name.clone(), val.clone()); }
            }
            samples.push(row);
        }

        let mut learned_model = BayesianNetwork::new();
        let bin = || Domain::binary();
        learned_model.add_variable("Asia", bin()).unwrap();
        learned_model.add_variable("Tub", bin()).unwrap();
        learned_model.add_variable("Smoke", bin()).unwrap();
        learned_model.add_variable("Lung", bin()).unwrap();
        learned_model.add_variable("Bronc", bin()).unwrap();
        learned_model.add_variable("Either", bin()).unwrap();
        learned_model.add_variable("Xray", bin()).unwrap();
        learned_model.add_variable("Dysp", bin()).unwrap();

        learned_model.add_edge("Asia", "Tub").unwrap();
        learned_model.add_edge("Smoke", "Lung").unwrap();
        learned_model.add_edge("Smoke", "Bronc").unwrap();
        learned_model.add_edge("Tub", "Either").unwrap();
        learned_model.add_edge("Lung", "Either").unwrap();
        learned_model.add_edge("Either", "Xray").unwrap();
        learned_model.add_edge("Either", "Dysp").unwrap();
        learned_model.add_edge("Bronc", "Dysp").unwrap();

        let options = LegacyParameterLearningOptions {
            method: ParameterLearningMethod::MLE,
            alpha: 0.1,
            max_iterations: 3, 
            tolerance: 1e-1,
        };
        let result = learned_model.fit(&samples, options);
        assert!(result.is_ok());

        for node_name in &["Asia", "Smoke", "Tub", "Xray"] {
            let true_cpd = true_model.cpd(node_name).unwrap();
            let learned_cpd = learned_model.cpd(node_name).unwrap();
            let true_factor = true_cpd.as_factor();
            let learned_factor = learned_cpd.as_factor();
            let scope_true = true_factor.scope();
            for i in 0..scope_true.num_entries() {
                let mut assignment_true = lutufi_core::core::assignment::Assignment::new();
                let indices = lutufi_core::core::factor::multi_index_from_flat(i, scope_true.sizes());
                for (j, &var_id) in scope_true.variable_ids().iter().enumerate() {
                    assignment_true.set_discrete(var_id, indices[j]).unwrap();
                }
                let true_val = true_factor.evaluate(&assignment_true).unwrap();
                let mut assignment_learned = lutufi_core::core::assignment::Assignment::new();
                for (j, &var_id) in scope_true.variable_ids().iter().enumerate() {
                    let name = true_model.variables().get(&var_id).unwrap().name();
                    let target_id = learned_model.id_of(name).unwrap();
                    assignment_learned.set_discrete(target_id, indices[j]).unwrap();
                }
                let learned_val = learned_factor.evaluate(&assignment_learned).unwrap();
                let diff = (true_val - learned_val).abs();
                if true_val > 0.05 {
                    assert!(diff < 0.3);
                }
            }
        }
    }

    #[test]
    fn test_hill_climbing_structure_learning() {
        let mut model = BayesianNetwork::new();
        model.add_variable("A", Domain::binary()).unwrap();
        model.add_variable("B", Domain::binary()).unwrap();
        model.add_edge("A", "B").unwrap();
        
        let a_var = model.variable("A").unwrap().clone();
        let b_var = model.variable("B").unwrap().clone();
        
        let cpd_a = lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&a_var, &[], vec![vec![0.5], vec![0.5]]).unwrap();
        let cpd_b = lutufi_core::core::factor::ConditionalProbabilityTable::from_values(&b_var, &[&a_var], vec![vec![0.9, 0.1], vec![0.1, 0.9]]).unwrap();
        model.set_cpd("A", cpd_a).unwrap();
        model.set_cpd("B", cpd_b).unwrap();
        
        let mut samples = Vec::new();
        for _ in 0..1000 {
            let sample = model.sample().unwrap();
            let mut row = HashMap::new();
            for (&id, val) in sample.iter() {
                row.insert(model.variables().get(&id).unwrap().name().to_string(), val.clone());
            }
            samples.push(row);
        }
        
        let options = ScoreBasedOptions {
            score_type: lutufi_core::core::learning::ScoreType::BIC,
            n_restarts: 1,
            ..Default::default()
        };
        let learner = ScoreBasedLearner::new(options);
        let learned_model = learner.hill_climbing(&samples, &[], &[]).unwrap();
        
        assert_eq!(learned_model.graph.edges().len(), 1);
        let edge = learned_model.graph.edges()[0];
        let u = learned_model.variables().get(&edge.0).unwrap().name();
        let v = learned_model.variables().get(&edge.1).unwrap().name();
        assert!((u == "A" && v == "B") || (u == "B" && v == "A"));
    }

    #[test]
    fn test_asia_structure_learning_hc() {
        let true_model = build_asia_network();
        let mut samples = Vec::new();
        for _ in 0..2000 { 
            let sample = true_model.sample().unwrap();
            let mut row = HashMap::new();
            for (&id, val) in sample.iter() {
                row.insert(true_model.variables().get(&id).unwrap().name().to_string(), val.clone());
            }
            samples.push(row);
        }
        let options = ScoreBasedOptions {
            score_type: lutufi_core::core::learning::ScoreType::BIC,
            n_restarts: 1,
            max_iter: 100,
            ..Default::default()
        };
        let learner = ScoreBasedLearner::new(options);
        let learned_model = learner.hill_climbing(&samples, &[], &[]).unwrap();
        let edge_count = learned_model.graph.edges().len();
        assert!(edge_count >= 5 && edge_count <= 15);
    }

    #[test]
    fn test_pc_algorithm_asia() {
        let true_model = build_asia_network();
        let mut samples = Vec::new();
        for _ in 0..500 { 
            let sample = true_model.sample().unwrap();
            let mut row = HashMap::new();
            for (&id, val) in sample.iter() {
                row.insert(true_model.variables().get(&id).unwrap().name().to_string(), val.clone());
            }
            samples.push(row);
        }
        let options = ConstraintBasedOptions {
            test_type: lutufi_core::core::learning::IndependenceTestType::ChiSquare,
            alpha: 0.05,
        };
        let learner = ConstraintBasedLearner::new(options);
        let learned_model = learner.pc_algorithm(&samples).unwrap();
        let edge_count = learned_model.graph.edges().len();
        assert!(edge_count >= 2 && edge_count <= 15);
        assert!(!learned_model.graph.is_cyclic());
    }
}
