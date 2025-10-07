'''
#[cfg(test)]
mod tests {
    use crate::neg_cycle::NegCycleFinder;
    use crate::parametric::{MaxParametricSolver, ParametricAPI};
    use crate::{bellman_ford, find_negative_cycle};
    use num::rational::Ratio;
    use petgraph::graph::{DiGraph, EdgeReference, NodeIndex};
    use petgraph::Graph;

    #[test]
    fn test_neg_cycle_multiple_neg_cycles() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 0, Ratio::new(-2, 1)), // Cycle 1: 0 -> 1 -> 0 (weight -1)
            (2, 3, Ratio::new(1, 1)),
            (3, 2, Ratio::new(-3, 1)), // Cycle 2: 2 -> 3 -> 2 (weight -2)
        ]);

        let mut ncf = NegCycleFinder::new(&digraph);
        let mut dist = [
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
        ];
        let result = ncf.howard(&mut dist, |e| *e.weight());
        assert!(result.is_some());
        let cycle = result.unwrap();
        let cycle_weight: Ratio<i32> = cycle.iter().map(|e| *e.weight()).sum();
        assert!(cycle_weight < Ratio::new(0, 1));
    }

    #[test]
    fn test_neg_cycle_not_reachable() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (2, 3, Ratio::new(-1, 1)),
            (3, 2, Ratio::new(-1, 1)),
        ]);

        let mut ncf = NegCycleFinder::new(&digraph);
        let mut dist = [
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
        ];
        let result = ncf.howard(&mut dist, |e| *e.weight());
        assert!(result.is_some());
    }

    struct TestParametricAPI;

    impl ParametricAPI<(), Ratio<i32>> for TestParametricAPI {
        fn distance(&self, ratio: &Ratio<i32>, edge: &EdgeReference<Ratio<i32>>) -> Ratio<i32> {
            *edge.weight() - *ratio
        }

        fn zero_cancel(&self, cycle: &[EdgeReference<Ratio<i32>>]) -> Ratio<i32> {
            let mut sum_a = Ratio::new(0, 1);
            let mut sum_b = Ratio::new(0, 1);
            for edge in cycle {
                sum_a += *edge.weight();
                sum_b += Ratio::new(1, 1);
            }
            sum_a / sum_b
        }
    }

    #[test]
    fn test_max_parametric_solver_no_neg_cycle() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 2, Ratio::new(1, 1)),
            (2, 0, Ratio::new(1, 1)),
        ]);

        let mut solver = MaxParametricSolver::new(&digraph, TestParametricAPI);
        let mut dist = [Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
        let mut ratio = Ratio::new(0, 1);

        let cycle = solver.run(&mut dist, &mut ratio);
        assert!(cycle.is_empty());
    }

    #[test]
    fn test_max_parametric_solver_multiple_neg_cycles() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 0, Ratio::new(-2, 1)), // Cycle 1: ratio -1/1
            (2, 3, Ratio::new(1, 1)),
            (3, 2, Ratio::new(-4, 1)), // Cycle 2: ratio -3/1
        ]);

        let mut solver = MaxParametricSolver::new(&digraph, TestParametricAPI);
        let mut dist = [
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
        ];
        let mut ratio = Ratio::new(0, 1);

        let cycle = solver.run(&mut dist, &mut ratio);
        assert!(!cycle.is_empty());
        assert_eq!(ratio, Ratio::new(-3, 2));
    }

    #[test]
    fn test_bellman_ford_neg_cycle() {
        let graph_with_neg_cycle =
            Graph::<(), f32, Directed>::from_edges(&[(0, 1, 1.0), (1, 2, 1.0), (2, 0, -3.0)]);
        let result = bellman_ford(&graph_with_neg_cycle, NodeIndex::new(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_bellman_ford_no_edge() {
        let mut graph = Graph::<(), f32, Directed>::new();
        let n0 = graph.add_node(());
        let result = bellman_ford(&graph, n0);
        assert!(result.is_ok());
        let paths = result.unwrap();
        assert_eq!(paths.distances, vec![0.0]);
        assert_eq!(paths.predecessors, vec![None]);
    }

    #[test]
    fn test_bellman_ford_disconnected() {
        let mut graph = Graph::<(), f32, Directed>::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        graph.add_edge(n0, n1, 1.0);

        let result = bellman_ford(&graph, n0);
        assert!(result.is_ok());
        let paths = result.unwrap();
        // Node 2 is unreachable, so its distance should be infinite
        assert_eq!(paths.distances.len(), 3);
        assert_eq!(paths.distances[n0.index()], 0.0);
        assert_eq!(paths.distances[n1.index()], 1.0);
        assert!(paths.distances[n2.index()].is_infinite());
        assert_eq!(paths.predecessors, vec![None, Some(n0), None]);
    }

    #[test]
    fn test_find_negative_cycle_multiple() {
        let graph_with_neg_cycle = Graph::<(), f32, Directed>::from_edges(&[
            (0, 1, 1.0),
            (1, 0, -2.0),
            (2, 3, 1.0),
            (3, 2, -3.0),
        ]);
        let result = find_negative_cycle(&graph_with_neg_cycle, NodeIndex::new(0));
        assert!(result.is_some());
    }

    #[test]
    fn test_find_negative_cycle_no_neg_cycle() {
        let graph =
            Graph::<(), f32, Directed>::from_edges(&[(0, 1, 1.0), (1, 2, 1.0), (2, 3, 1.0)]);
        let result = find_negative_cycle(&graph, NodeIndex::new(0));
        assert!(result.is_none());
    }

    #[test]
    fn test_find_negative_cycle_unreachable_neg_cycle() {
        let graph =
            Graph::<(), f32, Directed>::from_edges(&[(0, 1, 1.0), (2, 3, -1.0), (3, 2, -1.0)]);
        let result = find_negative_cycle(&graph, NodeIndex::new(0));
        assert!(result.is_none());
    }
}
''