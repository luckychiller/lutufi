# Contributing to Lutufi

Thank you for your interest in contributing to Lutufi! This project aims to provide mathematically rigorous tools for complex network analysis.

## Code Style Requirements

- **Python**: Use `black` for formatting and `isort` for import sorting. We use `mypy` for static type checking.
- **Rust**: Use `cargo fmt` and `cargo clippy`.
- **Commit Messages**: Use the [Conventional Commits](https://www.conventionalcommits.org/) format. Example: `feat(inference): add variable elimination algorithm`.

## How to Run Tests

### Python
```bash
pytest tests/
```

### Rust
```bash
cargo test
```

## Adding a New Ground Truth Case

1. Create a JSON file in `tests/ground_truth/`.
2. Follow the established schema: name, nodes, edges, CPTs/potentials, and queries with expected results.
3. Ensure the expected results are analytically derived or sourced from trusted literature.
4. Run `pytest tests/test_ground_truth.py` to verify the runner picks it up.

## Adding a New Example

1. Place your example script in `examples/`.
2. Include a brief README or header comment explaining the domain and the model.
3. If the example uses specific data, place it in a subdirectory.

## Pull Request Process

1. Fork the repository and create your branch from `main`.
2. Ensure all tests pass (including ground truth tests, which should `xfail` only if inference isn't implemented).
3. Update documentation if you're adding features or changing APIs.
4. Submit the PR and wait for review.

## Architecture & Design

Please refer to `docs/design/` for the core architectural principles of the project. Every PR should align with these principles.
