//! Benchmarks for file-input parsing components.
//!
//! # Overview
//!
//! This target benchmarks:
//! - `FileInputGraphResult` construction,
//! - parse-error variant creation,
//! - graph loading from sample files.
//!
//! # Run
//!
//! ```text
//! cargo bench --bench pathfinder_data_input
//! ```

use divan::bench;
use shortest_path_finder::{
    data_input::file::{FileInputGraphResult, retrieve_graph_data_from_file},
    error::ParseError,
    graph::{DirectedGraph, TwoDimensionalCoordinateGraph, UndirectedGraph},
    nodes::{DefaultNode, TwoDimensionalNode},
};

fn main() {
    divan::main();
}

// ----- Benchmarks for the 'FileInputGraphResult' enum -----

#[bench(
    args = [
        DirectedGraph::new(vec![]),
        DirectedGraph::new(vec![
            DefaultNode::new("A".to_string()),
            DefaultNode::new("B".to_string()),
            DefaultNode::new("C".to_string())
        ])
    ]
)]
fn create_file_input_graph_result_with_directed_graph(dir_graph: &DirectedGraph) {
    let _result = FileInputGraphResult::DirectedGraph(dir_graph.clone());
}

#[bench(
    args = [
        UndirectedGraph::new(vec![]),
        UndirectedGraph::new(vec![
            DefaultNode::new("X".to_string()),
            DefaultNode::new("Y".to_string()),
            DefaultNode::new("Z".to_string())
        ])
    ]
)]
fn create_file_input_graph_result_with_undirected_graph(undir_graph: &UndirectedGraph) {
    let _result = FileInputGraphResult::UndirectedGraph(undir_graph.clone());
}

#[bench(
    args = [
        TwoDimensionalCoordinateGraph::<i32>::new(vec![]),
        TwoDimensionalCoordinateGraph::<i32>::new(vec![
            TwoDimensionalNode::new(1, 2, "P1".to_string()).expect("Failed to create node"),
            TwoDimensionalNode::new(3, 4, "P2".to_string()).expect("Failed to create node"),
            TwoDimensionalNode::new(5, 6, "P3".to_string()).expect("Failed to create node")
        ])
    ]
)]
fn create_file_input_graph_result_with_2d_coordinate_graph(
    coord_graph: &TwoDimensionalCoordinateGraph,
) {
    let _result = FileInputGraphResult::TwoDimensionalGraph(coord_graph.clone());
}

// ----- Benchmarks of the 'ParseError' enum -----

#[bench(
    args = ["The operation has been terminated due to excessive optimism. Please try again with a more pessimistic approach.", "Invalid input detected. It appears you've attempted to feed the system a contradictory paradox. Please try again with a more logical thought process.", "Authentication failed: It seems you've tried to log in with a password that's been lost in the void of time. Try again with a more temporal password.", "Error 404: The requested item has been misplaced in the vast expanse of cyberspace. Please try again with a more precise query."]
)]
fn create_parse_error_invalid_data_input_variant(message: &str) {
    let _err = ParseError::InvalidDataInput(message.to_string());
}

// ----- Benchmarks of the 'retrieve_graph_data_from_file' function -----

#[bench(
    args = ["../test_files/directed_graph.txt", "../test_files/undirected_graph.txt"]
)]
fn generate_graphs_from_source_files(file_path: &str) {
    let _res_graph = match retrieve_graph_data_from_file(file_path) {
        Ok(graph) => graph,
        Err(_) => return,
    };
}
