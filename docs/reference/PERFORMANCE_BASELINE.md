# Measured Performance Baseline

**Status:** Living record
**First captured:** 2026-07-25
**Purpose:** Real, reproducible numbers to replace and check the unsourced claims
in [TECHNICAL_SUMMARY.md](../TECHNICAL_SUMMARY.md).

Every number here was produced by a command recorded alongside it. Nothing in this
file is estimated, extrapolated, or carried over from a design document.

---

## Reference hardware

| Component | Value |
|---|---|
| GPU | NVIDIA GeForce MX250 (2 GB dedicated), driver 582.42, Vulkan backend |
| OS | Windows 11 Pro 22000 |
| Build | `--release` (`opt-level=3`, `lto=true`, `codegen-units=1`) |

The MX250 is an entry-level mobile GPU. Treat it as a *lower* bound on GPU
capability and a useful stress case: if a workload does not beat the CPU here, the
GPU path is not carrying its weight on modest hardware.

---

## Baseline: CPU factor algebra (pre-WS-1.1)

Captured **before** the vectorized tensor kernel, against the scalar
`multiply_internal` / `marginalize_internal` implementations that allocate a `Vec`
per output element.

```
cargo bench --bench factor_operations -- --warm-up-time 1 --measurement-time 3
```

### Factor product

| Result scope | Result entries | Time (median) | Throughput |
|---|---:|---:|---:|
| 10 binary vars | 1 024 | 274 µs | ~3.7 M entries/s |
| 16 binary vars | 65 536 | 33.3 ms | ~2.0 M entries/s |
| 20 binary vars | 1 048 576 | 689 ms | ~1.5 M entries/s |
| 24 binary vars | 16 777 216 | 18.0 s | ~0.9 M entries/s |

### Marginalization

| Input scope | Summed out | Time (median) |
|---|---:|---:|
| 10 binary vars | 3 | 240 µs |
| 14 binary vars | 4 | 5.08 ms |
| 18 binary vars | 6 | 103 ms |
| 20 binary vars | 6 | 568 ms |

**Reading of these numbers.** Throughput *falls* as the problem grows — the
opposite of what a cache-friendly kernel does. The per-element `Vec` allocation in
`project_indices` dominates, and allocator pressure worsens with size. An
18-second factor product is not a tolerable operation inside an inference loop.
This is the case for WS-1.1.

---

## Baseline: GPU factor product

```
cargo test --release --features gpu --test unit_gpu_backend -- --ignored --nocapture
```

| Metric | Value |
|---|---:|
| Adapter | NVIDIA GeForce MX250 [Vulkan, DiscreteGpu] |
| Product size | 8 five-state variables = 390 625 entries |
| Sustained rate | 231 products/s |
| Throughput | **90.1 M entries/s** |
| Duration | 45 s continuous, 207 spot-checks all matching the CPU reference |

Includes per-dispatch buffer creation, f64→f32 upload, and f32→f64 readback — it
is end-to-end cost, not kernel time.

---

## After WS-1.1: vectorized CPU tensor kernel

Same benchmark, same machine, after replacing the per-element index projection
with precomputed broadcast strides and an odometer traversal.

### Factor product

| Result entries | Before | After | Speedup | Throughput after |
|---:|---:|---:|---:|---:|
| 1 024 | 274 µs | **16.0 µs** | **17×** | 64 M entries/s |
| 65 536 | 33.3 ms | **1.10 ms** | **30×** | 60 M entries/s |
| 1 048 576 | 689 ms | **17.8 ms** | **39×** | 59 M entries/s |
| 16 777 216 | 18.0 s | **276 ms** | **65×** | 61 M entries/s |

### Marginalization

| Input scope | Summed out | Before | After | Speedup |
|---|---:|---:|---:|---:|
| 10 binary vars | 3 | 240 µs | **40.7 µs** | 5.9× |
| 14 binary vars | 4 | 5.08 ms | **709 µs** | 7.2× |
| 18 binary vars | 6 | 103 ms | **10.1 ms** | 10.2× |
| 20 binary vars | 6 | 568 ms | **42.2 ms** | 13.4× |

**Throughput now holds flat at ~60 M entries/s across four orders of magnitude**,
where before it *degraded* from 3.7 to 0.9 M entries/s as problems grew. That
flatness is the real result: it says the kernel is bound by memory traffic rather
than by allocator pressure, which is what a factor product should be bound by.

---

## What the comparison shows now

| | Throughput |
|---|---:|
| CPU (vectorized, f64) | ~61 M entries/s |
| GPU (MX250, f32) | ~90 M entries/s |

**Roughly 1.5×, not 90×.** Against the original scalar kernel the GPU looked
transformative; against a competent CPU kernel it is a modest gain that costs f32
precision (ADR-011) and caps scopes at 8 variables.

This is exactly why the plan sequenced the tensor kernel before GPU optimization.
Had the order been reversed, the GPU's apparent 90× would have justified
substantial investment in a path whose real advantage — on this hardware — barely
clears measurement noise for many workloads. A larger GPU would change the
arithmetic; the methodological point stands regardless.

**Consequences already taken:** GPU acceleration stays opt-in (ADR-011), and
further GPU work is not scheduled ahead of profiling real inference workloads.

---

## Notes on the benchmark itself

The previous version of `benches/factor_operations.rs` built its two operands from
independent `VariableId::new()` calls, giving them disjoint scopes. Its "18
variable" product therefore had a 36-variable union — 2^36 entries — and could
never have completed. The benchmark had never been run. The current version fixes
shared variable pools so scopes overlap, as they do in a real model, and reports
result size rather than operand size.
