//! Core algorithm abstractions used by this crate.
//!
//! This module provides three key building blocks:
//! - [`Algorithms`]: a user-facing selector for supported shortest-path algorithms.
//! - [`Algorithm`]: a trait that algorithm engines (for example Dijkstra or A*) implement.
//! - [`SearchResult`]: a trait describing the result object returned by an algorithm run.
//!
//! The traits in this module are intentionally generic so they can be reused for different
//! graph implementations, node types, and numeric weight/distance types.
//!
//! # Typical usage
//!
//! Convert CLI/user text input to an algorithm selection:
//!
//! ```rust
//! use shortest_path_finder::algorithms::algorithm::Algorithms;
//!
//! let algorithm = Algorithms::get_from_string("Dijkstra");
//! assert!(matches!(algorithm, Algorithms::Dijkstra));
//! ```
//!
//! Consume a search result produced by a concrete algorithm implementation:
//!
//! ```rust
//! use shortest_path_finder::algorithms::algorithm::SearchResult;
//! use shortest_path_finder::algorithms::dijkstra::DijkstraSearchResult;
//! use shortest_path_finder::nodes::default_node::DefaultNode;
//!
//! let path = vec![
//!     DefaultNode::new("A".to_string()),
//!     DefaultNode::new("B".to_string()),
//! ];
//! let result = DijkstraSearchResult::new(path, 7u16).unwrap();
//!
//! assert_eq!(result.get_total_distance(), 7u16);
//! assert_eq!(result.get_path().len(), 2);
//! ```

use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::graphs::graph::GraphNode;

/// Enumeration over all algorithms currently exposed by the application layer.
///
/// This enum is primarily used by CLI/config parsing to select which concrete
/// algorithm implementation should be executed at runtime.
#[derive(Debug)]
pub enum Algorithms {
    /// Select the Dijkstra shortest-path algorithm.
    Dijkstra,
    /// Select the A* shortest-path algorithm.
    AStar,
}

impl Algorithms {
    /// Converts a user-provided string into an [`Algorithms`] value.
    ///
    /// # Parameters
    ///
    /// - `src`: The input token used to determine the algorithm.
    ///
    /// Recognized values are currently:
    /// - `"Dijkstra"`
    /// - `"AStar"`
    ///
    /// Any unknown value falls back to [`Algorithms::Dijkstra`].
    ///
    /// # Returns
    ///
    /// A concrete [`Algorithms`] variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::algorithm::Algorithms;
    ///
    /// assert!(matches!(Algorithms::get_from_string("Dijkstra"), Algorithms::Dijkstra));
    /// assert!(matches!(Algorithms::get_from_string("AStar"), Algorithms::AStar));
    ///
    /// // Unknown input currently defaults to Dijkstra.
    /// assert!(matches!(Algorithms::get_from_string("unknown"), Algorithms::Dijkstra));
    /// ```
    pub fn get_from_string(src: &str) -> Self {
        match src {
            "Dijkstra" => Self::Dijkstra,
            "AStar" => Self::AStar,
            _ => Self::Dijkstra,
        }
    }
}

/// Common behavior for shortest-path algorithms operating on graph data.
///
/// Implementors encapsulate algorithm-specific logic and expose a consistent
/// interface via [`Algorithm::shortest_path`].
pub trait Algorithm {
    /// Error type returned when execution fails.
    ///
    /// Implementations should use an error type that provides both human-readable
    /// context and standard Rust error semantics.
    type ExecutionError: Error + Display + Debug;

    /// Concrete search result type produced by this algorithm.
    ///
    /// The result must implement [`SearchResult`] so callers can inspect both
    /// the total distance/cost and the ordered path.
    type AlgorithmSearchResult: SearchResult;

    /// Node type of the graph consumed by this algorithm.
    ///
    /// Must satisfy [`GraphNode`] so algorithm implementations can work with
    /// node identifiers and cloning/formatting requirements defined by the graph model.
    type NodeOfUsedGraph: GraphNode;

    /// Computes the shortest path between two node identifiers.
    ///
    /// # Parameters
    ///
    /// - `start_node_id`: Identifier of the start node.
    /// - `end_node_id`: Identifier of the destination node.
    ///
    /// # Returns
    ///
    /// - `Ok(Self::AlgorithmSearchResult)` if a valid path was computed.
    /// - `Err(Self::ExecutionError)` if execution fails, for example because input
    ///   data is invalid or no path can be determined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
    /// use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    /// use shortest_path_finder::graphs::graph::Graph;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let mut graph = DirectedGraph::<DefaultNode, u16>::new();
    /// let a = DefaultNode::new("A".to_string());
    /// let b = DefaultNode::new("B".to_string());
    /// graph.add_edge(a.clone(), b.clone(), Some(3u16));
    ///
    /// let algorithm = DijkstraAlgorithm::new(graph);
    /// let result = algorithm.shortest_path(&a, &b).unwrap();
    ///
    /// assert_eq!(result.get_total_distance(), 3u16);
    /// assert_eq!(result.get_path().len(), 2);
    /// ```
    fn shortest_path(
        &self,
        start_node_id: &str,
        end_node_id: &str,
    ) -> Result<Self::AlgorithmSearchResult, Self::ExecutionError>;
}

/// Common interface for algorithm output objects.
///
/// A search result stores two pieces of information:
/// - The total path distance/cost.
/// - The ordered list of nodes representing the path from start to destination.
///
/// Consumers should depend on this trait when they only need read access to
/// path information and do not care about a specific algorithm implementation.
pub trait SearchResult: Display + Debug {
    /// Numeric type representing the total path distance/cost.
    ///
    /// Implementations may use unsigned integers, signed integers, or floating
    /// point values depending on graph constraints.
    type Distance: PartialEq + PartialOrd + Display;

    /// Node type used in the returned path.
    ///
    /// Must satisfy [`GraphNode`] so callers can inspect identifiers and display
    /// node values in a uniform way.
    type Node: GraphNode;

    /// Returns the total distance/cost from start node to destination node.
    ///
    /// # Returns
    ///
    /// The full distance/cost value of the computed path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::algorithm::SearchResult;
    /// use shortest_path_finder::algorithms::dijkstra::DijkstraSearchResult;
    /// use shortest_path_finder::graphs::graph::GraphNode;
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

    /// Returns the ordered path from start node to destination node.
    ///
    /// The vector is expected to include both start and end nodes when a path
    /// exists.
    ///
    /// # Returns
    ///
    /// A borrowed vector containing path nodes in traversal order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shortest_path_finder::algorithms::algorithm::SearchResult;
    /// use shortest_path_finder::algorithms::dijkstra::DijkstraSearchResult;
    /// use shortest_path_finder::nodes::default_node::DefaultNode;
    ///
    /// let path = vec![
    ///     DefaultNode::new("A".to_string()),
    ///     DefaultNode::new("B".to_string()),
    ///     DefaultNode::new("C".to_string()),
    /// ];
    /// let result = DijkstraSearchResult::new(path, 10u16).unwrap();
    ///
    /// assert_eq!(result.get_path().first().unwrap().get_id(), "A");
    /// assert_eq!(result.get_path().last().unwrap().get_id(), "C");
    /// assert_eq!(result.get_path().len(), 3);
    /// ```
    fn get_path(&self) -> &Vec<Self::Node>;
}
