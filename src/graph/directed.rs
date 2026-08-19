//! Directed graph implementation.
//!
//! # Overview
//!
//! This module provides a concrete weighted directed graph type:
//! - [`DirectedGraph`] stores [`DefaultNode`] values and adjacency data.
//! - [`DirectedGraphInsertionError`] reports insertion failures.
//!
//! It implements the shared [`crate::graphs::graph::Graph`] trait and
//! is used by shortest-path algorithms such as Dijkstra.
//!
//! # File Abbreviation
//!
//! The graph abbreviation used in file input is `D`.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graph::DirectedGraph;
//! use shortest_path_finder::graph::Graph;
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

use crate::{
    graph::{Graph, GraphNode},
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
/// - Duplicate nodes provided at construction time are ignored.
/// - Duplicate edges (same `from` and `to`) are rejected.
/// - Edges can only be inserted if both endpoint nodes exist in the graph.
/// - Neighbor traversal is backed by an index-based adjacency list.
///
/// # Example
/// ```
/// use shortest_path_finder::graph::DirectedGraph;
/// use shortest_path_finder::graph::Graph;
/// use shortest_path_finder::nodes::default_node::DefaultNode;
///
/// let mut graph = DirectedGraph::new(vec![
///     DefaultNode::new("A".to_string()),
///     DefaultNode::new("B".to_string()),
/// ]);
/// let from = DefaultNode::new("A".to_string());
/// let to = DefaultNode::new("B".to_string());
/// graph.insert_edge(&from, &to, Some(4));
/// assert_eq!(graph.get_all_nodes().len(), 2);
/// assert_eq!(graph.neighbors(&from).count(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct DirectedGraph {
    /// All nodes currently contained in the graph.
    nodes: Vec<DefaultNode>,
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
            return Some(DirectedGraphInsertionError::EdgeAlreadyExists {
                from: from.get_id().to_string(),
                to: to.get_id().to_string(),
            });
        }

        let from_index = match self.node_index_for_id(from.get_id()) {
            Some(index) => index,
            None => {
                return Some(DirectedGraphInsertionError::SourceNodeDoesNotExist {
                    node_id: from.get_id().to_string(),
                });
            }
        };
        let to_index = match self.node_index_for_id(to.get_id()) {
            Some(index) => index,
            None => {
                return Some(DirectedGraphInsertionError::DestinationNodeDoesNotExist {
                    node_id: to.get_id().to_string(),
                });
            }
        };

        let weight = match weight {
            Some(w) => w,
            None => {
                return Some(DirectedGraphInsertionError::MissingEdgeWeight {
                    expected_weight: "u16".to_string(),
                });
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

    /// Creates a new directed graph from a node vector.
    ///
    /// # Parameters
    ///
    /// - `nodes`: initial node list.
    ///   Duplicate node IDs are ignored.
    ///
    /// # Returns
    ///
    /// A new [`DirectedGraph`] instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graph::DirectedGraph;
    /// use shortest_path_finder::graph::Graph;
    ///
    /// let graph = DirectedGraph::new(vec![]);
    /// assert_eq!(graph.get_all_nodes().len(), 0);
    /// assert_eq!(graph.get_all_nodes().len(), 0);
    /// ```
    pub fn new(nodes: Vec<DefaultNode>) -> Self {
        let mut graph = Self {
            nodes: Vec::new(),
            node_index_by_id: HashMap::new(),
            adjacency: Vec::new(),
        };

        for node in nodes {
            graph.insert_node(node);
        }

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
    /// use shortest_path_finder::graph::DirectedGraph;
    /// use shortest_path_finder::graph::Graph;
    ///
    /// let graph = DirectedGraph::default();
    /// assert!(graph.get_all_nodes().is_empty());
    /// ```
    fn default() -> Self {
        Self::new(vec![])
    }
}

// ----- Implementation of the 'DirectedGraphInsertionError' enum -----

/// Errors encountered when mutating a [`DirectedGraph`].
///
/// # When this type is returned
///
/// The directed graph rejects an insertion when one of the following is true:
///
/// - the edge already exists,
/// - the source node is missing,
/// - the destination node is missing,
/// - the caller omits a required edge weight.
///
/// # Variant guide
///
/// - [`DirectedGraphInsertionError::EdgeAlreadyExists`][]: the exact `from -> to`
///   edge is already present.
/// - [`DirectedGraphInsertionError::SourceNodeDoesNotExist`][]: the source node
///   must be inserted before retrying the edge insertion.
/// - [`DirectedGraphInsertionError::DestinationNodeDoesNotExist`][]: the
///   destination node must be inserted before retrying the edge insertion.
/// - [`DirectedGraphInsertionError::MissingEdgeWeight`][]: the graph expects a
///   weight and the caller passed `None`.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::graph::directed::DirectedGraphInsertionError;
///
/// let err = DirectedGraphInsertionError::SourceNodeDoesNotExist {
///     node_id: "A".to_string(),
/// };
///
/// assert!(err.to_string().contains("Source node 'A' does not exist"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectedGraphInsertionError {
    /// The directed edge from `from` to `to` already exists.
    ///
    /// The graph remains unchanged when this error is returned.
    EdgeAlreadyExists {
        /// ID of the source node whose outgoing edge already exists.
        from: String,
        /// ID of the destination node already targeted by `from`.
        to: String,
    },

    /// The source node does not exist in the graph yet.
    ///
    /// Insert the source node before retrying the edge insertion.
    SourceNodeDoesNotExist {
        /// ID of the missing source node.
        node_id: String,
    },

    /// The destination node does not exist in the graph yet.
    ///
    /// Insert the destination node before retrying the edge insertion.
    DestinationNodeDoesNotExist {
        /// ID of the missing destination node.
        node_id: String,
    },

    /// The caller omitted a required edge weight.
    ///
    /// The `expected_weight` text is intended for diagnostics and usually names
    /// the concrete weight type expected by the graph.
    MissingEdgeWeight {
        /// Human-readable description of the expected weight type.
        expected_weight: String,
    },
}

impl Display for DirectedGraphInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectedGraphInsertionError::EdgeAlreadyExists { from, to } => {
                write!(f, "Edge from '{}' to '{}' already exists!", from, to)
            }
            DirectedGraphInsertionError::SourceNodeDoesNotExist { node_id } => {
                write!(f, "Source node '{}' does not exist in the graph!", node_id)
            }
            DirectedGraphInsertionError::DestinationNodeDoesNotExist { node_id } => {
                write!(
                    f,
                    "Destination node '{}' does not exist in the graph!",
                    node_id
                )
            }
            DirectedGraphInsertionError::MissingEdgeWeight { expected_weight } => {
                write!(f, "Missing edge weight: {}", expected_weight)
            }
        }
    }
}

impl Error for DirectedGraphInsertionError {}
