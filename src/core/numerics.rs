use serde::{Deserialize, Serialize};

/// Tolerance for floating-point comparisons of probability values.
pub const PROBABILITY_TOLERANCE: f64 = 1e-9;
/// Tolerance for comparisons in log-probability space.
pub const LOG_PROBABILITY_TOLERANCE: f64 = 1e-7;
/// Tolerance for checking that conditional probability tables sum to one.
pub const CPT_NORMALIZATION_TOLERANCE: f64 = 1e-9;
/// Tolerance for convergence checks in iterative inference algorithms.
pub const CONVERGENCE_TOLERANCE: f64 = 1e-8;
/// Density threshold below which a factor table is stored sparsely.
pub const SPARSE_DENSITY_THRESHOLD: f64 = 0.3;
/// Threshold below which values are treated as zero to avoid underflow.
pub const UNDERFLOW_THRESHOLD: f64 = 1e-12;

/// Returns `true` if `a` and `b` are within `tolerance` of each other.
pub fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() <= tolerance
}

/// Returns `true` if `p` is a valid probability in `[0, 1]` within tolerance.
pub fn is_valid_probability(p: f64) -> bool {
    p >= 0.0 && p <= 1.0 + PROBABILITY_TOLERANCE
}

/// Numerically stable softmax: computes `exp(x_i) / sum(exp(x_j))`.
pub fn softmax(log_values: &[f64]) -> Vec<f64> {
    let max = log_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exps: Vec<f64> = log_values.iter().map(|&v| (v - max).exp()).collect();
    let sum: f64 = exps.iter().sum();
    if sum <= 0.0 {
        return vec![0.0; log_values.len()];
    }
    exps.iter().map(|e| e / sum).collect()
}

/// Numerically stable log-softmax: computes `log(softmax(x_i))`.
pub fn log_softmax(log_values: &[f64]) -> Vec<f64> {
    let max = log_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let log_sum = max + log_values.iter().map(|&v| (v - max).exp()).sum::<f64>().ln();
    log_values.iter().map(|&v| v - log_sum).collect()
}

/// Estimates the proportion of values above [`UNDERFLOW_THRESHOLD`].
pub fn estimate_density(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let non_zero = values.iter().filter(|&&v| v > UNDERFLOW_THRESHOLD).count();
    non_zero as f64 / values.len() as f64
}

/// Sparse storage format for factor tables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SparseFactorFormat {
    /// Stores all entries in a single contiguous vector.
    Dense {
        /// All factor values in a dense array.
        data: Vec<f64>,
    },
    /// Stores only non-zero entries with their indices (coordinate format).
    SparseCoo {
        /// Row-major indices of non-zero entries.
        indices: Vec<usize>,
        /// Values at the corresponding indices.
        data: Vec<f64>,
    },
}

impl SparseFactorFormat {
    /// Constructs a [`SparseFactorFormat`] from a dense slice,
    /// choosing sparse or dense storage based on [`estimate_density`].
    pub fn from_dense(values: &[f64]) -> Self {
        if estimate_density(values) < SPARSE_DENSITY_THRESHOLD {
            let indices: Vec<usize> = values.iter()
                .enumerate()
                .filter(|(_, &v)| v > UNDERFLOW_THRESHOLD)
                .map(|(i, _)| i)
                .collect();
            let data: Vec<f64> = indices.iter().map(|&i| values[i]).collect();
            SparseFactorFormat::SparseCoo { indices, data }
        } else {
            SparseFactorFormat::Dense { data: values.to_vec() }
        }
    }

    /// Reconstitutes the full dense vector of factor values.
    pub fn values(&self) -> Vec<f64> {
        match self {
            SparseFactorFormat::Dense { data } => data.clone(),
            SparseFactorFormat::SparseCoo { indices, data } => {
                let max_idx = indices.iter().max().copied().unwrap_or(0);
                let mut vals = vec![0.0; max_idx + 1];
                for (&i, &v) in indices.iter().zip(data.iter()) {
                    vals[i] = v;
                }
                vals
            }
        }
    }
}

/// Resource budget for inference operations.
#[derive(Debug, Clone)]
pub struct ResourceBudget {
    /// Maximum memory usage in megabytes.
    pub max_memory_mb: usize,
    /// Maximum wall-clock time for inference in seconds.
    pub max_inference_time_secs: usize,
    /// Maximum number of nodes in the model.
    pub max_nodes: usize,
    /// Maximum number of edges in the model.
    pub max_edges: usize,
    /// Maximum number of entries in a single CPT.
    pub max_cpt_size: usize,
    /// Number of threads to use for parallel operations.
    pub n_threads: usize,
}

