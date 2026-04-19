//! A* shortest-path implementation for coordinate-based graphs.
//!
//! # Overview
//!
//! This module provides:
//! - [`AStar`]: the algorithm engine,
//! - [`AStarSearchResult`]: the output type returned by successful searches,
//! - [`AStarQueueElement`]: queue payload used by the internal priority queue,
//! - [`AStarExecutionError`]: execution error type.
//!
//! # Algorithm Model
//!
//! The implementation follows the common A* scoring model:
//! - $g(n)$: known cost from start to node $n$,
//! - $h(n)$: heuristic estimate from $n$ to the goal,
//! - $f(n) = g(n) + h(n)$: priority score used in the open queue.
//!
//! The heuristic uses a cross-product based estimate derived from start, goal,
//! and current coordinates and then applies `adjust_for_heuristic()` from
//! [`NumericDatatype`].
//!
//! # Requirements
//!
//! - Graph must implement [`Graph`] with node type implementing [`CoordinatesNode`].
//! - The graph must be weighted (`graph.is_weighted() == true`).
//! - `start_node_id` and `end_node_id` must exist in the graph.
//!
//! # Usage Example
//!
//! ```no_run
//! use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStar;
//! use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
//! use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
//! use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
//!
//! let start = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
//! let graph = TwoDimensionalCoordinateGraph::new(vec![start], vec![]);
//! let algorithm = AStar::new(graph);
//!
//! // Minimal valid query where start and destination are identical.
//! let result = algorithm.shortest_path("A", "A").unwrap();
//! assert_eq!(result.get_total_distance(), 0.0_f32);
//! assert_eq!(result.get_path().len(), 1);
//! ```
//!
//! # Sources
//!
//! - <https://www.datacamp.com/tutorial/a-star-algorithm>
//! - <https://www.geeksforgeeks.org/dsa/a-search-algorithm/>
//! - <https://theory.stanford.edu/~amitp/GameProgramming/AStarComparison.html>

use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fmt::Display,
};

use log::warn;

use crate::{
    algorithms::{
        a_star_algorithm::utils::{determine_path_cost, prepare_g_cost_map},
        algorithm::{Algorithm, SearchResult},
    },
    graphs::graph::Graph,
    nodes::trait_decl::{
        coordinate_datatype::CoordinateDatatype, coordinates_node::CoordinatesNode,
    },
    weight_types::numeric_datatype::NumericDatatype,
};

/// A* pathfinding engine for coordinate-aware graph nodes.
///
/// # Type Parameters
///
/// - `WD`: numeric datatype used for graph edge weights and path costs.
/// - `N`: node type implementing [`CoordinatesNode`] for heuristic coordinates.
/// - `G`: graph type implementing [`Graph<Node = N, Weight = WD>`].
///
/// # Responsibilities
///
/// - Resolve start and goal nodes by ID.
/// - Execute the open/closed queue loop.
/// - Reconstruct path and distance.
///
/// # Example
///
/// ```no_run
/// use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStar;
/// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
/// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
///
/// let n = TwoDimensionalNode::new(1, 2, "S".to_string()).unwrap();
/// let graph = TwoDimensionalCoordinateGraph::new(vec![n], vec![]);
/// let _algorithm = AStar::new(graph);
/// ```
#[derive(Debug)]
pub struct AStar<WD: NumericDatatype, N: CoordinatesNode, G: Graph<Node = N, Weight = WD> + Display>
{
    /// Graph instance used as the search domain.
    ///
    /// This field is public to keep interoperability with existing integration
    /// points in the project.
    pub graph: G,
}

// ----- Implementation of the 'A_Star' struct -----

