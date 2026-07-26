//! Factor algebra benchmarks.
//!
//! These measure the two operations that dominate every inference algorithm in
//! Lutufi: factor product and marginalization.
//!
//! **Scope overlap is the point.** An earlier version of this benchmark built its
//! two operands with independent `VariableId::new()` calls, giving them disjoint
//! scopes — so the "18 variable" product actually had a 36-variable union and
//! 2^36 entries. Real factors in a PGM share variables; a product whose operands
//! share nothing is both unrepresentative and astronomically large. Every case
//! below therefore fixes a shared variable pool and slices overlapping scopes out
//! of it, and the reported size is the size of the *result*.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lutufi_core::core::{
    factor::{Scope, TabularFactor},
    variable::VariableId,
};

/// A fixed pool of binary variables shared by all operands in a benchmark case.
fn variable_pool(n: usize) -> Vec<VariableId> {
    (0..n).map(|_| VariableId::new()).collect()
}

/// Build a dense factor over `ids`, filled with a non-uniform, non-degenerate
/// distribution. Uniform 0.5 values let a compiler or a future SIMD path hit
/// unrealistically friendly cases; varying values keeps the measurement honest.
fn factor_over(ids: &[VariableId]) -> TabularFactor {
    let sizes = vec![2usize; ids.len()];
    let scope = Scope::from_ids_and_sizes(ids.to_vec(), sizes);
    let n = scope.num_entries();
    let values: Vec<f64> = (0..n)
        .map(|i| 0.1 + 0.8 * ((i % 7) as f64 / 7.0))
        .collect();
    TabularFactor::from_values(scope, values)
        .expect("factor construction must succeed in benchmark setup")
}

/// Two factors whose scopes overlap in `shared` variables, with `distinct`
/// variables unique to each. The product spans `shared + 2 * distinct` variables.
fn overlapping_pair(
    pool: &[VariableId],
    shared: usize,
    distinct: usize,
) -> (TabularFactor, TabularFactor) {
    let a_ids: Vec<VariableId> = pool[..shared + distinct].to_vec();
    let b_ids: Vec<VariableId> = pool[..shared]
        .iter()
        .chain(pool[shared + distinct..shared + 2 * distinct].iter())
        .copied()
        .collect();
    (factor_over(&a_ids), factor_over(&b_ids))
}

fn bench_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("factor_product");

    // (shared, distinct) -> result spans shared + 2*distinct binary variables.
    for &(shared, distinct) in &[(4usize, 3usize), (6, 5), (8, 6), (10, 7)] {
        let result_vars = shared + 2 * distinct;
        let pool = variable_pool(shared + 2 * distinct);
        let (a, b) = overlapping_pair(&pool, shared, distinct);

        group.throughput(criterion::Throughput::Elements(1u64 << result_vars));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!(
                "{result_vars}vars_{}entries",
                1usize << result_vars
            )),
            &(a, b),
            |bench, (a, b)| {
                bench.iter(|| black_box(a.multiply_internal(black_box(b))).unwrap());
            },
        );
    }

    group.finish();
}

fn bench_marginalization(c: &mut Criterion) {
    let mut group = c.benchmark_group("factor_marginalize");

    for &n_vars in &[10usize, 14, 18, 20] {
        let pool = variable_pool(n_vars);
        let f = factor_over(&pool);
        // Sum out roughly a third of the scope — a typical elimination step.
        let sum_out: Vec<VariableId> = pool[..n_vars / 3].to_vec();

        group.throughput(criterion::Throughput::Elements(1u64 << n_vars));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{n_vars}vars_sumout{}", n_vars / 3)),
            &(f, sum_out),
            |bench, (f, sum_out)| {
                bench.iter(|| black_box(f.marginalize_internal(black_box(sum_out))).unwrap());
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_multiplication, bench_marginalization);
criterion_main!(benches);
