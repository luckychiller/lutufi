/// LMF (Lutufi Model Format) serialization/deserialization module.
pub mod lmf;
/// BIF (Bayesian Interchange Format) serialization/deserialization module.
pub mod bif;
/// XMLBIF (XML Bayesian Interchange Format) serialization/deserialization module.
pub mod xmlbif;
/// UAI (UAI Competition Format) serialization/deserialization module.
pub mod uai;
/// CSV export module for inference results and marginals.
pub mod export;
/// Format conversion service between supported file formats.
pub mod convert;

pub use lmf::{LmfDocument, LmfMetadata, LmfGraph, LmfVariable, LmfDomain, LmfParameters,
    LmfCpd, LmfEvidence, LmfInferenceSettings, LmfResults, ModelType,
    VerifyReport, VerifyCheck, LMF_CURRENT_VERSION};
pub use bif::BifFormat;
pub use xmlbif::XmlBifFormat;
pub use uai::UaiFormat;
pub use export::CsvExport;
pub use convert::ConversionService;

use std::path::Path;

/// Trait for reading graphs from file or string content.
pub trait GraphReader {
    /// The graph type produced by this reader.
    type Graph;

    /// Reads a graph from the specified file path.
    fn read<P: AsRef<Path>>(&self, path: P) -> crate::Result<Self::Graph>;

    /// Reads a graph from a string slice.
    fn read_str(&self, content: &str) -> crate::Result<Self::Graph>;
}

/// Trait for writing graphs to file or string content.
pub trait GraphWriter {
    /// The graph type written by this writer.
    type Graph;

    /// Writes a graph to the specified file path.
    fn write<P: AsRef<Path>>(&self, graph: &Self::Graph, path: P) -> crate::Result<()>;

    /// Writes a graph to a string and returns it.
    fn write_str(&self, graph: &Self::Graph) -> crate::Result<String>;
}

/// Supported file formats for graph and Bayesian network serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// GraphML XML-based graph format.
    GraphML,
    /// GEXF (Graph Exchange XML Format).
    GEXF,
    /// Pajek .net format.
    Pajek,
    /// GML (Graph Modelling Language) format.
    GML,
    /// Simple edge list text format.
    EdgeList,
    /// Adjacency matrix format.
    AdjacencyMatrix,
    /// Binary serialization format.
    Binary,
    /// Lutufi Model Format (JSON-based).
    LMF,
    /// Bayesian Interchange Format (BIF).
    BIF,
    /// XML Bayesian Interchange Format (XMLBIF).
    XMLBIF,
    /// UAI Competition Format.
    UAI,
    /// Comma-separated values format.
    CSV,
}

impl FileFormat {
    /// Returns the standard file extension for this format (without leading dot).
    pub fn extension(&self) -> &'static str {
        match self {
            FileFormat::GraphML => "graphml",
            FileFormat::GEXF => "gexf",
            FileFormat::Pajek => "net",
            FileFormat::GML => "gml",
            FileFormat::EdgeList => "txt",
            FileFormat::AdjacencyMatrix => "adj",
            FileFormat::Binary => "bin",
            FileFormat::LMF => "lmf",
            FileFormat::BIF => "bif",
            FileFormat::XMLBIF => "xmlbif",
            FileFormat::UAI => "uai",
            FileFormat::CSV => "csv",
        }
    }

    /// Determines the file format from a file path's extension.
    /// Returns `None` if the extension is not recognized.
    pub fn from_extension(path: impl AsRef<Path>) -> Option<Self> {
        let ext = path.as_ref().extension()?.to_str()?.to_lowercase();
        match ext.as_str() {
            "lmf" => Some(FileFormat::LMF),
            "bif" => Some(FileFormat::BIF),
            "xmlbif" | "xbif" | "xml" => Some(FileFormat::XMLBIF),
            "uai" => Some(FileFormat::UAI),
            "csv" => Some(FileFormat::CSV),
            "graphml" => Some(FileFormat::GraphML),
            "gexf" => Some(FileFormat::GEXF),
            "net" => Some(FileFormat::Pajek),
            "gml" => Some(FileFormat::GML),
            "txt" | "edgelist" => Some(FileFormat::EdgeList),
            "adj" => Some(FileFormat::AdjacencyMatrix),
            "bin" => Some(FileFormat::Binary),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_format_extensions() {
        assert_eq!(FileFormat::GraphML.extension(), "graphml");
        assert_eq!(FileFormat::BIF.extension(), "bif");
        assert_eq!(FileFormat::LMF.extension(), "lmf");
        assert_eq!(FileFormat::XMLBIF.extension(), "xmlbif");
        assert_eq!(FileFormat::UAI.extension(), "uai");
        assert_eq!(FileFormat::CSV.extension(), "csv");
    }

    #[test]
    fn test_from_extension() {
        assert_eq!(
            FileFormat::from_extension("model.lmf"),
            Some(FileFormat::LMF)
        );
        assert_eq!(
            FileFormat::from_extension("model.bif"),
            Some(FileFormat::BIF)
        );
        assert_eq!(
            FileFormat::from_extension("model.uai"),
            Some(FileFormat::UAI)
        );
        assert_eq!(FileFormat::from_extension("model.xyz"), None);
    }
}
