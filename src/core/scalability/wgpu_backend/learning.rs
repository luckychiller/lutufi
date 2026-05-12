use super::WgpuBackend;
#[cfg(feature = "gpu")]
use super::shaders;

use crate::core::error::{LutufiError, LutufiResult};

#[cfg(feature = "gpu")]
use wgpu::util::DeviceExt;

#[cfg(feature = "gpu")]
pub(crate) fn gpu_accumulate_counts(
    backend: &WgpuBackend,
    data_rows: &[Vec<f64>],
    scope_num_entries: usize,
) -> LutufiResult<Vec<f64>> {
    if data_rows.is_empty() || scope_num_entries == 0 {
        return Err(LutufiError::InternalError { message: "Invalid count accumulation inputs".to_string() });
    }
    if data_rows.len() < 64 {
        return Err(LutufiError::InternalError { message: "GPU count accumulation needs more data".to_string() });
    }

    let row_stride = data_rows[0].len();
    let mut flat: Vec<f32> = Vec::with_capacity(data_rows.len() * row_stride);
    for row in data_rows {
        for &v in row {
            flat.push(v as f32);
        }
    }

    let data_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Count Data"), contents: bytemuck::cast_slice(&flat), usage: wgpu::BufferUsages::STORAGE,
    });
    let counts_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Counts"),
        contents: bytemuck::cast_slice(&vec![0u32; scope_num_entries]),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    struct CountParamsRaw { num_rows: u32, num_entries: u32, row_stride: u32 }
    let cparams = CountParamsRaw { num_rows: data_rows.len() as u32, num_entries: scope_num_entries as u32, row_stride: row_stride as u32 };
    let cparam_buf = backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("CountParams"), contents: bytemuck::cast_slice(&[cparams]), usage: wgpu::BufferUsages::UNIFORM,
    });

    backend.run_compute(
        shaders::PARAM_COUNT_ATOMIC_SHADER, "main",
        &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
        ],
        &[
            wgpu::BindGroupEntry { binding: 0, resource: cparam_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: data_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: counts_buf.as_entire_binding() },
        ],
        ((data_rows.len() as u32 + 63) / 64, 1, 1),
    )?;

    let res_u32 = backend.read_buffer_u32(&counts_buf, scope_num_entries)?;
    Ok(res_u32.iter().map(|&v| v as f64).collect())
}

#[cfg(not(feature = "gpu"))]
pub(crate) fn gpu_accumulate_counts(
    _backend: &WgpuBackend,
    _data_rows: &[Vec<f64>],
    _scope_num_entries: usize,
) -> LutufiResult<Vec<f64>> {
    Err(LutufiError::InternalError { message: "GPU count accumulation requires 'gpu' feature".to_string() })
}
