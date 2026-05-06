//! Algorithm execution error types and CLI-facing error helpers.
//!
//! # Overview
//!
//! This module centralizes error types produced by shortest-path algorithms and
//! provides a wrapper for CLI consumers to classify errors and map them to
//! exit codes.
//!
//! # Error Taxonomy
//!
//! - [`AlgorithmErrorKind`] groups error categories used by the CLI.
//! - [`AlgorithmError`] wraps algorithm-specific error payloads.
//! - [`AStarExecutionError`] and [`DijkstraError`] describe concrete failures.
//! - [`PathReconstructionError`] captures failures while rebuilding paths.
//!
//! # Exit Codes
//!
//! `AlgorithmErrorKind::exit_code()` maps error categories to stable, numeric
//! exit codes for the CLI runtime.
//!
//! # Examples
//!
//! ```rust
//! use shortest_path_finder::algorithms::dijkstra::DijkstraError;
//! use shortest_path_finder::error::algorithm_error::{AlgorithmError, AlgorithmErrorKind};
//!
//! let err = AlgorithmError::from(DijkstraError::NoPathFound {
//!     start: "A".to_string(),
//!     end: "B".to_string(),
//! });
//! assert_eq!(err.kind(), AlgorithmErrorKind::NoPath);
//! assert_eq!(err.kind().exit_code(), 6);
//! ```

use std::{error::Error, fmt};

/// High-level categories for algorithm execution failures.
///
/// # Exit Codes
///
/// - `InvalidGraph` => `2`
/// - `MissingNode` => `3`
/// - `InvalidWeight` => `4`
/// - `InvalidHeuristic` => `5`
/// - `NoPath` => `6`
/// - `InvariantViolation` => `7`
/// - `InvalidResult` => `8`
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::AlgorithmErrorKind;
///
/// assert_eq!(AlgorithmErrorKind::NoPath.exit_code(), 6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmErrorKind {
    /// Graph configuration or constraints are invalid for the algorithm.
    InvalidGraph,
    /// Required nodes are missing from the graph.
    MissingNode,
    /// Edge weights violate algorithm constraints.
    InvalidWeight,
    /// Heuristic values are invalid or non-finite.
    InvalidHeuristic,
    /// No path exists between the requested nodes.
    NoPath,
    /// Internal invariant or bookkeeping was violated.
    InvariantViolation,
    /// Algorithm produced a result that failed validation.
    InvalidResult,
}

impl AlgorithmErrorKind {
    /// Returns a stable CLI exit code for this error category.
    pub fn exit_code(self) -> i32 {
        match self {
            AlgorithmErrorKind::InvalidGraph => 2,
            AlgorithmErrorKind::MissingNode => 3,
            AlgorithmErrorKind::InvalidWeight => 4,
            AlgorithmErrorKind::InvalidHeuristic => 5,
            AlgorithmErrorKind::NoPath => 6,
            AlgorithmErrorKind::InvariantViolation => 7,
            AlgorithmErrorKind::InvalidResult => 8,
        }
    }
}

/// Wrapper for algorithm-specific execution errors.
///
/// # Usage
///
/// ```rust
/// use shortest_path_finder::algorithms::dijkstra::DijkstraError;
/// use shortest_path_finder::error::algorithm_error::{AlgorithmError, AlgorithmErrorKind};
///
/// let err = AlgorithmError::from(DijkstraError::NoPathFound {
///     start: "A".to_string(),
///     end: "B".to_string(),
/// });
/// assert_eq!(err.kind(), AlgorithmErrorKind::NoPath);
/// ```
#[derive(Debug)]
pub enum AlgorithmError {
    /// A* algorithm execution error.
    AStar(AStarExecutionError),
    /// Dijkstra algorithm execution error.
    Dijkstra(DijkstraError),
}

impl AlgorithmError {
    /// Returns the categorized error kind.
    pub fn kind(&self) -> AlgorithmErrorKind {
        match self {
            AlgorithmError::AStar(err) => err.kind(),
            AlgorithmError::Dijkstra(err) => err.kind(),
        }
    }
}

impl fmt::Display for AlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgorithmError::AStar(err) => write!(f, "{}", err),
            AlgorithmError::Dijkstra(err) => write!(f, "{}", err),
        }
    }
}

impl Error for AlgorithmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AlgorithmError::AStar(err) => Some(err),
            AlgorithmError::Dijkstra(err) => Some(err),
        }
    }
}

impl From<AStarExecutionError> for AlgorithmError {
    fn from(err: AStarExecutionError) -> Self {
        Self::AStar(err)
    }
}

