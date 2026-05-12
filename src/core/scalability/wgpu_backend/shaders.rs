#[cfg(feature = "gpu")]
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GpuFactorParams {
    pub(crate) num_entries: u32,
    pub(crate) num_vars: u32,
    pub(crate) sizes: [u32; 8],
    pub(crate) strides_a: [u32; 8],
    pub(crate) strides_b: [u32; 8],
    pub(crate) strides_res: [u32; 8],
    pub(crate) map_a: [i32; 8],
    pub(crate) map_b: [i32; 8],
}

#[cfg(feature = "gpu")]
pub(crate) const MULTIPLY_SHADER: &str = "
struct GpuFactorParams {
    num_entries: u32,
    num_vars: u32,
    sizes: array<u32, 8>,
    strides_a: array<u32, 8>,
    strides_b: array<u32, 8>,
    strides_res: array<u32, 8>,
    map_a: array<i32, 8>,
    map_b: array<i32, 8>,
};

@group(0) @binding(0) var<uniform> params: GpuFactorParams;
@group(0) @binding(1) var<storage, read> a_data: array<f32>;
@group(0) @binding(2) var<storage, read> b_data: array<f32>;
@group(0) @binding(3) var<storage, read_write> result_data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let res_idx = global_id.x;
    if (res_idx >= params.num_entries) { return; }

    var a_idx: u32 = 0u;
    var b_idx: u32 = 0u;
    var temp = res_idx;

    for (var i: u32 = 0u; i < params.num_vars; i = i + 1u) {
        let rev = params.num_vars - 1u - i;
        let val = temp % params.sizes[rev];
        temp = temp / params.sizes[rev];

        if (params.map_a[rev] >= 0) {
            a_idx = a_idx + val * params.strides_a[u32(params.map_a[rev])];
        }
        if (params.map_b[rev] >= 0) {
            b_idx = b_idx + val * params.strides_b[u32(params.map_b[rev])];
        }
    }
    result_data[res_idx] = a_data[a_idx] + b_data[b_idx];
}
";

#[cfg(feature = "gpu")]
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct GpuMarginalizeParams {
    pub(crate) num_entries_res: u32,
    pub(crate) num_entries_sum: u32,
    pub(crate) num_vars_res: u32,
    pub(crate) num_vars_sum: u32,
    pub(crate) sizes_res: [u32; 8],
    pub(crate) strides_a_res: [u32; 8],
    pub(crate) sizes_sum: [u32; 8],
    pub(crate) strides_a_sum: [u32; 8],
}

#[cfg(feature = "gpu")]
pub(crate) const MARGINALIZE_SHADER: &str = "
struct GpuMarginalizeParams {
    num_entries_res: u32,
    num_entries_sum: u32,
    num_vars_res: u32,
    num_vars_sum: u32,
    sizes_res: array<u32, 8>,
    strides_a_res: array<u32, 8>,
    sizes_sum: array<u32, 8>,
    strides_a_sum: array<u32, 8>,
};

@group(0) @binding(0) var<uniform> params: GpuMarginalizeParams;
@group(0) @binding(1) var<storage, read> a_data: array<f32>;
@group(0) @binding(2) var<storage, read_write> result_data: array<f32>;

fn log_sum_exp_gpu(a: f32, b: f32) -> f32 {
    let neg_inf = -1.0e38;
    if (a <= neg_inf) { return b; }
    if (b <= neg_inf) { return a; }
    let m = max(a, b);
    return m + log(exp(a - m) + exp(b - m));
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let res_idx = global_id.x;
    if (res_idx >= params.num_entries_res) { return; }

    var base_a: u32 = 0u;
    var tmp = res_idx;
    for (var i: u32 = 0u; i < params.num_vars_res; i = i + 1u) {
        let rev = params.num_vars_res - 1u - i;
        let val = tmp % params.sizes_res[rev];
        tmp = tmp / params.sizes_res[rev];
        base_a = base_a + val * params.strides_a_res[rev];
    }

    var sum: f32 = -1.0e38;
    for (var s: u32 = 0u; s < params.num_entries_sum; s = s + 1u) {
        var off: u32 = 0u;
        var ts = s;
        for (var j: u32 = 0u; j < params.num_vars_sum; j = j + 1u) {
            let rev = params.num_vars_sum - 1u - j;
            let val = ts % params.sizes_sum[rev];
            ts = ts / params.sizes_sum[rev];
            off = off + val * params.strides_a_sum[rev];
        }
        sum = log_sum_exp_gpu(sum, a_data[base_a + off]);
    }
    result_data[res_idx] = sum;
}
";

