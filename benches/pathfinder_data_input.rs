use divan::bench;
use pathfinder::{
    data_input::file_input::{
        FileInputGraphResult, InvalidDataInputError, retrieve_graph_data_from_file,
    },
    graphs::{directed::DirectedGraph, undirected::UndirectedGraph},
};

fn main() {
    divan::main();
}

// ----- Benchmarks for the 'FileInputGraphResult' struct -----

#[bench(
    args = [(None, None), (Some(UndirectedGraph::new(vec![], vec![])), Some(DirectedGraph::new(vec![],vec![]))), (None, Some(DirectedGraph::new(vec![], vec![]))), (Some(UndirectedGraph::new(vec![], vec![])), None)]
)]
fn create_file_input_grap_result(graphs: &(Option<UndirectedGraph>, Option<DirectedGraph>)) {
    let _file_input_graph_result = FileInputGraphResult::new(graphs.1.clone(), graphs.0.clone());
}

// ----- Benchmarks of the 'InvalidDateInputError' struct -----

#[bench(
    args = ["The operation has been terminated due to excessive optimism. Please try again with a more pessimistic approach.", "Invalid input detected. It appears you've attempted to feed the system a contradictory paradox. Please try again with a more logical thought process.", "Authentication failed: It seems you've tried to log in with a password that's been lost in the void of time. Try again with a more temporal password.", "Error 404: The requested item has been misplaced in the vast expanse of cyberspace. Please try again with a more precise query."]
)]
fn create_invalid_data_input_error_struct(message: &str) {
    let _err = InvalidDataInputError::new(message.to_string());
}

// ----- Benchmarks of the 'retrieve_graph_data_from_file' function -----

#[bench(
    args = ["../test_files/directed-graph.txt", "../test_files/undirected_graph.txt"]
)]
fn generate_graphs_from_source_files(file_path: &str) {
    let _res_graph = match retrieve_graph_data_from_file(file_path) {
        Ok(graph) => graph,
        Err(_) => return,
    };
}

// ----- Benchmarks for the 'validate_line_syntax' function -----

#[bench(args = ["ABCDEFGHIJKLMNOPQRSTOVWXYZ-WOW:4", "A->B:46", "SourceNode-DestinationNode:456"])]
fn check_few_lines_for_syntax(line: &str) {}
