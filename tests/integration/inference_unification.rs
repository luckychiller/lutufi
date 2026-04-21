use lutufi_core::core::{
    assignment::Assignment,
    domain::Domain,
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    inference::{Algorithm, Diagnostics},
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
fn test_unified_query_all_algorithms() {
    let bn = build_asia_network();
    let evidence = Assignment::new();
    let query_vars = vec!["Asia", "Smoking"];

    // 1. Exact
    let res_exact = bn.query(&query_vars, &evidence, Algorithm::Exact).unwrap();
    assert_eq!(res_exact.algorithm_used, Algorithm::Exact);
    assert_abs_diff_eq!(res_exact.distributions["Asia"].value_at(1), 0.01, epsilon = 1e-10);

    // 2. LBP
    let res_lbp = bn.query(&query_vars, &evidence, Algorithm::LBP).unwrap();
    assert_eq!(res_lbp.algorithm_used, Algorithm::LBP);
    assert_abs_diff_eq!(res_lbp.distributions["Asia"].value_at(1), 0.01, epsilon = 0.01);

    // 3. MCMC
    let res_mcmc = bn.query(&query_vars, &evidence, Algorithm::MCMC).unwrap();
    assert_eq!(res_mcmc.algorithm_used, Algorithm::MCMC);
    assert_abs_diff_eq!(res_mcmc.distributions["Asia"].value_at(1), 0.01, epsilon = 0.02);

    // 4. Variational
    let res_vi = bn.query(&query_vars, &evidence, Algorithm::Variational).unwrap();
    assert_eq!(res_vi.algorithm_used, Algorithm::Variational);
    assert_abs_diff_eq!(res_vi.distributions["Asia"].value_at(1), 0.01, epsilon = 0.1);

    // 5. Auto (should select Exact for Asia as it's small)
    let res_auto = bn.query(&query_vars, &evidence, Algorithm::Auto).unwrap();
    assert_eq!(res_auto.algorithm_used, Algorithm::Exact);
}

#[test]
fn test_diagnostics_availability() {
    let bn = build_asia_network();
    let evidence = Assignment::new();

    let res = bn.query(&["Asia"], &evidence, Algorithm::LBP).unwrap();
    match res.diagnostics {
        Diagnostics::LBP { iterations, .. } => assert!(iterations > 0),
        _ => panic!("Expected LBP diagnostics"),
    }

    let res = bn.query(&["Asia"], &evidence, Algorithm::MCMC).unwrap();
    match res.diagnostics {
        Diagnostics::MCMC { .. } => {},
        _ => panic!("Expected MCMC diagnostics"),
    }
}
