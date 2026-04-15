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
//! ```

use std::{error::Error, fmt::Display};

use log::debug;
use uuid::Uuid;

use crate::{
    graphs::graph::{Graph, GraphEdge, GraphNode},
    nodes::{
        trait_decl::coordinates_node::CoordinatesNode, two_dimensional_node::TwoDimensionalNode,
    },
};

/// Represents a two dimensional graph, which contains nodes with two ordinates X and Y.
///
/// Its sign is 'TD' utilised in files where graph data is stored.
///
/// # Fields
///
/// - 'nodes' -> Nodes in the graph
/// - 'edges' -> Edges in the graph
#[derive(Debug, Clone, Default)]
pub struct TwoDimensionalCoordinateGraph {
    /// ----- Private field -----
    ///
    /// List of 'TwoDimensionalNode' placed in the graph.
    nodes: Vec<TwoDimensionalNode>,
    ///  ----- Private field -----
    ///
    /// All existing edges in the graph.
    ///
    /// Number of edges can't exceed |'nodes'| * |'nodes'|.
    edges: Vec<TwoDimensionalEdge>,
}

impl TwoDimensionalCoordinateGraph {
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
    pub fn new(nodes: Vec<TwoDimensionalNode>, edges: Vec<TwoDimensionalEdge>) -> Self {
        Self { nodes, edges }
    }
}

impl Graph for TwoDimensionalCoordinateGraph {
    // types
    type Node = TwoDimensionalNode;
    type Edge = TwoDimensionalEdge;
    type Weight = f32;
    type InsertionError = TwoDimensionalGraphInsertionError;

    // methods
    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a> {
        let mut neighbours: Vec<(&Self::Node, Self::Weight)> = vec![];

        // iterate over edges and return tuple with reference to the node with the weight of the
        // edge
        for e in &self.edges {
            if &e.node_one == u {
                neighbours.push((&e.node_two, e.weight));
            } else if &e.node_two == u {
                neighbours.push((&e.node_one, e.weight));
            }
        }
        Box::new(neighbours.into_iter())
    }

    fn is_directed(&self) -> bool {
        false
    }

    fn insert_node(&mut self, new_node: Self::Node) {
        // if the node already exists  -> then dont add it -> return
        if self.does_node_already_exist(&new_node) {
            return;
        }

        self.nodes.push(new_node);
    }

    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError> {
        // if the edge with the nodes already exists -> return error
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
        // if at least one its predefined nodes isn't in the graph -> return error
        if !self.does_node_already_exist(&edge.node_one)
            || !self.does_node_already_exist(&edge.node_two)
        {
            return Some(TwoDimensionalGraphInsertionError::new(
                format!(
                    "One of the two nodes A {} or B {} of the edge {} are not in the graph! ",
                    edge.node_one, edge.node_two, edge
                ),
                None,
                Some([edge.node_one, edge.node_two]),
            ));
        }

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
        self.nodes
            .iter()
            .find(|&n| n.get_id() == id)
            .map(|v| v as _)
    }

    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<&Self::Edge> {
        self.edges
            .iter()
            .find(|&e| e.get_id() == *id)
            .map(|v| v as _)
    }

    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {
        for e in &self.edges {
            if e.id == edge.id
                || (e.node_one == edge.node_one && e.node_two == edge.node_two)
                || (e.node_one == edge.node_two && e.node_two == edge.node_one)
            {
                return true;
            }
        }
        false
    }

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        for n in &self.nodes {
            if n.get_y() == node.get_y() && n.get_x() == node.get_x() || node.get_id() == n.get_id()
            {
                return true;
            }
        }
        false
    }

    fn abbreviation() -> String {
        String::from("TD")
    }
}

impl Display for TwoDimensionalCoordinateGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // use template strings to display the nodes and edges in a clear manner
        // nodes
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
                    "( ID: {}, Node A: {}, Node B: {}, Weight: {} )\n",
                    e.id, e.node_one, e.node_two, e.weight
                )
                .as_str(),
            );
        }
        write!(f, "{}{}", nodes_string, edges_string)
    }
}

// ----- Implementation of the 'TwoDimensionalEdge' -----

/// Represents the edge in a 'TwoDimensionalCoordinateGraph' graph holding two nodes which have two
/// ordinates for a two dimensional coordinate system.
///
/// # Fields
///
/// * 'id' -> Identifier
/// * 'node_one' -> Node A of the edge
/// * 'node_two' -> Node B ...
/// * 'weight' -> Determined weight of the edge
#[derive(Debug, PartialEq, Clone)]
pub struct TwoDimensionalEdge {
    /// Unique identifier of the edge.
    ///
    /// UUID
    id: Uuid,
    /// The first node of the 'TwoDimensionalEdge'.
    pub node_one: TwoDimensionalNode,
    /// The second node of the 'TwoDimensionalEdge'.
    pub node_two: TwoDimensionalNode,
    /// The weight of the edge.
    ///
    /// It is calculated directly after creating the object. There was the option to determine on a
    /// method call but storing another 32bit extra to save computing time later.
    weight: f32,
}

