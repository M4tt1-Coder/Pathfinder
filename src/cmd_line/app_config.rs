//! Command-line configuration parsing for the `pathfinder` binary.
//!
//! This module turns raw CLI arguments into strongly typed runtime configuration.
//! The main entry point is [`AppConfig::setup_config`], which validates arguments,
//! applies defaults, and returns an [`AppConfigOutcome`] used by the application runtime.
//!
//! # Supported flags
//!
//! - `--graph-file <relative_path_to_file>`: file used to build the graph.
//! - `--start <node_name>`: start node identifier (required).
//! - `--end <node_name>`: destination node identifier (required).
//! - `--algo <algorithm_name>`: algorithm selector (defaults to `Dijkstra`).
//! - `--origin <file|cmd-line>`: intended input-origin selector.
//! - `--help`/`-h`: print usage information and exit.
//! - `--version`/`-V`: print version information and exit.
//!
//! # Defaults and compatibility notes
//!
//! - Missing `--graph-file` defaults to `graph.txt`.
//! - Missing `--algo` defaults to `Dijkstra`.
//! - Unknown `--algo` values are rejected unless used for legacy origin fallback.
//! - Input-origin parsing primarily reads from `--origin`.
//! - Compatibility fallback: if `--origin` is absent, parser also accepts legacy
//!   origin values from `--algo` (`file` or `cmd-line`).
//! - Use `--` between a flag and its value to allow values that start with `--`.
//! - Unknown flags, duplicate flags, missing values, and unexpected tokens are
//!   rejected with structured [`ConfigParseError`] values.
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::algorithms::Algorithms;
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
//! let config = AppConfig::setup_config(args)
//!     .unwrap()
//!     .into_config()
//!     .expect("expected config");
//!
//! assert_eq!(config.file_path, "test_files/directed_graph.txt");
//! assert_eq!(config.start_node_id, "A");
//! assert_eq!(config.end_node_id, "D");
//! assert!(matches!(config.algorithm, Algorithms::Dijkstra));
//! assert!(matches!(config.data_input, InputOrigin::File));
//! ```

use std::{error::Error as StdError, fmt};

use crate::{algorithms::Algorithms, error::config_error::ConfigParseError};

/// Default file path used when `--graph-file` is not provided.
const DEFAULT_GRAPH_FILE: &str = "graph.txt";

/// Application name used in CLI output.
const APP_NAME: &str = "pathfinder";

/// End-of-options marker used to allow values starting with `--`.
const END_OF_OPTIONS: &str = "--";

/// Allowed origin values for `--origin`.
const VALID_ORIGINS: [&str; 2] = ["file", "cmd-line"];

/// Allowed algorithm values for `--algo`.
const VALID_ALGORITHMS: [&str; 2] = ["Dijkstra", "AStar"];

