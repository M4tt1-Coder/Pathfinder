//! Dijkstra test helpers.

use std::{collections::HashMap, fmt::Display};

use shortest_path_finder::{
    graph::{Graph, GraphNode, directed::DirectedGraphInsertionError},
    nodes::DefaultNode,
};

/// Creates a new [`DefaultNode`] with the given `id`.
///
/// # Arguments
/// * `id` - A string slice that holds the identifier for the node.
///
/// # Returns
/// * A [`DefaultNode`] instance with the specified `id`.
pub fn node(id: &str) -> DefaultNode {
    DefaultNode::new(id.to_string())
}

// ----- Implementation of struct TestDirectedGraph -----

/// A simple directed graph structure for testing purposes.
///
/// Need for testing Dijkstra's algorithm. The graph is represented as an adjacency list, where each
/// node points to a vector of tuples containing neighboring nodes and the weights of the edges
/// connecting them.
///
/// Need a primitive datatype for the weights of the edges. In this case, we use `i8` to represent
/// the weights, which allows for a range of values from -128 to 127. This is sufficient for many
/// test cases and keeps the implementation simple.
#[derive(Debug, Clone, Default)]
pub struct TestDirectedGraph {
    /// The nodes in the graph.
    nodes: Vec<DefaultNode>,
    /// The adjacency list representation of the graph.
    ///
    /// One node can have multiple edges to other nodes, each with a specific weight. The `HashMap`
    /// maps each node to a vector of tuples, where each tuple contains a neighboring node and the
    /// weight of the edge connecting them.
    adj_list: HashMap<DefaultNode, Vec<(DefaultNode, i8)>>,
}

impl Graph for TestDirectedGraph {
    type Node = DefaultNode;
    type Weight = i8;
    type InsertionError = DirectedGraphInsertionError;

    fn abbreviation() -> String {
        "TDG".to_string() // Test Directed Graph
    }

    fn is_directed(&self) -> bool {
        true
    }

    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a> {
        let neighbors = self
            .adj_list
            .get(u)
            .map(|edges| {
                edges.iter().map(|(neighbor, weight)| (neighbor, *weight))
            })
            .into_iter()
            .flatten();

        Box::new(neighbors)
    }

    fn insert_node(&mut self, new_node: Self::Node) {
        // prevent inserting a node that already exists in the graph -> overwritting its edges
        if self.does_node_already_exist(&new_node) {
            return;
        }
        self.nodes.push(new_node.clone());
        self.adj_list.insert(new_node, Vec::new());
    }

    fn insert_edge(
        &mut self,
        from: &Self::Node,
        to: &Self::Node,
        weight: Option<Self::Weight>,
    ) -> Option<Self::InsertionError> {
        if !self.does_node_already_exist(from) {
            return Some(DirectedGraphInsertionError::SourceNodeDoesNotExist {
                node_id: from.get_id().to_string(),
            });
        }

        if !self.does_node_already_exist(to) {
            return Some(DirectedGraphInsertionError::SourceNodeDoesNotExist {
                node_id: to.get_id().to_string(),
            });
        }

        if self.does_edge_already_exist(from, to) {
            return Some(DirectedGraphInsertionError::EdgeAlreadyExists {
                from: from.get_id().to_string(),
                to: to.get_id().to_string(),
            });
        }

        let edge_weight = match weight {
            Some(w) => w,
            None => {
                return Some(DirectedGraphInsertionError::MissingEdgeWeight {
                    expected_weight: "i8".to_string(),
                });
            },
        };

        self.adj_list
            .entry(from.clone())
            .or_default()
            .push((to.clone(), edge_weight));

        None
    }

    fn is_weighted(&self) -> bool {
        true
    }

    fn get_all_nodes(&self) -> &Vec<Self::Node> {
        &self.nodes
    }

    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node> {
        self.adj_list.keys().find(|node| node.get_id() == id)
    }

