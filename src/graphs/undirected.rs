//! Undirected graph implementation.
//!
//! # Overview
//!
//! This module provides:
//! - [`UndirectedGraph`] as a weighted, non-directional graph container,
//! - [`UndirectedEdge`] as an edge connecting two nodes symmetrically,
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
//! use shortest_path_finder::graphs::undirected::{UndirectedEdge, UndirectedGraph};
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let mut graph = UndirectedGraph::default();
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! graph.insert_node(a.clone());
//! graph.insert_node(b.clone());
//! assert!(graph.insert_edge(UndirectedEdge::new(a, b, 2)).is_none());
//! assert!(!graph.is_directed());
//! ```

use std::{collections::HashMap, error::Error, fmt::Display};

use uuid::Uuid;

use crate::{
    graphs::graph::{Graph, GraphEdge, GraphNode},
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
/// - Duplicate edges are rejected regardless of endpoint order (`A-B` equals `B-A`).
/// - Edges can only be inserted if both endpoint nodes already exist.
/// - Neighbor traversal is backed by an index-based adjacency list.
#[derive(Debug, Clone)]
pub struct UndirectedGraph {
    /// Nodes currently contained in the graph.
    pub nodes: Vec<DefaultNode>,
    /// Undirected edges currently contained in the graph.
    pub edges: Vec<UndirectedEdge>,
    /// Fast ID-to-index lookup for node access.
    node_index_by_id: HashMap<String, usize>,
    /// Adjacency list storing `(neighbor_index, weight)` for each node index.
    adjacency: Vec<Vec<(usize, u16)>>,
}

impl Graph for UndirectedGraph {
    type Node = DefaultNode;

    type Edge = UndirectedEdge;

    type Weight = u16;

    type InsertionError = UndirectedGraphInsertionError;

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        self.node_index_by_id.contains_key(node.get_id())
    }

    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {
        for existing in &self.edges {
            if (existing.a_node_id == edge.a_node_id && existing.b_node_id == edge.b_node_id)
                || (existing.b_node_id == edge.a_node_id && existing.a_node_id == edge.b_node_id)
            {
                return true;
            }
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

    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError> {
        if self.does_edge_already_exist(&edge) {
            return Some(UndirectedGraphInsertionError::new(format!(
                "The edge {} already exists in the graph!",
                edge
            )));
        }

        let a_index = match self.node_index_for_id(edge.a_node_id()) {
            Some(index) => index,
            None => {
                return Some(UndirectedGraphInsertionError::new(format!(
                    "The node '{}' in edge {} isn't part of the graph!",
                    edge.a_node_id(),
                    edge
                )));
            }
        };
        let b_index = match self.node_index_for_id(edge.b_node_id()) {
            Some(index) => index,
            None => {
                return Some(UndirectedGraphInsertionError::new(format!(
                    "The node '{}' in edge {} isn't part of the graph!",
                    edge.b_node_id(),
                    edge
                )));
            }
        };

        self.adjacency[a_index].push((b_index, edge.weight));
        self.adjacency[b_index].push((a_index, edge.weight));
        self.edges.push(edge);

        None
    }

    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node> {
        self.node_index_by_id
            .get(id)
            .and_then(|&index| self.nodes.get(index))
    }

    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<&Self::Edge> {
        self.edges.iter().find(|&e| &e.id == id).map(|v| v as _)
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
    fn node_index_for_id(&self, id: &str) -> Option<usize> {
        self.node_index_by_id.get(id).copied()
    }

    fn rebuild_internal_adjacency(&mut self) {
        self.node_index_by_id = self
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.get_id().to_string(), index))
            .collect();

        self.adjacency = vec![Vec::new(); self.nodes.len()];

        for edge in &self.edges {
            let Some(a_index) = self.node_index_for_id(edge.a_node_id()) else {
                continue;
            };
            let Some(b_index) = self.node_index_for_id(edge.b_node_id()) else {
                continue;
            };

            self.adjacency[a_index].push((b_index, edge.weight));
            self.adjacency[b_index].push((a_index, edge.weight));
        }
    }

    /// Creates a new undirected graph from node and edge vectors.
    ///
    /// # Arguments
    ///
    /// - `nodes`: list of graph nodes.
    /// - `edges`: list of undirected edges.
    ///
    /// # Returns
    ///
    /// A new [`UndirectedGraph`] value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::undirected::UndirectedGraph;
    ///
    /// let graph = UndirectedGraph::new(vec![], vec![]);
    /// assert_eq!(graph.nodes.len(), 0);
    /// assert_eq!(graph.edges.len(), 0);
    /// ```
    pub fn new(nodes: Vec<DefaultNode>, edges: Vec<UndirectedEdge>) -> Self {
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

impl Display for UndirectedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes: {:?}, Edges: {:?}", self.nodes, self.edges)
    }
}

impl Default for UndirectedGraph {
    fn default() -> Self {
        Self::new(vec![], vec![])
    }
}

// ----- Implementation of the 'UndirectedEdge' struct -----

/// Edge connecting two nodes in an undirected graph.
///
/// # Semantics
///
/// Endpoint order does not change edge meaning: `(A, B)` and `(B, A)` are
/// treated as equivalent by duplicate checks.
///
/// # Fields
///
/// - [`UndirectedEdge::a_node_id`]: first endpoint ID.
/// - [`UndirectedEdge::b_node_id`]: second endpoint ID.
/// - [`UndirectedEdge::weight`]: traversal cost.
#[derive(Clone, PartialEq, Debug)]
pub struct UndirectedEdge {
    /// First endpoint ID of the edge.
    pub a_node_id: String,
    /// Second endpoint ID of the edge.
    pub b_node_id: String,
    /// Cost/weight associated with traversing this edge.
    pub weight: u16,
    id: Uuid,
}

impl UndirectedEdge {
    /// Creates a new undirected edge.
    ///
    /// # Parameters
    ///
    /// - `a_node`: first endpoint of the edge.
    /// - `b_node`: second endpoint of the edge.
    /// - `weight`: edge weight.
    ///
    /// # Returns
    ///
    /// A new [`UndirectedEdge`] with a generated UUID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::undirected::UndirectedEdge;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let edge = UndirectedEdge::new(
    ///     DefaultNode::new("A".to_string()),
    ///     DefaultNode::new("B".to_string()),
    ///     9,
    /// );
    ///
    /// assert_eq!(edge.weight, 9);
    /// assert_eq!(edge.a_node_id(), "A");
    /// assert_eq!(edge.b_node_id(), "B");
    /// ```
    pub fn new(a_node: DefaultNode, b_node: DefaultNode, weight: u16) -> Self {
        Self {
            a_node_id: a_node.id,
            b_node_id: b_node.id,
            weight,
            id: Uuid::new_v4(),
        }
    }

    /// Returns the first endpoint ID.
    pub fn a_node_id(&self) -> &str {
        &self.a_node_id
    }

    /// Returns the second endpoint ID.
    pub fn b_node_id(&self) -> &str {
        &self.b_node_id
    }
}

impl Display for UndirectedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n            Node a: {},\n            Node b: {},\n            weight: {}\n        ",
            self.a_node_id, self.b_node_id, self.weight
        )
    }
}

impl GraphEdge for UndirectedEdge {
    type ID = Uuid;

    fn get_id(&self) -> Self::ID {
        self.id
    }
}

// ----- Implementation of the 'UndirectedGraphInsertionError' struct -----

/// Error returned when undirected graph insertion fails.
///
/// # Typical causes
///
/// - duplicate edge insertion,
/// - inserting an edge for nodes that are missing in the graph.
#[derive(Debug)]
pub struct UndirectedGraphInsertionError {
    /// Human-readable explanation of the insertion failure.
    pub message: String,
}

impl UndirectedGraphInsertionError {
    /// Creates a new insertion error with a descriptive message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::undirected::UndirectedGraphInsertionError;
    ///
    /// let err = UndirectedGraphInsertionError::new("duplicate edge".to_string());
    /// assert_eq!(err.to_string(), "duplicate edge");
    /// ```
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for UndirectedGraphInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for UndirectedGraphInsertionError {}