#[cfg(feature = "gpu")]
pub(crate) const BATCH_MULTIPLY_SHADER: &str = "
struct BatchHeader {
    num_pairs: u32,
    max_entries: u32,
};
@group(0) @binding(0) var<uniform> header: BatchHeader;
@group(0) @binding(1) var<storage, read> offsets: array<u32>;
@group(0) @binding(2) var<storage, read> params: array<u32>;
@group(0) @binding(3) var<storage, read> a_data: array<f32>;
@group(0) @binding(4) var<storage, read> b_data: array<f32>;
@group(0) @binding(5) var<storage, read_write> result_data: array<f32>;

fn unpack_params(base: u32) -> (u32, u32, array<u32,8>, array<u32,8>, array<u32,8>, array<u32,8>, array<i32,8>, array<i32,8>) {
    var num_entries = params[base];
    var num_vars = params[base + 1u];
    var sizes: array<u32,8>;
    var strides_a: array<u32,8>;
    var strides_b: array<u32,8>;
    var strides_res: array<u32,8>;
    var map_a: array<i32,8>;
    var map_b: array<i32,8>;
    for (var i: u32 = 0u; i < 8u; i = i + 1u) {
        sizes[i] = params[base + 2u + i];
        strides_a[i] = params[base + 10u + i];
        strides_b[i] = params[base + 18u + i];
        strides_res[i] = params[base + 26u + i];
        map_a[i] = i32(params[base + 34u + i]);
        map_b[i] = i32(params[base + 42u + i]);
    }
    return (num_entries, num_vars, sizes, strides_a, strides_b, strides_res, map_a, map_b);
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let batch_idx = gid.y;
    if (batch_idx >= header.num_pairs) { return; }

    let (num_entries, num_vars, sizes, strides_a, strides_b, strides_res, map_a, map_b) = unpack_params(batch_idx * 50u);

    let res_idx = gid.x;
    if (res_idx >= num_entries) { return; }

    let a_off = offsets[batch_idx * 2u];
    let b_off = offsets[batch_idx * 2u + 1u];

    var a_idx: u32 = 0u;
    var b_idx: u32 = 0u;
    var tmp = res_idx;
    for (var i: u32 = 0u; i < num_vars; i = i + 1u) {
        let rev = num_vars - 1u - i;
        let val = tmp % sizes[rev];
        tmp = tmp / sizes[rev];
        if (map_a[rev] >= 0) { a_idx = a_idx + val * strides_a[u32(map_a[rev])]; }
        if (map_b[rev] >= 0) { b_idx = b_idx + val * strides_b[u32(map_b[rev])]; }
    }
    result_data[batch_idx * header.max_entries + res_idx] = a_data[a_off + a_idx] + b_data[b_off + b_idx];
}
";

#[cfg(feature = "gpu")]
pub(crate) const BATCH_MARGINALIZE_SHADER: &str = "
struct BatchMargHeader {
    num_pairs: u32,
    max_res_entries: u32,
    max_sum_entries: u32,
};
@group(0) @binding(0) var<uniform> header: BatchMargHeader;
@group(0) @binding(1) var<storage, read> offsets: array<u32>;
@group(0) @binding(2) var<storage, read> params: array<u32>;
@group(0) @binding(3) var<storage, read> a_data: array<f32>;
@group(0) @binding(4) var<storage, read_write> result_data: array<f32>;

fn log_sum_exp_gpu(a: f32, b: f32) -> f32 {
    let neg_inf = -1.0e38;
    if (a <= neg_inf) { return b; }
    if (b <= neg_inf) { return a; }
    let m = max(a, b);
    return m + log(exp(a - m) + exp(b - m));
}

