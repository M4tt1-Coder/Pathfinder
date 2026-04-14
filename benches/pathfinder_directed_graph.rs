use divan::{Bencher, bench};
use shortest_path_finder::{
    graphs::{
        directed::{DirectedEdge, DirectedGraph},
        graph::Graph,
    },
    nodes::default_node::DefaultNode,
};

fn main() {
    divan::main();
}
// ----- Benchmark the 'DirectedGraph' -----

#[bench()]
fn create_directed_graph() {
    let _ = DirectedGraph::default();
}

#[bench]
fn add_edge_to_directed_graph() {
    let mut graph = DirectedGraph::default();
    graph.insert_node(DefaultNode::new("A".to_string()));
    graph.insert_node(DefaultNode::new("B".to_string()));
    graph.insert_edge(DirectedEdge::new(
        DefaultNode::new("A".to_string()),
        DefaultNode::new("B".to_string()),
        5,
    ));
}

#[bench]
fn get_all_nodes_from_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _nodes = dg.get_all_nodes();
        });
}

#[bench]
fn get_neighbors_from_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _neighbors = dg.neighbors(&DefaultNode::new("A".to_string()));
        });
}

#[bench]
fn does_edge_already_exist_in_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _exists = dg.does_edge_already_exist(&DirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            ));
        });
}

#[bench]
fn does_node_already_exist_in_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _exists = dg.does_node_already_exist(&DefaultNode::new("A".to_string()));
        });
}

#[bench]
fn get_node_by_id_from_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _node = dg.get_node_by_id("A");
        });
}
