//! Node data models used by graph implementations.
//!
//! # Overview
//!
//! This module groups all node shapes used throughout the crate:
//! - [`default_node`]: simple ID-based node type.
//! - [`two_dimensional_node`]: coordinate-aware node type.
//! - [`node_types`]: enum wrapper for parser output.
//! - [`trait_decl`]: shared node trait declarations.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::graph::GraphNode;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let node = DefaultNode::new("Hub".to_string());
//! assert_eq!(node.get_id(), "Hub");
//! ```

pub mod default_node;
pub mod node_types;
pub mod trait_decl;
pub mod two_dimensional_node;
