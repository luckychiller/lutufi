use std::collections::HashMap;
use rand::prelude::*;
use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::{Scope, TabularFactor},
    models::factor_graph::FactorGraph,
    variable::VariableId,
};

/// MCMC algorithm variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCMCMethod {
    /// Gibbs Sampling (standard for discrete BNs).
    Gibbs,
    /// Metropolis-Hastings (more general, supports continuous).
    MetropolisHastings,
}

impl Default for MCMCMethod {
    fn default() -> Self {
        MCMCMethod::Gibbs
    }
}

/// Options for MCMC inference.
#[derive(Debug, Clone)]
pub struct MCMCOptions {
    /// Number of samples to collect.
    pub n_samples: usize,
    /// Number of initial samples to discard.
    pub burn_in: usize,
    /// Keep only every K-th sample.
    pub thin: usize,
    /// Number of parallel chains to run.
    pub chains: usize,
    /// Optional random seed.
    pub seed: Option<u64>,
    /// Which MCMC algorithm to use.
    pub method: MCMCMethod,
}

impl Default for MCMCOptions {
    fn default() -> Self {
        MCMCOptions {
            n_samples: 1000,
            burn_in: 100,
            thin: 1,
            chains: 3,
            seed: None,
            method: MCMCMethod::Gibbs,
        }
    }
}

/// Result of MCMC inference.
pub struct MCMCResult {
    /// Marginal probabilities for each variable.
    pub marginals: HashMap<VariableId, TabularFactor>,
    /// Estimated variance for each state's probability.
    pub variances: HashMap<VariableId, Vec<f64>>,
    /// Effective Sample Size (ESS) for each variable.
    pub ess: HashMap<VariableId, f64>,
    /// Gelman-Rubin convergence statistic (R-hat).
    pub r_hat: HashMap<VariableId, f64>,
    /// Sample autocorrelation at different lags.
    pub autocorrelations: HashMap<VariableId, Vec<f64>>,
    /// 95% credible intervals for each state's probability.
    pub credible_intervals: HashMap<VariableId, Vec<(f64, f64)>>,
    /// Number of samples collected per chain.
    pub n_samples: usize,
    /// Number of initial samples discarded.
    pub burn_in: usize,
    /// Number of parallel chains run.
    pub chains: usize,
    /// Which MCMC method was used.
    pub method: MCMCMethod,
}

/// Markov Chain Monte Carlo (MCMC) inference engine.
pub struct MCMCEngine {
    graph: FactorGraph,
    options: MCMCOptions,
}

impl MCMCEngine {
    /// Create a new MCMCEngine.
    pub fn new(graph: FactorGraph, options: MCMCOptions) -> Self {
        MCMCEngine { graph, options }
    }

    /// Run MCMC inference.
    pub fn run(&self, evidence: &Assignment) -> LutufiResult<MCMCResult> {
        match self.options.method {
            MCMCMethod::Gibbs => self.gibbs_sample(evidence),
            MCMCMethod::MetropolisHastings => self.metropolis_hastings(evidence),
        }
    }

    /// Run Gibbs sampling.
    pub fn gibbs_sample(&self, evidence: &Assignment) -> LutufiResult<MCMCResult> {
        let mut all_chain_samples = Vec::new();
        let mut rng = match self.options.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        for _ in 0..self.options.chains {
            let chain_samples = self.run_gibbs_chain(evidence, &mut rng)?;
            all_chain_samples.push(chain_samples);
        }

        self.process_samples(all_chain_samples)
    }

    /// Run Metropolis-Hastings sampling.
    pub fn metropolis_hastings(&self, evidence: &Assignment) -> LutufiResult<MCMCResult> {
        let mut all_chain_samples = Vec::new();
        let mut rng = match self.options.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        for _ in 0..self.options.chains {
            let chain_samples = self.run_mh_chain(evidence, &mut rng)?;
            all_chain_samples.push(chain_samples);
        }

        self.process_samples(all_chain_samples)
    }

