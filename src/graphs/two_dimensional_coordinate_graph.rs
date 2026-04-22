//! Two-dimensional coordinate graph implementation.
//!
//! # Overview
//!
//! This module models a graph whose nodes carry x/y coordinates:
//! - [`TwoDimensionalCoordinateGraph`] stores nodes and computed edges.
//! - [`TwoDimensionalEdge`] connects two coordinate nodes.
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
//! - [`TwoDimensionalEdge<C>`]
//! - [`TwoDimensionalGraphInsertionError<C>`]
//!
//! `C` must implement
//! [`CoordinateDatatype`](crate::nodes::trait_decl::coordinate_datatype::CoordinateDatatype).
//! Library users can therefore build coordinate graphs with types such as
//! `i32`, `f32`, or `u8`.
//!
//! # File Abbreviation
//!
//! The graph abbreviation used in file input is `TD`.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::graphs::two_dimensional_coordinate_graph::{
//!     TwoDimensionalCoordinateGraph, TwoDimensionalEdge,
//! };
//! use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
//!
//! let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
//! let b = TwoDimensionalNode::new(2, 3, "B".to_string()).unwrap();
//! let mut graph = TwoDimensionalCoordinateGraph::new(vec![a.clone(), b.clone()], vec![]);
//! assert!(graph.insert_edge(TwoDimensionalEdge::new(a, b)).is_none());
//! assert!(!graph.is_directed());
//!
//! let c = TwoDimensionalNode::<f32>::new(0.5, 1.5, "C".to_string()).unwrap();
//! let d = TwoDimensionalNode::<f32>::new(1.5, 3.0, "D".to_string()).unwrap();
//! let graph_f32 = TwoDimensionalCoordinateGraph::<f32>::new(vec![c, d], vec![]);
//! assert_eq!(graph_f32.get_all_nodes().len(), 2);
//! ```

use std::{collections::HashMap, error::Error, fmt::Display, marker::PhantomData};

use log::debug;
use uuid::Uuid;