    fn does_edge_already_exist(
        &self,
        from: &Self::Node,
        to: &Self::Node,
    ) -> bool {
        self.adj_list.get(from).is_some_and(|edges| {
            edges.iter().any(|(neighbor, _)| neighbor == to)
        })
    }

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        self.adj_list.contains_key(node)
    }
}

impl Display for TestDirectedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TestDirectedGraph {{")?;
        for (node, edges) in &self.adj_list {
            write!(f, "  {} -> [", node.get_id())?;
            for (neighbor, weight) in edges {
                write!(f, "({}, {}), ", neighbor.get_id(), weight)?;
            }
            writeln!(f, "]")?;
        }
        writeln!(f, "}}")
    }
}

// ----- Implementation of struct 'TestUnweightedGraph' -----

/// A simple unweighted graph structure for testing purposes.
///
/// This graph is represented as an adjacency list, where each node points to a vector of
/// neighboring nodes. The graph is unweighted, meaning that all edges are considered to have the
/// same weight (or no weight at all).
#[derive(Debug, Clone, Default)]
pub struct TestUnweightedGraph {
    /// Vector of nodes in the graph.
    nodes: Vec<DefaultNode>,
    /// The adjacency list representation of the graph.
    adj_list: HashMap<DefaultNode, Vec<DefaultNode>>,
}

impl Graph for TestUnweightedGraph {
    type Node = DefaultNode;

    // Using i8 as a placeholder for unweighted graphs, where the weight is not necessary.
    type Weight = i8;

    type InsertionError = DirectedGraphInsertionError;

    fn is_weighted(&self) -> bool {
        false // Unweighted graph, so no weights are used for edges
    }

    fn abbreviation() -> String {
        "TUG".to_string() // Test Unweighted Graph
    }

    fn is_directed(&self) -> bool {
        true
    }

    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a> {
        let neighbors = self
            .adj_list
            .get(u)
            .map(|neighbors| neighbors.iter().map(|neighbor| (neighbor, 0))) // Assign a default weight of 0 for unweighted graphs
            .into_iter()
            .flatten();

        Box::new(neighbors)
    }

    fn insert_node(&mut self, new_node: Self::Node) {
        if self.does_node_already_exist(&new_node) {
            return;
        }
        self.nodes.push(new_node.clone());
        self.adj_list.insert(new_node, Vec::new());
    }

    fn insert_edge(
        &mut self,
        from: &Self::Node,
        to: &Self::Node,
        _weight: Option<Self::Weight>,
    ) -> Option<Self::InsertionError> {
        if !self.does_node_already_exist(from) {
            return Some(DirectedGraphInsertionError::SourceNodeDoesNotExist {
                node_id: from.get_id().to_string(),
            });
        }

        if !self.does_node_already_exist(to) {
            return Some(DirectedGraphInsertionError::SourceNodeDoesNotExist {
                node_id: to.get_id().to_string(),
            });
        }

        if self.does_edge_already_exist(from, to) {
            return Some(DirectedGraphInsertionError::EdgeAlreadyExists {
                from: from.get_id().to_string(),
                to: to.get_id().to_string(),
            });
        }

        self.adj_list
            .entry(from.clone())
            .or_default()
            .push(to.clone());

        None
    }

    fn get_all_nodes(&self) -> &Vec<Self::Node> {
        &self.nodes
    }

    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node> {
        self.adj_list.keys().find(|node| node.get_id() == id)
    }

    fn does_edge_already_exist(
        &self,
        from: &Self::Node,
        to: &Self::Node,
    ) -> bool {
        self.adj_list.get(from).is_some_and(|neighbors| {
            neighbors.iter().any(|neighbor| neighbor == to)
        })
    }

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        self.adj_list.contains_key(node)
    }
}

impl Display for TestUnweightedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TestUnweightedGraph {{")?;
        for (node, neighbors) in &self.adj_list {
            write!(f, "  {} -> [", node.get_id())?;
            for neighbor in neighbors {
                write!(f, "{}, ", neighbor.get_id())?;
            }
            writeln!(f, "]")?;
        }
        writeln!(f, "}}")
    }
}
