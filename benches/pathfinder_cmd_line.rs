use divan::{Bencher, bench};
use pathfinder::cmd_line::app_config::{AppConfig, InputOrigin, SetupProcessError};

fn main() {
    divan::main();
}

// ----- Benchmark 'InputOrigin' enum -----

// mocked 'get_from_string' function benchmark
fn get_from_string(src: &str) -> InputOrigin {
    match src {
        "file" => InputOrigin::File,
        "cmd-line" => InputOrigin::CommandLine,
        _ => InputOrigin::File,
    }
}

#[bench]
fn get_input_origin_from_string(bencher: Bencher) {
    bencher
        .with_inputs(|| "file".to_string())
        .bench_refs(|input| {
            let _origin = get_from_string(input);
        });
}

// ----- Benchmarks 'AppConfig' struct -----

#[bench(
    args = [
        vec!["--origin", "file", "--graph-file", "graph.txt", "--algo", "dijkstra", "--start", "A", "--end", "D"],
        vec!["--origin", "file", "--graph-file", "graph.txt", "--algo", "a_star", "--start", "B", "--end", "E"],
        vec!["--origin", "cmd-line", "--start", "C", "--end", "F"]
    ]
)]
fn create_app_config_instance(args: &Vec<&str>) {
    // convert the &Vec<&str> to Vec<String>
    let args_string: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let _config = AppConfig::setup_config(args_string).unwrap();
}

// ----- Benchmarks 'SetupProcessError' struct -----

#[bench]
fn create_setup_process_error_instance() {
    let _err = SetupProcessError::new("Some error message!".to_string());
}
