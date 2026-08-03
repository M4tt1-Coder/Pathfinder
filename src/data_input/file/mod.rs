//! File-input namespace for graph construction.
//!
//! # Overview
//!
//! This module re-exports the production file parser and exposes the nested
//! `cli_config` helpers used to parse runtime flags that select file input.
//!
//! The canonical entry point is [`retrieve_graph_data_from_file`], which reads a
//! graph fixture and returns one [`FileInputGraphResult`] variant.
//!
//! # Examples
//!
//! ```rust
//! use shortest_path_finder::data_input::file::{
//!     FileInputGraphResult, retrieve_graph_data_from_file,
//! };
//!
//! let parsed = retrieve_graph_data_from_file("test_files/undirected_graph.txt")
//!     .expect("graph fixture should parse");
//! assert!(matches!(parsed, FileInputGraphResult::UndirectedGraph(_)));
//! ```

mod file_input;

// ~ flatten module paths ~

pub use file_input::*;

// ~ public modules ~

// ~ private modules ~

pub mod cli_config;
