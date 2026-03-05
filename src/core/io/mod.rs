//! I/O module
//!
//! Provides input/output operations for various formats:
//! - GraphML
//! - GEXF
//! - Pajek/NET
//! - GML
//! - Adjacency list/matrix formats
//! - CSV/TSV edge lists
//! - Binary formats

use std::path::Path;

/// Trait for format readers
pub trait GraphReader {
    /// Type of graph being read
    type Graph;
    
    /// Read graph from a file path
    fn read<P: AsRef<Path>>(&self, path: P) -> crate::Result<Self::Graph>;
    
    /// Read graph from a string
    fn read_str(&self, content: &str) -> crate::Result<Self::Graph>;
}

/// Trait for format writers
pub trait GraphWriter {
    /// Type of graph being written
    type Graph;
    
    /// Write graph to a file path
    fn write<P: AsRef<Path>>(&self, graph: &Self::Graph, path: P) -> crate::Result<()>;
    
    /// Write graph to a string
    fn write_str(&self, graph: &Self::Graph) -> crate::Result<String>;
}

/// Supported file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// GraphML format
    GraphML,
    /// GEXF format
    GEXF,
    /// Pajek/NET format
    Pajek,
    /// GML format
    GML,
    /// Edge list format
    EdgeList,
    /// Adjacency matrix format
    AdjacencyMatrix,
    /// Binary format
    Binary,
}

impl FileFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            FileFormat::GraphML => "graphml",
            FileFormat::GEXF => "gexf",
            FileFormat::Pajek => "net",
            FileFormat::GML => "gml",
            FileFormat::EdgeList => "txt",
            FileFormat::AdjacencyMatrix => "adj",
            FileFormat::Binary => "bin",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_format_extensions() {
        assert_eq!(FileFormat::GraphML.extension(), "graphml");
        assert_eq!(FileFormat::GEXF.extension(), "gexf");
        assert_eq!(FileFormat::Pajek.extension(), "net");
        assert_eq!(FileFormat::EdgeList.extension(), "txt");
    }
}