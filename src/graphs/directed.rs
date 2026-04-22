//! Directed graph implementation.
//!
//! # Overview
//!
//! This module provides a concrete weighted directed graph type:
//! - [`DirectedGraph`] stores [`DefaultNode`] values and [`DirectedEdge`] edges.
//! - [`DirectedGraphInsertionError`] reports insertion failures.
//!
//! It implements the shared [`Graph`](crate::graphs::graph::Graph) trait and
//! is used by shortest-path algorithms such as Dijkstra.
//!
//! # File Abbreviation
//!
//! The graph abbreviation used in file input is `D`.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::directed::{DirectedEdge, DirectedGraph};
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let mut graph = DirectedGraph::default();
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! graph.insert_node(a.clone());
//! graph.insert_node(b.clone());
//! assert!(graph.insert_edge(DirectedEdge::new(a, b, 5)).is_none());
//! assert!(graph.is_directed());
//! ```

use std::{collections::HashMap, error::Error, fmt::Display};

use log::info;
use uuid::Uuid;

use crate::{
    graphs::graph::{Graph, GraphEdge, GraphNode},
    nodes::default_node::DefaultNode,
};

/// Directed weighted graph using [`DefaultNode`] nodes and [`DirectedEdge`] edges.
///
/// # File-format marker
///
/// This graph is represented by the abbreviation `D` in file-input headers.
///
/// # Invariants
///
/// - Duplicate nodes are ignored on insertion.
/// - Duplicate edges (same `from` and `to`) are rejected.
/// - Edges can only be inserted if both endpoint nodes exist in the graph.
/// - Neighbor traversal is backed by an index-based adjacency list.
///
/// # Example
/// ```
/// use shortest_path_finder::graphs::directed::{DirectedEdge, DirectedGraph};
/// use shortest_path_finder::nodes::default_node::DefaultNode;
///
/// let graph = DirectedGraph::new(
///     vec![
///         DefaultNode::new("A".to_string()),
///         DefaultNode::new("B".to_string()),
///     ],
///     vec![DirectedEdge::new(
///         DefaultNode::new("A".to_string()),
///         DefaultNode::new("B".to_string()),
///         4,
///     )],
/// );
/// assert_eq!(graph.nodes.len(), 2);
/// assert_eq!(graph.edges.len(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct DirectedGraph {
    /// All nodes currently contained in the graph.
    pub nodes: Vec<DefaultNode>,
    /// All directed edges currently contained in the graph.
    pub edges: Vec<DirectedEdge>,
    /// Fast ID-to-index lookup for node access.
    node_index_by_id: HashMap<String, usize>,
    /// Adjacency list storing `(to_index, weight)` for each source node index.
    adjacency: Vec<Vec<(usize, u16)>>,
}

impl Graph for DirectedGraph {
    type Node = DefaultNode;

    type Weight = u16;

    type Edge = DirectedEdge;

    type InsertionError = DirectedGraphInsertionError;

    fn is_directed(&self) -> bool {
        true
    }

    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a> {
        let Some(source_index) = self.node_index_for_id(u.get_id()) else {
            return Box::new(std::iter::empty());
        };

        Box::new(
            self.adjacency[source_index]
                .iter()
                .map(move |(neighbor_index, weight)| (&self.nodes[*neighbor_index], *weight)),
        )
    }

    fn insert_node(&mut self, new_node: Self::Node) {
        if self.does_node_already_exist(&new_node) {
            return;
        }

        let new_index = self.nodes.len();
        self.node_index_by_id
            .insert(new_node.get_id().to_string(), new_index);
        self.nodes.push(new_node);
        self.adjacency.push(Vec::new());
    }

    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError> {
        if self.does_edge_already_exist(&edge) {
            return Some(DirectedGraphInsertionError::new(format!(
                "The edge {} already exists in the graph!",
                edge
            )));
        }

        let from_index = match self.node_index_for_id(edge.from_id()) {
            Some(index) => index,
            None => {
                return Some(DirectedGraphInsertionError::new(format!(
                    "The source node '{}' in edge {} doesn't exist!",
                    edge.from_id(),
                    edge
                )));
            }
        };
        let to_index = match self.node_index_for_id(edge.to_id()) {
            Some(index) => index,
            None => {
                return Some(DirectedGraphInsertionError::new(format!(
                    "The destination node '{}' in edge {} doesn't exist!",
                    edge.to_id(),
                    edge
                )));
            }
        };

        self.adjacency[from_index].push((to_index, edge.weight));
        self.edges.push(edge);

        None
    }

    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {
        for existing in &self.edges {
            if existing.from_id == edge.from_id && existing.to_id == edge.to_id {
                return true;
            }
        }
        false
    }

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        self.node_index_by_id.contains_key(node.get_id())
    }

    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<&Self::Edge> {
        self.edges
            .iter()
            .find(|&e| &e.get_id() == id)
            .map(|v| v as _)
    }

    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node> {
        self.node_index_by_id
            .get(id)
            .and_then(|&index| self.nodes.get(index))
    }

    fn get_all_nodes(&self) -> &Vec<Self::Node> {
        &self.nodes
    }

    fn is_weighted(&self) -> bool {
        true
    }

    fn abbreviation() -> String {
        String::from("D")
    }
}

impl DirectedGraph {
    /// Looks up the index of a node by its string identifier.
    ///
    /// # Parameters
    ///
    /// - `id`: Node identifier to resolve.
    ///
    /// # Returns
    ///
    /// - `Some(index)` when the node exists.
    /// - `None` when the node ID is unknown.
    fn node_index_for_id(&self, id: &str) -> Option<usize> {
        self.node_index_by_id.get(id).copied()
    }

