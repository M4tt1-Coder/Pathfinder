use divan::{Bencher, bench};
use pathfinder::{
    algorithms::{algorithm::Algorithm, dijkstra::DijkstraAlgorithm},
    graphs::{
        directed::{DirectedEdge, DirectedGraph},
        graph::Node,
        undirected::{UndirectedEdge, UndirectedGraph},
    },
};

fn main() {
    divan::main();
}

// ----- Benchmark the 'DijkstraAlgorithm' struct -----

// Example constant (uncomment and adapt as needed)
// const GRAPHS: &[&str] = &[];

#[bench]
fn create_dijkstra_algorithm_instance(bencher: Bencher) {
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
            let _algo_d = DijkstraAlgorithm::new(dg.clone());
        });
}

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
