use rand::prelude::*;
use lutufi_core::core::{
    domain::Domain,
    error::LutufiResult,
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    variable::Variable,
};

pub fn generate_1m_edge_benchmark_network(
    num_nodes: usize,
    seed: u64,
) -> LutufiResult<BayesianNetwork> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut bn = BayesianNetwork::new();

    for i in 0..num_nodes {
        let name = format!("X{}", i);
        bn.add_variable(&name, Domain::binary())?;
    }

    let target_edges = (num_nodes as f64 * 10.0) as usize;
    let mut edge_count = 0;
    let mut attempts = 0;

    while edge_count < target_edges && attempts < target_edges * 3 {
        attempts += 1;
        let from = rng.gen_range(0..num_nodes);
        let to = rng.gen_range(0..num_nodes);

        if from == to { continue; }

        let from_name = format!("X{}", from);
        let to_name = format!("X{}", to);

        if bn.add_edge(&from_name, &to_name).is_ok() {
            edge_count += 1;
        }
    }

    let node_names: Vec<String> = (0..num_nodes)
        .map(|i| format!("X{}", i))
        .collect();

    for name in &node_names {
        if let Err(_) = bn.cpd(name) {
            let var = bn.variable(name)?;
            let parents = bn.graph.parents(&var.id());
            let parent_vars: Vec<&Variable> = parents.iter()
                .filter_map(|id| bn.variables().get(id))
                .collect();

            if parent_vars.is_empty() {
                let p = rng.gen::<f64>();
                let values = vec![vec![p], vec![1.0 - p]];
                let cpd = ConditionalProbabilityTable::from_values(var, &[], values)?;
                bn.set_cpd(name, cpd)?;
            } else {
                let num_parent_configs: usize = parent_vars.iter()
                    .map(|v| v.domain().size().unwrap_or(2))
                    .product();
                let mut values = vec![vec![0.0; num_parent_configs]; 2];
                for col in 0..num_parent_configs {
                    let p = rng.gen::<f64>() * 0.8 + 0.1;
                    values[0][col] = p;
                    values[1][col] = 1.0 - p;
                }
                let cpd = ConditionalProbabilityTable::from_values(
                    var, &parent_vars, values,
                )?;
                bn.set_cpd(&name, cpd)?;
            }
        }
    }

    Ok(bn)
}

pub fn generate_sparse_benchmark_network(
    num_nodes: usize,
    avg_parents: usize,
    seed: u64,
) -> LutufiResult<BayesianNetwork> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut bn = BayesianNetwork::new();

    for i in 0..num_nodes {
        let name = format!("V{}", i);
        bn.add_variable(&name, Domain::binary())?;
    }

    for i in 1..num_nodes {
        let child = format!("V{}", i);
        let num_parents = rng.gen_range(1..=avg_parents.min(i));

        let mut parent_indices: Vec<usize> = (0..i).collect();
        parent_indices.shuffle(&mut rng);

        for &p_idx in parent_indices.iter().take(num_parents) {
            let parent = format!("V{}", p_idx);
            let _ = bn.add_edge(&parent, &child);
        }
    }

    for i in 0..num_nodes {
        let name = format!("V{}", i);
        if let Err(_) = bn.cpd(&name) {
            let var = bn.variable(&name)?;
            let parents = bn.graph.parents(&var.id());
            let parent_vars: Vec<&Variable> = parents.iter()
                .filter_map(|id| bn.variables().get(id))
                .collect();

            if parent_vars.is_empty() {
                let p = rng.gen::<f64>() * 0.5 + 0.25;
                let cpd = ConditionalProbabilityTable::from_values(
                    var, &[], vec![vec![p], vec![1.0 - p]],
                )?;
                bn.set_cpd(&name, cpd)?;
            } else {
                let num_parent_configs: usize = parent_vars.iter()
                    .map(|v| v.domain().size().unwrap_or(2))
                    .product();
                let mut values = vec![vec![0.0; num_parent_configs]; 2];
                for col in 0..num_parent_configs {
                    let p = rng.gen::<f64>() * 0.6 + 0.2;
                    values[0][col] = p;
                    values[1][col] = 1.0 - p;
                }
                let cpd = ConditionalProbabilityTable::from_values(
                    var, &parent_vars, values,
                )?;
                bn.set_cpd(&name, cpd)?;
            }
        }
    }

    Ok(bn)
}

pub fn memory_stress_test(network: &BayesianNetwork) -> LutufiResult<String> {
    let n_nodes = network.nodes().len();
    let n_edges = network.edges().len();
    let mut total_cpt_entries: usize = 0;

    for cpd in network.cpd_iter() {
        total_cpt_entries += cpd.as_factor().scope().num_entries();
    }

    let estimated_memory_bytes = total_cpt_entries * std::mem::size_of::<f64>()
        + n_nodes * 100
        + n_edges * 16;

    let report = format!(
        "Network: {} nodes, {} edges, {} total CPT entries\nEstimated memory: {:.2} MB",
        n_nodes,
        n_edges,
        total_cpt_entries,
        estimated_memory_bytes as f64 / (1024.0 * 1024.0),
    );

    Ok(report)
}

#[test]
fn test_generate_sparse_network() {
    let bn = generate_sparse_benchmark_network(50, 3, 42).unwrap();
    assert!(bn.nodes().len() <= 50);
    assert!(bn.is_valid());
    assert_eq!(bn.nodes().len(), 50);
}

#[test]
fn test_memory_stress_report() {
    let bn = generate_sparse_benchmark_network(20, 2, 42).unwrap();
    let report = memory_stress_test(&bn).unwrap();
    assert!(report.contains("nodes"));
    assert!(report.contains("edges"));
    assert!(report.contains("CPT entries"));
}

#[test]
fn test_1m_edge_benchmark_constructs() {
    let bn = generate_1m_edge_benchmark_network(20, 42).unwrap();
    assert!(bn.nodes().len() == 20);
    assert!(bn.edges().len() > 0);
}
