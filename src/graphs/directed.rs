//! Directed graph implementation.
//!
//! # Overview
//!
//! This module provides a concrete weighted directed graph type:
//! - [`DirectedGraph`] stores [`DefaultNode`] values and adjacency data.
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
//! use shortest_path_finder::graphs::directed::DirectedGraph;
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let mut graph = DirectedGraph::default();
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! graph.insert_node(a.clone());
//! graph.insert_node(b.clone());
//! assert!(graph.insert_edge(&a, &b, Some(5)).is_none());
//! assert!(graph.is_directed());
//! ```

use std::{collections::HashMap, error::Error, fmt::Display};

use log::info;

use crate::{
    graphs::graph::{Graph, GraphNode},
    nodes::default_node::DefaultNode,
};

/// Directed weighted graph using [`DefaultNode`] nodes and adjacency lists.
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
/// use shortest_path_finder::graphs::directed::DirectedGraph;
/// use shortest_path_finder::graphs::graph::Graph;
/// use shortest_path_finder::nodes::default_node::DefaultNode;
///
/// let mut graph = DirectedGraph::new(vec![
///     DefaultNode::new("A".to_string()),
///     DefaultNode::new("B".to_string()),
/// ]);
/// let from = DefaultNode::new("A".to_string());
/// let to = DefaultNode::new("B".to_string());
/// graph.insert_edge(&from, &to, Some(4));
/// assert_eq!(graph.nodes.len(), 2);
/// assert_eq!(graph.neighbors(&from).count(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct DirectedGraph {
    /// All nodes currently contained in the graph.
    pub nodes: Vec<DefaultNode>,
    /// Fast ID-to-index lookup for node access.
    node_index_by_id: HashMap<String, usize>,
    /// Adjacency list storing `(to_index, weight)` for each source node index.
    adjacency: Vec<Vec<(usize, u16)>>,
}

impl Graph for DirectedGraph {
    type Node = DefaultNode;

    type Weight = u16;

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

    fn insert_edge(
        &mut self,
        from: &Self::Node,
        to: &Self::Node,
        weight: Option<Self::Weight>,
    ) -> Option<Self::InsertionError> {
        if self.does_edge_already_exist(from, to) {
            return Some(DirectedGraphInsertionError::new(format!(
                "Edge from '{}' to '{}' already exists!",
                from.get_id(),
                to.get_id()
            )));
        }

        let from_index = match self.node_index_for_id(from.get_id()) {
            Some(index) => index,
            None => {
                return Some(DirectedGraphInsertionError::new(format!(
                    "The source node '{}' in edge from '{}' to '{}' doesn't exist!",
                    from.get_id(),
                    from.get_id(),
                    to.get_id()
                )));
            }
        };
        let to_index = match self.node_index_for_id(to.get_id()) {
            Some(index) => index,
            None => {
                return Some(DirectedGraphInsertionError::new(format!(
                    "The destination node '{}' in edge from '{}' to '{}' doesn't exist!",
                    to.get_id(),
                    from.get_id(),
                    to.get_id()
                )));
            }
        };

        let weight = match weight {
            Some(w) => w,
            None => {
                return Some(DirectedGraphInsertionError::new(format!(
                    "Edge from '{}' to '{}' must have a weight!",
                    from.get_id(),
                    to.get_id()
                )));
            }
        };

        self.adjacency[from_index].push((to_index, weight));

        None
    }

    fn does_edge_already_exist(&self, from: &Self::Node, to: &Self::Node) -> bool {
        if let (Some(from_index), Some(to_index)) = (
            self.node_index_for_id(from.get_id()),
            self.node_index_for_id(to.get_id()),
        ) {
            return self.adjacency[from_index]
                .iter()
                .any(|(neighbor_index, _)| *neighbor_index == to_index);
        }
        false
    }

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        self.node_index_by_id.contains_key(node.get_id())
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

    /// Rebuilds the internal node index map and adjacency list from node data.
    ///
    /// # Why this exists
    ///
    /// Constructors can receive pre-populated `nodes`. This helper restores
    /// derived lookup structures so read operations (`neighbors`, `get_node_by_id`)
    /// remain fast and consistent.
    ///
    /// # Behavior
    ///
    /// - Recomputes `node_index_by_id` from current node order.
    /// - Resets adjacency to one bucket per node.
    fn prepare_internal_adjacency(&mut self) {
        // Build a stable id -> index lookup table from the current node vector.
        self.node_index_by_id = self
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.get_id().to_string(), index))
            .collect();

        // Pre-allocate one adjacency bucket per node.
        self.adjacency = vec![Vec::new(); self.nodes.len()];
    }

    /// Creates a new directed graph from a node vector.
    ///
    /// # Parameters
    ///
    /// - `nodes`: initial node list.
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
    /// let graph = DirectedGraph::new(vec![]);
    /// assert_eq!(graph.nodes.len(), 0);
    /// assert_eq!(graph.nodes.len(), 0);
    /// ```
    pub fn new(nodes: Vec<DefaultNode>) -> Self {
        let mut graph = Self {
            nodes,
            node_index_by_id: HashMap::new(),
            adjacency: Vec::new(),
        };
        graph.prepare_internal_adjacency();
        graph
    }
}

impl Display for DirectedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Nodes: {:?}, Adjacency: {:?}",
            self.nodes, self.adjacency
        )
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
    /// ```
    fn default() -> Self {
        Self::new(vec![])
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
