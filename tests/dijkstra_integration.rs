//! Integration tests for Dijkstra shortest path behavior.
//!
//! These cases verify successful shortest-path execution and common error
//! conditions expected in production input handling.

use shortest_path_finder::{
    algorithms::{
        algorithm::{Algorithm, SearchResult},
        dijkstra::DijkstraAlgorithm,
    },
    graphs::{
        directed::{DirectedEdge, DirectedGraph},
        graph::{Graph, GraphNode},
        undirected::{UndirectedEdge, UndirectedGraph},
    },
    nodes::default_node::DefaultNode,
};

fn node(id: &str) -> DefaultNode {
    DefaultNode::new(id.to_string())
}

#[test]
fn dijkstra_finds_shortest_path_in_directed_graph() {
    let mut graph = DirectedGraph::default();

    for id in ["A", "B", "C", "D"] {
        graph.insert_node(node(id));
    }

    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("A"), node("B"), 1))
            .is_none(),
        "edge insertion should succeed"
    );
    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("A"), node("C"), 5))
            .is_none(),
        "edge insertion should succeed"
    );
    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("B"), node("C"), 1))
            .is_none(),
        "edge insertion should succeed"
    );
    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("C"), node("D"), 1))
            .is_none(),
        "edge insertion should succeed"
    );

    let dijkstra = DijkstraAlgorithm::new(graph);
    let result = dijkstra
        .shortest_path(&node("A"), &node("D"))
        .expect("path should exist");

    let path_ids: Vec<&str> = result.get_path().iter().map(|n| n.get_id()).collect();

    assert_eq!(path_ids, vec!["A", "B", "C", "D"]);
    assert_eq!(result.get_total_distance(), 3);
}

#[test]
fn dijkstra_finds_shortest_path_in_undirected_graph() {
    let mut graph = UndirectedGraph::default();

    for id in ["A", "B", "C"] {
        graph.insert_node(node(id));
    }

    assert!(
        graph
            .insert_edge(UndirectedEdge::new(node("A"), node("B"), 2))
            .is_none(),
        "edge insertion should succeed"
    );
    assert!(
        graph
            .insert_edge(UndirectedEdge::new(node("B"), node("C"), 2))
            .is_none(),
        "edge insertion should succeed"
    );
    assert!(
        graph
            .insert_edge(UndirectedEdge::new(node("A"), node("C"), 10))
            .is_none(),
        "edge insertion should succeed"
    );

    let dijkstra = DijkstraAlgorithm::new(graph);
    let result = dijkstra
        .shortest_path(&node("A"), &node("C"))
        .expect("path should exist");

    let path_ids: Vec<&str> = result.get_path().iter().map(|n| n.get_id()).collect();

    assert_eq!(path_ids, vec!["A", "B", "C"]);
    assert_eq!(result.get_total_distance(), 4);
}

#[test]
fn dijkstra_returns_error_when_start_node_is_missing() {
    let mut graph = DirectedGraph::default();
    graph.insert_node(node("B"));
    graph.insert_node(node("C"));
    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("B"), node("C"), 4))
            .is_none(),
        "edge insertion should succeed"
    );

    let dijkstra = DijkstraAlgorithm::new(graph);
    let err = dijkstra
        .shortest_path(&node("A"), &node("C"))
        .expect_err("start node is not part of the graph");

    assert!(err.message.contains("not in the graph"));
}

#[test]
fn dijkstra_returns_error_when_no_path_exists() {
    let mut graph = DirectedGraph::default();

    for id in ["A", "B", "C"] {
        graph.insert_node(node(id));
    }

    assert!(
        graph
            .insert_edge(DirectedEdge::new(node("A"), node("B"), 1))
            .is_none(),
        "edge insertion should succeed"
    );

    let dijkstra = DijkstraAlgorithm::new(graph);
    let err = dijkstra
        .shortest_path(&node("A"), &node("C"))
        .expect_err("there is no route from A to C");

    assert!(
        err.message.contains("Unable to determine a valid path")
            || err.message.contains("A path could not be found")
    );
}