fn unpack_marg(base: u32) -> (u32, u32, u32, u32, array<u32,8>, array<u32,8>, array<u32,8>, array<u32,8>) {
    var num_entries_res = params[base];
    var num_entries_sum = params[base + 1u];
    var num_vars_res = params[base + 2u];
    var num_vars_sum = params[base + 3u];
    var sizes_res: array<u32,8>;
    var strides_res: array<u32,8>;
    var sizes_sum: array<u32,8>;
    var strides_sum: array<u32,8>;
    for (var i: u32 = 0u; i < 8u; i = i + 1u) {
        sizes_res[i] = params[base + 4u + i];
        strides_res[i] = params[base + 12u + i];
        sizes_sum[i] = params[base + 20u + i];
        strides_sum[i] = params[base + 28u + i];
    }
    return (num_entries_res, num_entries_sum, num_vars_res, num_vars_sum, sizes_res, strides_res, sizes_sum, strides_sum);
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let batch_idx = gid.y;
    if (batch_idx >= header.num_pairs) { return; }

    let (num_entries_res, num_entries_sum, num_vars_res, num_vars_sum, sizes_res, strides_res, sizes_sum, strides_sum) = unpack_marg(batch_idx * 36u);

    let res_idx = gid.x;
    if (res_idx >= num_entries_res) { return; }

    let a_off = offsets[batch_idx];

    var base_a: u32 = a_off;
    var tmp = res_idx;
    for (var i: u32 = 0u; i < num_vars_res; i = i + 1u) {
        let rev = num_vars_res - 1u - i;
        let val = tmp % sizes_res[rev];
        tmp = tmp / sizes_res[rev];
        base_a = base_a + val * strides_res[rev];
    }

    var sum: f32 = -1.0e38;
    for (var s: u32 = 0u; s < num_entries_sum; s = s + 1u) {
        var off: u32 = 0u;
        var ts = s;
        for (var j: u32 = 0u; j < num_vars_sum; j = j + 1u) {
            let rev = num_vars_sum - 1u - j;
            let val = ts % sizes_sum[rev];
            ts = ts / sizes_sum[rev];
            off = off + val * strides_sum[rev];
        }
        sum = log_sum_exp_gpu(sum, a_data[base_a + off]);
    }
    result_data[batch_idx * header.max_res_entries + res_idx] = sum;
}
";

#[cfg(feature = "gpu")]
pub(crate) const MCMC_GIBBS_SHADER: &str = "
struct GibbsParams {
    domain_size: u32,
    num_factors: u32,
    flat_idx_stride: u32,
};
@group(0) @binding(0) var<uniform> params: GibbsParams;
@group(0) @binding(1) var<storage, read> factor_data: array<f32>;
@group(0) @binding(2) var<storage, read> factor_meta: array<u32>;
@group(0) @binding(3) var<storage, read_write> log_probs: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let state_idx = gid.x;
    if (state_idx >= params.domain_size) { return; }

    var log_prob: f32 = 0.0;
    var meta_off: u32 = 0u;
    for (var f: u32 = 0u; f < params.num_factors; f = f + 1u) {
        let num_entries = factor_meta[meta_off];
        let num_vars = factor_meta[meta_off + 1u];
        meta_off = meta_off + 2u;

        var flat_idx: u32 = state_idx;
        if (flat_idx < num_entries) {
            log_prob = log_prob + factor_data[meta_off * 0u + flat_idx];
        }
        meta_off = meta_off + num_vars * 2u;
    }
    log_probs[state_idx] = log_prob;
}
";

#[cfg(feature = "gpu")]
pub(crate) const MCMC_CHAIN_STEP_SHADER: &str = "
struct ChainStepParams {
    num_chains: u32,
    num_vars: u32,
    num_factors: u32,
};
@group(0) @binding(0) var<uniform> params: ChainStepParams;
@group(0) @binding(1) var<storage, read_write> chain_states: array<u32>;
@group(0) @binding(2) var<storage, read> factor_data: array<f32>;
@group(0) @binding(3) var<storage, read> factor_meta: array<u32>;
@group(0) @binding(4) var<storage, read_write> chain_log_probs: array<f32>;
@group(0) @binding(5) var<storage, read_write> rng_state: array<u32>;

