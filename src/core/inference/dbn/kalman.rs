use std::time::Instant;

use ndarray::{Array1, Array2};
use crate::core::error::{LutufiError, LutufiResult};

/// Kalman Filter result.
#[derive(Debug, Clone)]
pub struct KalmanFilterResult {
    pub means: Vec<Array1<f64>>,
    pub covariances: Vec<Array2<f64>>,
    pub log_likelihoods: Vec<f64>,
    pub computation_time: std::time::Duration,
}

/// Kalman Filter for linear Gaussian DBNs.
pub struct KalmanFilter {
    dim: usize,
    f: Array2<f64>,
    h: Array2<f64>,
    q: Array2<f64>,
    r: Array2<f64>,
    mu: Array1<f64>,
    sigma: Array2<f64>,
}

impl KalmanFilter {
    /// Create a new Kalman filter with system matrices.
    ///
    /// # Arguments
    /// * `f` - State transition matrix (dim × dim).
    /// * `h` - Observation matrix (obs_dim × dim).
    /// * `q` - Process noise covariance (dim × dim).
    /// * `r` - Observation noise covariance (obs_dim × obs_dim).
    /// * `initial_mu` - Initial state mean.
    /// * `initial_sigma` - Initial state covariance.
    pub fn new(
        f: Array2<f64>,
        h: Array2<f64>,
        q: Array2<f64>,
        r: Array2<f64>,
        initial_mu: Array1<f64>,
        initial_sigma: Array2<f64>,
    ) -> LutufiResult<Self> {
        let dim = f.nrows();
        if f.ncols() != dim { return Err(LutufiError::InternalError { message: format!("F must be {}x{}", dim, dim) }); }
        if h.ncols() != dim { return Err(LutufiError::InternalError { message: "H must match state dimension".to_string() }); }
        if q.nrows() != dim || q.ncols() != dim { return Err(LutufiError::InternalError { message: "Q must match state dimension".to_string() }); }
        if r.nrows() != h.nrows() || r.ncols() != h.nrows() { return Err(LutufiError::InternalError { message: "R must match observation dimension".to_string() }); }
        if initial_mu.len() != dim { return Err(LutufiError::InternalError { message: format!("Initial mean must be dimension {}", dim) }); }

        Ok(KalmanFilter { dim, f, h, q, r, mu: initial_mu, sigma: initial_sigma })
    }

    /// Get the state dimension.
    pub fn dim(&self) -> usize { self.dim }
    /// Get the current state mean estimate.
    pub fn state_mean(&self) -> &Array1<f64> { &self.mu }
    /// Get the current state covariance estimate.
    pub fn state_covariance(&self) -> &Array2<f64> { &self.sigma }

    /// Predict the next state using the transition model.
    pub fn predict(&mut self) {
        self.mu = self.f.dot(&self.mu);
        let ft = self.f.t();
        self.sigma = self.f.dot(&self.sigma).dot(&ft) + &self.q;
    }

    /// Update the state estimate given an observation.
    pub fn update(&mut self, observation: &Array1<f64>) {
        let ht = self.h.t();
        let innovation = observation - &self.h.dot(&self.mu);
        let s = self.h.dot(&self.sigma).dot(&ht) + &self.r;

        let s_inv = matrix_inverse(s.view());
        let kalman_gain = self.sigma.dot(&ht).dot(&s_inv);
        self.mu = &self.mu + kalman_gain.dot(&innovation);
        self.sigma = &self.sigma - kalman_gain.dot(&self.h).dot(&self.sigma);
    }

    /// Perform one predict-update step with the given observation.
    pub fn step(&mut self, observation: &Array1<f64>) {
        self.predict();
        self.update(observation);
    }

