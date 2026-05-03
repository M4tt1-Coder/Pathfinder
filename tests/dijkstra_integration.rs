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
        directed::DirectedGraph,
        graph::{Graph, GraphNode},
        undirected::UndirectedGraph,
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

    let node_a = node("A");
    let node_b = node("B");
    let node_c = node("C");
    let node_d = node("D");
    assert!(graph.insert_edge(&node_a, &node_b, Some(1)).is_none());
    assert!(graph.insert_edge(&node_a, &node_c, Some(5)).is_none());
    assert!(graph.insert_edge(&node_b, &node_c, Some(1)).is_none());
    assert!(graph.insert_edge(&node_c, &node_d, Some(1)).is_none());

    let dijkstra = DijkstraAlgorithm::new(graph);
    let result = dijkstra.shortest_path("A", "D").expect("path should exist");

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

    let node_a = node("A");
    let node_b = node("B");
    let node_c = node("C");
    assert!(graph.insert_edge(&node_a, &node_b, Some(2)).is_none());
    assert!(graph.insert_edge(&node_b, &node_c, Some(2)).is_none());
    assert!(graph.insert_edge(&node_a, &node_c, Some(10)).is_none());

    let dijkstra = DijkstraAlgorithm::new(graph);
    let result = dijkstra.shortest_path("A", "C").expect("path should exist");

    let path_ids: Vec<&str> = result.get_path().iter().map(|n| n.get_id()).collect();

    assert_eq!(path_ids, vec!["A", "B", "C"]);
    assert_eq!(result.get_total_distance(), 4);
}

#[test]
fn dijkstra_returns_error_when_start_node_is_missing() {
    let mut graph = DirectedGraph::default();
    graph.insert_node(node("B"));
    graph.insert_node(node("C"));
    let node_b = node("B");
    let node_c = node("C");
    assert!(graph.insert_edge(&node_b, &node_c, Some(4)).is_none());

    let dijkstra = DijkstraAlgorithm::new(graph);
    let err = dijkstra
        .shortest_path("A", "C")
        .expect_err("start node is not part of the graph");

    assert!(err.message.contains("not in the graph"));
}

#[test]
fn dijkstra_returns_error_when_no_path_exists() {
    let mut graph = DirectedGraph::default();

    for id in ["A", "B", "C"] {
        graph.insert_node(node(id));
    }

    let node_a = node("A");
    let node_b = node("B");
    assert!(graph.insert_edge(&node_a, &node_b, Some(1)).is_none());

    let dijkstra = DijkstraAlgorithm::new(graph);
    let err = dijkstra
        .shortest_path("A", "C")
        .expect_err("there is no route from A to C");

    assert!(
        err.message.contains("Unable to determine a valid path")
            || err.message.contains("A path could not be found")
    );
}
