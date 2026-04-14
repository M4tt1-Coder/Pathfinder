use divan::{Bencher, bench};
use shortest_path_finder::{
    algorithms::{algorithm::Algorithm, dijkstra::DijkstraAlgorithm},
    graphs::{
        directed::{DirectedEdge, DirectedGraph},
        undirected::{UndirectedEdge, UndirectedGraph},
    },
    nodes::default_node::DefaultNode,
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
                vec![DefaultNode::new("A".into()), DefaultNode::new("B".into())],
                vec![DirectedEdge::new(
                    DefaultNode::new("A".into()),
                    DefaultNode::new("B".into()),
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
                vec![DefaultNode::new("A".into()), DefaultNode::new("B".into())],
                vec![DirectedEdge::new(
                    DefaultNode::new("A".into()),
                    DefaultNode::new("B".into()),
                    3,
                )],
            )
        })
        .bench_refs(|dg| {
            let algo_d = DijkstraAlgorithm::new(dg.clone());
            let _result = match algo_d.shortest_path("A", "B") {
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
                vec![DefaultNode::new("A".into()), DefaultNode::new("B".into())],
                vec![UndirectedEdge::new(
                    DefaultNode::new("A".into()),
                    DefaultNode::new("B".into()),
                    3,
                )],
            )
        })
        .bench_refs(|dg| {
            let algo_d = DijkstraAlgorithm::new(dg.clone());
            let _result = match algo_d.shortest_path("A", "B") {
                Ok(path) => path,
                Err(_) => return,
            };
        });
}
