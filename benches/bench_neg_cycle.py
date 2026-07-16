#!/usr/bin/env python3
"""
Python benchmark for NegCycleFinder.howard.
Same graph topology as Rust and C++ benchmarks.
"""

import time
from digraphx.neg_cycle import NegCycleFinder


def create_cycle_graph(n: int):
    """Create a cycle graph with a negative edge at the end.
    Same topology as Rust benchmark: 0->1->2->...->(n-1)->0 with one negative edge."""
    gra = {}
    for i in range(n):
        j = (i + 1) % n
        w = -3 if j == 0 else 1  # last edge back to 0 is negative
        gra[i] = {j: w}
    return gra


def bench_howard(gra, n, iterations=1000):
    """Time howard() calls."""
    # Warmup
    dist = {i: 0 for i in range(n)}
    ncf = NegCycleFinder(gra)
    for _ in ncf.howard(dist, lambda w: w):
        pass

    # Timed runs
    total = 0.0
    for _ in range(iterations):
        dist = {i: 0 for i in range(n)}
        ncf = NegCycleFinder(gra)
        start = time.perf_counter_ns()
        for _ in ncf.howard(dist, lambda w: w):
            pass
        end = time.perf_counter_ns()
        total += (end - start)

    return total / iterations


if __name__ == "__main__":
    print("=== Python NegCycleFinder Benchmark ===")
    print(f"{'Nodes':<12} {'Time (ns)':<20}")
    print("-" * 40)

    for n in [10, 50, 100, 200]:
        gra = create_cycle_graph(n)
        iters = 50000 if n <= 10 else (10000 if n <= 50 else 5000)
        avg_ns = bench_howard(gra, n, iterations=iters)
        print(f"{n:<12} {avg_ns:<20.2f}")
