//! Pathfinder library crate.
//!
//! # Overview
//!
//! This crate provides reusable building blocks for shortest-path workflows:
//! - graph abstractions and concrete graph implementations,
//! - file-based graph parsing,
//! - command-line configuration models,
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
//! - [`data_input`]: graph input parsing (currently file-based).
//! - [`cmd_line`]: CLI configuration parsing helpers.
//! - [`error`]: parse-time error definitions.
//! - [`weight_types`] and [`numeric_datatypes`]: numeric traits and impls.
//!
//! # Quick Start
//!
//! ```rust
//! use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
//! use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
//! use shortest_path_finder::graphs::directed::{DirectedEdge, DirectedGraph};
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let mut graph = DirectedGraph::default();
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! graph.insert_node(a.clone());
//! graph.insert_node(b.clone());
//! assert!(graph.insert_edge(DirectedEdge::new(a, b, 7)).is_none());
//!
//! let dijkstra = DijkstraAlgorithm::new(graph);
//! let result = dijkstra.shortest_path("A", "B").unwrap();
//! assert_eq!(result.get_total_distance(), 7);
//! ```

pub mod algorithms;
pub mod cmd_line;
pub mod data_input;
pub mod error;
pub mod graphs;
pub mod nodes;
pub mod numeric_datatypes;
pub mod weight_types;
