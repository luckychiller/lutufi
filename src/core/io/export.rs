use std::collections::HashMap;
use std::path::Path;
use crate::core::{
    error::{LutufiError, LutufiResult},
    inference::InferenceResult,
};

/// Exports inference results and marginal distributions to CSV format.
pub struct CsvExport;

impl CsvExport {
    /// Exports an inference result (variable distributions) to a CSV string.
    pub fn export_inference_result(result: &InferenceResult) -> LutufiResult<String> {
        let mut out = String::new();

        out.push_str("variable,state,probability\n");

        let mut var_names: Vec<&String> = result.distributions.keys().collect();
        var_names.sort();

        for var_name in var_names {
            let factor = match result.distributions.get(var_name) {
                Some(f) => f,
                None => continue,
            };
            let scope = factor.scope();
            let n = scope.num_entries();

            let states: Vec<String> = if n == 2 {
                vec!["false".to_string(), "true".to_string()]
            } else {
                (0..n).map(|i| i.to_string()).collect()
            };

            for i in 0..n {
                let prob = factor.value_at(i);
                let state = states.get(i).cloned().unwrap_or_else(|| i.to_string());
                out.push_str(&format!(
                    "{},{},{}\n",
                    var_name,
                    state,
                    format_prob(prob)
                ));
            }
        }

        out.push('\n');
        out.push_str(&format!("# algorithm: {}\n", result.algorithm_used.name()));
        out.push_str(&format!(
            "# computation_time_ms: {}\n",
            result.computation_time.as_millis()
        ));
        out.push_str(&format!("# log_likelihood: {}\n", result.log_z));

        Ok(out)
    }

    /// Exports an inference result to a CSV file at the given path.
    pub fn export_inference_to_file<P: AsRef<Path>>(
        result: &InferenceResult,
        path: P,
    ) -> LutufiResult<()> {
        let content = Self::export_inference_result(result)?;
        std::fs::write(path.as_ref(), &content).map_err(|e| {
            LutufiError::SerializationError {
                reason: format!("Failed to write CSV file: {}", e),
            }
        })?;
        Ok(())
    }

    /// Exports a map of marginal distributions (variable -> probabilities) to a CSV string.
    pub fn export_marginals_table(
        marginals: &HashMap<String, Vec<f64>>,
    ) -> LutufiResult<String> {
        let mut out = String::new();
        let mut var_names: Vec<&String> = marginals.keys().collect();
        var_names.sort();

        let max_states = var_names
            .iter()
            .filter_map(|v| marginals.get(*v))
            .map(|probs| probs.len())
            .max()
            .unwrap_or(0);

        out.push_str("variable");
        for i in 0..max_states {
            out.push_str(&format!(",P(state={})", i));
        }
        out.push('\n');

        for var_name in var_names {
            if let Some(probs) = marginals.get(var_name) {
                out.push_str(var_name);
                for prob in probs {
                    out.push_str(&format!(",{}", format_prob(*prob)));
                }
                for _ in probs.len()..max_states {
                    out.push_str(",0.0");
                }
                out.push('\n');
            }
        }

        Ok(out)
    }
}

fn format_prob(p: f64) -> String {
    if p.is_nan() {
        "NaN".to_string()
    } else if p.is_infinite() {
        if p.is_sign_positive() {
            "inf".to_string()
        } else {
            "-inf".to_string()
        }
    } else if p == 0.0 {
        "0.0".to_string()
    } else if (p - 1.0).abs() < 1e-12 {
        "1.0".to_string()
    } else {
        format!("{:.10}", p)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Duration;
    use crate::core::{
        factor::{Scope, TabularFactor},
        inference::{Algorithm, Diagnostics},
        variable::VariableId,
    };

    fn make_mock_result() -> InferenceResult {
        let mut distributions = HashMap::new();
        let scope = Scope::from_ids_and_sizes(vec![VariableId::new()], vec![2]);
        let factor =
            TabularFactor::from_values(scope, vec![0.9, 0.1]).unwrap();
        distributions.insert("Smoking".to_string(), factor);

        InferenceResult {
            variables: vec!["Smoking".to_string()],
            distributions,
            joint_factor: None,
            log_z: -0.5,
            algorithm_used: Algorithm::VariableElimination,
            computation_time: Duration::from_millis(42),
            diagnostics: Diagnostics::None,
        }
    }

    #[test]
    fn test_csv_export_contains_header() {
        let result = make_mock_result();
        let csv = CsvExport::export_inference_result(&result).unwrap();
        assert!(csv.contains("variable,state,probability"));
        assert!(csv.contains("Smoking"));
        assert!(csv.contains("0.9"));
        assert!(csv.contains("0.1"));
    }

    #[test]
    fn test_csv_export_metadata() {
        let result = make_mock_result();
        let csv = CsvExport::export_inference_result(&result).unwrap();
        assert!(csv.contains("Variable Elimination"));
        assert!(csv.contains("42"));
    }

    #[test]
    fn test_csv_export_file() {
        let result = make_mock_result();
        let tmp = std::env::temp_dir().join("test_result.csv");
        CsvExport::export_inference_to_file(&result, &tmp).unwrap();
        let content = std::fs::read_to_string(&tmp).unwrap();
        let _ = std::fs::remove_file(&tmp);
        assert!(content.contains("Smoking"));
    }

    #[test]
    fn test_marginals_table_export() {
        let mut marginals = HashMap::new();
        marginals.insert("A".to_string(), vec![0.5, 0.5]);
        marginals.insert("B".to_string(), vec![0.3, 0.7]);
        let csv = CsvExport::export_marginals_table(&marginals).unwrap();
        assert!(csv.contains("variable,P(state=0),P(state=1)"));
        assert!(csv.contains("A"));
        assert!(csv.contains("B"));
    }
}
