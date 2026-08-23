//! Graph abstractions and concrete graph data structures.
//!
//! # Overview
//!
//! The graph layer is organized around a small shared trait surface and a set
//! of concrete graph types that model the supported input formats:
//!
//! - [`graph`]: shared traits used by algorithms and integration tests.
//! - [`directed`]: a weighted directed graph over [`crate::nodes::default_node::DefaultNode`].
//! - [`undirected`]: a weighted undirected graph over [`crate::nodes::default_node::DefaultNode`].
//! - [`two_dimensional_coordinate_graph`]: a coordinate-based weighted graph
//!   built on [`crate::nodes::two_dimensional_node::TwoDimensionalNode`].
//!
//! The public [`GraphInsertionError`] wrapper lets callers handle insertion
//! failures without committing to a concrete graph implementation. It is
//! especially useful when graph parsing selects an implementation at runtime.
//!
//! # Common use cases
//!
//! - Build a graph directly in memory for algorithm tests.
//! - Inspect whether a graph is directed or weighted before running a path
//!   search.
//! - Match on [`GraphInsertionError`] when a graph mutation fails.
//!
//! # Examples
//!
//! Create a directed graph and query its capabilities:
//!
//! ```rust
//! use shortest_path_finder::graph::DirectedGraph;
//! use shortest_path_finder::graph::Graph;
//!
//! let graph = DirectedGraph::new(vec![]);
//! assert!(graph.is_directed());
//! assert!(graph.is_weighted());
//! ```
//!
//! Handle a graph insertion failure in a concrete, type-safe way:
//!
//! ```rust
//! use shortest_path_finder::graph::{directed::DirectedGraphInsertionError, GraphInsertionError};
//!
//! let error = GraphInsertionError::Directed(
//!     DirectedGraphInsertionError::SourceNodeDoesNotExist {
//!         node_id: "A".to_string(),
//!     },
//! );
//!
//! assert!(matches!(
//!     error,
//!     GraphInsertionError::Directed(DirectedGraphInsertionError::SourceNodeDoesNotExist { .. })
//! ));
//! ```

use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::Add,
};

/// High-level error wrapper for graph insertion failures.
///
/// This type erases the concrete graph implementation while preserving the
/// exact insertion failure that occurred. Use it when a parsing or graph
/// construction boundary can select between multiple graph types at runtime,
/// but the caller still needs to inspect the specific reason the insertion
/// failed.
///
/// # Why this wrapper exists
///
/// Each concrete graph has its own insertion error enum, but callers at higher
/// layers often only care that graph mutation failed. The wrapper keeps the API
/// ergonomic without losing the original error details.
///
/// # How to use it
///
/// - Pattern-match on a variant when you need graph-specific recovery logic.
/// - Call [`std::string::ToString::to_string`] or use the [`std::fmt::Display`]
///   implementation for user-facing diagnostics.
/// - Convert it into higher-level errors such as [`crate::error::ParseError`].
///
/// # Variants
///
/// - [`GraphInsertionError::Directed`]: errors from [`directed::DirectedGraph`].
/// - [`GraphInsertionError::Undirected`]: errors from [`undirected::UndirectedGraph`].
/// - [`GraphInsertionError::TwoDimensional`]: errors from
///   [`two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph`].
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::graph::{directed::DirectedGraphInsertionError, GraphInsertionError};
///
/// let error = GraphInsertionError::Directed(
///     DirectedGraphInsertionError::MissingEdgeWeight {
///         expected_weight: "u16".to_string(),
///     },
/// );
///
/// assert!(error.to_string().contains("Missing edge weight"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphInsertionError {
    /// Error produced by the directed graph implementation.
    Directed(directed::DirectedGraphInsertionError),
    /// Error produced by the undirected graph implementation.
    Undirected(undirected::UndirectedGraphInsertionError),
    /// Error produced by the two-dimensional coordinate graph implementation.
    TwoDimensional(two_dimensional::TwoDimensionalGraphInsertionError),
}

