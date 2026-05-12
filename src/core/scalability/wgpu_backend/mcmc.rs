use super::WgpuBackend;
#[cfg(feature = "gpu")]
use super::{shaders, factor_to_f32};

use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::TabularFactor,
    variable::VariableId,
};

#[cfg(feature = "gpu")]
use wgpu::util::DeviceExt;

#[cfg(feature = "gpu")]
pub(crate) fn gpu_mcmc_gibbs_probs(
    backend: &WgpuBackend,
    var_id: VariableId,
    state: &Assignment,
    factors: &[TabularFactor],
    domain_size: usize,
) -> LutufiResult<Vec<f64>> {
    let _ = (var_id, state);
    if factors.is_empty() || domain_size == 0 {
        return Err(LutufiError::InternalError { message: "Invalid MCMC Gibbs inputs".to_string() });
    }

    let total_entries: usize = factors.iter().map(|f| f.scope().num_entries()).sum();
    let mut all_data = Vec::with_capacity(total_entries);
    let mut meta = Vec::new();
    for f in factors {
        let f32v = factor_to_f32(f);
        meta.push(f32v.len() as u32);
        meta.push(f.scope().len() as u32);
        all_data.extend_from_slice(&f32v);
    }

    let data_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("MCMC Data"), contents: bytemuck::cast_slice(&all_data), usage: wgpu::BufferUsages::STORAGE,
    });
    let meta_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("MCMC Meta"), contents: bytemuck::cast_slice(&meta), usage: wgpu::BufferUsages::STORAGE,
    });
    let res_buf = backend.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("MCMC Res"), size: (domain_size * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false,
    });

    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    struct GibbsParamsRaw { domain_size: u32, num_factors: u32, _pad: u32 }
    let gparams = GibbsParamsRaw { domain_size: domain_size as u32, num_factors: factors.len() as u32, _pad: 0 };
    let gparam_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("GibbsParams"), contents: bytemuck::cast_slice(&[gparams]), usage: wgpu::BufferUsages::UNIFORM,
    });

    backend.run_compute(
        shaders::MCMC_GIBBS_SHADER, "main",
        &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
        ],
        &[
            wgpu::BindGroupEntry { binding: 0, resource: gparam_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: data_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: meta_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: res_buf.as_entire_binding() },
        ],
        ((domain_size as u32 + 63) / 64, 1, 1),
    )?;

    let res_f32 = backend.read_buffer_f32(&res_buf, domain_size)?;
    Ok(res_f32.iter().map(|&v| v as f64).collect())
}

