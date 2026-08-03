//! Data-input boundary for graph construction.
//!
//! # Overview
//!
//! This namespace groups every supported graph input origin:
//! - [`file`]: production file parser and its CLI configuration helpers.
//! - [`cmd_line`]: placeholder for the interactive terminal input path that is
//!   planned for a later release.
//!
//! The current runtime uses the file parser exclusively. When parsing succeeds,
//! [`file::retrieve_graph_data_from_file`] returns a [`file::FileInputGraphResult`]
//! variant (`DirectedGraph`, `UndirectedGraph`, or `TwoDimensionalGraph`) that
//! matches the file header. Failures surface as [`file::FileInputError`] and are
//! typically wrapped in [`crate::error::data_input_error::DataInputError`] at the
//! CLI boundary.
//!
//! # File Input
//!
//! The file pipeline is the stable production path.
//!
//! ```rust
//! use shortest_path_finder::data_input::file::{
//!     FileInputGraphResult, retrieve_graph_data_from_file,
//! };
//!
//! let parsed = retrieve_graph_data_from_file("test_files/directed_graph.txt")
//!     .expect("graph fixture should parse");
//! assert!(matches!(parsed, FileInputGraphResult::DirectedGraph(_)));
//! ```
//!
//! # Command-Line Input
//!
//! The interactive command-line origin is documented separately so callers can
//! see the intended boundary even though the runtime implementation is still
//! pending.
//!
//! ```rust
//! use shortest_path_finder::data_input::file::cli_config::InputOrigin;
//!
//! let origin = InputOrigin::CommandLine;
//! assert_eq!(origin.as_str(), "cmd-line");
//! ```
//!
//! # Error Propagation
//!
//! ```rust
//! use shortest_path_finder::data_input::file::retrieve_graph_data_from_file;
//! use shortest_path_finder::error::data_input_error::DataInputError;
//!
//! let err = retrieve_graph_data_from_file("definitely/missing.graph").unwrap_err();
//! assert!(matches!(err, DataInputError::File(_)));
//! assert!(err.to_string().contains("File input error"));
//! ```

pub mod cmd_line;
pub mod file;
