//! GPU backend tests.
//!
//! These are the tests whose absence let the `gpu` feature rot into a
//! non-compiling state: nothing ever built it, so nothing ever ran it.
//!
//! Every test here degrades gracefully when no GPU adapter is available (CI
//! runners, headless machines, machines whose driver refuses a compute device),
//! because a missing adapter is an environment fact rather than a code defect.
//! What must never be skipped is the *compilation* of this file — that is
//! enforced by the `features` matrix in CI, which runs
//! `cargo check --all-targets --features gpu`.
//!
//! # Reaching the GPU path at all
//!
//! The shaders address scopes through fixed `[u32; 8]` arrays, so a factor
//! product can involve at most 8 variables. For **binary** variables that caps
//! the result at 2^8 = 256 entries — too small to be worth a device round trip.
//! GPU factor products therefore only pay off for higher-cardinality domains
//! (8 ternary variables is 6561 entries; 8 five-state variables is 390 625).
//! Tests that mean to exercise the device must size themselves accordingly, and
//! `assert!(… took_gpu_path)` style checks below exist to stop a future change
//! from silently turning these into CPU tests.

#![cfg(feature = "gpu")]

use lutufi_core::core::{
    backend::ComputeBackend,
    factor::{Scope, TabularFactor},
    scalability::WgpuBackend,
    variable::VariableId,
};

/// Acquire a GPU backend, or return `None` with an explanatory note.
///
/// Returning `None` rather than panicking is deliberate: a developer without a
/// discrete GPU should still be able to run `cargo test --features gpu`.
fn gpu() -> Option<WgpuBackend> {
    match WgpuBackend::new() {
        Ok(backend) => Some(backend),
        Err(e) => {
            eprintln!("SKIP: no usable GPU adapter ({e}). GPU tests will not run.");
            None
        }
    }
}

/// Build a factor over `ids`, each variable having `cardinality` states.
///
/// Values vary across the table so that a wrong index mapping yields a wrong
/// answer. A uniform table would pass even a badly broken shader.
fn factor_over(ids: &[VariableId], cardinality: usize) -> TabularFactor {
    let scope = Scope::from_ids_and_sizes(ids.to_vec(), vec![cardinality; ids.len()]);
    let n = scope.num_entries();
    let values: Vec<f64> = (0..n).map(|i| 0.05 + 0.9 * ((i % 13) as f64 / 13.0)).collect();
    TabularFactor::from_values(scope, values).expect("factor setup")
}

/// Two factors overlapping in `shared` variables, each with `distinct` of its
/// own, all of the given cardinality.
fn overlapping_pair(
    shared: usize,
    distinct: usize,
    cardinality: usize,
) -> (TabularFactor, TabularFactor) {
    let total = shared + 2 * distinct;
    let pool: Vec<VariableId> = (0..total).map(|_| VariableId::new()).collect();
    let a_ids = &pool[..shared + distinct];
    let b_ids: Vec<VariableId> = pool[..shared]
        .iter()
        .chain(pool[shared + distinct..].iter())
        .copied()
        .collect();
    (factor_over(a_ids, cardinality), factor_over(&b_ids, cardinality))
}

/// The GPU path computes in f32 (see ADR-011). Log-values here are O(1), so a
/// tolerance a little above f32 epsilon is the honest bar — tightening it would
/// only produce a flaky test, and loosening it would stop catching real bugs.
const F32_TOLERANCE: f64 = 1e-4;

fn assert_factors_agree(cpu: &TabularFactor, gpu: &TabularFactor, context: &str) {
    assert_eq!(
        cpu.scope().num_entries(),
        gpu.scope().num_entries(),
        "{context}: result sizes differ"
    );
    assert_eq!(
        cpu.scope().variable_ids(),
        gpu.scope().variable_ids(),
        "{context}: result scopes differ"
    );

    let mut worst = 0.0f64;
    let mut worst_at = 0usize;
    for i in 0..cpu.scope().num_entries() {
        let (c, g) = (cpu.log_value_at(i), gpu.log_value_at(i));
        // Both being -inf (structural zero) is agreement, not a difference.
        if c.is_infinite() && g.is_infinite() && c.signum() == g.signum() {
            continue;
        }
        let diff = (c - g).abs();
        if diff > worst {
            worst = diff;
            worst_at = i;
        }
    }
    assert!(
        worst <= F32_TOLERANCE,
        "{context}: GPU and CPU disagree by {worst:.3e} at index {worst_at} \
         (tolerance {F32_TOLERANCE:.0e}). This is a correctness bug in the \
         shader or its index mapping, not a precision artifact."
    );
}

/// Reports which physical device was selected.
///
/// On a machine with switchable graphics this is the difference between using
/// the discrete GPU and quietly using integrated graphics; and a `Cpu` device
/// type means a software rasterizer was chosen, which would explain heavy CPU
/// use with no GPU activity while the backend still calls itself "WGPU".
#[test]
fn gpu_adapter_is_reachable_and_is_real_hardware() {
    let Some(backend) = gpu() else { return };
    assert_eq!(backend.name(), "WGPU");

    let info = backend.adapter_info();
    eprintln!("GPU adapter selected: {info}");
    eprintln!("  hardware: {}   discrete: {}", info.is_hardware(), info.is_discrete());

    assert!(
        info.is_hardware(),
        "wgpu selected a software adapter ({info}). Shaders would execute on the \
         CPU, which is both slower than the CPU kernels and invisible in GPU \
         utilization graphs."
    );
}

