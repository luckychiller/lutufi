use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::log_sum_exp,
};

/// Hidden Markov Model engine with exact inference algorithms.
pub struct HMMEngine {
    /// Number of hidden states in the model.
    pub num_states: usize,
    /// Number of distinct observation symbols.
    pub num_observations: usize,
    initial: Vec<f64>,
    transition: Vec<Vec<f64>>,
    emission: Vec<Vec<f64>>,
}

impl HMMEngine {
    /// Create a new HMM from probability-space parameters.
    pub fn new(initial: Vec<f64>, transition: Vec<Vec<f64>>, emission: Vec<Vec<f64>>) -> LutufiResult<Self> {
        let num_states = initial.len();
        if num_states == 0 {
            return Err(LutufiError::InternalError { message: "HMM must have at least one state".to_string() });
        }
        if transition.len() != num_states || transition.iter().any(|r| r.len() != num_states) {
            return Err(LutufiError::InternalError { message: format!("Transition matrix must be {}x{}", num_states, num_states) });
        }
        let num_observations = emission.first().map(|r| r.len()).unwrap_or(0);
        if num_observations == 0 {
            return Err(LutufiError::InternalError { message: "HMM must have at least one observation symbol".to_string() });
        }
        if emission.len() != num_states {
            return Err(LutufiError::InternalError { message: format!("Emission matrix must have {} rows", num_states) });
        }

        let to_log = |vals: &[f64]| -> Vec<f64> {
            vals.iter().map(|&v| if v <= 0.0 { f64::NEG_INFINITY } else { v.ln() }).collect()
        };

        Ok(HMMEngine {
            num_states,
            num_observations,
            initial: to_log(&initial),
            transition: transition.iter().map(|r| to_log(r)).collect(),
            emission: emission.iter().map(|r| to_log(r)).collect(),
        })
    }

    /// Forward algorithm: compute P(Z_t | O_{1:t}) and log-probability.
    pub fn forward(&self, observations: &[usize]) -> LutufiResult<(Vec<Vec<f64>>, f64)> {
        let t = observations.len();
        if t == 0 {
            return Err(LutufiError::InternalError { message: "Observations sequence must not be empty".to_string() });
        }

        let mut alpha = vec![vec![f64::NEG_INFINITY; self.num_states]; t];
        let o0 = observations[0];
        if o0 >= self.num_observations {
            return Err(LutufiError::InternalError { message: format!("Observation {} out of range (0..{})", o0, self.num_observations) });
        }

        for i in 0..self.num_states {
            alpha[0][i] = self.initial[i] + self.emission[i][o0];
        }

        for step in 1..t {
            let o = observations[step];
            if o >= self.num_observations {
                return Err(LutufiError::InternalError { message: format!("Observation {} out of range (0..{})", o, self.num_observations) });
            }
            for j in 0..self.num_states {
                let mut sum = f64::NEG_INFINITY;
                for i in 0..self.num_states {
                    sum = log_sum_exp(sum, alpha[step - 1][i] + self.transition[i][j]);
                }
                alpha[step][j] = sum + self.emission[j][o];
            }
        }

        let mut log_prob = f64::NEG_INFINITY;
        for i in 0..self.num_states { log_prob = log_sum_exp(log_prob, alpha[t - 1][i]); }

        let marginals = self.normalize_alpha(&alpha);
        Ok((marginals, log_prob))
    }

