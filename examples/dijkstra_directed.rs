//! Run Dijkstra's algorithm on a weighted directed graph.

use shortest_path_finder::algorithms::{Algorithm, SearchResult};
use shortest_path_finder::graph::{Graph, GraphNode};
use shortest_path_finder::nodes::DefaultNode;
use shortest_path_finder::{Dijkstra, DirectedGraph};

fn main() {
    let node_a = DefaultNode::new("A".to_string());
    let node_b = DefaultNode::new("B".to_string());
    let node_c = DefaultNode::new("C".to_string());
    let mut graph = DirectedGraph::new(vec![node_a.clone(), node_b.clone(), node_c.clone()]);

    assert!(graph.insert_edge(&node_a, &node_b, Some(4)).is_none());
    assert!(graph.insert_edge(&node_b, &node_c, Some(2)).is_none());
    assert!(graph.insert_edge(&node_a, &node_c, Some(10)).is_none());

    let result = Dijkstra::new(graph)
        .shortest_path("A", "C")
        .expect("a path from A to C should exist");
    let path = result
        .get_path()
        .iter()
        .map(GraphNode::get_id)
        .collect::<Vec<_>>()
        .join(" -> ");

    println!("Directed graph with Dijkstra");
    println!("Path: {path}");
    println!("Distance: {}", result.get_total_distance());
}