impl From<DijkstraError> for AlgorithmError {
    fn from(err: DijkstraError) -> Self {
        Self::Dijkstra(err)
    }
}

/// Error returned while reconstructing a path from A* bookkeeping data.
///
/// # Context
///
/// These errors indicate inconsistent predecessor links in the closed set.
/// They are wrapped by [`AStarExecutionError::PathReconstruction`].
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::PathReconstructionError;
///
/// let err = PathReconstructionError::MissingClosedEntry {
///     node_id: "X".to_string(),
/// };
/// assert!(err.to_string().contains("missing"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathReconstructionError {
    /// The closed set was empty when reconstruction started.
    EmptyClosedSet,
    /// A predecessor node could not be found in the closed set.
    MissingClosedEntry { node_id: String },
}

impl fmt::Display for PathReconstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathReconstructionError::EmptyClosedSet => {
                write!(f, "closed set is empty")
            }
            PathReconstructionError::MissingClosedEntry { node_id } => {
                write!(f, "predecessor node '{}' missing from closed set", node_id)
            }
        }
    }
}

impl Error for PathReconstructionError {}

/// A* execution errors.
///
/// # Categories
///
/// Use [`AStarExecutionError::kind`] to map failures to
/// [`AlgorithmErrorKind`] values.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::{AStarExecutionError, AlgorithmErrorKind};
///
/// let err = AStarExecutionError::UnweightedGraph;
/// assert_eq!(err.kind(), AlgorithmErrorKind::InvalidGraph);
/// ```
#[derive(Debug, Clone)]
pub enum AStarExecutionError {
    /// Graph is not weighted.
    UnweightedGraph,
    /// Start node does not exist in the graph.
    MissingStartNode { id: String },
    /// End node does not exist in the graph.
    MissingEndNode { id: String },
    /// Edge weight violates algorithm constraints.
    InvalidEdgeWeight {
        from: String,
        to: String,
        weight: String,
        reason: String,
    },
    /// Heuristic produced a non-finite value.
    InvalidHeuristic {
        start: String,
        goal: String,
        current: String,
        value: f32,
    },
    /// Expected g-cost entry is missing from the bookkeeping map.
    MissingGCost { node_id: String },
    /// No path exists between the start and end nodes.
    NoPathFound { start: String, end: String },
    /// Search result failed validation.
    InvalidSearchResult { reason: String },
    /// Path reconstruction failed using closed-set bookkeeping.
    PathReconstruction { source: PathReconstructionError },
}

impl AStarExecutionError {
    /// Returns the categorized error kind.
    pub fn kind(&self) -> AlgorithmErrorKind {
        match self {
            AStarExecutionError::UnweightedGraph => AlgorithmErrorKind::InvalidGraph,
            AStarExecutionError::MissingStartNode { .. } => AlgorithmErrorKind::MissingNode,
            AStarExecutionError::MissingEndNode { .. } => AlgorithmErrorKind::MissingNode,
            AStarExecutionError::InvalidEdgeWeight { .. } => AlgorithmErrorKind::InvalidWeight,
            AStarExecutionError::InvalidHeuristic { .. } => AlgorithmErrorKind::InvalidHeuristic,
            AStarExecutionError::MissingGCost { .. } => AlgorithmErrorKind::InvariantViolation,
            AStarExecutionError::NoPathFound { .. } => AlgorithmErrorKind::NoPath,
            AStarExecutionError::InvalidSearchResult { .. } => AlgorithmErrorKind::InvalidResult,
            AStarExecutionError::PathReconstruction { .. } => {
                AlgorithmErrorKind::InvariantViolation
            }
        }
    }
}

impl fmt::Display for AStarExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AStarExecutionError::UnweightedGraph => {
                write!(f, "Algorithm error (AStar): graph must be weighted")
            }
            AStarExecutionError::MissingStartNode { id } => {
                write!(f, "Algorithm error (AStar): start node '{}' not found", id)
            }
            AStarExecutionError::MissingEndNode { id } => {
                write!(f, "Algorithm error (AStar): end node '{}' not found", id)
            }
            AStarExecutionError::InvalidEdgeWeight {
                from,
                to,
                weight,
                reason,
            } => write!(
                f,
                "Algorithm error (AStar): invalid edge weight {} on {} -> {} ({})",
                weight, from, to, reason
            ),
            AStarExecutionError::InvalidHeuristic {
                start,
                goal,
                current,
                value,
            } => write!(
                f,
                "Algorithm error (AStar): heuristic produced non-finite value {} for start '{}', goal '{}', current '{}'",
                value, start, goal, current
            ),
            AStarExecutionError::MissingGCost { node_id } => write!(
                f,
                "Algorithm error (AStar): missing g-cost entry for node '{}'",
                node_id
            ),
            AStarExecutionError::NoPathFound { start, end } => write!(
                f,
                "Algorithm error (AStar): no path found from '{}' to '{}'",
                start, end
            ),
            AStarExecutionError::InvalidSearchResult { reason } => write!(
                f,
                "Algorithm error (AStar): invalid search result ({})",
                reason
            ),
            AStarExecutionError::PathReconstruction { source } => write!(
                f,
                "Algorithm error (AStar): path reconstruction failed: {}",
                source
            ),
        }
    }
}

