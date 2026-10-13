//! Oracle for minimum-period / maximum-yield clock skew scheduling.
//!
//! Node potentials are the clock arrival times; the iterate is `(TCP, beta)`
//! and the objective is `min TCP / beta` subject to the timing constraint
//! graph being feasible. A negative cycle, if any, supplies the separating
//! cutting plane. Mirrors the Python `ClockScheduleOracle` and the C++
//! `netoptim::ClockScheduleOracle`.

use std::cmp::Ordering;
use std::ops::Add;

use ellalgo_rs::arr::Arr;
use ellalgo_rs::cutting_plane::{OracleOptim, SingleCut};
use petgraph::graph::{DiGraph, EdgeReference};

use crate::neg_cycle::NegCycleFinder;

/// A timing-constraint edge: a scalar `value` (the relaxation distance)
/// carrying the raw `delay` and whether it is a setup (`true`) or hold
/// (`false`) edge.
#[derive(Clone, Copy, Debug)]
pub struct TimingEdge {
    /// Scalar consumed by the negative-cycle search (meaningless on stored edges).
    pub value: f64,
    /// The path delay of the edge.
    pub delay: f64,
    /// `true` for a setup edge, `false` for a hold edge.
    pub setup: bool,
}

impl TimingEdge {
    /// Build a stored timing edge.
    #[inline]
    pub fn new(delay: f64, setup: bool) -> Self {
        TimingEdge {
            value: 0.0,
            delay,
            setup,
        }
    }
}

impl PartialEq for TimingEdge {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for TimingEdge {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Add for TimingEdge {
    type Output = TimingEdge;
    #[inline]
    fn add(self, other: Self) -> Self {
        TimingEdge {
            value: self.value + other.value,
            ..self
        }
    }
}

/// Oracle for minimum-period / maximum-yield clock skew scheduling.
pub struct ClockScheduleOracle<'a, V> {
    #[allow(dead_code)]
    digraph: &'a DiGraph<V, TimingEdge>,
    ncf: NegCycleFinder<'a, V, TimingEdge>,
    dist: Vec<TimingEdge>,
}

impl<'a, V> ClockScheduleOracle<'a, V> {
    /// Construct a new clock-schedule oracle.
    pub fn new(digraph: &'a DiGraph<V, TimingEdge>, dist: Vec<TimingEdge>) -> Self {
        ClockScheduleOracle {
            digraph,
            ncf: NegCycleFinder::new(digraph),
            dist,
        }
    }
}

impl<V> OracleOptim<Arr> for ClockScheduleOracle<'_, V> {
    type CutChoice = SingleCut;

    fn assess_optim(&mut self, xc: &Arr, gamma: &mut f64) -> ((Arr, SingleCut), bool) {
        let tcp = xc[0];
        let beta = xc[1];
        if tcp < 0.0 {
            return ((Arr::from(vec![-1.0, 0.0]), SingleCut(-tcp)), false);
        }
        let fj = tcp - *gamma * beta;
        if fj > 0.0 {
            return ((Arr::from(vec![1.0, -*gamma]), SingleCut(fj)), false);
        }
        let get_weight = |e: EdgeReference<TimingEdge>| -> TimingEdge {
            let w = *e.weight();
            let value = if w.setup {
                tcp - w.delay - beta
            } else {
                w.delay - beta
            };
            TimingEdge { value, ..w }
        };
        // Howard may surface several cycles; keep the most-violated one.
        let mut best: Option<(Arr, f64)> = None;
        let mut min_weight = 0.0f64;
        for cycle in self.ncf.howard_all(&mut self.dist, get_weight) {
            let mut weight_sum = 0.0f64;
            let mut setup = 0.0f64;
            for e in &cycle {
                let w = *e.weight();
                weight_sum += if w.setup {
                    tcp - w.delay - beta
                } else {
                    w.delay - beta
                };
                if w.setup {
                    setup += 1.0;
                }
            }
            if weight_sum < min_weight {
                min_weight = weight_sum;
                best = Some((Arr::from(vec![-setup, cycle.len() as f64]), -weight_sum));
            }
        }
        match best {
            Some((g, f)) => ((g, SingleCut(f)), false),
            None => {
                *gamma = tcp / beta;
                ((Arr::from(vec![1.0, -1.0]), SingleCut(0.0)), true)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ellalgo_rs::cutting_plane::{cutting_plane_optim, Options};
    use ellalgo_rs::ell::Ell;
    use petgraph::graph::DiGraph;

    fn create_ratio_tcg() -> DiGraph<(), TimingEdge> {
        DiGraph::from_edges([
            (0, 3, TimingEdge::new(6.0, true)),
            (0, 2, TimingEdge::new(7.0, true)),
            (1, 2, TimingEdge::new(9.0, true)),
            (1, 4, TimingEdge::new(3.0, false)),
            (2, 0, TimingEdge::new(6.0, false)),
            (2, 1, TimingEdge::new(6.0, false)),
            (2, 3, TimingEdge::new(6.0, true)),
            (3, 4, TimingEdge::new(8.0, true)),
            (3, 0, TimingEdge::new(6.0, false)),
            (3, 2, TimingEdge::new(6.0, false)),
            (4, 1, TimingEdge::new(3.0, true)),
            (4, 3, TimingEdge::new(8.0, false)),
        ])
    }

    fn zero_dist() -> Vec<TimingEdge> {
        vec![TimingEdge::new(0.0, false); 5]
    }

    #[test]
    fn test_clock_schedule_oracle_feasible() {
        let gra = create_ratio_tcg();
        let mut omega = ClockScheduleOracle::new(&gra, zero_dist());
        let x = Arr::from(vec![10.0, 1.0]);
        let mut gamma = f64::INFINITY;
        let ((_g, _f), shrunk) = omega.assess_optim(&x, &mut gamma);
        assert!(shrunk);
        assert!((gamma - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_clock_schedule_oracle_minimize_ratio() {
        let gra = create_ratio_tcg();
        let mut omega = ClockScheduleOracle::new(&gra, zero_dist());
        let mut space = Ell::new_with_scalar(100.0, Arr::from(vec![7.5, 1.0]));
        let mut gamma = f64::INFINITY;
        let options = Options::new(2000, 1e-10);
        let (xbest, niter) = cutting_plane_optim(&mut omega, &mut space, &mut gamma, &options);
        assert!(xbest.is_some());
        assert!(niter <= options.max_iters);
        assert!((gamma - 3.040717067415985).abs() < 1e-6, "gamma = {gamma}");
    }
}
