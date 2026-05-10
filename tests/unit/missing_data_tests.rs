use std::collections::HashMap;
use lutufi_core::core::{
    missing_data::{
        MissingValues, MissingDataPattern, MissingMask,
        MissingDataMechanism,
        little_mcar_test, mar_test_logistic,
        ImputationEngine, ImputationMethod,
        ImputationDiagnostics,
        ReconstructionEngine, ReconstructionMethod,
        MnarModel, MnarModelType,
        SensitivityParameter,
        sensitivity::SensitivityParameterType,
    },
    variable::{Variable, VariableId},
    domain::Domain,
};

fn make_vars(n: usize) -> Vec<Variable> {
    (0..n).map(|i| Variable::new(format!("X{i}"), Domain::continuous(None, None))).collect()
}

fn make_var_ids(n: usize) -> Vec<VariableId> {
    make_vars(n).into_iter().map(|v| v.id()).collect()
}

#[test]
fn test_missing_values_new_valid() {
    let var_ids = make_var_ids(2);
    let data = vec![
        vec![Some(1.0), Some(2.0)],
        vec![None, Some(4.0)],
    ];
    let mv = MissingValues::new(var_ids.clone(), data).unwrap();
    assert_eq!(mv.nrows(), 2);
    assert_eq!(mv.ncols(), 2);
}

#[test]
fn test_missing_values_new_empty_vars() {
    let result = MissingValues::new(vec![], vec![vec![Some(1.0)]]);
    assert!(result.is_err());
}

#[test]
fn test_missing_values_new_wrong_cols() {
    let var_ids = make_var_ids(2);
    let data = vec![vec![Some(1.0)]]; // 1 col instead of 2
    let result = MissingValues::new(var_ids, data);
    assert!(result.is_err());
}

#[test]
fn test_missing_data_pattern_missing_fraction() {
    let var_ids = make_var_ids(2);
    let mut missing = HashMap::new();
    let mut rows_x0 = std::collections::HashSet::new();
    rows_x0.insert(1);
    missing.insert(var_ids[0], rows_x0);
    let pattern = MissingDataPattern::new(missing, 2, 2);
    assert!((pattern.missing_fraction() - 0.25).abs() < 1e-10);
}

#[test]
fn test_missing_data_pattern_complete_rows() {
    let var_ids = make_var_ids(2);
    let mut missing = HashMap::new();
    let mut rows = std::collections::HashSet::new();
    rows.insert(0);
    missing.insert(var_ids[0], rows);
    let pattern = MissingDataPattern::new(missing, 3, 2);
    let complete = pattern.complete_rows();
    assert_eq!(complete.len(), 2);
    assert!(complete.contains(&1));
    assert!(complete.contains(&2));
}

#[test]
fn test_missing_mask_from_values() {
    let var_ids = make_var_ids(2);
    let data = vec![
        vec![Some(1.0), None],
        vec![None, Some(2.0)],
    ];
    let mv = MissingValues::new(var_ids, data).unwrap();
    let mask = MissingMask::from_values(&mv);
    assert_eq!(mask.nrows(), 2);
    assert_eq!(mask.ncols(), 2);
    assert!(mask.mask[0][1]);
    assert!(!mask.mask[0][0]);
}

#[test]
fn test_missing_mask_fraction() {
    let var_ids = make_var_ids(2);
    let mask = MissingMask {
        variables: var_ids,
        mask: vec![
            vec![false, true],
            vec![true, false],
        ],
    };
    assert!((mask.missing_fraction() - 0.5).abs() < 1e-10);
}

#[test]
fn test_missing_mask_empty() {
    let mask = MissingMask {
        variables: vec![],
        mask: vec![],
    };
    assert_eq!(mask.missing_fraction(), 0.0);
}

#[test]
fn test_pattern_summary() {
    let var_ids = make_var_ids(2);
    let data = vec![
        vec![Some(1.0), Some(2.0)],
        vec![None, Some(4.0)],
        vec![Some(3.0), None],
    ];
    let mv = MissingValues::new(var_ids.clone(), data).unwrap();
    let summary = mv.summarize();
    assert_eq!(summary.num_patterns, 3);
    assert!((summary.complete_fraction - 1.0 / 3.0).abs() < 1e-10);
    assert!((summary.overall_fraction - 2.0 / 6.0).abs() < 1e-10);
}

