// TODO: Finish graph for A* search algorithm

// ----- Implementation of the 'TwoDimensionalCoordinateGraph' struct -----

use std::{error::Error, fmt::Display};

use uuid::Uuid;

use crate::graphs::graph::{Graph, GraphEdge};

///
pub struct TwoDimensionalCoordinateGraph {
    nodes: TwoDimensionalNode,
}

impl TwoDimensionalCoordinateGraph {}

impl Graph for TwoDimensionalCoordinateGraph {
    // types

    type Node = TwoDimensionalNode;
    type Edge = TwoDimensionalEdge;
    type Weight = u16;
    type InsertionError = TwoDimensionalGraphInsertionError;

    // methods
    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a> {
    }
    fn is_directed(&self) -> bool {
        false
    }
    fn insert_node(&mut self, new_node: Self::Node) {}
    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError> {}
    fn is_weighted(&self) -> bool {}
    fn get_all_nodes(&self) -> &Vec<super::graph::Node> {}
    fn get_node_by_id(&self, id: &str) -> Option<Self::Node> {}
    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<Self::Edge> {}
    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {}
    fn does_node_already_exist(&self, node: &Self::Node) -> bool {}
}

// ----- Implementation of the 'TwoDimensionalNode' struct -----

///
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TwoDimensionalNode {
    ///
    id: String,
    ///
    x: u16,
    ///
    y: u16,
}

impl Display for TwoDimensionalNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
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
    /// => 'TwoDimensionalEdge' object
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
    /// => The weight of the edge.
    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    /// Since the nodes are in a two dimensional coordinate system the weight of an edge needs to
    /// be calculate depending on the two nodes the edge holds.
    ///
    /// # Arguments
    ///
    /// - 'self' -> Any 'TwoDimensionalEdge' who's weight we want
    ///
    /// # Returns
    ///
    /// => Calculated <f32> weight using the Pythagorean theorem
    fn retrieve_actual_weight(&self) -> f32 {
        // use trigometry to calculate the distance -> pythagoras
        let height = (self.node_one.x - self.node_two.x) ^ 2;
        let width = (self.node_one.y - self.node_two.y) ^ 2;

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
    message: String,
    /// A 'TwoDimensionalEdge' instance which potentially be what caused the error.
    cause_edge: Option<TwoDimensionalEdge>,
    /// Two nodes passed when they caused the issue.
    cause_nodes: Option<[TwoDimensionalNode; 2]>,
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