#[cfg(feature = "gpu")]
pub(crate) fn gpu_mcmc_chain_step_parallel(
    backend: &WgpuBackend,
    chains: &[(Assignment, Vec<Vec<f64>>)],
    factors: &[TabularFactor],
) -> LutufiResult<Vec<(Assignment, f64)>> {
    if chains.is_empty() {
        return Ok(Vec::new());
    }

    let num_chains = chains.len();
    let num_factors = factors.len();
    if num_factors == 0 {
        return Err(LutufiError::InternalError { message: "No factors for MCMC chain step".to_string() });
    }

    // Collect all unique variable IDs from the first chain state and factors
    let mut all_var_ids: Vec<VariableId> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (state, _) in chains {
        for (&vid, _) in state.iter() {
            if seen.insert(vid) {
                all_var_ids.push(vid);
            }
        }
    }
    for f in factors {
        for &vid in f.scope().variable_ids() {
            if seen.insert(vid) {
                all_var_ids.push(vid);
            }
        }
    }

    let num_vars = all_var_ids.len();
    if num_vars == 0 {
        return Err(LutufiError::InternalError { message: "No variables in MCMC chain step".to_string() });
    }
    if num_vars > 32 {
        return Err(LutufiError::InternalError { message: "MCMC chain step supports max 32 variables".to_string() });
    }

    // Build position map: VariableId -> position index
    let pos_map: std::collections::HashMap<VariableId, u32> = all_var_ids.iter().enumerate()
        .map(|(i, &vid)| (vid, i as u32)).collect();

    // Serialize chain states: flat array [chain][var] = u32 value
    let mut chain_states_flat: Vec<u32> = vec![0u32; num_chains * num_vars];
    for (c, (state, _)) in chains.iter().enumerate() {
        for (&vid, val_str) in state.iter() {
            if let Some(&pos) = pos_map.get(&vid) {
                if let Ok(val) = val_str.parse::<u32>() {
                    chain_states_flat[c * num_vars + pos as usize] = val;
                }
            }
        }
    }

    // Build factor metadata: [num_factors, per_factor_data...]
    // Per factor: [num_entries, num_vars, var_pos_0..7, stride_0..7] = 18 u32s
    let mut factor_meta: Vec<u32> = Vec::with_capacity(1 + num_factors * 18);
    factor_meta.push(num_factors as u32);
    let mut factor_data_flat: Vec<f32> = Vec::new();

    for f in factors {
        let f_scope = f.scope();
        let f_num_entries = f_scope.num_entries() as u32;
        let f_num_vars = f_scope.len() as u32;

        factor_meta.push(f_num_entries);
        factor_meta.push(f_num_vars);

        // Variable positions (up to 8)
        let mut var_positions = [0u32; 8];
        let mut strides = [0u32; 8];
        let f_vids = f_scope.variable_ids();
        let f_sizes = f_scope.sizes();

        // Compute strides
        for i in 0..f_scope.len() {
            var_positions[i] = pos_map.get(&f_vids[i]).copied().unwrap_or(0);
            let mut stride = 1u32;
            for j in (i + 1)..f_scope.len() {
                stride *= f_sizes[j] as u32;
            }
            strides[i] = stride;
        }

        for i in 0..8 {
            factor_meta.push(var_positions[i]);
        }
        for i in 0..8 {
            factor_meta.push(strides[i]);
        }

        // Pad to 18 u32s
        while factor_meta.len() % 18 != 1 {
            factor_meta.push(0);
        }

        // Factor data: convert to f32
        let f32v = super::factor_to_f32(f);
        factor_data_flat.extend_from_slice(&f32v);
    }

    // RNG state: seed per chain
    let mut rng_state: Vec<u32> = (0..num_chains as u32).collect();

    // Create GPU buffers
    let chain_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("ChainStates"),
        contents: bytemuck::cast_slice(&chain_states_flat),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let factor_data_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("FactorData"),
        contents: bytemuck::cast_slice(&factor_data_flat),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let factor_meta_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("FactorMeta"),
        contents: bytemuck::cast_slice(&factor_meta),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let log_prob_buf = backend.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("ChainLogProbs"),
        size: (num_chains * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let rng_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("RNGState"),
        contents: bytemuck::cast_slice(&rng_state),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    struct ChainStepParamsRaw { num_chains: u32, num_vars: u32, num_factors: u32 }
    let sparams = ChainStepParamsRaw { num_chains: num_chains as u32, num_vars: num_vars as u32, num_factors: num_factors as u32 };
    let sparam_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("ChainStepParams"),
        contents: bytemuck::cast_slice(&[sparams]),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    backend.run_compute(
        shaders::MCMC_CHAIN_STEP_SHADER, "main",
        &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 5, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
        ],
        &[
            wgpu::BindGroupEntry { binding: 0, resource: sparam_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: chain_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: factor_data_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: factor_meta_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 4, resource: log_prob_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 5, resource: rng_buf.as_entire_binding() },
        ],
        ((num_chains as u32 + 63) / 64, 1, 1),
    )?;

    // Read back results
    let updated_states = backend.read_buffer_u32(&chain_buf, chain_states_flat.len())?;
    let log_probs_f32 = backend.read_buffer_f32(&log_prob_buf, num_chains)?;

    // Deserialize back to Vec<(Assignment, f64)>
    let mut results = Vec::with_capacity(num_chains);
    for c in 0..num_chains {
        let mut new_state = Assignment::new();
        for (pos, &vid) in all_var_ids.iter().enumerate() {
            let val = updated_states[c * num_vars + pos];
            new_state.set_discrete(vid, val as usize)
                .map_err(|e| LutufiError::InternalError { message: e.to_string() })?;
        }
        results.push((new_state, log_probs_f32[c] as f64));
    }

    Ok(results)
}

// Non-GPU fallbacks
#[cfg(not(feature = "gpu"))]
pub(crate) fn gpu_mcmc_gibbs_probs(
    _backend: &WgpuBackend,
    _var_id: VariableId,
    _state: &Assignment,
    _factors: &[TabularFactor],
    _domain_size: usize,
) -> LutufiResult<Vec<f64>> {
    Err(LutufiError::InternalError { message: "GPU MCMC requires 'gpu' feature".to_string() })
}

#[cfg(not(feature = "gpu"))]
pub(crate) fn gpu_mcmc_chain_step_parallel(
    _backend: &WgpuBackend,
    _chains: &[(Assignment, Vec<Vec<f64>>)],
    _factors: &[TabularFactor],
) -> LutufiResult<Vec<(Assignment, f64)>> {
    Err(LutufiError::InternalError { message: "GPU MCMC chains requires 'gpu' feature".to_string() })
}
