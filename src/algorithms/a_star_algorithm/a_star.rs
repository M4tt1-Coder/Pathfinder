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
    nodes::trait_decl::{coordinates_node::CoordinatesNode, numeric_datatype::NumericDatatype},
};

/// Represents the A* pathfinding algorithm, parameterized over numeric type, node type, and graph type.
///
/// This struct encapsulates the graph on which the algorithm operates and provides a foundation
/// for implementing pathfinding logic.
#[derive(Debug)]
pub struct AStar<
    ND: NumericDatatype,
    N: CoordinatesNode<CoordinateType = ND>,
    G: Graph<Node = N, Weight = ND> + Display,
> {
    /// The graph on which the A* algorithm will operate.
    ///
    /// This graph should implement the `Graph` trait with nodes of type `N`.
    /// It must also implement `Display` for potential visualization or debugging purposes.
    pub graph: G,
}

// ----- Implementation of the 'A_Star' struct -----

impl<
    ND: NumericDatatype,
    N: CoordinatesNode<CoordinateType = ND>,
    G: Graph<Node = N, Weight = ND> + Display,
> Algorithm for AStar<ND, N, G>
{
    type AlgorithmSearchResult = AStarSearchResult<ND, N>;

    type NodeOfUsedGraph = N;

    type ExecutionError = AStarExecutionError;

    fn shortest_path(
        &self,
        start: &N,
        end: &N,
    ) -> Result<Self::AlgorithmSearchResult, Self::ExecutionError> {
        // "open" queue with nodes that haven't been visited yet
        // add the start node to the queue
        let mut open_queue: BinaryHeap<AStarQueueElement<ND, N>> = BinaryHeap::new();

        open_queue.push(AStarQueueElement::new(
            start,
            ND::zero(),
            self.heuristic(start, end, start),
            None,
        ));

        // "closed" queue -> nodes that have been visited
        let mut closed_queue: Vec<AStarQueueElement<ND, N>> = Vec::new();

        let mut g_costs: HashMap<String, ND> = prepare_g_cost_map(&self.graph, start.get_id());

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
            if node == end {
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
                        self.heuristic(start, end, neighbour),
                        Some(node),
                    ));
                }
            }
        }

        // the last element in the "closed_queue" is the destination node, so we can reconstruct
        // the path from the destination node to the start node by following the predecessors
        let (path, distance) =
            determine_path_cost(closed_queue).map_err(|e| Self::ExecutionError::new(e.message))?;

        Ok(AStarSearchResult { distance, path })
    }
}

impl<
    ND: NumericDatatype,
    N: CoordinatesNode<CoordinateType = ND>,
    G: Graph<Node = N, Weight = ND> + Display,
> AStar<ND, N, G>
{
    /// Creates a new instance of the AStar algorithm with the provided graph.
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Calculates the heuristic estimate (often called h(n)) from the current node to the goal.
    ///
    /// This implementation uses a cross-product based heuristic, which is often used for
    /// certain types of pathfinding problems (e.g., for detecting the area swept by a point).
    ///
    /// It computes the absolute value of the cross product of vectors from the current node to the goal
    /// and from the start to the goal, then adjusts it for heuristic admissibility.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting node.
    /// * `goal` - The target node.
    /// * `current` - The current node for which the heuristic is being calculated.
    ///
    /// # Returns
    ///
    /// The estimated cost (`ND`) from `current` to `goal`.
    fn heuristic(&self, start: &N, goal: &N, current: &N) -> ND {
        let dx1 = current.get_x() - goal.get_x();
        let dy1 = current.get_y() - goal.get_y();
        let dx2 = start.get_x() - goal.get_x();
        let dy2 = start.get_y() - goal.get_y();

        // Cross product magnitude for heuristic estimation
        let cross = (dx1 * dy2 - dx2 * dy1).abs();

        // Adjust the heuristic to ensure admissibility and prevent overestimation
        cross.adjust_for_heuristic()
        // You can multiply by a small factor like 0.001 if needed to fine-tune the heuristic
        // * 0.001
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
pub struct AStarSearchResult<ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> {
    /// Total distance from one node A to node B.
    distance: ND,
    /// List of nodes in order of the nodes that where visited to get the shortest path from the
    /// start to the destination node.
    ///
    /// Must have at least 2 nodes (case where the starting node is also the destination)!
    path: Vec<N>,
}

impl<ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> AStarSearchResult<ND, N> {
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
    pub fn new(distance: ND, path: Vec<N>) -> Result<Self, String> {
        // path needs to have at least 2 nodes
        if path.len() < 2 {
            return Err(
                "A valid result must have a path with atleast two representative nodes!"
                    .to_string(),
            );
        }

        // the distance must not be negative
        if distance < ND::zero() {
            return Err(
                "A distance from a node A to B can not be less smaller then 0!".to_string(),
            );
        }

        Ok(Self { path, distance })
    }
}

impl<ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> Display
    for AStarSearchResult<ND, N>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_path = format!("{}", self.path[0]);
        for node in &self.path[1..] {
            formatted_path = format!("{} -> {}", formatted_path, node.get_id())
        }
        write!(f, "Path: {}\n Distance: {}", formatted_path, self.distance)
    }
}

