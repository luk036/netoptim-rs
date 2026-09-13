use crate::neg_cycle::NegCycleFinder;
use num::traits::ToPrimitive;
use petgraph::graph::{DiGraph, EdgeReference};

pub type Cut<X> = (X, f64);

/// Boxed per-edge weight callable produced by [`OracleFn::make_weight_fn`].
pub type WeightFn<'s, D> = Box<dyn Fn(EdgeReference<D>) -> D + 's>;

/// Newtype wrapper around `Vec<f64>` so we can implement `Neg` and `Sum`
/// (orphan rules prevent direct impls on `Vec<f64>`).
#[derive(Clone, Debug, PartialEq)]
pub struct GradVec(pub Vec<f64>);

impl std::ops::Neg for GradVec {
    type Output = GradVec;
    #[inline]
    fn neg(self) -> GradVec {
        GradVec(self.0.into_iter().map(|x| -x).collect())
    }
}

impl std::iter::Sum<GradVec> for GradVec {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = GradVec>,
    {
        let mut result = Vec::new();
        for v in iter {
            if result.is_empty() {
                result = v.0;
            } else {
                for (i, val) in v.0.iter().enumerate() {
                    result[i] += val;
                }
            }
        }
        GradVec(result)
    }
}

pub trait OracleFn<D> {
    type X: Clone;

    fn eval(&self, edge: &EdgeReference<D>, x: &Self::X) -> D;
    fn grad(&self, edge: &EdgeReference<D>, x: &Self::X) -> Self::X;
    fn update(&mut self, _gamma: &D) {}

    /// Build a weight callable with the iterate bound once.
    ///
    /// Optional fast path: override this to bind per-iterate state a single
    /// time, so the per-edge hot loop of Howard's method avoids re-deriving it.
    /// The default implementation forwards to [`OracleFn::eval`].
    fn make_weight_fn<'s>(&'s self, x: &'s Self::X) -> WeightFn<'s, D>
    where
        D: 's,
    {
        Box::new(move |edge| self.eval(&edge, x))
    }
}

pub struct NetworkOracle<'a, V, D, F>
where
    D: std::cmp::PartialOrd,
{
    #[allow(dead_code)]
    gra: &'a DiGraph<V, D>,
    potential: Vec<D>,
    ncf: NegCycleFinder<'a, V, D>,
    oracle: F,
}

impl<'a, V, D, F> NetworkOracle<'a, V, D, F>
where
    D: std::ops::Add<Output = D> + std::cmp::PartialOrd + Copy + 'a,
    F: OracleFn<D>,
{
    pub fn new(gra: &'a DiGraph<V, D>, potential: Vec<D>, oracle: F) -> Self {
        let ncf = NegCycleFinder::new(gra);
        NetworkOracle {
            gra,
            potential,
            ncf,
            oracle,
        }
    }

    #[inline]
    pub fn update(&mut self, gamma: &D) {
        self.oracle.update(gamma);
    }

    pub fn assess_feas(&mut self, x: &F::X) -> Option<Cut<F::X>>
    where
        F::X: std::ops::Neg<Output = F::X>,
        F::X: std::iter::Sum<F::X>,
        D: ToPrimitive,
    {
        let get_weight = self.oracle.make_weight_fn(x);

        if let Some(cycle) = self.ncf.howard(&mut self.potential, get_weight) {
            let f: f64 = -cycle
                .iter()
                .map(|edge| self.oracle.eval(edge, x))
                .filter_map(|d| d.to_f64())
                .sum::<f64>();
            let g: F::X = -cycle
                .iter()
                .map(|edge| self.oracle.grad(edge, x))
                .sum::<F::X>();
            return Some((g, f));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::rational::Ratio;

    struct TestOracle;

    impl OracleFn<Ratio<i32>> for TestOracle {
        type X = Ratio<i32>;

        #[inline]
        fn eval(&self, edge: &EdgeReference<Ratio<i32>>, x: &Ratio<i32>) -> Ratio<i32> {
            *edge.weight() - *x
        }

        #[inline]
        fn grad(&self, _edge: &EdgeReference<Ratio<i32>>, _x: &Ratio<i32>) -> Ratio<i32> {
            Ratio::new(-1, 1)
        }
    }

    #[test]
    fn test_network_oracle_feasible() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 2, Ratio::new(1, 1)),
        ]);
        let potential = vec![Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
        let mut oracle = NetworkOracle::new(&digraph, potential, TestOracle);
        let x = Ratio::new(0, 1);
        let result = oracle.assess_feas(&x);
        assert!(result.is_none());
    }

    #[test]
    fn test_network_oracle_infeasible() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 2, Ratio::new(1, 1)),
            (2, 0, Ratio::new(-3, 1)),
        ]);
        let potential = vec![Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
        let mut oracle = NetworkOracle::new(&digraph, potential, TestOracle);
        let x = Ratio::new(0, 1);
        let result = oracle.assess_feas(&x);
        assert!(result.is_some());
    }

    use std::cell::Cell;
    use std::rc::Rc;

    struct CountingOracle {
        calls: Rc<Cell<usize>>,
    }

    impl OracleFn<Ratio<i32>> for CountingOracle {
        type X = Ratio<i32>;

        fn eval(&self, edge: &EdgeReference<Ratio<i32>>, x: &Ratio<i32>) -> Ratio<i32> {
            *edge.weight() - *x
        }

        fn grad(&self, _edge: &EdgeReference<Ratio<i32>>, _x: &Ratio<i32>) -> Ratio<i32> {
            Ratio::new(-1, 1)
        }

        fn make_weight_fn<'s>(&'s self, x: &'s Self::X) -> WeightFn<'s, Ratio<i32>>
        where
            Ratio<i32>: 's,
        {
            self.calls.set(self.calls.get() + 1);
            Box::new(move |edge| *edge.weight() - *x)
        }
    }

    #[test]
    fn test_network_oracle_uses_make_weight_fn_hook() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (1, 2, Ratio::new(1, 1)),
            (2, 0, Ratio::new(-3, 1)),
        ]);

        let calls = Rc::new(Cell::new(0));
        let oracle = CountingOracle {
            calls: Rc::clone(&calls),
        };
        let potential = vec![Ratio::new(0, 1), Ratio::new(0, 1), Ratio::new(0, 1)];
        let mut net = NetworkOracle::new(&digraph, potential, oracle);
        let x = Ratio::new(0, 1);
        let result = net.assess_feas(&x);

        assert!(result.is_some());
        assert_eq!(calls.get(), 1);
    }
}
