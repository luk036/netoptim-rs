# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `network_oracle` module: NetworkOracle for feasibility assessment and cutting-plane generation via Howard's negative cycle detection (matches C++/Python NetworkOracle)
- `optscaling_oracle` module: OptScalingOracle for optimal matrix scaling using cutting-plane methods (matches C++/Python OptScalingOracle)
- `min_cycle_ratio` module: min_cycle_ratio function for minimum cost-to-time cycle ratio via parametric search (matches C++ min_cycle_ratio)
- `OracleFn::make_weight_fn` optional hook (with `WeightFn` alias) letting edge oracles bind per-iterate state once for the Howard hot loop; `Ratio` overrides it (matches the Python/C++ `make_weight_fn` fast path)
- `NegCycleFinder::howard_with_max_iter`: bounded variant of Howard's method that returns the new `MaxIterExceeded` error instead of looping unboundedly on degenerate graphs; `howard` is unchanged
- `solve` module: `solve_network_feas` / `solve_opt_scaling` facades plus `Options`, `default_options()`, and `DEFAULT_TOLERANCE` (1e-10) that drive the oracles with the ellalgo-rs cutting-plane method (matches the Python/C++ `solve_*` facades); new `ellalgo-rs` dependency

### Changed
- Renamed `grph` parameter to `gra` in `MaxParametricSolver::new` for naming consistency with sibling projects
