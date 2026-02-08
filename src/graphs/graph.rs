use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::Add,
};

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
    /// ```rust
    /// use pathfinder::graphs::graph::GraphNode;
    ///
    /// #[derive(Clone, PartialEq, Eq, Hash)]
    /// struct Node {
    ///     id: String,
    /// }
    ///
    /// impl GraphNode for Node {
    ///     fn get_id<'a>(&'a self) -> &'a str {
    ///         &self.id
    ///     }
    /// }
    /// ```
    type Node: GraphNode;

    /// The type representing the weights of the edges in the graph.
    ///
    /// Must support comparison and addition.
    ///
    /// # Example
    /// ```
    /// type Weight = u32;
    /// ```
    type Weight: GraphWeight;

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
    type Edge: GraphEdge;

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
    /// use pathfinder::graphs::{ directed::{ DirectedGraph, DirectedEdge }, graph::Node };
    /// use crate::pathfinder::graphs::graph::Graph;
    ///
    /// let graph = DirectedGraph::new(
    ///     vec![Node::new("A".to_string()), Node::new("B".to_string())],
    ///     vec![DirectedEdge::new(Node::new("A".to_string()), Node::new("B".to_string()), 6)]
    /// );
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

    /// Indicates whether the graph is directed.
    ///
    /// # Returns
    /// * `true` if the graph is directed.
    /// * `false` if the graph is undirected.
    /// # Example
    /// ```
    /// use crate::pathfinder::graphs::graph::Graph;
    /// use pathfinder::graphs::directed::DirectedGraph;
    ///
    /// let graph = DirectedGraph::new(vec![], vec![]);
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
    /// => Option<&Self::Node> if there is a Node with the specified id.
    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node>;

    /// Attempts to retrieve a 'Self::Edge' from the graph.
    ///
    /// # Arguments
    ///
    /// 'id' -> Identifier of an 'Self::Edge'.
    ///
    /// # Returns
    ///
    /// => Option<&Self::Edge>
    fn get_edge_by_id(&self, id: &uuid::Uuid) -> Option<&Self::Edge>;

    /// Retrieve all 'Self::Node's in a 'Graph'.
    ///
    /// # Returns
    ///
    /// => &Vec<Node> with all nodes in a graph.
    fn get_all_nodes(&self) -> &Vec<Self::Node>;

    /// States if a graph has weighed edges.
    ///
    /// This is relevant for some algorithms which need weighted edges.
    fn is_weighted(&self) -> bool;
}

// ----- Definition of the 'GraphTrait' trait -----

/// A trait representing a type suitable for use as a weight in graph algorithms.
///
/// # Requirements
/// This trait is implemented for types that:
/// - are `Copy`, allowing for inexpensive duplication,
/// - implement `PartialOrd`, enabling comparison of weights,
/// - and support addition with `Add<Output = Self>`, allowing weights to be combined.
///
/// # Usage
/// Use `GraphWeight` as a trait bound for generic types in graph algorithms,
/// such as shortest path or minimum spanning tree implementations,
/// where weights need to be comparable, clonable, and combinable via addition.
///
/// # Example
/// ```rust
/// use std::ops::Add;
///
/// fn total_weight<W: GraphWeight>(weights: &[W]) -> W {
///     weights.iter().cloned().fold(W::zero(), |acc, w| acc + w)
/// }
///
/// // Assuming W implements `Zero` trait or similar for W::zero()
/// ```
pub trait GraphWeight:
    Copy + PartialOrd + Add<Output = Self> + Display + Debug + PartialOrd + PartialEq
{
    /// Returns the maximum possible value for the weight type.
    ///
    /// This value is typically used to initialize distances or weights that need
    /// to be replaced with smaller values during algorithm execution.
    ///
    /// # Returns
    /// The maximum value of the implementing type.
    fn max_value() -> Self;

    /// Returns the zero value (additive identity) for the weight type.
    ///
    /// This value is used as the default or starting weight in graph algorithms,
    /// representing no cost or distance.
    ///
    /// # Returns
    /// The zero value of the implementing type.
    fn zero() -> Self;
}

// ----- Definition of the 'GraphEdge' trait -----

/// Makes sure that every edge has its own id (mostly UUID).
///
/// The 'getter' is there since the ID will be private.
pub trait GraphEdge: Clone + PartialEq {
    /// **Type**
    ///
    /// Identifier of the edge.
    type ID: Eq + PartialEq + Copy;

    /// **Method**
    ///
    /// Returns the identifier of the edge.
    ///
    /// # Arguments
    ///
    /// - *&self* -> Individual instance of an *GraphEdge* implementation.
    ///
    /// # Returns
    ///
    /// => *Self::ID*, the ID of the edge as ```String``` for example.
    fn get_id(&self) -> Self::ID;
}

// ----- Definition of the 'GraphNode' trait -----

/// **Trait**
///
/// Global trait that is required by every graph to implement individually.
///
/// Represents a node in any kind of graph
pub trait GraphNode: Display + Debug + Eq + std::hash::Hash + Clone + Ord {
    /// Provide the own ID of a 'GraphNode' struct.
    fn get_id<'a>(&'a self) -> &'a str;
}
