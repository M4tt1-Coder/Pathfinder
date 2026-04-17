//! Graph abstractions and concrete graph data structures.
//!
//! # Overview
//!
//! The crate exposes multiple graph implementations through this module:
//! - [`graph`]: shared graph traits used by algorithms.
//! - [`directed`]: directed weighted graph implementation.
//! - [`undirected`]: undirected weighted graph implementation.
//! - [`two_dimensional_coordinate_graph`]: coordinate-based graph model.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::directed::DirectedGraph;
//! use shortest_path_finder::graphs::graph::Graph;
//!
//! let graph = DirectedGraph::new(vec![], vec![]);
//! assert!(graph.is_directed());
//! assert!(graph.is_weighted());
//! ```

pub mod directed;
pub mod graph;
pub mod two_dimensional_coordinate_graph;
pub mod undirected;
