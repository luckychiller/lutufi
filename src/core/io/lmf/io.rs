use std::path::Path;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    inference::{Algorithm, InferenceEngine},
    models::bayesian_network::BayesianNetwork,
};
use super::migration::apply_migrations;
use super::types::{LmfDocument, VerifyCheck, VerifyReport};

impl LmfDocument {
    /// Serializes this document to JSON and writes it to the given file path.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> LutufiResult<()> {
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            LutufiError::SerializationError {
                reason: format!("JSON serialization failed: {}", e),
            }
        })?;
        std::fs::write(path.as_ref(), &json).map_err(|e| {
            LutufiError::SerializationError {
                reason: format!("Failed to write file: {}", e),
            }
        })?;
        Ok(())
    }

    /// Reads a JSON file from the given path and deserializes it into an
    /// [`LmfDocument`], applying any required format migrations.
    pub fn load<P: AsRef<Path>>(path: P) -> LutufiResult<Self> {
        let json = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            LutufiError::DeserializationError {
                reason: format!("Failed to read file: {}", e),
            }
        })?;
        Self::from_json(&json)
    }

    /// Deserializes an [`LmfDocument`] from a JSON string, applying any
    /// required format migrations and validating that variables are present.
    pub fn from_json(json: &str) -> LutufiResult<Self> {
        let doc: LmfDocument = serde_json::from_str(json).map_err(|e| {
            LutufiError::DeserializationError {
                reason: format!("JSON parsing failed: {}", e),
            }
        })?;

        let doc = apply_migrations(doc)?;

        if doc.graph.variables.is_empty() {
            return Err(LutufiError::DeserializationError {
                reason: "LMF document has no variables defined".to_string(),
            });
        }

        Ok(doc)
    }

    /// Runs inference on the given [`BayesianNetwork`] using the settings and
    /// evidence stored in this document, then compares the results against the
    /// expected marginals and log-likelihood recorded in the document.
    pub fn verify(&self, network: &BayesianNetwork) -> LutufiResult<VerifyReport> {
        let mut report = VerifyReport {
            passed: true,
            checks: Vec::new(),
        };

        if let Some(ref settings) = self.inference_settings {
            if let Some(ref expected_results) = self.results {
                let algorithm = match settings.algorithm.to_lowercase().as_str() {
                    "auto" => Algorithm::Auto,
                    "variableelimination" | "variable_elimination" => {
                        Algorithm::VariableElimination
                    }
                    "exact" | "junctiontree" | "junction_tree" => Algorithm::Exact,
                    "lbp" | "loopybeliefpropagation" | "loopy_belief_propagation" => {
                        Algorithm::LBP
                    }
                    "mcmc" | "gibbs" | "gibbsampling" | "gibbs_sampling" => Algorithm::MCMC,
                    "variational" | "meanfield" | "mean_field" => Algorithm::Variational,
                    _ => {
                        report.checks.push(VerifyCheck {
                            name: "algorithm".to_string(),
                            passed: false,
                            detail: format!("Unknown algorithm: {}", settings.algorithm),
                        });
                        report.passed = false;
                        return Ok(report);
                    }
                };

                let evidence = match &self.evidence {
                    Some(ev) => {
                        let mut assignment = Assignment::new();
                        for (var, val) in &ev.assignments {
                            if let Ok(vid) = network.id_of(var) {
                                assignment.set(vid, val.as_str());
                            }
                        }
                        assignment
                    }
                    None => Assignment::new(),
                };

                let qvars: Vec<&str> =
                    expected_results.query_variables.iter().map(|s| s.as_str()).collect();

                let result = match InferenceEngine::query(
                    network,
                    &qvars,
                    &evidence,
                    algorithm,
                ) {
                    Ok(r) => r,
                    Err(e) => {
                        report.checks.push(VerifyCheck {
                            name: "inference".to_string(),
                            passed: false,
                            detail: format!("Inference failed: {}", e),
                        });
                        report.passed = false;
                        return Ok(report);
                    }
                };

                for (var_name, expected_probs) in &expected_results.marginals {
                    match result.marginal_prob(var_name, "0") {
                        Ok(_) => {
                            let scope_len = match result.distributions.get(var_name) {
                                Some(f) => f.scope().num_entries(),
                                None => {
                                    report.checks.push(VerifyCheck {
                                        name: format!("marginal_{}", var_name),
                                        passed: false,
                                        detail: format!(
                                            "Variable '{}' not in result distributions",
                                            var_name
                                        ),
                                    });
                                    report.passed = false;
                                    continue;
                                }
                            };

                            if expected_probs.len() != scope_len {
                                report.checks.push(VerifyCheck {
                                    name: format!("marginal_{}", var_name),
                                    passed: false,
                                    detail: format!(
                                        "Expected {} probabilities for '{}', got {}",
                                        scope_len,
                                        var_name,
                                        expected_probs.len()
                                    ),
                                });
                                report.passed = false;
                                continue;
                            }

                            let mut all_match = true;
                            for (i, expected_p) in expected_probs.iter().enumerate() {
                                let actual_p = result.marginal_prob(var_name, &i.to_string())
                                    .unwrap_or(-1.0);
                                if (actual_p - expected_p).abs() > 1e-6 {
                                    all_match = false;
                                    report.checks.push(VerifyCheck {
                                        name: format!("marginal_{}[{}]", var_name, i),
                                        passed: false,
                                        detail: format!(
                                            "Expected {}, got {}",
                                            expected_p, actual_p
                                        ),
                                    });
                                }
                            }
                            if all_match {
                                report.checks.push(VerifyCheck {
                                    name: format!("marginal_{}", var_name),
                                    passed: true,
                                    detail: "All probabilities match".to_string(),
                                });
                            } else {
                                report.passed = false;
                            }
                        }
                        Err(e) => {
                            report.checks.push(VerifyCheck {
                                name: format!("marginal_{}", var_name),
                                passed: false,
                                detail: format!("Failed to compute marginal: {}", e),
                            });
                            report.passed = false;
                        }
                    }
                }

                report.checks.push(VerifyCheck {
                    name: "log_likelihood".to_string(),
                    passed: (result.log_z - expected_results.log_likelihood).abs() < 1e-6,
                    detail: format!(
                        "logZ: expected {}, got {}",
                        expected_results.log_likelihood, result.log_z
                    ),
                });
                if !report.checks.last().map(|c| c.passed).unwrap_or(false) {
                    report.passed = false;
                }
            }
        }

        if report.checks.is_empty() {
            report.checks.push(VerifyCheck {
                name: "structure".to_string(),
                passed: true,
                detail: "No inference results to verify; structural check passed".to_string(),
            });
        }

        Ok(report)
    }
}