impl Error for AStarExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AStarExecutionError::PathReconstruction { source } => Some(source),
            _ => None,
        }
    }
}

impl From<PathReconstructionError> for AStarExecutionError {
    fn from(source: PathReconstructionError) -> Self {
        AStarExecutionError::PathReconstruction { source }
    }
}

/// Dijkstra execution errors.
///
/// # Categories
///
/// Use [`DijkstraError::kind`] to map failures to [`AlgorithmErrorKind`].
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::{AlgorithmErrorKind, DijkstraError};
///
/// let err = DijkstraError::NoPathFound {
///     start: "A".to_string(),
///     end: "B".to_string(),
/// };
/// assert_eq!(err.kind(), AlgorithmErrorKind::NoPath);
/// ```
#[derive(Debug, Clone)]
pub enum DijkstraError {
    /// Graph is not weighted.
    UnweightedGraph,
    /// Start node does not exist in the graph.
    MissingStartNode { id: String, graph: String },
    /// End node does not exist in the graph.
    MissingEndNode { id: String, graph: String },
    /// A node is missing while computing distances.
    MissingNodeDuringProcessing { id: String },
    /// Edge weight violates algorithm constraints.
    InvalidEdgeWeight {
        from: String,
        to: String,
        weight: String,
        reason: String,
    },
    /// No path exists between the start and end nodes.
    NoPathFound { start: String, end: String },
    /// Search result failed validation.
    InvalidSearchResult { reason: String },
}

impl DijkstraError {
    /// Returns the categorized error kind.
    pub fn kind(&self) -> AlgorithmErrorKind {
        match self {
            DijkstraError::UnweightedGraph => AlgorithmErrorKind::InvalidGraph,
            DijkstraError::MissingStartNode { .. } => AlgorithmErrorKind::MissingNode,
            DijkstraError::MissingEndNode { .. } => AlgorithmErrorKind::MissingNode,
            DijkstraError::MissingNodeDuringProcessing { .. } => {
                AlgorithmErrorKind::InvariantViolation
            }
            DijkstraError::InvalidEdgeWeight { .. } => AlgorithmErrorKind::InvalidWeight,
            DijkstraError::NoPathFound { .. } => AlgorithmErrorKind::NoPath,
            DijkstraError::InvalidSearchResult { .. } => AlgorithmErrorKind::InvalidResult,
        }
    }
}

impl fmt::Display for DijkstraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DijkstraError::UnweightedGraph => {
                write!(f, "Algorithm error (Dijkstra): graph must be weighted")
            }
            DijkstraError::MissingStartNode { id, graph } => write!(
                f,
                "Algorithm error (Dijkstra): start node '{}' not found in graph {}",
                id, graph
            ),
            DijkstraError::MissingEndNode { id, graph } => write!(
                f,
                "Algorithm error (Dijkstra): end node '{}' not found in graph {}",
                id, graph
            ),
            DijkstraError::MissingNodeDuringProcessing { id } => write!(
                f,
                "Algorithm error (Dijkstra): node '{}' missing during distance processing",
                id
            ),
            DijkstraError::InvalidEdgeWeight {
                from,
                to,
                weight,
                reason,
            } => write!(
                f,
                "Algorithm error (Dijkstra): invalid edge weight {} on {} -> {} ({})",
                weight, from, to, reason
            ),
            DijkstraError::NoPathFound { start, end } => write!(
                f,
                "Algorithm error (Dijkstra): no path found from '{}' to '{}'",
                start, end
            ),
            DijkstraError::InvalidSearchResult { reason } => write!(
                f,
                "Algorithm error (Dijkstra): invalid search result ({})",
                reason
            ),
        }
    }
}

impl Error for DijkstraError {}
