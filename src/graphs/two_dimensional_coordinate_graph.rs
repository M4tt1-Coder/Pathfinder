//! Two-dimensional coordinate graph implementation.
//!
//! # Overview
//!
//! This module models a graph whose nodes carry x/y coordinates:
//! - [`TwoDimensionalCoordinateGraph`] stores nodes and computed adjacency weights.
//! - Edge weights are computed from node coordinates on insertion.
//! - [`TwoDimensionalGraphInsertionError`] reports insertion issues.
//!
//! The graph implements the shared [`Graph`](crate::graphs::graph::Graph)
//! trait and can be consumed by coordinate-aware algorithms such as A*.
//!
//! # Coordinate Type
//!
//! All main data structures in this module are generic over coordinate type
//! `C`, defaulting to `i32`:
//! - [`TwoDimensionalCoordinateGraph<C>`]
//! - [`TwoDimensionalGraphInsertionError<C>`]
//!
//! `C` must implement
//! [`CoordinateDatatype`](crate::nodes::trait_decl::coordinate_datatype::CoordinateDatatype).
//! Library users can therefore build coordinate graphs with types such as
//! `i32` or `f32`.
//!
//! # File Abbreviation
//!
//! The graph abbreviation used in file input is `TD`.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
//! use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
//!
//! let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
//! let b = TwoDimensionalNode::new(2, 3, "B".to_string()).unwrap();
//! let mut graph = TwoDimensionalCoordinateGraph::new(vec![a.clone(), b.clone()]);
//! assert!(graph.insert_edge(&a, &b, None).is_none());
//! assert!(!graph.is_directed());
//!
//! let c = TwoDimensionalNode::<f32>::new(0.5, 1.5, "C".to_string()).unwrap();
//! let d = TwoDimensionalNode::<f32>::new(1.5, 3.0, "D".to_string()).unwrap();
//! let graph_f32 = TwoDimensionalCoordinateGraph::<f32>::new(vec![c, d]);
//! assert_eq!(graph_f32.get_all_nodes().len(), 2);
//! ```

use std::{collections::HashMap, error::Error, fmt::Display};

use log::{debug, warn};

use crate::{
    graphs::{
        graph::{Graph, GraphNode},
        utils::calculate_weight,
    },
    nodes::{
        trait_decl::{coordinate_datatype::CoordinateDatatype, coordinates_node::CoordinatesNode},
        two_dimensional_node::TwoDimensionalNode,
    },
};

/// Undirected weighted graph whose nodes carry x/y coordinates.
///
/// # File-format marker
///
/// This graph is represented by `TD` in file-input headers.
///
/// # Invariants
///
/// - Duplicate nodes are rejected based on coordinates or ID.
/// - Duplicate nodes provided at construction time are ignored.
/// - Duplicate edges are rejected in either endpoint order.
/// - Self-loop edges are stored once.
/// - Explicit edge weights are ignored; weights are computed from coordinates.
/// - Edge insertion requires both endpoint nodes to already exist.
/// - Neighbor traversal is backed by an index-based adjacency list.
///
/// # Type Parameter
///
/// - `C`: coordinate scalar type used by graph nodes, defaulting to `i32`.
#[derive(Debug, Clone, Default)]
pub struct TwoDimensionalCoordinateGraph<C: CoordinateDatatype = i32> {
    /// List of graph nodes.
    nodes: Vec<TwoDimensionalNode<C>>,
    /// Fast ID-to-index lookup for node access.
    node_index_by_id: HashMap<String, usize>,
    /// Adjacency list storing `(neighbor_index, weight)` for each node index.
    adjacency: Vec<Vec<(usize, f32)>>,
}

impl<C: CoordinateDatatype> TwoDimensionalCoordinateGraph<C> {
    /// Resolves a node ID to its internal vector index.
    ///
    /// # Parameters
    ///
    /// - `id`: Node identifier.
    ///
    /// # Returns
    ///
    /// - `Some(index)` when the node is present.
    /// - `None` when the ID is unknown.
    fn node_index_for_id(&self, id: &str) -> Option<usize> {
        self.node_index_by_id.get(id).copied()
    }

    /// Creates a new two-dimensional graph from a node vector.
    ///
    /// # Arguments
    ///
    /// - `nodes`: initial node set.
    ///   Duplicate IDs or coordinates are ignored.
    ///
    /// # Returns
    ///
    /// Fresh [`TwoDimensionalCoordinateGraph`] object with initial nodes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::graph::Graph;
    /// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let node = TwoDimensionalNode::new(1, 2, "N1".to_string()).unwrap();
    /// let graph = TwoDimensionalCoordinateGraph::new(vec![node]);
    /// assert_eq!(graph.get_all_nodes().len(), 1);
    /// ```
    pub fn new(nodes: Vec<TwoDimensionalNode<C>>) -> Self {
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

impl<C: CoordinateDatatype> Graph for TwoDimensionalCoordinateGraph<C> {
    type Node = TwoDimensionalNode<C>;

    type Weight = f32;

    type InsertionError = TwoDimensionalGraphInsertionError<C>;

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
            return Some(TwoDimensionalGraphInsertionError::new(
                format!(
                    "The edge between '{}' and '{}' already exists in the graph!",
                    from.get_id(),
                    to.get_id()
                ),
                Some([from.clone(), to.clone()]),
            ));
        }

        let node_one_index = match self.node_index_for_id(from.get_id()) {
            Some(index) => index,
            None => {
                return Some(TwoDimensionalGraphInsertionError::new(
                    format!(
                        "The source node '{}' does not exist in the graph!",
                        from.get_id()
                    ),
                    Some([from.clone(), to.clone()]),
                ));
            }
        };

        let node_two_index = match self.node_index_for_id(to.get_id()) {
            Some(index) => index,
            None => {
                return Some(TwoDimensionalGraphInsertionError::new(
                    format!(
                        "The destination node '{}' does not exist in the graph!",
                        to.get_id()
                    ),
                    Some([from.clone(), to.clone()]),
                ));
            }
        };

        // Use canonical nodes from the graph to ensure weight is consistent with stored coordinates.
        let canonical_from = &self.nodes[node_one_index];
        let canonical_to = &self.nodes[node_two_index];

        let weight = match weight {
            Some(w) => {
                warn!(
                    "Explicit weight {} provided for edge from {} to {}, but coordinate graphs compute weights automatically!",
                    w, from, to
                );
                calculate_weight(canonical_from, canonical_to)
            }
            None => calculate_weight(canonical_from, canonical_to),
        };

        if node_one_index == node_two_index {
            self.adjacency[node_one_index].push((node_two_index, weight));
            return None;
        }

        self.adjacency[node_one_index].push((node_two_index, weight));
        self.adjacency[node_two_index].push((node_one_index, weight));

        None
    }

