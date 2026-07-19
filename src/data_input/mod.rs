//! Data-input boundary for graph construction.
//!
//! # Overview
//!
//! This module groups graph input origins supported by the project:
//! - [`file_input`]: production parser for file-based graph definitions.
//! - [`cmd_line_input`]: placeholder for future interactive terminal input.
//!
//! Successful file parsing returns a [`file_input::FileInputGraphResult`] enum
//! variant (`DirectedGraph`, `UndirectedGraph`, or `TwoDimensionalGraph`) that
//! matches the file header. Failures surface as
//! [`file_input::FileInputError`] and are typically wrapped in
//! [`crate::error::data_input_error::DataInputError`] at the CLI boundary.
//!
//! # Usage
//!
//! ```no_run
//! use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;
//!
//! let result = retrieve_graph_data_from_file("test_files/directed_graph.txt");
//! assert!(result.is_ok());
//! ```
//!
//! # Error Propagation
//!
//! ```rust
//! use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;
//! use shortest_path_finder::error::data_input_error::DataInputError;
//!
//! let err = retrieve_graph_data_from_file("definitely/missing.graph").unwrap_err();
//! assert!(matches!(err, DataInputError::File(_)));
//! assert!(err.to_string().contains("File input error"));
//! ```

pub mod cmd_line_input;
pub mod file_input;
