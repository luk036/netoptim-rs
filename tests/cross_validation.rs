//! Cross-language validation tests
//!
//! Verifies that the Rust implementation produces the same results
//! as the Python and C++ sibling projects for identical inputs.

use netoptim_rs::min_cycle_ratio::min_cycle_ratio;
use netoptim_rs::neg_cycle::NegCycleFinder;
use netoptim_rs::parametric::{MaxParametricSolver, ParametricAPI};
use num::rational::Ratio;
use petgraph::graph::{DiGraph, EdgeReference};

// =========================================================================
// C++ test_neg_cycle.cpp cases
// =========================================================================

#[test]
fn test_xval_neg_cycle_cpp_neg() {
    let d =
        DiGraph::<(), i32>::from_edges([(0, 1, -5), (1, 2, 1), (2, 3, 1), (3, 4, 1), (4, 0, 1)]);
    let mut n = NegCycleFinder::new(&d);
    assert!(n.howard(&mut [0; 5], |e| *e.weight()).is_some());
}

#[test]
fn test_xval_neg_cycle_cpp_no_neg() {
    let d = DiGraph::<(), i32>::from_edges([(0, 1, 2), (1, 2, 1), (2, 3, 1), (3, 4, 1), (4, 0, 1)]);
    let mut n = NegCycleFinder::new(&d);
    assert!(n.howard(&mut [0; 5], |e| *e.weight()).is_none());
}

#[test]
fn test_xval_neg_cycle_cpp_timing() {
    let d = DiGraph::<(), i32>::from_edges([
        (0, 1, 3),
        (1, 0, -4),
        (1, 2, 2),
        (2, 1, 0),
        (2, 0, -2),
        (0, 2, 1),
    ]);
    let mut n = NegCycleFinder::new(&d);
    assert!(n.howard(&mut [0; 3], |e| *e.weight()).is_some());
}

#[test]
fn test_xval_neg_cycle_cpp_raw() {
    let d = DiGraph::<(), i32>::from_edges([
        (0, 1, 7),
        (0, 2, 5),
        (1, 0, 0),
        (1, 2, 3),
        (2, 1, 1),
        (2, 0, 2),
    ]);
    let mut n = NegCycleFinder::new(&d);
    assert!(n.howard(&mut [0; 3], |e| *e.weight()).is_none());
}

// =========================================================================
// Python network_oracle tests via direct cycle verification
// =========================================================================

#[test]
fn test_xval_net_oracle_neg_cycle_found() {
    let d = DiGraph::<(), f64>::from_edges([(0, 1, 1.0), (1, 2, 1.0), (2, 0, -3.0)]);
    let mut n = NegCycleFinder::new(&d);
    let r = n.howard(&mut [0.0; 3], |e| *e.weight());
    assert!(r.is_some());
    let s: f64 = r.unwrap().iter().map(|e| *e.weight()).sum();
    assert!((s - (-1.0)).abs() < 1e-10);
}

#[test]
fn test_xval_net_oracle_no_cycle() {
    let d = DiGraph::<(), f64>::from_edges([(0, 1, 1.0), (1, 2, 1.0), (2, 0, 1.0)]);
    let mut n = NegCycleFinder::new(&d);
    assert!(n.howard(&mut [0.0; 3], |e| *e.weight()).is_none());
}

#[test]
fn test_xval_net_oracle_complex() {
    let d = DiGraph::<(), f64>::from_edges([
        (0, 1, 1.0),
        (1, 2, 1.0),
        (2, 3, 1.0),
        (3, 0, -4.0),
        (0, 2, 0.5),
    ]);
    let mut n = NegCycleFinder::new(&d);
    let r = n.howard(&mut [0.0; 4], |e| *e.weight());
    assert!(r.is_some());
    assert!(r.unwrap().iter().map(|e| *e.weight()).sum::<f64>() < 0.0);
}

// =========================================================================
// C++ test_cycle_ratio.cpp: 5-node cycle, costs [5,1,1,1,1], time=1 → ratio=9/5
// =========================================================================

#[test]
fn test_xval_min_cycle_ratio_5node() {
    let d = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(5, 1)),
        (1, 2, Ratio::new(1, 1)),
        (2, 3, Ratio::new(1, 1)),
        (3, 4, Ratio::new(1, 1)),
        (4, 0, Ratio::new(1, 1)),
    ]);
    let mut dist = [Ratio::new(0, 1); 5];
    let mut r0 = Ratio::new(5, 1);
    let c = min_cycle_ratio(
        &d,
        &mut r0,
        |e| *e.weight(),
        |_| Ratio::new(1, 1),
        &mut dist,
    );
    assert_eq!(c.len(), 5);
    assert_eq!(r0, Ratio::new(9, 5));
}

// =========================================================================
// C++ test_parametric.cpp: 5-node [2,1,1,1,1], r=0 → no neg cycle
// =========================================================================

struct CPP;

impl ParametricAPI<(), Ratio<i32>> for CPP {
    fn distance(&self, r: &Ratio<i32>, e: &EdgeReference<Ratio<i32>>) -> Ratio<i32> {
        *e.weight() - *r
    }
    fn zero_cancel(&self, c: &[EdgeReference<Ratio<i32>>]) -> Ratio<i32> {
        c.iter().map(|e| *e.weight()).sum::<Ratio<i32>>() / Ratio::new(c.len() as i32, 1)
    }
}

#[test]
fn test_xval_max_parametric_no_neg() {
    let d = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(2, 1)),
        (1, 2, Ratio::new(1, 1)),
        (2, 3, Ratio::new(1, 1)),
        (3, 4, Ratio::new(1, 1)),
        (4, 0, Ratio::new(1, 1)),
    ]);
    let mut s = MaxParametricSolver::new(&d, CPP);
    let mut dist = [Ratio::new(0, 1); 5];
    let mut r = Ratio::new(0, 1);
    assert!(s.run(&mut dist, &mut r).is_empty());
    assert_eq!(r, Ratio::new(0, 1));
}

#[test]
fn test_xval_max_parametric_disjoint() {
    let d = DiGraph::<(), Ratio<i32>>::from_edges([
        (0, 1, Ratio::new(-1, 1)),
        (1, 2, Ratio::new(-1, 1)),
        (2, 0, Ratio::new(-1, 1)),
    ]);
    let mut s = MaxParametricSolver::new(&d, CPP);
    let mut dist = [Ratio::new(0, 1); 3];
    let mut r = Ratio::new(0, 1);
    assert!(!s.run(&mut dist, &mut r).is_empty());
    assert!(r < Ratio::new(0, 1));
}
