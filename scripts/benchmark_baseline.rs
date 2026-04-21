use lutufi_core::core::{
    assignment::Assignment,
    domain::Domain,
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    inference::Algorithm,
};
use std::time::Instant;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use chrono::Local;

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

fn build_grid_network(rows: usize, cols: usize) -> BayesianNetwork {
    let mut bn = BayesianNetwork::new();
    for r in 0..rows {
        for c in 0..cols {
            let name = format!("V_{}_{}", r, c);
            bn.add_variable(&name, Domain::binary()).unwrap();
            
            if r > 0 {
                let parent = format!("V_{}_{}", r - 1, c);
                bn.add_edge(&parent, &name).unwrap();
            }
            if c > 0 {
                let parent = format!("V_{}_{}", r, c - 1);
                bn.add_edge(&parent, &name).unwrap();
            }
        }
    }

    for r in 0..rows {
        for c in 0..cols {
            let name = format!("V_{}_{}", r, c);
            let var = bn.variable(&name).unwrap().clone();
            let parents = bn.graph.parents(&var.id());
            let mut parent_vars = Vec::new();
            for p_id in parents {
                parent_vars.push(bn.variables.get(&p_id).unwrap());
            }
            
            let num_parents = parent_vars.len();
            let num_configs = 1 << num_parents;
            let values = vec![vec![0.5; num_configs], vec![0.5; num_configs]];
            
            bn.set_cpd(&name, ConditionalProbabilityTable::from_values(&var, &parent_vars, values).unwrap()).unwrap();
        }
    }
    bn
}

fn main() {
    let now = Local::now().format("%Y-%m-%d").to_string();
    let networks = vec![
        ("Asia", build_asia_network(), vec!["Asia", "Dyspnoea"]),
        ("Grid_5x5", build_grid_network(5, 5), vec!["V_4_4"]),
    ];

    let algorithms = vec![
        ("Exact", Algorithm::Exact),
        ("LBP", Algorithm::LBP),
        ("MCMC", Algorithm::MCMC),
        ("Variational", Algorithm::Variational),
    ];

    let mut report = json!({
        "timestamp": now,
        "results": {}
    });

    for (net_name, bn, query_vars) in networks {
        println!("Benchmarking {}...", net_name);
        let mut net_results = json!({});
        let evidence = Assignment::new();

        for (alg_name, algo) in &algorithms {
            let start = Instant::now();
            let iterations = if net_name == "Asia" { 100 } else { 10 };
            
            for _ in 0..iterations {
                let _ = bn.query(&query_vars, &evidence, *algo).unwrap();
            }
            
            let avg_duration = start.elapsed().as_secs_f64() / (iterations as f64);
            net_results[alg_name] = json!(avg_duration);
            println!("  {}: {:.6}s", alg_name, avg_duration);
        }
        report["results"][net_name] = net_results;
    }

    let filename = format!("tests/benchmarks/baseline_{}.json", now);
    let mut file = File::create(&filename).unwrap();
    file.write_all(serde_json::to_string_pretty(&report).unwrap().as_bytes()).unwrap();
    println!("Benchmark report saved to {}", filename);
}
