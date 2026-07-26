use super::{WgpuBackend};
#[cfg(feature = "gpu")]
use super::{
    shaders,
    shaders::{GpuFactorParams, GpuMarginalizeParams},
    factor_to_f32, get_strides, get_stride,
};

#[cfg(feature = "gpu")]
use crate::core::factor::Scope;

use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::{TabularFactor},
    variable::VariableId,
};

#[cfg(feature = "gpu")]
use wgpu::util::DeviceExt;

/// Widest scope the shaders can address, set by the `[u32; 8]` stride and map
/// arrays in [`GpuFactorParams`] / [`GpuMarginalizeParams`].
#[cfg(feature = "gpu")]
pub(crate) const GPU_MAX_SCOPE_VARS: usize = 8;

/// Result size at or above which a dispatch is worth its buffer upload, kernel
/// launch, and readback.
///
/// This is deliberately compared against the size of the **result**, not the
/// operands. Comparing against the operands made the GPU path unreachable for
/// binary variables: the widest expressible union scope is 8 variables, which is
/// only 2^8 = 256 entries, so both operands were always under any threshold above
/// 256 and every product silently fell back to the CPU. The result is the
/// quantity the kernel actually computes, so it is the quantity that decides
/// whether dispatching is worthwhile.
#[cfg(feature = "gpu")]
pub(crate) const GPU_MIN_RESULT_ENTRIES: usize = 512;

#[cfg(feature = "gpu")]
pub(crate) fn gpu_multiply(
    backend: &WgpuBackend,
    a: &TabularFactor,
    b: &TabularFactor,
) -> LutufiResult<TabularFactor> {
    let a_scope = a.scope();
    let b_scope = b.scope();

    let mut vars: std::collections::BTreeMap<VariableId, usize> = std::collections::BTreeMap::new();
    for (i, &vid) in a_scope.variable_ids().iter().enumerate() {
        vars.insert(vid, a_scope.sizes()[i]);
    }
    for (i, &vid) in b_scope.variable_ids().iter().enumerate() {
        if let Some(&es) = vars.get(&vid) {
            if es != b_scope.sizes()[i] {
                return Err(LutufiError::InternalError {
                    message: format!("Size mismatch: {} has {} and {}", vid, es, b_scope.sizes()[i])
                });
            }
        } else {
            vars.insert(vid, b_scope.sizes()[i]);
        }
    }

    // The shaders index through fixed-size `[u32; 8]` stride/map arrays, so a
    // union scope wider than 8 variables cannot be expressed. That is a property
    // of this backend, not an error in the caller's model — fall back to the CPU
    // kernel rather than failing a query that is perfectly well-formed.
    if vars.len() > GPU_MAX_SCOPE_VARS {
        return a.multiply_internal(b);
    }

    let new_vids: Vec<VariableId> = vars.keys().copied().collect();
    let new_sizes: Vec<usize> = vars.values().copied().collect();
    let res_scope = Scope::from_ids_and_sizes(new_vids, new_sizes);

    // Too small to be worth a round trip to the device.
    if res_scope.num_entries() < GPU_MIN_RESULT_ENTRIES {
        return a.multiply_internal(b);
    }

    let mut p = GpuFactorParams {
        num_entries: res_scope.num_entries() as u32,
        num_vars: res_scope.len() as u32,
        sizes: [0; 8],
        strides_a: [0; 8],
        strides_b: [0; 8],
        strides_res: [0; 8],
        map_a: [-1; 8],
        map_b: [-1; 8],
    };
    for i in 0..res_scope.len() { p.sizes[i] = res_scope.sizes()[i] as u32; }
    p.strides_a = get_strides(a_scope.sizes());
    p.strides_b = get_strides(b_scope.sizes());
    p.strides_res = get_strides(res_scope.sizes());
    for (i, &rv) in res_scope.variable_ids().iter().enumerate() {
        if let Some(pos) = a_scope.variable_ids().iter().position(|&v| v == rv) { p.map_a[i] = pos as i32; }
        if let Some(pos) = b_scope.variable_ids().iter().position(|&v| v == rv) { p.map_b[i] = pos as i32; }
    }

    let a_f32 = factor_to_f32(a);
    let b_f32 = factor_to_f32(b);

    let a_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("A"), contents: bytemuck::cast_slice(&a_f32), usage: wgpu::BufferUsages::STORAGE,
    });
    let b_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("B"), contents: bytemuck::cast_slice(&b_f32), usage: wgpu::BufferUsages::STORAGE,
    });
    let res_buf = backend.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Res"), size: (res_scope.num_entries() * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false,
    });
    let param_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("P"), contents: bytemuck::cast_slice(&[p]), usage: wgpu::BufferUsages::STORAGE,
    });

    backend.run_compute(
        shaders::MULTIPLY_SHADER, "main",
        &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
        ],
        &[
            wgpu::BindGroupEntry { binding: 0, resource: param_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: a_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: b_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: res_buf.as_entire_binding() },
        ],
        ((res_scope.num_entries() as u32).div_ceil(64), 1, 1),
    )?;

    let res_f32 = backend.read_buffer_f32(&res_buf, res_scope.num_entries())?;
    let res_f64: Vec<f64> = res_f32.iter().map(|&v| v as f64).collect();
    Ok(TabularFactor::Dense { scope: res_scope, log_table: res_f64 })
}

