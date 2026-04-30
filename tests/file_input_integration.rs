//! Integration tests for file-based graph input parsing.
//!
//! The parser is the main runtime input boundary, so these tests validate
//! successful loading for all supported graph headers and representative
//! malformed input cases.

use std::io::Write;

use shortest_path_finder::{
    data_input::file_input::retrieve_graph_data_from_file, graphs::graph::Graph,
};
use tempfile::NamedTempFile;

/// Counts edges in a directed graph by summing all outgoing neighbor lists.
fn count_directed_edges<G: Graph>(graph: &G) -> usize {
    graph
        .get_all_nodes()
        .iter()
        .map(|node| graph.neighbors(node).count())
        .sum()
}

/// Counts unique undirected edges by halving the bidirectional adjacency total.
fn count_undirected_edges<G: Graph>(graph: &G) -> usize {
    count_directed_edges(graph) / 2
}

fn write_temp_graph(contents: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("temp file creation should succeed");
    file.write_all(contents.as_bytes())
        .expect("temp file write should succeed");
    file.flush().expect("temp file flush should succeed");
    file
}

#[test]
fn parser_reads_directed_graph_from_file() {
    let file = write_temp_graph("D\nA->B:4\nB->C:2\nA->C:10\n");
    let path = file.path().to_string_lossy().into_owned();

    let result = retrieve_graph_data_from_file(&path).expect("directed parsing should succeed");

    assert!(result.directed_graph.is_some());
    assert!(result.undirected_graph.is_none());
    assert!(result.two_dimensional_graph.is_none());

    let graph = result.directed_graph.expect("directed graph must exist");
    assert_eq!(graph.get_all_nodes().len(), 3);
    assert_eq!(count_directed_edges(&graph), 3);
}

#[test]
fn parser_reads_undirected_graph_from_file() {
    let file = write_temp_graph("UN\nA-B:7\nB-C:3\nA-C:9\n");
    let path = file.path().to_string_lossy().into_owned();

    let result = retrieve_graph_data_from_file(&path).expect("undirected parsing should succeed");

    assert!(result.directed_graph.is_none());
    assert!(result.undirected_graph.is_some());
    assert!(result.two_dimensional_graph.is_none());

    let graph = result
        .undirected_graph
        .expect("undirected graph must exist");
    assert_eq!(graph.get_all_nodes().len(), 3);
    assert_eq!(count_undirected_edges(&graph), 3);
}

#[test]
fn parser_reads_two_dimensional_graph_from_file() {
    let file = write_temp_graph("TD\nA:0,0=>B:3,4\nB:3,4=>C:6,8\n");
    let path = file.path().to_string_lossy().into_owned();

    let result =
        retrieve_graph_data_from_file(&path).expect("two-dimensional parsing should succeed");

    assert!(result.directed_graph.is_none());
    assert!(result.undirected_graph.is_none());
    assert!(result.two_dimensional_graph.is_some());

    let graph = result
        .two_dimensional_graph
        .expect("two-dimensional graph must exist");
    assert_eq!(graph.get_all_nodes().len(), 3);
    assert!(!graph.is_directed());
    assert!(graph.is_weighted());
}

#[test]
fn parser_rejects_unknown_graph_header() {
    let file = write_temp_graph("XYZ\nA->B:4\n");
    let path = file.path().to_string_lossy().into_owned();

    let err = match retrieve_graph_data_from_file(&path) {
        Ok(_) => panic!("header should be invalid"),
        Err(err) => err,
    };

    assert!(err.to_string().contains("Invalid graph header"));
}

#[test]
fn parser_rejects_empty_file() {
    let file = write_temp_graph("");
    let path = file.path().to_string_lossy().into_owned();

    let err = match retrieve_graph_data_from_file(&path) {
        Ok(_) => panic!("empty file should fail"),
        Err(err) => err,
    };

    assert!(err.to_string().contains("file is empty"));
}

#[test]
fn parser_rejects_invalid_line_syntax() {
    let file = write_temp_graph("D\nA->B:not_a_number\n");
    let path = file.path().to_string_lossy().into_owned();

    let err = match retrieve_graph_data_from_file(&path) {
        Ok(_) => panic!("invalid line should fail"),
        Err(err) => err,
    };

    let message = err.to_string();
    assert!(message.contains("line 2"));
    assert!(message.contains("Expected directed syntax"));
}

#[test]
fn parser_ignores_whitespace_only_lines() {
    let file = write_temp_graph("D\nA->B:4\n   \n\t\nB->C:2\n");
    let path = file.path().to_string_lossy().into_owned();

    let result = retrieve_graph_data_from_file(&path)
        .expect("whitespace-only lines should be skipped during parsing");

    let graph = result.directed_graph.expect("directed graph must exist");
    assert_eq!(graph.get_all_nodes().len(), 3);
    assert_eq!(count_directed_edges(&graph), 2);
}

#[test]
fn parser_rejects_prefixed_graph_header() {
    let file = write_temp_graph("D_extra\nA->B:4\n");
    let path = file.path().to_string_lossy().into_owned();

    let err = match retrieve_graph_data_from_file(&path) {
        Ok(_) => panic!("prefixed header should be invalid"),
        Err(err) => err,
    };

    assert!(
        err.to_string()
            .contains("Expected exactly one of: D, UN, TD")
    );
}
