use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fmt::{Debug, Display},
};

use crate::{
    algorithms::algorithm::{Algorithm, SearchResult},
    graphs::graph::{Graph, GraphNode, GraphWeight},
};

// ----- Implementation of the 'ShortestDistance' struct -----

/// Represents the result of a shortest distance from 'Node' A to B.
///
/// # Fields
///
/// - 'distance' -> Minimum distance to a specific 'Node'.
/// - 'previous_node' -> The last 'Node' that was visited before reaching the specific 'Node'.
#[derive(Debug)]
pub struct ShortestDistance<N: GraphNode, W: GraphWeight + Ord> {
    distance: W,
    previous_node: Option<N>,
}

impl<N: GraphNode, W: GraphWeight + Ord> ShortestDistance<N, W> {
    /// Creates a new instance of `YourStruct` with the specified previous node and distance.
    ///
    /// # Parameters
    /// - `previous_node`: An optional reference to the previous node in the path.
    ///   Use `None` if there is no predecessor (e.g., for the start node).
    /// - `distance`: The accumulated distance or weight associated with this node.
    ///
    /// # Returns
    /// A new instance of `YourStruct` initialized with the provided `previous_node` and `distance`.
    ///
    /// # Example
    /// ```
    /// let start_node = None;
    /// let initial_distance = 0.0;
    /// let node = YourStruct::new(start_node, initial_distance);
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

// ----- Implementation of the 'DijkstraAlgorithm' struct -----

/// Implements the "Dijkstra" for weighted graphs.
///
/// The graphs need to have weighted edges!
#[derive(Debug)]
pub struct DijkstraAlgorithm<
    N: GraphNode,
    W: GraphWeight + Ord,
    G: Graph<Node = N, Weight = W> + Display,
> {
    /// Can be every (type) implementation of the 'Graph' trait.
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
        start: &N,
        end: &N,
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

        // check if the two 'Node's are in the graph <G>
        if self.graph.get_node_by_id(start.get_id()).is_none() {
            return Err(DijkstraError::new(format!(
                "The node {} is not in the graph {}!",
                start, self.graph
            )));
        }

        if self.graph.get_node_by_id(end.get_id()).is_none() {
            return Err(DijkstraError::new(format!(
                "The node {} is not in the graph {}!",
                end, self.graph
            )));
        }

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
                        start, end
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
    /// Creates a new instance of the 'DijkstraAlgorithm' struct.
    ///
    /// # Arguments
    ///
    /// - 'graph' -> Is a graph object implementing the 'Graph' trait.
    ///
    /// # Returns
    ///
    /// => 'DijkstraAlgorithm' instance.
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Prepares the "shortest distance" from one node A to every other node B.
    ///
    /// # Arguments
    ///
    /// - 'start' -> The start node of the algorithm.
    ///
    /// # Returns
    ///
    /// A hashmap of 'ShortestDistance's for every node in the graph.
    fn setup_shortest_distance(&self, start: &N) -> HashMap<String, ShortestDistance<N, W>> {
        let mut output: HashMap<String, ShortestDistance<N, W>> = HashMap::new();
        for n in self.graph.get_all_nodes() {
            if n.get_id() == start.get_id() {
                output.insert(
                    n.get_id().to_string().clone(),
                    ShortestDistance {
                        distance: W::zero(),
                        previous_node: Some(n.clone()),
                    },
                );
            } else {
                output.insert(
                    n.get_id().to_string().clone(),
                    ShortestDistance::new(None, W::max_value()),
                );
            }
        }
        output
    }

    /// Executes the whole core 'Dijkstra' algorithm on the provide data graph '<G>'.
    ///
    /// # Arguments
    ///
    /// - 'start' -> The 'Node' which we start the path from.
    ///
    /// # Returns
    ///
    /// => 'HashMap<String, ShortestDistance>' with all shortest distance from the 'start' Node.
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
                    distances
                        .entry(neighbour.get_id().to_string().clone())
                        .and_modify(|entry| {
                            entry.distance = updated_distance;
                            entry.previous_node = Some(position.clone())
                        });
                    queue.push(QueueItem::new(updated_distance, neighbour.clone()));
                }
            }
        }
        Ok(distances)
    }
}

// ----- Implementation of the 'QueueItem' struct -----

/// Temporary item in the step queue.
#[derive(Eq, PartialEq)]
struct QueueItem<N: GraphNode, W: GraphWeight> {
    /// Temporary distance during the process.
    ///
    /// Represents a potential shortest distance to a 'Node'.
    distance: W,
    /// The 'Node' we are at with this item.
    position: N,
}

impl<N: GraphNode, W: GraphWeight + Ord + Eq> QueueItem<N, W> {
    /// Creates a new instance of the 'QueueItem' struct.
    ///
    /// # Arguments
    ///
    /// - 'distance' -> The distance to a 'Node'.
    /// - 'position' -> The 'Node' we are checking in the next validation step.
    ///
    /// # Returns
    ///
    /// => 'QueueItem' object.
    fn new(distance: W, position: N) -> Self {
        Self { distance, position }
    }
}

impl<N: GraphNode, W: GraphWeight + Ord + Eq> PartialOrd for QueueItem<N, W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: GraphNode, W: GraphWeight + Ord + Eq> Ord for QueueItem<N, W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

// ----- Implementation of the 'DijkstraError' struct -----

/// Specific error for the *DijkstraAlgorithm*.
///
/// # Fields
///
/// - 'message' -> Description of the occured issue during the process.
#[derive(Debug)]
pub struct DijkstraError {
    pub message: String,
}

impl DijkstraError {
    /// Creates a new 'DijkstraError' instance.
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

// ----- Implementation of the 'DijkstraSearchResult' struct -----

/// Search result of all algorithms which implement the 'Algorithm' trait.
///
/// # Fields
///
/// - 'path' -> All nodes we need to go through to reach the destination.
/// - 'distance' -> Sum of all edges.
#[derive(Debug, Clone)]
pub struct DijkstraSearchResult<N: GraphNode, W: GraphWeight> {
    /// List of the nodes starting from the start to the final node.
    ///
    /// Must have atleast 2 elements.
    pub path: Vec<N>,

    /// All weighted edges combined and added together.
    pub distance: W,
}

impl<N: GraphNode, W: GraphWeight> DijkstraSearchResult<N, W> {
    /// Create a new 'SearchResult' instance.
    ///
    /// # FAILS
    ///
    /// ... if there are less then 2 nodes in the 'path' vector.
    ///
    /// # Returns
    ///
    /// => Ok(SearchResult), if a valid result has been created.
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
