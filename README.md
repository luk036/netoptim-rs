# 🌊 netoptim-rs

[![Crates.io](https://img.shields.io/crates/v/netoptim-rs.svg)](https://crates.io/crates/netoptim-rs)
[![Docs.rs](https://docs.rs/netoptim-rs/badge.svg)](https://docs.rs/netoptim-rs)
[![CI](https://github.com/luk036/netoptim-rs/workflows/CI/badge.svg)](https://github.com/luk036/netoptim-rs/actions)
[![codecov](https://codecov.io/gh/luk036/netoptim-rs/branch/main/graph/badge.svg?token=bamdGjpTmm)](https://codecov.io/gh/luk036/netoptim-rs)

A comprehensive Rust library for network optimization algorithms, built on top of the excellent `petgraph` library.

## 📋 Features

- **Bellman-Ford Algorithm**: Shortest paths with negative edge weight support
- **Negative Cycle Detection**: Efficient detection of negative cycles in directed graphs
- **Parametric Optimization**: Maximum parametric optimization for ratio-based problems
- **Dijkstra's Algorithm**: Fast shortest path computation for non-negative weights
- **Howard's Algorithm**: Polynomial-time negative cycle finding
- **Graph Utilities**: Comprehensive utilities for graph analysis and manipulation
- **Serialization**: JSON-based graph serialization and deserialization
- **Visualization**: DOT format export for Graphviz visualization

## 🚀 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
netoptim-rs = "0.1"
```

Or install via cargo:

```bash
cargo install netoptim-rs
```

## 📚 Quick Start

### Bellman-Ford Algorithm

```rust
use petgraph::Graph;
use petgraph::prelude::*;
use netoptim_rs::bellman_ford;

let mut g = Graph::new();
let a = g.add_node(());
let b = g.add_node(());
let c = g.add_node(());
g.extend_with_edges(&[
    (0, 1, 2.0),
    (1, 2, 1.0),
    (2, 0, -3.0), // Negative edge
]);

match bellman_ford(&g, a) {
    Ok(paths) => {
        println!("Shortest paths: {:?}", paths.distances);
    }
    Err(_) => {
        println!("Negative cycle detected!");
    }
}
```

### Dijkstra's Algorithm

```rust
use petgraph::Graph;
use petgraph::prelude::*;
use netoptim_rs::dijkstra::{dijkstra, dijkstra_path};

let mut g = Graph::new();
let a = g.add_node(());
let b = g.add_node(());
let c = g.add_node(());
g.extend_with_edges(&[(0, 1, 2.0), (1, 2, 3.0)]);

// Compute all shortest paths from node A
let result = dijkstra(&g, a).unwrap();
println!("Distances: {:?}", result.distances);

// Find shortest path from A to C
let path = dijkstra_path(&g, a, c);
println!("Path: {:?}", path);
```

### Negative Cycle Detection

```rust
use petgraph::graph::DiGraph;
use netoptim_rs::neg_cycle::NegCycleFinder;
use num::rational::Ratio;

let digraph = DiGraph::<(), Ratio<i32>>::from_edges(&[
    (0, 1, Ratio::new(1, 1)),
    (1, 2, Ratio::new(1, 1)),
    (2, 0, Ratio::new(-3, 1)), // Negative cycle
]);

let mut ncf = NegCycleFinder::new(&digraph);
let mut dist = [Ratio::new(0, 1); 3];

if let Some(cycle) = ncf.howard(&mut dist, |e| *e.weight()) {
    println!("Negative cycle found!");
    for edge in cycle {
        println!("  {} -> {}", edge.source().index(), edge.target().index());
    }
}
```

### Parametric Optimization

```rust
use petgraph::graph::DiGraph;
use netoptim_rs::parametric::{MaxParametricSolver, ParametricAPI};
use num::rational::Ratio;
use petgraph::graph::EdgeReference;

struct MyParametricAPI;

impl ParametricAPI<(), Ratio<i32>> for MyParametricAPI {
    fn distance(&self, ratio: &Ratio<i32>, edge: &EdgeReference<Ratio<i32>>) -> Ratio<i32> {
        *edge.weight() - *ratio
    }

    fn zero_cancel(&self, cycle: &[EdgeReference<Ratio<i32>>]) -> Ratio<i32> {
        let sum_a: Ratio<i32> = cycle.iter().map(|e| *e.weight()).sum();
        let sum_b = Ratio::new(cycle.len() as i32, 1);
        sum_a / sum_b
    }
}

let digraph = DiGraph::<(), Ratio<i32>>::from_edges(&[
    (0, 1, Ratio::new(1, 1)),
    (1, 2, Ratio::new(1, 1)),
    (2, 0, Ratio::new(-3, 1)),
]);

let mut solver = MaxParametricSolver::new(&digraph, MyParametricAPI);
let mut dist = [Ratio::new(0, 1); 3];
let mut ratio = Ratio::new(0, 1);

let cycle = solver.run(&mut dist, &mut ratio);
println!("Optimal ratio: {}", ratio);
```

### Graph Utilities

```rust
use netoptim_rs::utils::*;
use petgraph::Graph;

let g = Graph::<&str, i32>::from_edges(&[
    (0, 1, 5, "A-B"),
    (1, 2, 3, "B-C"),
]);

// Check for cycles
println!("Has cycle: {}", has_cycle(&g));

// Serialize to JSON
let json = serialize_graph(&g).unwrap();
println!("{}", json);

// Export to DOT for visualization
let dot = to_dot(&g);
println!("{}", dot);
```

## 📖 API Documentation

The complete API documentation is available on [docs.rs](https://docs.rs/netoptim-rs).

### Main Modules

- **`bellman_ford`**: Bellman-Ford shortest path algorithm
- **`dijkstra`**: Dijkstra's shortest path algorithm
- **`neg_cycle`**: Negative cycle detection using Howard's algorithm
- **`parametric`**: Maximum parametric optimization
- **`utils`**: Graph utility functions
- **`error`**: Error types for the library

## 🏃 Running Examples

The repository includes several examples demonstrating various features:

```bash
# Dijkstra algorithm examples
cargo run --example dijkstra_example

# Negative cycle detection examples
cargo run --example neg_cycle_example

# Graph utilities examples
cargo run --example utils_example

# Find negative cycles
cargo run --example find_negative_cycle

# Property-based tests
cargo run --example quickcheck_tests
```

## 🧪 Running Tests

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_dijkstra_simple
```

## 📊 Running Benchmarks

The library includes benchmarks using `criterion`:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench diijkstra_sparse
```

## 🔧 Features

### Default Features

No features are enabled by default.

### Optional Features

- `std`: Use standard library (enabled by default)

## 🎯 Use Cases

- **Network Routing**: Finding optimal paths in communication networks
- **Transportation Planning**: Route optimization for logistics
- **Currency Arbitrage**: Detecting profitable currency exchange cycles
- **Circuit Design**: Finding negative cycles in circuit networks
- **Game Development**: Pathfinding and AI decision-making
- **Scientific Computing**: Network flow optimization problems

## 📈 Performance Characteristics

| Algorithm | Time Complexity | Space Complexity | Notes |
|-----------|-----------------|------------------|-------|
| Bellman-Ford | O(VE) | O(V) | Handles negative weights |
| Dijkstra | O(E + V log V) | O(V) | Requires non-negative weights |
| Howard's Algorithm | O(V³) worst case | O(V) | Fast in practice for negative cycles |
| Parametric Solver | O(k · V³) | O(V) | k = number of iterations |

Where V = number of vertices, E = number of edges

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/luk036/netoptim-rs.git
cd netoptim-rs

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt

# Run benchmarks
cargo bench
```

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- Built on top of the excellent [petgraph](https://github.com/petgraph/petgraph) library
- Inspired by NetworkX and other graph algorithm libraries
- Uses [num](https://github.com/rust-num/num) crate for numerical operations

## 📚 References

- Cormen, T. H., et al. "Introduction to Algorithms" (3rd ed.)
- Ahuja, R. K., Magnanti, T. L., & Orlin, J. B. "Network Flows"
- Karp, R. M. "A characterization of the minimum cycle mean in a digraph"

## 🗺️ Roadmap

- [ ] Additional shortest path algorithms (Floyd-Warshall, A*)
- [ ] Maximum flow algorithms (Edmonds-Karp, Push-Relabel)
- [ ] Minimum spanning tree algorithms (Kruskal, Prim)
- [ ] Parallel algorithm implementations
- [ ] More comprehensive examples and tutorials
- [ ] Performance optimizations for large graphs

## 💬 Support

- 📖 [Documentation](https://docs.rs/netoptim-rs)
- 🐛 [Issue Tracker](https://github.com/luk036/netoptim-rs/issues)
- 💬 [Discussions](https://github.com/luk036/netoptim-rs/discussions)

---

Made with ❤️ in Rust