//! Dijkstra shortest-path algorithm implementation.
//!
//! This module contains the concrete Dijkstra implementation used by the
//! application. It supports graph types that implement [`Graph`] and uses a
//! priority queue (`BinaryHeap`) to iteratively relax edges.
//!
//! Dijkstra requires all traversed edge weights to be non-negative. If a
//! negative edge weight is encountered during processing, the algorithm returns
//! a [`DijkstraError`].
//!
//! # Main types
//!
//! - [`DijkstraAlgorithm`]: algorithm engine operating on a concrete graph.
//! - [`DijkstraSearchResult`]: successful path computation output.
//! - [`DijkstraError`]: execution error payload.
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
//! use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
//! use shortest_path_finder::graphs::directed::{DirectedEdge, DirectedGraph};
//! use shortest_path_finder::graphs::graph::Graph;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let mut graph = DirectedGraph::default();
//! let a = DefaultNode::new("A".to_string());
//! let b = DefaultNode::new("B".to_string());
//! let c = DefaultNode::new("C".to_string());
//! graph.insert_node(a.clone());
//! graph.insert_node(b.clone());
//! graph.insert_node(c.clone());
//! assert!(graph.insert_edge(DirectedEdge::new(a.clone(), b.clone(), 4)).is_none());
//! assert!(graph.insert_edge(DirectedEdge::new(b, c.clone(), 2)).is_none());
//! assert!(graph.insert_edge(DirectedEdge::new(a, c, 10)).is_none());
//!
//! let dijkstra = DijkstraAlgorithm::new(graph);
//! let result = dijkstra.shortest_path("A", "C").unwrap();
//!
//! assert_eq!(result.get_total_distance(), 6);
//! assert_eq!(result.get_path().len(), 3);
//! ```

use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    algorithms::algorithm::{Algorithm, SearchResult},
    graphs::graph::{Graph, GraphNode, GraphWeight},
};

/// Internal bookkeeping entry used while distances are being relaxed.
///
/// Each node maps to one instance of this type while the algorithm is running:
/// - `distance` stores the currently known best distance from the start node.
/// - `previous_node` stores the predecessor used to reconstruct the final path.
#[derive(Debug)]
pub struct ShortestDistance<N: GraphNode, W: GraphWeight + Ord> {
    distance: W,
    previous_node: Option<N>,
}

impl<N: GraphNode, W: GraphWeight + Ord> ShortestDistance<N, W> {
    /// Creates a new internal distance-tracking entry.
    ///
    /// # Parameters
    /// - `previous_node`: Optional predecessor of the current node.
    /// - `distance`: Current best-known distance from the start node.
    ///
    /// # Returns
    /// A new [`ShortestDistance`] value.
    ///
    /// # Example (internal-only helper)
    /// ```ignore
    /// // Used by Dijkstra's internal state map.
    /// let start_node = None;
    /// let initial_distance = 0u16;
    /// let node = ShortestDistance::new(start_node, initial_distance);
    /// ```
    fn new(previous_node: Option<N>, distance: W) -> Self {
        Self {
            previous_node,
            distance,
        }
    }
}

impl<N: GraphNode, W: GraphWeight + Ord> Display for ShortestDistance<N, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ PrevNode: {:?}, Distance: {} ]",
            self.previous_node, self.distance
        )
    }
}