use crate::{
    graphs::graph::{Graph, GraphEdge, GraphNode},
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
/// - Duplicate edges are rejected in either endpoint order.
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
    /// All existing edges in the graph.
    edges: Vec<TwoDimensionalEdge<C>>,
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

    /// Rebuilds internal lookup and adjacency caches from `nodes` and `edges`.
    ///
    /// # Why this matters
    ///
    /// `TwoDimensionalCoordinateGraph` stores canonical data in `nodes` and
    /// `edges`, while `node_index_by_id` and `adjacency` are derived caches for
    /// fast access. This function refreshes those caches whenever a graph is
    /// created from pre-populated vectors.
    ///
    /// # Behavior
    ///
    /// - Re-indexes nodes by ID.
    /// - Recreates adjacency buckets.
    /// - Replays each edge bidirectionally (undirected semantics).
    fn rebuild_internal_adjacency(&mut self) {
        // Build an ID -> index table so node lookups stay O(1).
        self.node_index_by_id = self
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.get_id().to_string(), index))
            .collect();

        // Reset adjacency to one list per node.
        self.adjacency = vec![Vec::new(); self.nodes.len()];

        for edge in &self.edges {
            // Ignore dangling edges that reference missing nodes.
            let Some(node_one_index) = self.node_index_for_id(edge.node_one_id()) else {
                continue;
            };
            let Some(node_two_index) = self.node_index_for_id(edge.node_two_id()) else {
                continue;
            };

            // Coordinate graph edges are undirected, so insert both directions.
            self.adjacency[node_one_index].push((node_two_index, edge.weight));
            self.adjacency[node_two_index].push((node_one_index, edge.weight));
        }
    }

    /// Creates a new two-dimensional graph from node and edge vectors.
    ///
    /// # Arguments
    ///
    /// - `nodes`: initial node set.
    /// - `edges`: initial edge set.
    ///
    /// # Returns
    ///
    /// Fresh [`TwoDimensionalCoordinateGraph`] object with initial nodes and edges.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::graph::Graph;
    /// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let node = TwoDimensionalNode::new(1, 2, "N1".to_string()).unwrap();
    /// let graph = TwoDimensionalCoordinateGraph::new(vec![node], vec![]);
    /// assert_eq!(graph.get_all_nodes().len(), 1);
    /// ```
    pub fn new(nodes: Vec<TwoDimensionalNode<C>>, edges: Vec<TwoDimensionalEdge<C>>) -> Self {
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

impl<C: CoordinateDatatype> Graph for TwoDimensionalCoordinateGraph<C> {
    type Node = TwoDimensionalNode<C>;
    type Edge = TwoDimensionalEdge<C>;
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

    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError> {
        if self.does_edge_already_exist(&edge) {
            return Some(TwoDimensionalGraphInsertionError::new(
                format!(
                    "The edge {} already exists! Can't be added to the graph!",
                    edge
                ),
                Some(edge),
                None,
            ));
        }

        let node_one_index = match self.node_index_for_id(edge.node_one_id()) {
            Some(index) => index,
            None => {
                return Some(TwoDimensionalGraphInsertionError::new(
                    format!(
                        "Node '{}' referenced by edge {} is not in the graph!",
                        edge.node_one_id(),
                        edge
                    ),
                    Some(edge),
                    None,
                ));
            }
        };

        let node_two_index = match self.node_index_for_id(edge.node_two_id()) {
            Some(index) => index,
            None => {
                return Some(TwoDimensionalGraphInsertionError::new(
                    format!(
                        "Node '{}' referenced by edge {} is not in the graph!",
                        edge.node_two_id(),
                        edge
                    ),
                    Some(edge),
                    None,
                ));
            }
        };

        self.adjacency[node_one_index].push((node_two_index, edge.weight));
        self.adjacency[node_two_index].push((node_one_index, edge.weight));
        self.edges.push(edge);

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

    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<&Self::Edge> {
        self.edges
            .iter()
            .find(|&e| e.get_id() == *id)
            .map(|v| v as _)
    }

    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {
        for existing in &self.edges {
            if existing.id == edge.id
                || (existing.node_one_id == edge.node_one_id
                    && existing.node_two_id == edge.node_two_id)
                || (existing.node_one_id == edge.node_two_id
                    && existing.node_two_id == edge.node_one_id)
            {
                return true;
            }
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
        let mut nodes_string = String::from("Nodes: \n");
        for n in &self.nodes {
            nodes_string.push_str(
                format!("{}: ( X: {}, Y: {} )\n", n.get_id(), n.get_x(), n.get_y()).as_str(),
            );
        }

        let mut edges_string = String::from("Edges: \n");
        for e in &self.edges {
            edges_string.push_str(
                format!(
                    "( ID: {}, Node A ID: {}, Node B ID: {}, Weight: {} )\n",
                    e.id, e.node_one_id, e.node_two_id, e.weight
                )
                .as_str(),
            );
        }

        write!(f, "{}{}", nodes_string, edges_string)
    }
}

// ----- Implementation of the 'TwoDimensionalEdge' -----

/// Edge connecting two nodes in a [`TwoDimensionalCoordinateGraph`].
///
/// # Weighting
///
/// The edge weight is calculated eagerly at construction and cached in the
/// struct for fast access during pathfinding.
#[derive(Debug, PartialEq, Clone)]
pub struct TwoDimensionalEdge<C: CoordinateDatatype = i32> {
    /// Unique identifier of the edge.
    id: Uuid,
    /// First endpoint node ID of the edge.
    node_one_id: String,
    /// Second endpoint node ID of the edge.
    node_two_id: String,
    /// Cached weight computed from endpoint coordinates at construction time.
    weight: f32,
    /// Marker tying this edge to the coordinate type used by participating nodes.
    _coordinate_type: PhantomData<C>,
}

impl<C: CoordinateDatatype> TwoDimensionalEdge<C> {
    /// Creates a new edge and computes its cached weight.
    ///
    /// # Arguments
    ///
    /// * `node_one` -> First node of the edge.
    /// * `node_two` -> Second node of the edge.
    ///
    /// # Returns
    ///
    /// [`TwoDimensionalEdge`] object.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalEdge;
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let a = TwoDimensionalNode::new(4, 2, "A".to_string()).unwrap();
    /// let b = TwoDimensionalNode::new(0, 0, "B".to_string()).unwrap();
    /// let edge = TwoDimensionalEdge::new(a, b);
    /// assert!(edge.get_weight().is_finite());
    /// ```
    pub fn new(node_one: TwoDimensionalNode<C>, node_two: TwoDimensionalNode<C>) -> Self {
        let weight = Self::calculate_weight(&node_one, &node_two);

        Self {
            id: Uuid::new_v4(),
            node_one_id: node_one.get_id().to_string(),
            node_two_id: node_two.get_id().to_string(),
            weight,
            _coordinate_type: PhantomData,
        }
    }

    /// Returns the cached weight of this edge.
    ///
    /// # Returns
    ///
    /// The weight of the edge.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalEdge;
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let a = TwoDimensionalNode::new(5, 2, "A".to_string()).unwrap();
    /// let b = TwoDimensionalNode::new(1, 1, "B".to_string()).unwrap();
    /// let edge = TwoDimensionalEdge::new(a, b);
    /// assert!(edge.get_weight().is_finite());
    /// ```
    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    /// Returns the first endpoint node ID.
    pub fn node_one_id(&self) -> &str {
        &self.node_one_id
    }

    /// Returns the second endpoint node ID.
    pub fn node_two_id(&self) -> &str {
        &self.node_two_id
    }

    /// Calculates edge weight using Euclidean distance between endpoints.
    ///
    /// # Formula
    ///
    /// For endpoint coordinates $(x_1, y_1)$ and $(x_2, y_2)$, this function
    /// computes:
    ///
    /// $$
    /// \sqrt{(x_1 - x_2)^2 + (y_1 - y_2)^2}
    /// $$
    ///
    /// # Returns
    ///
    /// Non-negative floating-point weight used by shortest-path algorithms.
    fn calculate_weight(node_one: &TwoDimensionalNode<C>, node_two: &TwoDimensionalNode<C>) -> f32 {
        // Convert coordinates to f32 to perform geometric calculations.
        let dx = node_one.get_x().to_f32() - node_two.get_x().to_f32();
        let dy = node_one.get_y().to_f32() - node_two.get_y().to_f32();

        // Euclidean norm in 2D.
        (dx * dx + dy * dy).sqrt()
    }
}

impl<C: CoordinateDatatype> GraphEdge for TwoDimensionalEdge<C> {
    type ID = Uuid;

    fn get_id(&self) -> Self::ID {
        self.id
    }
}

impl<C: CoordinateDatatype> Display for TwoDimensionalEdge<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {},\nNode A ID: {}\nNode B ID: {}\nWeight: {}",
            self.id, self.node_one_id, self.node_two_id, self.weight
        )
    }
}

