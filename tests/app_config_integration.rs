//! Integration tests for command-line configuration parsing.
//!
//! These tests focus on realistic user-facing argument combinations and
//! validate defaults, optional flags, and required field handling.

use shortest_path_finder::{
    algorithms::algorithm::Algorithms,
    cmd_line::app_config::{AppConfig, InputOrigin},
    error::config_error::ConfigParseError,
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

    assert_eq!(
        err,
        ConfigParseError::MissingRequiredFlag { flag: "--start" }
    );
}

#[test]
fn setup_config_requires_end_node() {
    let args = build_args(&["pathfinder", "--graph-file", "graph.txt", "--start", "A"]);

    let err = AppConfig::setup_config(args).expect_err("expected missing end error");

    assert_eq!(err, ConfigParseError::MissingRequiredFlag { flag: "--end" });
}

#[test]
fn setup_config_requires_minimum_argument_count() {
    let args = build_args(&["pathfinder", "--start", "A"]);

    let err = AppConfig::setup_config(args).expect_err("expected argument count error");

    assert_eq!(
        err,
        ConfigParseError::TooFewArguments {
            provided: 3,
            minimum: 4,
        }
    );
}

#[test]
fn setup_config_parses_origin_from_origin_flag() {
    let args = build_args(&[
        "pathfinder",
        "--origin",
        "cmd-line",
        "--start",
        "A",
        "--end",
        "B",
    ]);

    let config = AppConfig::setup_config(args).expect("expected valid config");

    assert!(matches!(config.data_input, InputOrigin::CommandLine));
}

#[test]
fn setup_config_keeps_legacy_origin_fallback_from_algo() {
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

#[test]
fn setup_config_rejects_missing_value_for_flag() {
    let args = build_args(&["pathfinder", "--start", "--end", "B"]);

    let err = AppConfig::setup_config(args).expect_err("expected missing value error");

    assert_eq!(
        err,
        ConfigParseError::MissingValueForFlag {
            flag: "--start".to_string(),
            index: 1,
        }
    );
}

#[test]
fn setup_config_rejects_unknown_flag() {
    let args = build_args(&["pathfinder", "--whoops", "x", "--start", "A", "--end", "B"]);

    let err = AppConfig::setup_config(args).expect_err("expected unknown flag error");

    assert_eq!(
        err,
        ConfigParseError::UnknownFlag {
            flag: "--whoops".to_string(),
            index: 1,
        }
    );
}

#[test]
fn setup_config_rejects_duplicate_flag() {
    let args = build_args(&["pathfinder", "--start", "A", "--start", "B", "--end", "C"]);

    let err = AppConfig::setup_config(args).expect_err("expected duplicate flag error");

    assert_eq!(
        err,
        ConfigParseError::DuplicateFlag {
            flag: "--start".to_string(),
            first_index: 1,
            duplicate_index: 3,
        }
    );
}

#[test]
fn setup_config_rejects_unexpected_non_flag_token() {
    let args = build_args(&["pathfinder", "start", "A", "--end", "B"]);

    let err = AppConfig::setup_config(args).expect_err("expected unexpected argument error");

    assert_eq!(
        err,
        ConfigParseError::UnexpectedArgument {
            value: "start".to_string(),
            index: 1,
        }
    );
}
