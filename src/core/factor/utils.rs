use crate::core::error::{LutufiError, LutufiResult};
use crate::core::variable::VariableId;

/// Computes `log(exp(a) + exp(b))` in a numerically stable way.
pub fn log_sum_exp(a: f64, b: f64) -> f64 {
    if a.is_infinite() && a.is_sign_negative() { return b; }
    if b.is_infinite() && b.is_sign_negative() { return a; }
    let max = a.max(b);
    max + ((a - max).exp() + (b - max).exp()).ln()
}

/// Converts a flat index into a multi-dimensional index given dimension sizes.
pub fn multi_index_from_flat(flat: usize, sizes: &[usize]) -> Vec<usize> {
    let mut result = vec![0; sizes.len()];
    let mut remainder = flat;
    for i in (0..sizes.len()).rev() {
        result[i] = remainder % sizes[i];
        remainder /= sizes[i];
    }
    result
}

/// Row-major strides for a table of the given dimension sizes.
///
/// `strides[i]` is how far the flat index moves per unit increment of dimension
/// `i`. The last dimension varies fastest, matching [`multi_index_from_flat`].
pub fn row_major_strides(sizes: &[usize]) -> Vec<usize> {
    let mut strides = vec![1usize; sizes.len()];
    for i in (0..sizes.len().saturating_sub(1)).rev() {
        strides[i] = strides[i + 1] * sizes[i + 1];
    }
    strides
}

/// For each variable of `outer`, the stride that variable induces in a table laid
/// out over `inner`.
///
/// A variable absent from `inner` yields stride 0, which is exactly broadcasting:
/// advancing along that axis does not move within the inner table. Computing
/// these once turns the per-element variable lookup in [`project_indices`] into a
/// per-*factor* cost.
pub fn broadcast_strides(
    outer_vars: &[VariableId],
    inner_vars: &[VariableId],
    inner_sizes: &[usize],
) -> Vec<usize> {
    let inner_strides = row_major_strides(inner_sizes);
    outer_vars
        .iter()
        .map(|v| {
            inner_vars
                .iter()
                .position(|iv| iv == v)
                .map_or(0, |p| inner_strides[p])
        })
        .collect()
}

/// A mixed-radix odometer over a multi-dimensional index space.
///
/// Walking the output table in flat order means incrementing the last dimension
/// most often and carrying into earlier ones only occasionally. Tracking that
/// carry explicitly lets each operand's offset be updated by addition rather than
/// recomputed from scratch, which is what removes the per-element allocation and
/// index arithmetic from the factor kernels.
///
/// `offsets` holds one running flat offset per attached table, advanced in step.
pub struct StridedOdometer<'a> {
    sizes: &'a [usize],
    counter: Vec<usize>,
}

impl<'a> StridedOdometer<'a> {
    /// Create an odometer over `sizes`, positioned at the all-zero index.
    pub fn new(sizes: &'a [usize]) -> Self {
        StridedOdometer {
            sizes,
            counter: vec![0; sizes.len()],
        }
    }

    /// Advance one step, updating each `offsets[t]` by `strides[t]`.
    ///
    /// `strides[t][k]` is the step table `t` takes along dimension `k`. Both
    /// slices must have one entry per attached table.
    #[inline]
    pub fn step(&mut self, strides: &[&[usize]], offsets: &mut [usize]) {
        for k in (0..self.sizes.len()).rev() {
            self.counter[k] += 1;
            for (t, off) in offsets.iter_mut().enumerate() {
                *off += strides[t][k];
            }
            if self.counter[k] < self.sizes[k] {
                return;
            }
            // Dimension `k` wrapped: rewind it fully and carry into `k - 1`.
            for (t, off) in offsets.iter_mut().enumerate() {
                *off -= strides[t][k] * self.sizes[k];
            }
            self.counter[k] = 0;
        }
    }
}

/// Projects a multi-dimensional index from a full variable set onto a subset and returns the corresponding flat index.
pub fn project_indices(
    full_indices: &[usize],
    full_vars: &[VariableId],
    sub_vars: &[VariableId],
    sub_sizes: &[usize],
) -> LutufiResult<usize> {
    let mut flat = 0;
    let mut stride = 1;
    for i in (0..sub_vars.len()).rev() {
        let var_id = sub_vars[i];
        let full_pos = full_vars.iter().position(|&id| id == var_id)
            .ok_or_else(|| LutufiError::InternalError {
                message: format!("Variable {:?} not found in full scope for projection", var_id),
            })?;
        flat += full_indices[full_pos] * stride;
        stride *= sub_sizes[i];
    }
    Ok(flat)
}
