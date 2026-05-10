use std::collections::HashMap;
use lutufi_core::core::{
    models::bayesian_network::BayesianNetwork,
    domain::Domain,
    learning::parameter::{ParameterLearner, ParameterLearningOptions, SmoothingMethod},
};

#[test]
fn test_parameter_learner_complete_data() -> Result<(), Box<dyn std::error::Error>> {
    let mut model = BayesianNetwork::builder()
        .add_variable("A", Domain::binary())?
        .add_variable("B", Domain::binary())?
        .add_edge("A", "B")?
        .build()?;

    let mut data = Vec::new();
    // 70% A=0, 30% A=1
    // If A=0, 90% B=0
    // If A=1, 20% B=0
    for _ in 0..700 {
        let mut row = HashMap::new();
        row.insert("A".to_string(), "0".to_string());
        row.insert("B".to_string(), if rand::random::<f64>() < 0.9 { "0".to_string() } else { "1".to_string() });
        data.push(row);
    }
    for _ in 0..300 {
        let mut row = HashMap::new();
        row.insert("A".to_string(), "1".to_string());
        row.insert("B".to_string(), if rand::random::<f64>() < 0.2 { "0".to_string() } else { "1".to_string() });
        data.push(row);
    }

    let options = ParameterLearningOptions {
        smoothing: SmoothingMethod::Laplace,
        alpha: 1.0,
        ..Default::default()
    };

    ParameterLearner::estimate_all(&mut model, &data, options)?;

    let cpd_a = model.cpd("A")?;
    let prob_a = cpd_a.probabilities(&[]);
    // MLE: (700+1)/(1000+2) approx 0.686
    assert!((prob_a[0] - 0.7).abs() < 0.05);

    let cpd_b = model.cpd("B")?;
    // For A=0: (630 approx + 1) / (700 approx + 2)
    assert!((cpd_b.probabilities(&["0"])[0] - 0.9).abs() < 0.05);

    Ok(())
}
