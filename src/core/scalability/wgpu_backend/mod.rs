#[cfg(feature = "gpu")]
use wgpu;

pub(crate) mod shaders;
mod compute;
mod mcmc;
mod learning;

use crate::core::{
    assignment::Assignment,
    error::{LutufiError, LutufiResult},
    factor::TabularFactor,
    variable::VariableId,
    backend::ComputeBackend,
};

/// Identity of the physical device a [`WgpuBackend`] is running on.
///
/// Exposed because "is the GPU actually being used?" is otherwise unanswerable
/// from outside the library. On laptops with switchable graphics, `wgpu` may
/// select integrated graphics over the discrete card; and a `device_type` of
/// `Cpu` means a software adapter was chosen, in which case the "GPU" backend is
/// really a slow CPU backend and should not be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuAdapterInfo {
    /// Device name as reported by the driver, e.g. `"NVIDIA GeForce MX250"`.
    pub name: String,
    /// Graphics API in use, e.g. `"Vulkan"`, `"Dx12"`, `"Gl"`.
    pub backend: String,
    /// One of `"DiscreteGpu"`, `"IntegratedGpu"`, `"VirtualGpu"`, `"Cpu"`, `"Other"`.
    pub device_type: String,
    /// PCI vendor ID.
    pub vendor: u32,
    /// PCI device ID.
    pub device: u32,
    /// Driver name and version, when the backend reports them.
    pub driver: String,
}

impl GpuAdapterInfo {
    /// Whether this is a real GPU rather than a software rasterizer.
    ///
    /// A `false` here explains the otherwise baffling combination of high CPU
    /// use and zero GPU use while the "GPU backend" is active.
    pub fn is_hardware(&self) -> bool {
        self.device_type != "Cpu"
    }

    /// Whether this is a discrete GPU (as opposed to integrated or software).
    pub fn is_discrete(&self) -> bool {
        self.device_type == "DiscreteGpu"
    }
}

impl std::fmt::Display for GpuAdapterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}, {}] vendor=0x{:04x} device=0x{:04x} driver={}",
            self.name, self.backend, self.device_type, self.vendor, self.device, self.driver
        )
    }
}

/// GPU acceleration backend using wgpu compute shaders.
pub struct WgpuBackend {
    #[cfg(feature = "gpu")]
    pub(crate) device: wgpu::Device,
    #[cfg(feature = "gpu")]
    pub(crate) queue: wgpu::Queue,
    #[cfg(feature = "gpu")]
    adapter_info: GpuAdapterInfo,
}

impl WgpuBackend {
    /// Create a new GPU backend by requesting a high-performance wgpu adapter.
    ///
    /// `PowerPreference::HighPerformance` asks for the discrete GPU on machines
    /// with switchable graphics, but the driver is free to hand back integrated
    /// graphics or even a software adapter. Inspect [`WgpuBackend::adapter_info`]
    /// to see what was actually selected, or use
    /// [`WgpuBackend::new_requiring_hardware`] to refuse a software fallback.
    #[cfg(feature = "gpu")]
    pub fn new() -> LutufiResult<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })).ok_or_else(|| LutufiError::InternalError {
            message: "Failed to find a suitable GPU adapter".to_string()
        })?;

        let info = adapter.get_info();
        let adapter_info = GpuAdapterInfo {
            name: info.name.clone(),
            backend: format!("{:?}", info.backend),
            device_type: format!("{:?}", info.device_type),
            vendor: info.vendor,
            device: info.device,
            driver: if info.driver_info.is_empty() {
                info.driver.clone()
            } else {
                format!("{} {}", info.driver, info.driver_info)
            },
        };

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Lutufi Wgpu Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )).map_err(|e| LutufiError::InternalError { message: e.to_string() })?;

        Ok(WgpuBackend { device, queue, adapter_info })
    }

    /// Like [`WgpuBackend::new`], but fails rather than returning a backend
    /// bound to a software adapter.
    ///
    /// A software adapter runs the "GPU" shaders on the CPU — usually slower
    /// than the plain CPU kernels, and confusing to diagnose because the backend
    /// reports itself as `"WGPU"` while no GPU activity is visible.
    #[cfg(feature = "gpu")]
    pub fn new_requiring_hardware() -> LutufiResult<Self> {
        let backend = Self::new()?;
        if !backend.adapter_info.is_hardware() {
            return Err(LutufiError::InternalError {
                message: format!(
                    "Selected adapter is a software rasterizer ({}), not a GPU. \
                     Shaders would run on the CPU.",
                    backend.adapter_info
                ),
            });
        }
        Ok(backend)
    }

    /// Which physical device this backend is running on.
    #[cfg(feature = "gpu")]
    pub fn adapter_info(&self) -> &GpuAdapterInfo {
        &self.adapter_info
    }

    /// Create a new GPU backend (fallback when the `gpu` feature is disabled).
    #[cfg(not(feature = "gpu"))]
    pub fn new() -> LutufiResult<Self> {
        Err(LutufiError::InternalError {
            message: "WgpuBackend requested but 'gpu' feature is not enabled".to_string()
        })
    }

    /// Create a hardware-only GPU backend (fallback when `gpu` is disabled).
    #[cfg(not(feature = "gpu"))]
    pub fn new_requiring_hardware() -> LutufiResult<Self> {
        Self::new()
    }

    /// Run a WGSL compute shader with the given bind group layout and entries.
    #[cfg(feature = "gpu")]
    pub(crate) fn run_compute(
        &self,
        shader_src: &str,
        entry_point: &str,
        bind_group_layout_entries: &[wgpu::BindGroupLayoutEntry],
        bind_group_entries: &[wgpu::BindGroupEntry],
        dispatch_size: (u32, u32, u32),
    ) -> LutufiResult<()> {
        let layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: bind_group_layout_entries,
        });
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: bind_group_entries,
        });
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });
        let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point,
            cache: None,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch_workgroups(dispatch_size.0, dispatch_size.1, dispatch_size.2);
        }
        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// Read back a buffer of u32 values from the GPU, blocking until complete.
    #[cfg(feature = "gpu")]
    pub(crate) fn read_buffer_u32(&self, buffer: &wgpu::Buffer, size: usize) -> LutufiResult<Vec<u32>> {
        let staging = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("StagingU32"),
            size: (size * 4) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(buffer, 0, &staging, 0, (size * 4) as u64);
        self.queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |v| { let _ = sender.send(v); });
        self.device.poll(wgpu::Maintain::Wait);
        match receiver.recv() {
            Ok(Ok(())) => {
                let data = slice.get_mapped_range();
                let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
                drop(data);
                staging.unmap();
                Ok(result)
            }
            _ => Err(LutufiError::InternalError { message: "Failed to read back u32 from GPU".to_string() })
        }
    }

    /// Read back a buffer of f32 values from the GPU, blocking until complete.
    #[cfg(feature = "gpu")]
    pub(crate) fn read_buffer_f32(&self, buffer: &wgpu::Buffer, size: usize) -> LutufiResult<Vec<f32>> {
        let staging = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging"),
            size: (size * 4) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(buffer, 0, &staging, 0, (size * 4) as u64);
        self.queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |v| { let _ = sender.send(v); });
        self.device.poll(wgpu::Maintain::Wait);
        match receiver.recv() {
            Ok(Ok(())) => {
                let data = slice.get_mapped_range();
                let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
                drop(data);
                staging.unmap();
                Ok(result)
            }
            _ => Err(LutufiError::InternalError { message: "Failed to read back from GPU".to_string() })
        }
    }
}

