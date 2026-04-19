//! Integration tests for concrete graph implementations.
//!
//! These tests cover insertion semantics, duplicate protection, and neighbor
//! traversal behavior for directed and undirected graphs.

use shortest_path_finder::{
    graphs::{
        directed::{DirectedEdge, DirectedGraph},
        graph::{Graph, GraphNode},
        two_dimensional_coordinate_graph::{TwoDimensionalCoordinateGraph, TwoDimensionalEdge},
        undirected::{UndirectedEdge, UndirectedGraph},
    },
    nodes::{default_node::DefaultNode, two_dimensional_node::TwoDimensionalNode},
};

fn node(id: &str) -> DefaultNode {
    DefaultNode::new(id.to_string())
}

#[test]
fn directed_graph_rejects_edge_when_nodes_are_missing() {
    let mut graph = DirectedGraph::default();

    let err = graph
        .insert_edge(DirectedEdge::new(node("A"), node("B"), 5))
        .expect("insertion should fail when nodes do not exist");

    assert!(err.message.contains("doesn't exist"));
}

#[test]
fn directed_graph_skips_duplicate_nodes_and_prevents_duplicate_edges() {
    let mut graph = DirectedGraph::default();

    graph.insert_node(node("A"));
    graph.insert_node(node("A"));
    graph.insert_node(node("B"));

    assert_eq!(graph.nodes.len(), 2);

    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("A"), node("B"), 1))
            .is_none(),
        "first insertion should succeed"
    );

    let duplicate = graph.insert_edge(DirectedEdge::new(node("A"), node("B"), 1));
    assert!(duplicate.is_some());
    assert_eq!(graph.edges.len(), 1);
}

#[test]
fn directed_graph_neighbors_return_outgoing_edges_only() {
    let mut graph = DirectedGraph::default();

    for id in ["A", "B", "C"] {
        graph.insert_node(node(id));
    }

    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("A"), node("B"), 2))
            .is_none(),
        "edge insertion should succeed"
    );
    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("C"), node("A"), 8))
            .is_none(),
        "edge insertion should succeed"
    );

    let neighbors: Vec<(String, u16)> = graph
        .neighbors(&node("A"))
        .map(|(n, w)| (n.id.clone(), w))
        .collect();

    assert_eq!(neighbors, vec![("B".to_string(), 2)]);
}

#[test]
fn undirected_graph_treats_reversed_duplicate_edges_as_same_edge() {
    let mut graph = UndirectedGraph::default();

    for id in ["A", "B"] {
        graph.insert_node(node(id));
    }

    assert!(
        graph
            .insert_edge(UndirectedEdge::new(node("A"), node("B"), 3))
            .is_none(),
        "first insertion should succeed"
    );

    let duplicate = graph.insert_edge(UndirectedEdge::new(node("B"), node("A"), 3));

    assert!(duplicate.is_some());
    assert_eq!(graph.edges.len(), 1);
}

#[test]
fn undirected_graph_neighbors_include_both_directions() {
    let mut graph = UndirectedGraph::default();

    for id in ["A", "B", "C"] {
        graph.insert_node(node(id));
    }

    assert!(
        graph
            .insert_edge(UndirectedEdge::new(node("A"), node("B"), 4))
            .is_none(),
        "edge insertion should succeed"
    );
    assert!(
        graph
            .insert_edge(UndirectedEdge::new(node("C"), node("A"), 6))
            .is_none(),
        "edge insertion should succeed"
    );

    let mut neighbors: Vec<(String, u16)> = graph
        .neighbors(&node("A"))
        .map(|(n, w)| (n.id.clone(), w))
        .collect();
    neighbors.sort_by(|left, right| left.0.cmp(&right.0));

    assert_eq!(neighbors, vec![("B".to_string(), 4), ("C".to_string(), 6)]);
}

#[test]
fn two_dimensional_graph_supports_f32_coordinates_and_computes_edge_weight() {
    let a = TwoDimensionalNode::<f32>::new(0.0, 0.0, "A".to_string())
        .expect("node A construction should succeed");
    let b = TwoDimensionalNode::<f32>::new(3.0, 4.0, "B".to_string())
        .expect("node B construction should succeed");

    let mut graph = TwoDimensionalCoordinateGraph::<f32>::new(vec![a.clone(), b.clone()], vec![]);
    let edge = TwoDimensionalEdge::new(a, b);

    assert!((edge.get_weight() - 5.0).abs() < 1e-6);
    assert!(graph.insert_edge(edge).is_none());

    let start = graph
        .get_node_by_id("A")
        .expect("graph should contain node A");
    let neighbors: Vec<(String, f32)> = graph
        .neighbors(start)
        .map(|(node, weight)| (node.get_id().to_string(), weight))
        .collect();

    assert_eq!(neighbors.len(), 1);
    assert_eq!(neighbors[0].0, "B");
    assert!((neighbors[0].1 - 5.0).abs() < 1e-6);
}

#[test]
fn two_dimensional_graph_rejects_duplicate_coordinates_even_with_different_ids() {
    let mut graph = TwoDimensionalCoordinateGraph::<f32>::default();

    let first = TwoDimensionalNode::<f32>::new(7.0, 11.0, "N1".to_string())
        .expect("first node construction should succeed");
    let second = TwoDimensionalNode::<f32>::new(7.0, 11.0, "N2".to_string())
        .expect("second node construction should succeed");

    graph.insert_node(first);
    graph.insert_node(second);

    assert_eq!(graph.get_all_nodes().len(), 1);
}
