//! Algorithm implementations and shared algorithm contracts.
//!
//! # Overview
//!
//! This module groups all shortest-path algorithm related code used by this
//! crate:
//! - [`algorithm`]: shared traits and algorithm-selection enum.
//! - [`dijkstra`]: concrete Dijkstra implementation.
//! - [`a_star_algorithm`]: coordinate-based A* implementation.
//!
//! # Usage
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
//! assert!(graph.insert_edge(DirectedEdge::new(a, b, 4)).is_none());
//!
//! let dijkstra = DijkstraAlgorithm::new(graph);
//! let result = dijkstra.shortest_path("A", "B").unwrap();
//! assert_eq!(result.get_total_distance(), 4);
//! ```

pub mod a_star_algorithm;
pub mod algorithm;
pub mod dijkstra;