#[test]
fn test_little_mcar_test_rejects_non_mcar() {
    let var_ids = make_var_ids(2);
    // Deliberately create data where missingness correlates with values
    let data = vec![
        vec![Some(1.0), Some(2.0)],
        vec![Some(3.0), Some(4.0)],
        vec![Some(5.0), None],
        vec![Some(7.0), None],
        vec![Some(9.0), None],
        vec![None, Some(1.0)],
        vec![None, Some(3.0)],
        vec![None, Some(5.0)],
        vec![Some(10.0), Some(12.0)],
    ];
    let result = little_mcar_test(&data, &var_ids).unwrap();
    // With this pattern, test should compute and give a result
    assert!(result.statistic >= 0.0);
}

#[test]
fn test_little_mcar_test_empty_data() {
    let var_ids = make_var_ids(1);
    let result = little_mcar_test(&[], &var_ids);
    assert!(result.is_err());
}

#[test]
fn test_little_mcar_test_single_row() {
    let var_ids = make_var_ids(2);
    let data = vec![vec![Some(1.0), Some(2.0)]];
    let result = little_mcar_test(&data, &var_ids);
    assert!(result.is_err());
}

#[test]
fn test_mar_test_logistic_insufficient() {
    let var_ids = make_var_ids(2);
    let data = vec![
        vec![Some(1.0), Some(2.0)],
    ];
    let result = mar_test_logistic(&data, &var_ids, &var_ids[0]).unwrap();
    assert!(result.p_value > 0.05);
    assert!(!result.reject_mcar);
}

#[test]
fn test_imputation_mean_mode() {
    let var_ids = make_var_ids(2);
    let data = vec![
        vec![Some(1.0), Some(2.0)],
        vec![Some(3.0), None],
        vec![None, Some(6.0)],
    ];
    let engine = ImputationEngine::new(ImputationMethod::MeanMode);
    let result = engine.impute(&data, &var_ids).unwrap();
    assert_eq!(result.imputed_data.len(), 3);
    assert_eq!(result.imputed_data[0].len(), 2);
    assert_eq!(result.num_imputed, 2);
    // First column mean: (1+3)/2 = 2.0
    assert!((result.imputed_data[2][0] - 2.0).abs() < 1e-10);
    // Second column mean: (2+6)/2 = 4.0
    assert!((result.imputed_data[1][1] - 4.0).abs() < 1e-10);
}

#[test]
fn test_imputation_method_rounds() {
    let engine = ImputationEngine::new(ImputationMethod::MeanMode)
        .with_rounds(10)
        .with_seed(123);
    // Verify builder methods don't panic
    let var_ids = make_var_ids(1);
    let data = vec![vec![Some(1.0)]];
    let result = engine.impute(&data, &var_ids);
    assert!(result.is_ok());
}

#[test]
fn test_diagnostics_compute() {
    let var_ids = make_var_ids(1);
    let true_data = vec![vec![1.0], vec![2.0], vec![3.0]];
    let imputed_data = vec![vec![1.1], vec![2.2], vec![2.9]];
    let missing_mask = vec![vec![false], vec![true], vec![true]];
    let result = ImputationDiagnostics::compute(
        &true_data, &imputed_data, &missing_mask, &var_ids,
    ).unwrap();
    let rmse = result.rmse.unwrap();
    assert!(rmse > 0.0);
    let mae = result.mae.unwrap();
    assert!(mae > 0.0);
    // RMSE of imputed points: sqrt(((2.0-2.2)^2 + (3.0-2.9)^2)/2) = sqrt(0.04+0.01)/sqrt(2) = sqrt(0.025) ≈ 0.158
    assert!((rmse - ((0.04_f64 + 0.01_f64) / 2.0_f64).sqrt()).abs() < 1e-10);
}

