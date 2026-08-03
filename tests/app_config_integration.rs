//! Integration tests for command-line configuration parsing.
//!
//! These tests focus on realistic user-facing argument combinations and
//! validate defaults, optional flags, and required field handling.

use shortest_path_finder::{
    algorithms::Algorithms,
    data_input::file::cli_config::{AppConfig, AppConfigOutcome, InputOrigin},
    error::config_error::ConfigParseError,
};

fn build_args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_string()).collect()
}

fn unwrap_config(outcome: AppConfigOutcome) -> AppConfig {
    outcome.into_config().expect("expected config")
}

#[test]
fn setup_config_parses_required_arguments_and_defaults() {
    let args = build_args(&["pathfinder", "--start", "A", "--end", "D"]);

    let config = unwrap_config(AppConfig::setup_config(args).expect("expected valid config"));

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

    let config = unwrap_config(AppConfig::setup_config(args).expect("expected valid config"));

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
fn setup_config_reports_missing_end_node_when_incomplete() {
    let args = build_args(&["pathfinder", "--start", "A"]);

    let err = AppConfig::setup_config(args).expect_err("expected missing end error");

    assert_eq!(err, ConfigParseError::MissingRequiredFlag { flag: "--end" });
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

    let config = unwrap_config(AppConfig::setup_config(args).expect("expected valid config"));

    assert!(matches!(config.data_input, InputOrigin::CommandLine));
    assert!(matches!(config.algorithm, Algorithms::Dijkstra));
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

    let config = unwrap_config(AppConfig::setup_config(args).expect("expected valid config"));

    assert!(matches!(config.data_input, InputOrigin::CommandLine));
    assert!(matches!(config.algorithm, Algorithms::Dijkstra));
}

#[test]
fn setup_config_rejects_invalid_origin_value() {
    let args = build_args(&[
        "pathfinder",
        "--origin",
        "nowhere",
        "--start",
        "A",
        "--end",
        "B",
    ]);

    let err = AppConfig::setup_config(args).expect_err("expected invalid origin error");

    assert_eq!(
        err,
        ConfigParseError::InvalidFlagValue {
            flag: "--origin".to_string(),
            value: "nowhere".to_string(),
            expected: "file | cmd-line".to_string(),
        }
    );
}

#[test]
fn setup_config_rejects_invalid_algorithm_value() {
    let args = build_args(&[
        "pathfinder",
        "--algo",
        "Whoops",
        "--start",
        "A",
        "--end",
        "B",
    ]);

    let err = AppConfig::setup_config(args).expect_err("expected invalid algorithm error");

    assert_eq!(
        err,
        ConfigParseError::InvalidFlagValue {
            flag: "--algo".to_string(),
            value: "Whoops".to_string(),
            expected: "Dijkstra | AStar".to_string(),
        }
    );
}

#[test]
fn setup_config_rejects_conflicting_origin_and_graph_file() {
    let args = build_args(&[
        "pathfinder",
        "--origin",
        "cmd-line",
        "--graph-file",
        "graph.txt",
        "--start",
        "A",
        "--end",
        "B",
    ]);

    let err = AppConfig::setup_config(args).expect_err("expected conflict error");

    assert_eq!(
        err,
        ConfigParseError::ConflictingFlags {
            flag: "--origin".to_string(),
            other: "--graph-file".to_string(),
            reason: "command-line origin cannot be combined with --graph-file".to_string(),
        }
    );
}

#[test]
fn setup_config_returns_help_requested() {
    let args = build_args(&["pathfinder", "--help"]);

    let outcome = AppConfig::setup_config(args).expect("expected help outcome");

    assert!(matches!(outcome, AppConfigOutcome::HelpRequested));
}

#[test]
fn setup_config_returns_version_requested() {
    let args = build_args(&["pathfinder", "--version"]);

    let outcome = AppConfig::setup_config(args).expect("expected version outcome");

    assert!(matches!(outcome, AppConfigOutcome::VersionRequested));
}

#[test]
fn setup_config_rejects_missing_value_for_flag() {
    let args = build_args(&["pathfinder", "--start", "--end", "B"]);

    let err = AppConfig::setup_config(args).expect_err("expected missing value error");

    assert_eq!(
        err,
        ConfigParseError::MissingValueForFlag {
            flag: "--start".to_string(),
            index: 2,
        }
    );
}

#[test]
fn setup_config_accepts_double_dash_value_escape() {
    let args = build_args(&[
        "pathfinder",
        "--graph-file",
        "--",
        "--strange",
        "--start",
        "A",
        "--end",
        "B",
    ]);

    let config = unwrap_config(AppConfig::setup_config(args).expect("expected valid config"));

    assert_eq!(config.file_path, "--strange");
}

#[test]
fn setup_config_rejects_unexpected_end_of_options() {
    let args = build_args(&["pathfinder", "--", "--start", "A", "--end", "B"]);

    let err = AppConfig::setup_config(args).expect_err("expected end-of-options error");

    assert_eq!(err, ConfigParseError::UnexpectedEndOfOptions { index: 2 });
}

#[test]
fn setup_config_rejects_unknown_flag() {
    let args = build_args(&["pathfinder", "--whoops", "x", "--start", "A", "--end", "B"]);

    let err = AppConfig::setup_config(args).expect_err("expected unknown flag error");

    assert_eq!(
        err,
        ConfigParseError::UnknownFlag {
            flag: "--whoops".to_string(),
            index: 2,
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
            first_index: 2,
            duplicate_index: 4,
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
            index: 2,
        }
    );
}
