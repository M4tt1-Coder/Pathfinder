//! Unified wrapper over parser-supported node representations.
//!
//! # Overview
//!
//! [`NodeType`] is used at input/parsing boundaries where node values are not yet
//! known at compile time. It enables conversion code to return one typed payload
//! while preserving the concrete node variant.
//!
//! # Typical usage
//!
//! - File parsing returns temporary values as [`NodeType`].
//! - Graph-type-specific code then matches on the enum and converts the node into
//!   the concrete representation required by that graph implementation.
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::nodes::{
//!     default_node::DefaultNode,
//!     node_types::NodeType,
//!     two_dimensional_node::TwoDimensionalNode,
//! };
//!
//! let generic = NodeType::DefaultNode(DefaultNode::new("A".to_string()));
//! let parsed = match generic {
//!     NodeType::DefaultNode(node) => node.id,
//!     NodeType::TwoDimensionalNode(_) => "unexpected".to_string(),
//! };
//!
//! assert_eq!(parsed, "A");
//!
//! let td = NodeType::TwoDimensionalNode(
//!     TwoDimensionalNode::new(1, 2, "P".to_string()).unwrap(),
//! );
//! assert!(matches!(td, NodeType::TwoDimensionalNode(_)));
//! ```

use crate::nodes::{default_node::DefaultNode, two_dimensional_node::TwoDimensionalNode};

/// Enum representing all node variants currently supported by parser output.
///
/// # Variant selection
///
/// - [`NodeType::DefaultNode`] is used by directed and undirected graph parsing.
/// - [`NodeType::TwoDimensionalNode`] is used by two-dimensional coordinate graph parsing.
#[derive(Debug)]
pub enum NodeType {
    /// A node with two-dimensional coordinates (e.g., for grid graphs).
    TwoDimensionalNode(TwoDimensionalNode),
    /// A generic node type, typically identified by a label or ID.
    DefaultNode(DefaultNode),
}
