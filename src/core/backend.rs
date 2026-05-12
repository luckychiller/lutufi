use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::TabularFactor,
    variable::VariableId,
};
use std::sync::OnceLock;

/// Abstraction for numerical computations on factors.
/// 
/// This allows switching between CPU-based (standard or parallel) 
/// and GPU-based (via wgpu or CUDA) implementations.
pub trait ComputeBackend: Send + Sync {
    /// Name of the backend.
    fn name(&self) -> &'static str;

    /// Compute the product of two tabular factors.
    fn multiply(&self, a: &TabularFactor, b: &TabularFactor) -> LutufiResult<TabularFactor>;

    /// Marginalize out variables from a tabular factor.
    fn marginalize(&self, a: &TabularFactor, variables: &[VariableId]) -> LutufiResult<TabularFactor>;
    
    /// Normalize a factor in-place.
    fn normalize(&self, factor: &mut TabularFactor);

    // === Batched inference ===

    /// Multiply many pairs of factors in a single batched GPU dispatch.
    fn batch_multiply(&self, pairs: &[(TabularFactor, TabularFactor)]) -> LutufiResult<Vec<TabularFactor>> {
        pairs.iter().map(|(a, b)| self.multiply(a, b)).collect()
    }

    /// Marginalize many factors in a single batched GPU dispatch.
    fn batch_marginalize(&self, factors: &[(TabularFactor, Vec<VariableId>)]) -> LutufiResult<Vec<TabularFactor>> {
        factors.iter().map(|(f, vars)| self.marginalize(f, vars)).collect()
    }

    // === MCMC acceleration ===

    /// Compute log-probabilities for all states of a variable given current state.
    /// Returns log-probabilities of size `domain_size` for each variable to sample.
    fn mcmc_gibbs_probs(
        &self,
        _var_id: VariableId,
        _state: &Assignment,
        _factors: &[TabularFactor],
        _domain_size: usize,
    ) -> LutufiResult<Vec<f64>> {
        Err(LutufiError::InternalError { message: "GPU MCMC not available".to_string() })
    }

    /// Run multiple MCMC chain steps in parallel on GPU.
    /// Returns (accepted_states, log_probs) for each chain.
    fn mcmc_chain_step_parallel(
        &self,
        _chains: &[(Assignment, Vec<Vec<f64>>)],
        _factors: &[TabularFactor],
    ) -> LutufiResult<Vec<(Assignment, f64)>> {
        Err(LutufiError::InternalError { message: "GPU MCMC chains not available".to_string() })
    }

    // === Parameter learning acceleration ===

    /// Accumulate counts from data rows in parallel on GPU.
    /// Returns a dense count vector for the given scope size.
    fn accumulate_counts(
        &self,
        _data_rows: &[Vec<f64>],
        _scope_num_entries: usize,
    ) -> LutufiResult<Vec<f64>> {
        Err(LutufiError::InternalError { message: "GPU count accumulation not available".to_string() })
    }
}

static ACTIVE_BACKEND: OnceLock<Box<dyn ComputeBackend>> = OnceLock::new();

/// Get the currently active backend.
pub fn get_backend() -> &'static dyn ComputeBackend {
    ACTIVE_BACKEND.get_or_init(|| Box::new(CpuBackend)).as_ref()
}

/// Set the active backend. 
/// 
/// Returns an error if the backend was already initialized.
pub fn set_backend(backend: Box<dyn ComputeBackend>) -> Result<(), Box<dyn ComputeBackend>> {
    ACTIVE_BACKEND.set(backend)
}

/// Initialize the GPU backend if the feature is enabled.
pub fn initialize_gpu() -> LutufiResult<()> {
    #[cfg(feature = "gpu")]
    {
        use crate::core::scalability::wgpu_backend::WgpuBackend;
        let backend = WgpuBackend::new()?;
        set_backend(Box::new(backend)).map_err(|_| LutufiError::InternalError { 
            message: "Backend already initialized".to_string() 
        })?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        Err(LutufiError::InternalError { 
            message: "GPU feature not enabled".to_string() 
        })
    }
}

/// Default CPU implementation using standard loops and log-sum-exp.
pub struct CpuBackend;

impl ComputeBackend for CpuBackend {
    fn name(&self) -> &'static str { "CPU" }

    fn multiply(&self, a: &TabularFactor, b: &TabularFactor) -> LutufiResult<TabularFactor> {
        // This will contain the logic currently in TabularFactor::multiply
        a.multiply_internal(b)
    }

    fn marginalize(&self, a: &TabularFactor, variables: &[VariableId]) -> LutufiResult<TabularFactor> {
        // This will contain the logic currently in TabularFactor::marginalize
        a.marginalize_internal(variables)
    }

    fn normalize(&self, factor: &mut TabularFactor) {
        factor.normalize_internal();
    }
}