    /// Rebuilds the internal node index map and adjacency list from edge data.
    ///
    /// # Why this exists
    ///
    /// Constructors can receive pre-populated `nodes` and `edges`. This helper
    /// restores derived lookup structures so read operations (`neighbors`,
    /// `get_node_by_id`) remain fast and consistent.
    ///
    /// # Behavior
    ///
    /// - Recomputes `node_index_by_id` from current node order.
    /// - Resets adjacency to one bucket per node.
    /// - Replays each edge into adjacency, skipping dangling endpoints.
    fn rebuild_internal_adjacency(&mut self) {
        // Build a stable id -> index lookup table from the current node vector.
        self.node_index_by_id = self
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.get_id().to_string(), index))
            .collect();

        // Pre-allocate one adjacency bucket per node.
        self.adjacency = vec![Vec::new(); self.nodes.len()];

        for edge in &self.edges {
            // Skip invalid edges that reference nodes no longer present.
            let Some(from_index) = self.node_index_for_id(edge.from_id()) else {
                continue;
            };
            let Some(to_index) = self.node_index_for_id(edge.to_id()) else {
                continue;
            };

            // Directed graph: only one direction gets inserted.
            self.adjacency[from_index].push((to_index, edge.weight));
        }
    }

    /// Creates a new directed graph from node and edge vectors.
    ///
    /// # Parameters
    ///
    /// - `nodes`: initial node list.
    /// - `edges`: initial directed edge list.
    ///
    /// # Returns
    ///
    /// A new [`DirectedGraph`] instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    ///
    /// let graph = DirectedGraph::new(vec![], vec![]);
    /// assert_eq!(graph.nodes.len(), 0);
    /// assert_eq!(graph.edges.len(), 0);
    /// ```
    pub fn new(nodes: Vec<DefaultNode>, edges: Vec<DirectedEdge>) -> Self {
        let mut graph = Self {
            nodes,
            edges,
            node_index_by_id: HashMap::new(),
            adjacency: Vec::new(),
        };
        graph.rebuild_internal_adjacency();
        graph
    }
}

impl Display for DirectedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes: {:?}, Edges: {:?}", self.nodes, self.edges)
    }
}

impl Default for DirectedGraph {
    /// Creates an empty directed graph.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    ///
    /// let graph = DirectedGraph::default();
    /// assert!(graph.nodes.is_empty());
    /// assert!(graph.edges.is_empty());
    /// ```
    fn default() -> Self {
        Self::new(vec![], vec![])
    }
}

// ----- Implementation of the 'DirectedEdge' struct -----

/// Directed edge from one node to another.
///
/// # Semantics
///
/// An edge `A -> B` is not equivalent to `B -> A`.
///
/// # Fields
///
/// - [`DirectedEdge::from_id`]: source node ID.
/// - [`DirectedEdge::to_id`]: destination node ID.
/// - [`DirectedEdge::weight`]: edge traversal cost.
#[derive(Clone, PartialEq, Debug)]
pub struct DirectedEdge {
    /// Source node ID of the directed edge.
    pub from_id: String,
    /// Destination node ID of the directed edge.
    pub to_id: String,
    /// Cost/weight associated with traversing this edge.
    pub weight: u16,
    id: Uuid,
}

impl DirectedEdge {
    /// Creates a new directed edge.
    ///
    /// # Parameters
    ///
    /// - `from`: edge source node.
    /// - `to`: edge destination node.
    /// - `weight`: edge weight.
    ///
    /// # Returns
    ///
    /// A new [`DirectedEdge`] with a generated UUID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::directed::DirectedEdge;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let edge = DirectedEdge::new(
    ///     DefaultNode::new("A".to_string()),
    ///     DefaultNode::new("B".to_string()),
    ///     8,
    /// );
    ///
    /// assert_eq!(edge.weight, 8);
    /// assert_eq!(edge.from_id(), "A");
    /// assert_eq!(edge.to_id(), "B");
    /// ```
    pub fn new(from: DefaultNode, to: DefaultNode, weight: u16) -> Self {
        Self {
            from_id: from.id,
            to_id: to.id,
            weight,
            id: Uuid::new_v4(),
        }
    }

    /// Returns the source node ID.
    pub fn from_id(&self) -> &str {
        &self.from_id
    }

    /// Returns the destination node ID.
    pub fn to_id(&self) -> &str {
        &self.to_id
    }
}

impl Display for DirectedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n            from: {},\n            to: {},\n            weight: {}\n        ",
            self.from_id, self.to_id, self.weight
        )
    }
}

impl GraphEdge for DirectedEdge {
    type ID = Uuid;

    fn get_id(&self) -> Self::ID {
        self.id
    }
}

// ----- Implementation of the 'DirectedGraphInsertionError' struct -----

/// Error returned when inserting nodes/edges into [`DirectedGraph`] fails.
///
/// # Typical causes
///
/// - duplicate edge insertion,
/// - inserting an edge whose endpoint node does not exist.
#[derive(Debug)]
pub struct DirectedGraphInsertionError {
    /// Human-readable description of the insertion failure.
    pub message: String,
}

impl DirectedGraphInsertionError {
    /// Creates a new insertion error with a descriptive message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::directed::DirectedGraphInsertionError;
    ///
    /// let err = DirectedGraphInsertionError::new("duplicate edge".to_string());
    /// assert_eq!(err.to_string(), "duplicate edge");
    /// ```
    pub fn new(message: String) -> Self {
        DirectedGraphInsertionError { message }
    }

    /// Logs the error message using the crate logger.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::directed::DirectedGraphInsertionError;
    ///
    /// let err = DirectedGraphInsertionError::new("duplicate edge".to_string());
    /// err.display();
    /// ```
    pub fn display(&self) {
        info!("{}", self.message)
    }
}

impl Display for DirectedGraphInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DirectedGraphInsertionError {}
