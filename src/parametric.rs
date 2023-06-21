// use std::collections::HashMap;
// use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use petgraph::graph::{DiGraph, EdgeReference, NodeIndex};
// use petgraph::prelude::*;
// use petgraph::visit::EdgeRef;
// use petgraph::visit::IntoNodeIdentifiers;
// use petgraph::Direction;

// use num::traits::Float;
use num::traits::Inv;
use num::traits::One;
use num::traits::Zero;

use crate::neg_cycle::NegCycleFinder;

trait ParametricAPI<V, R>
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
{
    fn distance(&self, ratio: &R, edge: &EdgeReference<R>) -> R;
    fn zero_cancel(&self, cycle: &Vec<(NodeIndex, NodeIndex)>) -> R;
}

#[derive(Debug)]
struct MaxParametricSolver<'a, V, R, P>
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
    #[allow(dead_code)]
    pub fn new(gra: &'a DiGraph<V, R>, omega: P) -> Self {
        Self {
            ncf: NegCycleFinder::new(gra),
            omega,
        }
    }

    #[allow(dead_code)]
    pub fn run(&mut self, dist: &mut [R], ratio: &mut R) -> Vec<(NodeIndex, NodeIndex)> {
        let mut r_min = *ratio;
        let mut c_min = Vec::<(NodeIndex, NodeIndex)>::new();
        let mut cycle = Vec::<(NodeIndex, NodeIndex)>::new();
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
