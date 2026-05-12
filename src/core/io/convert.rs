use std::path::Path;
use crate::core::{
    error::{LutufiError, LutufiResult},
    models::bayesian_network::BayesianNetwork,
};
use super::{BifFormat, XmlBifFormat, UaiFormat, FileFormat, LmfDocument};

/// Service for converting Bayesian networks between supported file formats.
pub struct ConversionService;

impl ConversionService {
    /// Converts a file from one format to another based on file extensions.
    pub fn convert(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> LutufiResult<()> {
        let input_fmt = FileFormat::from_extension(input_path.as_ref()).ok_or_else(|| {
            LutufiError::DeserializationError {
                reason: format!(
                    "Unknown input format for file: {:?}",
                    input_path.as_ref()
                ),
            }
        })?;

        let output_fmt = FileFormat::from_extension(output_path.as_ref()).ok_or_else(|| {
            LutufiError::SerializationError {
                reason: format!(
                    "Unknown output format for file: {:?}",
                    output_path.as_ref()
                ),
            }
        })?;

        let network = Self::load(input_path.as_ref(), input_fmt)?;
        Self::save(output_path.as_ref(), output_fmt, &network)
    }

    /// Loads a Bayesian network from a file in the specified format.
    pub fn load(path: impl AsRef<Path>, format: FileFormat) -> LutufiResult<BayesianNetwork> {
        match format {
            FileFormat::LMF => BayesianNetwork::load_lmf(path),
            FileFormat::BIF => BifFormat::import_from_file(path),
            FileFormat::XMLBIF => XmlBifFormat::import_from_file(path),
            FileFormat::UAI => UaiFormat::import_from_file(path),
            _ => Err(LutufiError::DeserializationError {
                reason: format!("Unsupported input format: {:?}", format),
            }),
        }
    }

    /// Saves a Bayesian network to a file in the specified format.
    pub fn save(
        path: impl AsRef<Path>,
        format: FileFormat,
        network: &BayesianNetwork,
    ) -> LutufiResult<()> {
        match format {
            FileFormat::LMF => network.save_lmf(path),
            FileFormat::BIF => BifFormat::export_to_file(network, path),
            FileFormat::XMLBIF => XmlBifFormat::export_to_file(network, path),
            FileFormat::UAI => UaiFormat::export_to_file(network, path),
            _ => Err(LutufiError::SerializationError {
                reason: format!("Unsupported output format: {:?}", format),
            }),
        }
    }

    /// Converts a Bayesian network string from one format to another.
    pub fn convert_str(input: &str, input_format: &str, output_format: &str) -> LutufiResult<String> {
        let network = match input_format.to_lowercase().as_str() {
            "lmf" => BayesianNetwork::load_lmf_from_str(input)?,
            "bif" => BifFormat::import(input)?,
            "xmlbif" => XmlBifFormat::import(input)?,
            "uai" => UaiFormat::import(input)?,
            _ => {
                return Err(LutufiError::DeserializationError {
                    reason: format!("Unsupported input format: {}", input_format),
                });
            }
        };

        match output_format.to_lowercase().as_str() {
            "lmf" => {
                let doc = LmfDocument::from_bayesian_network(&network)?;
                serde_json::to_string_pretty(&doc).map_err(|e| {
                    LutufiError::SerializationError {
                        reason: format!("JSON serialization failed: {}", e),
                    }
                })
            }
            "bif" => BifFormat::export(&network),
            "xmlbif" => XmlBifFormat::export(&network),
            "uai" => UaiFormat::export(&network),
            _ => Err(LutufiError::SerializationError {
                reason: format!("Unsupported output format: {}", output_format),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        domain::Domain,
        factor::ConditionalProbabilityTable,
        variable::Variable,
    };

    fn create_test_network() -> BayesianNetwork {
        let mut net = BayesianNetwork::new();
        net.add_variable("X", Domain::binary());
        net.add_variable("Y", Domain::binary());
        net.add_edge("X", "Y").unwrap();
        let x = net.variable("X").unwrap().clone();
        let y = net.variable("Y").unwrap().clone();
        let cx = ConditionalProbabilityTable::from_values(&x, &[] as &[&Variable], vec![vec![0.5], vec![0.5]]).unwrap();
        net.set_cpd("X", cx).unwrap();
        let cy = ConditionalProbabilityTable::from_values(&y, &[&x], vec![vec![0.9, 0.2], vec![0.1, 0.8]]).unwrap();
        net.set_cpd("Y", cy).unwrap();
        net
    }

    #[test]
    fn test_convert_bif_to_lmf_str() {
        let net = create_test_network();
        let bif = BifFormat::export(&net).unwrap();
        let lmf_json = ConversionService::convert_str(&bif, "bif", "lmf").unwrap();
        let reloaded = BayesianNetwork::load_lmf_from_str(&lmf_json).unwrap();
        assert_eq!(reloaded.nodes().len(), 2);
    }

    #[test]
    fn test_convert_lmf_to_bif_str() {
        let net = create_test_network();
        let doc = LmfDocument::from_bayesian_network(&net).unwrap();
        let lmf_json = serde_json::to_string_pretty(&doc).unwrap();
        let bif_out = ConversionService::convert_str(&lmf_json, "lmf", "bif").unwrap();
        assert!(bif_out.contains("variable X"));
        assert!(bif_out.contains("variable Y"));
    }

    #[test]
    fn test_convert_file_roundtrip() {
        let net = create_test_network();
        let tmp_lmf = std::env::temp_dir().join("test_convert.lmf");
        let tmp_bif = std::env::temp_dir().join("test_convert.bif");
        net.save_lmf(&tmp_lmf).unwrap();
        ConversionService::convert(&tmp_lmf, &tmp_bif).unwrap();
        let reloaded = BifFormat::import_from_file(&tmp_bif).unwrap();
        let _ = std::fs::remove_file(&tmp_lmf);
        let _ = std::fs::remove_file(&tmp_bif);
        assert_eq!(reloaded.nodes().len(), 2);
    }
}