    /// Run the Kalman filter over a sequence of observations.
    pub fn filter_sequence(&mut self, observations: &[Array1<f64>]) -> LutufiResult<KalmanFilterResult> {
        let start_time = Instant::now();
        let t = observations.len();
        if t == 0 {
            return Err(LutufiError::InternalError { message: "Observations must not be empty".to_string() });
        }

        let mut means = Vec::with_capacity(t);
        let mut covariances = Vec::with_capacity(t);
        let mut log_likelihoods = Vec::with_capacity(t);

        for obs in observations {
            if obs.len() != self.h.nrows() {
                return Err(LutufiError::InternalError { message: format!("Observation dimension {} != expected {}", obs.len(), self.h.nrows()) });
            }
            self.predict();
            self.update(obs);
            means.push(self.mu.clone());
            covariances.push(self.sigma.clone());

            let innovation = obs - &self.h.dot(&self.mu);
            let s = self.h.dot(&self.sigma).dot(&self.h.t()) + &self.r;
            let det_s = matrix_determinant(s.view()).abs();
            if det_s > 0.0 {
                let mahalanobis = innovation.dot(&s).dot(&innovation);
                let ll = -0.5 * (s.nrows() as f64 * (2.0 * std::f64::consts::PI).ln() + det_s.ln() + mahalanobis);
                log_likelihoods.push(ll);
            } else {
                log_likelihoods.push(f64::NEG_INFINITY);
            }
        }

        Ok(KalmanFilterResult { means, covariances, log_likelihoods, computation_time: start_time.elapsed() })
    }
}

/// Compute matrix determinant using Gaussian elimination.
fn matrix_determinant(a: ndarray::ArrayView2<f64>) -> f64 {
    let n = a.nrows();
    if n == 0 { return 0.0; }
    if n == 1 { return a[[0, 0]]; }
    if n == 2 { return a[[0, 0]] * a[[1, 1]] - a[[0, 1]] * a[[1, 0]]; }

    let mut mat = ndarray::Array2::<f64>::zeros((n, n));
    for i in 0..n { for j in 0..n { mat[[i, j]] = a[[i, j]]; } }
    let mut det = 1.0;

    for col in 0..n {
        let mut max_row = col;
        let mut max_val = mat[[col, col]].abs();
        for row in col + 1..n {
            let val = mat[[row, col]].abs();
            if val > max_val { max_val = val; max_row = row; }
        }
        if max_val < 1e-15 { return 0.0; }
        if max_row != col {
            for j in 0..n {
                let tmp = mat[[col, j]];
                mat[[col, j]] = mat[[max_row, j]];
                mat[[max_row, j]] = tmp;
            }
            det = -det;
        }
        det *= mat[[col, col]];
        for row in col + 1..n {
            let factor = mat[[row, col]] / mat[[col, col]];
            for j in col..n { mat[[row, j]] -= factor * mat[[col, j]]; }
        }
    }
    det
}

/// Compute matrix inverse using Gaussian elimination with partial pivoting.
fn matrix_inverse(a: ndarray::ArrayView2<f64>) -> ndarray::Array2<f64> {
    let n = a.nrows();
    let mut aug = ndarray::Array2::<f64>::zeros((n, 2 * n));
    for i in 0..n {
        for j in 0..n { aug[[i, j]] = a[[i, j]]; }
        aug[[i, n + i]] = 1.0;
    }

    for col in 0..n {
        let mut max_row = col;
        let mut max_val = aug[[col, col]].abs();
        for row in col + 1..n {
            let val = aug[[row, col]].abs();
            if val > max_val { max_val = val; max_row = row; }
        }
        if max_val < 1e-15 { continue; }
        if max_row != col {
            for j in 0..2 * n {
                let tmp = aug[[col, j]];
                aug[[col, j]] = aug[[max_row, j]];
                aug[[max_row, j]] = tmp;
            }
        }

        let pivot = aug[[col, col]];
        for j in 0..2 * n { aug[[col, j]] /= pivot; }

        for row in 0..n {
            if row != col {
                let factor = aug[[row, col]];
                if factor.abs() > 1e-15 {
                    for j in 0..2 * n { aug[[row, j]] -= factor * aug[[col, j]]; }
                }
            }
        }
    }

    let mut inv = ndarray::Array2::<f64>::zeros((n, n));
    for i in 0..n {
        for j in 0..n { inv[[i, j]] = aug[[i, n + j]]; }
    }
    inv
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_kalman_filter() {
        let f = array![[1.0, 1.0], [0.0, 1.0]];
        let h = array![[1.0, 0.0]];
        let q = array![[0.1, 0.0], [0.0, 0.1]];
        let r = array![[0.5]];
        let mu0 = array![0.0, 0.0];
        let sigma0 = array![[1.0, 0.0], [0.0, 1.0]];

        let mut kf = KalmanFilter::new(f, h, q, r, mu0, sigma0).unwrap();
        let result = kf.filter_sequence(&[array![0.5], array![1.2], array![1.8]]).unwrap();
        assert_eq!(result.means.len(), 3);
    }
}
