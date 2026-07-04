#[cfg(test)]
mod tests {
    use super::super::types::*;
    use crate::core::domain::Domain;
    use crate::core::variable::Variable;
    use crate::core::factor::ConditionalProbabilityTable;
    use crate::core::models::bayesian_network::BayesianNetwork;

    fn create_test_network() -> BayesianNetwork {
        let mut network = BayesianNetwork::new();
        network.add_variable("A", Domain::binary()).unwrap();
        network.add_variable("B", Domain::binary()).unwrap();
        network.add_edge("A", "B").unwrap();
        let a_var = network.variable("A").unwrap().clone();
        let b_var = network.variable("B").unwrap().clone();
        let cpt_a = ConditionalProbabilityTable::from_values(
            &a_var,
            &[] as &[&Variable],
            vec![vec![0.5], vec![0.5]],
        )
        .unwrap();
        network.set_cpd("A", cpt_a).unwrap();
        let cpt_b = ConditionalProbabilityTable::from_values(
            &b_var,
            &[&a_var],
            vec![vec![0.9, 0.2], vec![0.1, 0.8]],
        )
        .unwrap();
        network.set_cpd("B", cpt_b).unwrap();
        network
    }

    #[test]
    fn test_roundtrip_keeps_structure() {
        let original = create_test_network();
        let json = serde_json::to_string_pretty(
            &LmfDocument::from_bayesian_network(&original).unwrap(),
        )
        .unwrap();
        let loaded = BayesianNetwork::load_lmf_from_str(&json).unwrap();
        assert_eq!(original.nodes().len(), loaded.nodes().len());
        assert_eq!(original.edges().len(), loaded.edges().len());
    }

    #[test]
    fn test_roundtrip_preserves_cpds() {
        let original = create_test_network();
        let json = serde_json::to_string_pretty(
            &LmfDocument::from_bayesian_network(&original).unwrap(),
        )
        .unwrap();
        let loaded = BayesianNetwork::load_lmf_from_str(&json).unwrap();
        for name in original.nodes() {
            let orig_cpd = original.cpd(name).unwrap();
            let loaded_cpd = loaded.cpd(name).unwrap();
            let orig_factor = orig_cpd.as_factor();
            let loaded_factor = loaded_cpd.as_factor();
            let n = orig_factor.scope().num_entries();
            assert_eq!(n, loaded_factor.scope().num_entries());
            let mut orig_vals: Vec<f64> = (0..n).map(|i| orig_factor.value_at(i)).collect();
            let mut loaded_vals: Vec<f64> = (0..n).map(|i| loaded_factor.value_at(i)).collect();
            orig_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            loaded_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for (o, l) in orig_vals.iter().zip(loaded_vals.iter()) {
                let diff = (o - l).abs();
                assert!(
                    diff < 1e-12,
                    "Mismatch for '{}': {} vs {}",
                    name,
                    o,
                    l
                );
            }
        }
    }

