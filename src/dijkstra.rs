//! Dijkstra's shortest path algorithm implementation.

use petgraph::algo::FloatMeasure;
#[allow(unused_imports)]
use petgraph::graph::NodeIndex;
use petgraph::visit::{
    EdgeRef, IntoEdges, IntoNodeIdentifiers, NodeCount, NodeIndexable, VisitMap, Visitable,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Result of Dijkstra's shortest path algorithm.
///
/// Contains the distances from the source node to all other nodes,
/// and the predecessor of each node along the shortest path.
#[derive(Debug, Clone)]
pub struct DijkstraResult<NodeId, EdgeWeight> {
    pub distances: Vec<EdgeWeight>,
    pub predecessors: Vec<Option<NodeId>>,
}

/// State for the priority queue in Dijkstra's algorithm.
/// Contains a node and its current cost from the source.
#[derive(Clone, Debug)]
struct State<NodeId, Cost> {
    node: NodeId,
    cost: Cost,
}

impl<NodeId: PartialEq, Cost: PartialEq> PartialEq for State<NodeId, Cost> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.cost == other.cost
    }
}

impl<NodeId: PartialEq, Cost: PartialEq> Eq for State<NodeId, Cost> {}

impl<NodeId: PartialEq, Cost: FloatMeasure> Ord for State<NodeId, Cost> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use partial_cmp and unwrap since FloatMeasure guarantees total ordering via NaN handling
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl<NodeId: PartialEq, Cost: FloatMeasure> PartialOrd for State<NodeId, Cost> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// \[Generic\] Compute shortest paths from node `source` to all other nodes using Dijkstra's algorithm.
///
/// This implementation uses a binary heap for efficient priority queue operations.
/// The algorithm requires non-negative edge weights.
///
/// # Arguments
/// * `g` - The graph to compute shortest paths on
/// * `source` - The source node index
///
/// # Returns
/// * `Ok(DijkstraResult)` - Contains distances and predecessors for each node
/// * `Err` - If a negative edge weight is found
///
/// # Example
/// ```rust
/// use petgraph::Graph;
/// use petgraph::prelude::*;
/// use netoptim_rs::dijkstra::dijkstra;
///
/// let mut g = Graph::new();
/// let a = g.add_node(()); // node with no weight
/// let b = g.add_node(());
/// let c = g.add_node(());
/// let d = g.add_node(());
/// g.extend_with_edges(&[
///     (0, 1, 2.0),
///     (0, 3, 4.0),
///     (1, 2, 1.0),
///     (2, 4, 5.0),
///     (3, 4, 1.0),
/// ]);
///
/// let result = dijkstra(&g, a);
/// assert!(result.is_ok());
/// let paths = result.unwrap();
/// assert_eq!(paths.distances[a.index()], 0.0);
/// assert_eq!(paths.distances[b.index()], 2.0);
/// assert_eq!(paths.distances[c.index()], 3.0);
/// ```
pub fn dijkstra<G>(
    g: G,
    source: G::NodeId,
) -> Result<DijkstraResult<G::NodeId, G::EdgeWeight>, String>
where
    G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable + Visitable,
    G::EdgeWeight: FloatMeasure,
{
    let ix = |i| g.to_index(i);
    let node_count = g.node_count();

    let mut distances = vec![<_>::infinite(); node_count];
    let mut predecessors = vec![None; node_count];
    let mut visited = g.visit_map();

    distances[ix(source)] = <_>::zero();

    let mut heap = BinaryHeap::new();
    heap.push(State {
        node: source,
        cost: <_>::zero(),
    });

    while let Some(State { node, cost }) = heap.pop() {
        if visited.is_visited(&node) {
            continue;
        }

        visited.visit(node);

        for edge in g.edges(node) {
            let target = edge.target();
            let weight = *edge.weight();

            if weight < <_>::zero() {
                return Err("Dijkstra's algorithm requires non-negative edge weights".to_string());
            }

            let new_cost = cost + weight;
            if new_cost < distances[ix(target)] {
                distances[ix(target)] = new_cost;
                predecessors[ix(target)] = Some(node);
                heap.push(State {
                    node: target,
                    cost: new_cost,
                });
            }
        }
    }

    Ok(DijkstraResult {
        distances,
        predecessors,
    })
}

