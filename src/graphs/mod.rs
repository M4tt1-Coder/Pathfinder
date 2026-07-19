//! Graph abstractions and concrete graph data structures.
//!
//! # Overview
//!
//! The graph layer is organized around a small shared trait surface and a set
//! of concrete graph types that model the supported input formats:
//!
//! - [`graph`]: shared traits used by algorithms and integration tests.
//! - [`directed`]: a weighted directed graph over [`crate::nodes::default_node::DefaultNode`].
//! - [`undirected`]: a weighted undirected graph over [`crate::nodes::default_node::DefaultNode`].
//! - [`two_dimensional_coordinate_graph`]: a coordinate-based weighted graph
//!   built on [`crate::nodes::two_dimensional_node::TwoDimensionalNode`].
//!
//! The public [`GraphInsertionError`] wrapper lets callers handle insertion
//! failures without committing to a concrete graph implementation. It is
//! especially useful when graph parsing selects an implementation at runtime.
//!
//! # Common use cases
//!
//! - Build a graph directly in memory for algorithm tests.
//! - Inspect whether a graph is directed or weighted before running a path
//!   search.
//! - Match on [`GraphInsertionError`] when a graph mutation fails.
//!
//! # Examples
//!
//! Create a directed graph and query its capabilities:
//!
//! ```rust
//! use shortest_path_finder::graphs::directed::DirectedGraph;
//! use shortest_path_finder::graphs::graph::Graph;
//!
//! let graph = DirectedGraph::new(vec![]);
//! assert!(graph.is_directed());
//! assert!(graph.is_weighted());
//! ```
//!
//! Handle a graph insertion failure in a concrete, type-safe way:
//!
//! ```rust
//! use shortest_path_finder::graphs::{directed::DirectedGraphInsertionError, GraphInsertionError};
//!
//! let error = GraphInsertionError::Directed(
//!     DirectedGraphInsertionError::SourceNodeDoesNotExist {
//!         node_id: "A".to_string(),
//!     },
//! );
//!
//! assert!(matches!(
//!     error,
//!     GraphInsertionError::Directed(DirectedGraphInsertionError::SourceNodeDoesNotExist { .. })
//! ));
//! ```

pub mod directed;
pub mod graph;
pub mod two_dimensional_coordinate_graph;
pub mod undirected;

mod utils;

/// High-level error wrapper for graph insertion failures.
///
/// This type erases the concrete graph implementation while preserving the
/// exact insertion failure that occurred. Use it when a parsing or graph
/// construction boundary can select between multiple graph types at runtime,
/// but the caller still needs to inspect the specific reason the insertion
/// failed.
///
/// # Why this wrapper exists
///
/// Each concrete graph has its own insertion error enum, but callers at higher
/// layers often only care that graph mutation failed. The wrapper keeps the API
/// ergonomic without losing the original error details.
///
/// # How to use it
///
/// - Pattern-match on a variant when you need graph-specific recovery logic.
/// - Call [`std::string::ToString::to_string`] or use the [`std::fmt::Display`]
///   implementation for user-facing diagnostics.
/// - Convert it into higher-level errors such as [`crate::error::parse_error::ParseError`].
///
/// # Variants
///
/// - [`GraphInsertionError::Directed`]: errors from [`directed::DirectedGraph`].
/// - [`GraphInsertionError::Undirected`]: errors from [`undirected::UndirectedGraph`].
/// - [`GraphInsertionError::TwoDimensional`]: errors from
///   [`two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph`].
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::graphs::{directed::DirectedGraphInsertionError, GraphInsertionError};
///
/// let error = GraphInsertionError::Directed(
///     DirectedGraphInsertionError::MissingEdgeWeight {
///         expected_weight: "u16".to_string(),
///     },
/// );
///
/// assert!(error.to_string().contains("Missing edge weight"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphInsertionError {
    /// Error produced by the directed graph implementation.
    Directed(directed::DirectedGraphInsertionError),
    /// Error produced by the undirected graph implementation.
    Undirected(undirected::UndirectedGraphInsertionError),
    /// Error produced by the two-dimensional coordinate graph implementation.
    TwoDimensional(two_dimensional_coordinate_graph::TwoDimensionalGraphInsertionError),
}

impl std::fmt::Display for GraphInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphInsertionError::Directed(err) => {
                write!(f, "Directed graph insertion error: {}", err)
            }
            GraphInsertionError::Undirected(err) => {
                write!(f, "Undirected graph insertion error: {}", err)
            }
            GraphInsertionError::TwoDimensional(err) => {
                write!(f, "Two-dimensional graph insertion error: {}", err)
            }
        }
    }
}

impl std::error::Error for GraphInsertionError {}