#[cfg(feature = "gpu")]
pub(crate) fn gpu_marginalize(
    backend: &WgpuBackend,
    a: &TabularFactor,
    variables: &[VariableId],
) -> LutufiResult<TabularFactor> {
    let a_scope = a.scope();
    // Marginalization reads the whole input table, so the input size is the right
    // measure of the work involved (unlike the product, where it is the output).
    if a_scope.num_entries() < GPU_MIN_RESULT_ENTRIES * 2 {
        return a.marginalize_internal(variables);
    }

    let target: std::collections::HashSet<_> = variables.iter().collect();
    let mut remain_v = Vec::new();
    let mut remain_s = Vec::new();
    let mut sum_v = Vec::new();
    let mut sum_s = Vec::new();

    for (i, &vid) in a_scope.variable_ids().iter().enumerate() {
        if !target.contains(&vid) {
            remain_v.push(vid); remain_s.push(a_scope.sizes()[i]);
        } else {
            sum_v.push(vid); sum_s.push(a_scope.sizes()[i]);
        }
    }

    if remain_v.len() == a_scope.len() { return Ok(a.clone()); }
    // Beyond the shaders' addressable width, fall back rather than fail: the
    // query is well-formed, this backend just cannot express it.
    if remain_v.len() > GPU_MAX_SCOPE_VARS || sum_v.len() > GPU_MAX_SCOPE_VARS {
        return a.marginalize_internal(variables);
    }

    let res_scope = Scope::from_ids_and_sizes(remain_v, remain_s);
    let sum_scope = Scope::from_ids_and_sizes(sum_v, sum_s);

    let mut p = GpuMarginalizeParams {
        num_entries_res: res_scope.num_entries() as u32,
        num_entries_sum: sum_scope.num_entries() as u32,
        num_vars_res: res_scope.len() as u32,
        num_vars_sum: sum_scope.len() as u32,
        sizes_res: [0; 8], strides_a_res: [0; 8],
        sizes_sum: [0; 8], strides_a_sum: [0; 8],
    };
    for i in 0..res_scope.len() {
        p.sizes_res[i] = res_scope.sizes()[i] as u32;
        let pos = a_scope.variable_ids().iter().position(|&v| v == res_scope.variable_ids()[i])
            .ok_or_else(|| LutufiError::InternalError {
                message: "Variable not found in factor scope".to_string()
            })?;
        p.strides_a_res[i] = get_stride(a_scope.sizes(), pos);
    }
    for i in 0..sum_scope.len() {
        p.sizes_sum[i] = sum_scope.sizes()[i] as u32;
        let pos = a_scope.variable_ids().iter().position(|&v| v == sum_scope.variable_ids()[i])
            .ok_or_else(|| LutufiError::InternalError {
                message: "Variable not found in factor scope".to_string()
            })?;
        p.strides_a_sum[i] = get_stride(a_scope.sizes(), pos);
    }

    let a_f32 = factor_to_f32(a);
    let a_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("A"), contents: bytemuck::cast_slice(&a_f32), usage: wgpu::BufferUsages::STORAGE,
    });
    let res_buf = backend.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Res"), size: (res_scope.num_entries() * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false,
    });
    let param_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("P"), contents: bytemuck::cast_slice(&[p]), usage: wgpu::BufferUsages::STORAGE,
    });

    backend.run_compute(
        shaders::MARGINALIZE_SHADER, "main",
        &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
        ],
        &[
            wgpu::BindGroupEntry { binding: 0, resource: param_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: a_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: res_buf.as_entire_binding() },
        ],
        ((res_scope.num_entries() as u32).div_ceil(64), 1, 1),
    )?;

    let res_f32 = backend.read_buffer_f32(&res_buf, res_scope.num_entries())?;
    let res_f64: Vec<f64> = res_f32.iter().map(|&v| v as f64).collect();
    Ok(TabularFactor::Dense { scope: res_scope, log_table: res_f64 })
}

