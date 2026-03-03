//! Benchmarks for netoptim-rs

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use netoptim_rs::dijkstra::{dijkstra, dijkstra_path};
use netoptim_rs::neg_cycle::NegCycleFinder;
use num::rational::Ratio;
use petgraph::graph::{DiGraph, Graph};
use petgraph::prelude::*;

fn create_dense_graph(num_nodes: usize) -> Graph<(), f64> {
    let mut graph = Graph::new();
    let nodes: Vec<NodeIndex> = (0..num_nodes).map(|_| graph.add_node(())).collect();

    for i in 0..num_nodes {
        for j in i + 1..num_nodes {
            let weight = (j - i) as f64;
            graph.add_edge(nodes[i], nodes[j], weight);
            graph.add_edge(nodes[j], nodes[i], weight);
        }
    }

    graph
}

fn create_sparse_graph(num_nodes: usize, avg_degree: usize) -> Graph<(), f64> {
    let mut graph = Graph::new();
    let nodes: Vec<NodeIndex> = (0..num_nodes).map(|_| graph.add_node(())).collect();

    for i in 0..num_nodes {
        for j in 1..=avg_degree {
            if i + j < num_nodes {
                let weight = j as f64;
                graph.add_edge(nodes[i], nodes[i + j], weight);
            }
        }
    }

    graph
}

fn create_graph_with_negative_cycle(num_nodes: usize) -> DiGraph<(), Ratio<i32>> {
    let mut graph = DiGraph::new();
    let nodes: Vec<NodeIndex> = (0..num_nodes).map(|_| graph.add_node(())).collect();

    // Add edges to create a graph
    for i in 0..num_nodes - 1 {
        graph.add_edge(nodes[i], nodes[i + 1], Ratio::new(1, 1));
    }

    // Add a negative cycle at the end
    let n = num_nodes - 1;
    graph.add_edge(nodes[n - 1], nodes[n], Ratio::new(1, 1));
    graph.add_edge(nodes[n], nodes[n - 2], Ratio::new(-3, 1));

    graph
}

fn bench_dijkstra_sparse(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_sparse");

    for size in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let graph = create_sparse_graph(size, 3);
            let source = NodeIndex::new(0);

            b.iter(|| black_box(dijkstra(black_box(&graph), black_box(source))));
        });
    }

    group.finish();
}

fn bench_dijkstra_dense(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_dense");

    for size in [10, 20, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let graph = create_dense_graph(size);
            let source = NodeIndex::new(0);

            b.iter(|| black_box(dijkstra(black_box(&graph), black_box(source))));
        });
    }

    group.finish();
}

fn bench_dijkstra_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_path");

    for size in [50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let graph = create_sparse_graph(size, 5);
            let source = NodeIndex::new(0);
            let target = NodeIndex::new(size - 1);

            b.iter(|| {
                black_box(dijkstra_path(
                    black_box(&graph),
                    black_box(source),
                    black_box(target),
                ))
            });
        });
    }

    group.finish();
}

fn bench_neg_cycle_finder(c: &mut Criterion) {
    let mut group = c.benchmark_group("neg_cycle_finder");

    for size in [10, 50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let graph = create_graph_with_negative_cycle(size);
            let mut ncf = NegCycleFinder::new(&graph);
            let mut dist = vec![Ratio::new(0, 1); size];

            b.iter(|| black_box(ncf.howard(black_box(&mut dist), |e| *e.weight())));
        });
    }

    group.finish();
}

fn bench_graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_creation");

    for size in [100, 500, 1000, 2000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut graph = Graph::new();
                let nodes: Vec<NodeIndex> = (0..size).map(|_| graph.add_node(())).collect();

                for i in 0..size {
                    for j in i + 1..std::cmp::min(i + 5, size) {
                        let weight = (j - i) as f64;
                        graph.add_edge(nodes[i], nodes[j], weight);
                    }
                }

                black_box(graph)
            });
        });
    }

    group.finish();
}

fn bench_comparison_dijkstra_vs_bellman_ford(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithm_comparison");

    let sizes = [20, 50, 100];

    for size in sizes.iter() {
        let graph = create_sparse_graph(*size, 4);
        let source = NodeIndex::new(0);

        // Dijkstra
        group.bench_with_input(BenchmarkId::new("dijkstra", size), size, |b, _| {
            b.iter(|| black_box(dijkstra(black_box(&graph), black_box(source))));
        });

        // Bellman-Ford
        group.bench_with_input(BenchmarkId::new("bellman_ford", size), size, |b, _| {
            b.iter(|| {
                black_box(petgraph::algo::bellman_ford(
                    black_box(&graph),
                    black_box(source),
                ))
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_dijkstra_sparse,
    bench_dijkstra_dense,
    bench_dijkstra_path,
    bench_neg_cycle_finder,
    bench_graph_creation,
    bench_comparison_dijkstra_vs_bellman_ford
);

criterion_main!(benches);
