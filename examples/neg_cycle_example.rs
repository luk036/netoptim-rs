//! Negative cycle detection examples

use netoptim_rs::neg_cycle::NegCycleFinder;
use netoptim_rs::parametric::{MaxParametricSolver, ParametricAPI};
use num::rational::Ratio;
use petgraph::graph::DiGraph;
use petgraph::graph::EdgeReference;
use petgraph::visit::EdgeRef as _;

fn main() {
    println!("=== Negative Cycle Detection Examples ===\n");

    // Example 1: Simple negative cycle
    println!("Example 1: Simple Negative Cycle");
    println!("----------------------------------");
    let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(1, 1)),
        (1, 2, Ratio::new(1, 1)),
        (2, 0, Ratio::new(-3, 1)), // Cycle: 1 + 1 - 3 = -1
    ]);

    let mut ncf = NegCycleFinder::new(&digraph);
    let mut dist = [Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];

    match ncf.howard(&mut dist, |e| *e.weight()) {
        Some(cycle) => {
            println!("Negative cycle found:");
            let cycle_weight: Ratio<i32> = cycle.iter().map(|e| *e.weight()).sum();
            println!(
                "  Nodes: {:?}",
                cycle.iter().map(|e| e.source().index()).collect::<Vec<_>>()
            );
            println!("  Cycle weight: {}", cycle_weight);
        }
        None => println!("No negative cycle found"),
    }

    // Example 2: Multiple negative cycles
    println!("\n\nExample 2: Multiple Negative Cycles");
    println!("------------------------------------");
    let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(1, 1)),
        (1, 0, Ratio::new(-2, 1)), // Cycle 1: 1 - 2 = -1
        (2, 3, Ratio::new(1, 1)),
        (3, 2, Ratio::new(-3, 1)), // Cycle 2: 1 - 3 = -2
        (0, 2, Ratio::new(1, 1)),
    ]);

    let mut ncf = NegCycleFinder::new(&digraph);
    let mut dist = [
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
    ];

    match ncf.howard(&mut dist, |e| *e.weight()) {
        Some(cycle) => {
            println!("One negative cycle found:");
            let cycle_weight: Ratio<i32> = cycle.iter().map(|e| *e.weight()).sum();
            println!(
                "  Nodes: {:?}",
                cycle.iter().map(|e| e.source().index()).collect::<Vec<_>>()
            );
            println!("  Cycle weight: {}", cycle_weight);
        }
        None => println!("No negative cycle found"),
    }

    // Example 3: No negative cycle
    println!("\n\nExample 3: No Negative Cycle");
    println!("-----------------------------");
    let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(1, 1)),
        (1, 2, Ratio::new(1, 1)),
        (2, 3, Ratio::new(1, 1)),
    ]);

    let mut ncf = NegCycleFinder::new(&digraph);
    let mut dist = [
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
        Ratio::new(0, 1),
    ];

    match ncf.howard(&mut dist, |e| *e.weight()) {
        Some(cycle) => {
            println!("Negative cycle found (unexpected!):");
            println!(
                "  Nodes: {:?}",
                cycle.iter().map(|e| e.source().index()).collect::<Vec<_>>()
            );
        }
        None => println!("No negative cycle found (as expected)"),
    }

    // Example 4: Parametric optimization
    println!("\n\nExample 4: Parametric Optimization");
    println!("------------------------------------");

    struct TestParametricAPI;

    impl ParametricAPI<(), Ratio<i32>> for TestParametricAPI {
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
    ]);

    let mut solver = MaxParametricSolver::new(&digraph, TestParametricAPI);
    let mut dist = [Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
    let mut ratio = Ratio::new(0, 1);

    let cycle = solver.run(&mut dist, &mut ratio);

    if !cycle.is_empty() {
        println!("Optimal cycle found:");
        println!("  Cycle ratio: {}", ratio);
        println!("  Cycle edges:");
        for edge in &cycle {
            println!(
                "    {} -> {} (weight: {})",
                edge.source().index(),
                edge.target().index(),
                edge.weight()
            );
        }
    } else {
        println!("No cycle found");
    }

    // Example 5: Currency arbitrage detection
    println!("\n\nExample 5: Currency Arbitrage Detection");
    println!("------------------------------------------");
    println!("Detecting arbitrage opportunities in currency exchange rates");

    // Exchange rates: USD->EUR, EUR->GBP, GBP->USD
    // If there's an arbitrage, we can profit by converting currencies in a cycle
    let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(85, 100)),  // USD -> EUR: 0.85
        (1, 2, Ratio::new(88, 100)),  // EUR -> GBP: 0.88
        (2, 0, Ratio::new(135, 100)), // GBP -> USD: 1.35
    ]);

    // Use negative logarithms to find arbitrage (product < 1 becomes sum < 0)
    let mut ncf = NegCycleFinder::new(&digraph);
    let mut dist = [Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];

    // Check for negative cycle (arbitrage opportunity)
    match ncf.howard(&mut dist, |e| {
        // Convert to negative log for cycle detection
        let w = *e.weight();
        -Ratio::new(100, 1) / w // Negative log approximation
    }) {
        Some(cycle) => {
            println!("Arbitrage opportunity detected!");
            println!(
                "  Cycle: {:?}",
                cycle.iter().map(|e| e.source().index()).collect::<Vec<_>>()
            );

            // Calculate actual profit
            let profit: Ratio<i32> = cycle.iter().map(|e| *e.weight()).product();
            println!("  Exchange rate product: {}", profit);
            if profit < Ratio::new(100, 100) {
                println!("  This indicates potential arbitrage!");
            }
        }
        None => println!("No arbitrage opportunity found"),
    }
}
