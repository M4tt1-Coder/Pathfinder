//! Command-line configuration parsing for the `pathfinder` binary.
//!
//! This module turns raw CLI arguments into strongly typed runtime configuration.
//! The main entry point is [`AppConfig::setup_config`], which validates arguments,
//! applies defaults, and returns an [`AppConfig`] used by the application runtime.
//!
//! # Supported flags
//!
//! - `--graph-file <relative_path_to_file>`: file used to build the graph.
//! - `--start <node_name>`: start node identifier (required).
//! - `--end <node_name>`: destination node identifier (required).
//! - `--algo <algorithm_name>`: algorithm selector (defaults to `Dijkstra`).
//! - `--origin <file|cmd-line>`: intended input-origin selector.
//!
//! # Defaults and compatibility notes
//!
//! - Missing `--graph-file` defaults to `graph.txt`.
//! - Missing or unknown `--algo` defaults to `Dijkstra`.
//! - Current compatibility behavior: input-origin parsing reads from `--algo`
//!   (not from `--origin`) in [`AppConfig::retrieve_data_input`].
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::algorithms::algorithm::Algorithms;
//! use shortest_path_finder::cmd_line::app_config::{AppConfig, InputOrigin};
//!
//! let args = vec![
//!     "pathfinder",
//!     "--graph-file",
//!     "test_files/directed_graph.txt",
//!     "--start",
//!     "A",
//!     "--end",
//!     "D",
//!     "--algo",
//!     "Dijkstra",
//! ]
//! .into_iter()
//! .map(String::from)
//! .collect();
//!
//! let config = AppConfig::setup_config(args).unwrap();
//!
//! assert_eq!(config.file_path, "test_files/directed_graph.txt");
//! assert_eq!(config.start_node_id, "A");
//! assert_eq!(config.end_node_id, "D");
//! assert!(matches!(config.algorithm, Algorithms::Dijkstra));
//! assert!(matches!(config.data_input, InputOrigin::File));
//! ```

use crate::algorithms::algorithm::Algorithms;

// TODO: Implement a more complex error handling system for the setup process. (Maybe with an enum
// of different error types?)

/// Declares where graph data should be read from.
///
/// # Variants
///
/// - [`InputOrigin::File`]: parse graph data from a file.
/// - [`InputOrigin::CommandLine`]: parse graph data from terminal input.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::cmd_line::app_config::{AppConfig, InputOrigin};
///
/// // The current parser reads input-origin values from `--algo`.
/// let args = vec![
///     "pathfinder",
///     "--start",
///     "A",
///     "--end",
///     "B",
///     "--algo",
///     "cmd-line",
/// ]
/// .into_iter()
/// .map(String::from)
/// .collect();
///
/// let config = AppConfig::setup_config(args).unwrap();
/// assert!(matches!(config.data_input, InputOrigin::CommandLine));
/// ```
#[derive(Debug)]
pub enum InputOrigin {
    /// Use file-based data input.
    File,
    /// Use interactive command-line data input.
    CommandLine,
}

impl InputOrigin {
    /// Converts a raw text token into an [`InputOrigin`] variant.
    ///
    /// # Parameters
    ///
    /// - `src`: raw token to parse.
    ///
    /// # Returns
    ///
    /// - [`InputOrigin::File`] when `src` is `"file"`.
    /// - [`InputOrigin::CommandLine`] when `src` is `"cmd-line"`.
    /// - [`InputOrigin::File`] for any unknown token.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::{AppConfig, InputOrigin};
    ///
    /// // Demonstrated via the public setup function.
    /// let args = vec![
    ///     "pathfinder",
    ///     "--start",
    ///     "A",
    ///     "--end",
    ///     "B",
    ///     "--algo",
    ///     "file",
    /// ]
    /// .into_iter()
    /// .map(String::from)
    /// .collect();
    ///
    /// let config = AppConfig::setup_config(args).unwrap();
    /// assert!(matches!(config.data_input, InputOrigin::File));
    /// ```
    fn get_from_string(src: &str) -> Self {
        match src {
            "file" => Self::File,
            "cmd-line" => Self::CommandLine,
            _ => Self::File,
        }
    }
}

