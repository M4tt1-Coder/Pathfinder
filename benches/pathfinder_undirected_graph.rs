//! Benchmarks for undirected graph operations.
//!
//! # Overview
//!
//! This target measures key `UndirectedGraph` operations such as creation,
//! insertion, neighbor traversal, and edge/node lookup operations.
//!
//! # Run
//!
//! ```text
//! cargo bench --bench pathfinder_undirected_graph
//! ```

use divan::{Bencher, bench};
use shortest_path_finder::{
    graphs::{graph::Graph, undirected::UndirectedGraph},
    nodes::default_node::DefaultNode,
};

fn main() {
    divan::main();
}

// ----- Benchmark the 'UndirectedGraph' -----

#[bench]
fn create_undirected_graph() {
    let _ = UndirectedGraph::default();
}

#[bench]
fn insert_edge_to_undirected_graph() {
    let mut graph = UndirectedGraph::default();
    let from = DefaultNode::new("A".to_string());
    let to = DefaultNode::new("B".to_string());
    graph.insert_node(from.clone());
    graph.insert_node(to.clone());
    graph.insert_edge(&from, &to, Some(5));
}

#[bench]
fn does_node_already_exist_in_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            let from = DefaultNode::new("A".to_string());
            let to = DefaultNode::new("B".to_string());
            graph.insert_node(from.clone());
            graph.insert_node(to.clone());
            graph.insert_edge(&from, &to, Some(5));
            graph
        })
        .bench_refs(|dg| {
            let _exists = dg.does_node_already_exist(&DefaultNode::new("A".to_string()));
        });
}

#[bench]
fn does_edge_already_exist_in_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            let from = DefaultNode::new("A".to_string());
            let to = DefaultNode::new("B".to_string());
            graph.insert_node(from.clone());
            graph.insert_node(to.clone());
            graph.insert_edge(&from, &to, Some(5));
            graph
        })
        .bench_refs(|dg| {
            let from = DefaultNode::new("A".to_string());
            let to = DefaultNode::new("B".to_string());
            let _exists = dg.does_edge_already_exist(&from, &to);
        });
}

#[bench]
fn get_neighbors_of_node_in_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            let from = DefaultNode::new("A".to_string());
            let to = DefaultNode::new("B".to_string());
            graph.insert_node(from.clone());
            graph.insert_node(to.clone());
            graph.insert_edge(&from, &to, Some(5));
            graph
        })
        .bench_refs(|dg| {
            let _neighbors = dg.neighbors(&DefaultNode::new("A".to_string()));
        });
}

#[bench]
fn get_node_by_id_in_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            let from = DefaultNode::new("A".to_string());
            let to = DefaultNode::new("B".to_string());
            graph.insert_node(from.clone());
            graph.insert_node(to.clone());
            graph.insert_edge(&from, &to, Some(5));
            graph
        })
        .bench_refs(|dg| {
            let _node = dg.get_node_by_id("A");
        });
}

#[bench]
fn get_all_nodes_from_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            let from = DefaultNode::new("A".to_string());
            let to = DefaultNode::new("B".to_string());
            graph.insert_node(from.clone());
            graph.insert_node(to.clone());
            graph.insert_edge(&from, &to, Some(5));
            graph
        })
        .bench_refs(|dg| {
            let _nodes = dg.get_all_nodes();
        });
}
