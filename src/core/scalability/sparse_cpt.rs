use serde::{Deserialize, Serialize};
use sprs::{CsMat, CsMatView, TriMat};
use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    numerics::{SPARSE_DENSITY_THRESHOLD, UNDERFLOW_THRESHOLD, estimate_density},
};

/// Sparse CPT storage using COO (construction) and CSR (computation) formats.
///
/// When factor density is below SPARSE_DENSITY_THRESHOLD (30%),
/// this representation stores only non-zero entries, significantly
/// reducing memory usage for large sparse CPTs common in social networks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseCptStorage {
    /// Variables in scope (sorted by ID for deterministic indexing).
    scope: Scope,
    /// Log-space values in COO format: (row_idx, col_idx) -> log_value.
    row_indices: Vec<usize>,
    col_indices: Vec<usize>,
    log_values: Vec<f64>,
    /// Number of rows (parent configurations).
    num_rows: usize,
    /// Number of columns (child states).
    num_cols: usize,
    /// Whether data is valid (non-empty).
    has_data: bool,
}

impl SparseCptStorage {
    /// Construct a new `SparseCptStorage` from a scope and dense values.
    /// Automatically selects COO storage when density is below the threshold
    /// or the number of entries exceeds 1,000,000.
    pub fn new(scope: Scope, values: Vec<f64>) -> LutufiResult<Self> {
        if values.len() != scope.num_entries() {
            return Err(LutufiError::CptWrongShape {
                variable: "SparseCpt".to_string(),
                expected_shape: scope.num_entries().to_string(),
                actual_shape: values.len().to_string(),
            });
        }

        let density = estimate_density(&values);
        let num_entries = values.len();

        if density < SPARSE_DENSITY_THRESHOLD || num_entries > 1_000_000 {
            let mut row_indices = Vec::new();
            let mut col_indices = Vec::new();
            let mut log_values = Vec::new();
            for (i, &v) in values.iter().enumerate() {
                if v > UNDERFLOW_THRESHOLD {
                    row_indices.push(i);
                    col_indices.push(0);
                    log_values.push(v.ln());
                }
            }
            Ok(SparseCptStorage {
                scope,
                row_indices,
                col_indices,
                log_values,
                num_rows: num_entries,
                num_cols: 1,
                has_data: true,
            })
        } else {
            let log_table: Vec<f64> = values.iter()
                .map(|&v| if v < UNDERFLOW_THRESHOLD { f64::NEG_INFINITY } else { v.ln() })
                .collect();
            let row_indices: Vec<usize> = (0..num_entries).collect();
            let col_indices = vec![0; num_entries];
            Ok(SparseCptStorage {
                scope,
                row_indices,
                col_indices,
                log_values: log_table,
                num_rows: num_entries,
                num_cols: 1,
                has_data: true,
            })
        }
    }

    /// Build a sparse CPT from an existing `TabularFactor`, discarding near-zero entries.
    pub fn from_tabular_factor(factor: &TabularFactor) -> Self {
        let scope = factor.scope().clone();
        let num_entries = scope.num_entries();
        let mut row_indices = Vec::new();
        let mut col_indices = Vec::new();
        let mut log_values = Vec::new();

        for i in 0..num_entries {
            let lv = factor.log_value_at(i);
            if lv > f64::NEG_INFINITY && (lv.exp()) > UNDERFLOW_THRESHOLD {
                row_indices.push(i);
                col_indices.push(0);
                log_values.push(lv);
            }
        }

        if row_indices.is_empty() {
            row_indices = (0..num_entries).collect();
            col_indices = vec![0; num_entries];
            log_values = vec![f64::NEG_INFINITY; num_entries];
        }

        SparseCptStorage {
            scope,
            row_indices,
            col_indices,
            log_values,
            num_rows: num_entries,
            num_cols: 1,
            has_data: true,
        }
    }

    /// Returns a reference to the variable scope of this CPT.
    pub fn scope(&self) -> &Scope { &self.scope }

    /// Look up the log-probability at a flat index. Returns `NEG_INFINITY` for zero entries.
    pub fn log_value_at(&self, index: usize) -> f64 {
        if index >= self.num_rows {
            return f64::NEG_INFINITY;
        }
        self.row_indices.iter()
            .position(|&r| r == index)
            .map(|i| self.log_values[i])
            .unwrap_or(f64::NEG_INFINITY)
    }

    /// Total number of entries (including zeros) in the full CPT.
    pub fn num_entries(&self) -> usize { self.num_rows }
    /// Number of stored non-zero entries.
    pub fn num_nonzeros(&self) -> usize { self.log_values.len() }
    /// Ratio of non-zero entries to total entries.
    pub fn density(&self) -> f64 {
        if self.num_rows == 0 { 0.0 } else { self.num_nonzeros() as f64 / self.num_rows as f64 }
    }

    /// Attempt to return a CSR view (currently panics; use `to_csr_owned` instead).
    pub fn to_csr(&self) -> CsMatView<f64> {
        unreachable!("Use to_csr_owned instead");
    }