/// Runtime configuration extracted from command-line arguments.
///
/// # Fields
///
/// - `file_path`: path to graph input file.
/// - `start_node_id`: identifier of the start node.
/// - `end_node_id`: identifier of the destination node.
/// - `algorithm`: shortest-path algorithm selected by the user.
/// - `data_input`: graph-data origin.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::algorithms::algorithm::Algorithms;
/// use shortest_path_finder::cmd_line::app_config::{AppConfig, InputOrigin};
///
/// let args = vec![
///     "pathfinder",
///     "--graph-file",
///     "graph.txt",
///     "--start",
///     "A",
///     "--end",
///     "D",
///     "--algo",
///     "AStar",
/// ]
/// .into_iter()
/// .map(String::from)
/// .collect();
///
/// let config = AppConfig::setup_config(args).unwrap();
///
/// assert_eq!(config.file_path, "graph.txt");
/// assert_eq!(config.start_node_id, "A");
/// assert_eq!(config.end_node_id, "D");
/// assert!(matches!(config.algorithm, Algorithms::AStar));
/// assert!(matches!(config.data_input, InputOrigin::File));
/// ```
#[derive(Debug)]
pub struct AppConfig {
    /// Relative or absolute path to the graph input file.
    pub file_path: String,
    /// Identifier of the node where path search starts.
    pub start_node_id: String,
    /// Identifier of the node where path search ends.
    pub end_node_id: String,
    /// Selected shortest-path algorithm.
    pub algorithm: Algorithms,
    /// Origin used to read graph data.
    pub data_input: InputOrigin,
}

impl AppConfig {
    /// Builds an [`AppConfig`] by parsing and validating CLI arguments.
    ///
    /// # Parameters
    ///
    /// - `args`: full argument vector, typically from `std::env::args().collect()`.
    ///
    /// # Returns
    ///
    /// - `Ok(AppConfig)` when required information is present.
    /// - `Err(SetupProcessError)` when required information is missing.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - fewer than four arguments are provided,
    /// - `--start` is missing,
    /// - `--end` is missing.
    ///
    /// # Examples
    ///
    /// Successful parsing with defaults:
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::algorithm::Algorithms;
    /// use shortest_path_finder::cmd_line::app_config::{AppConfig, InputOrigin};
    ///
    /// let args = vec![
    ///     "pathfinder",
    ///     "--start",
    ///     "A",
    ///     "--end",
    ///     "B",
    /// ]
    /// .into_iter()
    /// .map(String::from)
    /// .collect();
    ///
    /// let config = AppConfig::setup_config(args).unwrap();
    ///
    /// assert_eq!(config.file_path, "graph.txt");
    /// assert!(matches!(config.algorithm, Algorithms::Dijkstra));
    /// assert!(matches!(config.data_input, InputOrigin::File));
    /// ```
    ///
    /// Failed parsing because required nodes are missing:
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::AppConfig;
    ///
    /// let args = vec![
    ///     "pathfinder",
    ///     "--graph-file",
    ///     "graph.txt",
    ///     "--algo",
    ///     "Dijkstra",
    /// ]
    /// .into_iter()
    /// .map(String::from)
    /// .collect();
    ///
    /// let result = AppConfig::setup_config(args);
    /// assert!(result.is_err());
    /// ```
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
        let start_node_id: String = match AppConfig::retrieve_node(&args, true) {
            Some(node) => node,
            None => {
                return Err(SetupProcessError::new(
                    "A start node haven't been specified! ('--start A')".to_string(),
                ));
            }
        };

