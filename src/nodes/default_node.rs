//! # DefaultNode Struct Implementation
//!
//! This module provides the implementation of a `DefaultNode` struct, which serves as a basic
//! representation of a node in a graph with a string identifier. It implements necessary traits
//! for comparison, hashing, display, and interaction with graph algorithms through the `GraphNode`
//! trait. This struct is suitable for cases where nodes are uniquely identified by string IDs.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::graph::GraphNode;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let node = DefaultNode::new("node1".to_string());
//! println!("Node ID: {}", node); // prints "node1"
//! ```

// ----- Implementation of the 'DefaultNode' struct -----

use std::{fmt::Display, str::FromStr};

use crate::graphs::graph::GraphNode;

/// `DefaultNode` is a simple implementation of a graph node identified by a string ID.
///
/// It derives common traits such as `Clone`, `PartialEq`, `Eq`, `Hash`, `Debug`, `Ord`, and `PartialOrd`
/// to facilitate comparison, hashing, cloning, and debugging.
///
/// # Fields
/// - `id`: A `String` representing the unique identifier of the node.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub struct DefaultNode {
    /// The unique identifier for the node.
    pub id: String,
}

impl DefaultNode {
    /// Creates a new `DefaultNode` with the given string ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A `String` representing the node's identifier.
    ///
    /// # Returns
    ///
    /// A new instance of `DefaultNode` with the specified ID.
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Display for DefaultNode {
    /// Formats the `DefaultNode` for display purposes.
    ///
    /// This implementation simply writes out the node's ID.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl GraphNode for DefaultNode {
    /// Retrieves the string ID of the node.
    ///
    /// # Returns
    ///
    /// A string slice referencing the node's ID.
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl FromStr for DefaultNode {
    type Err = ();

    /// Parses a `DefaultNode` from a string.
    ///
    /// This implementation simply creates a `DefaultNode` with the input string as its ID.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice to parse into a `DefaultNode`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `DefaultNode` or an error if parsing fails.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
