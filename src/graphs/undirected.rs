//! Undirected graph implementation.
//!
//! # Overview
//!
//! This module provides:
//! - [`UndirectedGraph`] as a weighted, non-directional graph container,
//! - adjacency lists for undirected neighbor traversal,
//! - [`UndirectedGraphInsertionError`] for insertion failures.
//!
//! # File Abbreviation
//!
//! The graph abbreviation used in file input is `UN`.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::graphs::undirected::UndirectedGraph;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let mut graph = UndirectedGraph::default();
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! graph.insert_node(a.clone());
//! graph.insert_node(b.clone());
//! assert!(graph.insert_edge(&a, &b, Some(2)).is_none());
//! assert!(!graph.is_directed());
//! ```

use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{
    graphs::graph::{Graph, GraphNode},
    nodes::default_node::DefaultNode,
};

/// Undirected weighted graph implementation.
///
/// # File-format marker
///
/// This graph is represented by the abbreviation `UN` in file-input headers.
///
/// # Invariants
///
/// - Duplicate nodes are ignored on insertion.
/// - Duplicate nodes provided at construction time are ignored.
/// - Duplicate edges are rejected regardless of endpoint order (`A-B` equals `B-A`).
/// - Edges can only be inserted if both endpoint nodes already exist.
/// - Self-loop edges are stored once.
/// - Neighbor traversal is backed by an index-based adjacency list.
#[derive(Debug, Clone)]
pub struct UndirectedGraph {
    /// Nodes currently contained in the graph.
    nodes: Vec<DefaultNode>,
    /// Fast ID-to-index lookup for node access.
    node_index_by_id: HashMap<String, usize>,
    /// Adjacency list storing `(neighbor_index, weight)` for each node index.
    adjacency: Vec<Vec<(usize, u16)>>,
}

impl Graph for UndirectedGraph {
    type Node = DefaultNode;

    type Weight = u16;

    type InsertionError = UndirectedGraphInsertionError;

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        self.node_index_by_id.contains_key(node.get_id())
    }

    fn does_edge_already_exist(&self, from: &Self::Node, to: &Self::Node) -> bool {
        if let (Some(from_index), Some(to_index)) = (
            self.node_index_for_id(from.get_id()),
            self.node_index_for_id(to.get_id()),
        ) {
            // Treat an edge as present if either adjacency list contains it.
            return self.adjacency[from_index]
                .iter()
                .any(|(neighbor_index, _)| *neighbor_index == to_index)
                || self.adjacency[to_index]
                    .iter()
                    .any(|(neighbor_index, _)| *neighbor_index == from_index);
        }

        false
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

    fn is_directed(&self) -> bool {
        false
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
            return Some(UndirectedGraphInsertionError::EdgeAlreadyExists {
                from_id: from.get_id().to_string(),
                to_id: to.get_id().to_string(),
            });
        }

        let a_index = match self.node_index_for_id(from.get_id()) {
            Some(index) => index,
            None => {
                return Some(UndirectedGraphInsertionError::NodeNotFound {
                    node_id: from.get_id().to_string(),
                });
            }
        };

        let b_index = match self.node_index_for_id(to.get_id()) {
            Some(index) => index,
            None => {
                return Some(UndirectedGraphInsertionError::NodeNotFound {
                    node_id: to.get_id().to_string(),
                });
            }
        };

        let weight = match weight {
            Some(w) => w,
            None => {
                return Some(UndirectedGraphInsertionError::MissingWeight {
                    from_id: from.get_id().to_string(),
                    to_id: to.get_id().to_string(),
                });
            }
        };

        if a_index == b_index {
            self.adjacency[a_index].push((b_index, weight));
            return None;
        }

        self.adjacency[a_index].push((b_index, weight));
        self.adjacency[b_index].push((a_index, weight));

        None
    }
    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node> {
        self.node_index_by_id
            .get(id)
            .and_then(|&index| self.nodes.get(index))
    }

    fn get_all_nodes(&self) -> &Vec<DefaultNode> {
        &self.nodes
    }

    fn is_weighted(&self) -> bool {
        true
    }

    fn abbreviation() -> String {
        String::from("UN")
    }
}

impl UndirectedGraph {
    /// Looks up the index of a node by its identifier.
    ///
    /// # Parameters
    ///
    /// - `id`: Node identifier to resolve.
    ///
    /// # Returns
    ///
    /// - `Some(index)` when `id` exists in the graph.
    /// - `None` otherwise.
    fn node_index_for_id(&self, id: &str) -> Option<usize> {
        self.node_index_by_id.get(id).copied()
    }

