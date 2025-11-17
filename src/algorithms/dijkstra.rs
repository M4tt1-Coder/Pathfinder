use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fmt::Display,
};

use crate::{
    algorithms::algorithm::{Algorithm, SearchResult},
    graphs::graph::{Graph, Node},
};

// ----- Implementation of the 'ShortestDistance' struct -----

/// Represents the result of a shortest distance from 'Node' A to B.
///
/// # Fields
///
/// - 'distance' -> Minimum distance to a specific 'Node'.
/// - 'previous_node' -> The last 'Node' that was visited before reaching the specific 'Node'.
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct ShortestDistance {
    distance: u16,
    previous_node: Option<Node>,
}

impl ShortestDistance {
    /// Create a fresh object of the 'ShortestDistance' struct.
    ///
    /// # Arguments
    ///
    /// -> 'previous_node' -> Initial value for the previous node of a 'Node'.
    ///
    /// # Returns
    ///
    /// => New 'ShortestDistance' object
    fn new(previous_node: Option<Node>) -> Self {
        Self {
            previous_node,
            distance: u16::MAX,
        }
    }
}

// ----- Implementation of the 'DijkstraAlgorithm' struct -----

/// Implements the "Dijkstra" for weighted graphs.
///
/// The graphs need to have weighted edges!
#[derive(Debug)]
pub struct DijkstraAlgorithm<G: Graph + Display> {
    /// Can be every (type) implementation of the 'Graph' trait.
    graph: G,
}

impl<G: Graph + Display> Algorithm for DijkstraAlgorithm<G> {
    type StepExecutionResult = ShortestDistance;
    type ExecutionError = DijkstraError;
    fn shortest_path(&self, start: Node, end: Node) -> Result<SearchResult, DijkstraError> {
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
        if self.graph.get_node_by_id(&start.id).is_none() {
            return Err(DijkstraError::new(format!(
                "The node {} is not in the graph {}!",
                start, self.graph
            )));
        }

        if self.graph.get_node_by_id(&end.id).is_none() {
            return Err(DijkstraError::new(format!(
                "The node {} is not in the graph {}!",
                end, self.graph
            )));
        }

        let distances = self.calculate_distances(&start)?;

        // search for the shortest route from the 'start' to the 'end' node
        let mut path: Vec<Node> = vec![];
        let mut current_node = end.clone();
        let mut output_distance = 0;

        while let Some(distance) = distances.get(&current_node.id) {
            if current_node.id == end.id {
                output_distance = distance.distance;
            }
            path.push(current_node);
            let prev = match &distance.previous_node {
                Some(node) => node,
                None => {
                    return Err(DijkstraError::new(format!(
                        "Unable to determine a valid path from {} to {}!",
                        start, end
                    )));
                }
            };
            if start.id == prev.id {
                path.push(start.clone());
                break;
            }
            current_node = prev.clone();
        }

        // check if a path really has been found
        if path.last() != Some(&start) {
            return Err(DijkstraError::new("A path could not be found!".to_string()));
        }

        path.reverse();

        Ok(match SearchResult::new(path, output_distance) {
            Ok(result) => result,
            Err(err) => return Err(DijkstraError::new(err)),
        })
    }
    fn execute_step() -> Option<Self::StepExecutionResult> {
        None
    }
}

impl<G: Graph + Display> DijkstraAlgorithm<G> {
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
    fn setup_shortest_distance(&self, start: &Node) -> HashMap<String, ShortestDistance> {
        let mut output: HashMap<String, ShortestDistance> = HashMap::new();
        for n in self.graph.get_all_nodes() {
            if n.id == start.id {
                output.insert(
                    n.id.clone(),
                    ShortestDistance {
                        distance: 0,
                        previous_node: Some(n.clone()),
                    },
                );
            } else {
                output.insert(n.id.clone(), ShortestDistance::new(None));
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
        start: &Node,
    ) -> Result<HashMap<String, ShortestDistance>, DijkstraError> {
        // - new list keeping track of the shortest distance from the start node to all others
        let mut distances: HashMap<String, ShortestDistance> = self.setup_shortest_distance(start);

        // queue for leftover steps to check if they lead on the shortest path to a node
        let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();

        queue.push(QueueItem {
            distance: 0,
            position: start.clone(),
        });

        while let Some(QueueItem { distance, position }) = queue.pop() {
            if distance
                > match distances.get(&position.id) {
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

            for (neighbour, weight) in self.graph.neighbours_as_standard_output(&position) {
                let updated_distance = distance + weight;

                if updated_distance
                    < match distances.get(&neighbour.id) {
                        Some(distance_data) => distance_data.distance,
                        None => {
                            return Err(DijkstraError::new(format!(
                                "Couldn't find the node {} in the graph! Please check if the original input data is valid!",
                                neighbour
                            )));
                        }
                    }
                {
                    distances.entry(neighbour.id.clone()).and_modify(|entry| {
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
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct QueueItem {
    /// Temporary distance during the process.
    ///
    /// Represents a potential shortest distance to a 'Node'.
    distance: u16,
    /// The 'Node' we are at with this item.
    position: Node,
}

impl QueueItem {
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
    fn new(distance: u16, position: Node) -> Self {
        Self { distance, position }
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
