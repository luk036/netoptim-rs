//! Property-based tests using proptest for netoptim-rs
//!
//! Run with: `cargo test --example proptest_tests`

use proptest::prelude::*;
use petgraph::prelude::*;

fn test_graph() -> impl Strategy<Value = Graph<(), f64>> {
    (1..7usize).prop_map(|size| {
        let mut graph = Graph::new();
        let nodes: Vec<_> = (0..size).map(|_| graph.add_node(())).collect();
        if nodes.len() >= 2 {
            graph.add_edge(nodes[0], nodes[1], 1.0);
            if nodes.len() > 2 {
                graph.add_edge(nodes[1], nodes[2], 2.0);
            }
        }
        graph
    })
}

proptest! {
    #[test]
    fn bellman_ford_source_distance_is_zero(g in test_graph()) {
        if g.node_count() == 0 { return Ok(()); }
        let source = g.node_indices().next().unwrap();
        match bellman_ford(&g, source) {
            Ok(paths) => {
                let idx = g.to_index(source);
                assert!(paths.distances[idx] == 0.0);
            }
            Err(_) => {}
        }
    }
}

fn main() {
    println!("Run `cargo test --example proptest_tests` to execute proptest tests.");
}