    /// Creates a new undirected graph from a node vector.
    ///
    /// # Arguments
    ///
    /// - `nodes`: list of graph nodes.
    ///   Duplicate node IDs are ignored.
    ///
    /// # Returns
    ///
    /// A new [`UndirectedGraph`] value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::graph::Graph;
    /// use shortest_path_finder::graphs::undirected::UndirectedGraph;
    ///
    /// let graph = UndirectedGraph::new(vec![]);
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

impl Display for UndirectedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "UndirectedGraph with {} nodes:", self.nodes.len())?;
        for node in &self.nodes {
            writeln!(f, "- Node '{}'", node.get_id())?;
        }
        writeln!(f, "Edges:")?;
        for (index, neighbors) in self.adjacency.iter().enumerate() {
            let node_id = &self.nodes[index].get_id();
            for (neighbor_index, weight) in neighbors {
                let neighbor_id = &self.nodes[*neighbor_index].get_id();
                writeln!(f, "- {} --({})--> {}", node_id, weight, neighbor_id)?;
            }
        }
        Ok(())
    }
}

impl Default for UndirectedGraph {
    fn default() -> Self {
        Self::new(vec![])
    }
}

// ----- Implementation of the 'UndirectedGraphInsertionError' enum -----

/// Errors encountered when mutating an [`UndirectedGraph`].
///
/// # When this type is returned
///
/// The undirected graph rejects an insertion when one of the following is
/// true:
///
/// - the edge already exists in either direction,
/// - one of the endpoint nodes is missing,
/// - the caller omits the required edge weight.
///
/// # Variant guide
///
/// - [`UndirectedGraphInsertionError::EdgeAlreadyExists`][]: the requested
///   undirected edge is already present.
/// - [`UndirectedGraphInsertionError::NodeNotFound`][]: one of the endpoint nodes
///   must be inserted before retrying the edge insertion.
/// - [`UndirectedGraphInsertionError::MissingWeight`][]: the graph requires an
///   explicit weight and the caller passed `None`.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::graphs::undirected::UndirectedGraphInsertionError;
///
/// let err = UndirectedGraphInsertionError::NodeNotFound {
///     node_id: "B".to_string(),
/// };
///
/// assert!(err.to_string().contains("was not found in the graph"));
/// ```
///
/// Error returned when undirected graph insertion fails.
///
/// # Variants
///
/// - `EdgeAlreadyExists`: the edge between the two nodes already exists.
/// - `NodeNotFound`: a node is missing from the graph.
/// - `MissingWeight`: the weight for the edge was not provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UndirectedGraphInsertionError {
    /// The undirected edge between the two nodes already exists.
    ///
    /// The graph treats `from_id -> to_id` and `to_id -> from_id` as the same
    /// edge, so either orientation can trigger this error.
    EdgeAlreadyExists {
        /// ID of the first endpoint reported by the failed insertion.
        from_id: String,
        /// ID of the second endpoint reported by the failed insertion.
        to_id: String,
    },
    /// One of the endpoint nodes is missing from the graph.
    ///
    /// Insert the missing node before retrying the edge insertion.
    NodeNotFound {
        /// ID of the missing node.
        node_id: String,
    },
    /// The caller omitted a weight for an edge that requires one.
    ///
    /// Because the graph is weighted, `None` is not a valid insertion weight.
    MissingWeight {
        /// ID of the first endpoint.
        from_id: String,
        /// ID of the second endpoint.
        to_id: String,
    },
}

impl Display for UndirectedGraphInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UndirectedGraphInsertionError::EdgeAlreadyExists { from_id, to_id } => {
                write!(
                    f,
                    "The edge between '{}' and '{}' already exists in the graph!",
                    from_id, to_id
                )
            }
            UndirectedGraphInsertionError::NodeNotFound { node_id } => {
                write!(
                    f,
                    "The node with ID '{}' was not found in the graph!",
                    node_id
                )
            }
            UndirectedGraphInsertionError::MissingWeight { from_id, to_id } => {
                write!(
                    f,
                    "The edge from '{}' to '{}' is missing a weight!",
                    from_id, to_id
                )
            }
        }
    }
}

impl Error for UndirectedGraphInsertionError {}