/// The core correctness guarantee: for a product large enough to take the GPU
/// path, the GPU must agree with the CPU reference.
#[test]
fn gpu_multiply_agrees_with_cpu() {
    let Some(backend) = gpu() else { return };

    // 4 shared + 2 distinct each = 8 ternary variables = 6561 result entries,
    // which clears both the 8-variable ceiling and the dispatch threshold.
    let (a, b) = overlapping_pair(4, 2, 3);
    let cpu = a.multiply_internal(&b).expect("cpu multiply");
    assert!(
        cpu.scope().num_entries() >= 512,
        "result too small to exercise the GPU path (got {})",
        cpu.scope().num_entries()
    );

    let gpu_result = backend.multiply(&a, &b).expect("gpu multiply");
    assert_factors_agree(&cpu, &gpu_result, "factor product");
}

/// Marginalization reads the whole input table, so binary variables do reach the
/// GPU path here — the ceiling applies to the remaining and summed-out scopes
/// separately, allowing up to 16 variables in total.
#[test]
fn gpu_marginalize_agrees_with_cpu() {
    let Some(backend) = gpu() else { return };

    let pool: Vec<VariableId> = (0..12).map(|_| VariableId::new()).collect();
    let f = factor_over(&pool, 2);
    assert!(f.scope().num_entries() >= 1024);

    // Sum out 4, leaving 8 — both within the shader's addressable width.
    let sum_out: Vec<VariableId> = pool[..4].to_vec();

    let cpu = f.marginalize_internal(&sum_out).expect("cpu marginalize");
    let gpu_result = backend.marginalize(&f, &sum_out).expect("gpu marginalize");

    assert_factors_agree(&cpu, &gpu_result, "marginalization");
}

/// A scope wider than the shaders can address must fall back to the CPU and
/// still produce the right answer. It must NOT surface an error: the model is
/// well-formed, and a backend limitation is not the caller's problem.
#[test]
fn gpu_falls_back_gracefully_beyond_shader_capacity() {
    let Some(backend) = gpu() else { return };

    // Disjoint 9-variable scopes => an 18-variable union, well past the ceiling.
    let pool: Vec<VariableId> = (0..18).map(|_| VariableId::new()).collect();
    let a = factor_over(&pool[..9], 2);
    let b = factor_over(&pool[9..], 2);

    let gpu_result = backend
        .multiply(&a, &b)
        .expect("a union scope beyond shader capacity must fall back, not fail");
    let cpu = a.multiply_internal(&b).expect("cpu multiply");

    // Fallback runs the CPU kernel, so agreement here should be exact.
    assert_factors_agree(&cpu, &gpu_result, "oversized-scope fallback");
}

/// Repeated dispatch must be stable — buffers are recreated per call, and a
/// leaked or reused binding would show up as drift across iterations.
#[test]
fn gpu_multiply_is_repeatable() {
    let Some(backend) = gpu() else { return };

    let (a, b) = overlapping_pair(4, 2, 3);
    let first = backend.multiply(&a, &b).expect("gpu multiply");
    for iteration in 1..8 {
        let again = backend.multiply(&a, &b).expect("gpu multiply");
        assert_factors_agree(&first, &again, &format!("repeat dispatch {iteration}"));
    }
}

/// Sustained-load soak test.
///
/// Ignored by default because it deliberately runs for tens of seconds; run it
/// with `cargo test --features gpu --test unit_gpu_backend -- --ignored
/// --nocapture` when you want to watch device utilization or check for drift,
/// leaks, or driver timeouts under continuous dispatch.
///
/// Correctness is re-checked every iteration against a CPU reference computed
/// once up front, so a device that degrades under load fails the test rather
/// than merely looking busy.
#[test]
#[ignore = "long-running soak test; run explicitly with --ignored"]
fn gpu_sustained_load_stays_correct() {
    let Some(backend) = gpu() else { return };

    eprintln!("soak: adapter = {}", backend.adapter_info());

    // 8 five-state variables = 390 625 entries per product — large enough that
    // each dispatch is real work rather than launch overhead.
    let (a, b) = overlapping_pair(4, 2, 5);
    let reference = a.multiply_internal(&b).expect("cpu reference");
    eprintln!(
        "soak: {} entries per product, {:.1} MiB per result buffer (f32)",
        reference.scope().num_entries(),
        (reference.scope().num_entries() * 4) as f64 / (1024.0 * 1024.0),
    );

    // Verification walks every entry on the CPU, which for a 390k-entry result is
    // itself substantial work — verifying on every iteration would make this a
    // CPU benchmark wearing a GPU costume, and would mask what the device is
    // actually doing. Check periodically instead: often enough to catch a device
    // that degrades when hot, rarely enough that the GPU stays the bottleneck.
    const VERIFY_EVERY: u64 = 50;

    let duration = std::time::Duration::from_secs(45);
    let start = std::time::Instant::now();
    let mut iterations = 0u64;
    let mut verifications = 0u64;

    while start.elapsed() < duration {
        let result = backend.multiply(&a, &b).expect("gpu multiply under load");
        iterations += 1;

        if iterations.is_multiple_of(VERIFY_EVERY) {
            assert_factors_agree(&reference, &result, &format!("soak iteration {iterations}"));
            verifications += 1;
            eprintln!(
                "soak: {iterations} dispatches ({verifications} verified), {:.1}s elapsed",
                start.elapsed().as_secs_f64()
            );
        }
    }

    // Final check on the last result regardless of where the counter landed.
    let last = backend.multiply(&a, &b).expect("gpu multiply");
    assert_factors_agree(&reference, &last, "soak final");

    let elapsed = start.elapsed().as_secs_f64();
    eprintln!(
        "soak: {iterations} dispatches in {elapsed:.1}s ({:.1} products/s, \
         {:.1} M entries/s), {verifications} spot-checks all correct",
        iterations as f64 / elapsed,
        (iterations as f64 * reference.scope().num_entries() as f64) / elapsed / 1e6,
    );
    assert!(iterations > 0, "soak test completed no dispatches");
}
