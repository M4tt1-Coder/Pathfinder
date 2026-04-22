//! Shared graph abstractions used across the crate.
//!
//! This module defines the core traits that all graph implementations and
//! graph-related algorithms rely on:
//! - [`Graph`] for graph containers and operations,
//! - [`GraphNode`] for node identity,
//! - [`GraphEdge`] for edge identity,
//! - [`GraphWeight`] for numeric edge weights.
//!
//! The traits are designed to support both directed and undirected graphs.
//! Concrete implementations live in sibling modules such as
//! `graphs::directed`, `graphs::undirected`, and
//! `graphs::two_dimensional_coordinate_graph`.
//!
//! # Quick Example
//!
//! ```rust
//! use shortest_path_finder::graphs::directed::{DirectedEdge, DirectedGraph};
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! let edge = DirectedEdge::new(a.clone(), b.clone(), 5);
//! let graph = DirectedGraph::new(vec![a, b], vec![edge]);
//!
//! assert!(graph.is_directed());
//! assert!(graph.is_weighted());
//! ```

use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::Add,
};

// TODO: Consider removing the edge structs from the graphs since they are now represented by
// adjacency lists. This would simplify the API and reduce redundancy, but may require reworking
// some algorithms that currently rely on edge objects.

/// Trait describing the behavior of a graph data structure.
///
/// A graph implementation can be directed or undirected, weighted or unweighted,
/// and can expose custom node/edge types as long as those types satisfy the
/// associated trait bounds.
///
/// # Design Notes
/// - Nodes are represented by [`Graph::Node`] and are identified by stable IDs.
/// - Edges are represented by [`Graph::Edge`] and carry a weight of type
///   [`Graph::Weight`].
/// - Neighbor traversal returns `(neighbor, weight)` pairs.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::graphs::directed::{DirectedEdge, DirectedGraph};
/// use shortest_path_finder::graphs::graph::{Graph, GraphNode};
/// use shortest_path_finder::nodes::default_node::DefaultNode;
///
/// let a = DefaultNode::new("A".to_string());
/// let b = DefaultNode::new("B".to_string());
/// let graph = DirectedGraph::new(
///     vec![a.clone(), b.clone()],
///     vec![DirectedEdge::new(a, b, 1)],
/// );
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
    /// use shortest_path_finder::graphs::graph::GraphNode;
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

    /// Edge type used by this graph.
    ///
    /// Must satisfy [`GraphEdge`] so edge identity can be accessed in a uniform way.
    ///
    /// # Example
    /// ```rust
    /// use shortest_path_finder::graphs::graph::GraphEdge;
    ///
    /// #[derive(Clone, PartialEq)]
    /// struct Edge {
    ///     id: u32,
    /// }
    ///
    /// impl GraphEdge for Edge {
    ///     type ID = u32;
    ///
    ///     fn get_id(&self) -> Self::ID {
    ///         self.id
    ///     }
    /// }
    /// ```
    type Edge: GraphEdge;

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
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::{
    ///     directed::{DirectedEdge, DirectedGraph},
    ///     graph::{Graph, GraphNode},
    /// };
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let graph = DirectedGraph::new(
    ///     vec![DefaultNode::new("A".to_string()), DefaultNode::new("B".to_string())],
    ///     vec![DirectedEdge::new(
    ///         DefaultNode::new("A".to_string()),
    ///         DefaultNode::new("B".to_string()),
    ///         6,
    ///     )],
    /// );
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
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    /// use shortest_path_finder::graphs::graph::Graph;
    ///
    /// let graph = DirectedGraph::new(vec![], vec![]);
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
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    /// use shortest_path_finder::graphs::graph::Graph;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let mut graph = DirectedGraph::new(vec![], vec![]);
    /// graph.insert_node(DefaultNode::new("A".to_string()));
    /// assert!(graph.get_node_by_id("A").is_some());
    /// ```
    fn insert_node(&mut self, new_node: Self::Node);

    /// Inserts an edge into the graph.
    ///
    /// # Parameters
    ///
    /// - `edge`: Edge to add.
    ///
    /// # Returns
    ///
    /// - `None` if insertion succeeded.
    /// - `Some(Self::InsertionError)` if insertion failed.
    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError>;

    /// Checks whether a semantically equivalent edge already exists.
    ///
    /// # Parameters
    ///
    /// - `edge`: Candidate edge.
    ///
    /// # Returns
    ///
    /// `true` if an equivalent edge is already present.
    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool;

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
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    /// use shortest_path_finder::graphs::graph::{Graph, GraphNode};
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let node = DefaultNode::new("A".to_string());
    /// let graph = DirectedGraph::new(vec![node], vec![]);
    /// assert_eq!(graph.get_node_by_id("A").unwrap().get_id(), "A");
    /// assert!(graph.get_node_by_id("missing").is_none());
    /// ```
    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node>;

    /// Retrieves an edge by ID.
    ///
    /// # Parameters
    ///
    /// - `id`: Edge identifier.
    ///
    /// # Returns
    ///
    /// - `Some(&Self::Edge)` if found.
    /// - `None` otherwise.
    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<&Self::Edge>;

    /// Returns all nodes currently contained in the graph.
    ///
    /// # Returns
    ///
    /// Borrowed vector of all graph nodes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    /// use shortest_path_finder::graphs::graph::Graph;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let graph = DirectedGraph::new(
    ///     vec![DefaultNode::new("A".to_string()), DefaultNode::new("B".to_string())],
    ///     vec![],
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
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    /// use shortest_path_finder::graphs::graph::Graph;
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
/// - and support addition with `Add<Output = Self>`, allowing weights to be combined.
///
/// # Usage
/// Use `GraphWeight` as a trait bound for generic types in graph algorithms,
/// such as shortest path or minimum spanning tree implementations,
/// where weights need to be comparable, clonable, and combinable via addition.
///
/// # Example
/// ```rust
/// use std::ops::Add;
/// use shortest_path_finder::graphs::graph::GraphWeight;
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
}

/// Trait for graph edges with stable identifiers.
///
/// Edges are required to be cloneable and comparable so graph containers can
/// perform duplicate checks and return references safely.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::graphs::graph::GraphEdge;
///
/// #[derive(Clone, PartialEq)]
/// struct Edge {
///     id: u64,
/// }
///
/// impl GraphEdge for Edge {
///     type ID = u64;
///
///     fn get_id(&self) -> Self::ID {
///         self.id
///     }
/// }
///
/// let edge = Edge { id: 42 };
/// assert_eq!(edge.get_id(), 42);
/// ```
pub trait GraphEdge: Clone + PartialEq {
    /// Identifier type of the edge.
    type ID: Eq + PartialEq + Copy;

    /// Returns the edge identifier.
    fn get_id(&self) -> Self::ID;
}

/// Trait for node values stored in graph implementations.
///
/// A node must have a stable textual identifier retrievable via [`GraphNode::get_id`].
///
/// # Example
///
/// ```rust
/// use std::fmt::{Display, Formatter};
/// use shortest_path_finder::graphs::graph::GraphNode;
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
