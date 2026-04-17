//! Data-input boundary for graph construction.
//!
//! # Overview
//!
//! This module groups all graph input origins supported by the project:
//! - [`file_input`]: production parser for file-based graph definitions.
//! - [`terminal_input`]: placeholder for future interactive terminal input.
//!
//! # Usage
//!
//! ```no_run
//! use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;
//!
//! let result = retrieve_graph_data_from_file("test_files/directed_graph.txt");
//! assert!(result.is_ok());
//! ```

pub mod file_input;
pub mod terminal_input;