/// Concrete implementation of the Dijkstra shortest-path algorithm.
///
/// The generic parameters are:
/// - `N`: graph node type.
/// - `W`: edge-weight/distance type.
/// - `G`: graph type implementing [`Graph`].
///
/// # Requirements
///
/// - The underlying graph must be weighted.
/// - Edge weights must be non-negative when the algorithm explores edges.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
/// use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
/// use shortest_path_finder::graphs::directed::{DirectedEdge, DirectedGraph};
/// use shortest_path_finder::graphs::graph::Graph;
/// use shortest_path_finder::nodes::default_node::DefaultNode;
///
/// let mut graph = DirectedGraph::default();
/// let a = DefaultNode::new("A".to_string());
/// let b = DefaultNode::new("B".to_string());
/// let c = DefaultNode::new("C".to_string());
/// graph.insert_node(a.clone());
/// graph.insert_node(b.clone());
/// graph.insert_node(c.clone());
/// assert!(graph.insert_edge(DirectedEdge::new(a.clone(), b.clone(), 1)).is_none());
/// assert!(graph.insert_edge(DirectedEdge::new(b, c.clone(), 1)).is_none());
/// assert!(graph.insert_edge(DirectedEdge::new(a, c, 5)).is_none());
///
/// let algorithm = DijkstraAlgorithm::new(graph);
/// let result = algorithm.shortest_path("A", "C").unwrap();
/// assert_eq!(result.get_total_distance(), 2u16);
/// ```
#[derive(Debug)]
pub struct DijkstraAlgorithm<
    N: GraphNode,
    W: GraphWeight + Ord,
    G: Graph<Node = N, Weight = W> + Display,
> {
    /// Graph instance processed by this algorithm implementation.
    graph: G,
}

impl<N: GraphNode, W: GraphWeight + Ord, G: Graph<Node = N, Weight = W> + Display> Algorithm
    for DijkstraAlgorithm<N, W, G>
{
    type AlgorithmSearchResult = DijkstraSearchResult<N, W>;

    type ExecutionError = DijkstraError;

    type NodeOfUsedGraph = N;

    fn shortest_path(
        &self,
        start_node_id: &str,
        end_node_id: &str,
    ) -> Result<DijkstraSearchResult<N, W>, DijkstraError> {
        // - loop:
        //  - get distance / weight of edge to all unvisited neighbours
        //  - if there is a short distance if it is shorter and the previous node
        //  - put current node into visited nodes list and remove from unvisited nodes list
        //  - choose new current node -> unvisited node with minimal distance

        // graphs need to be weighted else its not possible to calculate the distance
        if !self.graph.is_weighted() {
            return Err(DijkstraError::new(
                "The graph that was created needs to be weighted!".to_string(),
            ));
        }

        // check if the two 'Node's are in the graph <G> and get them as 'Node' objects
        let start: &N = match self.graph.get_node_by_id(start_node_id) {
            Some(node) => node,
            None => {
                return Err(DijkstraError::new(format!(
                    "The start node {} is not in the graph {}!",
                    start_node_id, self.graph
                )));
            }
        };

        let end: &N = match self.graph.get_node_by_id(end_node_id) {
            Some(node) => node,
            None => {
                return Err(DijkstraError::new(format!(
                    "The end node {} is not in the graph {}!",
                    end_node_id, self.graph
                )));
            }
        };

        let distances = self.calculate_distances(start)?;

        // search for the shortest route from the 'start' to the 'end' node
        let mut path: Vec<N> = vec![];
        let mut current_node = end.clone();
        let mut output_distance = W::zero();

        while let Some(distance) = distances.get(current_node.get_id()) {
            if current_node.get_id() == end.get_id() {
                output_distance = distance.distance;
            }
            path.push(current_node);
            let prev: &N = match &distance.previous_node {
                Some(node) => node,
                None => {
                    return Err(DijkstraError::new(format!(
                        "Unable to determine a valid path from {} to {}!",
                        start_node_id, end_node_id
                    )));
                }
            };
            if start.get_id() == prev.get_id() {
                path.push(start.clone());
                break;
            }
            current_node = prev.clone();
        }

        // check if a path really has been found
        if path.last() != Some(start) {
            return Err(DijkstraError::new("A path could not be found!".to_string()));
        }

        path.reverse();

        Ok(match DijkstraSearchResult::new(path, output_distance) {
            Ok(result) => result,
            Err(err) => return Err(DijkstraError::new(err)),
        })
    }
}

