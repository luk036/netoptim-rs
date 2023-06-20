use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::*;
use petgraph::visit::IntoNodeIdentifiers;

// use petgraph::visit::IntoNeighborsDirected;

pub struct NegCycleFinder<'a, V, D> {
    pub digraph: &'a DiGraph<V, D>,
    pub pred: std::collections::HashMap<NodeIndex, NodeIndex>,
}

impl<'a, V, D> NegCycleFinder<'a, V, D>
where
    D: std::ops::Add<Output = D> + std::cmp::PartialOrd + Copy,
{
    pub fn new(digraph: &'a DiGraph<V, D>) -> Self {
        Self {
            digraph,
            pred: std::collections::HashMap::new(),
        }
    }

    pub fn find_cycle(&self) -> Option<NodeIndex> {
        let mut visited = std::collections::HashMap::new();
        for vtx in self.digraph.node_identifiers() {
            if visited.contains_key(&vtx) {
                continue;
            }
            let mut utx = vtx;
            while !visited.contains_key(&utx) {
                visited.insert(utx, vtx);
                if !self.pred.contains_key(&utx) {
                    break;
                }
                utx = *self.pred.get(&utx).unwrap();
                if visited.contains_key(&utx) {
                    if visited[&utx] == vtx {
                        return Some(utx);
                    }
                    break;
                }
            }
        }
        None
    }

    pub fn relax(&mut self, dist: &mut [D]) -> bool {
        let mut changed = false;
        for utx in self.digraph.node_identifiers() {
            for edge in self.digraph.edges(utx) {
                let vtx = edge.target();
                let weight = *edge.weight();
                // for utx in self.digraph.node_indices() {
                //     for vtx in self
                //         .digraph
                //         .neighbors_directed(utx, petgraph::Direction::Outgoing)
                //     {
                // let weight = get_weight((utx, vtx));
                let distance = dist[utx.index()] + weight;
                if dist[vtx.index()] > distance {
                    dist[vtx.index()] = distance;
                    self.pred.insert(vtx, utx);
                    changed = true;
                }
            }
        }
        changed
    }

    pub fn howard(&mut self, dist: &mut [D]) -> Option<Vec<(NodeIndex, NodeIndex)>> {
        self.pred.clear();
        while self.relax(dist) {
            let v_opt = self.find_cycle();
            if let Some(vtx) = v_opt {
                return Some(self.cycle_list(vtx));
            }
        }
        None
    }

    fn cycle_list(&self, handle: NodeIndex) -> Vec<(NodeIndex, NodeIndex)> {
        let mut vtx = handle;
        let mut cycle = Vec::new();
        loop {
            let utx = self.pred[&vtx];
            cycle.push((utx, vtx));
            vtx = utx;
            if vtx == handle {
                break;
            }
        }
        cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::rational::Ratio;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_neg_cycle() {
        let digraph = DiGraph::<(), Ratio<i32>>::from_edges([
            (0, 1, Ratio::new(1, 1)),
            (0, 2, Ratio::new(1, 1)),
            (0, 3, Ratio::new(1, 1)),
            (1, 3, Ratio::new(1, 1)),
            (2, 1, Ratio::new(1, 1)),
            (3, 2, Ratio::new(-3, 1)),
        ]);

        let mut ncf = NegCycleFinder::new(&digraph);
        let mut dist = [
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
        ];
        let result = ncf.howard(&mut dist);
        assert!(result.is_some());
    }

    #[test]
    fn test_neg_cycle2() {
        let mut graph = DiGraph::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");
        let e = graph.add_node("e");
        let f = graph.add_node("f");
        let g = graph.add_node("g");
        let h = graph.add_node("h");
        let i = graph.add_node("i");
        graph.add_edge(a, b, Ratio::new(1, 1));
        graph.add_edge(a, c, Ratio::new(1, 1));
        graph.add_edge(b, d, Ratio::new(1, 1));
        graph.add_edge(c, d, Ratio::new(1, 1));
        graph.add_edge(d, e, Ratio::new(-3, 1));
        graph.add_edge(d, f, Ratio::new(1, 1));
        graph.add_edge(e, g, Ratio::new(1, 1));
        graph.add_edge(f, g, Ratio::new(1, 1));
        graph.add_edge(g, h, Ratio::new(1, 1));
        graph.add_edge(h, i, Ratio::new(1, 1));
        graph.add_edge(i, f, Ratio::new(1, 1));

        let mut ncf = NegCycleFinder::new(&graph);
        let mut dist = [
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
            Ratio::new(0, 1),
        ];
        let result = ncf.howard(&mut dist);
        assert!(result.is_none());
    }
}
