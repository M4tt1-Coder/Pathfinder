//! Binary entrypoint for the Pathfinder CLI application.
//!
//! # Overview
//!
//! The executable performs four high-level steps:
//! 1. Initialize logging.
//! 2. Parse command-line arguments into [`AppConfig`].
//! 3. Load graph data from the selected origin (currently file input).
//! 4. Execute the selected shortest-path algorithm and print the result.
//!
//! # Runtime Notes
//!
//! - `InputOrigin::File` is implemented and used in production flow.
//! - `InputOrigin::CommandLine` returns a structured CLI error for now.
//! - Algorithm selection: Dijkstra for directed (`D`) and undirected (`UN`)
//!   graphs; A* for two-dimensional (`TD`) graphs.
//!
//! # Error Handling
//!
//! The CLI wraps configuration, input, and algorithm failures in
//! [`shortest_path_finder::AppError`]. Algorithm-specific failures are still
//! mapped to exit codes via
//! [`shortest_path_finder::error::algorithm_error::AlgorithmErrorKind::exit_code`].
//! The CLI logs the error message before exiting.
//!
//! ```no_run
//! use shortest_path_finder::error::algorithm_error::AlgorithmErrorKind;
//!
//! assert_eq!(AlgorithmErrorKind::NoPath.exit_code(), 6);
//! ```
//!
//! # CLI Example
//!
//! ```no_run
//! use std::process::Command;
//!
//! let output = Command::new("pathfinder")
//!     .args([
//!         "--graph-file",
//!         "test_files/directed_graph.txt",
//!         "--start",
//!         "A",
//!         "--end",
//!         "B",
//!         "--algo",
//!         "Dijkstra",
//!     ])
//!     .output()
//!     .expect("failed to execute pathfinder process");
//!
//! assert!(output.status.success());
//! ```

use std::{env, process};

use log::error;
use shortest_path_finder::{
    AppError,
    algorithms::{
        a_star_algorithm::a_star::AStar,
        algorithm::{Algorithm, Algorithms},
        dijkstra::DijkstraAlgorithm,
    },
    cmd_line::app_config::{AppConfig, AppConfigOutcome, InputOrigin},
    data_input::file_input::{
        FileInputGraphResult::{DirectedGraph, TwoDimensionalGraph, UndirectedGraph},
        retrieve_graph_data_from_file,
    },
    error::algorithm_error::AlgorithmError,
};

// TODO: Add a visualization function where the user can see how the algorithm is working step by
// step (e.g. which nodes are being visited, which nodes are in the priority queue, ...). This can
// be done by adding a 'visualize' method to the 'Algorithm' trait and then implementing it for each
// algorithm. The user can then call this method after calling the 'shortest_path' method to see the
// visualization of the algorithm's execution.

// TODO: (Refactor) Refactor code -> apply best practices -> for each file indiviually, improve the
// visibility of the code + modulization
// - flatten import statements for the whole project with `use` in the the lib.rs file (code
//   elements in modules for the main functionality of the project) -> should be available in the
//   root -> also apply for all modules
// - export optional modules for the core functionality of the project (e.g. algorithms, data_input,
//   cmd_line, error) in the lib.rs with 'pub mod' -> longer import path with granular namespacing
// - private modules should stay private 'mod'

// TODO: Think of placing individual logic into features and then enabling them in the 'Cargo.toml'
// file (e.g. 'file_input', 'cmd_line_input', 'dijkstra_algorithm', 'a_star_algorithm', ...). This
// way, the user can choose which features to include in their project and which not (e.g. if they
// don't need the 'A*' algorithm, they can exclude it from their project and save some space).

// TODO: Feature that graphs can be selected to be none weighted -> each edge has weight of one.
// information is stored and the algorithm is executed accordingly.

// TODO: Try out CLI copilot or other agent cli tools + try out lightweight local open source models + Improve the workspace for working with AI -> let AI always write changes it made into a log
// file besides the diary so that it can look up what it did in the past and learn from it ->
// rewatch the video for some ideas => https://youtu.be/SuLHINfqJGI?si=RAqmGfo8pRcH5qjs

// TODO: Review existing tests and manually look for edge cases that are not covered by the existing
// tests and add tests for them

// TODO: Inspect benchmarks and cover edge cases, use bigger datasets (What about memory usage?)

