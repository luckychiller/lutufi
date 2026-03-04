# Lutufi Serialization and Reproducibility Document

**Document Version:** 1.0  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Serialization Goals](#serialization-goals)
3. [The Lutufi Format (.lut)](#the-lutufi-format-lut)
4. [Schema Evolution](#schema-evolution)
5. [What Gets Serialized](#what-gets-serialized)
6. [Text vs Binary](#text-vs-binary)
7. [Compression](#compression)
8. [Cross-Platform Compatibility](#cross-platform-compatibility)
9. [Security in Serialization](#security-in-serialization)
10. [Incremental Serialization](#incremental-serialization)
11. [External Format Support](#external-format-support)
12. [Reproducibility Guarantees](#reproducibility-guarantees)
13. [Version Migration](#version-migration)
14. [Digital Signatures](#digital-signatures)
15. [Archive Formats](#archive-formats)
16. [How Lutufi Ensures Reproducibility](#how-lutufi-ensures-reproducibility)
17. [Key References](#key-references)

---

## Executive Summary

Serialization is the foundation of model persistence, sharing, and reproducibility in Lutufi. This document defines the comprehensive serialization strategy, including the binary .lut format, schema evolution mechanisms, cross-platform compatibility guarantees, and security considerations. The design prioritizes speed, compactness, and long-term archival stability while maintaining full reproducibility of probabilistic computations.

Key serialization principles:

1. **Self-Description**: Serialized models carry their own schema metadata
2. **Forward Compatibility**: New versions can read old files
3. **Backward Compatibility**: Old versions can read essential data from new files
4. **Integrity Verification**: Checksums and optional digital signatures
5. **Reproducibility**: Complete capture of random state and algorithm parameters

---

## Serialization Goals

### Model Persistence

Lutufi serialization enables:

```rust
// Save trained model
model.save("disease_diagnosis.lut")?;

// Load and use later
let model = BayesianNetwork::load("disease_diagnosis.lut")?;
let prediction = model.predict(&new_patient_data)?;
```

**Persistence Requirements:**
- Fast save/load for models up to 1 million nodes
- Memory-mapped loading for out-of-core models
- Atomic writes (no partial files on failure)
- Compression for network transmission

### Reproducibility

Scientific reproducibility demands:

```rust
// Exact reproduction of inference
let result = model.query(&["Outcome"], &evidence)?;
assert_eq!(result, expected_result); // Bit-for-bit identical
```

**Reproducibility Requirements:**
- Deterministic algorithms with fixed seeds
- Complete serialization of random state
- Version information for all components
- Environment capture (library versions, hardware)

### Sharing

Interoperability with the research community:

```python
# Share models across platforms
model.save("research_model.lut")
# Colleague on different OS can load
model = lf.load("research_model.lut")
```

**Sharing Requirements:**
- Cross-platform binary format
- Self-contained (no external dependencies)
- Human-readable metadata
- Standard compression

### Archiving

Long-term preservation:

```rust
// Archive with full provenance
let archive = ModelArchive::new(model)
    .with_provenance(provenance)
    .with_checksums()
    .sign(&key);
archive.save("permanent_archive.lut")?;
```

**Archival Requirements:**
- Stable format specification
- Documentation of all fields
- Checksum verification
- Migration path for format updates

### Long-Term Storage

For models stored years before loading:

- Format version clearly identified
- Schema documentation archived
- Migration tools maintained
- Backward compatibility guarantees

---

## The Lutufi Format (.lut)

### Binary Format Design

The .lut format is a binary container with structured sections:

```
+----------------------------------+
|           HEADER                 |
+----------------------------------+
|  Magic: "LUTU" (4 bytes)         |
|  Version: major.minor (4 bytes)  |
|  Flags: compression, etc (4 bytes)|
|  Header checksum (4 bytes)       |
+----------------------------------+
|         SECTION TABLE            |
+----------------------------------+
|  Number of sections (4 bytes)    |
|  Section 1: offset, size, type   |
|  Section 2: offset, size, type   |
|  ...                             |
+----------------------------------+
|         SECTION DATA             |
+----------------------------------+
|  [Variable Section]              |
|  [Factor Section]                |
|  [CPD Section]                   |
|  [Evidence Section]              |
|  [Metadata Section]              |
+----------------------------------+
|         FOOTER                   |
+----------------------------------+
|  Total file checksum (8 bytes)   |
|  Signature (optional)            |
+----------------------------------+
```

### Header Structure

```rust
#[repr(C, packed)]
pub struct LutHeader {
    /// Magic number: b"LUTU"
    pub magic: [u8; 4],
    
    /// Format version: major (2 bytes) | minor (2 bytes)
    pub version: u32,
    
    /// Feature flags
    pub flags: HeaderFlags,
    
    /// Size of header in bytes
    pub header_size: u32,
    
    /// CRC32 of header (excluding this field)
    pub header_crc: u32,
}

bitflags! {
    pub struct HeaderFlags: u32 {
        /// Data is compressed
        const COMPRESSED = 0x0001;
        /// File is signed
        const SIGNED = 0x0002;
        /// Uses 64-bit addressing
        const LARGE_FILE = 0x0004;
        /// Includes inference state
        const HAS_INFERENCE_STATE = 0x0008;
        /// Includes learning state
        const HAS_LEARNING_STATE = 0x0010;
        /// Encrypted
        const ENCRYPTED = 0x0020;
    }
}
```

### Body Organization

```rust
pub struct SectionTable {
    pub sections: Vec<SectionEntry>,
}

pub struct SectionEntry {
    pub section_type: SectionType,
    pub offset: u64,
    pub size: u64,
    pub uncompressed_size: Option<u64>,
    pub compression: CompressionType,
    pub checksum: u32,
}

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum SectionType {
    Variables = 0x01,
    Factors = 0x02,
    Edges = 0x03,
    CPDs = 0x04,
    Evidence = 0x05,
    Beliefs = 0x06,
    Metadata = 0x07,
    InferenceState = 0x08,
    LearningState = 0x09,
    RandomState = 0x0A,
    Custom = 0xFF,
}
```

### Footer with Checksums

```rust
pub struct LutFooter {
    /// CRC32 of entire file (excluding footer)
    pub file_crc: u64,
    
    /// SHA-256 for cryptographic integrity (optional)
    pub sha256: Option<[u8; 32]>,
    
    /// Digital signature (if signed)
    pub signature: Option<Signature>,
    
    /// Offset to trailer (for append-only updates)
    pub trailer_offset: u64,
}
```

---

## Schema Evolution

### Version Markers

Every serialized model includes version information at multiple levels:

```rust
pub struct VersionInfo {
    /// Format version (e.g., "1.2")
    pub format_version: String,
    
    /// Library version that created the file
    pub library_version: String,
    
    /// Schema version for the data model
    pub schema_version: String,
    
    /// Minimum library version required to read
    pub min_readable_version: String,
}
```

### Backward Compatibility

New versions of Lutufi can read old files:

```rust
impl Deserializer {
    pub fn load_legacy_format(&self, data: &[u8], file_version: &Version) -> Result<FactorGraph, Error> {
        match file_version.major {
            0 => self.load_v0_format(data),
            1 => self.load_v1_format(data),
            _ => Err(Error::UnsupportedVersion(file_version.clone())),
        }
    }
    
    fn load_v0_format(&self, data: &[u8]) -> Result<FactorGraph, Error> {
        // V0 had different variable encoding
        let variables = self.read_v0_variables(data)?;
        
        // Convert to current representation
        let current_vars: Vec<Variable> = variables.into_iter()
            .map(|v| self.upgrade_variable(v))
            .collect();
        
        Ok(FactorGraph::from_variables(current_vars))
    }
}
```

### Forward Compatibility

Old versions can read essential data from new files:

```rust
pub struct ForwardCompatibleSection {
    /// Known fields that old versions understand
    pub known_fields: Vec<KnownField>,
    
    /// Unknown fields preserved but ignored
    pub unknown_data: Vec<u8>,
}

impl Serializer {
    /// When serializing, include placeholder for future fields
    pub fn serialize_with_extensions<T: Serialize>(
        &self,
        value: &T,
        extension_schema: &ExtensionSchema
    ) -> Vec<u8> {
        let base = bincode::serialize(value).unwrap();
        
        // Reserve space for future extensions
        let extension_bytes = extension_schema.serialize_empty();
        
        [base, extension_bytes].concat()
    }
}
```

### Migration Strategies

```rust
pub struct MigrationManager {
    migrations: HashMap<(Version, Version), Box<dyn Migration>>,
}

pub trait Migration {
    fn from_version(&self) -> Version;
    fn to_version(&self) -> Version;
    fn migrate(&self, data: &[u8]) -> Result<Vec<u8>, MigrationError>;
}

impl MigrationManager {
    pub fn migrate(&self, data: &[u8], from: Version, to: Version) -> Result<Vec<u8>, MigrationError> {
        // Find migration path
        let path = self.find_migration_path(from, to)?;
        
        // Apply migrations sequentially
        let mut current = data.to_vec();
        for migration in path {
            current = migration.migrate(&current)?;
        }
        
        Ok(current)
    }
    
    fn find_migration_path(&self, from: Version, to: Version) -> Result<Vec<&dyn Migration>, MigrationError> {
        // BFS to find shortest migration path
        let mut queue = VecDeque::new();
        queue.push_back((from, vec![]));
        
        while let Some((current, path)) = queue.pop_front() {
            if current == to {
                return Ok(path);
            }
            
            for migration in self.migrations_from(current) {
                let mut new_path = path.clone();
                new_path.push(migration);
                queue.push_back((migration.to_version(), new_path));
            }
        }
        
        Err(MigrationError::NoPathFound { from, to })
    }
}
```

---

## What Gets Serialized

### Network Structure

```rust
pub struct SerializedStructure {
    /// Node information
    pub nodes: Vec<SerializedNode>,
    
    /// Edge connections
    pub edges: Vec<SerializedEdge>,
    
    /// Graph properties
    pub properties: GraphProperties,
}

pub struct SerializedNode {
    pub id: NodeId,
    pub name: String,
    pub node_type: NodeType,
    pub domain: SerializedDomain,
    pub metadata: HashMap<String, MetadataValue>,
}

pub struct SerializedEdge {
    pub source: NodeId,
    pub target: NodeId,
    pub edge_type: EdgeType,
    pub weight: Option<f64>,
}
```

### CPD Parameters

```rust
pub enum SerializedCPD {
    Tabular {
        variable: NodeId,
        parents: Vec<NodeId>,
        values: Vec<f64>,  // Flattened
        shape: Vec<usize>,
    },
    
    Gaussian {
        variable: NodeId,
        parents: Vec<NodeId>,
        coefficients: Vec<f64>,
        intercept: f64,
        variance: f64,
    },
    
    Sparse {
        variable: NodeId,
        parents: Vec<NodeId>,
        non_zero_entries: Vec<(Vec<usize>, f64)>,
        default_value: f64,
        shape: Vec<usize>,
    },
    
    Functional {
        variable: NodeId,
        function_type: String,
        parameters: Vec<u8>,  // Serialized function params
    },
}
```

### Evidence

```rust
pub struct SerializedEvidence {
    /// Hard evidence
    pub hard_evidence: Vec<HardEvidenceEntry>,
    
    /// Soft evidence
    pub soft_evidence: Vec<SoftEvidenceEntry>,
    
    /// Virtual evidence (likelihoods)
    pub virtual_evidence: Vec<VirtualEvidenceEntry>,
}

pub struct HardEvidenceEntry {
    pub variable: NodeId,
    pub value: SerializedValue,
    pub timestamp: Option<DateTime<Utc>>,
}
```

### Inference State

For resumable inference:

```rust
pub struct SerializedInferenceState {
    /// Algorithm-specific state
    pub algorithm: String,
    pub algorithm_version: String,
    
    /// Iteration count
    pub iteration: usize,
    
    /// Convergence state
    pub convergence: ConvergenceState,
    
    /// Beliefs/messages
    pub beliefs: Vec<SerializedBelief>,
    pub messages: Vec<SerializedMessage>,
    
    /// Random state (for stochastic algorithms)
    pub random_state: SerializedRandomState,
}

pub struct SerializedMessage {
    pub from: NodeId,
    pub to: NodeId,
    pub content: SerializedBelief,
    pub iteration: usize,
}
```

### Metadata

```rust
pub struct SerializedMetadata {
    /// Creation information
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub library_version: String,
    
    /// Description
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    
    /// Provenance
    pub source: Option<String>,
    pub training_data_hash: Option<String>,
    pub training_parameters: Option<SerializedTrainingParams>,
    
    /// Custom user metadata
    pub custom: HashMap<String, serde_json::Value>,
}
```

### Random Seeds

Complete capture for reproducibility:

```rust
pub struct SerializedRandomState {
    /// RNG algorithm identifier
    pub algorithm: String,  // "xoshiro256++", "pcg64", etc.
    
    /// Serialized state
    pub state: Vec<u8>,
    
    /// Seed value (if deterministically seeded)
    pub seed: Option<u64>,
    
    /// Number of values generated
    pub stream_position: u64,
}
```

---

## Text vs Binary

### When Each is Appropriate

| Aspect | Binary (.lut) | Text (JSON/XML) |
|--------|---------------|-----------------|
| File size | Compact (2-10x smaller) | Verbose |
| Read/write speed | Fast (native formats) | Slower (parsing) |
| Human readability | Not readable | Readable |
| Version control | Poor (binary diffs) | Good (text diffs) |
| Cross-language | Requires libraries | Universal |
| Streaming | Supported | Challenging |
| Schema evolution | Controlled | Flexible |

### Binary Format (Default)

```rust
impl BayesianNetwork {
    /// Save to binary .lut format
    pub fn save(&self, path: &Path) -> Result<(), SaveError> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        
        // Write header
        let header = LutHeader::new(CURRENT_VERSION);
        writer.write_all(&header.to_bytes())?;
        
        // Write sections
        self.write_variables_section(&mut writer)?;
        self.write_factors_section(&mut writer)?;
        self.write_cpds_section(&mut writer)?;
        self.write_metadata_section(&mut writer)?;
        
        // Write footer with checksums
        let footer = self.compute_footer(&writer)?;
        writer.write_all(&footer.to_bytes())?;
        
        writer.flush()?;
        Ok(())
    }
}
```

### Text Format (JSON) for Interoperability

```rust
impl BayesianNetwork {
    /// Export to JSON for interoperability
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let serializable = SerializableBayesianNetwork::from(self);
        serde_json::to_string_pretty(&serializable)
    }
    
    /// Import from JSON
    pub fn from_json(json: &str) -> Result<Self, LoadError> {
        let serializable: SerializableBayesianNetwork = serde_json::from_str(json)?;
        Ok(Self::from(serializable))
    }
}

// JSON structure example:
// {
//   "format": "lutufi-bn-v1",
//   "nodes": [
//     {"id": 0, "name": "Disease", "states": ["present", "absent"]},
//     {"id": 1, "name": "Symptom", "states": ["severe", "mild", "none"]}
//   ],
//   "edges": [[0, 1]],
//   "cpds": [
//     {"variable": 0, "probabilities": [0.1, 0.9]},
//     {"variable": 1, "parents": [0], "probabilities": [[0.8, 0.5, 0.1], [0.2, 0.5, 0.9]]}
//   ]
// }
```

### XML Format

For legacy systems requiring XML:

```rust
pub mod xml_format {
    use quick_xml::events::{Event, BytesStart, BytesText};
    
    pub fn to_xml(model: &BayesianNetwork) -> String {
        let mut writer = Writer::new(Vec::new());
        
        // XML declaration
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)));
        
        // Root element
        writer.write_event(Event::Start(BytesStart::new("BayesianNetwork")));
        
        // Variables
        for node in model.nodes() {
            writer.write_event(Event::Start(BytesStart::new("Variable")));
            writer.write_event(Event::Text(BytesText::new(&node.name)));
            writer.write_event(Event::End(BytesEnd::new("Variable")));
        }
        
        // ... edges, CPDs
        
        writer.write_event(Event::End(BytesEnd::new("BayesianNetwork")));
        
        String::from_utf8(writer.into_inner()).unwrap()
    }
}
```

---

## Compression

### Optional Compression Algorithms

```rust
pub enum CompressionType {
    None,
    Zstd,      // Fast, good compression (default)
    Lz4,       // Very fast, moderate compression
    Gzip,      // Maximum compatibility
    Brotli,    // Maximum compression (slow)
}

pub struct CompressionConfig {
    pub algorithm: CompressionType,
    pub level: i32,  // Compression level (algorithm-specific)
    pub threshold: usize,  // Don't compress if smaller than this
}
```

### Tradeoffs

| Algorithm | Speed | Ratio | Use Case |
|-----------|-------|-------|----------|
| None | Fastest | 1.0x | Small models, speed critical |
| LZ4 | Very fast | 2-3x | Real-time applications |
| Zstd | Fast | 3-5x | Default choice |
| Gzip | Moderate | 3-5x | Maximum compatibility |
| Brotli | Slow | 5-8x | Archival storage |

### Automatic Detection

```rust
impl Serializer {
    pub fn serialize_with_compression(
        &self,
        model: &FactorGraph,
        config: &CompressionConfig
    ) -> Result<Vec<u8>, Error> {
        let uncompressed = self.serialize(model)?;
        
        // Don't compress small files
        if uncompressed.len() < config.threshold {
            return Ok(uncompressed);
        }
        
        // Try compression
        let compressed = match config.algorithm {
            CompressionType::Zstd => zstd::encode_all(
                &uncompressed[..],
                config.level
            )?,
            CompressionType::Lz4 => lz4::block::compress(
                &uncompressed,
                Some(config.level),
                true
            )?,
            CompressionType::Gzip => {
                let mut encoder = GzEncoder::new(
                    Vec::new(),
                    Compression::new(config.level as u32)
                );
                encoder.write_all(&uncompressed)?;
                encoder.finish()?
            }
            CompressionType::None => uncompressed,
        };
        
        // Only use compressed if it actually helps
        if compressed.len() < uncompressed.len() * 9 / 10 {
            Ok(compressed)
        } else {
            Ok(uncompressed)
        }
    }
}
```

---

## Cross-Platform Compatibility

### Endianness

```rust
pub fn write_u64<W: Write>(writer: &mut W, value: u64) -> io::Result<()> {
    // Always write in little-endian (most common)
    writer.write_all(&value.to_le_bytes())
}

pub fn read_u64<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

// Big-endian platforms convert automatically
#[cfg(target_endian = "big")]
pub fn write_u64<W: Write>(writer: &mut W, value: u64) -> io::Result<()> {
    writer.write_all(&value.to_be_bytes())
}
```

### Float Representation

```rust
pub fn write_f64<W: Write>(writer: &mut W, value: f64) -> io::Result<()> {
    // IEEE 754 is standard across platforms
    // Just ensure consistent endianness
    writer.write_all(&value.to_bits().to_le_bytes())
}

// Special handling for NaN and infinity
pub fn serialize_f64(value: f64) -> [u8; 8] {
    if value.is_nan() {
        // Use canonical NaN representation
        f64::NAN.to_bits().to_le_bytes()
    } else if value.is_infinite() {
        if value.is_sign_positive() {
            f64::INFINITY.to_bits().to_le_bytes()
        } else {
            f64::NEG_INFINITY.to_bits().to_le_bytes()
        }
    } else {
        value.to_bits().to_le_bytes()
    }
}
```

### Path Handling

```rust
pub struct PortablePath {
    /// Stored with forward slashes, relative to model root
    path: String,
}

impl From<&Path> for PortablePath {
    fn from(path: &Path) -> Self {
        let normalized = path
            .to_string_lossy()
            .replace('\\', "/");  // Normalize to forward slashes
        
        Self { path: normalized }
    }
}

impl PortablePath {
    pub fn to_local_path(&self) -> PathBuf {
        if cfg!(windows) {
            PathBuf::from(&self.path.replace('/', "\\"))
        } else {
            PathBuf::from(&self.path)
        }
    }
}
```

---

## Security in Serialization

### Safe Deserialization

```rust
pub struct SafeDeserializer {
    max_size: usize,
    max_depth: usize,
    allowed_types: HashSet<String>,
}

impl SafeDeserializer {
    pub fn deserialize(&self, data: &[u8]) -> Result<FactorGraph, DeserializationError> {
        // Check total size
        if data.len() > self.max_size {
            return Err(DeserializationError::SizeLimitExceeded {
                size: data.len(),
                limit: self.max_size,
            });
        }
        
        // Parse header
        let header = self.parse_header(&data[..128])?;
        
        // Verify checksums before full parsing
        self.verify_integrity(data)?;
        
        // Deserialize with depth tracking
        self.deserialize_with_limits(data, 0)
    }
    
    fn deserialize_with_limits(
        &self,
        data: &[u8],
        depth: usize
    ) -> Result<FactorGraph, DeserializationError> {
        if depth > self.max_depth {
            return Err(DeserializationError::DepthLimitExceeded);
        }
        
        // Actual deserialization with bounds checking
        // ...
    }
}
```

### Preventing Code Injection

```rust
pub struct TypeFilter;

impl TypeFilter {
    /// Only allow known safe types
    pub fn is_allowed_type(type_name: &str) -> bool {
        let allowed = [
            "Variable",
            "Factor",
            "CPD",
            "Evidence",
            "Belief",
            // ... other known types
        ];
        
        allowed.contains(&type_name)
    }
    
    /// Reject types that could execute code
    pub fn is_dangerous_type(type_name: &str) -> bool {
        let dangerous_patterns = [
            "exec",
            "eval",
            "__import__",
            "os.system",
            "subprocess",
        ];
        
        dangerous_patterns.iter().any(|p| type_name.contains(p))
    }
}
```

### Validation of Loaded Data

```rust
impl FactorGraph {
    pub fn validate_after_load(&self) -> Result<(), ValidationError> {
        // Check for cycles in DAGs
        if self.is_directed() && self.has_cycle() {
            return Err(ValidationError::CycleDetected);
        }
        
        // Validate all CPDs sum to 1
        for cpd in self.cpds() {
            if !cpd.is_normalized() {
                return Err(ValidationError::NonNormalizedCPD {
                    variable: cpd.variable().name.clone(),
                });
            }
        }
        
        // Check for NaN/Inf in parameters
        for factor in self.factors() {
            if factor.has_invalid_values() {
                return Err(ValidationError::InvalidValues);
            }
        }
        
        // Validate structure matches CPDs
        for node in self.nodes() {
            let parents_in_structure = self.parents(node);
            let parents_in_cpd = self.get_cpd(node).map(|c| c.parents());
            
            if parents_in_cpd != Some(&parents_in_structure) {
                return Err(ValidationError::StructureMismatch);
            }
        }
        
        Ok(())
    }
}
```

---

## Incremental Serialization

### Saving Partial Results

```rust
pub struct IncrementalSerializer {
    base_file: PathBuf,
    delta_file: PathBuf,
    last_checkpoint: Instant,
    checkpoint_interval: Duration,
}

impl IncrementalSerializer {
    pub fn save_checkpoint(&self, state: &InferenceState) -> Result<(), Error> {
        let delta = self.compute_delta(state);
        
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.delta_file)?;
        
        // Serialize delta
        let serialized = bincode::serialize(&delta)?;
        
        // Write length prefix then data
        file.write_all(&(serialized.len() as u64).to_le_bytes())?;
        file.write_all(&serialized)?;
        
        Ok(())
    }
    
    pub fn restore_from_checkpoints(&self) -> Result<InferenceState, Error> {
        // Load base state
        let mut state = self.load_base_state()?;
        
        // Apply deltas sequentially
        let deltas = self.load_deltas()?;
        for delta in deltas {
            state.apply_delta(delta)?;
        }
        
        Ok(state)
    }
}
```

### Resumable Computation

```rust
pub struct ResumableComputation {
    computation_id: Uuid,
    state_file: PathBuf,
}

impl ResumableComputation {
    pub async fn run_or_resume<F, T>(
        &self,
        computation: F
    ) -> Result<T, Error>
    where F: FnOnce(Option<Checkpoint>) -> Result<T, Error> {
        // Check for existing checkpoint
        let checkpoint = self.load_checkpoint()?;
        
        // Run computation
        let result = computation(checkpoint)?;
        
        // Clean up checkpoint on success
        if result.is_ok() {
            self.remove_checkpoint()?;
        }
        
        result
    }
    
    pub fn checkpoint(&self, state: &ComputationState) -> Result<(), Error> {
        let temp_file = self.state_file.with_extension("tmp");
        
        // Write to temp file first (atomic)
        let mut file = File::create(&temp_file)?;
        bincode::serialize_into(&mut file, state)?;
        file.sync_all()?;
        
        // Atomic rename
        std::fs::rename(&temp_file, &self.state_file)?;
        
        Ok(())
    }
}
```

### Checkpointing

```rust
pub struct CheckpointManager {
    checkpoints_dir: PathBuf,
    max_checkpoints: usize,
}

impl CheckpointManager {
    pub fn create_checkpoint<T: Serialize>(
        &self,
        name: &str,
        state: &T
    ) -> Result<PathBuf, Error> {
        let checkpoint_path = self.checkpoints_dir
            .join(format!("{}_{}.checkpoint", name, timestamp()));
        
        let temp_path = checkpoint_path.with_extension("tmp");
        
        // Serialize with compression
        let data = bincode::serialize(state)?;
        let compressed = zstd::encode_all(&data[..], 3)?;
        
        std::fs::write(&temp_path, compressed)?;
        std::fs::rename(&temp_path, &checkpoint_path)?;
        
        // Cleanup old checkpoints
        self.cleanup_old_checkpoints(name)?;
        
        Ok(checkpoint_path)
    }
    
    fn cleanup_old_checkpoints(&self, name: &str) -> Result<(), Error> {
        let mut checkpoints: Vec<_> = std::fs::read_dir(&self.checkpoints_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with(name)
            })
            .collect();
        
        // Sort by modification time
        checkpoints.sort_by_key(|e| {
            e.metadata().unwrap().modified().unwrap()
        });
        
        // Remove oldest if exceeding max
        while checkpoints.len() > self.max_checkpoints {
            if let Some(oldest) = checkpoints.first() {
                std::fs::remove_file(oldest.path())?;
                checkpoints.remove(0);
            }
        }
        
        Ok(())
    }
}
```

---

## External Format Support

### GraphML

```rust
pub mod graphml {
    use quick_xml::{Writer, events::*};
    
    pub fn export(model: &FactorGraph) -> String {
        let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
        
        // XML header
        writer.write_event(Event::Decl(
            BytesDecl::new("1.0", Some("UTF-8"), None)
        ));
        
        // GraphML root
        writer.write_event(Event::Start(BytesStart::new("graphml")));
        writer.write_event(Event::Start(BytesStart::new("graph")));
        
        // Nodes
        for node in model.nodes() {
            writer.write_event(Event::Start(
                BytesStart::new("node").with_attributes(vec![
                    ("id", &node.id.to_string()),
                ])
            ));
            
            // Add data elements for node properties
            writer.write_event(Event::Start(BytesStart::new("data")));
            writer.write_event(Event::Text(BytesText::new(&node.name)));
            writer.write_event(Event::End(BytesEnd::new("data")));
            
            writer.write_event(Event::End(BytesEnd::new("node")));
        }
        
        // Edges
        for edge in model.edges() {
            writer.write_event(Event::Start(
                BytesStart::new("edge").with_attributes(vec![
                    ("source", &edge.source.to_string()),
                    ("target", &edge.target.to_string()),
                ])
            ));
            writer.write_event(Event::End(BytesEnd::new("edge")));
        }
        
        writer.write_event(Event::End(BytesEnd::new("graph")));
        writer.write_event(Event::End(BytesEnd::new("graphml")));
        
        String::from_utf8(writer.into_inner()).unwrap()
    }
}
```

### GEXF

```rust
pub mod gexf {
    pub fn export(model: &FactorGraph) -> String {
        format!(r##"<?xml version="1.0" encoding="UTF-8"?>
<gexf xmlns="http://www.gexf.net/1.3" version="1.3">
  <graph defaultedgetype="{}">
    <nodes>
{}
    </nodes>
    <edges>
{}
    </edges>
  </graph>
</gexf>"##,
            if model.is_directed() { "directed" } else { "undirected" },
            model.nodes().map(|n| format!(r##"      <node id="{}" label="{}"/>"##, n.id, n.name)).join("\n"),
            model.edges().enumerate().map(|(i, e)| format!(r##"      <edge id="{}" source="{}" target="{}"/>"##, i, e.source, e.target)).join("\n")
        )
    }
}
```

### NetworkX Integration

```rust
impl BayesianNetwork {
    /// Export to Python NetworkX format via PyO3
    #[cfg(feature = "python")]
    pub fn to_networkx<'py>(&self, py: Python<'py>) -> PyResult<&'py PyObject> {
        let nx = py.import("networkx")?;
        let graph = nx.call_method0("DiGraph")?;
        
        // Add nodes with attributes
        for node in self.nodes() {
            let cpd = self.get_cpd(node);
            let attrs = [
                ("states", node.domain.states()),
                ("has_cpd", cpd.is_some()),
            ].into_py_dict(py);
            
            graph.call_method1("add_node", (node.name.clone(), attrs))?;
        }
        
        // Add edges
        for edge in self.edges() {
            graph.call_method1("add_edge", (&edge.source.name, &edge.target.name))?;
        }
        
        Ok(graph)
    }
    
    #[cfg(feature = "python")]
    pub fn from_networkx(graph: &PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            let mut model = BayesianNetwork::new();
            
            // Extract nodes
            let nodes: Vec<String> = graph.call_method0(py, "nodes")?.extract(py)?;
            for node in nodes {
                model.add_node(&node);
            }
            
            // Extract edges
            let edges: Vec<(String, String)> = graph.call_method0(py, "edges")?.extract(py)?;
            for (u, v) in edges {
                model.add_edge(&u, &v)?;
            }
            
            Ok(model)
        })
    }
}
```

### Cytoscape Format

```rust
pub mod cytoscape {
    use serde_json::json;
    
    pub fn export(model: &FactorGraph) -> serde_json::Value {
        let elements = json!({
            "nodes": model.nodes().map(|n| {
                json!({
                    "data": {
                        "id": n.id.to_string(),
                        "name": n.name,
                    }
                })
            }).collect::<Vec<_>>(),
            "edges": model.edges().enumerate().map(|(i, e)| {
                json!({
                    "data": {
                        "id": format!("e{}", i),
                        "source": e.source.to_string(),
                        "target": e.target.to_string(),
                    }
                })
            }).collect::<Vec<_>>(),
        });
        
        elements
    }
}
```

---

## Reproducibility Guarantees

### What Lutufi Guarantees

Lutufi provides the following reproducibility guarantees:

1. **Bit-for-Bit Reproducibility**: Same inputs, same random seed → identical results
2. **Cross-Platform Consistency**: Results identical across supported platforms
3. **Version Stability**: Results stable across patch versions
4. **Deterministic Algorithms**: All stochastic methods use controllable RNGs

```rust
pub struct ReproducibilityGuarantee;

impl ReproducibilityGuarantee {
    /// Verify that two inferences produce identical results
    pub fn verify_bitwise_reproducibility(
        model: &FactorGraph,
        query: &Query,
        seed: u64
    ) -> bool {
        let result1 = model.query_with_seed(query, seed);
        let result2 = model.query_with_seed(query, seed);
        
        result1 == result2
    }
    
    /// Verify cross-platform consistency
    pub fn verify_cross_platform(
        model_bytes: &[u8],
        expected_hash: &str
    ) -> Result<bool, Error> {
        let model = FactorGraph::from_bytes(model_bytes)?;
        let result = model.query(&[VariableId(0)], &EvidenceSet::new())?;
        
        let actual_hash = compute_result_hash(&result);
        Ok(actual_hash == expected_hash)
    }
}
```

### Random Seed Handling

```rust
pub struct ReproducibleRng {
    rng: Xoshiro256PlusPlus,
    seed: u64,
    stream_position: u64,
}

impl ReproducibleRng {
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: Xoshiro256PlusPlus::seed_from_u64(seed),
            seed,
            stream_position: 0,
        }
    }
    
    pub fn next_f64(&mut self) -> f64 {
        self.stream_position += 1;
        self.rng.gen()
    }
    
    pub fn state(&self) -> RngState {
        RngState {
            algorithm: "xoshiro256++".to_string(),
            seed: self.seed,
            stream_position: self.stream_position,
            state_bytes: self.rng.serialize(),
        }
    }
    
    pub fn restore(state: &RngState) -> Result<Self, Error> {
        if state.algorithm != "xoshiro256++" {
            return Err(Error::UnsupportedRngAlgorithm(state.algorithm.clone()));
        }
        
        Ok(Self {
            rng: Xoshiro256PlusPlus::deserialize(&state.state_bytes)?,
            seed: state.seed,
            stream_position: state.stream_position,
        })
    }
}
```

### Deterministic Algorithms

```rust
pub trait DeterministicAlgorithm {
    /// Run with full reproducibility guarantees
    fn run_deterministic(&self, rng: &mut ReproducibleRng) -> Result<Self::Output, Error>;
}

impl DeterministicAlgorithm for GibbsSampler {
    fn run_deterministic(&self, rng: &mut ReproducibleRng) -> Result<InferenceResult, Error> {
        // Use seeded RNG for all random choices
        let mut samples = Vec::with_capacity(self.n_samples);
        
        for _ in 0..self.n_samples {
            let sample = self.gibbs_step(rng)?;
            samples.push(sample);
        }
        
        Ok(InferenceResult::from_samples(samples))
    }
}
```

### Environment Capture

```rust
#[derive(Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Lutufi version
    pub lutufi_version: String,
    
    /// Rust version used to compile
    pub rust_version: String,
    
    /// Target architecture
    pub target_arch: String,
    
    /// Target OS
    pub target_os: String,
    
    /// CPU features available
    pub cpu_features: Vec<String>,
    
    /// Compilation profile (debug/release)
    pub profile: String,
    
    /// Dependency versions
    pub dependencies: HashMap<String, String>,
}

impl EnvironmentInfo {
    pub fn capture() -> Self {
        Self {
            lutufi_version: env!("CARGO_PKG_VERSION").to_string(),
            rust_version: rustc_version_runtime::version().to_string(),
            target_arch: std::env::consts::ARCH.to_string(),
            target_os: std::env::consts::OS.to_string(),
            cpu_features: Self::detect_cpu_features(),
            profile: if cfg!(debug_assertions) { "debug" } else { "release" }.to_string(),
            dependencies: Self::capture_dependencies(),
        }
    }
}
```

---

## Version Migration

### Upgrading Old Files

```rust
pub struct VersionUpgrader {
    migrations: Vec<Box<dyn Migration>>,
}

impl VersionUpgrader {
    pub fn upgrade_to_current(
        &self,
        data: &[u8],
        from_version: Version
    ) -> Result<Vec<u8>, MigrationError> {
        let current = Version::current();
        
        if from_version == current {
            return Ok(data.to_vec());
        }
        
        if from_version > current {
            return Err(MigrationError::NewerVersion {
                file_version: from_version,
                library_version: current,
            });
        }
        
        // Find and apply migrations
        let mut current_data = data.to_vec();
        let mut current_ver = from_version;
        
        while current_ver < current {
            let migration = self.find_migration(current_ver)
                .ok_or(MigrationError::NoMigrationPath(current_ver))?;
            
            current_data = migration.migrate(&current_data)?;
            current_ver = migration.to_version();
        }
        
        Ok(current_data)
    }
}
```

### Deprecation Warnings

```rust
pub struct DeprecationHandler {
    deprecated_formats: HashMap<Version, DeprecationInfo>,
}

impl DeprecationHandler {
    pub fn check_format_version(&self, version: Version) -> Result<(), Warning> {
        if let Some(info) = self.deprecated_formats.get(&version) {
            let warning = Warning::DeprecatedFormat {
                version,
                removal_date: info.removal_date,
                migration_command: info.migration_command.clone(),
            };
            
            if Utc::now() > info.removal_date {
                return Err(warning.into_error());
            } else {
                log::warn!("{}", warning);
            }
        }
        
        Ok(())
    }
}
```

### Automatic Conversion Tools

```rust
pub struct ConversionTool;

impl ConversionTool {
    pub fn convert_file(
        input: &Path,
        output: &Path,
        target_version: Version
    ) -> Result<(), ConversionError> {
        // Read input
        let data = std::fs::read(input)?;
        
        // Detect version
        let source_version = Self::detect_version(&data)?;
        
        // Convert
        let converted = VersionUpgrader::new()
            .upgrade_to_current(&data, source_version)?;
        
        // Write output
        std::fs::write(output, converted)?;
        
        println!("Converted {} from v{} to v{}",
            input.display(), source_version, target_version);
        
        Ok(())
    }
    
    pub fn batch_convert(
        input_dir: &Path,
        output_dir: &Path,
        pattern: &str
    ) -> Result<BatchConversionReport, ConversionError> {
        let entries: Vec<_> = glob::glob(&format!("{}/{}", input_dir.display(), pattern))?
            .filter_map(|e| e.ok())
            .collect();
        
        let mut report = BatchConversionReport::new();
        
        for entry in entries {
            let output = output_dir.join(entry.file_name().unwrap());
            
            match Self::convert_file(&entry, &output, Version::current()) {
                Ok(_) => report.add_success(entry),
                Err(e) => report.add_failure(entry, e),
            }
        }
        
        Ok(report)
    }
}
```

---

## Digital Signatures

### Optional Signing for Integrity

```rust
use ed25519_dalek::{Keypair, Signer, Verifier, Signature};

pub struct ModelSigner {
    keypair: Option<Keypair>,
}

impl ModelSigner {
    pub fn new_with_keypair(keypair: Keypair) -> Self {
        Self { keypair: Some(keypair) }
    }
    
    pub fn sign_model(&self, model_data: &[u8]) -> Result<SignedModel, SignError> {
        let keypair = self.keypair.as_ref()
            .ok_or(SignError::NoKeypair)?;
        
        // Hash the model data
        let hash = blake3::hash(model_data);
        
        // Sign the hash
        let signature = keypair.sign(hash.as_bytes());
        
        Ok(SignedModel {
            data: model_data.to_vec(),
            signature: signature.to_bytes().to_vec(),
            public_key: keypair.public.to_bytes().to_vec(),
            hash: hash.to_string(),
        })
    }
    
    pub fn verify_model(&self, signed_model: &SignedModel) -> Result<bool, VerifyError> {
        let public_key = ed25519_dalek::PublicKey::from_bytes(&signed_model.public_key)
            .map_err(|_| VerifyError::InvalidPublicKey)?;
        
        let signature = Signature::from_bytes(&signed_model.signature)
            .map_err(|_| VerifyError::InvalidSignature)?;
        
        let hash = blake3::hash(&signed_model.data);
        
        public_key.verify(hash.as_bytes(), &signature)
            .map_err(|_| VerifyError::VerificationFailed)?;
        
        // Also verify hash matches
        if hash.to_string() != signed_model.hash {
            return Ok(false);
        }
        
        Ok(true)
    }
}
```

### Authentication Use Cases

```rust
pub struct AuthenticatedModel {
    model: FactorGraph,
    signature: Option<Signature>,
    certificate: Option<Certificate>,
}

impl AuthenticatedModel {
    /// Verify model came from trusted source
    pub fn verify_trust(&self, trust_store: &TrustStore) -> Result<TrustLevel, TrustError> {
        let sig = self.signature.as_ref()
            .ok_or(TrustError::NotSigned)?;
        
        // Find certificate for this signature
        let cert = self.certificate.as_ref()
            .ok_or(TrustError::NoCertificate)?;
        
        // Verify certificate chain
        trust_store.verify_chain(cert)?;
        
        // Verify signature
        let public_key = cert.public_key();
        if !self.verify_signature(sig, public_key) {
            return Err(TrustError::InvalidSignature);
        }
        
        // Return trust level based on certificate
        Ok(trust_store.trust_level(cert))
    }
}
```

---

## Archive Formats

### Bundling Models with Data

```rust
pub struct ModelArchive {
    model: FactorGraph,
    training_data: Option<DataFrame>,
    test_data: Option<DataFrame>,
    metadata: ArchiveMetadata,
    provenance: ProvenanceRecord,
}

impl ModelArchive {
    pub fn create(
        model: FactorGraph,
        training_data: Option<DataFrame>,
    ) -> Self {
        Self {
            model,
            training_data,
            test_data: None,
            metadata: ArchiveMetadata::new(),
            provenance: ProvenanceRecord::capture(),
        }
    }
    
    pub fn save(&self, path: &Path) -> Result<(), ArchiveError> {
        let archive = tar::Builder::new(File::create(path)?);
        
        // Add model
        let model_bytes = self.model.to_bytes()?;
        archive.append_file("model.lut", &mut model_bytes.as_slice())?;
        
        // Add data if present
        if let Some(data) = &self.training_data {
            let data_bytes = data.to_parquet()?;
            archive.append_file("training_data.parquet", &mut data_bytes.as_slice())?;
        }
        
        // Add metadata
        let metadata_json = serde_json::to_vec(&self.metadata)?;
        archive.append_file("metadata.json", &mut metadata_json.as_slice())?;
        
        // Add provenance
        let provenance_json = serde_json::to_vec(&self.provenance)?;
        archive.append_file("provenance.json", &mut provenance_json.as_slice())?;
        
        Ok(())
    }
    
    pub fn load(path: &Path) -> Result<Self, ArchiveError> {
        let archive = tar::Archive::new(File::open(path)?);
        
        let mut model = None;
        let mut training_data = None;
        let mut metadata = None;
        
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            
            match path.file_name().unwrap().to_str().unwrap() {
                "model.lut" => {
                    let mut bytes = Vec::new();
                    entry.read_to_end(&mut bytes)?;
                    model = Some(FactorGraph::from_bytes(&bytes)?);
                }
                "training_data.parquet" => {
                    let mut bytes = Vec::new();
                    entry.read_to_end(&mut bytes)?;
                    training_data = Some(DataFrame::from_parquet(&bytes)?);
                }
                "metadata.json" => {
                    let mut bytes = Vec::new();
                    entry.read_to_end(&mut bytes)?;
                    metadata = Some(serde_json::from_slice(&bytes)?);
                }
                _ => {}
            }
        }
        
        Ok(Self {
            model: model.ok_or(ArchiveError::MissingModel)?,
            training_data,
            metadata: metadata.ok_or(ArchiveError::MissingMetadata)?,
            provenance: ProvenanceRecord::new(),
            test_data: None,
        })
    }
}
```

### Self-Contained Research Artifacts

```rust
pub struct ResearchArtifact {
    archive: ModelArchive,
    notebook: Option<String>,
    documentation: String,
    requirements: Vec<String>,
}

impl ResearchArtifact {
    pub fn create_complete(
        model: FactorGraph,
        notebook: &str,
    ) -> Result<Self, ArtifactError> {
        Ok(Self {
            archive: ModelArchive::create(model, None),
            notebook: Some(notebook.to_string()),
            documentation: Self::generate_documentation(),
            requirements: Self::capture_requirements(),
        })
    }
    
    pub fn verify_reproducibility(&self) -> ReproducibilityReport {
        let mut report = ReproducibilityReport::new();
        
        // Check all components present
        if self.notebook.is_none() {
            report.add_issue("No notebook provided for reproduction");
        }
        
        // Check versions match
        let current_version = env!("CARGO_PKG_VERSION");
        if self.archive.metadata.lutufi_version != current_version {
            report.add_warning(format!(
                "Artifact created with Lutufi {}, current version is {}",
                self.archive.metadata.lutufi_version,
                current_version
            ));
        }
        
        report
    }
}
```

---

## How Lutufi Ensures Reproducibility

### Implementation Patterns

**1. Versioned Serialization:**
```rust
impl Serializable for FactorGraph {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let versioned = VersionedData {
            version: CURRENT_VERSION,
            data: self,
        };
        bincode::serialize(&versioned)
    }
}
```

**2. Deterministic Ordering:**
```rust
impl FactorGraph {
    pub fn to_deterministic_bytes(&self) -> Vec<u8> {
        // Sort all collections before serialization
        let mut nodes: Vec<_> = self.nodes().collect();
        nodes.sort_by_key(|n| n.id);
        
        let mut edges: Vec<_> = self.edges().collect();
        edges.sort_by(|a, b| (a.source, a.target).cmp(&(b.source, b.target)));
        
        // Serialize in deterministic order
        serialize_ordered(&nodes, &edges)
    }
}
```

**3. Full State Capture:**
```rust
impl Checkpoint {
    pub fn capture_full_state(inference: &InferenceEngine) -> Self {
        Self {
            model: inference.model().clone(),
            algorithm_state: inference.algorithm_state(),
            rng_state: inference.rng().state(),
            iteration: inference.iteration(),
            timestamp: Utc::now(),
        }
    }
}
```

### Reproducibility Checklist

```rust
pub struct ReproducibilityChecklist;

impl ReproducibilityChecklist {
    pub fn verify() -> ChecklistResult {
        let mut result = ChecklistResult::new();
        
        // Check 1: Random seeds
        result.check("Random seeds captured", || {
            // Implementation
        });
        
        // Check 2: Algorithm determinism
        result.check("Algorithms deterministic", || {
            // Implementation
        });
        
        // Check 3: Cross-platform consistency
        result.check("Cross-platform results match", || {
            // Implementation
        });
        
        // Check 4: Version stability
        result.check("Patch version stability", || {
            // Implementation
        });
        
        result
    }
}
```

---

## Key References

### Serialization Patterns

1. **Google Protocol Buffers.** https://developers.google.com/protocol-buffers
   - Binary serialization format design
   - Schema evolution patterns

2. **Apache Avro.** https://avro.apache.org/
   - Schema evolution in binary formats
   - Self-describing data

3. **Cap'n Proto.** https://capnproto.org/
   - Zero-copy serialization
   - Capability-based security

4. **Postel, J. (1980).** "RFC 760: DoD Standard Internet Protocol."
   - "Be conservative in what you send, liberal in what you accept"

### Data Longevity

5. **Rothenberg, J. (1995).** "Ensuring the Longevity of Digital Documents." *Scientific American,* 272(1), 42-47.
   - Emulation vs. migration strategies

6. **Lorie, R. A. (2001).** "Long-Term Preservation of Digital Information." *ACM International Conference on Digital Libraries.*
   - Preservation metadata

7. **Rosenthal, D. S. H. (2010).** "Format Obsolescence: Scenarios." https://blog.dshr.org/2010/10/format-obsolescence-scenarios.html
   - Practical format longevity considerations

### Scientific Reproducibility

8. **Stodden, V. (2009).** "The Legal Framework for Reproducible Scientific Research." *Computing in Science & Engineering,* 11(1), 35-40.
   - Legal and practical aspects of reproducibility

9. **Peng, R. D. (2011).** "Reproducible Research in Computational Science." *Science,* 334(6060), 1226-1227.
   - Reproducibility in computational fields

10. **Goodman, S. N., Fanelli, D., & Ioannidis, J. P. (2016).** "What Does Research Reproducibility Mean?" *Science Translational Medicine,* 8(341), 341ps12.
    - Definitions and standards for reproducibility

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-01 | Wasswa Lutufi Sebbanja | Initial draft |
| 1.0 | 2026-03-03 | Wasswa Lutufi Sebbanja | Complete serialization document |

---

*This document is part of the Lutufi project documentation. For questions or contributions, please refer to the project's contribution guidelines.*
