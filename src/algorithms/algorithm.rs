use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::graphs::graph::Node;

// ----- Enumeration over all implemented algorithms -----

/// Enumeration over all algorithms.
///
/// Used to specify which algorithm is used by the user.
#[derive(Debug)]
pub enum Algorithms {
    Dijkstra,
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
            _ => Self::Dijkstra,
        }
    }
}

/// Abstract trait defining general behaviour of an path finding algorithm working with graphs.
///
/// Implementing algorithms should support directed and undirected graphs.
pub trait Algorithm {
    /// In each algorithm single steps will need to be executed.
    ///
    /// Represents the temporary result of the step execution.
    type StepExecutionResult: PartialEq;

    /// Error type to directly describe when an error occured during the process.
    ///
    /// Needs to implement basic behaviour of a an Rust error.
    type ExecutionError: Error + Display + Debug;

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
    fn shortest_path(&self, start: Node, end: Node) -> Result<SearchResult, Self::ExecutionError>;

    /// Executes a step in the path finding algorithm.
    ///
    /// # Returns
    ///
    /// => An individual 'Option<StepExecutionResult>'.
    fn execute_step() -> Option<Self::StepExecutionResult>;
}

// ----- Implementation of the 'SearchResult' struct -----

/// Search result of all algorithms which implement the 'Algorithm' trait.
///
/// # Fields
///
/// - 'path' -> All nodes we need to go through to reach the destination.
/// - 'distance' -> Sum of all edges.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// List of the nodes starting from the start to the final node.
    ///
    /// Must have atleast 2 elements.
    pub path: Vec<Node>,
    /// All weighted edges combined and added together.
    pub distance: u16,
}

impl SearchResult {
    /// Create a new 'SearchResult' instance.
    ///
    /// # FAILS
    ///
    /// ... if there are less then 2 nodes in the 'path' vector.
    ///
    /// # Returns
    ///
    /// => Ok(SearchResult), if a valid result has been created.
    pub fn new(path: Vec<Node>, distance: u16) -> Result<Self, String> {
        if path.len() < 2 {
            return Err("There need to be at least 2 nodes in the path from one node A to another node B! Couldn't create a 'SearchResult'!".to_string());
        }

        Ok(Self { path, distance })
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path_string = String::new();
        for n in &self.path {
            path_string = format!("{} -> {}", path_string, n.id);
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
