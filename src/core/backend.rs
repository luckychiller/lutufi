use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::TabularFactor,
    variable::VariableId,
};
use std::sync::OnceLock;

/// Arithmetic precision a backend computes factor operations in.
///
/// Lutufi's headline numerical claim is log-space stability, so a backend that
/// silently reduces precision would undermine the guarantee users rely on. This
/// makes the actual precision inspectable rather than implicit — see ADR-011.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precision {
    /// IEEE 754 double precision (~15-16 significant decimal digits).
    F64,
    /// IEEE 754 single precision (~7 significant decimal digits).
    ///
    /// Results are accurate to roughly `1e-6` relative rather than `1e-15`.
    /// Acceptable for exploratory work and large approximate inference; not for
    /// results where the last digits are load-bearing.
    F32,
}

impl std::fmt::Display for Precision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Precision::F64 => write!(f, "f64"),
            Precision::F32 => write!(f, "f32"),
        }
    }
}

/// Abstraction for numerical computations on factors.
///
/// This allows switching between CPU-based (standard or parallel)
/// and GPU-based (via wgpu or CUDA) implementations.
pub trait ComputeBackend: Send + Sync {
    /// Name of the backend.
    fn name(&self) -> &'static str;

    /// Precision this backend computes in.
    ///
    /// Defaults to [`Precision::F64`]; a backend that computes in anything less
    /// must say so, so that callers can record it alongside their results.
    fn precision(&self) -> Precision {
        Precision::F64
    }

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
///
/// # Precision
///
/// **The GPU backend computes in `f32`, not `f64`** — the shaders upload
/// log-values as single precision and read results back the same way. Factor
/// values are accurate to roughly `1e-6` relative instead of `1e-15`.
///
/// This is why GPU acceleration is opt-in and never automatic: activating it is
/// a decision to trade precision for throughput, and only the researcher can
/// judge whether their results tolerate that. Nothing in Lutufi calls this
/// function on your behalf. See ADR-011 and
/// [`ComputeBackend::precision`].
///
/// # Adapter selection
///
/// This accepts whatever adapter the driver returns, which on some systems is a
/// software rasterizer — shaders then execute on the CPU, slower than the CPU
/// kernels. Prefer [`initialize_gpu_requiring_hardware`] unless you specifically
/// want to allow that fallback.
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

/// Like [`initialize_gpu`], but refuses a software adapter.
///
/// Carries the same `f32` precision consequence described on [`initialize_gpu`].
pub fn initialize_gpu_requiring_hardware() -> LutufiResult<()> {
    #[cfg(feature = "gpu")]
    {
        use crate::core::scalability::wgpu_backend::WgpuBackend;
        let backend = WgpuBackend::new_requiring_hardware()?;
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

/// Precision the currently active backend computes in.
///
/// Record this alongside published results: it is the difference between numbers
/// good to ~15 digits and numbers good to ~7.
pub fn active_precision() -> Precision {
    get_backend().precision()
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
