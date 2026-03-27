//! Utility functions for graph operations

use petgraph::graph::{DiGraph, Graph, NodeIndex};
use petgraph::visit::{EdgeRef, IntoNodeIdentifiers};
use petgraph::Directed;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Compare two graphs for structural equality.
///
/// Returns `true` if both graphs have the same nodes and edges with the same weights.
pub fn graphs_equal<N, E, Ty>(g1: &Graph<N, E, Ty>, g2: &Graph<N, E, Ty>) -> bool
where
    N: PartialEq,
    E: PartialEq,
    Ty: petgraph::EdgeType,
{
    // Compare node count
    if g1.node_count() != g2.node_count() {
        return false;
    }

    // Compare edge count
    if g1.edge_count() != g2.edge_count() {
        return false;
    }

    // Compare node weights
    for node in g1.node_identifiers() {
        let weight1 = g1.node_weight(node);
        let weight2 = g2.node_weight(node);
        if weight1 != weight2 {
            return false;
        }
    }

    // Compare edges (for undirected graphs, this checks both directions)
    for edge in g1.edge_references() {
        let (source, target) = (edge.source(), edge.target());
        let weight = edge.weight();

        // Find corresponding edge in g2
        let mut found = false;
        for edge2 in g2.edge_references() {
            if edge2.source() == source && edge2.target() == target && edge2.weight() == weight {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}

/// Check if a graph contains any cycles.
///
/// Uses petgraph's `is_cyclic_directed` algorithm.
pub fn has_cycle<N, E>(g: &DiGraph<N, E>) -> bool
where
    N: Clone,
{
    use petgraph::algo::is_cyclic_directed;
    is_cyclic_directed(g)
}

/// Get all nodes reachable from a given source node.
///
/// Uses breadth-first search (BFS) to traverse the graph.
pub fn get_reachable_nodes<N, E>(g: &DiGraph<N, E>, source: NodeIndex) -> HashSet<NodeIndex>
where
    N: Clone,
{
    use petgraph::visit::Bfs;

    let mut reachable = HashSet::new();
    let mut bfs = Bfs::new(g, source);

    while let Some(node) = bfs.next(g) {
        reachable.insert(node);
    }

    reachable
}

/// Check if a directed graph is strongly connected.
///
/// A graph is strongly connected if there is a path from every node to every other node.
pub fn is_strongly_connected<N, E>(g: &DiGraph<N, E>) -> bool
where
    N: Clone,
{
    if g.node_count() == 0 {
        return false;
    }

    use petgraph::visit::{Dfs, VisitMap, Visitable};
    use petgraph::Direction;

    // Check if all nodes are reachable from the first node
    let start = g.node_indices().next().unwrap();
    let mut dfs = Dfs::new(g, start);
    let mut reachable = 0;
    while dfs.next(g).is_some() {
        reachable += 1;
    }

    if reachable != g.node_count() {
        return false;
    }

    // Check reverse connectivity using transpose graph
    let _dfs = Dfs::new(&g, start);
    let mut reverse_reachable = 0;

    // Manual DFS on reverse edges
    let mut visited = g.visit_map();
    let mut stack = vec![start];
    while let Some(node) = stack.pop() {
        if visited.is_visited(&node) {
            continue;
        }
        visited.visit(node);
        reverse_reachable += 1;

        for neighbor in g.neighbors_directed(node, Direction::Incoming) {
            if !visited.is_visited(&neighbor) {
                stack.push(neighbor);
            }
        }
    }

    reverse_reachable == g.node_count()
}

/// Count the number of connected components in an undirected graph.
pub fn count_connected_components<N, E>(g: &Graph<N, E>) -> usize
where
    N: Clone,
{
    use petgraph::algo::connected_components;
    connected_components(g)
}

/// Get the degree (number of edges) of each node in an undirected graph.
pub fn get_node_degrees<N, E, Ty>(g: &Graph<N, E, Ty>) -> Vec<usize>
where
    Ty: petgraph::EdgeType,
{
    g.node_indices().map(|node| g.edges(node).count()).collect()
}

/// Get the in-degree and out-degree of each node in a directed graph.
///
/// Returns a vector of tuples `(in_degree, out_degree)` for each node.
pub fn get_in_out_degrees<N, E>(g: &DiGraph<N, E>) -> Vec<(usize, usize)> {
    g.node_indices()
        .map(|node| {
            let in_degree = g
                .edges_directed(node, petgraph::Direction::Incoming)
                .count();
            let out_degree = g
                .edges_directed(node, petgraph::Direction::Outgoing)
                .count();
            (in_degree, out_degree)
        })
        .collect()
}

/// Serialize a graph to JSON format.
///
/// Returns a JSON string representation of the graph.
pub fn serialize_graph<N, E, Ty>(g: &Graph<N, E, Ty>) -> Result<String, serde_json::Error>
where
    N: Serialize + Clone,
    E: Serialize + Clone,
    Ty: petgraph::EdgeType + 'static,
{
    #[derive(Serialize)]
    #[allow(dead_code)]
    struct GraphJSON<N, E> {
        nodes: Vec<(usize, N)>,
        edges: Vec<(usize, usize, E)>,
        directed: bool,
    }

    let nodes: Vec<(usize, N)> = g
        .node_indices()
        .map(|node| (node.index(), g[node].clone()))
        .collect();

    let edges: Vec<(usize, usize, E)> = g
        .edge_indices()
        .map(|edge| {
            let (source, target) = g.edge_endpoints(edge).unwrap();
            (source.index(), target.index(), g[edge].clone())
        })
        .collect();

    let graph_json = GraphJSON {
        nodes,
        edges,
        directed: false,
    };

    serde_json::to_string_pretty(&graph_json)
}

/// Deserialize a graph from JSON format.
///
/// Takes a JSON string and returns a Graph.
pub fn deserialize_graph<N, E>(json: &str) -> Result<Graph<N, E>, serde_json::Error>
where
    N: for<'de> Deserialize<'de>,
    E: for<'de> Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct GraphJSON<N, E> {
        nodes: Vec<(usize, N)>,
        edges: Vec<(usize, usize, E)>,
        directed: bool,
    }

    let graph_json: GraphJSON<N, E> = serde_json::from_str(json)?;

    let mut graph = Graph::new();
    let mut node_indices = Vec::new();

    // Add nodes
    for (_original_index, weight) in graph_json.nodes {
        let idx = graph.add_node(weight);
        node_indices.push(idx);
    }

    // Add edges
    for (source_idx, target_idx, weight) in graph_json.edges {
        let source = node_indices[source_idx];
        let target = node_indices[target_idx];
        graph.add_edge(source, target, weight);
    }

    Ok(graph)
}

/// Generate a DOT format representation of the graph for visualization with Graphviz.
pub fn to_dot<N, E, Ty>(g: &Graph<N, E, Ty>) -> String
where
    N: std::fmt::Display,
    E: std::fmt::Display,
    Ty: petgraph::EdgeType + 'static,
{
    let is_directed = std::any::TypeId::of::<Ty>() == std::any::TypeId::of::<Directed>();
    let edge_connector = if is_directed { "->" } else { "--" };

    let mut dot = String::new();

    if is_directed {
        dot.push_str("digraph G {\n");
    } else {
        dot.push_str("graph G {\n");
    }

    // Add nodes
    for node in g.node_indices() {
        dot.push_str(&format!("    {} [label=\"{}\"];", node.index(), g[node]));
    }

    // Add edges
    for edge in g.edge_references() {
        let (source, target) = (edge.source(), edge.target());
        dot.push_str(&format!(
            "    {} {} {} [label=\"{}\"];",
            source.index(),
            edge_connector,
            target.index(),
            edge.weight()
        ));
    }

    dot.push('}');
    dot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphs_equal() {
        let g1 = Graph::<(), i32>::from_edges([(0, 1, 5), (1, 2, 3)]);
        let g2 = Graph::<(), i32>::from_edges([(0, 1, 5), (1, 2, 3)]);
        let g3 = Graph::<(), i32>::from_edges([(0, 1, 5), (1, 2, 4)]);

        assert!(graphs_equal(&g1, &g2));
        assert!(!graphs_equal(&g1, &g3));
    }

    #[test]
    fn test_has_cycle() {
        let cyclic = DiGraph::<(), i32>::from_edges([(0, 1, 1), (1, 2, 1), (2, 0, 1)]);
        let acyclic = DiGraph::<(), i32>::from_edges([(0, 1, 1), (1, 2, 1)]);

        assert!(has_cycle(&cyclic));
        assert!(!has_cycle(&acyclic));
    }

    #[test]
    fn test_get_reachable_nodes() {
        let graph = DiGraph::<(), i32>::from_edges([(0, 1, 1), (1, 2, 1), (0, 3, 1), (4, 5, 1)]);

        let reachable = get_reachable_nodes(&graph, NodeIndex::new(0));
        assert_eq!(reachable.len(), 4);
        assert!(reachable.contains(&NodeIndex::new(0)));
        assert!(reachable.contains(&NodeIndex::new(1)));
        assert!(reachable.contains(&NodeIndex::new(2)));
        assert!(reachable.contains(&NodeIndex::new(3)));
    }

    #[test]
    fn test_is_strongly_connected() {
        let strongly_connected = DiGraph::<(), i32>::from_edges([(0, 1, 1), (1, 2, 1), (2, 0, 1)]);

        let not_strongly_connected = DiGraph::<(), i32>::from_edges([(0, 1, 1), (1, 2, 1)]);

        assert!(is_strongly_connected(&strongly_connected));
        assert!(!is_strongly_connected(&not_strongly_connected));
    }

    #[test]
    fn test_count_connected_components() {
        let graph = Graph::<(), i32>::from_edges([(0, 1, 1), (1, 2, 1), (3, 4, 1)]);

        assert_eq!(count_connected_components(&graph), 2);
    }

    #[test]
    fn test_get_node_degrees() {
        use petgraph::Undirected;
        let graph = Graph::<(), i32, Undirected>::from_edges([(0, 1, 1), (1, 2, 1)]);
        let degrees = get_node_degrees(&graph);

        assert_eq!(degrees[0], 1);
        assert_eq!(degrees[1], 2);
        assert_eq!(degrees[2], 1);
    }

    #[test]
    fn test_get_in_out_degrees() {
        let graph = DiGraph::<(), i32>::from_edges([(0, 1, 1), (1, 2, 1), (2, 1, 1)]);

        let degrees = get_in_out_degrees(&graph);
        assert_eq!(degrees[0], (0, 1));
        assert_eq!(degrees[1], (2, 1));
        assert_eq!(degrees[2], (1, 1));
    }

    #[test]
    fn test_serialize_deserialize_graph() {
        let graph = Graph::<i32, f64>::from_edges([(0, 1, 1.5), (1, 2, 2.5)]);

        let json = serialize_graph(&graph).unwrap();
        let deserialized = deserialize_graph::<i32, f64>(&json).unwrap();

        assert!(graphs_equal(&graph, &deserialized));
    }

    #[test]
    fn test_to_dot() {
        let graph = DiGraph::<i32, f64>::from_edges([(0, 1, 1.5), (1, 2, 2.5)]);
        let dot = to_dot(&graph);

        assert!(dot.contains("digraph G"));
        assert!(dot.contains("->"));
    }
}
