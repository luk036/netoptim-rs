use crate::network_oracle::{Cut, GradVec, NetworkOracle, OracleFn};
use petgraph::graph::{DiGraph, EdgeReference};

pub struct OptScalingOracle<'a, V, F>
where
    F: Fn(&EdgeReference<f64>) -> (f64, f64),
{
    network: NetworkOracle<'a, V, f64, Ratio<F>>,
}

struct Ratio<F> {
    get_cost: F,
}

impl<F> Ratio<F> {
    #[inline]
    fn new(get_cost: F) -> Self {
        Ratio { get_cost }
    }
}

impl<F> OracleFn<f64> for Ratio<F>
where
    F: Fn(&EdgeReference<f64>) -> (f64, f64),
{
    type X = GradVec;

    fn eval(&self, edge: &EdgeReference<f64>, x: &GradVec) -> f64 {
        let (aij, aji) = (self.get_cost)(edge);
        f64::min(x.0[0] - aji, aij - x.0[1])
    }

    fn grad(&self, edge: &EdgeReference<f64>, x: &GradVec) -> GradVec {
        let (aij, aji) = (self.get_cost)(edge);
        if x.0[0] - aji < aij - x.0[1] {
            GradVec(vec![1.0, 0.0])
        } else {
            GradVec(vec![0.0, -1.0])
        }
    }
}

impl<'a, V, F> OptScalingOracle<'a, V, F>
where
    F: Fn(&EdgeReference<f64>) -> (f64, f64),
{
    pub fn new(gra: &'a DiGraph<V, f64>, potential: Vec<f64>, get_cost: F) -> Self {
        let ratio = Ratio::new(get_cost);
        let network = NetworkOracle::new(gra, potential, ratio);
        OptScalingOracle { network }
    }

    pub fn assess_optim(&mut self, x: &[f64; 2], gamma: &mut f64) -> (Cut<GradVec>, bool) {
        let cut = self.network.assess_feas(&GradVec(x.to_vec()));
        if let Some(c) = cut {
            return (c, false);
        }
        let s = x[0] - x[1];
        let fj = s - *gamma;
        if fj < 0.0 {
            *gamma = s;
            return ((GradVec(vec![1.0, -1.0]), 0.0), true);
        }
        ((GradVec(vec![1.0, -1.0]), fj), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    #[test]
    fn test_optscaling_oracle_improves() {
        let gra = DiGraph::<(), f64>::from_edges([(0, 1, 1.0), (1, 0, 1.0)]);
        let potential = vec![0.0, 0.0];
        let get_cost = |_edge: &EdgeReference<f64>| (1.0, 1.0);
        let mut oracle = OptScalingOracle::new(&gra, potential, get_cost);
        let x = [2.0, 0.0]; // pi=2, psi=0 → s=2, with gamma=3, fj=-1 < 0 → update
        let mut gamma = 3.0;
        let (_cut, updated) = oracle.assess_optim(&x, &mut gamma);
        assert!(updated);
        assert_eq!(gamma, 2.0);
    }

    #[test]
    fn test_optscaling_oracle_infeasible() {
        let gra = DiGraph::<(), f64>::from_edges([(0, 1, 1.0), (1, 0, 1.0)]);
        let potential = vec![0.0, 0.0];
        let get_cost = |_edge: &EdgeReference<f64>| (1.0, 1.0);
        let mut oracle = OptScalingOracle::new(&gra, potential, get_cost);
        let x = [0.0, 0.0]; // pi=0, psi=0 → infeasible (negative cycle)
        let mut gamma = 0.0;
        let (_cut, updated) = oracle.assess_optim(&x, &mut gamma);
        assert!(!updated); // infeasible → no gamma update
    }
}
