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
//! - Input-origin parsing primarily reads from `--origin`.
//! - Compatibility fallback: if `--origin` is absent, parser also accepts legacy
//!   origin values from `--algo` (`file` or `cmd-line`).
//! - Unknown flags, duplicate flags, missing values, and unexpected tokens are
//!   rejected with structured [`ConfigParseError`] values.
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

use crate::{algorithms::algorithm::Algorithms, error::config_error::ConfigParseError};

/// Minimum argument count required before parsing is attempted.
///
/// This guard prevents obviously incomplete invocations from entering detailed
/// flag parsing logic.
const MIN_ARGUMENT_COUNT: usize = 4;

/// Default file path used when `--graph-file` is not provided.
const DEFAULT_GRAPH_FILE: &str = "graph.txt";

/// Internal representation of supported CLI flags.
///
/// This enum centralizes known flags so parser logic can map raw tokens to a
/// fixed set of configuration slots and produce structured errors for unknown
/// switches.
#[derive(Copy, Clone, Debug)]
enum KnownFlag {
    GraphFile,
    Start,
    End,
    Algo,
    Origin,
}

impl KnownFlag {
    /// Converts a raw CLI token to a known flag discriminator.
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "--graph-file" => Some(Self::GraphFile),
            "--start" => Some(Self::Start),
            "--end" => Some(Self::End),
            "--algo" => Some(Self::Algo),
            "--origin" => Some(Self::Origin),
            _ => None,
        }
    }

    /// Returns the canonical string spelling for a known flag.
    fn as_str(self) -> &'static str {
        match self {
            Self::GraphFile => "--graph-file",
            Self::Start => "--start",
            Self::End => "--end",
            Self::Algo => "--algo",
            Self::Origin => "--origin",
        }
    }
}

/// Parsed key-value storage for all supported CLI options.
///
/// Each field stores the original flag index and its associated value, which
/// enables precise duplicate-flag diagnostics.
#[derive(Default, Debug)]
struct ParsedCliValues {
    graph_file: Option<(usize, String)>,
    start: Option<(usize, String)>,
    end: Option<(usize, String)>,
    algo: Option<(usize, String)>,
    origin: Option<(usize, String)>,
}

impl ParsedCliValues {
    /// Stores one parsed flag value and rejects duplicate occurrences.
    ///
    /// # Parameters
    ///
    /// - `slot`: Target storage location for a flag value.
    /// - `flag`: Logical flag identifier used for diagnostics.
    /// - `index`: Position of the current flag token in the original args.
    /// - `value`: Parsed value token associated with `flag`.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigParseError::DuplicateFlag`] when `slot` is already set.
    fn set_value(
        slot: &mut Option<(usize, String)>,
        flag: KnownFlag,
        index: usize,
        value: &str,
    ) -> Result<(), ConfigParseError> {
        if let Some((first_index, _)) = slot {
            return Err(ConfigParseError::DuplicateFlag {
                flag: flag.as_str().to_string(),
                first_index: *first_index,
                duplicate_index: index,
            });
        }

        *slot = Some((index, value.to_string()));
        Ok(())
    }

    /// Routes one parsed flag/value pair into the corresponding storage slot.
    ///
    /// # Parameters
    ///
    /// - `flag`: Known flag discriminator.
    /// - `index`: Index of the flag token in `args`.
    /// - `value`: Associated value token.
    ///
    /// # Errors
    ///
    /// Propagates duplicate-flag errors from [`ParsedCliValues::set_value`].
    fn insert(
        &mut self,
        flag: KnownFlag,
        index: usize,
        value: &str,
    ) -> Result<(), ConfigParseError> {
        match flag {
            KnownFlag::GraphFile => Self::set_value(&mut self.graph_file, flag, index, value),
            KnownFlag::Start => Self::set_value(&mut self.start, flag, index, value),
            KnownFlag::End => Self::set_value(&mut self.end, flag, index, value),
            KnownFlag::Algo => Self::set_value(&mut self.algo, flag, index, value),
            KnownFlag::Origin => Self::set_value(&mut self.origin, flag, index, value),
        }
    }

    /// Returns the parsed `--graph-file` value, if provided.
    fn graph_file_value(&self) -> Option<String> {
        self.graph_file.as_ref().map(|(_, value)| value.clone())
    }

    /// Returns the parsed `--start` value, if provided.
    fn start_value(&self) -> Option<String> {
        self.start.as_ref().map(|(_, value)| value.clone())
    }

    /// Returns the parsed `--end` value, if provided.
    fn end_value(&self) -> Option<String> {
        self.end.as_ref().map(|(_, value)| value.clone())
    }

    /// Returns the parsed `--algo` value, if provided.
    fn algorithm_value(&self) -> Option<String> {
        self.algo.as_ref().map(|(_, value)| value.clone())
    }

    /// Returns the parsed `--origin` value, if provided.
    fn origin_value(&self) -> Option<String> {
        self.origin.as_ref().map(|(_, value)| value.clone())
    }
}