impl Default for ResourceBudget {
    fn default() -> Self {
        Self {
            max_memory_mb: 4096,
            max_inference_time_secs: 3600,
            max_nodes: 100_000,
            max_edges: 1_000_000,
            max_cpt_size: 10_000_000,
            n_threads: std::cmp::min(num_cpus(), 8),
        }
    }
}

impl ResourceBudget {
    /// Returns an error if `count` exceeds [`max_nodes`](ResourceBudget::max_nodes).
    pub fn check_node_count(&self, count: usize) -> crate::core::error::LutufiResult<()> {
        if count > self.max_nodes {
            return Err(crate::core::error::LutufiError::ResourceLimitExceeded {
                resource: "nodes".to_string(),
                limit: format!("{}", self.max_nodes),
                requested: format!("{}", count),
                message: format!(
                    "Model has {} nodes, exceeding the configured maximum of {}",
                    count, self.max_nodes
                ),
            });
        }
        Ok(())
    }

    /// Returns an error if `count` exceeds [`max_edges`](ResourceBudget::max_edges).
    pub fn check_edge_count(&self, count: usize) -> crate::core::error::LutufiResult<()> {
        if count > self.max_edges {
            return Err(crate::core::error::LutufiError::ResourceLimitExceeded {
                resource: "edges".to_string(),
                limit: format!("{}", self.max_edges),
                requested: format!("{}", count),
                message: format!(
                    "Model has {} edges, exceeding the configured maximum of {}",
                    count, self.max_edges
                ),
            });
        }
        Ok(())
    }

    /// Returns an error if `size` exceeds [`max_cpt_size`](ResourceBudget::max_cpt_size).
    pub fn check_cpt_size(&self, size: usize) -> crate::core::error::LutufiResult<()> {
        if size > self.max_cpt_size {
            return Err(crate::core::error::LutufiError::ResourceLimitExceeded {
                resource: "cpt_size".to_string(),
                limit: format!("{}", self.max_cpt_size),
                requested: format!("{}", size),
                message: format!(
                    "CPT has {} entries, exceeding the configured maximum of {}",
                    size, self.max_cpt_size
                ),
            });
        }
        Ok(())
    }

    /// Returns an error if `bytes` exceeds [`max_memory_mb`](ResourceBudget::max_memory_mb).
    pub fn check_memory(&self, bytes: usize) -> crate::core::error::LutufiResult<()> {
        let max_bytes = self.max_memory_mb * 1024 * 1024;
        if bytes > max_bytes {
            return Err(crate::core::error::LutufiError::ResourceLimitExceeded {
                resource: "memory".to_string(),
                limit: format!("{} MB", self.max_memory_mb),
                requested: format!("{} MB", bytes / (1024 * 1024)),
                message: format!(
                    "Operation requires {} bytes ({} MB), exceeding the configured maximum of {} MB",
                    bytes,
                    bytes / (1024 * 1024),
                    self.max_memory_mb
                ),
            });
        }
        Ok(())
    }
}

fn num_cpus() -> usize {
    std::env::var("LUTUFI_NUM_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_limit_exceeded() {
        let budget = ResourceBudget {
            max_nodes: 5,
            ..Default::default()
        };
        assert!(budget.check_node_count(3).is_ok());
        assert!(budget.check_node_count(10).is_err());
    }

    #[test]
    fn test_edge_limit_exceeded() {
        let budget = ResourceBudget {
            max_edges: 100,
            ..Default::default()
        };
        assert!(budget.check_edge_count(50).is_ok());
        assert!(budget.check_edge_count(200).is_err());
    }

    #[test]
    fn test_cpt_size_limit_exceeded() {
        let budget = ResourceBudget {
            max_cpt_size: 1000,
            ..Default::default()
        };
        assert!(budget.check_cpt_size(500).is_ok());
        assert!(budget.check_cpt_size(5000).is_err());
    }

    #[test]
    fn test_memory_limit_exceeded() {
        let budget = ResourceBudget {
            max_memory_mb: 1,
            ..Default::default()
        };
        assert!(budget.check_memory(512 * 1024).is_ok());
        assert!(budget.check_memory(2 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_edge_case_exact_limit() {
        let budget = ResourceBudget {
            max_nodes: 10,
            ..Default::default()
        };
        assert!(budget.check_node_count(10).is_ok());
    }
}
