// TODO: Add logical benchmarks

use divan::{Bencher, bench};
use pathfinder::{
    algorithms::{algorithm::Algorithm, dijkstra::DijkstraAlgorithm},
    graphs::{
        directed::{DirectedEdge, DirectedGraph},
        graph::{Graph, Node},
        undirected::{UndirectedEdge, UndirectedGraph},
    },
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

#[bench]
fn get_all_nodes_from_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(Node::new("A".to_string()));
            graph.insert_node(Node::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                Node::new("A".to_string()),
                Node::new("B".to_string()),
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
            graph.insert_node(Node::new("A".to_string()));
            graph.insert_node(Node::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                Node::new("A".to_string()),
                Node::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _neighbors = dg.neighbors(&Node::new("A".to_string()));
        });
}

#[bench]
fn does_edge_already_exist_in_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(Node::new("A".to_string()));
            graph.insert_node(Node::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                Node::new("A".to_string()),
                Node::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _exists = dg.does_edge_already_exist(&DirectedEdge::new(
                Node::new("A".to_string()),
                Node::new("B".to_string()),
                5,
            ));
        });
}

#[bench]
fn does_node_already_exist_in_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(Node::new("A".to_string()));
            graph.insert_node(Node::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                Node::new("A".to_string()),
                Node::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _exists = dg.does_node_already_exist(&Node::new("A".to_string()));
        });
}

#[bench]
fn get_node_by_id_from_directed_graph(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let mut graph = DirectedGraph::default();
            graph.insert_node(Node::new("A".to_string()));
            graph.insert_node(Node::new("B".to_string()));
            graph.insert_edge(DirectedEdge::new(
                Node::new("A".to_string()),
                Node::new("B".to_string()),
                5,
            ));
            graph
        })
        .bench_refs(|dg| {
            let _node = dg.get_node_by_id("A");
        });
}

// ----- Benchmark the 'UndirectedGraph' -----

// ----- Benchmark the 'DijkstraAlgorithm' struct -----

// Example constant (uncomment and adapt as needed)
// const GRAPHS: &[&str] = &[];

#[bench]
fn find_shortest_path_in_directed_graph_with_dijkstra(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            DirectedGraph::new(
                vec![Node::new("A".into()), Node::new("B".into())],
                vec![DirectedEdge::new(
                    Node::new("A".into()),
                    Node::new("B".into()),
                    3,
                )],
            )
        })
        .bench_refs(|dg| {
            let algo_d = DijkstraAlgorithm::new(dg.clone());
            let _result = match algo_d
                .shortest_path(Node::new("A".to_string()), Node::new("B".to_string()))
            {
                Ok(path) => path,
                Err(_) => return,
            };
        });
}

#[bench]
fn find_shortest_path_in_undirected_graph_with_dijkstra(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            UndirectedGraph::new(
                vec![Node::new("A".into()), Node::new("B".into())],
                vec![UndirectedEdge::new(
                    Node::new("A".into()),
                    Node::new("B".into()),
                    3,
                )],
            )
        })
        .bench_refs(|dg| {
            let algo_d = DijkstraAlgorithm::new(dg.clone());
            let _result = match algo_d
                .shortest_path(Node::new("A".to_string()), Node::new("B".to_string()))
            {
                Ok(path) => path,
                Err(_) => return,
            };
        });
}