    fn is_weighted(&self) -> bool {
        true
    }

    fn get_all_nodes(&self) -> &Vec<Self::Node> {
        &self.nodes
    }

    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node> {
        self.node_index_by_id
            .get(id)
            .and_then(|&index| self.nodes.get(index))
    }

    fn does_edge_already_exist(&self, from: &Self::Node, to: &Self::Node) -> bool {
        if let Some(from_index) = self.node_index_for_id(from.get_id())
            && let Some(to_index) = self.node_index_for_id(to.get_id())
        {
            return self.adjacency[from_index]
                .iter()
                .any(|(neighbor_index, _)| *neighbor_index == to_index)
                || self.adjacency[to_index]
                    .iter()
                    .any(|(neighbor_index, _)| *neighbor_index == from_index);
        }
        false
    }

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        if self.node_index_by_id.contains_key(node.get_id()) {
            return true;
        }

        self.nodes
            .iter()
            .any(|n| n.get_y() == node.get_y() && n.get_x() == node.get_x())
    }

    fn abbreviation() -> String {
        String::from("TD")
    }
}

impl<C: CoordinateDatatype> Display for TwoDimensionalCoordinateGraph<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut graph_string = String::new();
        graph_string.push_str("TwoDimensionalCoordinateGraph {\n");
        for node in &self.nodes {
            graph_string.push_str(
                format!(
                    "  Node: {}, {}, {}\n",
                    node.get_id(),
                    node.get_x(),
                    node.get_y()
                )
                .as_str(),
            );
        }
        for (index, neighbors) in self.adjacency.iter().enumerate() {
            let node = &self.nodes[index];
            for (neighbor_index, weight) in neighbors {
                let neighbor_node = &self.nodes[*neighbor_index];
                graph_string.push_str(
                    format!(
                        "  Edge: {} --({})--> {}\n",
                        node.get_id(),
                        weight,
                        neighbor_node.get_id()
                    )
                    .as_str(),
                );
            }
        }
        graph_string.push('}');
        write!(f, "{}", graph_string)
    }
}

// ----- Implementation of the 'TwoDimensionalGraphInsertionError' struct -----

/// Error type for failed insertions into [`TwoDimensionalCoordinateGraph`].
///
/// # Fields
///
/// - `message`: human-readable error description.
/// - `cause_nodes`: node pair that may have caused the error.
///
/// # Type Parameter
///
/// - `C`: coordinate scalar type used by node payloads in this error.
#[derive(Debug)]
pub struct TwoDimensionalGraphInsertionError<C: CoordinateDatatype = i32> {
    /// Detailed description of the error.
    pub message: String,
    /// Two nodes passed when they caused the issue.
    cause_nodes: Option<[TwoDimensionalNode<C>; 2]>,
}

impl<C: CoordinateDatatype> TwoDimensionalGraphInsertionError<C> {
    /// Creates a new insertion error value.
    ///
    /// If `message` is empty, a default fallback message is used.
    ///
    /// # Arguments
    ///
    /// - `message` -> Descriptive message about what caused the error.
    /// - `nodes` -> Optional pair of nodes relevant to the failure.
    ///
    /// # Returns
    ///
    /// New instance of [`TwoDimensionalGraphInsertionError`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalGraphInsertionError;
    ///
    /// let err = TwoDimensionalGraphInsertionError::<i32>::new(
    ///     "invalid insertion".to_string(),
    ///     None,
    /// );
    /// assert!(err.to_string().contains("invalid insertion"));
    /// ```
    pub fn new(message: String, nodes: Option<[TwoDimensionalNode<C>; 2]>) -> Self {
        let err_message = if message.is_empty() {
            debug!("No message was provided for 'TwoDimensionalGraphInsertionError'!");
            String::from(
                "An insertion error occurred while trying to add data to the two-dimensional graph!",
            )
        } else {
            message
        };

        Self {
            message: err_message,
            cause_nodes: nodes,
        }
    }
}

impl<C: CoordinateDatatype> Display for TwoDimensionalGraphInsertionError<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; nodes involved in the occurred error: {:?}",
            self.message, self.cause_nodes
        )
    }
}

impl<C: CoordinateDatatype> Error for TwoDimensionalGraphInsertionError<C> {}
