//! Integration tests for Dijkstra shortest path behavior.
//!
//! # Overview
//!
//! These tests exercise the end-to-end Dijkstra flow against concrete graph
//! implementations.
//!
//! # Scenario Matrix
//!
//! - Directed and undirected graphs.
//! - Successful shortest-path computation.
//! - Missing start node validation.
//! - No-path scenarios.
//! - Trivial path when start equals end.
//! - Distance overflow handling.
//! - Stable error-kind mapping.
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
//! use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
//! use shortest_path_finder::graphs::directed::DirectedGraph;
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let mut graph = DirectedGraph::default();
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! graph.insert_node(a.clone());
//! graph.insert_node(b.clone());
//! graph.insert_edge(&a, &b, Some(2));
//!
//! let dijkstra = DijkstraAlgorithm::new(graph);
//! let result = dijkstra.shortest_path("A", "B").unwrap();
//! assert_eq!(result.get_total_distance(), 2);
//! ```

use shortest_path_finder::{
    algorithms::{
        algorithm::{Algorithm, SearchResult},
        dijkstra::{DijkstraAlgorithm, DijkstraError},
    },
    error::algorithm_error::{
        AlgorithmErrorKind, DijkstraPathReconstructionError, EdgeWeightViolation,
        MissingNodeContext,
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

    assert!(matches!(
        err,
        DijkstraError::MissingStartNode { id, .. } if id == "A"
    ));
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

    assert!(matches!(
        err,
        DijkstraError::NoPathFound { start, end } if start == "A" && end == "C"
    ));
}

#[test]
fn dijkstra_returns_trivial_path_when_start_equals_end() {
    let mut graph = DirectedGraph::default();

    for id in ["A", "B"] {
        graph.insert_node(node(id));
    }

    let node_a = node("A");
    let node_b = node("B");
    assert!(graph.insert_edge(&node_a, &node_b, Some(2)).is_none());

    let dijkstra = DijkstraAlgorithm::new(graph);
    let result = dijkstra
        .shortest_path("A", "A")
        .expect("start equals end should return a trivial path");

    assert_eq!(result.get_total_distance(), 0);
    assert_eq!(result.get_path().len(), 1);
    assert_eq!(result.get_path()[0].get_id(), "A");
}

#[test]
fn dijkstra_returns_error_on_distance_overflow() {
    let mut graph = DirectedGraph::default();

    for id in ["A", "B", "C"] {
        graph.insert_node(node(id));
    }

    let node_a = node("A");
    let node_b = node("B");
    let node_c = node("C");
    assert!(
        graph
            .insert_edge(&node_a, &node_b, Some(u16::MAX - 1))
            .is_none()
    );
    assert!(graph.insert_edge(&node_b, &node_c, Some(2)).is_none());

    let dijkstra = DijkstraAlgorithm::new(graph);
    let err = dijkstra
        .shortest_path("A", "C")
        .expect_err("overflow should surface as a Dijkstra error");

    match err {
        DijkstraError::DistanceOverflow { from, to, .. } => {
            assert_eq!(from, "B");
            assert_eq!(to, "C");
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn dijkstra_error_kind_mapping_is_stable() {
    let missing_start = DijkstraError::MissingStartNode {
        id: "A".to_string(),
        graph: "D(nodes=1)".to_string(),
    };
    let missing_end = DijkstraError::MissingEndNode {
        id: "B".to_string(),
        graph: "D(nodes=1)".to_string(),
    };
    let missing_processing = DijkstraError::MissingNodeDuringProcessing {
        id: "C".to_string(),
        context: MissingNodeContext::CurrentNode,
    };
    let invalid_weight = DijkstraError::InvalidEdgeWeight {
        from: "A".to_string(),
        to: "B".to_string(),
        weight: "-1".to_string(),
        reason: EdgeWeightViolation::Negative,
    };
    let overflow = DijkstraError::DistanceOverflow {
        from: "A".to_string(),
        to: "B".to_string(),
        current_distance: "65535".to_string(),
        edge_weight: "1".to_string(),
    };
    let reconstruction = DijkstraError::PathReconstruction {
        source: DijkstraPathReconstructionError::MissingPredecessor {
            node_id: "X".to_string(),
        },
    };

    assert_eq!(
        DijkstraError::UnweightedGraph.kind(),
        AlgorithmErrorKind::InvalidGraph
    );
    assert_eq!(missing_start.kind(), AlgorithmErrorKind::MissingNode);
    assert_eq!(missing_end.kind(), AlgorithmErrorKind::MissingNode);
    assert_eq!(
        missing_processing.kind(),
        AlgorithmErrorKind::InvariantViolation
    );
    assert_eq!(invalid_weight.kind(), AlgorithmErrorKind::InvalidWeight);
    assert_eq!(overflow.kind(), AlgorithmErrorKind::InvalidWeight);
    assert_eq!(
        DijkstraError::NoPathFound {
            start: "A".to_string(),
            end: "B".to_string(),
        }
        .kind(),
        AlgorithmErrorKind::NoPath
    );
    assert_eq!(
        DijkstraError::InvalidSearchResult {
            reason: "bad".to_string(),
        }
        .kind(),
        AlgorithmErrorKind::InvalidResult
    );
    assert_eq!(
        reconstruction.kind(),
        AlgorithmErrorKind::InvariantViolation
    );
}
