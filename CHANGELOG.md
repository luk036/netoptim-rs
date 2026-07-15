# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `network_oracle` module: NetworkOracle for feasibility assessment and cutting-plane generation via Howard's negative cycle detection (matches C++/Python NetworkOracle)
- `optscaling_oracle` module: OptScalingOracle for optimal matrix scaling using cutting-plane methods (matches C++/Python OptScalingOracle)
- `min_cycle_ratio` module: min_cycle_ratio function for minimum cost-to-time cycle ratio via parametric search (matches C++ min_cycle_ratio)

### Changed
- Renamed `grph` parameter to `gra` in `MaxParametricSolver::new` for naming consistency with sibling projects