    /// Forward-backward algorithm: compute P(Z_t | O_{1:T}).
    pub fn forward_backward(&self, observations: &[usize]) -> LutufiResult<(Vec<Vec<f64>>, f64)> {
        let t = observations.len();
        if t == 0 {
            return Err(LutufiError::InternalError { message: "Observations sequence must not be empty".to_string() });
        }

        let (alpha, log_prob) = self.forward(observations)?;
        let mut beta = vec![vec![f64::NEG_INFINITY; self.num_states]; t];
        for i in 0..self.num_states { beta[t - 1][i] = 0.0; }

        for step in (0..t - 1).rev() {
            let o_next = observations[step + 1];
            for i in 0..self.num_states {
                let mut sum = f64::NEG_INFINITY;
                for j in 0..self.num_states {
                    sum = log_sum_exp(sum, self.transition[i][j] + self.emission[j][o_next] + beta[step + 1][j]);
                }
                beta[step][i] = sum;
            }
        }

        let mut gamma = vec![vec![0.0; self.num_states]; t];
        for step in 0..t {
            let mut norm = f64::NEG_INFINITY;
            for i in 0..self.num_states {
                let alpha_log = if step == 0 { self.initial[i] + self.emission[i][observations[0]] } else {
                    let mut s = f64::NEG_INFINITY;
                    for j in 0..self.num_states {
                        let prev_log = if step == 1 { alpha[0][j] } else { alpha[step - 1][j] };
                        s = log_sum_exp(s, prev_log + self.transition[j][i]);
                    }
                    s + self.emission[i][observations[step]]
                };
                norm = log_sum_exp(norm, alpha_log + beta[step][i]);
            }
            if norm > f64::NEG_INFINITY {
                for i in 0..self.num_states {
                    let a_val = if step == 0 { alpha[0][i] } else { alpha[step][i] };
                    gamma[step][i] = (a_val + beta[step][i] - norm).exp();
                }
            }
        }

        Ok((gamma, log_prob))
    }

    /// Viterbi algorithm: most likely hidden state sequence.
    pub fn viterbi(&self, observations: &[usize]) -> LutufiResult<(Vec<usize>, f64)> {
        let t = observations.len();
        if t == 0 {
            return Err(LutufiError::InternalError { message: "Observations sequence must not be empty".to_string() });
        }

        let o0 = observations[0];
        if o0 >= self.num_observations {
            return Err(LutufiError::InternalError { message: format!("Observation {} out of range", o0) });
        }

        let mut delta = vec![vec![f64::NEG_INFINITY; self.num_states]; t];
        let mut psi = vec![vec![0usize; self.num_states]; t];

        for i in 0..self.num_states { delta[0][i] = self.initial[i] + self.emission[i][o0]; }

        for step in 1..t {
            let o = observations[step];
            if o >= self.num_observations {
                return Err(LutufiError::InternalError { message: format!("Observation {} out of range", o) });
            }
            for j in 0..self.num_states {
                let (best_val, best_idx) = (0..self.num_states)
                    .map(|i| (delta[step - 1][i] + self.transition[i][j], i))
                    .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or((f64::NEG_INFINITY, 0));
                delta[step][j] = best_val + self.emission[j][o];
                psi[step][j] = best_idx;
            }
        }

        let (best_log_prob, best_last) = (0..self.num_states)
            .map(|i| (delta[t - 1][i], i))
            .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((f64::NEG_INFINITY, 0));

        let mut path = vec![0usize; t];
        path[t - 1] = best_last;
        for step in (0..t - 1).rev() { path[step] = psi[step + 1][path[step + 1]]; }

        Ok((path, best_log_prob))
    }

    /// Baum-Welch EM: estimate parameters from observations.
    pub fn baum_welch(&mut self, observations: &[usize], max_iterations: usize, tolerance: f64) -> LutufiResult<f64> {
        let t = observations.len();
        if t == 0 {
            return Err(LutufiError::InternalError { message: "Observations must not be empty".to_string() });
        }

        let mut prev_ll = f64::NEG_INFINITY;
        for _ in 0..max_iterations {
            let (gamma, xi) = self.e_step(observations)?;
            let log_likelihood = self.m_step(observations, &gamma, &xi);

            if prev_ll > f64::NEG_INFINITY && (log_likelihood - prev_ll).abs() < tolerance {
                return Ok(log_likelihood);
            }
            prev_ll = log_likelihood;
        }
        Ok(prev_ll)
    }

    fn e_step(&self, observations: &[usize]) -> LutufiResult<(Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<Vec<f64>>>>)> {
        let t = observations.len();
        let (gamma_prob, _) = self.forward_backward(observations)?;

        let mut gamma = vec![vec![vec![0.0; 1]; self.num_states]; t];
        for step in 0..t {
            for i in 0..self.num_states { gamma[step][i][0] = gamma_prob[step][i]; }
        }

        let mut xi = vec![vec![vec![vec![0.0; self.num_states]; self.num_states]; t - 1]; 1];
        for step in 0..t - 1 {
            let o_next = observations[step + 1];
            let mut denom = 0.0;
            for i in 0..self.num_states {
                for j in 0..self.num_states {
                    xi[0][step][i][j] = gamma_prob[step][i] * (self.transition[i][j] + self.emission[j][o_next]).exp();
                    denom += xi[0][step][i][j];
                }
            }
            if denom > 0.0 {
                for i in 0..self.num_states {
                    for j in 0..self.num_states { xi[0][step][i][j] /= denom; }
                }
            }
        }

        Ok((gamma, xi))
    }