fn lcg_rand(state: ptr<function, u32>) -> f32 {
    let s = *state;
    *state = s * 1103515245u + 12345u;
    return f32(*state & 0x7fffffffu) / f32(0x7fffffffu);
}

fn compute_log_prob(chain_idx: u32, target_var: i32, target_val: u32) -> f32 {
    var lp: f32 = 0.0;
    var meta_off: u32 = 1u;
    for (var f: u32 = 0u; f < params.num_factors; f = f + 1u) {
        let num_entries = factor_meta[meta_off];
        let num_vars_f = factor_meta[meta_off + 1u];
        var flat_idx: u32 = 0u;
        for (var v: u32 = 0u; v < num_vars_f; v = v + 1u) {
            let var_id = i32(factor_meta[meta_off + 2u + v]);
            let stride = factor_meta[meta_off + 10u + v];
            let val = select(
                chain_states[chain_idx * params.num_vars + u32(var_id)],
                target_val,
                var_id == target_var
            );
            flat_idx = flat_idx + val * stride;
        }
        if (flat_idx < num_entries) {
            let factor_offset = f * num_entries;
            lp = lp + factor_data[factor_offset + flat_idx];
        }
        meta_off = meta_off + 18u;
    }
    return lp;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let chain_idx = gid.x;
    if (chain_idx >= params.num_chains) { return; }

    var rng = rng_state[chain_idx];
    let base = chain_idx * params.num_vars;

    // Single-variable Gibbs: pick a random variable
    let var_idx = u32(floor(lcg_rand(&rng) * f32(params.num_vars)));

    // Compute full conditional for this variable (binary: 2 states)
    let lp0 = compute_log_prob(chain_idx, i32(var_idx), 0u);
    let lp1 = compute_log_prob(chain_idx, i32(var_idx), 1u);

    // Softmax to get probabilities (in log space, then normalize)
    let max_lp = max(lp0, lp1);
    let w0 = exp(lp0 - max_lp);
    let w1 = exp(lp1 - max_lp);
    let p0 = w0 / (w0 + w1);

    // Sample from conditional
    let u = lcg_rand(&rng);
    let new_val = select(1u, 0u, u < p0);
    chain_states[base + var_idx] = new_val;

    // Compute new log-probability
    let new_lp = select(lp1, lp0, u < p0);

    rng_state[chain_idx] = rng;
    chain_log_probs[chain_idx] = new_lp;
}
";

#[cfg(feature = "gpu")]
pub(crate) const PARAM_COUNT_SHADER: &str = "
struct CountParams {
    num_rows: u32,
    num_entries: u32,
    row_stride: u32,
};
@group(0) @binding(0) var<uniform> params: CountParams;
@group(0) @binding(1) var<storage, read> data_rows: array<f32>;
@group(0) @binding(2) var<storage, read_write> counts: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let row_idx = gid.x;
    if (row_idx >= params.num_rows) { return; }
    let flat_idx = u32(data_rows[row_idx * params.row_stride]);
    if (flat_idx < params.num_entries) {
        counts[flat_idx] = counts[flat_idx] + 1.0;
    }
}
";

#[cfg(feature = "gpu")]
pub(crate) const PARAM_COUNT_ATOMIC_SHADER: &str = "
struct CountParams {
    num_rows: u32,
    num_entries: u32,
    row_stride: u32,
};
@group(0) @binding(0) var<uniform> params: CountParams;
@group(0) @binding(1) var<storage, read> data_rows: array<f32>;
@group(0) @binding(2) var<storage, read_write> counts: array<atomic<u32>>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let row_idx = gid.x;
    if (row_idx >= params.num_rows) { return; }
    let flat_idx = u32(data_rows[row_idx * params.row_stride]);
    if (flat_idx < params.num_entries) {
        atomicAdd(&counts[flat_idx], 1u);
    }
}
";