    fn run_gibbs_chain(&self, evidence: &Assignment, rng: &mut StdRng) -> LutufiResult<Vec<Assignment>> {
        let mut current_state = evidence.clone();
        for (&id, var) in &self.graph.variables {
            if !evidence.contains(&id) {
                let size = var.domain().size().ok_or_else(|| LutufiError::InternalError {
                    message: "Variable domain size missing".to_string()
                })?;
                let val = rng.gen_range(0..size);
                current_state.set_discrete(id, val)?;
            }
        }

        let mut samples = Vec::with_capacity(self.options.n_samples);
        let total_iterations = self.options.burn_in + self.options.n_samples * self.options.thin;

        for i in 0..total_iterations {
            for &var_id in self.graph.variables.keys() {
                if evidence.contains(&var_id) { continue; }
                self.sample_variable_gibbs(var_id, &mut current_state, rng)?;
            }

            if i >= self.options.burn_in && (i - self.options.burn_in) % self.options.thin == 0 {
                samples.push(current_state.clone());
            }
        }

        Ok(samples)
    }

    fn run_mh_chain(&self, evidence: &Assignment, rng: &mut StdRng) -> LutufiResult<Vec<Assignment>> {
        let mut current_state = evidence.clone();
        for (&id, var) in &self.graph.variables {
            if !evidence.contains(&id) {
                let size = var.domain().size().ok_or_else(|| LutufiError::InternalError {
                    message: "Variable domain size missing".to_string()
                })?;
                let val = rng.gen_range(0..size);
                current_state.set_discrete(id, val)?;
            }
        }

        let mut samples = Vec::with_capacity(self.options.n_samples);
        let total_iterations = self.options.burn_in + self.options.n_samples * self.options.thin;

        for i in 0..total_iterations {
            for &var_id in self.graph.variables.keys() {
                if evidence.contains(&var_id) { continue; }
                self.sample_variable_mh(var_id, &mut current_state, rng)?;
            }

            if i >= self.options.burn_in && (i - self.options.burn_in) % self.options.thin == 0 {
                samples.push(current_state.clone());
            }
        }

        Ok(samples)
    }

    fn sample_variable_gibbs(&self, var_id: VariableId, state: &mut Assignment, rng: &mut StdRng) -> LutufiResult<()> {
        let var = self.graph.variables.get(&var_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: var_id.to_string(),
            available: "".to_string()
        })?;
        let size = var.domain().size().ok_or_else(|| LutufiError::InternalError {
            message: "Variable domain size missing".to_string()
        })?;
        let mut log_probs = vec![0.0; size];

        for val in 0..size {
            state.set_discrete(var_id, val)?;
            let mut log_prob = 0.0;
            if let Some(factor_indices) = self.graph.var_to_factors.get(&var_id) {
                for &f_idx in factor_indices {
                    log_prob += self.graph.factors[f_idx].log_value_at_assignment(state)?;
                }
            }
            log_probs[val] = log_prob;
        }

        let max_log = log_probs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mut probs: Vec<f64> = log_probs.iter().map(|&l| (l - max_log).exp()).collect();
        let sum: f64 = probs.iter().sum();
        for p in &mut probs { *p /= sum; }

        let r: f64 = rng.gen();
        let mut cumulative = 0.0;
        for (val, &p) in probs.iter().enumerate() {
            cumulative += p;
            if r <= cumulative {
                state.set_discrete(var_id, val)?;
                return Ok(());
            }
        }