// ===== Helpers =====

#[cfg(feature = "gpu")]
pub(crate) fn factor_to_f32(factor: &TabularFactor) -> Vec<f32> {
    match factor {
        TabularFactor::Dense { log_table, .. } => log_table.iter().map(|&v| v as f32).collect(),
        TabularFactor::Sparse { .. } => {
            let mut dense = vec![f32::NEG_INFINITY; factor.scope().num_entries()];
            for i in 0..factor.scope().num_entries() {
                dense[i] = factor.log_value_at(i) as f32;
            }
            dense
        }
    }
}

#[cfg(feature = "gpu")]
pub(crate) fn get_strides(sizes: &[usize]) -> [u32; 8] {
    let mut strides = [0; 8];
    let mut s = 1;
    for i in (0..sizes.len()).rev() {
        strides[i] = s as u32;
        s *= sizes[i];
    }
    strides
}

#[cfg(feature = "gpu")]
pub(crate) fn get_stride(sizes: &[usize], pos: usize) -> u32 {
    let mut stride = 1;
    for i in (pos + 1)..sizes.len() {
        stride *= sizes[i];
    }
    stride as u32
}

// ===== Backend Implementation =====

impl ComputeBackend for WgpuBackend {
    fn name(&self) -> &'static str { "WGPU" }

    /// The shaders operate on `f32` buffers (see ADR-011).
    fn precision(&self) -> crate::core::backend::Precision {
        crate::core::backend::Precision::F32
    }

    fn multiply(&self, a: &TabularFactor, b: &TabularFactor) -> LutufiResult<TabularFactor> {
        compute::gpu_multiply(self, a, b)
    }

    fn marginalize(&self, a: &TabularFactor, variables: &[VariableId]) -> LutufiResult<TabularFactor> {
        compute::gpu_marginalize(self, a, variables)
    }

    fn normalize(&self, factor: &mut TabularFactor) {
        compute::gpu_normalize(self, factor);
    }

    fn batch_multiply(&self, pairs: &[(TabularFactor, TabularFactor)]) -> LutufiResult<Vec<TabularFactor>> {
        compute::gpu_batch_multiply(self, pairs)
    }

    fn batch_marginalize(&self, factors: &[(TabularFactor, Vec<VariableId>)]) -> LutufiResult<Vec<TabularFactor>> {
        compute::gpu_batch_marginalize(self, factors)
    }

    fn mcmc_gibbs_probs(
        &self,
        var_id: VariableId,
        state: &Assignment,
        factors: &[TabularFactor],
        domain_size: usize,
    ) -> LutufiResult<Vec<f64>> {
        mcmc::gpu_mcmc_gibbs_probs(self, var_id, state, factors, domain_size)
    }

    fn mcmc_chain_step_parallel(
        &self,
        chains: &[(Assignment, Vec<Vec<f64>>)],
        factors: &[TabularFactor],
    ) -> LutufiResult<Vec<(Assignment, f64)>> {
        mcmc::gpu_mcmc_chain_step_parallel(self, chains, factors)
    }

    fn accumulate_counts(
        &self,
        data_rows: &[Vec<f64>],
        scope_num_entries: usize,
    ) -> LutufiResult<Vec<f64>> {
        learning::gpu_accumulate_counts(self, data_rows, scope_num_entries)
    }
}
