use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::prelude::*;
use petgraph::visit::IntoNodeIdentifiers;

// use petgraph::visit::IntoNeighborsDirected;

pub struct NegCycleFinder<'a> {
    pub digraph: &'a DiGraph<(), f64>,
    pub pred: std::collections::HashMap<NodeIndex, NodeIndex>,
}

impl<'a> NegCycleFinder<'a> {
    pub fn new(digraph: &'a DiGraph<(), f64>) -> Self {
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

    pub fn relax(&mut self, dist: &mut [f64]) -> bool {
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

    pub fn find_neg_cycle(&mut self, dist: &mut [f64]) -> Option<Vec<(NodeIndex, NodeIndex)>> {
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

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_neg_cycle() {
        let digraph = DiGraph::<(), f64>::from_edges([
            (0, 1, 1.),
            (0, 2, 1.),
            (0, 3, 1.),
            (1, 3, 1.),
            (2, 1, 1.),
            (3, 2, -3.),
        ]);

        let mut ncf = NegCycleFinder::new(&digraph);
        let mut dist = [0.0, 0.0, 0.0, 0.0];
        let result = ncf.find_neg_cycle(&mut dist);
        assert!(result.is_some());
    }
}