/// Internal representation of supported CLI flags.
///
/// This enum centralizes known flags so parser logic can map raw tokens to a
/// fixed set of configuration slots and produce structured errors for unknown
/// switches.
#[derive(Copy, Clone, Debug)]
enum KnownFlag {
    /// `--graph-file` flag.
    GraphFile,
    /// `--start` flag.
    Start,
    /// `--end` flag.
    End,
    /// `--algo` flag.
    Algo,
    /// `--origin` flag.
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

/// Returns true when the token is a recognized help flag.
fn is_help_flag(token: &str) -> bool {
    matches!(token, "--help" | "-h")
}

/// Returns true when the token is a recognized version flag.
fn is_version_flag(token: &str) -> bool {
    matches!(token, "--version" | "-V")
}

/// Formats a list of expected values for diagnostic messages.
fn expected_values(values: &[&str]) -> String {
    values.join(" | ")
}

/// Parsed key-value storage for all supported CLI options.
///
/// Each field stores the 1-based flag position and its associated value, which
/// enables precise duplicate-flag diagnostics.
#[derive(Default, Debug)]
struct ParsedCliValues {
    /// Parsed `--graph-file` value and 1-based flag position.
    graph_file: Option<(usize, String)>,
    /// Parsed `--start` value and 1-based flag position.
    start: Option<(usize, String)>,
    /// Parsed `--end` value and 1-based flag position.
    end: Option<(usize, String)>,
    /// Parsed `--algo` value and 1-based flag position.
    algo: Option<(usize, String)>,
    /// Parsed `--origin` value and 1-based flag position.
    origin: Option<(usize, String)>,
}

impl ParsedCliValues {
    /// Stores one parsed flag value and rejects duplicate occurrences.
    ///
    /// # Parameters
    ///
    /// - `slot`: Target storage location for a flag value.
    /// - `flag`: Logical flag identifier used for diagnostics.
    /// - `index`: 1-based position of the current flag token in the original args.
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
    /// - `index`: 1-based position of the flag token in `args`.
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

/// Result of parsing raw CLI arguments before full validation.
#[derive(Debug)]
enum CliParseOutcome {
    /// Parsed key-value storage for the supported CLI flags.
    Values(ParsedCliValues),
    /// Caller requested help output via `--help`/`-h`.
    HelpRequested,
    /// Caller requested version output via `--version`/`-V`.
    VersionRequested,
}

/// Parses raw CLI arguments into validated key-value pairs.
///
/// # Behavior
///
/// - Accepts argument vectors both with and without executable name prefix.
/// - Requires every option token to start with `--`.
/// - Requires every known flag to be followed by a non-empty value.
/// - Accepts `--` between a flag and its value to allow values that start with `--`.
/// - Rejects unknown and duplicate flags.
/// - Short-circuits for `--help`/`--version` requests.
///
/// # Errors
///
/// Returns:
/// - [`ConfigParseError::UnexpectedArgument`] for non-flag tokens,
/// - [`ConfigParseError::UnknownFlag`] for unsupported switches,
/// - [`ConfigParseError::MissingValueForFlag`] when a flag has no usable value,
/// - [`ConfigParseError::DuplicateFlag`] when a known flag appears multiple times,
/// - [`ConfigParseError::UnexpectedEndOfOptions`] when `--` appears where a flag is expected.
fn parse_cli_values(args: &[String]) -> Result<CliParseOutcome, ConfigParseError> {
    let mut parsed = ParsedCliValues::default();
    // Allow both `["--start", "A", ...]` and `["pathfinder", "--start", "A", ...]` forms.
    // Skip argv[0] when it looks like the executable name.
    let mut index = if args.first().is_some_and(|value| value.starts_with("--")) {
        0
    } else {
        1
    };

    // Process tokens in pairs: flag followed by value.
    while index < args.len() {
        let token = &args[index];
        if is_help_flag(token) {
            return Ok(CliParseOutcome::HelpRequested);
        }
        if is_version_flag(token) {
            return Ok(CliParseOutcome::VersionRequested);
        }

        // Use 1-based positions to match user-facing error reporting.
        let display_index = index + 1;

        // A stray `--` is invalid outside a value-escape sequence.
        if token == END_OF_OPTIONS {
            return Err(ConfigParseError::UnexpectedEndOfOptions {
                index: display_index,
            });
        }

        // Validate that the current token is a flag.
        if !token.starts_with("--") {
            return Err(ConfigParseError::UnexpectedArgument {
                value: token.clone(),
                index: display_index,
            });
        }

        let flag = match KnownFlag::from_token(token) {
            Some(flag) => flag,
            None => {
                return Err(ConfigParseError::UnknownFlag {
                    flag: token.clone(),
                    index: display_index,
                });
            }
        };

        // Validate that the flag is followed by a usable value.
        let maybe_value = args.get(index + 1);
        let (value, next_index) = match maybe_value {
            Some(value) if value == END_OF_OPTIONS => {
                // Support `--flag -- --value` to accept values that start with `--`.
                let escaped_index = index + 2;
                let escaped_value = args.get(escaped_index);
                let escaped_value = match escaped_value {
                    Some(value) if !value.is_empty() => value,
                    _ => {
                        return Err(ConfigParseError::MissingValueForFlag {
                            flag: flag.as_str().to_string(),
                            index: display_index,
                        });
                    }
                };
                (escaped_value, escaped_index + 1)
            }
            Some(value) if !value.is_empty() && !value.starts_with("--") => (value, index + 2),
            _ => {
                return Err(ConfigParseError::MissingValueForFlag {
                    flag: flag.as_str().to_string(),
                    index: display_index,
                });
            }
        };

        parsed.insert(flag, display_index, value)?;
        // Advance by one full pair (`--flag` + `value`), or escape value if used.
        index = next_index;
    }

    Ok(CliParseOutcome::Values(parsed))
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
/// let config = AppConfig::setup_config(args)
///     .unwrap()
///     .into_config()
///     .expect("expected config");
/// assert!(matches!(config.data_input, InputOrigin::CommandLine));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputOrigin {
    /// Use file-based data input.
    File,
    /// Use interactive command-line data input.
    CommandLine,
}

/// Error returned when parsing an [`InputOrigin`] value from user input.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::cmd_line::app_config::InputOrigin;
///
/// let err = InputOrigin::try_from("nope").expect_err("invalid origin should fail");
/// assert_eq!(err.value, "nope");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputOriginParseError {
    /// Raw input that failed to match a known origin value.
    pub value: String,
}

impl fmt::Display for InputOriginParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown input origin '{}'", self.value)
    }
}

impl StdError for InputOriginParseError {}

