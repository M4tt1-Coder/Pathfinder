// TODO: Add logical benchmarks

use divan::bench;
use pathfinder::graphs::{
    directed::{DirectedEdge, DirectedGraph},
    graph::{Graph, Node},
};

fn main() {
    divan::main();
}

// ----- Benchmark the 'DirectedGraph' -----
#[bench]
fn create_directed_graph() {
    let _ = DirectedGraph::default();
}

#[bench]
fn add_edge_to_directed_graph() {
    let mut graph = DirectedGraph::default();
    graph.insert_node(Node::new("A".to_string()));
    graph.insert_node(Node::new("B".to_string()));
    graph.insert_edge(DirectedEdge::new(
        Node::new("A".to_string()),
        Node::new("B".to_string()),
        5,
    ));
}

// ----- Benchmark the 'DijkstraAlgorithm' struct -----

// #[divan::bench(
//     types = [DirectedGraph, UndirectedGraph],
//     args = []
// )]
// fn find_shortest_path_with_dijkstra() {}
