//! Error types used across parsing, configuration, and algorithm execution.
//!
//! # Overview
//!
//! The crate exposes layered error types so each boundary can fail with precise
//! context while the CLI can still collapse failures into [`AppError`]:
//!
//! - [`parse_error`]: low-level graph text parsing failures.
//! - [`data_input_error`]: unified graph-loading failures (file input today).
//! - [`cli_parse_error`]: CLI argument and configuration parsing failures.
//! - [`algorithm_error`]: algorithm execution and path reconstruction failures.
//! - [`app_error`]: top-level CLI wrapper with exit-code mapping.
//!
//! # Module Map
//!
//! | Module | Primary type | Responsibility |
//! |--------|--------------|----------------|
//! | [`parse_error`] | [`parse_error::ParseError`] | Line-level graph syntax validation |
//! | [`data_input_error`] | [`data_input_error::DataInputError`] | File/graph loading boundary |
//! | [`cli_parse_error`] | [`CLIParseError`] | CLI flag parsing |
//! | [`algorithm_error`] | [`algorithm_error::AlgorithmError`] | Shortest-path runtime failures |
//! | [`app_error`] | [`AppError`] | Binary exit codes and user messages |
//!
//! # Examples
//!
//! Classifying an algorithm failure for exit codes:
//!
//! ```rust
//! use shortest_path_finder::error::algorithm_error::AlgorithmErrorKind;
//!
//! assert_eq!(AlgorithmErrorKind::NoPath.exit_code(), 6);
//! ```
//!
//! Handling a parse-time graph syntax error:
//!
//! ```rust
//! use shortest_path_finder::error::parse_error::ParseError;
//!
//! let err = ParseError::InvalidLineSyntax;
//! assert!(err.to_string().contains("Invalid syntax"));
//! ```
//!
//! Wrapping a file-input failure for the CLI:
//!
//! ```rust
//! use shortest_path_finder::data_input::file::FileInputError;
//! use shortest_path_finder::error::app_error::AppError;
//! use shortest_path_finder::error::data_input_error::DataInputError;
//! use shortest_path_finder::error::parse_error::ParseError;
//!
//! let err = AppError::from(DataInputError::File(FileInputError::Parse {
//!     file_path: "graph.txt".to_string(),
//!     source: ParseError::InvalidDataInput("bad header".to_string()),
//! }));
//! assert_eq!(err.exit_code(), 1);
//! ```

mod cli_parse_error;

// ~ flatten module paths ~

pub use app_error::AppError;
pub use cli_parse_error::CLIParseError;

// ~ public modules ~

pub mod algorithm_error;
pub mod app_error;

// ~ private modules ~

pub mod data_input_error;
pub mod parse_error;
