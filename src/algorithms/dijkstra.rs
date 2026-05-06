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
//! use shortest_path_finder::graphs::directed::DirectedGraph;
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
//! assert!(graph.insert_edge(&a, &b, Some(4)).is_none());
//! assert!(graph.insert_edge(&b, &c, Some(2)).is_none());
//! assert!(graph.insert_edge(&a, &c, Some(10)).is_none());
//!
//! let dijkstra = DijkstraAlgorithm::new(graph);
//! let result = dijkstra.shortest_path("A", "C").unwrap();
//!
//! assert_eq!(result.get_total_distance(), 6);
//! assert_eq!(result.get_path().len(), 3);
//! ```

use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Debug, Display},
};

pub use crate::error::algorithm_error::DijkstraError;

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
    /// # Notes
    ///
    /// This helper is used internally while building the distance map for
    /// [`DijkstraAlgorithm`]. External callers should rely on
    /// [`DijkstraAlgorithm::shortest_path`] instead of constructing
    /// `ShortestDistance` entries directly.
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
/// use shortest_path_finder::graphs::directed::DirectedGraph;
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
/// assert!(graph.insert_edge(&a, &b, Some(1)).is_none());
/// assert!(graph.insert_edge(&b, &c, Some(1)).is_none());
/// assert!(graph.insert_edge(&a, &c, Some(5)).is_none());
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
            return Err(DijkstraError::UnweightedGraph);
        }

        // check if the two 'Node's are in the graph <G> and get them as 'Node' objects
        let graph_label = self.graph.to_string();
        let start: &N = self.graph.get_node_by_id(start_node_id).ok_or_else(|| {
            DijkstraError::MissingStartNode {
                id: start_node_id.to_string(),
                graph: graph_label.clone(),
            }
        })?;

        let end: &N = self.graph.get_node_by_id(end_node_id).ok_or_else(|| {
            DijkstraError::MissingEndNode {
                id: end_node_id.to_string(),
                graph: graph_label,
            }
        })?;

        let distances = self.calculate_distances(start)?;

        // Reconstruct the shortest route by walking predecessors from end to start.
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
                    return Err(DijkstraError::NoPathFound {
                        start: start_node_id.to_string(),
                        end: end_node_id.to_string(),
                    });
                }
            };
            if start.get_id() == prev.get_id() {
                // The start node references itself as predecessor sentinel.
                path.push(start.clone());
                break;
            }
            current_node = prev.clone();
        }

        // check if a path really has been found
        if path.last() != Some(start) {
            return Err(DijkstraError::NoPathFound {
                start: start_node_id.to_string(),
                end: end_node_id.to_string(),
            });
        }

        // Path is collected from end to start; reverse to return start -> end.
        path.reverse();

        let result = DijkstraSearchResult::new(path, output_distance)?;
        Ok(result)
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
    /// let graph = DirectedGraph::new(vec![]);
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
                        return Err(DijkstraError::MissingNodeDuringProcessing {
                            id: position.get_id().to_string(),
                        });
                    }
                }
            {
                continue;
            }

            for (neighbour, weight) in self.graph.neighbors(&position) {
                // for Dijkstra an edges weight can't be smaller then 0
                if weight < W::zero() {
                    return Err(DijkstraError::InvalidEdgeWeight {
                        from: position.get_id().to_string(),
                        to: neighbour.get_id().to_string(),
                        weight: format!("{}", weight),
                        reason: "negative".to_string(),
                    });
                }

                // Standard relaxation: candidate distance via the current node.
                let updated_distance = distance + weight;

                if updated_distance
                    < match distances.get(neighbour.get_id()) {
                        Some(distance_data) => distance_data.distance,
                        None => {
                            return Err(DijkstraError::MissingNodeDuringProcessing {
                                id: neighbour.get_id().to_string(),
                            });
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
    /// - `Err(DijkstraError)` with a detailed reason otherwise.
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
    pub fn new(path: Vec<N>, distance: W) -> Result<Self, DijkstraError> {
        if path.len() < 2 {
            return Err(DijkstraError::InvalidSearchResult {
                reason: "path must contain at least two nodes".to_string(),
            });
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
