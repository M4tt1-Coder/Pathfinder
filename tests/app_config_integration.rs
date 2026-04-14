//! Integration tests for command-line configuration parsing.
//!
//! These tests focus on realistic user-facing argument combinations and
//! validate defaults, optional flags, and required field handling.

use shortest_path_finder::{
    algorithms::algorithm::Algorithms,
    cmd_line::app_config::{AppConfig, InputOrigin},
};

fn build_args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_string()).collect()
}

#[test]
fn setup_config_parses_required_arguments_and_defaults() {
    let args = build_args(&["pathfinder", "--start", "A", "--end", "D"]);

    let config = AppConfig::setup_config(args).expect("expected valid config");

    assert_eq!(config.file_path, "graph.txt");
    assert_eq!(config.start_node_id, "A");
    assert_eq!(config.end_node_id, "D");
    assert!(matches!(config.algorithm, Algorithms::Dijkstra));
    assert!(matches!(config.data_input, InputOrigin::File));
}

#[test]
fn setup_config_parses_optional_graph_file_and_algorithm() {
    let args = build_args(&[
        "pathfinder",
        "--graph-file",
        "test_files/directed_graph.txt",
        "--algo",
        "AStar",
        "--start",
        "A",
        "--end",
        "B",
    ]);

    let config = AppConfig::setup_config(args).expect("expected valid config");

    assert_eq!(config.file_path, "test_files/directed_graph.txt");
    assert!(matches!(config.algorithm, Algorithms::AStar));
}

#[test]
fn setup_config_requires_start_node() {
    let args = build_args(&["pathfinder", "--graph-file", "graph.txt", "--end", "B"]);

    let err = AppConfig::setup_config(args).expect_err("expected missing start error");

    assert!(err.message.contains("start node"));
}

#[test]
fn setup_config_requires_end_node() {
    let args = build_args(&["pathfinder", "--graph-file", "graph.txt", "--start", "A"]);

    let err = AppConfig::setup_config(args).expect_err("expected missing end error");

    assert!(err.message.contains("end node"));
}

#[test]
fn setup_config_requires_minimum_argument_count() {
    let args = build_args(&["pathfinder", "--start", "A"]);

    let err = AppConfig::setup_config(args).expect_err("expected argument count error");

    assert!(err.message.contains("Not enough arguments"));
}

#[test]
fn setup_config_keeps_current_origin_parsing_behavior() {
    // Current implementation derives input origin from the --algo value.
    // This test documents and protects that behavior to prevent accidental
    // regressions until a dedicated CLI parsing fix is introduced.
    let args = build_args(&[
        "pathfinder",
        "--algo",
        "cmd-line",
        "--start",
        "A",
        "--end",
        "B",
    ]);

    let config = AppConfig::setup_config(args).expect("expected valid config");

    assert!(matches!(config.data_input, InputOrigin::CommandLine));
}