impl std::fmt::Display for GraphInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphInsertionError::Directed(err) => {
                write!(f, "Directed graph insertion error: {}", err)
            }
            GraphInsertionError::Undirected(err) => {
                write!(f, "Undirected graph insertion error: {}", err)
            }
            GraphInsertionError::TwoDimensional(err) => {
                write!(f, "Two-dimensional graph insertion error: {}", err)
            }
        }
    }
}

impl std::error::Error for GraphInsertionError {}

/// Trait describing the behavior of a graph data structure.
///
/// A graph implementation can be directed or undirected, weighted or unweighted,
/// and can expose custom node and weight types as long as those types satisfy
/// the associated trait bounds.
///
/// # Semantics
///
/// - Nodes are represented by [`Graph::Node`] and are identified by stable IDs.
/// - Edges are represented implicitly through adjacency lists using the weight
///   type [`Graph::Weight`].
/// - Neighbor traversal returns `(neighbor, weight)` pairs.
///
/// # Error Behavior
///
/// Edge and node insertion operations may return implementation-specific errors
/// when constraints are violated (duplicates, missing nodes, invalid weights).
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::graph::DirectedGraph;
/// use shortest_path_finder::graph::{Graph, GraphNode};
/// use shortest_path_finder::nodes::DefaultNode;
///
/// let a = DefaultNode::new("A".to_string());
/// let b = DefaultNode::new("B".to_string());
/// let mut graph = DirectedGraph::new(vec![a.clone(), b.clone()]);
/// graph.insert_edge(&a, &b, Some(1));
///
/// let node_a = graph.get_node_by_id("A").unwrap();
/// let neighbors: Vec<_> = graph.neighbors(node_a).collect();
/// assert_eq!(neighbors.len(), 1);
/// assert_eq!(neighbors[0].0.get_id(), "B");
/// assert_eq!(neighbors[0].1, 1);
/// ```
pub trait Graph {
    /// Node type stored by this graph.
    ///
    /// Must implement [`GraphNode`] so algorithms can retrieve node IDs and use
    /// equality/hash/order operations.
    ///
    /// # Example
    /// ```rust
    /// use std::fmt::{Display, Formatter};
    /// use shortest_path_finder::graph::GraphNode;
    ///
    /// #[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
    /// struct Node {
    ///     id: String,
    /// }
    ///
    /// impl Display for Node {
    ///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "{}", self.id)
    ///     }
    /// }
    ///
    /// impl GraphNode for Node {
    ///     fn get_id<'a>(&'a self) -> &'a str {
    ///         &self.id
    ///     }
    /// }
    /// ```
    type Node: GraphNode;

    /// Weight type used for edges.
    ///
    /// Must satisfy [`GraphWeight`].
    ///
    /// # Example
    /// ```rust
    /// type Weight = u32;
    /// ```
    type Weight: GraphWeight;

    /// Error type used by insertion/mutation operations.
    ///
    /// Should contain enough context to explain why a mutation failed.
    type InsertionError: Error + Display + Debug;

    /// Returns neighbors of `u` with the corresponding edge weight.
    ///
    /// # Parameters
    ///
    /// - `u`: Node whose outgoing (or adjacent) edges should be traversed.
    ///
    /// # Returns
    ///
    /// Iterator over `(neighbor, weight)` pairs.
    ///
    /// # Notes
    ///
    /// The iterator yields borrowed nodes. Ordering is implementation-specific.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graph::{
    ///     DirectedGraph,
    ///     Graph, GraphNode,
    /// };
    /// use shortest_path_finder::nodes::DefaultNode;
    ///
    /// let mut graph = DirectedGraph::new(vec![
    ///     DefaultNode::new("A".to_string()),
    ///     DefaultNode::new("B".to_string()),
    /// ]);
    /// let from = DefaultNode::new("A".to_string());
    /// let to = DefaultNode::new("B".to_string());
    /// graph.insert_edge(&from, &to, Some(6));
    ///
    /// let node = DefaultNode::new("A".to_string());
    ///
    /// let neighbors: Vec<_> = graph.neighbors(&node).collect();
    /// assert_eq!(neighbors.len(), 1);
    /// assert_eq!(neighbors[0].0.get_id(), "B");
    /// assert_eq!(neighbors[0].1, 6);
    /// ```
    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a>;

