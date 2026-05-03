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
//! - `InputOrigin::CommandLine` is currently `unimplemented!()`.
//! - Algorithm selection: Dijkstra for directed (`D`) and undirected (`UN`)
//!   graphs; A* for two-dimensional (`TD`) graphs.
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
    algorithms::{
        a_star_algorithm::a_star::AStar,
        algorithm::{Algorithm, Algorithms},
        dijkstra::DijkstraAlgorithm,
    },
    cmd_line::app_config::{AppConfig, InputOrigin},
    data_input::file_input::retrieve_graph_data_from_file,
};

// TODO: Add a visualization function where the user can see how the algorithm is working step by
// step (e.g. which nodes are being visited, which nodes are in the priority queue, ...). This can
// be done by adding a 'visualize' method to the 'Algorithm' trait and then implementing it for each
// algorithm. The user can then call this method after calling the 'shortest_path' method to see the
// visualization of the algorithm's execution.

// TODO: (Refactor) Refactor code -> apply best practices, apply better error handling -> for each file indiviually, improve the
// visibility of the code + modulization

// TODO: Think of placing individual logic into features and then enabling them in the 'Cargo.toml'
// file (e.g. 'file_input', 'cmd_line_input', 'dijkstra_algorithm', 'a_star_algorithm', ...). This
// way, the user can choose which features to include in their project and which not (e.g. if they
// don't need the 'A*' algorithm, they can exclude it from their project and save some space).

// TODO: Feature that graphs can be selected to be none weighted -> each edge has weight of one.
// information is stored and the algorithm is executed accordingly.

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
/// - `1`: setup, parsing, graph-loading, or algorithm execution failure.
fn main() {
    // enable logging to the terminal
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    // -> '--graph <relative_path_to_file>' specifies which file to use to generate the graph
    // -> '--start <node_name>' name of the node to start from
    // -> '--end <node_name>' destination node
    // -> '--algo <algorithm_name>' specify which path finder algorithm to use (default Dijkstra)
    // -> '--origin [file / cmd-line]' set the origin of how the graph data will be inserted
    // (default: file with the name 'graph.txt')

    // validate the arguments and generate config data
    let app_config = match AppConfig::setup_config(args) {
        Ok(config) => config,
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };

    // create the graph and execute the algorithm on it
    match app_config.data_input {
        InputOrigin::File => {
            let graphs = match retrieve_graph_data_from_file(&app_config.file_path) {
                Ok(graph) => graph,
                Err(err) => {
                    error!("{}", err);
                    process::exit(1);
                }
            };
            if let Some(graph) = graphs.directed_graph {
                let algo = match app_config.algorithm {
                    Algorithms::Dijkstra => DijkstraAlgorithm::new(graph),
                    _ => {
                        error!(
                            "Algorithm {:?} is not implemented for directed graphs yet or a directed graph is not supported by the implementation of the algorithm!",
                            app_config.algorithm
                        );
                        process::exit(1);
                    }
                };
                let result =
                    match algo.shortest_path(&app_config.start_node_id, &app_config.end_node_id) {
                        Ok(res) => res,
                        Err(err) => {
                            error!("{}", err.message);
                            process::exit(1);
                        }
                    };
                // display the result
                println!("{}", result);
                process::exit(0);
            } else if let Some(graph) = graphs.undirected_graph {
                let algo = match app_config.algorithm {
                    Algorithms::Dijkstra => DijkstraAlgorithm::new(graph),
                    _ => {
                        error!(
                            "Algorithm {:?} is not implemented for undirected graphs yet or an undirected graph is not supported by the implementation of the algorithm!",
                            app_config.algorithm
                        );
                        process::exit(1);
                    }
                };
                let result =
                    match algo.shortest_path(&app_config.start_node_id, &app_config.end_node_id) {
                        Ok(res) => res,
                        Err(err) => {
                            error!("{}", err.message);
                            process::exit(1);
                        }
                    };
                // display the result
                println!("{}", result);
                process::exit(0);
            } else if let Some(graph) = graphs.two_dimensional_graph {
                let algo = match app_config.algorithm {
                    Algorithms::AStar => AStar::new(graph),
                    _ => {
                        error!(
                            "Algorithm {:?} is not implemented for two dimensional graphs yet or a two dimensional graph is not supported by the implementation of the algorithm!",
                            app_config.algorithm
                        );
                        process::exit(1);
                    }
                };
                let result =
                    match algo.shortest_path(&app_config.start_node_id, &app_config.end_node_id) {
                        Ok(res) => res,
                        Err(err) => {
                            error!("{}", err.message);
                            process::exit(1);
                        }
                    };
                // display the result
                println!("{}", result);
                process::exit(0);
            } else {
                error!(
                    "No graph was create from the file {}!",
                    app_config.file_path
                );
                process::exit(1);
            };
        }
        InputOrigin::CommandLine => unimplemented!(),
    }
}