impl TryFrom<&str> for InputOrigin {
    type Error = InputOriginParseError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        match src {
            "file" => Ok(Self::File),
            "cmd-line" => Ok(Self::CommandLine),
            _ => Err(InputOriginParseError {
                value: src.to_string(),
            }),
        }
    }
}

impl InputOrigin {
    /// Returns the canonical string representation for the origin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::InputOrigin;
    ///
    /// assert_eq!(InputOrigin::File.as_str(), "file");
    /// assert_eq!(InputOrigin::CommandLine.as_str(), "cmd-line");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            InputOrigin::File => "file",
            InputOrigin::CommandLine => "cmd-line",
        }
    }
}

impl fmt::Display for InputOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
/// use shortest_path_finder::algorithms::Algorithms;
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
/// let config = AppConfig::setup_config(args)
///     .unwrap()
///     .into_config()
///     .expect("expected config");
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

/// Outcome of CLI configuration parsing.
#[derive(Debug)]
pub enum AppConfigOutcome {
    /// Parsed configuration for normal runtime execution.
    Config(AppConfig),
    /// Help flag was requested; caller should print help and exit successfully.
    HelpRequested,
    /// Version flag was requested; caller should print version and exit successfully.
    VersionRequested,
}

impl AppConfigOutcome {
    /// Extracts the parsed configuration when available.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::AppConfigOutcome;
    ///
    /// let outcome = AppConfigOutcome::HelpRequested;
    /// assert!(outcome.into_config().is_none());
    /// ```
    pub fn into_config(self) -> Option<AppConfig> {
        match self {
            AppConfigOutcome::Config(config) => Some(config),
            AppConfigOutcome::HelpRequested | AppConfigOutcome::VersionRequested => None,
        }
    }
}

impl AppConfig {
    /// Builds an [`AppConfigOutcome`] by parsing and validating CLI arguments.
    ///
    /// # Parameters
    ///
    /// - `args`: full argument vector, typically from `std::env::args().collect()`.
    ///
    /// # Returns
    ///
    /// - `Ok(AppConfigOutcome::Config)` when required information is present.
    /// - `Ok(AppConfigOutcome::HelpRequested)` when help flags are present.
    /// - `Ok(AppConfigOutcome::VersionRequested)` when version flags are present.
    /// - `Err(ConfigParseError)` when parsing or validation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - required flags are missing,
    /// - a known flag is missing a value,
    /// - unknown or duplicate flags are provided,
    /// - a known flag is supplied with an invalid value,
    /// - conflicting flags are provided,
    /// - or unexpected non-flag tokens appear.
    ///
    /// Concrete variant mapping:
    /// - [`ConfigParseError::MissingRequiredFlag`]
    /// - [`ConfigParseError::MissingValueForFlag`]
    /// - [`ConfigParseError::UnknownFlag`]
    /// - [`ConfigParseError::DuplicateFlag`]
    /// - [`ConfigParseError::InvalidFlagValue`]
    /// - [`ConfigParseError::ConflictingFlags`]
    /// - [`ConfigParseError::UnexpectedArgument`]
    /// - [`ConfigParseError::UnexpectedEndOfOptions`]
    ///
    /// # Examples
    ///
    /// Successful parsing with defaults:
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::Algorithms;
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
    /// let config = AppConfig::setup_config(args)
    ///     .unwrap()
    ///     .into_config()
    ///     .expect("expected config");
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
    pub fn setup_config(args: Vec<String>) -> Result<AppConfigOutcome, ConfigParseError> {
        let parsed = match parse_cli_values(&args)? {
            CliParseOutcome::Values(values) => values,
            CliParseOutcome::HelpRequested => return Ok(AppConfigOutcome::HelpRequested),
            CliParseOutcome::VersionRequested => return Ok(AppConfigOutcome::VersionRequested),
        };

        let file_path = parsed
            .graph_file_value()
            .unwrap_or_else(|| DEFAULT_GRAPH_FILE.to_string());
        let algorithm_token = parsed.algorithm_value();
        let (data_input, used_legacy_origin) =
            AppConfig::retrieve_data_input(&parsed, algorithm_token.as_deref())?;
        AppConfig::validate_flag_combinations(&parsed, &data_input)?;
        let algorithm =
            AppConfig::retrieve_algorithm(algorithm_token.as_deref(), used_legacy_origin)?;

        let start_node_id = parsed
            .start_value()
            .ok_or(ConfigParseError::MissingRequiredFlag { flag: "--start" })?;
        let end_node_id = parsed
            .end_value()
            .ok_or(ConfigParseError::MissingRequiredFlag { flag: "--end" })?;

        Ok(AppConfigOutcome::Config(Self {
            file_path,
            start_node_id,
            end_node_id,
            algorithm,
            data_input,
        }))
    }

