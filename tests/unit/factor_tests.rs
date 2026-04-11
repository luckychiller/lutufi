#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use lutufi_core::core::{
        assignment::Assignment,
        domain::Domain,
        factor::{TabularFactor, ConditionalProbabilityTable, Scope, log_sum_exp, Factor},
        variable::Variable,
    };

    fn make_binary_var(name: &str) -> Variable {
        Variable::new(name, Domain::binary())
    }

    fn make_discrete_var(name: &str, states: Vec<&str>) -> Variable {
        Variable::new(name, Domain::discrete(states).unwrap())
    }

    // log_sum_exp tests

    #[test]
    fn log_sum_exp_basic() {
        // log(exp(0) + exp(0)) = log(2)
        let result = log_sum_exp(0.0, 0.0);
        assert_abs_diff_eq!(result, std::f64::consts::LN_2, epsilon = 1e-12);
    }

    #[test]
    fn log_sum_exp_with_neg_inf() {
        // log(0 + exp(1)) = 1
        let result = log_sum_exp(f64::NEG_INFINITY, 1.0);
        assert_abs_diff_eq!(result, 1.0, epsilon = 1e-12);

        let result2 = log_sum_exp(1.0, f64::NEG_INFINITY);
        assert_abs_diff_eq!(result2, 1.0, epsilon = 1e-12);
    }

    // TabularFactor tests

    fn make_simple_factor() -> (Variable, Variable, TabularFactor) {
        let a = make_binary_var("A");
        let b = make_binary_var("B");
        let scope = Scope::new(vec![&a, &b]);
        
        // Ensure values match (A, B) order regardless of UUID order
        let mut values = vec![0.0; 4];
        let a_id = a.id();
        let b_id = b.id();
        
        for i in 0..2 { // A
            for j in 0..2 { // B
                let mut assignment = Assignment::new();
                assignment.set(a_id, i.to_string());
                assignment.set(b_id, j.to_string());
                
                let flat_idx = project_indices_onto_scope(&assignment, &scope);
                // P(A=F, B=F)=0.1, P(A=F, B=T)=0.4, P(A=T, B=F)=0.2, P(A=T, B=T)=0.3
                values[flat_idx] = match (i, j) {
                    (0, 0) => 0.1,
                    (0, 1) => 0.4,
                    (1, 0) => 0.2,
                    (1, 1) => 0.3,
                    _ => unreachable!(),
                };
            }
        }
        
        let factor = TabularFactor::from_values(scope, values).unwrap();
        (a, b, factor)
    }

    fn project_indices_onto_scope(assignment: &Assignment, scope: &Scope) -> usize {
        let mut flat = 0;
        let mut stride = 1;
        for i in (0..scope.len()).rev() {
            let var_id = scope.variable_ids()[i];
            let val = assignment.get_discrete(&var_id).unwrap();
            flat += val * stride;
            stride *= scope.sizes()[i];
        }
        flat
    }

    #[test]
    fn factor_evaluate_known_values() {
        let (a, b, factor) = make_simple_factor();
        
        let mut assign_ff = Assignment::new();
        assign_ff.set(a.id(), "0");
        assign_ff.set(b.id(), "0");
        assert_abs_diff_eq!(factor.evaluate(&assign_ff).unwrap(), 0.1, epsilon = 1e-10);

        let mut assign_ft = Assignment::new();
        assign_ft.set(a.id(), "0");
        assign_ft.set(b.id(), "1");
        assert_abs_diff_eq!(factor.evaluate(&assign_ft).unwrap(), 0.4, epsilon = 1e-10);

        let mut assign_tf = Assignment::new();
        assign_tf.set(a.id(), "1");
        assign_tf.set(b.id(), "0");
        assert_abs_diff_eq!(factor.evaluate(&assign_tf).unwrap(), 0.2, epsilon = 1e-10);

        let mut assign_tt = Assignment::new();
        assign_tt.set(a.id(), "1");
        assign_tt.set(b.id(), "1");
        assert_abs_diff_eq!(factor.evaluate(&assign_tt).unwrap(), 0.3, epsilon = 1e-10);
    }

    #[test]
    fn factor_marginalize_over_b() {
        let (a, b, factor) = make_simple_factor();
        // Marginalizing B: P(A=F) = 0.1 + 0.4 = 0.5, P(A=T) = 0.2 + 0.3 = 0.5
        let marginal = factor.marginalize(&[b.id()]).unwrap();
        assert_eq!(marginal.scope().len(), 1); // Only A remains
        assert_abs_diff_eq!(marginal.log_value_at(0).exp(), 0.5, epsilon = 1e-10); // P(A=F)
        assert_abs_diff_eq!(marginal.log_value_at(1).exp(), 0.5, epsilon = 1e-10); // P(A=T)
    }

    #[test]
    fn factor_marginalize_over_a() {
        let (a, b, factor) = make_simple_factor();
        // Marginalizing A: P(B=F) = 0.1 + 0.2 = 0.3, P(B=T) = 0.4 + 0.3 = 0.7
        let marginal = factor.marginalize(&[a.id()]).unwrap();
        assert_eq!(marginal.scope().len(), 1); // Only B remains
        assert_abs_diff_eq!(marginal.log_value_at(0).exp(), 0.3, epsilon = 1e-10); // P(B=F)
        assert_abs_diff_eq!(marginal.log_value_at(1).exp(), 0.7, epsilon = 1e-10); // P(B=T)
    }

    #[test]
    fn factor_normalize() {
        let a = make_binary_var("A");
        let scope = Scope::new(vec![&a]);
        // Unnormalized: [2.0, 3.0]
        let mut factor = TabularFactor::from_values(scope, vec![2.0, 3.0]).unwrap();
        factor.normalize();
        assert_abs_diff_eq!(factor.value_at(0), 0.4, epsilon = 1e-10);
        assert_abs_diff_eq!(factor.value_at(1), 0.6, epsilon = 1e-10);
    }

    #[test]
    fn factor_multiply_overlapping_scopes() {
        let a = make_binary_var("A");
        let b = make_binary_var("B");

        let scope1 = Scope::new(vec![&a]);
        let f1 = TabularFactor::from_values(scope1, vec![0.3, 0.7]).unwrap();

        let scope2 = Scope::new(vec![&b]);
        let f2 = TabularFactor::from_values(scope2, vec![0.4, 0.6]).unwrap();

        // Product should have scope over both A and B
        let product = f1.multiply(&f2).unwrap();
        assert_eq!(product.scope().len(), 2);
        
        let mut assign_ff = Assignment::new();
        assign_ff.set(a.id(), "0");
        assign_ff.set(b.id(), "0");
        // P(A=F, B=F) = 0.3 * 0.4 = 0.12
        assert_abs_diff_eq!(product.evaluate(&assign_ff).unwrap(), 0.12, epsilon = 1e-10);

        let mut assign_ft = Assignment::new();
        assign_ft.set(a.id(), "0");
        assign_ft.set(b.id(), "1");
        // P(A=F, B=T) = 0.3 * 0.6 = 0.18
        assert_abs_diff_eq!(product.evaluate(&assign_ft).unwrap(), 0.18, epsilon = 1e-10);

        let mut assign_tf = Assignment::new();
        assign_tf.set(a.id(), "1");
        assign_tf.set(b.id(), "0");
        // P(A=T, B=F) = 0.7 * 0.4 = 0.28
        assert_abs_diff_eq!(product.evaluate(&assign_tf).unwrap(), 0.28, epsilon = 1e-10);

        let mut assign_tt = Assignment::new();
        assign_tt.set(a.id(), "1");
        assign_tt.set(b.id(), "1");
        // P(A=T, B=T) = 0.7 * 0.6 = 0.42
        assert_abs_diff_eq!(product.evaluate(&assign_tt).unwrap(), 0.42, epsilon = 1e-10);
    }

    // CPT tests

    #[test]
    fn cpt_construction_no_parents() {
        let a = make_binary_var("A");
        // Simple prior: P(A=F) = 0.3, P(A=T) = 0.7
        let cpt = ConditionalProbabilityTable::from_values(
            &a,
            &[],
            vec![vec![0.3], vec![0.7]],
        ).unwrap();
        assert_eq!(cpt.child_id(), a.id());
        assert!(cpt.parent_ids().is_empty());
    }

    #[test]
    fn cpt_construction_with_parent() {
        let parent = make_binary_var("Cloudy");
        let child = make_binary_var("Rain");
        // P(Rain | Cloudy):
        //              Cloudy=F  Cloudy=T
        // Rain=F         0.8       0.2
        // Rain=T         0.2       0.8
        let cpt = ConditionalProbabilityTable::from_values(
            &child,
            &[&parent],
            vec![
                vec![0.8, 0.2], // Rain=F
                vec![0.2, 0.8], // Rain=T
            ],
        ).unwrap();
        assert_eq!(cpt.child_id(), child.id());
        assert_eq!(cpt.parent_ids(), &[parent.id()]);
    }

    #[test]
    fn cpt_fails_if_not_normalized() {
        let a = make_binary_var("A");
        let result = ConditionalProbabilityTable::from_values(
            &a,
            &[],
            vec![vec![0.3], vec![0.8]], // sums to 1.1, should fail
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("sums to 1.1"), "Error message was: {}", msg);
    }

    #[test]
    fn cpt_fails_with_wrong_shape() {
        let parent = make_binary_var("P");
        let child = make_binary_var("C");
        // Only one column but parent is binary (needs 2 columns)
        let result = ConditionalProbabilityTable::from_values(
            &child,
            &[&parent],
            vec![
                vec![0.6], // Missing second column
                vec![0.4],
            ],
        );
        assert!(result.is_err());
    }

    #[test]
    fn cpt_validates_each_column_independently() {
        let parent = make_binary_var("P");
        let child = make_binary_var("C");
        // First column sums to 1, second column doesn't
        let result = ConditionalProbabilityTable::from_values(
            &child,
            &[&parent],
            vec![
                vec![0.6, 0.9], // C=F
                vec![0.4, 0.9], // C=T — second column sums to 1.8
            ],
        );
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("sums to 1.8"), "Error message was: {}", msg);
    }
}