        state.set_discrete(var_id, size - 1)?;
        Ok(())
    }

    fn sample_variable_mh(&self, var_id: VariableId, state: &mut Assignment, rng: &mut StdRng) -> LutufiResult<()> {
        let var = self.graph.variables.get(&var_id).ok_or_else(|| LutufiError::VariableNotFound {
            name: var_id.to_string(),
            available: "".to_string()
        })?;
        let size = var.domain().size().ok_or_else(|| LutufiError::InternalError {
            message: "Variable domain size missing".to_string()
        })?;

        let current_val = state.get_discrete(&var_id)?;
        let proposed_val = self.propose_discrete_value(current_val, size, rng);
        if proposed_val == current_val {
            return Ok(());
        }

        let current_log = self.compute_local_log_potential(var_id, state)?;
        state.set_discrete(var_id, proposed_val)?;
        let proposed_log = self.compute_local_log_potential(var_id, state)?;

        let log_ratio = proposed_log - current_log;
        let accept_prob = log_ratio.min(0.0).exp();

        if rng.gen::<f64>() > accept_prob {
            state.set_discrete(var_id, current_val)?;
        }

        Ok(())
    }

    fn propose_discrete_value(&self, current_val: usize, size: usize, rng: &mut StdRng) -> usize {
        if size <= 1 {
            return current_val;
        }

        let mut candidate = rng.gen_range(0..size);
        while candidate == current_val {
            candidate = rng.gen_range(0..size);
        }
        candidate
    }

    fn compute_local_log_potential(&self, var_id: VariableId, state: &Assignment) -> LutufiResult<f64> {
        let mut log_prob = 0.0;
        if let Some(factor_indices) = self.graph.var_to_factors.get(&var_id) {
            for &f_idx in factor_indices {
                log_prob += self.graph.factors[f_idx].log_value_at_assignment(state)?;
            }
        }
        Ok(log_prob)
    }

    fn process_samples(&self, all_chain_samples: Vec<Vec<Assignment>>) -> LutufiResult<MCMCResult> {
        if all_chain_samples.is_empty() || all_chain_samples[0].is_empty() {
            return Err(LutufiError::InternalError { message: "No samples produced by MCMC".to_string() });
        }

        let mut marginals = HashMap::new();
        let mut variances = HashMap::new();
        let mut ess = HashMap::new();
        let mut r_hat = HashMap::new();
        let mut autocorrelations = HashMap::new();
        let mut credible_intervals = HashMap::new();

        let n_chains = all_chain_samples.len();
        let n_per_chain = all_chain_samples[0].len();
        let total_n = (n_chains * n_per_chain) as f64;

        for &var_id in self.graph.variables.keys() {
            let var = self.graph.variables.get(&var_id).ok_or_else(|| LutufiError::VariableNotFound {
                name: var_id.to_string(),
                available: "".to_string()
            })?;
            let size = var.domain().size().ok_or_else(|| LutufiError::InternalError {
                message: "Variable domain size missing".to_string()
            })?;

            let mut counts = vec![0.0; size];
            for chain in &all_chain_samples {
                for sample in chain {
                    let val = sample.get_discrete(&var_id)?;
                    counts[val] += 1.0;
                }
            }

            let marginal_probs: Vec<f64> = counts.iter().map(|&c| c / total_n).collect();
            marginals.insert(var_id, TabularFactor::from_values(Scope::new(vec![var]), marginal_probs.clone())?);
            variances.insert(var_id, marginal_probs.iter().map(|&p| p * (1.0 - p)).collect());

            let mut best_r_hat: f64 = 1.0;
            let mut min_ess = f64::INFINITY;
            let mut variable_state_acfs: Vec<Vec<f64>> = Vec::with_capacity(size);
            let mut state_intervals = Vec::with_capacity(size);

            for state_val in 0..size {
                let mut chain_means = Vec::new();
                let mut chain_vars = Vec::new();
                let mut chain_acfs = Vec::new();

                for chain in &all_chain_samples {
                    let series: Vec<f64> = chain.iter()
                        .map(|sample| if sample.get_discrete(&var_id).unwrap_or(0) == state_val { 1.0 } else { 0.0 })
                        .collect();

                    let n = series.len() as f64;
                    let mean = series.iter().sum::<f64>() / n;
                    let variance = if series.len() > 1 {
                        series.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
                    } else {
                        0.0
                    };
                    chain_means.push(mean);
                    chain_vars.push(variance);
                    chain_acfs.push(self.autocorrelation(&series)?);
                }

                let r_hat_state = self.compute_r_hat(&chain_means, &chain_vars, n_per_chain);
                best_r_hat = best_r_hat.max(r_hat_state);

                let avg_acf = self.average_autocorrelation(&chain_acfs);
                let ess_state = self.compute_ess(&avg_acf, total_n);
                min_ess = min_ess.min(ess_state);
                variable_state_acfs.push(avg_acf);

                let p = marginal_probs[state_val];
                let se = if ess_state > 0.0 { (p * (1.0 - p) / ess_state).sqrt() } else { 0.0 };
                state_intervals.push(((p - 1.96 * se).max(0.0), (p + 1.96 * se).min(1.0)));
            }

            let averaged_acf = self.average_state_acfs(&variable_state_acfs);
            autocorrelations.insert(var_id, averaged_acf);
            credible_intervals.insert(var_id, state_intervals);
            r_hat.insert(var_id, best_r_hat);
            ess.insert(var_id, min_ess.max(1.0));
        }

        Ok(MCMCResult {
            marginals,
            variances,
            ess,
            r_hat,
            autocorrelations,
            credible_intervals,
            n_samples: self.options.n_samples,
            burn_in: self.options.burn_in,
            chains: self.options.chains,
            method: self.options.method,
        })
    }

    fn compute_r_hat(&self, chain_means: &[f64], chain_vars: &[f64], chain_length: usize) -> f64 {
        if chain_means.len() < 2 || chain_length < 2 {
            return 1.0;
        }

        let m = chain_means.len() as f64;
        let n = chain_length as f64;
        let grand_mean = chain_means.iter().sum::<f64>() / m;

        let mut b = 0.0;
        for &mean in chain_means {
            b += (mean - grand_mean).powi(2);
        }
        b *= n / (m - 1.0);

        let w = chain_vars.iter().sum::<f64>() / m;
        if w <= 0.0 {
            return 1.0;
        }

        let var_plus = (n - 1.0) / n * w + b / n;
        (var_plus / w).sqrt()
    }

    fn autocorrelation(&self, series: &[f64]) -> LutufiResult<Vec<f64>> {
        let n = series.len();
        if n < 2 {
            return Ok(Vec::new());
        }

        let mean = series.iter().sum::<f64>() / n as f64;
        let variance = series.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n as f64;
        if variance <= 0.0 {
            return Ok(vec![0.0; std::cmp::min(n / 2, 20)]);
        }

        let max_lag = std::cmp::min(n / 2, 20);
        let mut acf = Vec::with_capacity(max_lag);
        for lag in 1..=max_lag {
            let cov = (0..n - lag)
                .map(|t| (series[t] - mean) * (series[t + lag] - mean))
                .sum::<f64>() / n as f64;
            acf.push(cov / variance);
        }
        Ok(acf)
    }

    fn average_autocorrelation(&self, acfs: &[Vec<f64>]) -> Vec<f64> {
        if acfs.is_empty() {
            return Vec::new();
        }

        let max_len = acfs.iter().map(|acf| acf.len()).max().unwrap_or(0);
        let mut average = vec![0.0; max_len];
        let mut counts = vec![0; max_len];

        for acf in acfs {
            for (i, &value) in acf.iter().enumerate() {
                average[i] += value;
                counts[i] += 1;
            }
        }

        for i in 0..max_len {
            if counts[i] > 0 {
                average[i] /= counts[i] as f64;
            }
        }

        average
    }

    fn average_state_acfs(&self, state_acfs: &[Vec<f64>]) -> Vec<f64> {
        if state_acfs.is_empty() {
            return Vec::new();
        }
        let max_len = state_acfs.iter().map(|acf| acf.len()).max().unwrap_or(0);
        let mut summed = vec![0.0; max_len];
        let mut count = vec![0; max_len];

        for acf in state_acfs {
            for (i, &value) in acf.iter().enumerate() {
                summed[i] += value;
                count[i] += 1;
            }
        }

        (0..max_len)
            .map(|i| if count[i] > 0 { summed[i] / count[i] as f64 } else { 0.0 })
            .collect()
    }

    fn compute_ess(&self, avg_acf: &[f64], total_samples: f64) -> f64 {
        let positive_rhos: f64 = avg_acf.iter().copied().filter(|rho| *rho > 0.0).sum();
        let denom = 1.0 + 2.0 * positive_rhos;
        if denom <= 0.0 {
            total_samples
        } else {
            (total_samples / denom).max(1.0)
        }
    }
}
