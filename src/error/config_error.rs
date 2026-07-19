//! Global error types for command-line configuration parsing.
//!
//! This module defines [`ConfigParseError`], a structured error enum used by
//! CLI argument parsing in [`crate::cmd_line::app_config`].
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::error::config_error::ConfigParseError;
//!
//! let err = ConfigParseError::MissingRequiredFlag { flag: "--start" };
//! assert!(err.to_string().contains("--start"));
//! ```

use std::{error::Error, fmt};

/// Structured errors returned while parsing CLI configuration arguments.
///
/// Argument indices reported in this enum are 1-based positions from the
/// original argument list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigParseError {
    /// Fewer than the minimum expected argument count was supplied.
    TooFewArguments {
        /// Number of arguments actually provided.
        provided: usize,
        /// Minimum number of arguments required by the parser.
        minimum: usize,
    },
    /// A required flag is missing from the argument list.
    MissingRequiredFlag {
        /// Canonical flag name that was not supplied (for example `--start`).
        flag: &'static str,
    },
    /// A flag was provided without a usable value.
    MissingValueForFlag {
        /// Flag name that lacked a value token.
        flag: String,
        /// 1-based position of the flag token in the original argument list.
        index: usize,
    },
    /// The same flag appears more than once.
    DuplicateFlag {
        /// Flag name that was repeated.
        flag: String,
        /// 1-based position of the first occurrence.
        first_index: usize,
        /// 1-based position of the duplicate occurrence.
        duplicate_index: usize,
    },
    /// A token looked like a flag but is not supported.
    UnknownFlag {
        /// Unrecognized flag token (including the leading `--`).
        flag: String,
        /// 1-based position of the unknown flag in the argument list.
        index: usize,
    },
    /// A non-flag token appeared where a flag was expected.
    UnexpectedArgument {
        /// Raw token that was not recognized as a flag.
        value: String,
        /// 1-based position of the unexpected token.
        index: usize,
    },
    /// The end-of-options sentinel (`--`) appeared where a flag was expected.
    UnexpectedEndOfOptions {
        /// 1-based position of the stray `--` token.
        index: usize,
    },
    /// A flag value is not one of the expected options.
    InvalidFlagValue {
        /// Flag name whose value was rejected.
        flag: String,
        /// Value token supplied by the user.
        value: String,
        /// Pipe-separated list of accepted values for diagnostics.
        expected: String,
    },
    /// Mutually exclusive or conflicting flags were provided together.
    ConflictingFlags {
        /// Primary flag involved in the conflict.
        flag: String,
        /// Secondary flag that cannot be combined with `flag`.
        other: String,
        /// Human-readable explanation of why the combination is invalid.
        reason: String,
    },
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigParseError::TooFewArguments { provided, minimum } => write!(
                f,
                "Not enough arguments passed (provided {}, minimum {}).",
                provided, minimum
            ),
            ConfigParseError::MissingRequiredFlag { flag } => {
                write!(f, "Missing required flag {}.", flag)
            }
            ConfigParseError::MissingValueForFlag { flag, index } => write!(
                f,
                "Missing value for flag {} at argument position {}.",
                flag, index
            ),
            ConfigParseError::DuplicateFlag {
                flag,
                first_index,
                duplicate_index,
            } => write!(
                f,
                "Flag {} was provided more than once (first at position {}, duplicate at position {}).",
                flag, first_index, duplicate_index
            ),
            ConfigParseError::UnknownFlag { flag, index } => {
                write!(f, "Unknown flag {} at argument position {}.", flag, index)
            }
            ConfigParseError::UnexpectedArgument { value, index } => write!(
                f,
                "Unexpected argument '{}' at position {}. Flags must start with '--'.",
                value, index
            ),
            ConfigParseError::UnexpectedEndOfOptions { index } => write!(
                f,
                "Unexpected end-of-options marker '--' at position {}.",
                index
            ),
            ConfigParseError::InvalidFlagValue {
                flag,
                value,
                expected,
            } => write!(
                f,
                "Invalid value '{}' for flag {} (expected {}).",
                value, flag, expected
            ),
            ConfigParseError::ConflictingFlags {
                flag,
                other,
                reason,
            } => write!(f, "Conflicting flags {} and {}: {}.", flag, other, reason),
        }
    }
}

impl Error for ConfigParseError {}
