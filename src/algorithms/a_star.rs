//! # Theory for the A* algorithm
//!
//! ## Sources
//!
//! - https://www.datacamp.com/tutorial/a-star-algorithm
//! - https://www.geeksforgeeks.org/dsa/a-search-algorithm/
//! - https://theory.stanford.edu/~amitp/GameProgramming/AStarComparison.html
//!
//! g(n) -> cost from start to node n
//! h(n) -> heuristic cost estimate from n to goal
//!
//! f(n) = g(n) + h(n) -> estimated total cost from start to goal through n
//!
//! - in the main loop, we pick the node with the lowest f(n) = g(n) + h(n)
//!
//! - use the euclidean distance or Manhattan distance for h(n)
//!
//! function heuristic(node) =
//!    dx = abs(node.x - goal.x)
//!    dy = abs(node.y - goal.y)
//!    return D * sqrt(dx * dx + dy * dy)

// TODO: Add the implementation of the A* algorithm

use std::{error::Error, fmt::Display};

use crate::{
    algorithms::algorithm::{Algorithm, SearchResult},
    graphs::{
        graph::{Graph, GraphNode},
        two_dimensional_coordinate_graph::TwoDimensionalNode,
    },
};

///
#[derive(Debug)]
pub struct AStar<G: Graph + Display> {
    ///
    graph: G,
}

// ----- Implementation of the 'A_Star' struct -----

impl<G: Graph + Display> Algorithm for AStar<G> {
    type AlgorithmSearchResult = AStarSearchResult;

    fn shortest_path<N: GraphNode>(
        &self,
        start: &N,
        end: &N,
    ) -> Result<Self::AlgorithmSearchResult, Self::ExecutionError> {
        unimplemented!()
    }
    type ExecutionError = AStarExecutionError;
}

impl<G: Graph + Display> AStar<G> {
    ///
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    fn heuristic(
        &self,
        node: &crate::graphs::graph::Node,
        goal: &crate::graphs::graph::Node,
    ) -> u32 {
        // Placeholder heuristic function
        0
    }
}

// ----- Implementation of the 'AStarSearchResult' struct -----

/// **Struct**
///
/// Represents the result that the A* algorithm returns containing the determined distance and the
/// visited nodes
///
/// # Fields
///
/// - *distance* -> Total distance; sum of all edges
/// - *path* -> List of nodes thatt were visited
///
/// # Example
///
/// ```rust
/// use pathfinder::{ graphs::two_dimensional_coordinate_graph::TwoDimensionalNode,
/// algorithms::a_star::AStarSearchResult };
///
/// let node_A = TwoDimensionalNode::new(0,0,"A".to_string());
/// let node_B = TwoDimensionalNode::new(0,1,"B".to_string());
///
/// // create the 'AStarSearchResult' object
/// let search_result = AStarSearchResult::new(1., vec![node_A, node_B]);
/// ```
#[derive(Debug)]
pub struct AStarSearchResult {
    /// Total distance from one node A to node B.
    distance: f32,
    /// List of nodes in order of the nodes that where visited to get the shortest path from the
    /// start to the destination node.
    ///
    /// Must have at least 2 nodes (case where the starting node is also the destination)!
    path: Vec<TwoDimensionalNode>,
}

impl AStarSearchResult {
    /// Creates a new instance of `AStarSearchResult` with the specified distance and path.
    ///
    /// This function initializes a new `AStarSearchResult`, ensuring that the provided
    /// path contains at least two nodes and that the distance is non-negative.
    ///
    /// # Parameters
    ///
    /// * `distance`: A non-negative float representing the distance between the start
    ///   and end nodes. It is crucial for evaluating the effectiveness of the path.
    ///
    /// * `path`: A `Vec<TwoDimensionalNode>` that represents the path. This vector
    ///   must contain at least two nodes, which are essential for defining a valid path.
    ///
    /// # Errors
    ///
    /// Returns an `Err(String)` if any of the following conditions are met:
    /// - The `path` contains fewer than two nodes.
    /// - The `distance` is less than zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let path = vec![TwoDimensionalNode::new(0.0, 0.0), TwoDimensionalNode::new(1.0, 1.0)];
    /// let result = AStarSearchResult::new(5.0, path);
    /// assert!(result.is\_ok());
    /// ```
    ///
    /// # Returns
    ///
    ///  => `Ok(AStarSearchResult)` if the path is valid and the distance is non-negative.
    pub fn new(distance: f32, path: Vec<TwoDimensionalNode>) -> Result<Self, String> {
        // path needs to have at least 2 nodes
        if path.len() < 2 {
            return Err(
                "A valid result must have a path with atleast two representative nodes!"
                    .to_string(),
            );
        }

        // the distance must not be negative
        if distance < 0. {
            return Err(
                "A distance from a node A to B can not be less smaller then 0!".to_string(),
            );
        }

        Ok(Self { path, distance })
    }
}

