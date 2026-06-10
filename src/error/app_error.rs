//! CLI error wrapper for the Pathfinder application.
//!
//! This module defines [`AppError`], a single error type that wraps
//! configuration, input, and algorithm execution failures and provides
//! stable exit code mapping for the CLI.

use std::{error::Error, fmt};

use crate::error::{
    algorithm_error::AlgorithmError, config_error::ConfigParseError,
    data_input_error::DataInputError,
};

/// Unified CLI error for the Pathfinder binary.
#[derive(Debug)]
pub enum AppError {
    /// Configuration parsing or validation failed.
    Config(ConfigParseError),
    /// File input or parse failure while loading graph data or CLI input parsing failure.
    Input(DataInputError),
    /// Shortest-path algorithm execution failed.
    Algorithm(AlgorithmError),
    /// Input origin is not supported by the CLI runtime yet.
    UnsupportedInputOrigin { origin: String },
    /// Generic runtime error not covered by other variants.
    Runtime { message: String },
}

impl AppError {
    /// Returns the exit code associated with this error.
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::Algorithm(err) => err.kind().exit_code(),
            AppError::Config(_)
            | AppError::Input(_)
            | AppError::UnsupportedInputOrigin { .. }
            | AppError::Runtime { .. } => 1,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(err) => write!(f, "Configuration error: {}", err),
            AppError::Input(err) => write!(f, "Input error: {}", err),
            AppError::Algorithm(err) => write!(f, "{}", err),
            AppError::UnsupportedInputOrigin { origin } => write!(
                f,
                "Input origin '{}' is not supported in the CLI runtime yet",
                origin
            ),
            AppError::Runtime { message } => write!(f, "{}", message),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Config(err) => Some(err),
            AppError::Input(err) => Some(err),
            AppError::Algorithm(err) => Some(err),
            AppError::UnsupportedInputOrigin { .. } => None,
            AppError::Runtime { .. } => None,
        }
    }
}

impl From<ConfigParseError> for AppError {
    fn from(err: ConfigParseError) -> Self {
        Self::Config(err)
    }
}

impl From<DataInputError> for AppError {
    fn from(err: DataInputError) -> Self {
        Self::Input(err)
    }
}

impl From<AlgorithmError> for AppError {
    fn from(err: AlgorithmError) -> Self {
        Self::Algorithm(err)
    }
}
