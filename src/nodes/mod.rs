//! Node data models used by graph implementations.
//!
//! # Overview
//!
//! This module groups all node shapes used throughout the crate:
//! - [`DefaultNode`]: simple ID-based node type.
//! - [`TwoDimensionalNode`]: coordinate-aware node type.
//! - [`NodeType`]: enum wrapper for parser output.
//! - [`trait_decl`]: shared node trait declarations.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graph::GraphNode;
//! use shortest_path_finder::nodes::DefaultNode;
//!
//! let node = DefaultNode::new("Hub".to_string());
//! assert_eq!(node.get_id(), "Hub");
//! ```

// ~ private modules ~

mod default_node;
mod node_type;
mod two_dimensional_node;

// ~ public modules ~

pub mod trait_decl;

// ~ public re-exports ~

pub use default_node::DefaultNode;
pub use node_type::NodeType;
pub use two_dimensional_node::TwoDimensionalNode;
