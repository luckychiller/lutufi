use crate::core::{
    error::{LutufiError, LutufiResult},
    variable::VariableId,
};

/// Method used for data reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconstructionMethod {
    /// Matrix factorization (SVD-based).
    MatrixFactorization,
    /// Low-rank approximation.
    LowRank,
    /// Iterative reconstruction (EM-like).
    Iterative,
    /// Nearest-neighbor hot deck imputation.
    HotDeck,
}

/// Result of a reconstruction operation.
#[derive(Debug, Clone)]
pub struct ReconstructionResult {
    /// Reconstructed data matrix.
    pub reconstructed: Vec<Vec<f64>>,
    /// Method used.
    pub method: ReconstructionMethod,
    /// Number of iterations (if iterative).
    pub iterations: usize,
    /// Final reconstruction error (Frobenius norm of residuals).
    pub reconstruction_error: f64,
    /// Whether the reconstruction converged.
    pub converged: bool,
}

/// Engine for reconstructing data from incomplete observations.
#[derive(Debug, Clone)]
pub struct ReconstructionEngine {
    /// The reconstruction method.
    method: ReconstructionMethod,
    /// Target rank for low-rank methods.
    rank: usize,
    /// Maximum iterations.
    max_iterations: usize,
    /// Convergence tolerance.
    tolerance: f64,
}

