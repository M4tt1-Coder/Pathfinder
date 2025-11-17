use std::{
    error::Error,
    fmt::{Debug, Display},
};

/// Makes sure that every edge has its own id (mostly UUID).
///
/// The 'getter' is there since the ID will be private.
pub trait GraphEdge {
    type ID: Eq + PartialEq + Copy;

    fn get_id(&self) -> Self::ID;
}

/// A trait representing a weighted graph structure.
///
/// The graph can be either directed or undirected.
///
/// # Associated Types
///
/// * `Node`: The type representing the nodes in the graph. Must implement `Eq`, `Hash`, and
///   `Clone`.
/// * `Weight`: The type representing the weights of the edges. Must implement `Copy`, `PartialOrd`,
///   and support addition.
///
/// # Methods
/// * `neighbors(&self, u: &Self::Node) -> Box<dyn Iterator<Item = (&Self::Node, Self::Weight)>>`:
///   Returns an iterator over the neighbors of the given node along with the weights of the edges.
/// * `is_directed(&self) -> bool`: Indicates whether the graph is directed.
pub trait Graph {
    /// The type representing the nodes in the graph.
    ///
    /// Must support equality comparison, hashing, and cloning.
    ///
    /// # Example
    /// ```
    /// #[derive(Clone, PartialEq, Eq, Hash)]
    /// struct Node {
    ///     id: String,
    /// }
    /// ```
    type Node: Eq + std::hash::Hash + Clone + GraphNode;

    /// The type representing the weights of the edges in the graph.
    ///
    /// Must support comparison and addition.
    ///
    /// # Example
    /// ```
    /// type Weight = u32;
    /// ```
    type Weight: Copy + PartialOrd + std::ops::Add<Output = Self::Weight>;

    /// The type representing the edges in the graph.
    ///
    /// Must support cloning and equality comparison.
    ///
    /// # Example
    /// ```
    /// use pathfinder::graphs::graph::Node;
    ///
    /// #[derive(Clone, PartialEq)]
    /// struct Edge {
    ///     from: Node,
    ///     to: Node,
    ///     weight: u16,
    /// }
    /// ```
    type Edge: Clone + PartialEq + GraphEdge;

    /// The error occurs when a node or an edge couldn't be added to the graph.
    ///
    /// Implements the 'std::error::Error' trait!
    type InsertionError: Error + Display + Debug;

    /// Returns an iterator over the neighbors of the given node along with the weights of the
    /// edges.
    ///
    /// # Arguments
    ///
    /// * `u`: A reference to the node whose neighbors are to be retrieved.
    ///
    /// # Returns
    ///
    /// An iterator over tuples containing references to neighboring nodes and the weights of the
    /// edges connecting them.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::graphs::{directed::DirectedGraph, graph::Node};
    ///
    /// let graph = DirectedGraph::new();
    ///
    /// let node = Node::new("A".to_string());
    ///
    /// let neighbors = graph.neighbors(&node);
    /// for (neighbor, weight) in neighbors {
    ///     println!("Neighbor: {:?}, Weight: {}", neighbor, weight);
    /// }
    /// ```
    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a>;

    /// Converts the generic iterator over the neighbours of a "Self::Node" to a iterator of the
    /// 'Node' struct.
    ///
    /// # Returns
    ///
    /// => Converted iterator with 'Item = (&'a Node, u16)'
    fn neighbours_as_standard_output<'a>(
        &'a self,
        u: &Node,
    ) -> Box<dyn Iterator<Item = (&'a Node, u16)> + 'a>;

    /// Indicates whether the graph is directed.
    ///
    /// # Returns
    /// * `true` if the graph is directed.
    /// * `false` if the graph is undirected.
    /// # Example
    /// ```
    /// use crate::graphs::directed::DirectedGraph;
    ///
    /// let graph = DirectedGraph::new();
    /// if graph.is_directed() {
    ///     println!("The graph is directed.");
    /// } else {
    ///     println!("The graph is undirected.");
    /// }
    /// ```
    fn is_directed(&self) -> bool;

    /// Inserts a node into the graph.
    ///
    /// # Arguments
    ///
    /// - 'new_node' -> The actual Node to be added to the graph.
    fn insert_node(&mut self, new_node: Self::Node);

    /// A new edge will be added to the graph.
    ///
    /// # Arguments
    ///
    /// - 'edge' -> The 'Self::Edge' to be added to the graph.
    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError>;

    /// When attempting to mutate the graph in some cases there needs to be checked if an
    /// 'Self::Edge' already exists.
    ///
    /// # Arguments
    ///
    /// * 'edge' -> The 'Self::Edge' which is going to be look for if there is duplicate in the current
    ///   edge list.
    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool;

    /// When attempting to mutate the graph in some cases there needs to be checked if an
    /// 'Self::Node' already exists.
    ///
    /// # Arguments
    ///
    /// * 'edge' -> The 'Self::Node' which is going to be look for if there is duplicate in the current
    ///   node list.
    fn does_node_already_exist(&self, node: &Self::Node) -> bool;

    /// Gets a 'Self::Node' by its id.
    ///
    /// # Arguments
    ///
    /// - 'id' -> The idenfier of a Node.
    ///
    /// # Returns
    ///
    /// => Option<Self::Node> if there is a Node with the specified id.
    fn get_node_by_id(&self, id: &str) -> Option<Self::Node>;

    /// Attempts to retrieve a 'Self::Edge' from the graph.
    ///
    /// # Arguments
    ///
    /// 'id' -> Identifier of an 'Self::Edge'.
    ///
    /// # Returns
    ///
    /// => Option<Self::Edge>
    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<Self::Edge>;

    /// Retrieve all 'Self::Node's in a 'Graph'.
    ///
    /// # Returns
    ///
    /// => &Vec<Node> with all nodes in a graph.
    fn get_all_nodes(&self) -> &Vec<Node>;

    /// States if a graph has weighed edges.
    ///
    /// This is relevant for some algorithms which need weighted edges.
    fn is_weighted(&self) -> bool;
}

// ----- Definition of the 'GraphNode' trait -----
pub trait GraphNode {
    /// There can be implemented other customized node types but need to return and create from
    /// themself so that algorithm can work with standardized nodes
    ///
    /// # Returns
    ///
    /// => A 'Node' instance from a custom generic node.
    fn get_self_as_standard_node(&self) -> Node;

    /// Provide the own ID of a 'GraphNode' struct.
    fn get_id(&self) -> String;
}

// ----- Implementation of the 'Node' struct -----

/// A general node in a graph structure (directed & undirected).
///
/// # Fields
///
/// - 'id' -> name of the node like "A" or "B", "Ulm"
/// - 'number_of_edges' -> In how many edges the node is in.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub struct Node {
    /// Key or Identifier of a 'Node' in a graph
    pub id: String,
    // pub number_of_edges: u8,
}

impl Node {
    /// Returns a new 'Node' object.
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl GraphNode for Node {
    fn get_self_as_standard_node(&self) -> Node {
        self.clone()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}
