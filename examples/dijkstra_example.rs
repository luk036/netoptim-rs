//! Dijkstra's algorithm examples

use netoptim_rs::dijkstra::{dijkstra, dijkstra_path};
use petgraph::graph::Graph;
use petgraph::prelude::*;

fn main() {
    println!("=== Dijkstra's Algorithm Examples ===\n");

    // Example 1: Simple shortest path
    println!("Example 1: Simple Shortest Path");
    println!("---------------------------------");
    let mut g = Graph::new();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");
    let d = g.add_node("D");
    let e = g.add_node("E");

    g.extend_with_edges([
        (a, b, 4.0),
        (a, c, 2.0),
        (b, c, 1.0),
        (b, d, 5.0),
        (c, d, 8.0),
        (c, e, 10.0),
        (d, e, 2.0),
    ]);

    let result = dijkstra(&g, a).unwrap();
    println!("Distances from node A:");
    for (i, dist) in result.distances.iter().enumerate() {
        println!("  Node {}: {}", i, dist);
    }

    println!("\nShortest path from A to E:");
    if let Some(path) = dijkstra_path(&g, a, e) {
        println!(
            "  Path: {:?}",
            path.iter().map(|n| g[*n]).collect::<Vec<_>>()
        );
        println!("  Cost: {}", result.distances[e.index()]);
    }

    // Example 2: Network routing
    println!("\n\nExample 2: Network Routing");
    println!("--------------------------");
    let mut network = Graph::new();
    let routers: Vec<NodeIndex> = (0..6).map(|_| network.add_node(())).collect();

    let connections = [
        (0, 1, 2.0),
        (0, 2, 5.0),
        (1, 2, 2.0),
        (1, 3, 4.0),
        (2, 3, 1.0),
        (2, 4, 7.0),
        (3, 4, 3.0),
        (3, 5, 6.0),
        (4, 5, 1.0),
    ];

    network.extend_with_edges(connections.iter().cloned());

    let source = routers[0];
    let destination = routers[5];

    let result = dijkstra(&network, source).unwrap();
    println!("Routing from router 0 to router 5:");

    if let Some(path) = dijkstra_path(&network, source, destination) {
        println!(
            "  Optimal path: {:?}",
            path.iter().map(|n| n.index()).collect::<Vec<_>>()
        );
        println!("  Total cost: {}", result.distances[destination.index()]);

        println!("\n  Detailed route:");
        for i in 0..path.len() - 1 {
            let u = path[i];
            let v = path[i + 1];
            let edge = network.find_edge(u, v).unwrap();
            println!(
                "    Router {} -> Router {} (cost: {})",
                u.index(),
                v.index(),
                network[edge]
            );
        }
    }

    // Example 3: All-pairs shortest paths
    println!("\n\nExample 3: Distance Matrix");
    println!("---------------------------");
    let mut g = Graph::new();
    let nodes: Vec<NodeIndex> = (0..4).map(|i| g.add_node(i)).collect();

    g.extend_with_edges([
        (0, 1, 3.0),
        (0, 2, 6.0),
        (1, 2, 2.0),
        (1, 3, 7.0),
        (2, 3, 1.0),
    ]);

    println!("Distance matrix:");
    print!("    ");
    for i in 0..4 {
        print!("{:4}", i);
    }
    println!();

    for (i, node) in nodes.iter().enumerate() {
        print!("{:2}: ", i);
        let result = dijkstra(&g, *node).unwrap();
        for j in 0..4 {
            let dist: f64 = result.distances[j];
            if dist.is_infinite() {
                print!("  INF");
            } else {
                print!("{:5.1}", dist);
            }
        }
        println!();
    }

    // Example 4: Error handling - negative weights
    println!("\n\nExample 4: Error Handling");
    println!("-------------------------");
    let g_with_negative = Graph::<(), f64>::from_edges([
        (0, 1, 2.0),
        (1, 2, -1.0), // Negative weight
    ]);

    match dijkstra(&g_with_negative, NodeIndex::new(0)) {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Error (as expected): {}", e),
    }
}