#[cfg(feature = "gpu")]
pub(crate) fn gpu_normalize(
    _backend: &WgpuBackend,
    factor: &mut TabularFactor,
) {
    factor.normalize_internal();
}

/// Multiply many factor pairs.
///
/// Currently dispatches each pair as its own GPU submission. A single fused
/// dispatch over all pairs is a future optimization; the sequential form is kept
/// because it is correct and because per-pair dispatch already amortizes well for
/// the large factors that reach the GPU path at all.
#[cfg(feature = "gpu")]
pub(crate) fn gpu_batch_multiply(
    backend: &WgpuBackend,
    pairs: &[(TabularFactor, TabularFactor)],
) -> LutufiResult<Vec<TabularFactor>> {
    pairs.iter().map(|(a, b)| gpu_multiply(backend, a, b)).collect()
}

/// Marginalize many factors. See [`gpu_batch_multiply`] on batching.
#[cfg(feature = "gpu")]
pub(crate) fn gpu_batch_marginalize(
    backend: &WgpuBackend,
    factors: &[(TabularFactor, Vec<VariableId>)],
) -> LutufiResult<Vec<TabularFactor>> {
    factors.iter().map(|(f, v)| gpu_marginalize(backend, f, v)).collect()
}

// Non-GPU fallbacks (used when feature is disabled, called from mod.rs)
#[cfg(not(feature = "gpu"))]
pub(crate) fn gpu_normalize(
    _backend: &WgpuBackend,
    _factor: &mut TabularFactor,
) {
}

#[cfg(not(feature = "gpu"))]
pub(crate) fn gpu_multiply(
    _backend: &WgpuBackend,
    _a: &TabularFactor,
    _b: &TabularFactor,
) -> LutufiResult<TabularFactor> {
    Err(LutufiError::InternalError { message: "WGPU backend requires 'gpu' feature".to_string() })
}

#[cfg(not(feature = "gpu"))]
pub(crate) fn gpu_marginalize(
    _backend: &WgpuBackend,
    _a: &TabularFactor,
    _variables: &[VariableId],
) -> LutufiResult<TabularFactor> {
    Err(LutufiError::InternalError { message: "WGPU backend requires 'gpu' feature".to_string() })
}

#[cfg(not(feature = "gpu"))]
pub(crate) fn gpu_batch_multiply(
    _backend: &WgpuBackend,
    _pairs: &[(TabularFactor, TabularFactor)],
) -> LutufiResult<Vec<TabularFactor>> {
    Err(LutufiError::InternalError { message: "WGPU backend requires 'gpu' feature".to_string() })
}

#[cfg(not(feature = "gpu"))]
pub(crate) fn gpu_batch_marginalize(
    _backend: &WgpuBackend,
    _factors: &[(TabularFactor, Vec<VariableId>)],
) -> LutufiResult<Vec<TabularFactor>> {
    Err(LutufiError::InternalError { message: "WGPU backend requires 'gpu' feature".to_string() })
}
