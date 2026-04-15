//! Defines the `NodeType` enum, which encapsulates supported node variants for graph algorithms.
//!
//! This module provides a unified enum to represent different node types used throughout the
//! pathfinding library, enabling algorithms and data structures to operate generically over
//! heterogeneous node representations.

use crate::nodes::{default_node::DefaultNode, two_dimensional_node::TwoDimensionalNode};

/// Represents the supported node variants in the pathfinding library.
///
/// `NodeType` is an enum that can wrap either a [`TwoDimensionalNode`] (for grid-based graphs)
/// or a [`DefaultNode`] (for generic node identifiers). This abstraction allows graph
/// structures and algorithms to handle multiple node representations in a type-safe manner.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::nodes::{
///     default_node::DefaultNode,
///     node_types::NodeType,
///     two_dimensional_node::TwoDimensionalNode,
/// };
///
/// let default = NodeType::DefaultNode(DefaultNode::new("A".to_string()));
/// assert!(matches!(default, NodeType::DefaultNode(_)));
///
/// let td = NodeType::TwoDimensionalNode(
///     TwoDimensionalNode::new(1, 2, "P".to_string()).unwrap(),
/// );
/// assert!(matches!(td, NodeType::TwoDimensionalNode(_)));
/// ```
#[derive(Debug)]
pub enum NodeType {
    /// A node with two-dimensional coordinates (e.g., for grid graphs).
    TwoDimensionalNode(TwoDimensionalNode),
    /// A generic node type, typically identified by a label or ID.
    DefaultNode(DefaultNode),
}
