use std::path::Path;
use crate::core::error::LutufiResult;
use crate::core::models::bayesian_network::BayesianNetwork;
use super::types::LmfDocument;

impl BayesianNetwork {
    pub fn save_lmf<P: AsRef<Path>>(&self, path: P) -> LutufiResult<()> {
        LmfDocument::from_bayesian_network(self)?.save(path)
    }

    pub fn load_lmf<P: AsRef<Path>>(path: P) -> LutufiResult<Self> {
        LmfDocument::load(path)?.to_bayesian_network()
    }

    pub fn load_lmf_from_str(json: &str) -> LutufiResult<Self> {
        LmfDocument::from_json(json)?.to_bayesian_network()
    }
}
