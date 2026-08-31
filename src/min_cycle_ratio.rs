use crate::parametric::{MaxParametricSolver, ParametricAPI};
use num::traits::Inv;
use petgraph::graph::{DiGraph, EdgeReference};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

struct CycleRatioAPI<V, D, F1, F2> {
    get_cost: F1,
    get_time: F2,
    _phantom: PhantomData<(V, D)>,
}

impl<V, D, F1, F2> ParametricAPI<V, D> for CycleRatioAPI<V, D, F1, F2>
where
    V: Clone,
    D: Copy + PartialOrd + Add<Output = D> + Sub<Output = D> + Mul<Output = D> + Div<Output = D>,
    F1: Fn(&EdgeReference<D>) -> D,
    F2: Fn(&EdgeReference<D>) -> D,
{
    #[inline]
    fn distance(&self, ratio: &D, edge: &EdgeReference<D>) -> D {
        (self.get_cost)(edge) - *ratio * (self.get_time)(edge)
    }

    fn zero_cancel(&self, cycle: &[EdgeReference<D>]) -> D {
        let total_cost = cycle
            .iter()
            .map(|e| (self.get_cost)(e))
            .fold(None, |acc, c| Some(acc.map_or(c, |a| a + c)));
        let total_time = cycle
            .iter()
            .map(|e| (self.get_time)(e))
            .fold(None, |acc, t| Some(acc.map_or(t, |a| a + t)));
        total_cost.expect("cycle must not be empty") / total_time.expect("cycle must not be empty")
    }
}

/// Find the minimum cost-to-time cycle ratio in a directed graph.
///
/// $$ r^* = \min_{C \in \text{cycles}(G)} \frac{\sum_{e \in C} \text{cost}(e)}{\sum_{e \in C} \text{time}(e)} $$
pub fn min_cycle_ratio<'a, V, D, F1, F2>(
    gra: &'a DiGraph<V, D>,
    r0: &mut D,
    get_cost: F1,
    get_time: F2,
    dist: &mut [D],
) -> Vec<EdgeReference<'a, D>>
where
    D: Copy
        + PartialOrd
        + Add<Output = D>
        + Sub<Output = D>
        + Mul<Output = D>
        + Div<Output = D>
        + Neg<Output = D>
        + Inv<Output = D>
        + num::traits::Zero
        + num::traits::One,
    V: Eq + std::hash::Hash + Clone,
    F1: Fn(&EdgeReference<D>) -> D,
    F2: Fn(&EdgeReference<D>) -> D,
{
    let api = CycleRatioAPI {
        get_cost,
        get_time,
        _phantom: PhantomData,
    };
    let mut solver = MaxParametricSolver::new(gra, api);
    solver.run(dist, r0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::rational::Ratio;

    #[test]
    fn test_min_cycle_ratio_simple() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(5, 1)),
            (1, 2, Ratio::new(1, 1)),
            (2, 3, Ratio::new(1, 1)),
            (3, 4, Ratio::new(1, 1)),
            (4, 0, Ratio::new(1, 1)),
        ]);
        let mut r0 = Ratio::new(5, 1);
        let mut dist = [Ratio::new(0, 1); 5];
        let get_cost = |edge: &EdgeReference<Ratio<i32>>| *edge.weight();
        let get_time = |_edge: &EdgeReference<Ratio<i32>>| Ratio::new(1, 1);

        let cycle = min_cycle_ratio(&digraph, &mut r0, get_cost, get_time, &mut dist);
        assert!(!cycle.is_empty());
        assert_eq!(cycle.len(), 5);
    }
}