/// Parses raw CLI arguments into validated key-value pairs.
///
/// # Behavior
///
/// - Accepts argument vectors both with and without executable name prefix.
/// - Requires every option token to start with `--`.
/// - Requires every known flag to be followed by a non-empty, non-flag value.
/// - Rejects unknown and duplicate flags.
///
/// # Errors
///
/// Returns:
/// - [`ConfigParseError::UnexpectedArgument`] for non-flag tokens,
/// - [`ConfigParseError::UnknownFlag`] for unsupported switches,
/// - [`ConfigParseError::MissingValueForFlag`] when a flag has no usable value,
/// - [`ConfigParseError::DuplicateFlag`] when a known flag appears multiple times.
fn parse_cli_values(args: &[String]) -> Result<ParsedCliValues, ConfigParseError> {
    let mut parsed = ParsedCliValues::default();
    // Allow both `["--start", "A", ...]` and `["pathfinder", "--start", "A", ...]` forms.
    let mut index = if args.first().is_some_and(|value| value.starts_with("--")) {
        0
    } else {
        1
    };

    // Process tokens in pairs: flag followed by value.
    while index < args.len() {
        let token = &args[index];

        // Validate that the current token is a flag.
        if !token.starts_with("--") {
            return Err(ConfigParseError::UnexpectedArgument {
                value: token.clone(),
                index,
            });
        }

        let flag = match KnownFlag::from_token(token) {
            Some(flag) => flag,
            None => {
                return Err(ConfigParseError::UnknownFlag {
                    flag: token.clone(),
                    index,
                });
            }
        };

        // Validate that the flag is followed by a usable value.
        let maybe_value = args.get(index + 1);
        let value = match maybe_value {
            Some(value) if !value.is_empty() && !value.starts_with("--") => value,
            _ => {
                return Err(ConfigParseError::MissingValueForFlag {
                    flag: flag.as_str().to_string(),
                    index,
                });
            }
        };

        parsed.insert(flag, index, value)?;
        // Advance by one full pair (`--flag` + `value`).
        index += 2;
    }

    Ok(parsed)
}

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
/// // Input origin is read from `--origin` when provided.
/// let args = vec![
///     "pathfinder",
///     "--start",
///     "A",
///     "--end",
///     "B",
///     "--origin",
///     "cmd-line",
/// ]
/// .into_iter()
/// .map(String::from)
/// .collect();
///
/// let config = AppConfig::setup_config(args).unwrap();
/// assert!(matches!(config.data_input, InputOrigin::CommandLine));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// - `Err(ConfigParseError)` when parsing or validation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - fewer than four arguments are provided,
    /// - required flags are missing,
    /// - a known flag is missing a value,
    /// - unknown or duplicate flags are provided,
    /// - or unexpected non-flag tokens appear.
    ///
    /// Concrete variant mapping:
    /// - [`ConfigParseError::TooFewArguments`]
    /// - [`ConfigParseError::MissingRequiredFlag`]
    /// - [`ConfigParseError::MissingValueForFlag`]
    /// - [`ConfigParseError::UnknownFlag`]
    /// - [`ConfigParseError::DuplicateFlag`]
    /// - [`ConfigParseError::UnexpectedArgument`]
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
    ///
    /// Failed parsing because of an unknown flag:
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::AppConfig;
    /// use shortest_path_finder::error::config_error::ConfigParseError;
    ///
    /// let args = vec![
    ///     "pathfinder",
    ///     "--whoops",
    ///     "x",
    ///     "--start",
    ///     "A",
    ///     "--end",
    ///     "B",
    /// ]
    /// .into_iter()
    /// .map(String::from)
    /// .collect();
    ///
    /// let err = AppConfig::setup_config(args).expect_err("unknown flag should fail");
    /// assert!(matches!(err, ConfigParseError::UnknownFlag { .. }));
    /// ```
    pub fn setup_config(args: Vec<String>) -> Result<Self, ConfigParseError> {
        if args.len() < MIN_ARGUMENT_COUNT {
            return Err(ConfigParseError::TooFewArguments {
                provided: args.len(),
                minimum: MIN_ARGUMENT_COUNT,
            });
        }

        let parsed = parse_cli_values(&args)?;
        let file_path = parsed
            .graph_file_value()
            .unwrap_or_else(|| DEFAULT_GRAPH_FILE.to_string());
        let algorithm_token = parsed.algorithm_value();
        let algorithm = AppConfig::retrieve_algorithm(algorithm_token.as_deref());
        let data_input = AppConfig::retrieve_data_input(&parsed, algorithm_token.as_deref());

        let start_node_id = parsed
            .start_value()
            .ok_or(ConfigParseError::MissingRequiredFlag { flag: "--start" })?;
        let end_node_id = parsed
            .end_value()
            .ok_or(ConfigParseError::MissingRequiredFlag { flag: "--end" })?;

        Ok(Self {
            file_path,
            start_node_id,
            end_node_id,
            algorithm,
            data_input,
        })
    }

    /// Converts optional algorithm text into a concrete [`Algorithms`] value.
    ///
    /// Falls back to [`Algorithms::Dijkstra`] when the algorithm flag is not
    /// provided.
    fn retrieve_algorithm(raw_algorithm: Option<&str>) -> Algorithms {
        raw_algorithm
            .map(Algorithms::get_from_string)
            .unwrap_or(Algorithms::Dijkstra)
    }

    /// Resolves input origin with compatibility fallback.
    ///
    /// Resolution order:
    /// 1. `--origin` value,
    /// 2. legacy `--algo` values `file`/`cmd-line`,
    /// 3. [`InputOrigin::File`] default.
    fn retrieve_data_input(parsed: &ParsedCliValues, raw_algorithm: Option<&str>) -> InputOrigin {
        if let Some(origin) = parsed.origin_value() {
            return InputOrigin::get_from_string(&origin);
        }

        // Keep backward compatibility for existing callers that pass
        // `--algo cmd-line` or `--algo file` as origin markers.
        if let Some(algo_token) = raw_algorithm {
            return InputOrigin::get_from_string(algo_token);
        }

        InputOrigin::File
    }
}

/// Compatibility alias for previous naming in benches/docs.
///
/// New code should use [`ConfigParseError`] directly.
pub type SetupProcessError = ConfigParseError;