impl ReconstructionEngine {
    /// Create a new reconstruction engine.
    pub fn new(method: ReconstructionMethod) -> Self {
        ReconstructionEngine {
            method,
            rank: 2,
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }

    /// Set the target rank for low-rank methods.
    pub fn with_rank(mut self, rank: usize) -> Self {
        self.rank = rank.max(1);
        self
    }

    /// Set maximum iterations.
    pub fn with_max_iterations(mut self, max_iter: usize) -> Self {
        self.max_iterations = max_iter;
        self
    }

    /// Set convergence tolerance.
    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    /// Reconstruct the full data matrix from incomplete observations.
    ///
    /// # Arguments
    /// * `data` - Data matrix with `None` for missing entries.
    /// * `variables` - Variable IDs for columns.
    pub fn reconstruct(
        &self,
        data: &[Vec<Option<f64>>],
        variables: &[VariableId],
    ) -> LutufiResult<ReconstructionResult> {
        match self.method {
            ReconstructionMethod::MatrixFactorization => {
                self.reconstruct_svd(data, variables)
            }
            ReconstructionMethod::LowRank => {
                self.reconstruct_low_rank(data, variables)
            }
            ReconstructionMethod::Iterative => {
                self.reconstruct_iterative(data, variables)
            }
            ReconstructionMethod::HotDeck => {
                self.reconstruct_hot_deck(data, variables)
            }
        }
    }

    /// SVD-based matrix factorization reconstruction.
    fn reconstruct_svd(
        &self,
        data: &[Vec<Option<f64>>],
        _variables: &[VariableId],
    ) -> LutufiResult<ReconstructionResult> {
        let n_rows = data.len();
        let n_cols = if n_rows > 0 { data[0].len() } else { 0 };
        if n_rows == 0 || n_cols == 0 {
            return Err(LutufiError::InternalError {
                message: "Empty data matrix".to_string(),
            });
        }

        let k = self.rank.min(n_rows.min(n_cols));

        // Initialize with mean imputation.
        let mut matrix = vec![vec![0.0_f64; n_cols]; n_rows];
        let mut col_means = vec![0.0_f64; n_cols];
        let mut col_counts = vec![0_usize; n_cols];

        for i in 0..n_rows {
            for j in 0..n_cols {
                if let Some(v) = data[i][j] {
                    matrix[i][j] = v;
                    col_means[j] += v;
                    col_counts[j] += 1;
                }
            }
        }
        for j in 0..n_cols {
            if col_counts[j] > 0 {
                col_means[j] /= col_counts[j] as f64;
            }
        }
        for i in 0..n_rows {
            for j in 0..n_cols {
                if data[i][j].is_none() {
                    matrix[i][j] = col_means[j];
                }
            }
        }

        // Simple iterative SVD imputation via alternating least squares.
        let mut prev_error = f64::INFINITY;
        let mut iterations = 0;
        let mut converged = false;

        for iter in 0..self.max_iterations {
            iterations = iter + 1;

            // Center the matrix.
            for j in 0..n_cols {
                let mean: f64 = (0..n_rows).map(|i| matrix[i][j]).sum::<f64>() / n_rows as f64;
                for i in 0..n_rows {
                    matrix[i][j] -= mean;
                }
            }

            // Approximate SVD via power iteration.
            // U (n_rows x k), V (n_cols x k)
            let mut u = vec![vec![0.0_f64; k]; n_rows];
            let mut v = vec![vec![0.0_f64; k]; n_cols];

            // Initialize V with small random values.
            for j in 0..n_cols {
                for r in 0..k {
                    v[j][r] = (j as f64 * 0.01 + r as f64 * 0.001).sin();
                }
            }

            // Power iteration for each component.
            for r in 0..k {
                for _it in 0..10 {
                    // U = M * V_r
                    for i in 0..n_rows {
                        u[i][r] = 0.0;
                        for j in 0..n_cols {
                            u[i][r] += matrix[i][j] * v[j][r];
                        }
                    }
                    // Normalize U.
                    let norm_u: f64 = (0..n_rows).map(|i| u[i][r] * u[i][r]).sum::<f64>().sqrt();
                    if norm_u > 1e-15 {
                        for i in 0..n_rows {
                            u[i][r] /= norm_u;
                        }
                    }
                    // V = M^T * U
                    for j in 0..n_cols {
                        v[j][r] = 0.0;
                        for i in 0..n_rows {
                            v[j][r] += matrix[i][j] * u[i][r];
                        }
                    }
                    let norm_v: f64 = (0..n_cols).map(|j| v[j][r] * v[j][r]).sum::<f64>().sqrt();
                    if norm_v > 1e-15 {
                        for j in 0..n_cols {
                            v[j][r] /= norm_v;
                        }
                    }
                }
            }

            // Reconstruct: update missing entries only.
            let mut error = 0.0_f64;
            let mut _missing_count = 0_usize;

            for i in 0..n_rows {
                for j in 0..n_cols {
                    let reconstructed = (0..k).map(|r| u[i][r] * v[j][r]).sum::<f64>();
                    if data[i][j].is_none() {
                        matrix[i][j] = reconstructed;
                        _missing_count += 1;
                    }
                    let diff = matrix[i][j] - reconstructed;
                    error += diff * diff;
                }
            }

            if (prev_error - error).abs() < self.tolerance {
                converged = true;
                break;
            }
            prev_error = error;
        }

        Ok(ReconstructionResult {
            reconstructed: matrix,
            method: ReconstructionMethod::MatrixFactorization,
            iterations,
            reconstruction_error: prev_error.sqrt(),
            converged,
        })
    }

    /// Low-rank approximation reconstruction.
    fn reconstruct_low_rank(
        &self,
        data: &[Vec<Option<f64>>],
        variables: &[VariableId],
    ) -> LutufiResult<ReconstructionResult> {
        // Delegate to SVD with the specified rank.
        self.reconstruct_svd(data, variables)
    }

    /// Iterative reconstruction (EM-like) using weighted least squares.
    fn reconstruct_iterative(
        &self,
        data: &[Vec<Option<f64>>],
        _variables: &[VariableId],
    ) -> LutufiResult<ReconstructionResult> {
        let n_rows = data.len();
        let n_cols = if n_rows > 0 { data[0].len() } else { 0 };

        let mut matrix = vec![vec![0.0_f64; n_cols]; n_rows];
        let mut missing_mask = vec![vec![false; n_cols]; n_rows];
        let mut col_means = vec![0.0_f64; n_cols];

        for i in 0..n_rows {
            for j in 0..n_cols {
                if let Some(v) = data[i][j] {
                    matrix[i][j] = v;
                    col_means[j] += v;
                } else {
                    missing_mask[i][j] = true;
                }
            }
        }
        for j in 0..n_cols {
            let observed: usize = (0..n_rows).filter(|&i| !missing_mask[i][j]).count();
            if observed > 0 {
                col_means[j] /= observed as f64;
            }
        }
        for i in 0..n_rows {
            for j in 0..n_cols {
                if missing_mask[i][j] {
                    matrix[i][j] = col_means[j];
                }
            }
        }

        let mut prev_error = f64::INFINITY;
        let mut iterations = 0;
        let mut converged = false;

        for iter in 0..self.max_iterations {
            iterations = iter + 1;

            // E-like step: estimate row and column effects.
            let row_effects: Vec<f64> = (0..n_rows).map(|i| {
                let total: f64 = (0..n_cols).map(|j| matrix[i][j] - col_means[j]).sum();
                total / n_cols as f64
            }).collect();

            let col_effects: Vec<f64> = (0..n_cols).map(|j| {
                let total: f64 = (0..n_rows).map(|i| matrix[i][j] - col_means[j] - row_effects[i]).sum();
                total / n_rows as f64
            }).collect();

            // M-like step: update missing values.
            let mut error = 0.0_f64;
            for i in 0..n_rows {
                for j in 0..n_cols {
                    if missing_mask[i][j] {
                        let new_val = col_means[j] + row_effects[i] + col_effects[j];
                        error += (matrix[i][j] - new_val).powi(2);
                        matrix[i][j] = new_val;
                    }
                }
            }

            if (prev_error - error).abs() < self.tolerance {
                converged = true;
                break;
            }
            prev_error = error;
        }

        Ok(ReconstructionResult {
            reconstructed: matrix,
            method: ReconstructionMethod::Iterative,
            iterations,
            reconstruction_error: prev_error.sqrt(),
            converged,
        })
    }

    /// Hot deck imputation: replace missing values with observed values from similar rows.
    fn reconstruct_hot_deck(
        &self,
        data: &[Vec<Option<f64>>],
        _variables: &[VariableId],
    ) -> LutufiResult<ReconstructionResult> {
        let n_rows = data.len();
        let n_cols = if n_rows > 0 { data[0].len() } else { 0 };
        let mut result = vec![vec![0.0_f64; n_cols]; n_rows];
        let mut missing_mask = vec![vec![false; n_cols]; n_rows];

        for i in 0..n_rows {
            for j in 0..n_cols {
                match data[i][j] {
                    Some(v) => result[i][j] = v,
                    None => missing_mask[i][j] = true,
                }
            }
        }

        for i in 0..n_rows {
            for j in 0..n_cols {
                if !missing_mask[i][j] {
                    continue;
                }

                // Find the most similar row (by Euclidean distance on observed columns).
                let mut best_dist = f64::INFINITY;
                let mut donor_val = 0.0_f64;
                let mut found = false;

                for k in 0..n_rows {
                    if k == i { continue; }
                    if data[k][j].is_none() { continue; }

                    let mut dist = 0.0_f64;
                    let mut shared_dims = 0_usize;
                    for q in 0..n_cols {
                        if q == j { continue; }
                        if let (Some(a), Some(b)) = (data[i][q], data[k][q]) {
                            dist += (a - b).powi(2);
                            shared_dims += 1;
                        }
                    }
                    if shared_dims == 0 { continue; }
                    dist = dist / shared_dims as f64;

                    if dist < best_dist {
                        best_dist = dist;
                        donor_val = data[k][j].unwrap_or(0.0);
                        found = true;
                    }
                }

                result[i][j] = if found { donor_val } else { 0.0 };
            }
        }

        Ok(ReconstructionResult {
            reconstructed: result,
            method: ReconstructionMethod::HotDeck,
            iterations: 1,
            reconstruction_error: 0.0,
            converged: true,
        })
    }
}
