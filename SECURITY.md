# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

Lutufi is a probabilistic inference library. While we take security seriously, the primary risks are:

1. **Malicious model files** — LMF, BIF, XMLBIF, and UAI files that trigger excessive memory allocation or code paths that could crash the process.
2. **Denial of service** — Crafted models that trigger exponential-time algorithms (e.g., variable elimination on high-treewidth models).

### What to Expect

- **File format parsers** validate all inputs before processing. Malformed files produce `DeserializationError` with clear messages, not crashes.
- **Resource limits** are enforced via `ResourceBudget`: configurable caps on node count, edge count, CPT size, memory usage, and inference time. Exceeding a limit raises `ResourceLimitExceeded`, not an OOM crash.
- **No arbitrary code execution** — Lutufi does not use `eval`, dynamic code loading, or any mechanism that could execute attacker-controlled code. All file formats are pure data formats.

### Fuzzing

The file format parsers (LMF, BIF, XMLBIF, UAI) are designed to be fuzzed. We recommend running `cargo fuzz` against the `from_json`, `import`, and `import_from_file` entry points before deployment.

### Reporting Process

If you discover a security vulnerability, please report it by opening a [GitHub Security Advisory](https://github.com/luckychiller/lutufi/security/advisories) rather than a public issue.

Please include:
- A description of the vulnerability
- Steps to reproduce (including any crafted input file)
- The version of Lutufi affected
- Any suggested fix (if known)

### Response Timeline

- **24 hours**: Acknowledgment of receipt
- **7 days**: Initial assessment and mitigation plan
- **30 days**: Fix released (depends on severity and complexity)

## Security-Related Configuration

### Environment Variables

- `LUTUFI_NUM_THREADS`: Limits parallelism (default: min(num_cpus, 8)). Setting this to a low value can prevent resource exhaustion on shared systems.

### ResourceBudget

All inference operations accept a `&ResourceBudget` parameter that controls:

| Field                  | Default  | Description                          |
|------------------------|----------|--------------------------------------|
| `max_memory_mb`        | 4096     | Maximum memory before limit error    |
| `max_inference_time_secs` | 3600  | Maximum wall-clock time for inference|
| `max_nodes`            | 100,000  | Maximum nodes in a model             |
| `max_edges`            | 1,000,000| Maximum edges in a model             |
| `max_cpt_size`         | 10,000,000| Maximum entries in a single CPT     |

Set these conservatively in multi-tenant or untrusted-input environments.

## Known Security Considerations

1. **Variable Elimination** on a model with treewidth > 30 can require gigabytes of memory. The `max_cpt_size` budget cap prevents this.
2. **LMF files** are JSON and can be arbitrarily large. The JSON parser (`serde_json`) is memory-safe but not streaming; very large files will be loaded entirely into memory.
3. **No sandboxing**: Lutufi does not sandbox its computation. In a multi-tenant environment, use operating-system-level isolation (containers, processes, resource limits).

## Responsible Disclosure

We follow responsible disclosure: we ask that you do not publicize a vulnerability until we have had reasonable time to address it.
