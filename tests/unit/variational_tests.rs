use lutufi_core::core::{
    assignment::Assignment,
    domain::Domain,
    factor::ConditionalProbabilityTable,
    models::{bayesian_network::BayesianNetwork, factor_graph::FactorGraph},
    inference::variational::{VariationalEngine, VariationalOptions},
    inference::junction_tree::JunctionTreeEngine,
};
use approx::assert_abs_diff_eq;

fn build_asia_network() -> BayesianNetwork {
    let mut bn = BayesianNetwork::new();

    bn.add_variable("Asia",       Domain::binary()).unwrap();
    bn.add_variable("Tuberculosis", Domain::binary()).unwrap();
    bn.add_variable("Smoking",    Domain::binary()).unwrap();
    bn.add_variable("LungCancer", Domain::binary()).unwrap();
    bn.add_variable("Bronchitis", Domain::binary()).unwrap();
    bn.add_variable("TbOrCa",     Domain::binary()).unwrap();
    bn.add_variable("XRay",       Domain::binary()).unwrap();
    bn.add_variable("Dyspnoea",   Domain::binary()).unwrap();

    bn.add_edge("Asia",         "Tuberculosis").unwrap();
    bn.add_edge("Smoking",      "LungCancer").unwrap();
    bn.add_edge("Smoking",      "Bronchitis").unwrap();
    bn.add_edge("Tuberculosis", "TbOrCa").unwrap();
    bn.add_edge("LungCancer",   "TbOrCa").unwrap();
    bn.add_edge("TbOrCa",       "XRay").unwrap();
    bn.add_edge("TbOrCa",       "Dyspnoea").unwrap();
    bn.add_edge("Bronchitis",   "Dyspnoea").unwrap();

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
fn test_variational_asia() {
    let bn = build_asia_network();
    let fg = FactorGraph::from_bayesian_network(&bn).unwrap();
    let options = VariationalOptions {
        max_iterations: 100,
        tolerance: 1e-6,
        n_restarts: 3,
        seed: Some(42),
    };
    let engine = VariationalEngine::new(fg, options);
    let evidence = Assignment::new();

    let result = engine.run_with_evidence(&evidence).unwrap();
    assert!(result.converged);

    for i in 1..result.elbo_history.len() {
        assert!(result.elbo_history[i] >= result.elbo_history[i-1] - 1e-10);
    }

    let asia_id = bn.id_of("Asia").unwrap();
    let smoking_id = bn.id_of("Smoking").unwrap();

    let jt = JunctionTreeEngine::new(&bn).unwrap();
    
    let jt_asia = jt.query(&["Asia"], &evidence).unwrap();
    let vi_asia = result.marginals.get(&asia_id).unwrap();
    assert_abs_diff_eq!(jt_asia.value_at(1), vi_asia.value_at(1), epsilon = 0.1);

    let jt_smoking = jt.query(&["Smoking"], &evidence).unwrap();
    let vi_smoking = result.marginals.get(&smoking_id).unwrap();
    assert_abs_diff_eq!(jt_smoking.value_at(1), vi_smoking.value_at(1), epsilon = 0.1);
}
