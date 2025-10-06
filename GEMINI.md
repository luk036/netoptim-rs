# GEMINI Project Analysis: netoptim-rs

## Project Overview

This project, `netoptim-rs`, is a Rust-based library for network optimization algorithms. The primary goal is to provide efficient implementations of common network optimization algorithms. The project is in its early stages of development.

**Key Technologies:**

*   **Language:** Rust
*   **Package Manager:** Cargo
*   **Core Dependencies:**
    *   `petgraph`: For graph data structures and algorithms.
    *   `serde_json`: For serialization and deserialization of data.
    *   `num` and `num-traits`: For numerical operations.

### Core Functionality

The library provides the following core functions:

*   `bellman_ford`: Computes the shortest paths from a source node to all other nodes in a graph using the Bellman-Ford algorithm. It can handle negative edge weights, but will return an error if the graph contains a negative cycle.
*   `find_negative_cycle`: Finds a negative cycle in a graph reachable from a source node. It returns an `Option<Vec<NodeId>>` containing the path of the negative cycle if one is found.

### Negative Cycle Detection

The library provides the `NegCycleFinder` struct for detecting negative cycles in a directed graph using Howard's algorithm.

*   **`NegCycleFinder` struct:** This struct provides a `howard` method that implements Howard's algorithm for finding negative cycles. It is more efficient than the Bellman-Ford algorithm for this purpose.

### Parametric Search

The library also provides a framework for parametric search algorithms on graphs. This is exposed through the `ParametricAPI` trait and the `MaxParametricSolver` struct.

*   **`ParametricAPI` trait:** This trait defines the `distance` and `zero_cancel` methods, which are used to define the cost function for the parametric search.
*   **`MaxParametricSolver` struct:** This struct implements a generic framework for finding the minimum ratio and corresponding cycle in a graph, given a `ParametricAPI` implementation.
## Building and Running

### Prerequisites

*   Rust toolchain (including `cargo`)

### Installation

To install the library, run the following command:

```bash
cargo install netoptim-rs
```

### Building

To build the project from the source, use the following command:

```bash
cargo build
```

### Running Tests

To run the test suite, use the following command:

```bash
cargo test
```

## Development Conventions

### Contribution Guidelines

The project has a `CONTRIBUTING.md` file that outlines the contribution process. Key points include:

*   Discussing changes by creating a new issue before making non-straightforward contributions.
*   Creating one pull request per change.
*   Updating the `CHANGELOG.md` file under the "Unreleased" section for any changes made.

All contributions are expected to be dual-licensed under Apache 2.0 and MIT licenses.

### Code Style

The project follows standard Rust conventions. The following commands are useful for maintaining code quality:

*   **Check formatting:** `cargo fmt --all -- --check`
*   **Format code:** `cargo fmt --all`
*   **Run Clippy (linter):** `cargo clippy --all-targets --all-features --workspace`

### Useful Commands

*   **Build and run release version:** `cargo build --release && cargo run --release`
*   **Run all tests:** `cargo test --all-features --workspace`
