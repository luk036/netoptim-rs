//! Benchmarks for netoptim-rs

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use netoptim_rs::dijkstra::{dijkstra, dijkstra_path};
use netoptim_rs::neg_cycle::NegCycleFinder;
use num::rational::Ratio;
use petgraph::graph::{DiGraph, Graph};
use petgraph::prelude::*;

use std::collections::HashMap;

use ellalgo_rs::arr::Arr;
use ellalgo_rs::ell::Ell;
use netoptim_rs::optscaling_oracle::OptScalingOracle;
use netoptim_rs::solve::{solve_opt_scaling, Options};

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

use netoptim_rs::parametric::{MaxParametricSolver, ParametricAPI};
use petgraph::graph::EdgeReference;

struct BenchParametricAPI;

impl ParametricAPI<(), f64> for BenchParametricAPI {
    fn distance(&self, ratio: &f64, edge: &EdgeReference<f64>) -> f64 {
        *edge.weight() - *ratio
    }
    fn zero_cancel(&self, cycle: &[EdgeReference<f64>]) -> f64 {
        let sum: f64 = cycle.iter().map(|e| *e.weight()).sum();
        sum / cycle.len() as f64
    }
}

fn create_cycle_graph(num_nodes: usize) -> DiGraph<(), f64> {
    let mut graph = DiGraph::new();
    let nodes: Vec<NodeIndex> = (0..num_nodes).map(|_| graph.add_node(())).collect();
    for i in 0..num_nodes - 1 {
        graph.add_edge(nodes[i], nodes[i + 1], 1.0);
    }
    graph.add_edge(nodes[num_nodes - 1], nodes[0], -3.0); // negative cycle
    graph
}

fn bench_max_parametric(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_parametric");

    for size in [10, 50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let graph = create_cycle_graph(size);
            let mut solver = MaxParametricSolver::new(&graph, BenchParametricAPI);
            let mut dist = vec![0.0; size];
            let mut ratio = 0.0;

            b.iter(|| black_box(solver.run(black_box(&mut dist), black_box(&mut ratio))));
        });
    }

    group.finish();
}

fn create_ratio_graph(num_nodes: usize) -> DiGraph<(), f64> {
    let mut graph = DiGraph::new();
    let nodes: Vec<NodeIndex> = (0..num_nodes).map(|_| graph.add_node(())).collect();
    for i in 0..num_nodes - 1 {
        graph.add_edge(nodes[i], nodes[i + 1], (i + 1) as f64);
    }
    graph.add_edge(nodes[num_nodes - 1], nodes[0], -3.0);
    graph
}

fn bench_min_cycle_ratio(c: &mut Criterion) {
    use netoptim_rs::min_cycle_ratio::min_cycle_ratio;

    let mut group = c.benchmark_group("min_cycle_ratio");

    for size in [10, 50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let graph = create_ratio_graph(size);
            let mut dist = vec![0.0; size];
            let mut r0 = 0.0;

            b.iter(|| {
                black_box(min_cycle_ratio(
                    black_box(&graph),
                    black_box(&mut r0),
                    |e| *e.weight(),
                    |_| 1.0,
                    black_box(&mut dist),
                ))
            });
        });
    }

    group.finish();
}

fn bench_network_oracle(c: &mut Criterion) {
    use netoptim_rs::network_oracle::{NetworkOracle, OracleFn};

    struct BenchOracle;

    impl OracleFn<f64> for BenchOracle {
        type X = f64;
        fn eval(&self, edge: &EdgeReference<f64>, _x: &f64) -> f64 {
            *edge.weight()
        }
        fn grad(&self, _edge: &EdgeReference<f64>, _x: &f64) -> f64 {
            1.0
        }
    }

    let mut group = c.benchmark_group("network_oracle");

    for size in [10, 50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let graph = create_cycle_graph(size);
            let potential = vec![0.0; size];
            let mut oracle = NetworkOracle::new(&graph, potential, BenchOracle);
            let x = 0.0;

            b.iter(|| black_box(oracle.assess_feas(black_box(&x))));
        });
    }

    group.finish();
}

type CostMap = HashMap<(usize, usize), (f64, f64)>;

fn build_fixed_graph() -> (DiGraph<(), f64>, CostMap) {
    let mut gra = DiGraph::<(), f64>::new();
    for _ in 0..5 {
        gra.add_node(());
    }
    let l = |x: f64| x.ln();
    let edges: [(usize, usize, f64, f64); 17] = [
        (0, 2, l(22.0), l(125.0)),
        (0, 3, l(16.0), l(18.0)),
        (0, 4, l(15.0), l(11.0)),
        (1, 1, l(10.0), l(10.0)),
        (1, 2, l(20.0), l(19.0)),
        (1, 3, l(14.0), l(12.0)),
        (1, 4, 100.0, l(21.0)),
        (2, 0, l(125.0), l(22.0)),
        (2, 1, l(19.0), l(20.0)),
        (2, 2, l(13.0), l(13.0)),
        (3, 0, l(18.0), l(16.0)),
        (3, 1, l(12.0), l(14.0)),
        (3, 4, l(24.0), l(23.0)),
        (4, 0, l(11.0), l(15.0)),
        (4, 1, l(21.0), -100.0),
        (4, 3, l(23.0), l(24.0)),
        (4, 4, l(17.0), l(17.0)),
    ];
    let mut costs = HashMap::new();
    for (u, v, aij, aji) in edges {
        gra.add_edge(NodeIndex::new(u), NodeIndex::new(v), aij);
        costs.insert((u, v), (aij, aji));
    }
    (gra, costs)
}

fn bench_solve_opt_scaling(c: &mut Criterion) {
    let (gra, costs) = build_fixed_graph();
    let t = 125.0f64.ln() - 10.0f64.ln();
    let xinit = [125.0f64.ln(), 10.0f64.ln()];

    let run = |tol: f64| -> (bool, f64, usize) {
        let get_cost = |e: &EdgeReference<f64>| costs[&(e.source().index(), e.target().index())];
        let mut oracle = OptScalingOracle::new(&gra, vec![0.0; 5], get_cost);
        let mut space = Ell::new_with_scalar(200.0 * t, Arr::from(xinit.to_vec()));
        let mut gamma = f64::INFINITY;
        let options = Options {
            max_iters: 2000,
            tolerance: tol,
            verbose: false,
        };
        let (xb, n) = solve_opt_scaling(&mut oracle, &mut space, &mut gamma, Some(&options));
        (xb.is_some(), gamma, n)
    };

    let (_, gref, _) = run(1e-20);
    println!("solve_opt_scaling fixed graph: reference gamma = {gref:.12}");
    for tol in [1e-6f64, 1e-8, 1e-10, 1e-12, 1e-14, 1e-20] {
        let (ok, g, n) = run(tol);
        println!(
            "  tol={tol:.0e} niter={n:<4} gamma={g:.12} dgamma={:.2e} ok={ok}",
            (g - gref).abs()
        );
    }

    let mut group = c.benchmark_group("solve_opt_scaling");
    for tol in [1e-8f64, 1e-10, 1e-12, 1e-20] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{tol:.0e}")),
            &tol,
            |b, &tol| {
                b.iter(|| black_box(run(tol).2));
            },
        );
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
    bench_comparison_dijkstra_vs_bellman_ford,
    bench_max_parametric,
    bench_min_cycle_ratio,
    bench_network_oracle,
    bench_solve_opt_scaling
);

criterion_main!(benches);
