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
/// - Duplicate edges are rejected regardless of endpoint order (`A-B` equals `B-A`).
/// - Edges can only be inserted if both endpoint nodes already exist.
/// - Neighbor traversal is backed by an index-based adjacency list.
#[derive(Debug, Clone)]
pub struct UndirectedGraph {
    /// Nodes currently contained in the graph.
    pub nodes: Vec<DefaultNode>,
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
            // Check for an edge in either direction since the graph is undirected.
            if self.adjacency[from_index]
                .iter()
                .any(|(neighbor_index, _)| *neighbor_index == to_index)
                && self.adjacency[to_index]
                    .iter()
                    .any(|(neighbor_index, _)| *neighbor_index == from_index)
            {
                return true;
            };
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
            return Some(UndirectedGraphInsertionError::new(format!(
                "The edge between '{}' and '{}' already exists in the graph!",
                from.get_id(),
                to.get_id()
            )));
        }

        let a_index = match self.node_index_for_id(from.get_id()) {
            Some(index) => index,
            None => {
                return Some(UndirectedGraphInsertionError::new(format!(
                    "The node '{}' in the edge from {} to {} isn't part of the graph!",
                    from.get_id(),
                    from.get_id(),
                    to.get_id()
                )));
            }
        };

        let b_index = match self.node_index_for_id(to.get_id()) {
            Some(index) => index,
            None => {
                return Some(UndirectedGraphInsertionError::new(format!(
                    "The node '{}' in the edge from {} to {} isn't part of the graph!",
                    to.get_id(),
                    from.get_id(),
                    to.get_id()
                )));
            }
        };

        let weight = match weight {
            Some(w) => w,
            None => {
                return Some(UndirectedGraphInsertionError::new(format!(
                    "The edge from {} to {} is missing a weight!",
                    from.get_id(),
                    to.get_id()
                )));
            }
        };

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

    /// Rebuilds lookup and adjacency structures from current nodes.
    ///
    /// # Behavior
    ///
    /// - Recomputes `node_index_by_id` from the current `nodes` vector.
    /// - Reinitializes adjacency with one bucket per node.
    fn prepare_internal_adjacency(&mut self) {
        // Re-index nodes to keep ID lookups in sync with vector positions.
        self.node_index_by_id = self
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.get_id().to_string(), index))
            .collect();

        // Allocate one neighbor bucket for each node index.
        self.adjacency = vec![Vec::new(); self.nodes.len()];
    }

    /// Creates a new undirected graph from a node vector.
    ///
    /// # Arguments
    ///
    /// - `nodes`: list of graph nodes.
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
    /// let graph = UndirectedGraph::new(vec![]);
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