impl Display for AStarSearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_path = format!("{}", self.path[0]);
        for node in &self.path[1..] {
            formatted_path = format!("{} -> {}", formatted_path, node.get_id())
        }
        write!(f, "Path: {}\n Distance: {}", formatted_path, self.distance)
    }
}

impl SearchResult for AStarSearchResult {
    type Node = TwoDimensionalNode;

    type Distance = f32;

    fn get_path(&self) -> &Vec<Self::Node> {
        &self.path
    }

    fn get_total_distance(&self) -> Self::Distance {
        self.distance
    }
}

// ----- Implementation of the 'AStarQueueElement' struct -----

/// Represents an element in the A* priority queue.
///
/// # Fields
///
/// - 'node' -> Reference to the graph node.
/// - 'g_cost' -> Cost from start to this node.
/// - 'h_cost' -> Heuristic cost estimate to goal.
/// - 'f_cost' -> Total estimated cost (g + h).
#[derive(Debug)]
struct AStarQueueElement<'n, N: GraphNode> {
    /// Reference to the graph node
    node: &'n N,
    /// Cost from start to this node
    g_cost: f32,
    /// Heuristic cost estimate to goal
    h_cost: f32,
    /// Total estimated cost (g + h)
    ///
    /// Is public so that the priority queue can access it for ordering and mutated.
    ///
    /// Can be adjusted for weighted graphs by multiplying with a weight factor if needed.
    pub f_cost: f32,
}

impl<'n, N: GraphNode> AStarQueueElement<'n, N> {
    /// Create a new 'AStarQueueElement' instance.
    ///
    /// # Arguments
    ///
    /// -> 'node' -> Reference to the graph node.
    /// -> 'g_cost' -> Cost from start to this node.
    /// -> 'h_cost' -> Heuristic cost estimate to goal.
    ///
    /// # Returns
    ///
    /// => New 'AStarQueueElement' object
    ///
    /// # Example
    ///
    /// ```rust
    /// use your_crate::graphs::graph::Node;
    /// use your_crate::algorithms::a_star::AStarQueueElement;
    ///
    /// let node = Node::new(1);
    /// let g_cost = 10;
    /// let h_cost = 20;
    /// let element = AStarQueueElement::new(&node, g_cost, h_cost);
    /// ```
    pub fn new(node: &'n N, g_cost: f32, h_cost: f32) -> AStarQueueElement<'n, N> {
        AStarQueueElement {
            node,
            g_cost,
            h_cost,
            f_cost: g_cost + h_cost,
        }
    }

    /// Get the node reference that this queue element represents.
    ///
    /// # Returns
    ///
    /// => Reference to the node.
    pub fn get_node(&self) -> &'n N {
        self.node
    }

    /// Get the g(n) cost from start to this node.
    ///
    /// # Returns
    ///
    /// => g(n) cost.
    pub fn get_g_cost(&self) -> f32 {
        self.g_cost
    }

    /// Get the h(n) heuristic cost estimate to the goal.
    ///
    /// # Returns
    ///
    /// => h(n) cost.
    pub fn get_h_cost(&self) -> f32 {
        self.h_cost
    }
}

impl<'n, N: GraphNode> Ord for AStarQueueElement<'n, N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_cost.total_cmp(&other.f_cost).reverse()
    }
}

impl<'n, N: GraphNode> PartialOrd for AStarQueueElement<'n, N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'n, N: GraphNode> PartialEq for AStarQueueElement<'n, N> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.f_cost == other.f_cost
    }
}

impl<'n, N: GraphNode> Eq for AStarQueueElement<'n, N> {}

// ----- Implementation of the 'AStarExecutionError' struct -----

/// **Struct**
///
/// In case any error occured during duing the runtime of the A* algorithm, this struct represents
/// the error and holds the important information.
///
/// # Fields
///
/// - *message* -> descriptive message of the error
#[derive(Debug)]
pub struct AStarExecutionError {
    /// Message of the error
    pub message: String,
    // potentially more fields
}

impl AStarExecutionError {
    /// Create a new 'AStarExecutionError' instance.
    ///
    /// # Arguments
    ///
    /// -> 'message' -> Description of what caused the error to occur.
    ///
    /// # Returns
    ///
    /// => New 'AStarExecutionError' object
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for AStarExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A* Execution Error: {}", self.message)
    }
}

impl Error for AStarExecutionError {}
