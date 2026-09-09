//! Run A* on a weighted graph whose nodes have two-dimensional coordinates.

use shortest_path_finder::algorithms::{Algorithm, SearchResult};
use shortest_path_finder::graph::{Graph, GraphNode};
use shortest_path_finder::nodes::TwoDimensionalNode;
use shortest_path_finder::{AStar, TwoDimensionalCoordinateGraph};

fn main() {
    let node_a = TwoDimensionalNode::new(0, 0, "A".to_string()).expect("node ID is non-empty");
    let node_b = TwoDimensionalNode::new(2, 0, "B".to_string()).expect("node ID is non-empty");
    let node_c = TwoDimensionalNode::new(4, 0, "C".to_string()).expect("node ID is non-empty");
    let mut graph =
        TwoDimensionalCoordinateGraph::new(vec![node_a.clone(), node_b.clone(), node_c.clone()]);

    assert!(graph.insert_edge(&node_a, &node_b, None).is_none());
    assert!(graph.insert_edge(&node_b, &node_c, None).is_none());

    let result = AStar::new(graph)
        .shortest_path("A", "C")
        .expect("a path from A to C should exist");
    let path = result
        .get_path()
        .iter()
        .map(GraphNode::get_id)
        .collect::<Vec<_>>()
        .join(" -> ");

    println!("Two-dimensional coordinate graph with A*");
    println!("Path: {path}");
    println!("Distance: {}", result.get_total_distance());
}