/// \[Generic\] Compute shortest path from `source` to `target` using Dijkstra's algorithm.
///
/// # Arguments
/// * `g` - The graph to compute shortest path on
/// * `source` - The source node index
/// * `target` - The target node index
///
/// # Returns
/// * `Some(path)` - A vector of node indices representing the shortest path from source to target
/// * `None` - If no path exists
///
/// # Example
/// ```rust
/// use petgraph::Graph;
/// use petgraph::prelude::*;
/// use netoptim_rs::dijkstra::dijkstra_path;
///
/// let mut g = Graph::new();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.extend_with_edges(&[(0, 1, 2.0), (1, 2, 3.0)]);
///
/// let path = dijkstra_path(&g, a, c);
/// assert_eq!(path, Some(vec![a, b, c]));
/// ```
pub fn dijkstra_path<G>(g: G, source: G::NodeId, target: G::NodeId) -> Option<Vec<G::NodeId>>
where
    G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable + Visitable,
    G::EdgeWeight: FloatMeasure,
    G::NodeId: PartialEq,
{
    let ix = |i| g.to_index(i);

    let result = dijkstra(g, source).ok()?;

    if result.predecessors[ix(target)].is_none() && source != target {
        return None;
    }

    let mut path = vec![target];
    let mut current = target;

    while current != source {
        let pred = result.predecessors[ix(current)]?;
        path.push(pred);
        current = pred;
    }

    path.reverse();
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    #[test]
    fn test_dijkstra_simple() {
        let mut g = Graph::new();
        let a = g.add_node(());
        let _b = g.add_node(());
        let _c = g.add_node(());
        let _d = g.add_node(());
        g.extend_with_edges([(0, 1, 2.0), (0, 3, 4.0), (1, 2, 1.0), (3, 4, 1.0)]);

        let result = dijkstra(&g, a);
        assert!(result.is_ok());
        let paths = result.unwrap();
        assert_eq!(paths.distances[0], 0.0);
        assert_eq!(paths.distances[1], 2.0);
        assert_eq!(paths.distances[2], 3.0);
        assert_eq!(paths.distances[3], 4.0);
    }

    #[test]
    fn test_dijkstra_negative_weight() {
        let g: Graph<(), f32> = Graph::<(), f32>::from_edges([(0, 1, -1.0)]);
        let result = dijkstra(&g, NodeIndex::new(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_dijkstra_path() {
        let mut g = Graph::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.extend_with_edges([(0, 1, 2.0), (1, 2, 3.0)]);

        let path = dijkstra_path(&g, a, c);
        assert_eq!(path, Some(vec![a, b, c]));
    }

    #[test]
    fn test_dijkstra_path_no_path() {
        let mut g = Graph::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, b, 1.0);

        let path = dijkstra_path(&g, a, c);
        assert_eq!(path, None);
    }

    #[test]
    fn test_dijkstra_disconnected() {
        let mut g: Graph<(), f64> = Graph::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, b, 1.0);

        let result = dijkstra(&g, a);
        assert!(result.is_ok());
        let paths = result.unwrap();
        assert_eq!(paths.distances[a.index()], 0.0);
        assert_eq!(paths.distances[b.index()], 1.0);
        assert!(paths.distances[c.index()].is_infinite());
    }

    #[test]
    fn test_dijkstra_same_node() {
        let mut g: Graph<(), f64> = Graph::new();
        let a = g.add_node(());

        let result = dijkstra(&g, a);
        assert!(result.is_ok());
        let paths = result.unwrap();
        assert_eq!(paths.distances[a.index()], 0.0);
    }

    #[test]
    fn test_dijkstra_path_same_node() {
        let mut g: Graph<(), f64> = Graph::new();
        let a = g.add_node(());

        let path = dijkstra_path(&g, a, a);
        assert_eq!(path, Some(vec![a]));
    }
}