    fn m_step(&mut self, observations: &[usize], gamma: &[Vec<Vec<f64>>], xi: &[Vec<Vec<Vec<f64>>>]) -> f64 {
        let t = observations.len();
        let eps = 1e-12;

        for i in 0..self.num_states {
            let sum_gamma: f64 = gamma.iter().map(|step| step[i][0]).sum();
            self.initial[i] = if sum_gamma > 0.0 { (gamma[0][i][0] / sum_gamma).ln() } else { f64::NEG_INFINITY };
        }

        for i in 0..self.num_states {
            let denom: f64 = (0..t - 1).flat_map(|step| (0..self.num_states).map(move |k| xi[0][step][i][k])).sum();
            for j in 0..self.num_states {
                let numer: f64 = (0..t - 1).map(|step| xi[0][step][i][j]).sum();
                let val = (numer + eps) / (denom + eps * self.num_states as f64);
                self.transition[i][j] = val.max(1e-15).ln();
            }
        }

        for j in 0..self.num_states {
            let denom: f64 = (0..t).map(|step| gamma[step][j][0]).sum();
            for k in 0..self.num_observations {
                let numer: f64 = (0..t).filter(|&step| observations[step] == k).map(|step| gamma[step][j][0]).sum();
                let val = (numer + eps) / (denom + eps * self.num_observations as f64);
                self.emission[j][k] = val.max(1e-15).ln();
            }
        }

        let mut ll = 0.0;
        for step in 0..t {
            let mut step_ll = f64::NEG_INFINITY;
            for i in 0..self.num_states {
                let g = gamma[step][i][0];
                step_ll = log_sum_exp(step_ll, if g > 0.0 { g.ln() } else { f64::NEG_INFINITY });
            }
            if step_ll > f64::NEG_INFINITY { ll += step_ll; }
        }
        ll
    }

    fn normalize_alpha(&self, alpha: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let t = alpha.len();
        let mut marginals = Vec::with_capacity(t);
        for step in 0..t {
            let mut row = vec![0.0; self.num_states];
            let mut norm = f64::NEG_INFINITY;
            for i in 0..self.num_states { norm = log_sum_exp(norm, alpha[step][i]); }
            if norm > f64::NEG_INFINITY {
                for i in 0..self.num_states { row[i] = (alpha[step][i] - norm).exp(); }
            }
            marginals.push(row);
        }
        marginals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmm_forward() {
        let hmm = HMMEngine::new(
            vec![0.6, 0.4],
            vec![vec![0.7, 0.3], vec![0.4, 0.6]],
            vec![vec![0.5, 0.5], vec![0.3, 0.7]],
        ).unwrap();
        let (marginals, log_prob) = hmm.forward(&[0, 1, 0]).unwrap();
        assert_eq!(marginals.len(), 3);
        assert!(log_prob < 0.0);
    }

    #[test]
    fn test_hmm_viterbi() {
        let hmm = HMMEngine::new(
            vec![0.5, 0.5],
            vec![vec![0.8, 0.2], vec![0.3, 0.7]],
            vec![vec![0.9, 0.1], vec![0.2, 0.8]],
        ).unwrap();
        let (path, log_prob) = hmm.viterbi(&[0, 0, 1, 1]).unwrap();
        assert_eq!(path.len(), 4);
        assert!(log_prob < 0.0);
    }

    #[test]
    fn test_hmm_baum_welch() {
        let mut hmm = HMMEngine::new(
            vec![0.5, 0.5],
            vec![vec![0.6, 0.4], vec![0.3, 0.7]],
            vec![vec![0.7, 0.3], vec![0.4, 0.6]],
        ).unwrap();
        let ll = hmm.baum_welch(&[0, 0, 1, 1, 0, 1, 1, 1, 0, 0], 50, 1e-4).unwrap();
        assert!(ll.is_finite());
    }
}
