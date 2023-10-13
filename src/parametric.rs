// use std::collections::HashMap;
// use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use petgraph::graph::{DiGraph, EdgeReference};
// use petgraph::prelude::*;
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
/// type parameters: 'a, V, and R. 'a represents the lifetime of the struct, V represents the type of
/// the vertices in the graph, and R represents the type of the weights or
/// * `omega`: The `omega` property is of type `P`, which is a generic type parameter that implements
/// the `ParametricAPI` trait. This trait is not defined in the code snippet you provided, so it is
/// likely defined elsewhere in the codebase.
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
    /// type `V` and edges of type `R`.
    /// * `omega`: The `omega` parameter is of type `P`. It represents some value or parameter that is
    /// used in the implementation of the `new` function. The specific meaning or purpose of `omega`
    /// would depend on the context and the code that uses this function.
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
    /// or array, where `R` is the type of the elements in the matrix.
    /// * `ratio`: The `ratio` parameter is a mutable reference to a value of type `R`. It represents
    /// the current ratio value that is being used in the algorithm. The algorithm will update this
    /// value if it finds a smaller ratio during its execution.
    ///
    /// Returns:
    ///
    /// a vector of `EdgeReference<R>`.
    pub fn run(&mut self, dist: &mut [R], ratio: &mut R) -> Vec<EdgeReference<R>> {
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
            cycle = c_min.clone();
            *ratio = r_min;
        }
        cycle
    }
}
