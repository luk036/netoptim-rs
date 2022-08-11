// -*- coding: utf-8 -*-

/*!
Negative cycle detection for weighed graphs.
**/

use petgraph::graph::Graph;
use std::collections::HashMap;
use petgraph::graph::{NodeIndex, EdgeIndex};

/*!
 * negative cycle
 *
 * Note: Bellman-Ford's shortest-path algorithm (BF) is NOT the best way to
 *       detect negative cycles, because
 *
 *  1. BF needs a source node.
 *  2. BF detect whether there is a negative cycle at the fianl stage.
 *  3. BF restarts the solution (dist[u]) every time.
 */
pub struct NegCycleFinder {
    gr: &Graph<u32, u64>,
    pub pred: HashMap<u32, u32>,
    pub edge: HashMap<u32, u64>,
}

impl NegCycleFinder {
    /*!
     * Construct a new neg Cycle Finder object
     */
    #[inline]
    pub fn new(gr: &Graph) -> Self { 
        Self { 
            gr: gr, 
            pred: HashMap::new(), 
            edge: HashMap::new()
        }
    }

    /*!
     * find negative cycle
     */
    template <typename Container, typename WeightFn>
    pub fn find_neg_cycle(Container&& dist, WeightFn&& get_weight) -> Vec<u64> {
        self.pred.clear();
        self.edge.clear();

        while self.relax(dist, get_weight) {
            let v = self.find_cycle();
            if (v) {
                assert!(self.is_negative(*v, dist, get_weight));
                return self.cycle_list(*v);
            }
        }
        return Vec<u64>{};  // ???
    }

    /*!
     * Find a cycle on policy graph
     *
     * @return u32 a start node of the cycle
     */
    pub fn find_cycle() -> Option<u32> {
        let mut visited = HashMap::<u32, u32>::new();

        for v in self.gr.node_indices() {
            // Takes a reference and returns Option<&V>
            if visited.contains_key(&v) {
                continue;
            }
            let mut u = v;
            loop {
                visited.insert(u, v);
                if (!self.pred.contains_key(&u)) {
                    break;
                }
                u = self.pred.get(&u).unwrap();
                if (visited.contains_key(&u)) {
                    if (visited.get(&u).unwrap() == v) {
                        // if (self.is_negative(u)) {
                        // should be "yield u";
                        return Some(u);
                        // }
                    }
                    break;
                }
            }
        }

        return None;
    }

    /*!
     * Perform one relaxation
     *
     * @tparam Container
     * @tparam WeightFn
     * @param[in,out] dist
     * @param[in] get_weight
     * @return true
     * @return false
     */
    template <typename Container, typename WeightFn>
    pub fn relax(Container&& dist, WeightFn&& get_weight) -> bool {
        let mut changed = false;
        // ThreadPool pool(std::thread::hardware_concurrency());
        // Vec<std::future<void>> results;
        // Vec<std::mutex> n_mutex(self.gr.number_of_nodes());
        for e in self.gr.edge_indices() {
            let vs = self.gr.end_points(e);
            let u = &vs.first;
            let v = &vs.second;
            if u == v {
                continue;
            }  // unlikely
               // results.emplace_back(pool.enqueue([&, e]() {
            // let vs = self.gr.end_points(e);
            // let& u = vs.first;
            // let& v = vs.second;
            // let mut relax = [&]() {
            let wt = get_weight(e);
            // assume it takes a long time
            let d = dist[u] + wt;
            if dist[v] > d {
                self.pred[v] = u;
                self.edge[v] = e;  // ???
                dist[v] = d;
                changed = true;
            }
            // };

            // if (u < v) {
            //     std::lock_guard lock(n_mutex[u]);
            //     {
            //         std::lock_guard lock(n_mutex[v]);
            //         relax();
            //     }
            // } else {
            //     std::lock_guard lock(n_mutex[v]);
            //     {
            //         std::lock_guard lock(n_mutex[u]);
            //         relax();
            //     }
            // }
            // }));
        }

        // for (let mut&& result in results) result.get();

        return changed;
    }

    /*!
     * generate a cycle list
     */
    pub fn _cycle_list(handle: &u32) -> Vec<u64> {
        let mut v = handle;
        let mut cycle = Vec<u64>{};  // ???
        loop {
            let& u = self.pred[v];
            cycle.push(self.edge[v]);  // ???
            v = u;
            if v == handle {
                break;
            }
        } 
        cycle
    }

    /*!
     * check if it is really a negative cycle
     */
    template <typename Container, typename WeightFn>
    pub fn _is_negative(const u32& handle, const Container& dist, WeightFn&& get_weight) -> bool {
        let mut v = handle;
        loop {
            let u = self.pred[v];
            let e = self.edge[v];
            let wt = get_weight(e);  // ???
            if (dist[v] > dist[u] + wt) {
                return true;
            }
            v = u;
            if v == handle {
                break;
            }
        } 

        false
    }
}
