use std::{error::Error, fmt::Display};

use uuid::Uuid;

use crate::graphs::graph::{Graph, GraphEdge, Node};

/// Undirected graphs don't have the restriction that you can't go along some edges from a specific
/// direction. Here you go along all ways.
///
/// # Fields
///
/// * 'nodes' -> The nodes of the graph.
/// * 'edges' -> The edges of the graph.
#[derive(Debug, Clone)]
pub struct UndirectedGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<UndirectedEdge>,
}

impl Graph for UndirectedGraph {
    type Node = Node;
    type Edge = UndirectedEdge;
    type Weight = u16;
    type InsertionError = UndirectedGraphInsertionError;

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        for n in &self.nodes {
            if n == node {
                return true;
            }
        }
        false
    }

    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {
        for e in &self.edges {
            if (e.a_node == edge.a_node && e.b_node == edge.b_node)
                || (e.b_node == edge.a_node && e.a_node == edge.b_node)
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
        let mut neighbors: Vec<(&Self::Node, Self::Weight)> = vec![];

        for e in &self.edges {
            if &e.a_node == u {
                neighbors.push((&e.b_node, e.weight));
            } else if &e.b_node == u {
                neighbors.push((&e.a_node, e.weight))
            }
        }

        Box::new(neighbors.into_iter())
    }

    fn neighbours_as_standard_output<'a>(
        &'a self,
        u: &Node,
    ) -> Box<dyn Iterator<Item = (&'a Node, u16)> + 'a> {
        self.neighbors(u)
    }
    fn is_directed(&self) -> bool {
        false
    }

    fn insert_node(&mut self, new_node: Self::Node) {
        if self.does_node_already_exist(&new_node) {
            return;
        }

        self.nodes.push(new_node.clone());
    }

    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError> {
        if self.does_edge_already_exist(&edge) {
            return Some(UndirectedGraphInsertionError::new(format!(
                "The edge {} already exists in the graph!",
                edge
            )));
        }

        if !self.does_node_already_exist(&edge.a_node)
            || !self.does_node_already_exist(&edge.b_node)
        {
            return Some(UndirectedGraphInsertionError::new(format!(
                "One of the or both nodes in the edge {} aren't part of the graph!",
                edge
            )));
        }

        self.edges.push(edge);

        None
    }

    fn get_node_by_id(&self, id: &str) -> Option<Self::Node> {
        for n in &self.nodes {
            if n.id == id {
                return Some(n.clone());
            }
        }
        None
    }
    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<Self::Edge> {
        for e in &self.edges {
            if &e.id == id {
                return Some(e.clone());
            }
        }
        None
    }
    fn get_all_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
    fn is_weighted(&self) -> bool {
        true
    }
}

impl UndirectedGraph {
    /// Creates a new instance of a 'UndirectedGraph' struct.
    ///
    /// # Arguments
    ///
    /// - 'nodes' -> List of nodes.
    /// - 'edges' -> Array of edges
    ///
    /// # Returns
    ///
    /// => A new instance of the 'UndirectedGraph'.
    pub fn new(nodes: Vec<Node>, edges: Vec<UndirectedEdge>) -> Self {
        Self { nodes, edges }
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

/// The edge of a undirected graph, where you can either come from 'A' or 'B'.
///
/// # Fields
///
/// * 'a_node' -> One node of the edge ...
/// * 'b_node' -> Other node of the edge ...
/// * 'weight' -> Fictional 'length' of the edge
#[derive(Clone, PartialEq, Debug)]
pub struct UndirectedEdge {
    pub a_node: Node,
    pub b_node: Node,
    pub weight: u16,
    id: Uuid,
}

impl UndirectedEdge {
    /// Create a new instance of the 'UndirectedEdge' struct.
    pub fn new(a_node: Node, b_node: Node, weight: u16) -> Self {
        Self {
            a_node,
            b_node,
            weight,
            id: Uuid::new_v4(),
        }
    }
}

impl Display for UndirectedEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            Node a: {},
            Node b: {},
            weight: {}
        ",
            self.a_node, self.b_node, self.weight
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

/// Represents an error that occured when an edge or node was inserted into the undirected graph.
///
/// # Fields
///
/// - 'message' -> Description of what caused the error to occur.
#[derive(Debug)]
pub struct UndirectedGraphInsertionError {
    pub message: String,
}

impl UndirectedGraphInsertionError {
    /// Create a new 'UndirectedGraphInsertionError' instance.
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
