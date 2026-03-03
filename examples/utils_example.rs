//! Graph utility functions examples

use netoptim_rs::utils::*;
use petgraph::graph::{DiGraph, Graph};
use petgraph::prelude::*;

fn main() {
    println!("=== Graph Utility Functions Examples ===\n");

    // Example 1: Graph comparison
    println!("Example 1: Graph Comparison");
    println!("---------------------------");
    let mut g1 = Graph::new();
    let a1 = g1.add_node("A");
    let b1 = g1.add_node("B");
    let c1 = g1.add_node("C");
    g1.extend_with_edges([(a1, b1, 5), (b1, c1, 3)]);

    let mut g2 = Graph::new();
    let a2 = g2.add_node("A");
    let b2 = g2.add_node("B");
    let c2 = g2.add_node("C");
    g2.extend_with_edges([(a2, b2, 5), (b2, c2, 3)]);

    let mut g3 = Graph::new();
    let a3 = g3.add_node("A");
    let b3 = g3.add_node("B");
    let c3 = g3.add_node("C");
    g3.extend_with_edges([(a3, b3, 5), (b3, c3, 4)]); // Different weight

    println!("g1 == g2: {}", graphs_equal(&g1, &g2));
    println!("g1 == g3: {}", graphs_equal(&g1, &g3));

    // Example 2: Cycle detection
    println!("\n\nExample 2: Cycle Detection");
    println!("----------------------------");
    let mut cyclic = DiGraph::new();
    let a = cyclic.add_node("A");
    let b = cyclic.add_node("B");
    let c = cyclic.add_node("C");
    cyclic.extend_with_edges([(a, b, 1), (b, c, 1), (c, a, 1)]);

    let mut acyclic = DiGraph::new();
    let d = acyclic.add_node("A");
    let e = acyclic.add_node("B");
    let f = acyclic.add_node("C");
    acyclic.extend_with_edges([(d, e, 1), (e, f, 1)]);

    println!("Cyclic graph has cycle: {}", has_cycle(&cyclic));
    println!("Acyclic graph has cycle: {}", has_cycle(&acyclic));

    // Example 3: Reachability analysis
    println!("\n\nExample 3: Reachability Analysis");
    println!("----------------------------------");
    let mut graph = DiGraph::new();
    let nodes: Vec<NodeIndex> = (0..6).map(|_| graph.add_node(())).collect();
    graph.extend_with_edges([
        (nodes[0], nodes[1], 1),
        (nodes[1], nodes[2], 1),
        (nodes[0], nodes[3], 1),
        (nodes[4], nodes[5], 1),
    ]);

    let reachable_from_a = get_reachable_nodes(&graph, nodes[0]);
    println!("Nodes reachable from node 0 (A):");
    for node in &reachable_from_a {
        println!("  Node {}", node.index());
    }

    // Example 4: Strong connectivity
    println!("\n\nExample 4: Strong Connectivity");
    println!("--------------------------------");
    let mut strongly_connected = DiGraph::new();
    let a = strongly_connected.add_node("A");
    let b = strongly_connected.add_node("B");
    let c = strongly_connected.add_node("C");
    strongly_connected.extend_with_edges([(a, b, 1), (b, c, 1), (c, a, 1)]);

    let mut not_strongly_connected = DiGraph::new();
    let d = not_strongly_connected.add_node("A");
    let e = not_strongly_connected.add_node("B");
    let f = not_strongly_connected.add_node("C");
    not_strongly_connected.extend_with_edges([(d, e, 1), (e, f, 1)]);

    println!(
        "Strongly connected graph is strongly connected: {}",
        is_strongly_connected(&strongly_connected)
    );
    println!(
        "Not strongly connected graph is strongly connected: {}",
        is_strongly_connected(&not_strongly_connected)
    );

    // Example 5: Connected components
    println!("\n\nExample 5: Connected Components");
    println!("---------------------------------");
    let mut graph = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    graph.extend_with_edges([(a, b, 1), (b, c, 1), (d, e, 1)]);

    let num_components = count_connected_components(&graph);
    println!("Number of connected components: {}", num_components);

    // Example 6: Node degrees
    println!("\n\nExample 6: Node Degrees");
    println!("--------------------------");
    let mut graph = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.extend_with_edges([(a, b, 1), (b, c, 1), (a, c, 1)]);

    let degrees = get_node_degrees(&graph);
    println!("Node degrees:");
    for (i, deg) in degrees.iter().enumerate() {
        println!("  Node {}: degree = {}", i, deg);
    }

    // Example 7: In-degrees and out-degrees (directed)
    println!("\n\nExample 7: In/Out Degrees (Directed)");
    println!("--------------------------------------");
    let mut graph = DiGraph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.extend_with_edges([(a, b, 1), (b, c, 1), (c, b, 1)]);

    let degrees = get_in_out_degrees(&graph);
    println!("Node degrees (in, out):");
    for (i, (in_deg, out_deg)) in degrees.iter().enumerate() {
        println!("  Node {}: in={}, out={}", i, in_deg, out_deg);
    }

    // Example 8: Graph serialization
    println!("\n\nExample 8: Graph Serialization");
    println!("-------------------------------");
    let mut graph = Graph::new();
    let a = graph.add_node("A".to_string());
    let b = graph.add_node("B".to_string());
    let c = graph.add_node("C".to_string());
    graph.extend_with_edges([(a, b, 1.5), (b, c, 2.5)]);

    let json = serialize_graph(&graph).unwrap();
    println!("Serialized graph (JSON):");
    println!("{}", json);

    let deserialized = deserialize_graph::<String, f64>(&json).unwrap();
    println!(
        "\nDeserialization successful: {}",
        graphs_equal(&graph, &deserialized)
    );

    // Example 9: DOT format for visualization
    println!("\n\nExample 9: DOT Format (Graphviz)");
    println!("---------------------------------");
    let mut graph = DiGraph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.extend_with_edges([(a, b, 5), (b, c, 3), (c, a, 2)]);

    let dot = to_dot(&graph);
    println!("DOT format output:");
    println!("{}", dot);
    println!("\nYou can save this to a .dot file and visualize with Graphviz using:");
    println!("  dot -Tpng graph.dot -o graph.png");
}