    /// Indicates whether edge direction is respected.
    ///
    /// # Returns
    /// - `true` for directed graphs.
    /// - `false` for undirected graphs.
    ///
    /// # Example
    /// ```rust
    /// use shortest_path_finder::graph::DirectedGraph;
    /// use shortest_path_finder::graph::Graph;
    ///
    /// let graph = DirectedGraph::new(vec![]);
    /// assert!(graph.is_directed());
    /// ```
    fn is_directed(&self) -> bool;

    /// Inserts a node into the graph.
    ///
    /// Implementations may ignore duplicates instead of returning an error.
    ///
    /// # Parameters
    ///
    /// - `new_node`: Node to add.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graph::DirectedGraph;
    /// use shortest_path_finder::graph::Graph;
    /// use shortest_path_finder::nodes::DefaultNode;
    ///
    /// let mut graph = DirectedGraph::new(vec![]);
    /// graph.insert_node(DefaultNode::new("A".to_string()));
    /// assert!(graph.get_node_by_id("A").is_some());
    /// ```
    fn insert_node(&mut self, new_node: Self::Node);

    /// Inserts an edge into the graph.
    ///
    /// # Parameters
    ///
    /// - `from`: Source node.
    /// - `to`: Destination node.
    /// - `weight`: Optional edge weight, when applicable.
    ///
    /// # Returns
    ///
    /// - `None` if insertion succeeded.
    /// - `Some(Self::InsertionError)` if insertion failed.
    ///
    /// # Notes
    ///
    /// Implementations decide how to treat `weight = None` for unweighted
    /// graphs (ignore, use a default, or reject).
    fn insert_edge(
        &mut self,
        from: &Self::Node,
        to: &Self::Node,
        weight: Option<Self::Weight>,
    ) -> Option<Self::InsertionError>;

    /// Checks whether a semantically equivalent edge already exists.
    ///
    /// # Parameters
    ///
    /// - `edge`: Candidate edge.
    ///
    /// # Returns
    ///
    /// `true` if an equivalent edge is already present.
    fn does_edge_already_exist(&self, from: &Self::Node, to: &Self::Node) -> bool;

    /// Checks whether a semantically equivalent node already exists.
    ///
    /// # Parameters
    ///
    /// - `node`: Candidate node.
    ///
    /// # Returns
    ///
    /// `true` if an equivalent node is already present.
    fn does_node_already_exist(&self, node: &Self::Node) -> bool;

    /// Retrieves a node by ID.
    ///
    /// # Parameters
    ///
    /// - `id`: Node identifier.
    ///
    /// # Returns
    ///
    /// - `Some(&Self::Node)` if found.
    /// - `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graph::DirectedGraph;
    /// use shortest_path_finder::graph::{Graph, GraphNode};
    /// use shortest_path_finder::nodes::DefaultNode;
    ///
    /// let node = DefaultNode::new("A".to_string());
    /// let graph = DirectedGraph::new(vec![node]);
    /// assert_eq!(graph.get_node_by_id("A").unwrap().get_id(), "A");
    /// assert!(graph.get_node_by_id("missing").is_none());
    /// ```
    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node>;

    /// Returns all nodes currently contained in the graph.
    ///
    /// # Returns
    ///
    /// Borrowed vector of all graph nodes.
    ///
    /// # Notes
    ///
    /// Ordering is implementation-specific and should not be relied upon.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graph::DirectedGraph;
    /// use shortest_path_finder::graph::Graph;
    /// use shortest_path_finder::nodes::DefaultNode;
    ///
    /// let graph = DirectedGraph::new(
    ///     vec![DefaultNode::new("A".to_string()), DefaultNode::new("B".to_string())],
    /// );
    /// assert_eq!(graph.get_all_nodes().len(), 2);
    /// ```
    fn get_all_nodes(&self) -> &Vec<Self::Node>;

