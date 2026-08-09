//! CLI error wrapper for the Pathfinder application.
//!
//! # Overview
//!
//! [`AppError`] is the top-level error type used by the `pathfinder` binary.
//! It unifies configuration, graph-loading, and algorithm failures behind one
//! [`std::fmt::Display`] implementation and a stable [`AppError::exit_code`]
//! mapping consumed by the `pathfinder` binary entrypoint.
//!
//! # Error Hierarchy
//!
//! ```text
//! AppError
//! ├── Config(CLIParseError)   — CLI flag parsing and validation
//! ├── Input(DataInputError)      — file I/O and graph parse failures
//! ├── Algorithm(AlgorithmError)  — shortest-path execution failures
//! ├── UnsupportedInputOrigin     — parsed origin not implemented in the runtime
//! └── Runtime                    — generic application-level failures
//! ```
//!
//! # Exit Codes
//!
//! | Variant | Exit code | Notes |
//! |---------|-----------|-------|
//! | [`AppError::Algorithm`] | [`crate::error::algorithm_error::AlgorithmErrorKind::exit_code`] | Per-kind algorithm mapping (2–8) |
//! | All other variants | `1` | Setup, parsing, or loading failures |
//!
//! # Examples
//!
//! Converting a configuration error:
//!
//! ```rust
//! use shortest_path_finder::error::app_error::AppError;
//! use shortest_path_finder::error::CLIParseError;
//!
//! let err = AppError::from(CLIParseError::MissingRequiredFlag { flag: "--start" });
//! assert_eq!(err.exit_code(), 1);
//! assert!(err.to_string().contains("--start"));
//! ```
//!
//! Converting a file-input error:
//!
//! ```rust
//! use shortest_path_finder::data_input::file::FileInputError;
//! use shortest_path_finder::error::app_error::AppError;
//! use shortest_path_finder::error::data_input_error::DataInputError;
//! use shortest_path_finder::error::parse_error::ParseError;
//!
//! let file_err = FileInputError::Parse {
//!     file_path: "graph.txt".to_string(),
//!     source: ParseError::InvalidLineSyntax,
//! };
//! let err = AppError::from(DataInputError::File(file_err));
//! assert_eq!(err.exit_code(), 1);
//! assert!(err.to_string().contains("Input error"));
//! ```
//!
//! Mapping an algorithm failure to a non-default exit code:
//!
//! ```rust
//! use shortest_path_finder::algorithms::dijkstra::DijkstraError;
//! use shortest_path_finder::error::algorithm_error::{AlgorithmError, AlgorithmErrorKind};
//! use shortest_path_finder::error::app_error::AppError;
//!
//! let err = AppError::from(AlgorithmError::from(DijkstraError::NoPathFound {
//!     start: "A".to_string(),
//!     end: "B".to_string(),
//! }));
//! assert_eq!(err.exit_code(), AlgorithmErrorKind::NoPath.exit_code());
//! ```

use std::{error::Error, fmt};

use crate::error::{
    CLIParseError, algorithm_error::AlgorithmError, data_input_error::DataInputError,
};

/// Unified CLI error for the Pathfinder binary.
///
/// # Variants
///
/// - [`AppError::Config`]: invalid or incomplete CLI arguments.
/// - [`AppError::Input`]: graph file could not be read or parsed.
/// - [`AppError::Algorithm`]: shortest-path search failed at runtime.
/// - [`AppError::UnsupportedInputOrigin`]: origin was parsed but is not wired up.
/// - [`AppError::Runtime`]: catch-all for application logic failures.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::app_error::AppError;
///
/// let err = AppError::UnsupportedInputOrigin {
///     origin: "cmd-line".to_string(),
/// };
/// assert!(err.to_string().contains("not supported"));
/// ```
#[derive(Debug)]
pub enum AppError {
    /// Configuration parsing or validation failed.
    Config(CLIParseError),
    /// File input or parse failure while loading graph data.
    Input(DataInputError),
    /// Shortest-path algorithm execution failed.
    Algorithm(AlgorithmError),
    /// Input origin is not supported by the CLI runtime yet.
    UnsupportedInputOrigin {
        /// Canonical origin string that was requested (for example `cmd-line`).
        origin: String,
    },
    /// Generic runtime error not covered by other variants.
    Runtime {
        /// Human-readable explanation of the failure.
        message: String,
    },
}

impl AppError {
    /// Returns the process exit code associated with this error.
    ///
    /// Algorithm failures delegate to [`AlgorithmError::kind`] and
    /// [`crate::error::algorithm_error::AlgorithmErrorKind::exit_code`].
    /// All other variants map to `1`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::error::app_error::AppError;
    /// use shortest_path_finder::error::CLIParseError;
    ///
    /// let err = AppError::Config(CLIParseError::MissingRequiredFlag { flag: "--end" });
    /// assert_eq!(err.exit_code(), 1);
    /// ```
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

impl From<CLIParseError> for AppError {
    /// Wraps a CLI configuration error as [`AppError::Config`].
    fn from(err: CLIParseError) -> Self {
        Self::Config(err)
    }
}

impl From<DataInputError> for AppError {
    /// Wraps a data-input error as [`AppError::Input`].
    fn from(err: DataInputError) -> Self {
        Self::Input(err)
    }
}

impl From<AlgorithmError> for AppError {
    /// Wraps an algorithm execution error as [`AppError::Algorithm`].
    fn from(err: AlgorithmError) -> Self {
        Self::Algorithm(err)
    }
}
