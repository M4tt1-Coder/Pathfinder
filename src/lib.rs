//! Pathfinder library crate.
//!
//! # Overview
//!
//! This crate provides reusable building blocks for shortest-path workflows:
//! - graph abstractions and concrete graph implementations,
//! - file-based graph parsing and its CLI configuration boundary,
//! - planned command-line graph input plumbing,
//! - shortest-path algorithms (currently Dijkstra and A* modules).
//!
//! The binary target wires these modules together, but consumers can use the
//! library directly in their own applications and tests.
//!
//! # Module Map
//!
//! - [`algorithms`]: algorithm traits and concrete implementations.
//! - [`graphs`]: graph traits and graph data structures.
//! - [`nodes`]: node models used by graph implementations.
//! - [`data_input`]: graph-input boundaries, including file parsing and the
//!   planned command-line input namespace.
//! - [`error`]: parse-time, CLI configuration, and algorithm execution errors.
//! - [`weight_types`] and [`numeric_datatypes`]: numeric traits and impls.
//!
//! # Error Handling
//!
//! Errors are layered by boundary:
//! - [`error::ParseError`] for line-level graph syntax failures.
//! - [`error::DataInputError`] for file loading and parsing.
//! - [`error::CLIParseError`] for CLI flag validation.
//! - [`error::algorithm_error::AlgorithmError`] for shortest-path execution.
//! - [`error::AppError`] as the binary-level wrapper with exit-code mapping.
//!
//! Use [`error::algorithm_error::AlgorithmErrorKind`] when you need stable
//! categories (for example exit codes or telemetry).
//!
//! ```rust
//! use shortest_path_finder::error::algorithm_error::{AlgorithmError, AlgorithmErrorKind,
//! dijkstra_error::DijkstraError};
//!
//! let err = AlgorithmError::from(DijkstraError::NoPathFound {
//!     start: "A".to_string(),
//!     end: "B".to_string(),
//! });
//! assert_eq!(err.kind(), AlgorithmErrorKind::NoPath);
//! ```
//!
//! File-input failures are typically wrapped before they reach application code:
//!
//! ```rust
//! use shortest_path_finder::data_input::file::FileInputError;
//! use shortest_path_finder::error::DataInputError;
//! use shortest_path_finder::error::ParseError;
//!
//! let err = DataInputError::from(FileInputError::Parse {
//!     file_path: "graph.txt".to_string(),
//!     source: ParseError::MissingColon,
//! });
//! assert!(err.to_string().contains("File input error"));
//! ```
//!
//! # Quick Start
//!
//! ```rust
//! use shortest_path_finder::algorithms::{Algorithm, SearchResult};
//! use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
//! use shortest_path_finder::graph::DirectedGraph;
//! use shortest_path_finder::graph::Graph;
//! use shortest_path_finder::nodes::DefaultNode;
//!
//! let mut graph = DirectedGraph::default();
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! graph.insert_node(a.clone());
//! graph.insert_node(b.clone());
//! assert!(graph.insert_edge(&a, &b, Some(7)).is_none());
//!
//! let dijkstra = DijkstraAlgorithm::new(graph);
//! let result = dijkstra.shortest_path("A", "B").unwrap();
//! assert_eq!(result.get_total_distance(), 7);
//! ```

pub mod algorithms;
pub mod data_input;
pub mod error;
pub mod graph;
pub mod nodes;
pub mod numeric_datatypes;
pub mod weight_types;

pub use error::AppError;