        let end_node_id: String = match AppConfig::retrieve_node(&args, false) {
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
            start_node_id,
            end_node_id,
        })
    }

    /// Extracts the graph file path from CLI arguments.
    ///
    /// # Parameters
    ///
    /// - `args`: immutable argument vector slice.
    ///
    /// # Returns
    ///
    /// - Value after `--graph-file` if present and non-empty.
    /// - `"graph.txt"` fallback if no valid flag/value pair is present.
    ///
    /// # Panics
    ///
    /// Panics if `--graph-file` appears as the last argument without a value.
    fn retrieve_file_path(args: &[String]) -> String {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--graph-file" && !args[i + 1].is_empty() {
                return args[i + 1].clone();
            }
        }
        "graph.txt".to_string()
    }

    /// Extracts either the start or end node identifier from CLI arguments.
    ///
    /// # Parameters
    ///
    /// - `args`: immutable argument vector slice.
    /// - `is_start_node_requested`: `true` for `--start`, `false` for `--end`.
    ///
    /// # Returns
    ///
    /// - `Some(node_id)` when the requested flag exists and contains a value.
    /// - `None` when the flag is missing or value is empty.
    ///
    /// # Panics
    ///
    /// Panics if the requested flag appears as the last argument without a value.
    fn retrieve_node(args: &[String], is_start_node_requested: bool) -> Option<String> {
        let flag = if is_start_node_requested {
            "--start"
        } else {
            "--end"
        };
        for (i, arg) in args.iter().enumerate() {
            if arg == flag && !args[i + 1].is_empty() {
                return Some(args[i + 1].clone());
            }
        }
        None
    }

    /// Extracts algorithm selection from CLI arguments.
    ///
    /// # Parameters
    ///
    /// - `args`: immutable argument vector slice.
    ///
    /// # Returns
    ///
    /// - Parsed algorithm from `--algo`.
    /// - [`Algorithms::Dijkstra`] fallback if no valid `--algo` value exists.
    ///
    /// # Panics
    ///
    /// Panics if `--algo` appears as the last argument without a value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::algorithm::Algorithms;
    /// use shortest_path_finder::cmd_line::app_config::AppConfig;
    ///
    /// let args = vec![
    ///     "pathfinder",
    ///     "--start",
    ///     "A",
    ///     "--end",
    ///     "B",
    ///     "--algo",
    ///     "AStar",
    /// ]
    /// .into_iter()
    /// .map(String::from)
    /// .collect();
    ///
    /// let config = AppConfig::setup_config(args).unwrap();
    /// assert!(matches!(config.algorithm, Algorithms::AStar));
    /// ```
    fn retrieve_algorithm(args: &[String]) -> Algorithms {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--algo" && !args[i + 1].is_empty() {
                return Algorithms::get_from_string(&args[i + 1]);
            }
        }
        Algorithms::Dijkstra
    }

    /// Extracts input-origin selection from CLI arguments.
    ///
    /// # Compatibility behavior
    ///
    /// This method currently looks at `--algo` instead of `--origin`.
    /// The behavior is intentionally preserved to avoid changing runtime logic.
    ///
    /// # Parameters
    ///
    /// - `args`: immutable argument vector slice.
    ///
    /// # Returns
    ///
    /// Parsed [`InputOrigin`] from the value behind `--algo`, or
    /// [`InputOrigin::File`] as fallback.
    ///
    /// # Panics
    ///
    /// Panics if `--algo` appears as the last argument without a value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::{AppConfig, InputOrigin};
    ///
    /// let args = vec![
    ///     "pathfinder",
    ///     "--start",
    ///     "A",
    ///     "--end",
    ///     "B",
    ///     "--algo",
    ///     "cmd-line",
    /// ]
    /// .into_iter()
    /// .map(String::from)
    /// .collect();
    ///
    /// let config = AppConfig::setup_config(args).unwrap();
    /// assert!(matches!(config.data_input, InputOrigin::CommandLine));
    /// ```
    fn retrieve_data_input(args: &[String]) -> InputOrigin {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--algo" && !args[i + 1].is_empty() {
                return InputOrigin::get_from_string(&args[i + 1]);
            }
        }
        InputOrigin::File
    }
}

/// Error returned when setup cannot build a valid [`AppConfig`].
///
/// # Fields
///
/// - `message`: human-readable explanation of the setup failure.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::cmd_line::app_config::SetupProcessError;
///
/// let err = SetupProcessError::new("missing --start".to_string());
/// assert_eq!(err.message, "missing --start");
/// ```
#[derive(Debug)]
pub struct SetupProcessError {
    /// Detailed error description for display/logging.
    pub message: String,
}

impl SetupProcessError {
    /// Constructs a new [`SetupProcessError`] from a text message.
    ///
    /// # Parameters
    ///
    /// - `message`: detailed setup failure explanation.
    ///
    /// # Returns
    ///
    /// A new [`SetupProcessError`] value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::SetupProcessError;
    ///
    /// let error = SetupProcessError::new("invalid CLI arguments".to_string());
    /// assert_eq!(error.message, "invalid CLI arguments");
    /// ```
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