    /// Indicates whether this graph carries meaningful edge weights.
    ///
    /// Some algorithms (for example Dijkstra and A*) require weighted edges.
    ///
    /// # Returns
    ///
    /// `true` if edge weights are available.
    fn is_weighted(&self) -> bool;

    /// Returns a short, stable graph-type abbreviation.
    ///
    /// Commonly used by parsing/serialization code to identify graph kinds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graph::DirectedGraph;
    /// use shortest_path_finder::graph::Graph;
    ///
    /// assert_eq!(DirectedGraph::abbreviation(), "D");
    /// ```
    fn abbreviation() -> String;
}

/// A trait representing a type suitable for use as a weight in graph algorithms.
///
/// # Requirements
/// This trait is implemented for types that:
/// - are `Copy`, allowing for inexpensive duplication,
/// - implement `PartialOrd`, enabling comparison of weights,
/// - support addition with `Add<Output = Self>`, allowing weights to be combined,
/// - and can perform overflow-aware addition via [`GraphWeight::checked_add`].
///
/// # Overflow Handling
///
/// Algorithms use `checked_add` to prevent overflow or non-finite sums. For
/// floating-point weights, `checked_add` should return `None` for NaN or
/// infinite results.
///
/// # Usage
/// Use `GraphWeight` as a trait bound for generic types in graph algorithms,
/// such as shortest path or minimum spanning tree implementations,
/// where weights need to be comparable, clonable, and combinable via addition.
///
/// # Example
/// ```rust
/// use std::ops::Add;
/// use shortest_path_finder::graph::GraphWeight;
///
/// fn total_weight<W: GraphWeight>(weights: &[W]) -> W {
///     weights.iter().cloned().fold(W::zero(), |acc, w| acc + w)
/// }
///
/// let weights = vec![1u16, 2u16, 3u16];
/// assert_eq!(total_weight(&weights), 6u16);
/// ```
pub trait GraphWeight:
    Copy + PartialOrd + Add<Output = Self> + Display + Debug + PartialOrd + PartialEq
{
    /// Returns the maximum possible value for the weight type.
    ///
    /// This value is typically used to initialize distances or weights that need
    /// to be replaced with smaller values during algorithm execution.
    ///
    /// # Returns
    /// The maximum value of the implementing type.
    fn max_value() -> Self;

    /// Returns the zero value (additive identity) for the weight type.
    ///
    /// This value is used as the default or starting weight in graph algorithms,
    /// representing no cost or distance.
    ///
    /// # Returns
    /// The zero value of the implementing type.
    fn zero() -> Self;

    /// Returns the sum of two weights if it can be represented safely.
    ///
    /// Implementations should return `None` if the addition would overflow or
    /// produce a non-finite value (for floating-point weights).
    ///
    /// # Parameters
    ///
    /// - `other`: The weight to add to `self`.
    fn checked_add(self, other: Self) -> Option<Self>;
}

/// Trait for node values stored in graph implementations.
///
/// # Requirements
///
/// A node must have a stable textual identifier retrievable via
/// [`GraphNode::get_id`]. The identifier is used for lookup, display, and
/// serialization.
///
/// # Example
///
/// ```rust
/// use std::fmt::{Display, Formatter};
/// use shortest_path_finder::graph::GraphNode;
///
/// #[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
/// struct City {
///     id: String,
/// }
///
/// impl Display for City {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}", self.id)
///     }
/// }
///
/// impl GraphNode for City {
///     fn get_id(&self) -> &str {
///         &self.id
///     }
/// }
///
/// let berlin = City { id: "BER".to_string() };
/// assert_eq!(berlin.get_id(), "BER");
/// ```
pub trait GraphNode: Display + Debug + Eq + std::hash::Hash + Clone + Ord {
    /// Returns the node identifier.
    fn get_id(&self) -> &str;
}

// ========== Modulization ==========

// ~ public modules ~

pub mod directed;
pub mod two_dimensional;
pub mod undirected;

// ~ private modules ~

mod utils;

// ~ re-exports ~

pub use directed::DirectedGraph;
pub use two_dimensional::TwoDimensionalCoordinateGraph;
pub use undirected::UndirectedGraph;
