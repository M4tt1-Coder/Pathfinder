use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::graphs::graph::GraphNode;

// ----- Enumeration over all implemented algorithms -----

/// Enumeration over all algorithms.
///
/// Used to specify which algorithm is used by the user.
#[derive(Debug)]
pub enum Algorithms {
    Dijkstra,
    AStar,
}

impl Algorithms {
    /// Converts a string to an 'Algorithms' enum value.
    ///
    /// # Arguments
    ///
    /// - 'src' -> The string that is used to determine the algorithm.
    ///
    /// # Returns
    ///
    /// => Some(Algorithms) if the 'src' string matches a required key string for an algorithm.
    pub fn get_from_string(src: &str) -> Self {
        match src {
            "Dijkstra" => Self::Dijkstra,
            "A*" => Self::AStar,
            _ => Self::Dijkstra,
        }
    }
}

/// Abstract trait defining general behaviour of an path finding algorithm working with graphs.
///
/// Implementing algorithms should support directed and undirected graphs.
pub trait Algorithm {
    /// Error type to directly describe when an error occured during the process.
    ///
    /// Needs to implement basic behaviour of a an Rust error.
    type ExecutionError: Error + Display + Debug;

    /// The end result containing the determined path from the node A to node B.
    ///
    /// Needs to have a 'distance' in case the used graph is weighted and
    /// always has a path, which is a list of nodes in the order of the nodes to go from the starting node to the destination.
    type AlgorithmSearchResult: SearchResult;

    /// Represents the used node in the graph implementing the *Graph* trait.
    ///
    /// Is a *GraphNode* trait implementation.
    type NodeOfUsedGraph: GraphNode;

    /// Method to find the shortest path between two nodes.
    ///
    /// # Arguments
    ///
    /// -> 'start' -> The node where to start the algorithm.
    /// -> 'end' -> The destination node we try to reach.
    ///
    /// # Returns
    ///
    /// => The 'SearchResult' of the execution.
    fn shortest_path(
        &self,
        start: &Self::NodeOfUsedGraph,
        end: &Self::NodeOfUsedGraph,
    ) -> Result<Self::AlgorithmSearchResult, Self::ExecutionError>;
}

// ----- Implementation of the 'SearchResult' trait

/// **Trait**
///
/// A trait that represents the behaviour that of a returned object from an algorithm that has
/// finished running.
///
/// It specifies that they have implemented a distance type that also implements all mandatory
/// traits for mathimatical operations. (Ord, Eq, ...).
pub trait SearchResult: Display + Debug {
    /// The total distance stored in a data type like u8, i16 and f64.
    ///
    /// Sum of all edges from A to B.
    type Distance: PartialEq + PartialOrd + Display;

    /// Can be any struct or data type which impersonates the required functions etc from the
    /// **GraphNode** trait
    ///
    /// Should be any NODE.
    type Node: GraphNode;

    /// **Method**
    ///
    /// Returns the total distance from one node X to Y.
    ///
    /// # Arguments
    ///
    /// - '&self' -> Instance of a struct implementing the *SearchResult* trait.
    ///
    /// # Returns
    ///
    /// => Total distance (u16, ...)
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::algorithm::SearchResult;
    /// use shortest_path_finder::algorithms::dijkstra::DijkstraSearchResult;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let path = vec![
    ///     DefaultNode::new("A".to_string()),
    ///     DefaultNode::new("B".to_string()),
    /// ];
    /// let artificial_search_result = DijkstraSearchResult::new(path, 3u16).unwrap();
    ///
    /// assert_eq!(artificial_search_result.get_total_distance(), 3u16);
    /// ```
    fn get_total_distance(&self) -> Self::Distance;

    /// **Method**
    ///
    /// Provides the list of all nodes in the path which the individual algorithm visited to get
    /// from A to B.
    ///
    /// # Arguments
    ///
    /// - '&self' -> Instance of a struct implementing the *SearchResult* trait.
    ///
    /// # Returns
    ///
    /// => Vector of nodes implementing the *GraphNode* trait. (Self::Node type)
    fn get_path(&self) -> &Vec<Self::Node>;
}