// ----- Implementation of the 'TwoDimensionalGraphInsertionError' struct -----

/// Error type for failed insertions into [`TwoDimensionalCoordinateGraph`].
///
/// # Fields
///
/// - `message`: human-readable error description.
/// - `cause_edge`: edge payload that may have caused the error.
/// - `cause_nodes`: node pair that may have caused the error.
///
/// # Type Parameter
///
/// - `C`: coordinate scalar type used by node payloads in this error.
#[derive(Debug)]
pub struct TwoDimensionalGraphInsertionError<C: CoordinateDatatype = i32> {
    /// Detailed description of the error.
    pub message: String,
    /// Edge payload that may have caused the error.
    cause_edge: Option<TwoDimensionalEdge<C>>,
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
    /// - `edge` -> Relevant [`TwoDimensionalEdge`] payload.
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
    ///     None,
    /// );
    /// assert_eq!(err.to_string(), "invalid insertion");
    /// ```
    pub fn new(
        message: String,
        edge: Option<TwoDimensionalEdge<C>>,
        nodes: Option<[TwoDimensionalNode<C>; 2]>,
    ) -> Self {
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
            cause_edge: edge,
            cause_nodes: nodes,
        }
    }
}

impl<C: CoordinateDatatype> Display for TwoDimensionalGraphInsertionError<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cause_string = String::new();
        if let Some(edge) = &self.cause_edge {
            cause_string.push_str(format!("; Causing edge: {}", edge).as_str());
        }
        if let Some(nodes) = &self.cause_nodes {
            for (i, n) in nodes.iter().enumerate() {
                cause_string.push_str(format!("; Causing Node {}: {}", i, n).as_str());
            }
        }
        write!(f, "{}", self.message)
    }
}

impl<C: CoordinateDatatype> Error for TwoDimensionalGraphInsertionError<C> {}