impl<N: GraphNode, W: GraphWeight + Ord, G: Graph<Node = N, Weight = W> + Display>
    DijkstraAlgorithm<N, W, G>
{
    /// Creates a new [`DijkstraAlgorithm`] bound to a graph instance.
    ///
    /// # Parameters
    ///
    /// - `graph`: Graph object implementing [`Graph`].
    ///
    /// # Returns
    ///
    /// A ready-to-use algorithm instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    ///
    /// let graph = DirectedGraph::new(vec![], vec![]);
    /// let _algorithm = DijkstraAlgorithm::new(graph);
    /// ```
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Initializes the distance map for Dijkstra processing.
    ///
    /// The start node receives distance `0` and references itself as previous node.
    /// Every other node receives `W::max_value()` and no predecessor.
    ///
    /// # Parameters
    ///
    /// - `start`: Start node used as the origin of all distance calculations.
    ///
    /// # Returns
    ///
    /// A map from node ID to internal [`ShortestDistance`] state.
    fn setup_shortest_distance(&self, start: &N) -> HashMap<String, ShortestDistance<N, W>> {
        let mut output: HashMap<String, ShortestDistance<N, W>> = HashMap::new();
        for n in self.graph.get_all_nodes() {
            if n.get_id() == start.get_id() {
                // Start node begins with distance 0 and itself as predecessor sentinel.
                output.insert(
                    n.get_id().to_string().clone(),
                    ShortestDistance {
                        distance: W::zero(),
                        previous_node: Some(n.clone()),
                    },
                );
            } else {
                // Unknown paths are initialized with "infinite" distance.
                output.insert(
                    n.get_id().to_string().clone(),
                    ShortestDistance::new(None, W::max_value()),
                );
            }
        }
        output
    }

    /// Executes the core Dijkstra relaxation loop.
    ///
    /// # Parameters
    ///
    /// - `start`: Node from which shortest distances are computed.
    ///
    /// # Returns
    ///
    /// - `Ok(HashMap<...>)` containing shortest-distance metadata for all nodes.
    /// - `Err(DijkstraError)` if graph consistency checks fail or an invalid
    ///   edge weight (negative) is encountered.
    fn calculate_distances(
        &self,
        start: &N,
    ) -> Result<HashMap<String, ShortestDistance<N, W>>, DijkstraError> {
        // - new list keeping track of the shortest distance from the start node to all others
        let mut distances: HashMap<String, ShortestDistance<N, W>> =
            self.setup_shortest_distance(start);

        // queue for leftover steps to check if they lead on the shortest path to a node
        let mut queue: BinaryHeap<QueueItem<N, W>> = BinaryHeap::new();

        queue.push(QueueItem {
            distance: W::zero(),
            position: start.clone(),
        });

        while let Some(QueueItem { distance, position }) = queue.pop() {
            // Skip stale queue entries superseded by a shorter known path.
            if distance
                > match distances.get(position.get_id()) {
                    Some(distance_data) => distance_data.distance,
                    None => {
                        return Err(DijkstraError::new(format!(
                            "Couldn't find the node {} in the graph! Please check if the original input data is valid!",
                            position
                        )));
                    }
                }
            {
                continue;
            }

            for (neighbour, weight) in self.graph.neighbors(&position) {
                // for Dijkstra an edges weight can't be smaller then 0
                if weight < W::zero() {
                    return Err(DijkstraError::new(format!(
                        "In the 'Dijkstra' algorithm only positive edge weights are allowed! Edge: [ from: {}, to: {}, weight: {} ]",
                        position, neighbour, weight
                    )));
                }

                // Standard relaxation: candidate distance via the current node.
                let updated_distance = distance + weight;

                if updated_distance
                    < match distances.get(neighbour.get_id()) {
                        Some(distance_data) => distance_data.distance,
                        None => {
                            return Err(DijkstraError::new(format!(
                                "Couldn't find the node {} in the graph! Please check if the original input data is valid!",
                                neighbour
                            )));
                        }
                    }
                {
                    // Persist better path and predecessor for later reconstruction.
                    distances
                        .entry(neighbour.get_id().to_string().clone())
                        .and_modify(|entry| {
                            entry.distance = updated_distance;
                            entry.previous_node = Some(position.clone())
                        });

                    // Re-enqueue neighbor with its improved tentative distance.
                    queue.push(QueueItem::new(updated_distance, neighbour.clone()));
                }
            }
        }
        Ok(distances)
    }
}

