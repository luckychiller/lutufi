use std::path::Path;
use crate::core::error::LutufiResult;
use crate::core::models::bayesian_network::BayesianNetwork;
use super::types::LmfDocument;

impl BayesianNetwork {
    /// Serializes this network to the LMF JSON format and writes it to the
    /// given file path.
    pub fn save_lmf<P: AsRef<Path>>(&self, path: P) -> LutufiResult<()> {
        LmfDocument::from_bayesian_network(self)?.save(path)
    }

    /// Reads an LMF JSON file from the given path and deserializes it into a
    /// [`BayesianNetwork`].
    pub fn load_lmf<P: AsRef<Path>>(path: P) -> LutufiResult<Self> {
        LmfDocument::load(path)?.to_bayesian_network()
    }

    /// Deserializes a [`BayesianNetwork`] from an LMF JSON string.
    pub fn load_lmf_from_str(json: &str) -> LutufiResult<Self> {
        LmfDocument::from_json(json)?.to_bayesian_network()
    }
}
