# Agents Guidance

## Verified Commands

```bash
# Full test suite
cargo test --workspace

# With all features (for CI/complete coverage)
cargo test --all-features --workspace

# Clippy check
cargo clippy --all-targets --workspace

# Format check
cargo fmt --all --check

# Documentation (CI enforces -D warnings)
cargo doc --no-deps --document-private-items --workspace --examples
```

## CI Pipeline Order

CI runs: test → rustfmt → clippy → docs. All must pass.

## Feature Flags

- `std` is **default** (includes logging support via `log` and `env_logger`)
- `log` and `env_logger` are optional, only available with `std` feature

## Changelog Workflow

Update `CHANGELOG.md` under **Unreleased** section before every PR. Use subsections: Added, Changed, Deprecated, Removed, Fixed, Security.

## Project Structure

```
src/
├── lib.rs          # Entry point + bellman_ford + find_negative_cycle
├── dijkstra.rs     # Dijkstra implementation
├── neg_cycle.rs    # Howard's algorithm for negative cycle detection
├── parametric.rs   # Parametric optimization framework
├── utils.rs        # Graph utilities, serialization, DOT export
└── error.rs        # Error types
```

## Key Dependencies

- `petgraph` (0.8.0): Graph data structures, serde feature enabled
- `num` / `num-traits`: Rational numbers (`Ratio<i32>`) for exact arithmetic
- `serde_json`: JSON serialization for graphs

## Examples

```bash
cargo run --example dijkstra_example
cargo run --example neg_cycle_example
cargo run --example find_negative_cycle
cargo run --example quickcheck_tests
```

## Benchmarks

Uses `criterion`. Run with `cargo bench`. Named groups: `dijkstra_sparse`, `dijkstra_dense`, `dijkstra_path`, `neg_cycle_finder`, `graph_creation`, `algorithm_comparison`.