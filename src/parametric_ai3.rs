use std::collections::HashMap;
// use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::*;
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoNodeIdentifiers;
// use petgraph::Direction;

use num::traits::Float;
use num::traits::Inv;
use num::traits::One;
use num::traits::Zero;

use crate::neg_cycle_ai::NegCycleFinder;

type Cycle<V> = Vec<(V, V)>;

// struct NegCycleFinder<V, R, P>
// where
//     R: Float
//         + PartialOrd
//         + Zero
//         + One
//         + Add<Output = R>
//         + Sub<Output = R>
//         + Mul<Output = R>
//         + Div<Output = R>
//         + Neg<Output = R>
//         + Inv<Output = R>,
//     V: Eq + Hash + Clone,
//     P: ParametricAPI<V, R>,
// {
//     gra: DiGraph<V, R>,
//     omega: P,
// }
//
// impl<V, R, P> NegCycleFinder<V, R, P>
// where
//     R: Float
//         + PartialOrd
//         + Zero
//         + One
//         + Add<Output = R>
//         + Sub<Output = R>
//         + Mul<Output = R>
//         + Div<Output = R>
//         + Neg<Output = R>
//         + Inv<Output = R>,
//     V: Eq + Hash + Clone,
//     P: ParametricAPI<V, R>,
// {
//     fn new(gra: DiGraph<V, R>, omega: P) -> Self {
//         Self { gra, omega }
//     }
//
//     fn find_neg_cycle<F>(&self, dist: &mut HashMap<V, R>, mut cost: F) -> Option<Cycle<V>>
//     where
//         F: FnMut(&petgraph::graph::EdgeReference<'_, R, ()>) -> R,
//     {
//         let mut prev = HashMap::new();
//         let mut updated = None;
//         let mut n = self.gra.node_count();
//         while n > 0 {
//             updated = None;
//             for e in self.gra.edge_references() {
//                 let u = e.source();
//                 let v = e.target();
//                 let w = cost(&e);
//                 if dist.contains_key(u) && dist.get(v).map_or(true, |&d| dist[u] + w < d) {
//                     dist.insert(v.clone(), dist[u] + w);
//                     prev.insert(v.clone(), u.clone());
//                     updated = Some(v.clone());
//                 }
//             }
//             n -= 1;
//             if updated.is_none() {
//                 break;
//             }
//         }
//         if let Some(u) = updated {
//             let mut cycle = vec![];
//             let mut v = u.clone();
//             loop {
//                 let u = prev[&v].clone();
//                 cycle.push((u, v));
//                 if u == v {
//                     cycle.reverse();
//                     return Some(cycle);
//                 }
//                 v = u;
//             }
//         } else {
//             None
//         }
//     }
// }

trait ParametricAPI<V, R>
where
    R: Float
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
{
    fn distance(&self, ratio: R, edge: &(NodeIndex, NodeIndex)) -> R;
    fn zero_cancel(&self, cycle: &Cycle<NodeIndex>) -> R;
}

struct MaxParametricSolver<'a, V, R, P>
where
    R: Float
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
    ncf: NegCycleFinder<'a, V, R>,
    omega: P,
}

impl<'a, V, R, P> MaxParametricSolver<'a, V, R, P>
where
    R: Float
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
    fn new(gra: &'a DiGraph<V, R>, omega: P) -> Self {
        Self {
            ncf: NegCycleFinder::new(&gra),
            omega,
        }
    }

    fn run(&mut self, dist: &mut [R], ratio: R) -> (R, Cycle<NodeIndex>) {
        let mut r_min = ratio;
        let mut c_min = vec![];
        let mut cycle = vec![];
        loop {
            for ci in self.ncf.howard(dist, |e| self.omega.distance(ratio, &e)) {
                let ri = self.omega.zero_cancel(&ci);
                if r_min > ri {
                    r_min = ri;
                    c_min = ci;
                }
            }
            if r_min >= ratio {
                break;
            }
            cycle = c_min.clone();
            ratio = r_min;
        }
        (ratio, cycle)
    }
}