impl<WD: NumericDatatype, N: CoordinatesNode, G: Graph<Node = N, Weight = WD> + Display> Algorithm
    for AStar<WD, N, G>
{
    type AlgorithmSearchResult = AStarSearchResult<WD, N>;

    type NodeOfUsedGraph = N;

    type ExecutionError = AStarExecutionError;

    /// Computes a shortest path between two node IDs using A*.
    ///
    /// # Parameters
    ///
    /// - `start_node_id`: identifier of the start node.
    /// - `end_node_id`: identifier of the destination node.
    ///
    /// # Returns
    ///
    /// - `Ok(AStarSearchResult<...>)` when a route can be produced.
    /// - `Err(AStarExecutionError)` when graph constraints are violated or
    ///   required nodes cannot be found.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - graph is not weighted,
    /// - `start_node_id` does not exist,
    /// - `end_node_id` does not exist,
    /// - path reconstruction fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStar;
    /// use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
    /// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let node = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
    /// let graph = TwoDimensionalCoordinateGraph::new(vec![node], vec![]);
    /// let a_star = AStar::new(graph);
    ///
    /// let result = a_star.shortest_path("A", "A").unwrap();
    /// assert_eq!(result.get_total_distance(), 0.0_f32);
    /// assert_eq!(result.get_path().len(), 1);
    /// ```
    fn shortest_path(
        &self,
        start_node_id: &str,
        end_node_id: &str,
    ) -> Result<Self::AlgorithmSearchResult, Self::ExecutionError> {
        if !self.graph.is_weighted() {
            return Err(Self::ExecutionError::new(
                "The graph needs to be weighted for the A* algorithm to work!".to_string(),
            ));
        }

        let start_node = self.graph.get_node_by_id(start_node_id).ok_or_else(|| {
            Self::ExecutionError::new(format!(
                "Start node with id '{}' not found in the graph!",
                start_node_id
            ))
        })?;

        let end_node = self.graph.get_node_by_id(end_node_id).ok_or_else(|| {
            Self::ExecutionError::new(format!(
                "End node with id '{}' not found in the graph!",
                end_node_id
            ))
        })?;

        // "open" queue with nodes that haven't been visited yet
        // add the start node to the queue
        let mut open_queue: BinaryHeap<AStarQueueElement<WD, N>> = BinaryHeap::new();

        open_queue.push(AStarQueueElement::new(
            start_node,
            WD::zero(),
            self.heuristic(start_node, end_node, start_node),
            None,
        ));

        // "closed" queue -> nodes that have been visited
        let mut closed_queue: Vec<AStarQueueElement<WD, N>> = Vec::new();

        let mut g_costs: HashMap<String, WD> = prepare_g_cost_map(&self.graph, start_node.get_id());

        // while open is not empty -> continue
        while let Some(AStarQueueElement {
            node,
            g_cost,
            f_cost: _,
            h_cost,
            predecessor,
        }) = open_queue.pop()
        {
            // if the node is the destination node -> break
            if node == end_node {
                // move the node to the "closed_queue", don't change any data of the node, because
                // we need the predecessor for the path reconstruction
                closed_queue.push(AStarQueueElement::new(node, g_cost, h_cost, predecessor));
                break;
            }
            // add the node to the "closed_queue"
            closed_queue.push(AStarQueueElement::new(node, g_cost, h_cost, predecessor));
            // get all neighbours and check if ... -> add all neighbours to "open_queue" + add
            // current node to the "closed_queue"
            for (neighbour, weight) in self.graph.neighbors(node) {
                let tentative_g_cost = g_cost + weight;
                // get the 'g_cost' of the neighbour from the "open_queue" if it exists
                // if the neighbour is already in the "open_queue" and the 'g_cost' is higher than
                // the 'tentative_g_cost' -> update the 'g_cost' of the neighbour in the
                // "open_queue" and set the current node as the predecessor of the neighbour

                let mut neighbour_is_in_open_queue = open_queue
                    .iter()
                    .any(|e| e.get_node().get_id() == neighbour.get_id());

                let mut neighbour_is_in_closed_queue = closed_queue
                    .iter()
                    .any(|e| e.get_node().get_id() == neighbour.get_id());

                let g_cost_neighbour = g_costs.get(neighbour.get_id());

                if neighbour_is_in_open_queue
                    && let Some(o_g_cost) = g_cost_neighbour
                    && tentative_g_cost < *o_g_cost
                {
                    open_queue.retain(|e| e.get_node().get_id() != neighbour.get_id());
                }

                // if the neighbour is already in the "closed_queue" and the 'g_cost' is higher
                // than the 'tentative_g_cost' -> update the 'g_cost' of the neighbour in the
                // "closed_queue" and set the current node as the predecessor of the neighbour
                // let g_cost_neighbour_closed_queue = g_costs.get(neighbour.get_id());
                if neighbour_is_in_closed_queue
                    && let Some(c_g_cost) = g_cost_neighbour
                    && tentative_g_cost < *c_g_cost
                {
                    closed_queue.retain(|e| e.get_node().get_id() != neighbour.get_id());
                }

                // if the neighbour is not in the "open_queue" and not in the "closed_queue" ->
                // add the neighbour to the "open_queue" and set the current node as the predecessor
                neighbour_is_in_open_queue = open_queue
                    .iter()
                    .any(|e| e.get_node().get_id() == neighbour.get_id());

                neighbour_is_in_closed_queue = closed_queue
                    .iter()
                    .any(|e| e.get_node().get_id() == neighbour.get_id());

                if !neighbour_is_in_open_queue && !neighbour_is_in_closed_queue {
                    g_costs.insert(neighbour.get_id().to_string(), tentative_g_cost);
                    open_queue.push(AStarQueueElement::new(
                        neighbour,
                        tentative_g_cost,
                        self.heuristic(start_node, end_node, neighbour),
                        Some(node),
                    ));
                }
            }
        }

        // the last element in the "closed_queue" is the destination node, so we can reconstruct
        // the path from the destination node to the start node by following the predecessors
        let (path, distance) =
            determine_path_cost(closed_queue).map_err(|e| Self::ExecutionError::new(e.message))?;

        AStarSearchResult::new(distance, path).map_err(Self::ExecutionError::new)
    }
}