impl TwoDimensionalEdge {
    /// Stores copies of the two nodes which the edge will connect. Furthermore, the weight is
    /// calculated based on the coordinates of the two nodes.
    ///
    /// # Arguments
    ///
    /// * 'node_one' -> First node of the edge.
    /// * 'node_two' -> Second mentioned node.
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
    pub fn new(node_one: TwoDimensionalNode, node_two: TwoDimensionalNode) -> Self {
        let mut edge: Self = Self {
            id: Uuid::new_v4(),
            node_one,
            node_two,
            weight: 0.0_f32, // temporary value -> isn't the actual value
        };
        // calculate the weight and save it
        edge.weight = edge.retrieve_actual_weight();

        edge
    }

    /// Retrieves the private value of the weight of the 'TwoDimensionalEdge'.
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

    /// Calculates the internal edge weight from the two endpoint coordinates.
    ///
    /// # Implementation Detail
    ///
    /// The current implementation uses bitwise XOR with `2` for both coordinate
    /// deltas and then takes the square root of their sum. This means the
    /// computed value is not a mathematical Euclidean distance.
    ///
    /// # Arguments
    ///
    /// - 'self' -> Any 'TwoDimensionalEdge' who's weight we want
    ///
    /// # Returns
    ///
    /// Computed `f32` weight value used internally by the graph.
    fn retrieve_actual_weight(&self) -> f32 {
        // Current implementation applies XOR-based transformation to deltas.
        let height = (self.node_one.get_x() - self.node_two.get_x()) ^ 2;
        let width = (self.node_one.get_y() - self.node_two.get_y()) ^ 2;

        // take the square root of the height and width
        let temp_sum = (height + width) as f32;

        temp_sum.sqrt()
    }
}

// Implement the 'GraphEdge' trait for the 'TwoDimensionalEdge'
impl GraphEdge for TwoDimensionalEdge {
    type ID = Uuid;

    fn get_id(&self) -> Self::ID {
        self.id
    }
}

impl Display for TwoDimensionalEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {},\nNode A: {}\nNode B: {}\nWeight: {}",
            self.id, self.node_one, self.node_two, self.weight
        )
    }
}

// ----- Implementation of the 'TwoDimensionalGraphInsertionError' struct -----

/// A global error for an issue in the insertion process when inserting an edge or node in the
/// graph.
///
/// # Fields
///
/// * 'message' -> Clear message / description of the occured error.
/// * 'cause_edge' -> 'TwoDimensionalEdge' object which could have caused the error.
/// * 'cause_nodes' -> Array of two nodes which could be the reason the problem was faced.
#[derive(Debug)]
pub struct TwoDimensionalGraphInsertionError {
    /// Detailed description of the error
    pub message: String,
    /// A 'TwoDimensionalEdge' instance which potentially be what caused the error.
    cause_edge: Option<TwoDimensionalEdge>,
    /// Two nodes passed when they caused the issue.
    cause_nodes: Option<[TwoDimensionalNode; 2]>,
}

impl TwoDimensionalGraphInsertionError {
    /// Generates a new object of the 'TwoDimensionalGraphInsertionError' struct.
    ///
    /// When the passed 'message' to the function is emtpy an default message is used.
    ///
    /// # Arguments
    ///
    /// - 'message' -> Descriptive message about what caused the error! Can refer to provided data
    /// - 'cause_edge' -> An 'TwoDimensionalEdge' object relevant for the cause of the error!
    /// - 'cause_nodes' -> Array of two 'TwoDimensionalNode' also important to explain why the
    ///   error occured.
    ///
    /// # Returns
    ///
    /// New instance of the [`TwoDimensionalGraphInsertionError`] struct.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalGraphInsertionError;
    ///
    /// let err = TwoDimensionalGraphInsertionError::new(
    ///     "invalid insertion".to_string(),
    ///     None,
    ///     None,
    /// );
    /// assert_eq!(err.to_string(), "invalid insertion");
    /// ```
    pub fn new(
        message: String,
        edge: Option<TwoDimensionalEdge>,
        nodes: Option<[TwoDimensionalNode; 2]>,
    ) -> Self {
        // if an empty message was provided -> apply default message BUT log info saying no error
        // message provided
        let err_message = if message.is_empty() {
            debug!("No message was provided to the for the 'TwoDimensionalGraphInsertionError'!");
            String::from(
                "An insertion error occured while trying to add data to the two dimensional graph!",
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

impl Display for TwoDimensionalGraphInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // check if data was passed with the error
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

impl Error for TwoDimensionalGraphInsertionError {}
