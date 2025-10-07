// use std::collections::HashMap;
// use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use petgraph::graph::{DiGraph, EdgeReference};

// use petgraph::visit::EdgeRef;
// use petgraph::visit::IntoNodeIdentifiers;
// use petgraph::Direction;

// use num::traits::Float;
use num::traits::Inv;
use num::traits::One;
use num::traits::Zero;

use crate::neg_cycle::NegCycleFinder;

/// The `ParametricAPI` trait defines two methods: `distance` and `zero_cancel`.
pub trait ParametricAPI<E, R>
where
    R: Copy + PartialOrd,
    E: Clone,
{
    fn distance(&self, ratio: &R, edge: &EdgeReference<R>) -> R;
    fn zero_cancel(&self, cycle: &[EdgeReference<R>]) -> R;
}

/// The `MaxParametricSolver` struct is a generic type that takes in parameters `V`, `R`, and `P` and
/// contains a `NegCycleFinder` and `omega` of type `P`.
///
/// Properties:
///
/// * `ncf`: NegCycleFinder is a struct that is used to find negative cycles in a graph. It takes three
///             type parameters: 'a, V, and R. 'a represents the lifetime of the struct, V represents the type of
///             the vertices in the graph, and R represents the type of the weights or
/// * `omega`: The `omega` property is of type `P`, which is a generic type parameter that implements
///             the `ParametricAPI` trait. This trait is not defined in the code snippet you provided, so it is
///             likely defined elsewhere in the codebase.
#[derive(Debug)]
pub struct MaxParametricSolver<'a, V, R, P>
where
    R: Copy
        + PartialOrd
        + Add<Output = R>
        + Sub<Output = R>
        + Mul<Output = R>
        + Div<Output = R>
        + Neg<Output = R>
        + Inv<Output = R>,
    V: Eq + Hash + Clone,
    P: ParametricAPI<V, R>,
{
    ncf: NegCycleFinder<'a, V, R>,
    omega: P,
}

