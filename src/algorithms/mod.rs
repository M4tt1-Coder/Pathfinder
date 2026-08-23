//! Algorithm implementations and shared algorithm contracts.
//!
//! # Overview
//!
//! This module groups all shortest-path algorithm related code used by this
//! crate:
//! - [`Algorithm`]: shared trait for algorithm implementations.
//! - [`Algorithms`]: enumeration for selecting between available algorithms.
//! - [`dijkstra`]: concrete Dijkstra implementation.
//! - [`a_star_algorithm`]: coordinate-based A* implementation.
//!
//! # Usage
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
//! assert!(graph.insert_edge(&a, &b, Some(4)).is_none());
//!
//! let dijkstra = DijkstraAlgorithm::new(graph);
//! let result = dijkstra.shortest_path("A", "B").unwrap();
//! assert_eq!(result.get_total_distance(), 4);
//! ```

mod algorithm;

// ~ flatten module paths ~

pub use algorithm::*;

// ~ public modules ~

pub mod a_star_algorithm;
pub mod dijkstra;

// ~ private modules ~
