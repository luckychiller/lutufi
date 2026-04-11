use lutufi_core::core::{
    factor::{Factor, TabularFactor, Scope, log_sum_exp},
    variable::Variable,
    domain::Domain,
    assignment::Assignment,
};

#[test]
fn test_log_sum_exp_stability() {
    // Test that log_sum_exp handles very small probabilities (very negative log values)
    let a = -1000.0;
    let b = -1001.0;
    let res = log_sum_exp(a, b);
    
    // Naive sum: exp(-1000) + exp(-1001) would be 0.0 + 0.0 = 0.0
    // log(0.0) = -inf
    
    assert!(res > -1000.0);
    assert!(res < -999.0);
    
    // Exact value should be -1000 + ln(1 + exp(-1))
    let expected = -1000.0 + (1.0 + (-1.0f64).exp()).ln();
    assert!((res - expected).abs() < 1e-10);
}

#[test]
fn test_factor_underflow_prevention() {
    // Create variables
    let domain = Domain::discrete(vec!["0", "1"]).unwrap();
    let v1 = Variable::new("V1", domain);
    let scope = Scope::new(vec![&v1]);
    
    // Very small probabilities that would underflow if multiplied many times
    // p = 1e-100 => log_p = -230.2585
    let log_p = -230.2585092994046; 
    let f1 = TabularFactor::from_log_values(scope.clone(), vec![log_p, log_p]).unwrap();
    
    let mut product: Box<dyn Factor> = Box::new(f1.clone());
    
    // Multiply 10 times: (1e-100)^10 = 1e-1000 (underflows f64)
    // In log space: -230.2585 * 10 = -2302.585 (doesn't underflow)
    for _ in 0..9 {
        product = product.multiply(&f1).unwrap();
    }
    
    assert!((product.log_value_at(0) - log_p * 10.0).abs() < 1e-10);
    assert!(product.log_value_at(0).exp() == 0.0); // It still underflows when we call exp()
    // But the log value is preserved!
}

#[test]
fn test_marginalize_log_sum_exp() {
    let domain = Domain::discrete(vec!["0", "1"]).unwrap();
    let v1 = Variable::new("V1", domain.clone());
    let v2 = Variable::new("V2", domain);
    let scope = Scope::new(vec![&v1, &v2]);
    
    // Two very small but different probabilities
    let log_p1 = -500.0;
    let log_p2 = -501.0;
    
    // Ensure values match (V1, V2) order regardless of UUID order
    let mut values = vec![0.0; 4];
    for i in 0..2 { // V1
        for j in 0..2 { // V2
            let mut assignment = Assignment::new();
            assignment.set(v1.id(), i.to_string());
            assignment.set(v2.id(), j.to_string());
            
            let mut flat = 0;
            let mut stride = 1;
            for k in (0..scope.len()).rev() {
                let var_id = scope.variable_ids()[k];
                let val = assignment.get_discrete(&var_id).unwrap();
                flat += val * stride;
                stride *= scope.sizes()[k];
            }
            
            values[flat] = if j == 0 { log_p1 } else { log_p2 };
        }
    }
    
    let f = TabularFactor::from_log_values(scope, values).unwrap();
    
    // Marginalize out V2
    let marginalized = f.marginalize(&[v2.id()]).unwrap();
    
    // The result for V1=0 should be log_sum_exp(log_p1, log_p2)
    let expected = log_sum_exp(log_p1, log_p2);
    assert!((marginalized.log_value_at(0) - expected).abs() < 1e-10);
}
