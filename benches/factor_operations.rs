use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lutufi_core::core::{
    factor::{Scope, TabularFactor},
    variable::VariableId,
};

fn create_large_factor(vars_count: usize) -> TabularFactor {
    let ids: Vec<VariableId> = (0..vars_count).map(|_| VariableId::new()).collect();
    let sizes: Vec<usize> = vec![2; ids.len()];
    let scope = Scope::from_ids_and_sizes(ids, sizes);
    let num_entries = scope.num_entries();
    let values = vec![0.5; num_entries];
    TabularFactor::from_values(scope, values).unwrap()
}

fn bench_multiplication(c: &mut Criterion) {
    // CPU bench: 18 variables = 262K entries (~118 ms/op on CPU)
    let f1_cpu = create_large_factor(18);
    let f2_cpu = create_large_factor(18);

    let mut group = c.benchmark_group("Factor Multiplication");

    group.bench_function("CPU (18 vars)", |b| b.iter(|| {
        black_box(f1_cpu.multiply(black_box(&f2_cpu))).unwrap();
    }));

    #[cfg(feature = "gpu")]
    {
        if initialize_gpu().is_ok() {
            // GPU bench: 24 variables = 16M entries per factor
            let f1_gpu = create_large_factor(24);
            let f2_gpu = create_large_factor(24);

            group.bench_function("GPU (24 vars, 10x loop)", |b| b.iter(|| {
                for _ in 0..10 {
                    black_box(f1_gpu.multiply(black_box(&f2_gpu))).unwrap();
                }
            }));
        }
    }

    group.finish();
}

fn bench_marginalization(c: &mut Criterion) {
    // CPU bench: 18 variables = 262K entries
    let f_cpu = create_large_factor(18);
    let to_sum_out_cpu = vec![
        f_cpu.scope().variable_ids()[0],
        f_cpu.scope().variable_ids()[1],
        f_cpu.scope().variable_ids()[2],
    ];

    let mut group = c.benchmark_group("Factor Marginalization");

    group.bench_function("CPU (18 vars)", |b| b.iter(|| {
        black_box(f_cpu.marginalize(black_box(&to_sum_out_cpu))).unwrap();
    }));

    #[cfg(feature = "gpu")]
    {
        if lutufi_core::core::backend::get_backend().name() == "WGPU" {
            let f_gpu = create_large_factor(24);
            let to_sum_out_gpu = vec![
                f_gpu.scope().variable_ids()[0],
                f_gpu.scope().variable_ids()[1],
                f_gpu.scope().variable_ids()[2],
            ];

            group.bench_function("GPU (24 vars, 10x loop)", |b| b.iter(|| {
                for _ in 0..10 {
                    black_box(f_gpu.marginalize(black_box(&to_sum_out_gpu))).unwrap();
                }
            }));
        }
    }

    group.finish();
}

criterion_group!(benches, bench_multiplication, bench_marginalization);
criterion_main!(benches);
