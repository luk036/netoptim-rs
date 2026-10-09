//! Convenience facades for driving the netoptim oracles with the ellipsoid
//! cutting-plane method from [`ellalgo_rs`].
//!
//! Mirrors the Python (`solve_network_feas` / `solve_opt_scaling`) and C++
//! (`solve.hpp`) siblings: both facades forward an `Options` (defaulting to
//! [`DEFAULT_TOLERANCE`]) to the ellalgo cutting-plane drivers.

use ellalgo_rs::arr::Arr;
use ellalgo_rs::cutting_plane::{
    cutting_plane_feas, cutting_plane_optim, OracleFeas, OracleOptim, SearchSpace, SingleCut,
    UpdateByCutChoice,
};
use petgraph::graph::EdgeReference;

use crate::network_oracle::{GradVec, NetworkOracle, OracleFn};
use crate::optscaling_oracle::OptScalingOracle;

pub use ellalgo_rs::cutting_plane::Options;

/// Default convergence tolerance used by the solver facades.
///
/// Matches the Python/C++ siblings' `1e-10`: the objective error scales as
/// `sqrt(tolerance)` (~1e-5), while ellalgo's built-in `1e-20` is unreachable
/// at double precision for typical scales and only inflates the iteration count.
pub const DEFAULT_TOLERANCE: f64 = 1e-10;

/// Build the library's default [`Options`] (2000 iterations, [`DEFAULT_TOLERANCE`]).
#[inline]
pub fn default_options() -> Options {
    Options {
        max_iters: 2000,
        tolerance: DEFAULT_TOLERANCE,
        verbose: false,
    }
}

impl<'a, V, F> OracleFeas<Arr> for NetworkOracle<'a, V, f64, F>
where
    F: OracleFn<f64, X = GradVec>,
{
    type CutChoice = SingleCut;

    fn assess_feas(&mut self, xc: &Arr) -> Option<(Arr, SingleCut)> {
        let x = GradVec(xc.data().to_vec());
        NetworkOracle::assess_feas(self, &x).map(|(g, f)| (Arr::from(g.0), SingleCut(f)))
    }
}

impl<'a, V, F> OracleOptim<Arr> for OptScalingOracle<'a, V, F>
where
    F: Fn(&EdgeReference<f64>) -> (f64, f64),
{
    type CutChoice = SingleCut;

    fn assess_optim(&mut self, xc: &Arr, gamma: &mut f64) -> ((Arr, SingleCut), bool) {
        let x = [xc[0], xc[1]];
        let ((g, f), shrunk) = OptScalingOracle::assess_optim(self, &x, gamma);
        ((Arr::from(g.0), SingleCut(f)), shrunk)
    }
}

/// Solve a parametric network feasibility problem.
///
/// `options` defaults to [`default_options`] when `None`.
pub fn solve_network_feas<T, O, S>(
    oracle: &mut O,
    space: &mut S,
    options: Option<&Options>,
) -> (Option<S::ArrayType>, usize)
where
    T: UpdateByCutChoice<S, ArrayType = S::ArrayType>,
    O: OracleFeas<S::ArrayType, CutChoice = T>,
    S: SearchSpace,
    S::ArrayType: Clone,
{
    let default = default_options();
    cutting_plane_feas::<T, O, S>(oracle, space, options.unwrap_or(&default))
}

/// Maximize the scaling ratio of an optimal matrix scaling problem.
///
/// `gamma` is the initial best-so-far value (use [`f64::INFINITY`] for no
/// incumbent); `options` defaults to [`default_options`] when `None`.
pub fn solve_opt_scaling<T, O, S>(
    oracle: &mut O,
    space: &mut S,
    gamma: &mut f64,
    options: Option<&Options>,
) -> (Option<S::ArrayType>, usize)
where
    T: UpdateByCutChoice<S, ArrayType = S::ArrayType>,
    O: OracleOptim<S::ArrayType, CutChoice = T>,
    S: SearchSpace,
    S::ArrayType: Clone,
{
    let default = default_options();
    cutting_plane_optim::<T, O, S>(oracle, space, gamma, options.unwrap_or(&default))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ellalgo_rs::ell::Ell;
    use petgraph::graph::DiGraph;

    #[test]
    fn test_default_options() {
        assert_eq!(DEFAULT_TOLERANCE, 1e-10);
        let opts = default_options();
        assert_eq!(opts.max_iters, 2000);
        assert_eq!(opts.tolerance, 1e-10);
    }

    struct FeasOracle;

    impl OracleFn<f64> for FeasOracle {
        type X = GradVec;

        fn eval(&self, _edge: &EdgeReference<f64>, _x: &GradVec) -> f64 {
            0.0
        }

        fn grad(&self, _edge: &EdgeReference<f64>, _x: &GradVec) -> GradVec {
            GradVec(vec![0.0, 0.0])
        }
    }

    #[test]
    fn test_solve_network_feas_feasible_immediately() {
        let gra = DiGraph::<(), f64>::from_edges([(0, 1, 1.0), (1, 2, 1.0), (2, 0, 1.0)]);
        let potential = vec![0.0, 0.0, 0.0];
        let mut oracle = NetworkOracle::new(&gra, potential, FeasOracle);
        let mut space = Ell::new_with_scalar(10.0, Arr::from(vec![0.0, 0.0]));

        let (xbest, niter) = solve_network_feas(&mut oracle, &mut space, None);

        assert!(xbest.is_some());
        assert_eq!(niter, 0);
    }

    #[test]
    fn test_solve_opt_scaling_runs() {
        let gra = DiGraph::<(), f64>::from_edges([(0, 1, 1.0), (1, 0, 1.0)]);
        let potential = vec![0.0, 0.0];
        let get_cost = |edge: &EdgeReference<f64>| (*edge.weight(), *edge.weight());
        let mut oracle = OptScalingOracle::new(&gra, potential, get_cost);
        let mut space = Ell::new_with_scalar(10.0, Arr::from(vec![2.0, 0.0]));
        let mut gamma = f64::INFINITY;

        let (xbest, niter) = solve_opt_scaling(&mut oracle, &mut space, &mut gamma, None);

        assert!(xbest.is_some());
        assert!(niter <= default_options().max_iters);
    }
}
