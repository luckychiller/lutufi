#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use lutufi_core::core::{
        assignment::Assignment,
        domain::Domain,
        factor::ConditionalProbabilityTable,
        models::bayesian_network::BayesianNetwork,
        inference::junction_tree::JunctionTreeEngine,
    };

    fn build_asia_network() -> BayesianNetwork {
        let mut bn = BayesianNetwork::new();

        // Add all 8 variables
        bn.add_variable("Asia",       Domain::binary()).unwrap();
        bn.add_variable("Tuberculosis", Domain::binary()).unwrap();
        bn.add_variable("Smoking",    Domain::binary()).unwrap();
        bn.add_variable("LungCancer", Domain::binary()).unwrap();
        bn.add_variable("Bronchitis", Domain::binary()).unwrap();
        bn.add_variable("TbOrCa",     Domain::binary()).unwrap();
        bn.add_variable("XRay",       Domain::binary()).unwrap();
        bn.add_variable("Dyspnoea",   Domain::binary()).unwrap();

        // Add edges
        bn.add_edge("Asia",         "Tuberculosis").unwrap();
        bn.add_edge("Smoking",      "LungCancer").unwrap();
        bn.add_edge("Smoking",      "Bronchitis").unwrap();
        bn.add_edge("Tuberculosis", "TbOrCa").unwrap();
        bn.add_edge("LungCancer",   "TbOrCa").unwrap();
        bn.add_edge("TbOrCa",       "XRay").unwrap();
        bn.add_edge("TbOrCa",       "Dyspnoea").unwrap();
        bn.add_edge("Bronchitis",   "Dyspnoea").unwrap();

        // Set CPTs
        let asia_var = bn.variable("Asia").unwrap().clone();
        bn.set_cpd("Asia", ConditionalProbabilityTable::from_values(&asia_var, &[], vec![vec![0.99], vec![0.01]]).unwrap()).unwrap();

        let smoking_var = bn.variable("Smoking").unwrap().clone();
        bn.set_cpd("Smoking", ConditionalProbabilityTable::from_values(&smoking_var, &[], vec![vec![0.5], vec![0.5]]).unwrap()).unwrap();

        let tb_var = bn.variable("Tuberculosis").unwrap().clone();
        let asia_var2 = bn.variable("Asia").unwrap().clone();
        bn.set_cpd("Tuberculosis", ConditionalProbabilityTable::from_values(&tb_var, &[&asia_var2], vec![vec![0.99, 0.95], vec![0.01, 0.05]]).unwrap()).unwrap();

        let lc_var = bn.variable("LungCancer").unwrap().clone();
        let smoking_var2 = bn.variable("Smoking").unwrap().clone();
        bn.set_cpd("LungCancer", ConditionalProbabilityTable::from_values(&lc_var, &[&smoking_var2], vec![vec![0.99, 0.90], vec![0.01, 0.10]]).unwrap()).unwrap();

        let br_var = bn.variable("Bronchitis").unwrap().clone();
        let smoking_var3 = bn.variable("Smoking").unwrap().clone();
        bn.set_cpd("Bronchitis", ConditionalProbabilityTable::from_values(&br_var, &[&smoking_var3], vec![vec![0.70, 0.40], vec![0.30, 0.60]]).unwrap()).unwrap();

        let tborca_var = bn.variable("TbOrCa").unwrap().clone();
        let tb_var2 = bn.variable("Tuberculosis").unwrap().clone();
        let lc_var2 = bn.variable("LungCancer").unwrap().clone();
        bn.set_cpd("TbOrCa", ConditionalProbabilityTable::from_values(&tborca_var, &[&tb_var2, &lc_var2], vec![vec![1.0, 0.0, 0.0, 0.0], vec![0.0, 1.0, 1.0, 1.0]]).unwrap()).unwrap();

        let xray_var = bn.variable("XRay").unwrap().clone();
        let tborca_var2 = bn.variable("TbOrCa").unwrap().clone();
        bn.set_cpd("XRay", ConditionalProbabilityTable::from_values(&xray_var, &[&tborca_var2], vec![vec![0.95, 0.02], vec![0.05, 0.98]]).unwrap()).unwrap();

        let dy_var = bn.variable("Dyspnoea").unwrap().clone();
        let tborca_var3 = bn.variable("TbOrCa").unwrap().clone();
        let br_var2 = bn.variable("Bronchitis").unwrap().clone();
        bn.set_cpd("Dyspnoea", ConditionalProbabilityTable::from_values(&dy_var, &[&tborca_var3, &br_var2], vec![vec![0.90, 0.20, 0.30, 0.10], vec![0.10, 0.80, 0.70, 0.90]]).unwrap()).unwrap();

        bn
    }

    #[test]
    fn test_jt_asia_no_evidence() {
        let bn = build_asia_network();
        let engine = JunctionTreeEngine::new(&bn).unwrap();
        let evidence = Assignment::new();

        // P(Asia) = [0.99, 0.01]
        let res = engine.query(&["Asia"], &evidence).unwrap();
        assert_abs_diff_eq!(res.value_at(0), 0.99, epsilon = 1e-9);
        assert_abs_diff_eq!(res.value_at(1), 0.01, epsilon = 1e-9);

        // P(Tuberculosis) = [0.9896, 0.0104]
        let res = engine.query(&["Tuberculosis"], &evidence).unwrap();
        assert_abs_diff_eq!(res.value_at(1), 0.0104, epsilon = 1e-9);

        // P(LungCancer) = [0.945, 0.055]
        let res = engine.query(&["LungCancer"], &evidence).unwrap();
        assert_abs_diff_eq!(res.value_at(1), 0.055, epsilon = 1e-9);
    }

    #[test]
    fn test_jt_asia_with_evidence() {
        let bn = build_asia_network();
        let engine = JunctionTreeEngine::new(&bn).unwrap();
        let mut evidence = Assignment::new();
        
        // P(Tuberculosis | Asia=T) = [0.95, 0.05]
        let asia_id = bn.id_of("Asia").unwrap();
        evidence.set(asia_id, "1");
        let res = engine.query(&["Tuberculosis"], &evidence).unwrap();
        assert_abs_diff_eq!(res.value_at(0), 0.95, epsilon = 1e-9);
        assert_abs_diff_eq!(res.value_at(1), 0.05, epsilon = 1e-9);
    }

    #[test]
    fn test_jt_treewidth() {
        let bn = build_asia_network();
        let engine = JunctionTreeEngine::new(&bn).unwrap();
        // Asia network treewidth is 2
        assert_eq!(engine.treewidth(), 2);
    }
}
