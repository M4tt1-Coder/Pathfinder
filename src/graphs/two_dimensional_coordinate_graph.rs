// ----- Implementation of the 'TwoDimensionalCoordinateGraph' struct -----

use std::{error::Error, fmt::Display};

use log::debug;
use uuid::Uuid;

use crate::graphs::graph::{Graph, GraphEdge, GraphNode};

/// Represents a two dimensional graph, which contains nodes with two ordinates X and Y.
///
/// # Fields
///
/// - 'nodes' -> Nodes in the graph
/// - 'edges' -> Edges in the graph
#[derive(Debug, Clone)]
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
    /// Creates the new 'TwoDimensionalCoordinateGraph' instance.
    ///
    /// The nodes and edges mainly will be added later when processing the data source of the
    /// graph.
    ///
    /// # Arguments
    ///
    /// - 'nodes' -> Vector of 'TwoDimensionalNode' elements, doesn't need to have any nodes in it.
    /// - 'edges' -> Vector of 'TwoDimensionalEdge's same applies for the edges.
    ///
    /// # Returns
    ///
    /// => Fresh 'TwoDimensionalCoordinateGraph' object with initial nodes and edges.
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
                neighbours.push((&e.node_one, e.weight));
            } else if &e.node_two == u {
                neighbours.push((&e.node_two, e.weight));
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
        for n in &self.nodes {
            if n.id == id {
                return Some(n);
            }
        }

        None
    }

    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<&Self::Edge> {
        for e in &self.edges {
            if e.id == *id {
                return Some(e);
            }
        }
        None
    }
    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {
        for e in &self.edges {
            if e == edge {
                return true;
            }
        }
        false
    }
    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        for n in &self.nodes {
            if n == node {
                return true;
            }
        }
        false
    }
}

impl Display for TwoDimensionalCoordinateGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // use template strings to display the nodes and edges in a clear manner
    }
}

// ----- Implementation of the 'TwoDimensionalNode' struct -----

/// Node in a 'TwoDimensionalCoordinateGraph'.
///
/// In that context the node needs to hold information about where the node is placed on the 'map'.
///
/// All attributes are private and can't be mutated from outside after inizialization.
///
/// # Fields
///
/// - 'id' -> Identifier
/// - 'x' -> X - ordinate
/// - 'y' -> Y - ordinate
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct TwoDimensionalNode {
    /// -- Private Field --
    ///
    /// The unique identifier for the node. It can be seen as its name too, but is used as an
    /// IDsince it the name needs to be unique in a graph.
    id: String,
    /// -- Private Field --
    ///
    /// X - ordinate of the individual 'TwoDimensionalNode' struct instance.
    x: u16,
    /// -- Private field --
    ///
    /// Y - ordinate of the individual 'TwoDimensionalNode' struct instance.
    y: u16,
}

impl TwoDimensionalNode {
    /// Creates a new instance of the 'TwoDimensionalNode' struct.
    ///
    /// When the identifier has a length of 0, then no new object is being created.
    ///
    /// # Arguments
    ///
    /// - 'x' -> X-ordinate of the node
    /// - 'y' -> Y-ordinate of the node
    /// - 'id' -> unique identifier of the node, which can't be null or a duplicate in the graph
    /// (external check)
    ///
    /// # Returns
    ///
    /// => Validated fresh 'TwoDimensionalNode'
    pub fn new(x: u16, y: u16, id: String) -> Option<Self> {
        // id must be longer then 0
        if id.len() == 0 {
            return None;
        };
        Some(Self { x, y, id })
    }

    /// Returns the Y ordinate of the 'TwoDimensionalNode' in the graph.
    pub fn get_x(&self) -> u16 {
        self.x
    }

    /// Provides the Y ordinate of the node in the graph.
    pub fn get_y(&self) -> u16 {
        self.y
    }
}

impl GraphNode for TwoDimensionalNode {
    fn get_id<'a>(&'a self) -> &'a str {
        &self.id
    }
}

impl Display for TwoDimensionalNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}, X-ordinate: {}, Y-ordinate: {}",
            self.id, self.x, self.y
        )
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
    ///                    error occured.
    ///
    /// # Returns
    ///
    /// => New instance of the 'TwoDimensionalGraphInsertionError' struct.
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
