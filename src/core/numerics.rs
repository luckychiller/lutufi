use serde::{Deserialize, Serialize};

pub const PROBABILITY_TOLERANCE: f64 = 1e-9;
pub const LOG_PROBABILITY_TOLERANCE: f64 = 1e-7;
pub const CPT_NORMALIZATION_TOLERANCE: f64 = 1e-9;
pub const CONVERGENCE_TOLERANCE: f64 = 1e-8;
pub const SPARSE_DENSITY_THRESHOLD: f64 = 0.3;
pub const UNDERFLOW_THRESHOLD: f64 = 1e-12;

pub fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() <= tolerance
}

pub fn is_valid_probability(p: f64) -> bool {
    p >= 0.0 && p <= 1.0 + PROBABILITY_TOLERANCE
}

pub fn softmax(log_values: &[f64]) -> Vec<f64> {
    let max = log_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exps: Vec<f64> = log_values.iter().map(|&v| (v - max).exp()).collect();
    let sum: f64 = exps.iter().sum();
    if sum <= 0.0 {
        return vec![0.0; log_values.len()];
    }
    exps.iter().map(|e| e / sum).collect()
}

pub fn log_softmax(log_values: &[f64]) -> Vec<f64> {
    let max = log_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let log_sum = max + log_values.iter().map(|&v| (v - max).exp()).sum::<f64>().ln();
    log_values.iter().map(|&v| v - log_sum).collect()
}

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
    Dense { data: Vec<f64> },
    SparseCoo { indices: Vec<usize>, data: Vec<f64> },
}

impl SparseFactorFormat {
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
    pub max_memory_mb: usize,
    pub max_inference_time_secs: usize,
    pub max_nodes: usize,
    pub max_edges: usize,
    pub max_cpt_size: usize,
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

fn num_cpus() -> usize {
    std::env::var("LUTUFI_NUM_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4))
}