    #[test]
    fn test_migration_0_9_to_1_0() {
        let old_json = r#"{
            "format_version": "0.9.0",
            "metadata": {
                "name": "test",
                "lutufi_version": "0.1.0-dev",
                "created_at": "2026-01-01T00:00:00Z",
                "author": null,
                "description": null
            },
            "model_type": "BayesianNetwork",
            "graph": {
                "variables": [
                    {"name": "X", "domain": {"type": "Binary"}}
                ],
                "edges": []
            },
            "parameters": {
                "cpds": [
                    {
                        "child": "X",
                        "parents": [],
                        "table": [0.5, 0.5],
                        "log_space": false
                    }
                ]
            },
            "evidence": null,
            "inference_settings": null,
            "results": null
        }"#;
        let doc = BayesianNetwork::load_lmf_from_str(old_json).unwrap();
        assert_eq!(doc.nodes().len(), 1);
        assert_eq!(doc.nodes()[0], "X");
    }

    #[test]
    fn test_rejects_future_version() {
        let future_json = r#"{
            "format_version": "99.0.0",
            "metadata": {
                "name": "future",
                "lutufi_version": "99.0.0",
                "created_at": "2026-01-01T00:00:00Z",
                "author": null,
                "description": null
            },
            "model_type": "BayesianNetwork",
            "graph": {
                "variables": [
                    {"name": "X", "domain": {"type": "Binary"}}
                ],
                "edges": []
            },
            "parameters": {"cpds": []},
            "evidence": null,
            "inference_settings": null,
            "results": null
        }"#;
        let result = BayesianNetwork::load_lmf_from_str(future_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_variables_rejected() {
        let empty_json = r#"{
            "format_version": "1.0.0",
            "metadata": {
                "name": "empty",
                "lutufi_version": "0.1.0",
                "created_at": "2026-01-01T00:00:00Z",
                "author": null,
                "description": null
            },
            "model_type": "BayesianNetwork",
            "graph": {"variables": [], "edges": []},
            "parameters": {"cpds": []},
            "evidence": null,
            "inference_settings": null,
            "results": null
        }"#;
        assert!(BayesianNetwork::load_lmf_from_str(empty_json).is_err());
    }

    #[test]
    fn test_save_load_file_roundtrip() {
        let original = create_test_network();
        let tmp = std::env::temp_dir().join("test_roundtrip.lmf");
        original.save_lmf(&tmp).unwrap();
        let loaded = BayesianNetwork::load_lmf(&tmp).unwrap();
        let _ = std::fs::remove_file(&tmp);
        assert_eq!(original.nodes().len(), loaded.nodes().len());
        assert_eq!(original.edges().len(), loaded.edges().len());
    }

    #[test]
    fn test_verify_passes_without_results() {
        let network = create_test_network();
        let doc = LmfDocument::from_bayesian_network(&network).unwrap();
        let report = doc.verify(&network).unwrap();
        assert!(report.passed);
        assert_eq!(report.checks.len(), 1);
    }

    #[test]
    fn test_discrete_domain_roundtrip() {
        let mut network = BayesianNetwork::new();
        network
            .add_variable(
                "Color",
                Domain::discrete(vec!["red", "green", "blue"]).unwrap(),
            )
            .unwrap();
        let var = network.variable("Color").unwrap().clone();
        let cpt = ConditionalProbabilityTable::from_values(
            &var,
            &[] as &[&Variable],
            vec![vec![0.3], vec![0.4], vec![0.3]],
        )
        .unwrap();
        network.set_cpd("Color", cpt).unwrap();
        let doc = LmfDocument::from_bayesian_network(&network).unwrap();
        let lmf_domain = &doc.graph.variables[0].domain;
        assert_eq!(
            lmf_domain,
            &LmfDomain::Discrete {
                states: vec!["red".to_string(), "green".to_string(), "blue".to_string()]
            }
        );
        let reloaded = doc.to_bayesian_network().unwrap();
        let rvar = reloaded.variable("Color").unwrap();
        match rvar.domain() {
            Domain::Discrete { states } => {
                assert_eq!(states.len(), 3);
                assert_eq!(states[0], "red");
            }
            _ => panic!("Expected Discrete domain"),
        }
    }

    #[test]
    fn test_wrong_cpd_size_rejected() {
        let json = r#"{
            "format_version": "1.0.0",
            "metadata": {
                "name": "bad",
                "lutufi_version": "0.1.0",
                "created_at": "2026-01-01T00:00:00Z",
                "author": null,
                "description": null
            },
            "model_type": "BayesianNetwork",
            "graph": {
                "variables": [
                    {"name": "X", "domain": {"type": "Binary"}}
                ],
                "edges": []
            },
            "parameters": {
                "cpds": [
                    {
                        "child": "X",
                        "parents": [],
                        "table": [0.5, 0.5, 0.5],
                        "log_space": false
                    }
                ]
            },
            "evidence": null,
            "inference_settings": null,
            "results": null
        }"#;
        assert!(BayesianNetwork::load_lmf_from_str(json).is_err());
    }
}
