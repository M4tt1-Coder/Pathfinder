use std::{error::Error, fmt::Display};

use log::info;

use crate::{
    graphs::graph::{Graph, GraphEdge, GraphNode},
    nodes::default_node::DefaultNode,
};

/// A directed graph implementation.
///
/// Its representational sign is 'D' relevant for storing data in a file.
///
/// # Example
/// ```
/// use shortest_path_finder::graphs::directed::{DirectedEdge, DirectedGraph};
/// use shortest_path_finder::nodes::default_node::DefaultNode;
/// let graph = DirectedGraph {
///     nodes: vec![
///         DefaultNode::new("A".to_string()),
///         DefaultNode::new("B".to_string()),
///     ],
///     edges: vec![DirectedEdge::new(
///         DefaultNode::new("A".to_string()),
///         DefaultNode::new("B".to_string()),
///         4,
///     )],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct DirectedGraph {
    pub nodes: Vec<DefaultNode>,
    pub edges: Vec<DirectedEdge>,
}

impl Graph for DirectedGraph {
    type Node = DefaultNode;

    type Weight = u16;

    type Edge = DirectedEdge;

    type InsertionError = DirectedGraphInsertionError;

    fn is_directed(&self) -> bool {
        true
    }

    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a> {
        let mut neighbors: Vec<(&Self::Node, Self::Weight)> = vec![];
        // search in the edges where 'u' is the start node in a directed edge
        for e in &self.edges {
            if &e.from == u {
                neighbors.push((&e.to, e.weight));
            }
        }

        Box::new(neighbors.into_iter())
    }

    fn insert_node(&mut self, new_node: Self::Node) {
        if self.does_node_already_exist(&new_node) {
            return;
        }

        // add the node to the graph
        self.nodes.push(new_node);
    }

    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError> {
        if self.does_edge_already_exist(&edge) {
            return Some(DirectedGraphInsertionError::new(format!(
                "The edge {} already exists in the graph!",
                edge
            )));
        }

        if !self.does_node_already_exist(&edge.from) || !self.does_node_already_exist(&edge.to) {
            return Some(DirectedGraphInsertionError::new(format!(
                "One of the two nodes or both in the edge {} doesn't exist!",
                edge
            )));
        }

        // add the edge to the list
        self.edges.push(edge);

        None
    }

    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {
        for e in &self.edges {
            if e.from.id == edge.from.id && e.to.id == edge.to.id {
                return true;
            }
        }
        false
    }

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        for n in &self.nodes {
            if n.id == node.id {
                return true;
            }
        }
        false
    }

    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<&Self::Edge> {
        self.edges
            .iter()
            .find(|&e| &e.get_id() == id)
            .map(|v| v as _)
    }

    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node> {
        self.nodes
            .iter()
            .find(|&n| n.get_id() == id)
            .map(|v| v as _)
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
    /// Create new 'DirectedGraph' instance.
    pub fn new(nodes: Vec<DefaultNode>, edges: Vec<DirectedEdge>) -> Self {
        Self { nodes, edges }
    }
}

impl Display for DirectedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes: {:?}, Edges: {:?}", self.nodes, self.edges)
    }
}

impl Default for DirectedGraph {
    /// Initializes a 'DirectedGraph' instance with no edges and nodes.
    fn default() -> Self {
        Self::new(vec![], vec![])
    }
}

// ----- Implementation of the 'DirectedEdge' struct -----

/// An edge for a directed graph, where you only start beginning at 'from' and go to 'to'.
///
/// # Fields
///
/// - 'from' -> The node from which you start walking along the edge.
/// - 'to' -> The node you end up, when you walked along the edge.
/// - 'weight' -> The abstract "distance" between the two nodes.
#[derive(Clone, PartialEq, Debug)]
pub struct DirectedEdge {
    pub from: DefaultNode,
    pub to: DefaultNode,
    pub weight: u16,
    id: uuid::Uuid,
}

impl DirectedEdge {
    /// Create a new 'DirectedEdge' instance.
    pub fn new(from: DefaultNode, to: DefaultNode, weight: u16) -> Self {
        Self {
            from,
            to,
            weight,
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl Display for DirectedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            from: {},
            to: {},
            weight: {}
        ",
            self.from, self.to, self.weight
        )
    }
}

impl GraphEdge for DirectedEdge {
    type ID = uuid::Uuid;
    fn get_id(&self) -> Self::ID {
        self.id
    }
}

// ----- Implementation of the 'DirectedGraphInsertionError' struct -----

/// The error object that is returned when an insertion operation on an existing 'DirectedGraph'
/// instance goes wrong.
///
/// # Fields
///
/// - 'message' -> Explanation what went wrong.
#[derive(Debug)]
pub struct DirectedGraphInsertionError {
    pub message: String,
}

impl DirectedGraphInsertionError {
    /// Create a new 'DirectedGraphInsertionError' instance.
    pub fn new(message: String) -> Self {
        DirectedGraphInsertionError { message }
    }

    /// Log the 'DirectedGraphInsertionError' to the terminal.
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