/// Internal priority-queue element used by the Dijkstra processing loop.
///
/// The queue stores candidate nodes ordered by distance.
#[derive(Eq, PartialEq)]
struct QueueItem<N: GraphNode, W: GraphWeight> {
    /// Candidate distance for this queue step.
    distance: W,
    /// Candidate node position associated with `distance`.
    position: N,
}

impl<N: GraphNode, W: GraphWeight + Ord + Eq> QueueItem<N, W> {
    /// Creates a new queue item.
    ///
    /// # Parameters
    ///
    /// - `distance`: Tentative distance for `position`.
    /// - `position`: Node to be processed next by the queue consumer.
    ///
    /// # Returns
    ///
    /// A new [`QueueItem`].
    fn new(distance: W, position: N) -> Self {
        Self { distance, position }
    }
}

impl<N: GraphNode, W: GraphWeight + Ord + Eq> PartialOrd for QueueItem<N, W> {
    /// Defers partial ordering to [`Ord`] for `BinaryHeap` compatibility.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: GraphNode, W: GraphWeight + Ord + Eq> Ord for QueueItem<N, W> {
    /// Orders queue entries by distance.
    ///
    /// `BinaryHeap` is a max-heap, so this ordering currently yields largest
    /// distance first. The implementation compensates by skipping stale entries
    /// when popped, which preserves correctness for this algorithm.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

/// Error returned when Dijkstra execution fails.
///
/// This type wraps a user-facing diagnostic message.
#[derive(Debug)]
pub struct DijkstraError {
    /// Human-readable explanation of the failure.
    pub message: String,
}

impl DijkstraError {
    /// Creates a new [`DijkstraError`] from a message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::dijkstra::DijkstraError;
    ///
    /// let err = DijkstraError::new("invalid input".to_string());
    /// assert_eq!(err.to_string(), "invalid input");
    /// ```
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for DijkstraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DijkstraError {}

/// Search result produced by [`DijkstraAlgorithm`].
///
/// Contains the final path and total distance of the shortest route.
#[derive(Debug, Clone)]
pub struct DijkstraSearchResult<N: GraphNode, W: GraphWeight> {
    /// Ordered node sequence from start node to destination node.
    ///
    /// The path must contain at least two nodes.
    pub path: Vec<N>,

    /// Sum of all edge weights along `path`.
    pub distance: W,
}

impl<N: GraphNode, W: GraphWeight> DijkstraSearchResult<N, W> {
    /// Creates a validated [`DijkstraSearchResult`].
    ///
    /// # Errors
    ///
    /// Returns an error if `path` contains fewer than two nodes.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` when the provided path is valid.
    /// - `Err(String)` with a detailed reason otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::dijkstra::DijkstraSearchResult;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let path = vec![
    ///     DefaultNode::new("A".to_string()),
    ///     DefaultNode::new("B".to_string()),
    /// ];
    /// let result = DijkstraSearchResult::new(path, 9u16);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::dijkstra::DijkstraSearchResult;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let invalid_path = vec![DefaultNode::new("A".to_string())];
    /// let result = DijkstraSearchResult::new(invalid_path, 0u16);
    /// assert!(result.is_err());
    /// ```
    pub fn new(path: Vec<N>, distance: W) -> Result<Self, String> {
        if path.len() < 2 {
            return Err("There need to be at least 2 nodes in the path from one node A to another node B! Couldn't create a 'SearchResult'!".to_string());
        }

        Ok(Self { path, distance })
    }
}

impl<N: GraphNode, W: GraphWeight> Display for DijkstraSearchResult<N, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path_string = String::new();
        for n in &self.path {
            path_string = format!("{} -> {}", path_string, n.get_id());
        }
        write!(
            f,
            "
            Path: {},
            Distance: {}
            ",
            path_string, self.distance
        )
    }
}

impl<N: GraphNode + Debug, W: GraphWeight> SearchResult for DijkstraSearchResult<N, W> {
    type Node = N;

    type Distance = W;

    fn get_path(&self) -> &Vec<Self::Node> {
        &self.path
    }

    fn get_total_distance(&self) -> Self::Distance {
        self.distance
    }
}