impl<ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> SearchResult
    for AStarSearchResult<ND, N>
{
    type Node = N;

    type Distance = ND;

    fn get_path(&self) -> &Vec<Self::Node> {
        &self.path
    }

    fn get_total_distance(&self) -> Self::Distance {
        self.distance
    }
}

// ----- Implementation of the 'AStarQueueElement' struct -----

/// Represents an element in the priority queue used by the A* search algorithm.
///
/// This struct encapsulates all the necessary information for a node during the search,
/// including references to the graph node, costs, and path reconstruction data.
///
/// # Lifetime
/// - `'n`: Lifetime for references to nodes, ensuring they do not outlive the graph data.
///
/// # Type Parameters
/// - `ND`: Numeric datatype used for costs (e.g., `f64`, `u32`). Must implement traits like `PartialOrd`, `Ord`.
/// - `N`: The node type in your graph. Must implement `CoordinatesNode` with `CoordinateType = ND`.
#[derive(Debug)]
pub struct AStarQueueElement<'n, ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> {
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
    g_cost: ND,

    /// Estimated cost from this node to the goal (`h(n)`).
    ///
    /// Typically calculated using a heuristic function (e.g., Euclidean distance).
    h_cost: ND,

    /// Total estimated cost of the path through this node (`f(n) = g(n) + h(n)`).
    ///
    /// This value is public so that priority queues can access and compare it directly.
    /// It may be adjusted for weighted graphs by applying a weight factor.
    pub f_cost: ND,
}
impl<'n, ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>>
    AStarQueueElement<'n, ND, N>
{
    /// Creates a new `AStarQueueElement` with the provided node and costs.
    ///
    /// # Parameters
    /// - `node`: A reference to the current graph node.
    /// - `g_cost`: The cost from the start node to this node (`g(n)`).
    /// - `h_cost`: The heuristic estimate of the cost from this node to the goal (`h(n)`).
    /// - `predecessor`: An optional reference to the previous node in the path. This is used for path reconstruction.
    ///
    /// # Returns
    /// A new instance of `AStarQueueElement` with the `f_cost` calculated as `g_cost + h_cost`.
    pub fn new(node: &'n N, g_cost: ND, h_cost: ND, predecessor: Option<&'n N>) -> Self {
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
    /// Set the predecessor node for this queue element.
    ///
    /// # Arguments
    ///
    /// * `predescessor` - An optional reference to the predecessor node. This is used for path
    ///   reconstruction after reaching the goal.
    ///
    /// # Returns
    ///
    /// => None (this method mutates the internal state of the queue element)
    pub fn set_predecessor(&mut self, predecessor: Option<&'n N>) {
        self.predecessor = predecessor;
    }

    /// Get the predecessor node reference for this queue element.
    ///
    /// # Returns
    ///
    /// => An optional reference to the predecessor node, if it exists. This can be used for path
    /// reconstruction after reaching the goal.
    ///
    /// If the predecessor is `None`, it indicates that this node is the starting node or that the
    /// predecessor has not been set yet.
    ///
    /// Note: The predecessor reference is crucial for backtracking the path from the goal to the
    /// starting node once the goal is reached.
    /// It allows the algorithm to reconstruct the path taken to reach the goal by following the
    /// chain of predecessor nodes.
    pub fn get_predecessor(&self) -> Option<&'n N> {
        self.predecessor
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
    pub fn get_g_cost(&self) -> ND {
        self.g_cost
    }

    /// Get the h(n) heuristic cost estimate to the goal.
    ///
    /// # Returns
    ///
    /// => h(n) cost.
    pub fn get_h_cost(&self) -> ND {
        self.h_cost
    }
}

impl<'n, ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> Ord
    for AStarQueueElement<'n, ND, N>
{
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

impl<'n, ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> PartialOrd
    for AStarQueueElement<'n, ND, N>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'n, ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> PartialEq
    for AStarQueueElement<'n, ND, N>
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.f_cost == other.f_cost
    }
}

impl<'n, ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>> Eq
    for AStarQueueElement<'n, ND, N>
{
}

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
