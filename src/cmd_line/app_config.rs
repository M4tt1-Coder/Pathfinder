// -> '--graph-file <relative_path_to_file>' specifies which file to use to generate the graph
// -> '--start <node_name>' name of the node to start from
// -> '--end <node_name>' destination node
// -> '--algo <algorithm_name>' specify which path finder algorithm to use (default Dijkstra)
// -> '--origin [file / cmd-line]' set the origin of how the graph data will be inserted

// ----- Implementation of the 'AppConfig' struct -----

use crate::{algorithms::algorithm::Algorithms, graphs::graph::Node};

/// Represents the data input method used to gather the information to generate the graph.
///
/// # Elements
///
/// - 'File' -> file input
/// - 'COMMAND_LINE' -> command line input
#[derive(Debug)]
pub enum InputOrigin {
    File,
    CommandLine,
}

impl InputOrigin {
    /// Generates an 'InputOrigin' enum value from a string.
    ///
    /// 'File' is the default.
    ///
    /// # Arguments
    ///
    /// - 'src' -> The string that is used for the process.
    ///
    /// # Returns
    ///
    /// => 'InputOrigin' in case a valid string was passed to the function.
    fn get_from_string(src: &str) -> Self {
        match src {
            "file" => Self::File,
            "cmd-line" => Self::CommandLine,
            _ => Self::File,
        }
    }
}

/// Contains the data for executing a path finder algorithm on a graph.
///
/// # Fields
///
/// - 'file_path' -> The path to the file with the graph data
/// - 'start_node' -> The node where the algorithm starts running from.
/// - 'end_node' -> Should be the last node in the path.
/// - 'algorithm' -> A specified algorithm by the user.
/// - 'data_input' -> A specification in which way the data will be inserted.
#[derive(Debug)]
pub struct AppConfig {
    pub file_path: String,
    pub start_node: Node,
    pub end_node: Node,
    pub algorithm: Algorithms,
    pub data_input: InputOrigin,
}

// pathfinder --graph graph.txt -start A --end D

impl AppConfig {
    /// Prepares all necessary data for the program to run.
    ///
    /// # Arguments
    ///
    /// - 'args' -> All passed arguments when the user called the binary executable.
    ///
    /// # Returns
    ///
    /// => Ok(AppConfig) containing all determined data.
    pub fn setup_config(args: Vec<String>) -> Result<Self, SetupProcessError> {
        if args.len() < 4 {
            return Err(SetupProcessError::new(
                "Not enough arguments passed! ('pathfinder [ --origin <file / cmd-line> --graph-file <path_to_file> --algo <algorithm_name>] --start <node> --end <node>')".to_string(),
            ));
        }
        // get all data and settings
        let file_path = AppConfig::retrieve_file_path(&args);
        let algorithm = AppConfig::retrieve_algorithm(&args);
        let data_input = AppConfig::retrieve_data_input(&args);

        // make sure 2 two 'start' and 'end' nodes have been passed
        let start_node = match AppConfig::retrieve_node(&args, true) {
            Some(node) => node,
            None => {
                return Err(SetupProcessError::new(
                    "A start node haven't been specified! ('--start A')".to_string(),
                ));
            }
        };

        let end_node = match AppConfig::retrieve_node(&args, false) {
            Some(node) => node,
            None => {
                return Err(SetupProcessError::new(
                    "A end node haven't been specified! ('--end B')".to_string(),
                ));
            }
        };

        Ok(AppConfig {
            file_path,
            algorithm,
            data_input,
            start_node,
            end_node,
        })
    }

    /// Gets the file path to a file containing the needed data to create a graph.
    ///
    /// # Arguments
    ///
    /// - 'args' -> List of all passed arguments to the executable.
    ///
    /// # Returns
    ///
    /// => The file path mentioned
    fn retrieve_file_path(args: &[String]) -> String {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--graph-file" && !args[i + 1].is_empty() {
                return args[i + 1].clone();
            }
        }
        "graph.txt".to_string()
    }

    /// Either the 'start' and 'end' node needed for the graph and shortest path algorithm.
    ///
    /// # Arguments
    ///
    /// - 'args' -> List of all arguments
    /// - 'is_start_node_requested' -> Determines wether to check for the 'start' or 'end' node.
    ///
    /// # Returns
    ///
    /// => The requested 'Node'
    fn retrieve_node(args: &[String], is_start_node_requested: bool) -> Option<Node> {
        let flag = if is_start_node_requested {
            "--start"
        } else {
            "--end"
        };
        for (i, arg) in args.iter().enumerate() {
            if arg == flag && !args[i + 1].is_empty() {
                return Some(Node::new(args[i + 1].clone()));
            }
        }
        None
    }

    /// Converts an algorithm from the passed string from the user.
    ///
    /// # Arguments
    ///
    /// - 'args' -> All arguments that were specified by the user during the call of the app.
    ///
    /// # Returns
    ///
    /// => Some(Algorithms) an implemented algorithm.
    fn retrieve_algorithm(args: &[String]) -> Algorithms {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--algo" && !args[i + 1].is_empty() {
                return Algorithms::get_from_string(&args[i + 1]);
            }
        }
        Algorithms::Dijkstra
    }

    /// Input method to get all data used.
    ///
    /// # Arguments
    ///
    /// - 'args' -> List of all arguments
    ///
    /// # Returns
    ///
    /// => Some(InputOrigin) in which way the dat is entered
    fn retrieve_data_input(args: &[String]) -> InputOrigin {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--algo" && !args[i + 1].is_empty() {
                return InputOrigin::get_from_string(&args[i + 1]);
            }
        }
        InputOrigin::File
    }
}

// ----- Implementation of the 'SetupProcessError' struct -----

/// The error returned by the setup service.
///
/// # Fields
///
/// - 'message' -> Description of the cause
#[derive(Debug)]
pub struct SetupProcessError {
    pub message: String,
}

impl SetupProcessError {
    /// Creates a new object of the 'SetupProcessError' struct.
    ///
    /// # Arguments
    ///
    /// - 'message' -> The descriptive message of the occured error.
    ///
    /// # Returns
    ///
    /// => 'SetupProcessError' struct.
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
