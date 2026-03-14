use divan::{Bencher, bench};
use shortest_path_finder::{
    graphs::{
        graph::{Graph, GraphEdge},
        undirected::{UndirectedEdge, UndirectedGraph},
    },
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
    graph.insert_node(DefaultNode::new("A".to_string()));
    graph.insert_node(DefaultNode::new("B".to_string()));
    graph.insert_edge(UndirectedEdge::new(
        DefaultNode::new("A".to_string()),
        DefaultNode::new("B".to_string()),
        5,
    ));
}

#[bench]
fn does_node_already_exist_in_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(UndirectedEdge::new(
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
fn does_edge_already_exist_in_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(UndirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _exists = dg.does_edge_already_exist(&UndirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            ));
        });
}

#[bench]
fn get_neighbors_of_node_in_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(UndirectedEdge::new(
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
fn get_node_by_id_in_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(UndirectedEdge::new(
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

#[bench]
fn get_edge_by_id_from_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            let edge = UndirectedEdge::new(
                DefaultNode::new("A".to_string()),
                DefaultNode::new("B".to_string()),
                5,
            );
            let edge_id = edge.get_id();
            graph.insert_edge(edge);
            (graph, edge_id)
        })
        .bench_refs(|(dg, edge_id)| {
            let _edge = dg.get_edge_by_id(edge_id);
        });
}

#[bench]
fn get_all_nodes_from_undirected_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = UndirectedGraph::default();
            graph.insert_node(DefaultNode::new("A".to_string()));
            graph.insert_node(DefaultNode::new("B".to_string()));
            graph.insert_edge(UndirectedEdge::new(
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
