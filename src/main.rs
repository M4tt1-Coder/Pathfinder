use std::{env, process};

use log::error;
use shortest_path_finder::{
    algorithms::{
        algorithm::{Algorithm, Algorithms},
        dijkstra::DijkstraAlgorithm,
    },
    cmd_line::app_config::{AppConfig, InputOrigin},
    data_input::file_input::retrieve_graph_data_from_file,
};

// TODO: Run a prompt that searches for errors in the whole source code and then ask if they should
// be made global and if they should be refactored to a more generic error type (e.g. 'AppError' or
// 'GraphError' or 'ParseError' or 'InputError' or ...) and place them in the 'error' module. Also,
// check if the error types are used in the right way and if they are used at all (if not, remove
// them).

// TODO: Refactor code -> check if in some cases references are better then cloning (if possible),
// apply best practices, apply better error handling

// TODO: Make 'A*' algorithm usable (test it) and benchmark it against Dijkstra's algorithm

// TODO: Publish to crates.io and add a badge to the README.md file

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
            error!("{}", err.message);
            process::exit(1);
        }
    };

    // create the graph and execute the algorithm on it
    match app_config.data_input {
        InputOrigin::File => {
            let graphs = match retrieve_graph_data_from_file(&app_config.file_path) {
                Ok(graph) => graph,
                Err(err) => {
                    error!("Here: {}", err);
                    process::exit(1);
                }
            };
            if let Some(graph) = graphs.directed_graph {
                let algo = match app_config.algorithm {
                    Algorithms::Dijkstra => DijkstraAlgorithm::new(graph),
                    Algorithms::AStar => unimplemented!(),
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
                    Algorithms::AStar => unimplemented!(),
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
