//! Benchmarks for command-line configuration parsing components.
//!
//! # Overview
//!
//! This target benchmarks creation and parsing behavior for:
//! - `InputOrigin` conversion,
//! - `AppConfig` setup across argument sets,
//! - `ConfigParseError` construction.
//!
//! # Run
//!
//! ```text
//! cargo bench --bench pathfinder_cmd_line
//! ```

use divan::{Bencher, bench};
use shortest_path_finder::{
    data_input::file::cli_config::{AppConfig, InputOrigin},
    error::config_error::ConfigParseError,
};

fn main() {
    divan::main();
}

// ----- Benchmark 'InputOrigin' enum -----

#[bench]
fn get_input_origin_from_string(bencher: Bencher) {
    bencher
        .with_inputs(|| "file".to_string())
        .bench_refs(|input| {
            let _origin = InputOrigin::try_from(input.as_str()).expect("valid origin");
        });
}

// ----- Benchmarks 'AppConfig' struct -----

#[bench(
    args = [
        vec!["--origin", "file", "--graph-file", "graph.txt", "--algo", "Dijkstra", "--start", "A", "--end", "D"],
        vec!["--origin", "file", "--graph-file", "graph.txt", "--algo", "AStar", "--start", "B", "--end", "E"],
        vec!["--origin", "cmd-line", "--start", "C", "--end", "F"]
    ]
)]
fn create_app_config_instance(args: &Vec<&str>) {
    // convert the &Vec<&str> to Vec<String>
    let args_string: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let _config = AppConfig::setup_config(args_string)
        .unwrap()
        .into_config()
        .expect("expected config");
}

// ----- Benchmarks 'ConfigParseError' enum -----

#[bench]
fn create_setup_process_error_instance() {
    let _err = ConfigParseError::MissingRequiredFlag { flag: "--start" };
}