impl<'a, V, R, P> MaxParametricSolver<'a, V, R, P>
where
    R: Copy
        + PartialOrd
        + Zero
        + One
        + Add<Output = R>
        + Sub<Output = R>
        + Mul<Output = R>
        + Div<Output = R>
        + Neg<Output = R>
        + Inv<Output = R>,
    V: Eq + Hash + Clone,
    P: ParametricAPI<V, R>,
{
    /// The function creates a new instance of a struct with a given directed graph and a value.
    ///
    /// Arguments:
    ///
    /// * `grph`: The `grph` parameter is a reference to a directed graph (`DiGraph`) with vertices of
    ///   type `V` and edges of type `R`.
    /// * `omega`: The `omega` parameter is of type `P`. It represents some value or parameter that is
    ///   used in the implementation of the `new` function. The specific meaning or purpose of `omega`
    ///   would depend on the context and the code that uses this function.
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the struct that it is defined in.
    pub fn new(grph: &'a DiGraph<V, R>, omega: P) -> Self {
        Self {
            ncf: NegCycleFinder::new(grph),
            omega,
        }
    }

    /// The function `run` finds the minimum ratio and corresponding cycle in a given graph.
    ///
    /// Arguments:
    ///
    /// * `dist`: `dist` is a mutable reference to a slice of type `R`. It represents a distance matrix
    ///   or array, where `R` is the type of the elements in the matrix.
    /// * `ratio`: The `ratio` parameter is a mutable reference to a value of type `R`. It represents
    ///   the current ratio value that is being used in the algorithm. The algorithm will update this
    ///   value if it finds a smaller ratio during its execution.
    ///
    /// Returns:
    ///
    /// a vector of `EdgeReference<R>`.
    /// # Example
    /// ```rust
    /// use petgraph::graph::DiGraph;
    /// use petgraph::prelude::*;
    /// use num::rational::Ratio;
    /// use netoptim_rs::parametric::{MaxParametricSolver, ParametricAPI};
    /// use petgraph::graph::EdgeReference;
    ///
    /// struct TestParametricAPI;
    ///
    /// impl ParametricAPI<(), Ratio<i32>> for TestParametricAPI {
    ///     fn distance(&self, ratio: &Ratio<i32>, edge: &EdgeReference<Ratio<i32>>) -> Ratio<i32> {
    ///         *edge.weight() - *ratio
    ///     }
    ///
    ///     fn zero_cancel(&self, cycle: &[EdgeReference<Ratio<i32>>]) -> Ratio<i32> {
    ///         let mut sum_a = Ratio::new(0, 1);
    ///         let mut sum_b = Ratio::new(0, 1);
    ///         for edge in cycle {
    ///             sum_a += *edge.weight();
    ///             sum_b += Ratio::new(1, 1);
    ///         }
    ///         sum_a / sum_b
    ///     }
    /// }
    ///
    /// let digraph = DiGraph::<(), Ratio<i32>>::from_edges(&[
    ///     (0, 1, Ratio::new(1, 1)),
    ///     (1, 2, Ratio::new(1, 1)),
    ///     (2, 0, Ratio::new(-3, 1)),
    /// ]);
    ///
    /// let mut solver = MaxParametricSolver::new(&digraph, TestParametricAPI);
    /// let mut dist = [Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
    /// let mut ratio = Ratio::new(0, 1);
    ///
    /// let cycle = solver.run(&mut dist, &mut ratio);
    /// assert!(!cycle.is_empty());
    /// assert_eq!(ratio, Ratio::new(-1, 3));
    /// ```
    pub fn run(&mut self, dist: &mut [R], ratio: &mut R) -> Vec<EdgeReference<'a, R>> {
        let mut r_min = *ratio;
        let mut c_min = Vec::<EdgeReference<R>>::new();
        let mut cycle = Vec::<EdgeReference<R>>::new();
        loop {
            if let Some(ci) = self.ncf.howard(dist, |e| self.omega.distance(ratio, &e)) {
                let ri = self.omega.zero_cancel(&ci);
                if r_min > ri {
                    r_min = ri;
                    c_min = ci;
                }
            }
            if r_min >= *ratio {
                break;
            }
            cycle.clone_from(&c_min);
            *ratio = r_min;
        }
        cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    use num::rational::Ratio;

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
    fn test_max_parametric_solver_simple_cycle() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 2, Ratio::new(1, 1)),
            (2, 0, Ratio::new(-3, 1)),
        ]);

        let mut solver = MaxParametricSolver::new(&digraph, TestParametricAPI);
        let mut dist = [Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
        let mut ratio = Ratio::new(0, 1);

        let cycle = solver.run(&mut dist, &mut ratio);
        assert!(!cycle.is_empty());
        assert_eq!(ratio, Ratio::new(-1, 3));
    }

    #[test]
    fn test_max_parametric_solver_no_cycle() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 2, Ratio::new(1, 1)),
            (0, 2, Ratio::new(3, 1)),
        ]);

        let mut solver = MaxParametricSolver::new(&digraph, TestParametricAPI);
        let mut dist = [Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
        let mut ratio = Ratio::new(0, 1);

        let cycle = solver.run(&mut dist, &mut ratio);
        assert!(cycle.is_empty());
        assert_eq!(ratio, Ratio::new(0, 1)); // Should remain initial ratio
    }

    #[test]
    fn test_max_parametric_solver_multiple_cycles() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 0, Ratio::new(-2, 1)), // Cycle 1: ratio -1/1
            (2, 3, Ratio::new(1, 1)),
            (3, 2, Ratio::new(-4, 1)), // Cycle 2: ratio -3/1
            (0, 2, Ratio::new(1, 1)),
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
        assert_eq!(ratio, Ratio::new(-3, 2)); // Should find the cycle with ratio -3/1
    }
}