impl<WD: NumericDatatype, N: CoordinatesNode, G: Graph<Node = N, Weight = WD> + Display>
    AStar<WD, N, G>
{
    /// Creates a new [`AStar`] instance bound to `graph`.
    ///
    /// # Parameters
    ///
    /// - `graph`: concrete graph used during subsequent path search calls.
    ///
    /// # Returns
    ///
    /// A ready-to-use algorithm instance.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStar;
    /// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
    ///
    /// let graph = TwoDimensionalCoordinateGraph::<i32>::new(vec![], vec![]);
    /// let _a_star = AStar::new(graph);
    /// ```
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Internal heuristic function used for A* queue prioritization.
    ///
    /// # Heuristic Formula
    ///
    /// Uses a cross-product magnitude based estimate:
    /// - build vectors `(current -> goal)` and `(start -> goal)`,
    /// - compute their cross-product magnitude,
    /// - scale the value for heuristic usage,
    /// - convert the estimate to the graph weight type.
    ///
    /// # Parameters
    ///
    /// - `start`: start node.
    /// - `goal`: destination node.
    /// - `current`: node currently being evaluated.
    ///
    /// # Returns
    ///
    /// Heuristic estimate from `current` toward `goal`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Internal helper called during queue expansion.
    /// // It is not part of the public API.
    /// let estimate = a_star.heuristic(start, goal, current);
    /// ```
    fn heuristic(&self, start: &N, goal: &N, current: &N) -> WD {
        let dx1 = current.get_x().to_f32() - goal.get_x().to_f32();
        let dy1 = current.get_y().to_f32() - goal.get_y().to_f32();
        let dx2 = start.get_x().to_f32() - goal.get_x().to_f32();
        let dy2 = start.get_y().to_f32() - goal.get_y().to_f32();

        // Cross product magnitude for heuristic estimation in coordinate space.
        let cross = (dx1 * dy2 - dx2 * dy1).abs();

        // Convert the scaled heuristic to the graph's weight datatype.
        WD::from_f32(cross.adjust_for_heuristic())
    }
}

// ----- Implementation of the 'AStarSearchResult' struct -----