    /// Returns the CLI help text for the Pathfinder binary.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::AppConfig;
    ///
    /// let help = AppConfig::help_text();
    /// assert!(help.contains("Usage:"));
    /// assert!(help.contains("--graph-file"));
    /// ```
    pub fn help_text() -> String {
        format!(
            "Usage: {app} [--origin <file|cmd-line>] [--graph-file <path_to_file>] \
[--algo <algorithm_name>] --start <node> --end <node>\n\n\
Options:\n\
  --graph-file <path>         Graph input file (default: graph.txt)\n\
  --start <node>              Start node identifier (required)\n\
  --end <node>                Destination node identifier (required)\n\
  --algo <name>               Algorithm to run (Dijkstra | AStar)\n\
  --origin <file|cmd-line>    Input origin (default: file)\n\
  --help, -h                  Show this help message and exit\n\
  --version, -V               Show version information and exit\n\n\
Notes:\n\
  Use `--` between a flag and its value to allow values starting with `--`.\n\
  Legacy: when --origin is absent, --algo file|cmd-line sets the input origin.\n\
  Command-line origin is parsed but not implemented in the CLI runtime yet.\n",
            app = APP_NAME
        )
    }

    /// Returns the version banner for the Pathfinder binary.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::cmd_line::app_config::AppConfig;
    ///
    /// let version = AppConfig::version_text();
    /// assert!(version.starts_with("pathfinder "));
    /// ```
    pub fn version_text() -> String {
        format!("{} {}", APP_NAME, env!("CARGO_PKG_VERSION"))
    }

    /// Converts optional algorithm text into a concrete [`Algorithms`] value.
    fn retrieve_algorithm(
        raw_algorithm: Option<&str>,
        used_legacy_origin: bool,
    ) -> Result<Algorithms, ConfigParseError> {
        // Legacy origin markers consume --algo, so default to Dijkstra here.
        if used_legacy_origin || raw_algorithm.is_none() {
            return Ok(Algorithms::Dijkstra);
        }

        let token = raw_algorithm.expect("algorithm token should be present");
        Algorithms::try_from(token).map_err(|err| ConfigParseError::InvalidFlagValue {
            flag: "--algo".to_string(),
            value: err.value,
            expected: expected_values(&VALID_ALGORITHMS),
        })
    }

    /// Resolves input origin with compatibility fallback.
    ///
    /// Resolution order:
    /// 1. `--origin` value,
    /// 2. legacy `--algo` values `file`/`cmd-line`,
    /// 3. [`InputOrigin::File`] default.
    fn retrieve_data_input(
        parsed: &ParsedCliValues,
        raw_algorithm: Option<&str>,
    ) -> Result<(InputOrigin, bool), ConfigParseError> {
        // `--origin` takes precedence over legacy origin markers.
        if let Some(origin) = parsed.origin_value() {
            let origin = InputOrigin::try_from(origin.as_str()).map_err(|err| {
                ConfigParseError::InvalidFlagValue {
                    flag: "--origin".to_string(),
                    value: err.value,
                    expected: expected_values(&VALID_ORIGINS),
                }
            })?;
            return Ok((origin, false));
        }

        // Keep backward compatibility for existing callers that pass
        // `--algo cmd-line` or `--algo file` as origin markers.
        if let Some(algo_token) = raw_algorithm {
            if let Some(origin) = AppConfig::legacy_origin_from_algorithm(algo_token) {
                return Ok((origin, true));
            }
        }

        Ok((InputOrigin::File, false))
    }

    /// Maps legacy `--algo` origin markers to [`InputOrigin`].
    fn legacy_origin_from_algorithm(raw_algorithm: &str) -> Option<InputOrigin> {
        match raw_algorithm {
            "file" => Some(InputOrigin::File),
            "cmd-line" => Some(InputOrigin::CommandLine),
            _ => None,
        }
    }

    /// Validates mutually exclusive flags against the resolved input origin.
    fn validate_flag_combinations(
        parsed: &ParsedCliValues,
        data_input: &InputOrigin,
    ) -> Result<(), ConfigParseError> {
        // Command-line input conflicts with explicit file-path selection.
        if matches!(data_input, InputOrigin::CommandLine) && parsed.graph_file.is_some() {
            return Err(ConfigParseError::ConflictingFlags {
                flag: "--origin".to_string(),
                other: "--graph-file".to_string(),
                reason: "command-line origin cannot be combined with --graph-file".to_string(),
            });
        }

        Ok(())
    }
}

/// Compatibility alias for previous naming in benches/docs.
///
/// New code should use [`ConfigParseError`] directly.
pub type SetupProcessError = ConfigParseError;
