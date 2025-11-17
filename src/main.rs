use std::{env, process};

use log::error;
use pathfinder::{
    algorithms::{algorithm::Algorithm, dijkstra::DijkstraAlgorithm},
    cmd_line::app_config::AppConfig,
    data_input::file_input::retrieve_graph_data_from_file,
};

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
            error!("Here: {}", err.message);
            process::exit(1);
        }
    };

    // create the graph and execute the algorithm on it
    match app_config.data_input {
        pathfinder::cmd_line::app_config::InputOrigin::File => {
            let graphs = match retrieve_graph_data_from_file(&app_config.file_path) {
                Ok(graph) => graph,
                Err(err) => {
                    error!("Here: {}", err);
                    process::exit(1);
                }
            };
            if let Some(graph) = graphs.directed_graph {
                let algo = match app_config.algorithm {
                    pathfinder::algorithms::algorithm::Algorithms::Dijkstra => {
                        DijkstraAlgorithm::new(graph)
                    }
                };
                let result = match algo.shortest_path(app_config.start_node, app_config.end_node) {
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
                    pathfinder::algorithms::algorithm::Algorithms::Dijkstra => {
                        DijkstraAlgorithm::new(graph)
                    }
                };
                let result = match algo.shortest_path(app_config.start_node, app_config.end_node) {
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
        pathfinder::cmd_line::app_config::InputOrigin::CommandLine => unimplemented!(),
    }
}
