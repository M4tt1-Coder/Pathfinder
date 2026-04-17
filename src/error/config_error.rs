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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigParseError {
    /// Fewer than the minimum expected argument count was supplied.
    TooFewArguments { provided: usize, minimum: usize },
    /// A required flag is missing from the argument list.
    MissingRequiredFlag { flag: &'static str },
    /// A flag was provided without a usable value.
    MissingValueForFlag { flag: String, index: usize },
    /// The same flag appears more than once.
    DuplicateFlag {
        flag: String,
        first_index: usize,
        duplicate_index: usize,
    },
    /// A token looked like a flag but is not supported.
    UnknownFlag { flag: String, index: usize },
    /// A non-flag token appeared where a flag was expected.
    UnexpectedArgument { value: String, index: usize },
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
                "Missing value for flag {} at argument index {}.",
                flag, index
            ),
            ConfigParseError::DuplicateFlag {
                flag,
                first_index,
                duplicate_index,
            } => write!(
                f,
                "Flag {} was provided more than once (first at index {}, duplicate at index {}).",
                flag, first_index, duplicate_index
            ),
            ConfigParseError::UnknownFlag { flag, index } => {
                write!(f, "Unknown flag {} at argument index {}.", flag, index)
            }
            ConfigParseError::UnexpectedArgument { value, index } => write!(
                f,
                "Unexpected argument '{}' at index {}. Flags must start with '--'.",
                value, index
            ),
        }
    }
}

impl Error for ConfigParseError {}
