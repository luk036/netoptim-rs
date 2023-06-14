use petgraph::graph::{DiGraph, NodeIndex};
// use petgraph::visit::IntoNeighborsDirected;

pub struct NegCycleFinder<'a> {
    pub gra: &'a DiGraph<(), f64>,
    pub pred: std::collections::HashMap<NodeIndex, NodeIndex>,
}

impl<'a> NegCycleFinder<'a> {
    pub fn new(gra: &'a DiGraph<(), f64>) -> Self {
        Self {
            gra,
            pred: std::collections::HashMap::new(),
        }
    }

    pub fn find_cycle(&self) -> Option<NodeIndex> {
        let mut visited = std::collections::HashMap::new();
        for v in self.gra.node_indices() {
            if visited.contains_key(&v) {
                continue;
            }
            let mut u = v;
            while !visited.contains_key(&u) {
                visited.insert(u, v);
                u = *self.pred.get(&u).unwrap();
                if visited.contains_key(&u) {
                    if visited[&u] == v {
                        return Some(u);
                    }
                    break;
                }
            }
        }
        None
    }

    pub fn relax<F>(&mut self, dist: &mut [f64], get_weight: F) -> bool
    where
        F: Fn((NodeIndex, NodeIndex)) -> f64,
    {
        let mut changed = false;
        for u in self.gra.node_indices() {
            for v in self
                .gra
                .neighbors_directed(u, petgraph::Direction::Outgoing)
            {
                let wt = get_weight((u, v));
                let d = dist[u.index()] + wt;
                if dist[v.index()] > d {
                    dist[v.index()] = d;
                    self.pred.insert(v, u);
                    changed = true;
                }
            }
        }
        changed
    }

    pub fn find_neg_cycle<F>(
        &mut self,
        dist: &mut [f64],
        get_weight: F,
    ) -> Option<Vec<(NodeIndex, NodeIndex)>>
    where
        F: Fn((NodeIndex, NodeIndex)) -> f64,
    {
        self.pred.clear();
        while self.relax(dist, &get_weight) {
            let v_opt = self.find_cycle();
            if let Some(v) = v_opt {
                return Some(self.cycle_list(v));
            }
        }
        None
    }

    fn cycle_list(&self, handle: NodeIndex) -> Vec<(NodeIndex, NodeIndex)> {
        let mut v = handle;
        let mut cycle = Vec::new();
        loop {
            let u = self.pred[&v];
            cycle.push((u, v));
            v = u;
            if v == handle {
                break;
            }
        }
        cycle
    }
}