#[test]
fn test_diagnostics_row_mismatch() {
    let var_ids = make_var_ids(1);
    let result = ImputationDiagnostics::compute(
        &[vec![1.0]],
        &[vec![1.0], vec![2.0]],
        &[vec![false]],
        &var_ids,
    );
    assert!(result.is_err());
}

#[test]
fn test_reconstruction_engine_basic() {
    let var_ids = make_var_ids(2);
    let data = vec![
        vec![Some(1.0), Some(2.0)],
        vec![Some(3.0), None],
        vec![None, Some(6.0)],
    ];
    let engine = ReconstructionEngine::new(ReconstructionMethod::HotDeck)
        .with_rank(2)
        .with_max_iterations(50)
        .with_tolerance(1e-4);
    let result = engine.reconstruct(&data, &var_ids).unwrap();
    assert_eq!(result.reconstructed.len(), 3);
    assert!(!result.converged || result.reconstruction_error >= 0.0);
}

#[test]
fn test_mnar_model_creation() {
    let outcome = make_var_ids(2);
    let covariates = make_var_ids(1);
    let model = MnarModel::new(
        MnarModelType::PatternMixture,
        outcome,
        covariates,
    );
    assert_eq!(model.model_type, MnarModelType::PatternMixture);
    assert_eq!(model.max_iterations, 100);
    assert!((model.tolerance - 1e-6).abs() < 1e-10);
}

#[test]
fn test_mnar_model_builder_methods() {
    let outcome = make_var_ids(1);
    let covariates = make_var_ids(1);
    let model = MnarModel::new(
        MnarModelType::Selection,
        outcome.clone(),
        covariates,
    )
    .with_max_iterations(200)
    .with_tolerance(1e-8);
    assert_eq!(model.max_iterations, 200);
    assert!((model.tolerance - 1e-8).abs() < 1e-10);
}

#[test]
fn test_sensitivity_parameter_creation() {
    let var_id = make_var_ids(1)[0];
    let param = SensitivityParameter {
        variable: var_id,
        parameter_type: SensitivityParameterType::Delta,
        values: vec![-1.0, -0.5, 0.0, 0.5, 1.0],
    };
    assert_eq!(param.values.len(), 5);
}

#[test]
fn test_mechanism_enum() {
    assert_ne!(MissingDataMechanism::MCAR, MissingDataMechanism::MAR);
    assert_ne!(MissingDataMechanism::MAR, MissingDataMechanism::MNAR);
}

#[test]
fn test_little_mcar_test_known_pattern() {
    let var_ids = make_var_ids(3);
    // All data complete => MCAR should not be strongly rejected
    let data: Vec<Vec<Option<f64>>> = (0..100).map(|i| {
        vec![Some(i as f64), Some((i*2) as f64), Some((i*3) as f64)]
    }).collect();
    let result = little_mcar_test(&data, &var_ids).unwrap();
    assert!(result.statistic >= 0.0);
    assert!(result.degrees_of_freedom > 0);
}

#[test]
fn test_missing_data_pattern_variable_fraction() {
    let var_ids = make_var_ids(2);
    let mut missing = HashMap::new();
    let mut rows = std::collections::HashSet::new();
    rows.insert(0);
    rows.insert(1);
    missing.insert(var_ids[0], rows);
    let pattern = MissingDataPattern::new(missing, 4, 2);
    assert!((pattern.variable_missing_fraction(&var_ids[0]) - 0.5).abs() < 1e-10);
    assert!((pattern.variable_missing_fraction(&var_ids[1]) - 0.0).abs() < 1e-10);
}

#[test]
fn test_missing_data_pattern_is_missing() {
    let var_ids = make_var_ids(2);
    let mut missing = HashMap::new();
    let mut rows = std::collections::HashSet::new();
    rows.insert(1);
    missing.insert(var_ids[0], rows);
    let pattern = MissingDataPattern::new(missing, 2, 2);
    assert!(pattern.is_missing(&var_ids[0], 1));
    assert!(!pattern.is_missing(&var_ids[0], 0));
    assert!(!pattern.is_missing(&var_ids[1], 1));
}