/// Result object returned by A* search execution.
///
/// # Contents
///
/// - `distance`: total path cost.
/// - `path`: ordered node sequence from start to destination.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStarSearchResult;
/// use shortest_path_finder::algorithms::algorithm::SearchResult;
/// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
///
/// let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
/// let b = TwoDimensionalNode::new(0, 1, "B".to_string()).unwrap();
/// let result = AStarSearchResult::new(1, vec![a, b]).unwrap();
///
/// assert_eq!(result.get_total_distance(), 1);
/// assert_eq!(result.get_path().len(), 2);
/// ```
#[derive(Debug)]
pub struct AStarSearchResult<WD: NumericDatatype, N: CoordinatesNode> {
    /// Total distance from one node A to node B.
    distance: WD,
    /// List of nodes in order of the nodes that where visited to get the shortest path from the
    /// start to the destination node.
    ///
    /// Must have at least 1 node (the `start == destination` case produces a single-node path).
    path: Vec<N>,
}

impl<WD: NumericDatatype, N: CoordinatesNode> AStarSearchResult<WD, N> {
    /// Creates a validated [`AStarSearchResult`] instance.
    ///
    /// # Validation Rules
    ///
    /// - `path` must contain at least one node.
    /// - `distance` must be greater than or equal to zero.
    ///
    /// # Parameters
    ///
    /// - `distance`: total route cost.
    /// - `path`: ordered route nodes.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` when inputs are valid.
    /// - `Err(String)` when one or more rules are violated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStarSearchResult;
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let path = vec![
    ///     TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap(),
    ///     TwoDimensionalNode::new(1, 1, "B".to_string()).unwrap(),
    /// ];
    /// assert!(AStarSearchResult::new(5, path).is_ok());
    /// ```
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStarSearchResult;
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let invalid_path: Vec<TwoDimensionalNode> = vec![];
    /// assert!(AStarSearchResult::new(0, invalid_path).is_err());
    /// ```
    pub fn new(distance: WD, path: Vec<N>) -> Result<Self, String> {
        // path needs to have at least 1 node
        if path.is_empty() {
            return Err("A valid result must have at least one representative node!".to_string());
        }

        // the distance must not be negative
        if distance < WD::zero() {
            return Err(
                "A distance from a node A to B can not be less smaller then 0!".to_string(),
            );
        }

        Ok(Self { path, distance })
    }
}

impl<WD: NumericDatatype, N: CoordinatesNode> Display for AStarSearchResult<WD, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_path = self.path[0].get_id().to_string();
        for node in &self.path[1..] {
            formatted_path = format!("{} -> {}", formatted_path, node.get_id())
        }
        write!(f, "Path: {}\n Distance: {}", formatted_path, self.distance)
    }
}

impl<WD: NumericDatatype, N: CoordinatesNode> SearchResult for AStarSearchResult<WD, N> {
    type Node = N;

    type Distance = WD;

    fn get_path(&self) -> &Vec<Self::Node> {
        &self.path
    }

    fn get_total_distance(&self) -> Self::Distance {
        self.distance
    }
}

// ----- Implementation of the 'AStarQueueElement' struct -----

/// Element stored in the A* priority queue.
///
/// # Purpose
///
/// Each queue entry tracks:
/// - current node,
/// - predecessor (for path reconstruction),
/// - `g(n)`, `h(n)`, and `f(n)` values.
///
/// # Ordering
///
/// Ordering is implemented on `f_cost` with reversed comparison to emulate a
/// min-heap on top of Rust's `BinaryHeap`.
///
/// # Type Parameters
///
/// - `WD`: numeric cost type implementing [`NumericDatatype`].
/// - `N`: coordinate-based node type.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStarQueueElement;
/// use shortest_path_finder::graphs::graph::GraphNode;
/// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
///
/// let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
/// let b = TwoDimensionalNode::new(1, 0, "B".to_string()).unwrap();
/// let mut element = AStarQueueElement::new(&a, 2_i32, 3_i32, None);
///
/// assert_eq!(element.f_cost, 5_i32);
/// element.set_predecessor(Some(&b));
/// assert_eq!(element.get_predecessor().unwrap().get_id(), "B");
/// ```
#[derive(Debug)]
pub struct AStarQueueElement<'n, WD: NumericDatatype, N: CoordinatesNode> {
    /// Reference to the current node in the graph.
    ///
    /// This points to the node data structure representing the current position in the graph.
    node: &'n N,

    /// Optional reference to the predecessor node.
    ///
    /// Used for path reconstruction after reaching the goal.
    /// If `None`, this node is the start node.
    predecessor: Option<&'n N>,

    /// Cost from the start node to this node (`g(n)`).
    ///
    /// Represents the accumulated cost along the path from the start to this node.
    g_cost: WD,

    /// Estimated cost from this node to the goal (`h(n)`).
    ///
    /// Typically calculated using a heuristic function (e.g., Euclidean distance).
    h_cost: WD,

    /// Total estimated cost of the path through this node (`f(n) = g(n) + h(n)`).
    ///
    /// This value is public so that priority queues can access and compare it directly.
    /// It may be adjusted for weighted graphs by applying a weight factor.
    pub f_cost: WD,
}