    /// Convert to an owned CSR sparse matrix by exponentiating log-values.
    pub fn to_csr_owned(&self) -> CsMat<f64> {
        let mut triplet = TriMat::new((self.num_rows, self.num_cols));
        for (&r, &lv) in self.row_indices.iter().zip(self.log_values.iter()) {
            let v = if lv.is_infinite() && lv.is_sign_negative() { 0.0 } else { lv.exp() };
            triplet.add_triplet(r, 0, v);
        }
        triplet.to_csr()
    }

    /// Approximate memory used by this sparse CPT (indices + values + scope overhead).
    pub fn memory_bytes(&self) -> usize {
        self.row_indices.len() * std::mem::size_of::<usize>()
            + self.col_indices.len() * std::mem::size_of::<usize>()
            + self.log_values.len() * std::mem::size_of::<f64>()
            + std::mem::size_of::<Scope>()
    }

    /// Expand the sparse storage back to a dense vector of probabilities.
    pub fn to_dense(&self) -> Vec<f64> {
        let mut result = vec![0.0; self.num_rows];
        for (&r, &lv) in self.row_indices.iter().zip(self.log_values.iter()) {
            if r < self.num_rows {
                result[r] = lv.exp();
            }
        }
        result
    }
}

/// Choose between sparse and dense storage for a CPT based on its density.
/// Returns `TabularFactor::Sparse` when density is below `SPARSE_DENSITY_THRESHOLD`
/// or the number of entries exceeds 1,000,000; otherwise returns `TabularFactor::Dense`.
pub fn choose_storage_format(scope: &Scope, values: &[f64]) -> TabularFactor {
    let density = estimate_density(values);
    if density < SPARSE_DENSITY_THRESHOLD || values.len() > 1_000_000 {
        let mut log_entries = std::collections::HashMap::new();
        for (i, &v) in values.iter().enumerate() {
            if v > UNDERFLOW_THRESHOLD {
                log_entries.insert(i, v.ln());
            }
        }
        TabularFactor::Sparse { scope: scope.clone(), log_entries }
    } else {
        let log_table = values.iter()
            .map(|&v| if v < UNDERFLOW_THRESHOLD { f64::NEG_INFINITY } else { v.ln() })
            .collect();
        TabularFactor::Dense { scope: scope.clone(), log_table }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::variable::Variable;
    use crate::core::domain::Domain;
    use crate::core::factor::Scope;

    #[test]
    fn test_sparse_cpt_dense_roundtrip() {
        let v = Variable::new("X", Domain::binary());
        let scope = Scope::new(vec![&v]);
        let values = vec![0.3, 0.7];
        let sparse = SparseCptStorage::new(scope.clone(), values.clone()).unwrap();
        let roundtrip = sparse.to_dense();
        assert_eq!(roundtrip.len(), 2);
        assert!((roundtrip[0] - 0.3).abs() < 1e-10);
        assert!((roundtrip[1] - 0.7).abs() < 1e-10);
    }

    #[test]
    fn test_sparse_storage_detection() {
        let v = Variable::new("X", Domain::discrete(vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]).unwrap());
        let scope = Scope::new(vec![&v]);
        // 20% density (2 non-zero out of 10)
        let mut sparse_vals = vec![0.0; 10];
        sparse_vals[0] = 1.0;
        sparse_vals[5] = 1.0;
        let storage = SparseCptStorage::new(scope.clone(), sparse_vals).unwrap();
        assert!(storage.density() < 0.3);
        assert!(storage.num_nonzeros() < storage.num_entries());
    }

    #[test]
    fn test_sparse_cpt_from_tabular_dense() {
        let v = Variable::new("Y", Domain::binary());
        let scope = Scope::new(vec![&v]);
        let values = vec![0.5, 0.5];
        let tf = TabularFactor::from_values(scope, values).unwrap();
        let sparse = SparseCptStorage::from_tabular_factor(&tf);
        assert!((sparse.log_value_at(0).exp() - 0.5).abs() < 1e-10);
        assert!((sparse.log_value_at(1).exp() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_choose_storage_format_sparse() {
        let v = Variable::new("Z", Domain::discrete(vec!["a", "b", "c", "d"]).unwrap());
        let scope = Scope::new(vec![&v]);
        let values = vec![1.0, 0.0, 0.0, 0.0];
        let factor = choose_storage_format(&scope, &values);
        match factor {
            TabularFactor::Sparse { .. } => {}
            _ => panic!("Expected Sparse variant for low density"),
        }
    }

    #[test]
    fn test_choose_storage_format_dense() {
        let v = Variable::new("W", Domain::binary());
        let scope = Scope::new(vec![&v]);
        let values = vec![0.5, 0.5];
        let factor = choose_storage_format(&scope, &values);
        match factor {
            TabularFactor::Dense { .. } => {}
            _ => panic!("Expected Dense variant for high density"),
        }
    }

    #[test]
    fn test_wrong_shape_errors() {
        let v = Variable::new("E", Domain::binary());
        let scope = Scope::new(vec![&v]);
        let result = SparseCptStorage::new(scope, vec![0.5, 0.3, 0.2]);
        assert!(result.is_err());
    }
}
