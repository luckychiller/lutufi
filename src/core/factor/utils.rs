use crate::core::error::{LutufiError, LutufiResult};
use crate::core::variable::VariableId;

pub fn log_sum_exp(a: f64, b: f64) -> f64 {
    if a.is_infinite() && a.is_sign_negative() { return b; }
    if b.is_infinite() && b.is_sign_negative() { return a; }
    let max = a.max(b);
    max + ((a - max).exp() + (b - max).exp()).ln()
}

pub fn multi_index_from_flat(flat: usize, sizes: &[usize]) -> Vec<usize> {
    let mut result = vec![0; sizes.len()];
    let mut remainder = flat;
    for i in (0..sizes.len()).rev() {
        result[i] = remainder % sizes[i];
        remainder /= sizes[i];
    }
    result
}

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
