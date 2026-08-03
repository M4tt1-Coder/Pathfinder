//! Internal CLI parser for file-input configuration.
//!
//! # Overview
//!
//! This private module implements the token-by-token parsing used by
//! [`crate::data_input::file::cli_config::AppConfig`]. It keeps the lower-level
//! command-line validation logic separate from the higher-level configuration
//! builder so the public API stays small and the parser can be unit tested in
//! isolation.
//!
//! # Responsibilities
//!
//! - Recognize supported CLI flags and sentinel tokens.
//! - Collect raw flag/value pairs with stable duplicate detection.
//! - Produce structured [`crate::error::config_error::ConfigParseError`] values.
//!
//! # Notes
//!
//! The module is intentionally internal. Public consumers should use
//! [`crate::data_input::file::cli_config::AppConfig`] instead of depending on
//! these helpers directly.

use crate::error::config_error::ConfigParseError;

/// Default file path used when `--graph-file` is not provided.
pub const DEFAULT_GRAPH_FILE: &str = "graph.txt";

/// Application name used in CLI output.
pub const APP_NAME: &str = "pathfinder";

/// End-of-options marker used to allow values starting with `--`.
pub const END_OF_OPTIONS: &str = "--";

/// Allowed origin values for `--origin`.
pub const VALID_ORIGINS: [&str; 2] = ["file", "cmd-line"];

/// Allowed algorithm values for `--algo`.
pub const VALID_ALGORITHMS: [&str; 2] = ["Dijkstra", "AStar"];

/// Internal representation of supported CLI flags.
///
/// This enum centralizes known flags so parser logic can map raw tokens to a
/// fixed set of configuration slots and produce structured errors for unknown
/// switches.
#[derive(Copy, Clone, Debug)]
pub enum KnownFlag {
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
pub fn is_help_flag(token: &str) -> bool {
    matches!(token, "--help" | "-h")
}

/// Returns true when the token is a recognized version flag.
pub fn is_version_flag(token: &str) -> bool {
    matches!(token, "--version" | "-V")
}

/// Formats a list of expected values for diagnostic messages.
pub fn expected_values(values: &[&str]) -> String {
    values.join(" | ")
}

/// Parsed key-value storage for all supported CLI options.
///
/// Each field stores the 1-based flag position and its associated value, which
/// enables precise duplicate-flag diagnostics.
#[derive(Default, Debug)]
pub struct ParsedCliValues {
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

    /// Returns true if the `--graph-file` flag was provided.
    pub fn has_graph_file(&self) -> bool {
        self.graph_file.is_some()
    }

    /// Returns the parsed `--graph-file` value, if provided.
    pub fn graph_file_value(&self) -> Option<String> {
        self.graph_file.as_ref().map(|(_, value)| value.clone())
    }

    /// Returns the parsed `--start` value, if provided.
    pub fn start_value(&self) -> Option<String> {
        self.start.as_ref().map(|(_, value)| value.clone())
    }

    /// Returns the parsed `--end` value, if provided.
    pub fn end_value(&self) -> Option<String> {
        self.end.as_ref().map(|(_, value)| value.clone())
    }

    /// Returns the parsed `--algo` value, if provided.
    pub fn algorithm_value(&self) -> Option<String> {
        self.algo.as_ref().map(|(_, value)| value.clone())
    }

    /// Returns the parsed `--origin` value, if provided.
    pub fn origin_value(&self) -> Option<String> {
        self.origin.as_ref().map(|(_, value)| value.clone())
    }
}

/// Result of parsing raw CLI arguments before full validation.
#[derive(Debug)]
pub enum CliParseOutcome {
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
pub fn parse_cli_values(args: &[String]) -> Result<CliParseOutcome, ConfigParseError> {
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
