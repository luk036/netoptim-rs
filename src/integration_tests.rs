//! Integration tests for netoptim-rs

use num::rational::Ratio;
use petgraph::graph::DiGraph;
use petgraph::prelude::*;

use crate::dijkstra::{dijkstra, dijkstra_path};
use crate::neg_cycle::NegCycleFinder;
use crate::parametric::{MaxParametricSolver, ParametricAPI};
use petgraph::graph::EdgeReference;

#[test]
fn test_integration_bellman_ford_with_neg_cycle() {
    use petgraph::algo::{bellman_ford, find_negative_cycle};

    let graph = Graph::<(), f64, Directed>::from_edges([
        (0, 1, 2.0),
        (1, 2, 3.0),
        (2, 0, -6.0), // Negative cycle: 2.0 + 3.0 - 6.0 = -1.0
    ]);

    // Test bellman_ford detects negative cycle
    let result = bellman_ford(&graph, NodeIndex::new(0));
    assert!(result.is_err());

    // Test find_negative_cycle finds the cycle
    let cycle = find_negative_cycle(&graph, NodeIndex::new(0));
    assert!(cycle.is_some());
}

#[test]
fn test_integration_neg_cycle_finder_with_parametric() {
    struct TestAPI;

    impl ParametricAPI<(), Ratio<i32>> for TestAPI {
        fn distance(&self, ratio: &Ratio<i32>, edge: &EdgeReference<Ratio<i32>>) -> Ratio<i32> {
            *edge.weight() - *ratio
        }

        fn zero_cancel(&self, cycle: &[EdgeReference<Ratio<i32>>]) -> Ratio<i32> {
            let sum_a: Ratio<i32> = cycle.iter().map(|e| *e.weight()).sum();
            let sum_b = Ratio::new(cycle.len() as i32, 1);
            sum_a / sum_b
        }
    }

    let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(1, 1)),
        (1, 2, Ratio::new(1, 1)),
        (2, 0, Ratio::new(-3, 1)),
    ]);

    let mut solver = MaxParametricSolver::new(&digraph, TestAPI);
    let mut dist = [Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
    let mut ratio = Ratio::new(0, 1);

    let cycle = solver.run(&mut dist, &mut ratio);
    assert!(!cycle.is_empty());
    assert_eq!(ratio, Ratio::new(-1, 3));
}

#[test]
fn test_integration_dijkstra_neg_cycle_comparison() {
    use petgraph::algo::bellman_ford;

    let graph = Graph::<(), f64, Directed>::from_edges([
        (0, 1, 4.0),
        (0, 2, 2.0),
        (1, 2, 1.0),
        (1, 3, 5.0),
        (2, 3, 8.0),
        (2, 4, 10.0),
        (3, 4, 2.0),
    ]);

    let source = NodeIndex::new(0);

    // Compare Dijkstra and Bellman-Ford results
    let dijkstra_result = dijkstra(&graph, source).unwrap();
    let bellman_ford_result = bellman_ford(&graph, source).unwrap();

    assert_eq!(dijkstra_result.distances, bellman_ford_result.distances);
}

#[test]
fn test_integration_complete_workflow() {
    use petgraph::algo::bellman_ford;

    // Create a complex graph
    let mut graph = Graph::new();
    let nodes: Vec<NodeIndex> = (0..6).map(|_| graph.add_node(())).collect();

    let edges = [
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

    graph.extend_with_edges(edges.iter().cloned());

    // Test shortest paths from node 0
    let source = nodes[0];
    let dijkstra_result = dijkstra(&graph, source).unwrap();
    let bellman_ford_result = bellman_ford(&graph, source).unwrap();

    // Verify algorithms agree
    for (i, (d_dist, bf_dist)) in dijkstra_result
        .distances
        .iter()
        .zip(bellman_ford_result.distances.iter())
        .enumerate()
    {
        assert_eq!(*d_dist, *bf_dist, "Distance mismatch at node {}", i);
    }

    // Test specific path
    let target = nodes[5];
    let path = dijkstra_path(&graph, source, target);
    assert!(path.is_some());

    // Verify path cost matches distance
    let path = path.unwrap();
    let mut path_cost = 0.0;
    for i in 0..path.len() - 1 {
        let u = path[i];
        let v = path[i + 1];
        let edge = graph.find_edge(u, v).expect("Edge not found");
        path_cost += graph[edge];
    }

    assert_eq!(path_cost, dijkstra_result.distances[target.index()]);
}

#[test]
fn test_integration_large_graph_performance() {
    use petgraph::Graph;

    // Create a larger graph to test performance
    let mut graph = Graph::new();
    let num_nodes = 100;
    let nodes: Vec<NodeIndex> = (0..num_nodes).map(|_| graph.add_node(())).collect();

    // Add edges in a pattern that ensures connectivity
    for i in 0..num_nodes {
        for j in i + 1..std::cmp::min(i + 5, num_nodes) {
            graph.add_edge(nodes[i], nodes[j], (j - i) as f64);
        }
    }

    let source = nodes[0];
    let result = dijkstra(&graph, source);

    assert!(result.is_ok());
    let paths = result.unwrap();
    assert_eq!(paths.distances.len(), num_nodes);

    // Verify source distance is 0
    assert_eq!(paths.distances[source.index()], 0.0);

    // Verify all distances are non-negative
    for (i, &dist) in paths.distances.iter().enumerate() {
        assert!(dist >= 0.0, "Negative distance at node {}", i);
    }
}

#[test]
fn test_integration_parametric_with_neg_cycle_finder() {
    struct LinearAPI;

    impl ParametricAPI<(), Ratio<i32>> for LinearAPI {
        fn distance(&self, ratio: &Ratio<i32>, edge: &EdgeReference<Ratio<i32>>) -> Ratio<i32> {
            *edge.weight() - *ratio
        }

        fn zero_cancel(&self, cycle: &[EdgeReference<Ratio<i32>>]) -> Ratio<i32> {
            let sum_a: Ratio<i32> = cycle.iter().map(|e| *e.weight()).sum();
            let sum_b = Ratio::new(cycle.len() as i32, 1);
            sum_a / sum_b
        }
    }

    let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(2, 1)),
        (1, 2, Ratio::new(3, 1)),
        (2, 0, Ratio::new(-7, 1)),
        (0, 3, Ratio::new(1, 1)),
        (3, 4, Ratio::new(1, 1)),
        (4, 5, Ratio::new(1, 1)),
        (5, 3, Ratio::new(-4, 1)),
    ]);

    // Test neg_cycle_finder directly
    let mut ncf = NegCycleFinder::new(&digraph);
    let mut dist = [
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
    ];

    let neg_cycle = ncf.howard(&mut dist, |e| *e.weight());
    assert!(neg_cycle.is_some());

    // Test with parametric solver
    let mut solver = MaxParametricSolver::new(&digraph, LinearAPI);
    let mut dist2 = [
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
    ];
    let mut ratio = Ratio::new(0, 1);

    let cycle = solver.run(&mut dist2, &mut ratio);
    assert!(!cycle.is_empty());
}
