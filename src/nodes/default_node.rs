//! Default string-identifier node model.
//!
//! # Overview
//!
//! [`DefaultNode`] is the simplest node type in the crate. It is primarily used
//! by one-dimensional directed and undirected graph implementations where a node
//! is uniquely identified by its textual ID.
//!
//! # Design Notes
//!
//! - The node stores exactly one field: `id`.
//! - The type implements [`crate::graphs::graph::GraphNode`] so it can be used
//!   with all generic graph and algorithm traits.
//! - [`std::str::FromStr`] is intentionally permissive and treats the full input
//!   as the node ID.
//!
//! # Examples
//!
//! ```rust
//! use shortest_path_finder::graphs::graph::GraphNode;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let node = DefaultNode::new("Hub".to_string());
//! assert_eq!(node.get_id(), "Hub");
//! assert_eq!(node.to_string(), "Hub");
//! ```

// ----- Implementation of the 'DefaultNode' struct -----

use std::{fmt::Display, str::FromStr};

use crate::graphs::graph::GraphNode;

/// Node type identified by a unique string ID.
///
/// # When to use
///
/// Use this type when:
/// - coordinate data is not required,
/// - node identity is naturally represented by labels such as `"A"`,
/// - and compatibility with [`crate::graphs::directed::DirectedGraph`] or
///   [`crate::graphs::undirected::UndirectedGraph`] is needed.
///
/// # Examples
///
/// ```rust
/// use shortest_path_finder::graphs::graph::GraphNode;
/// use shortest_path_finder::nodes::default_node::DefaultNode;
///
/// let source = DefaultNode::new("Source".to_string());
/// assert_eq!(source.get_id(), "Source");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub struct DefaultNode {
    /// The unique identifier for the node.
    pub id: String,
}

impl DefaultNode {
    /// Creates a new [`DefaultNode`] with the given identifier.
    ///
    /// # Parameters
    ///
    /// - `id`: node identifier.
    ///
    /// # Returns
    ///
    /// New [`DefaultNode`] instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let node = DefaultNode::new("N-01".to_string());
    /// assert_eq!(node.id, "N-01");
    /// ```
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Display for DefaultNode {
    /// Formats this node as its raw identifier.
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

    /// Parses a [`DefaultNode`] from a plain string.
    ///
    /// # Behavior
    ///
    /// Parsing is infallible and copies `s` into [`DefaultNode::id`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use shortest_path_finder::graphs::graph::GraphNode;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let node = DefaultNode::from_str("Station-42").unwrap();
    /// assert_eq!(node.get_id(), "Station-42");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