impl<'n, WD: NumericDatatype, N: CoordinatesNode> AStarQueueElement<'n, WD, N> {
    /// Creates a queue element with precomputed score components.
    ///
    /// # Parameters
    /// - `node`: current node.
    /// - `g_cost`: known path cost from start to `node`.
    /// - `h_cost`: heuristic estimate from `node` to goal.
    /// - `predecessor`: previous node on best known route.
    ///
    /// # Returns
    /// Queue element where `f_cost = g_cost + h_cost`.
    pub fn new(node: &'n N, g_cost: WD, h_cost: WD, predecessor: Option<&'n N>) -> Self {
        // Calculate the total estimated cost for this node.
        let f_cost = g_cost + h_cost;
        AStarQueueElement {
            node,
            g_cost,
            h_cost,
            f_cost,
            predecessor,
        }
    }
    /// Updates the predecessor reference.
    ///
    /// # Parameters
    ///
    /// - `predecessor`: optional predecessor node used during path reconstruction.
    pub fn set_predecessor(&mut self, predecessor: Option<&'n N>) {
        self.predecessor = predecessor;
    }

    /// Returns the predecessor node reference, if present.
    ///
    /// # Returns
    ///
    /// Optional predecessor.
    pub fn get_predecessor(&self) -> Option<&'n N> {
        self.predecessor
    }

    /// Returns the node represented by this queue entry.
    ///
    /// # Returns
    ///
    /// Node reference.
    pub fn get_node(&self) -> &'n N {
        self.node
    }

    /// Returns `g(n)` for this queue entry.
    ///
    /// # Returns
    ///
    /// Cost from start to current node.
    pub fn get_g_cost(&self) -> WD {
        self.g_cost
    }

    /// Returns `h(n)` for this queue entry.
    ///
    /// # Returns
    ///
    /// Heuristic estimate from current node to goal.
    pub fn get_h_cost(&self) -> WD {
        self.h_cost
    }
}

impl<'n, WD: NumericDatatype, N: CoordinatesNode> Ord for AStarQueueElement<'n, WD, N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.f_cost.partial_cmp(&other.f_cost) {
            Some(ordering) => ordering.reverse(), // Reverse for min-heap behavior
            None => {
                warn!("Comparison of f_cost resulted in NaN. Treating as equal for ordering.");
                std::cmp::Ordering::Equal
            } // Treat NaN as equal (or handle as needed)
        }
    }
}

impl<'n, WD: NumericDatatype, N: CoordinatesNode> PartialOrd for AStarQueueElement<'n, WD, N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'n, WD: NumericDatatype, N: CoordinatesNode> PartialEq for AStarQueueElement<'n, WD, N> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.f_cost == other.f_cost
    }
}

impl<'n, WD: NumericDatatype, N: CoordinatesNode> Eq for AStarQueueElement<'n, WD, N> {}

// ----- Implementation of the 'AStarExecutionError' struct -----

/// Error type returned by A* execution and helper utilities.
///
/// # Contents
///
/// - `message`: user-facing diagnostic description.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStarExecutionError;
///
/// let err = AStarExecutionError::new("missing start node".to_string());
/// assert!(err.to_string().contains("missing start node"));
/// ```
#[derive(Debug)]
pub struct AStarExecutionError {
    /// Message of the error
    pub message: String,
    // potentially more fields
}

impl AStarExecutionError {
    /// Creates a new execution error with a custom message.
    ///
    /// # Parameters
    ///
    /// - `message`: descriptive error text.
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