/// Runs the Pathfinder CLI application lifecycle.
///
/// # Behavior
///
/// - Initializes logger output through `env_logger`.
/// - Parses CLI arguments into [`AppConfig`].
/// - Loads graph data according to `InputOrigin`.
/// - Executes selected algorithm for start/end node IDs.
/// - Prints the resulting path output and exits with status code.
///
/// # Exit Codes
///
/// - `0`: successful path computation.
/// - `1`: setup, parsing, or graph-loading failure.
/// - `2`: algorithm rejected an invalid graph (for example unweighted).
/// - `3`: required node is missing from the graph.
/// - `4`: invalid edge weight encountered.
/// - `5`: heuristic produced an invalid value.
/// - `6`: no path exists between the requested nodes.
/// - `7`: algorithm bookkeeping invariant failed.
/// - `8`: algorithm returned an invalid result.
fn run() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    // CLI arguments are parsed by AppConfig:
    // - --graph-file <path> selects the input file (default: graph.txt)
    // - --start <node> and --end <node> select the search endpoints
    // - --algo <algorithm_name> selects Dijkstra or AStar
    // - --origin <file|cmd-line> selects where graph data should come from

    // validate the arguments and generate config data
    let app_config = match AppConfig::setup_config(args)? {
        AppConfigOutcome::Config(config) => config,
        AppConfigOutcome::HelpRequested => {
            println!("{}", AppConfig::help_text());
            return Ok(());
        }
        AppConfigOutcome::VersionRequested => {
            println!("{}", AppConfig::version_text());
            return Ok(());
        }
    };

    // create the graph and execute the algorithm on it
    match app_config.data_input {
        InputOrigin::File => {
            let generated_graph_res = retrieve_graph_data_from_file(&app_config.file_path)?;

            // match all possible graph types
            match generated_graph_res {
                DirectedGraph(graph) => {
                    let algo = match app_config.algorithm {
                        Algorithms::Dijkstra => DijkstraAlgorithm::new(graph),
                        _ => {
                            return Err(AppError::Runtime {
                                message: format!(
                                    "Algorithm {:?} is not implemented for directed graphs yet or a directed graph is not supported by the implementation of the algorithm!",
                                    app_config.algorithm
                                ),
                            });
                        }
                    };
                    let result = algo
                        .shortest_path(&app_config.start_node_id, &app_config.end_node_id)
                        .map_err(AlgorithmError::from)?;
                    // display the result
                    println!("{}", result);
                    Ok(())
                }
                UndirectedGraph(graph) => {
                    let algo = match app_config.algorithm {
                        Algorithms::Dijkstra => DijkstraAlgorithm::new(graph),
                        _ => {
                            return Err(AppError::Runtime {
                                message: format!(
                                    "Algorithm {:?} is not implemented for undirected graphs yet or an undirected graph is not supported by the implementation of the algorithm!",
                                    app_config.algorithm
                                ),
                            });
                        }
                    };
                    let result = algo
                        .shortest_path(&app_config.start_node_id, &app_config.end_node_id)
                        .map_err(AlgorithmError::from)?;
                    // display the result
                    println!("{}", result);
                    Ok(())
                }
                TwoDimensionalGraph(graph) => {
                    let algo = match app_config.algorithm {
                        Algorithms::AStar => AStar::new(graph),
                        _ => {
                            return Err(AppError::Runtime {
                                message: format!(
                                    "Algorithm {:?} is not implemented for two dimensional graphs yet or a two dimensional graph is not supported by the implementation of the algorithm!",
                                    app_config.algorithm
                                ),
                            });
                        }
                    };
                    let result = algo
                        .shortest_path(&app_config.start_node_id, &app_config.end_node_id)
                        .map_err(AlgorithmError::from)?;
                    // display the result
                    println!("{}", result);
                    Ok(())
                }
            }
        }
        InputOrigin::CommandLine => Err(AppError::UnsupportedInputOrigin {
            origin: app_config.data_input.as_str().to_string(),
        }),
    }
}

/// Binary entrypoint: initializes logging and runs [`run`].
///
/// On failure, logs the [`AppError`] message and exits with
/// [`AppError::exit_code`].
fn main() {
    // enable logging to the terminal
    env_logger::init();

    if let Err(err) = run() {
        error!("{}", err);
        process::exit(err.exit_code());
    }
}
