//! A* execution errors and path-reconstruction failures.
//!
//! This module collects the concrete failures that can surface while A*
//! searches a coordinate-aware graph. The errors are grouped around three
//! common causes:
//! - invalid graph setup, such as a graph that is not weighted,
//! - invalid search state, such as missing start or end nodes, invalid edge
//!   weights, or a non-finite heuristic value, and
//! - bookkeeping problems while reconstructing the predecessor chain.
//!
//! The public entry point is [`AStarError`], which carries enough context for
//! the CLI to classify the failure through
//! [`crate::error::algorithm_error::AlgorithmErrorKind`]. For low-level
//! predecessor-chain failures, [`PathReconstructionError`] exposes the
//! specific invariant that broke.
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::error::algorithm_error::{AStarError, AlgorithmErrorKind};
//!
//! let err = AStarError::MissingEndNode {
//!     id: "B".to_string(),
//! };
//! assert_eq!(err.kind(), AlgorithmErrorKind::MissingNode);
//! assert!(err.to_string().contains("end node"));
//! ```

use crate::error::algorithm_error::AlgorithmErrorKind;
use std::{error::Error, fmt};

/// Error returned while reconstructing a path from A* bookkeeping data.
///
/// # Context
///
/// These errors indicate inconsistent predecessor links in the closed set.
/// They are wrapped by [`AStarError::PathReconstruction`].
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::a_star_error::PathReconstructionError;
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
    MissingClosedEntry {
        /// Identifier of the node missing from the closed set.
        node_id: String,
    },
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
/// Use [`AStarError::kind`] to map failures to
/// [`AlgorithmErrorKind`] values.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::{AStarError, AlgorithmErrorKind};
///
/// let err = AStarError::UnweightedGraph;
/// assert_eq!(err.kind(), AlgorithmErrorKind::InvalidGraph);
/// ```
#[derive(Debug, Clone)]
pub enum AStarError {
    /// Graph is not weighted.
    UnweightedGraph,
    /// Start node does not exist in the graph.
    MissingStartNode {
        /// Identifier of the missing start node.
        id: String,
    },
    /// End node does not exist in the graph.
    MissingEndNode {
        /// Identifier of the missing end node.
        id: String,
    },
    /// Edge weight violates algorithm constraints.
    InvalidEdgeWeight {
        /// Source node identifier of the offending edge.
        from: String,
        /// Destination node identifier of the offending edge.
        to: String,
        /// String rendering of the rejected weight value.
        weight: String,
        /// Human-readable explanation of the constraint violation.
        reason: String,
    },
    /// Heuristic produced a non-finite value.
    InvalidHeuristic {
        /// Start node identifier used for the search.
        start: String,
        /// Goal node identifier used for the search.
        goal: String,
        /// Node identifier evaluated when the heuristic failed.
        current: String,
        /// Non-finite heuristic value that was produced.
        value: f32,
    },
    /// Expected g-cost entry is missing from the bookkeeping map.
    MissingGCost {
        /// Node identifier with no g-cost entry.
        node_id: String,
    },
    /// No path exists between the start and end nodes.
    NoPathFound {
        /// Start node identifier for the failed search.
        start: String,
        /// End node identifier for the failed search.
        end: String,
    },
    /// Search result failed validation.
    InvalidSearchResult {
        /// Explanation of why the result was rejected.
        reason: String,
    },
    /// Path reconstruction failed using closed-set bookkeeping.
    PathReconstruction {
        /// Underlying closed-set reconstruction failure.
        source: PathReconstructionError,
    },
}

impl AStarError {
    /// Returns the categorized error kind.
    pub fn kind(&self) -> AlgorithmErrorKind {
        match self {
            AStarError::UnweightedGraph => AlgorithmErrorKind::InvalidGraph,
            AStarError::MissingStartNode { .. } => AlgorithmErrorKind::MissingNode,
            AStarError::MissingEndNode { .. } => AlgorithmErrorKind::MissingNode,
            AStarError::InvalidEdgeWeight { .. } => AlgorithmErrorKind::InvalidWeight,
            AStarError::InvalidHeuristic { .. } => AlgorithmErrorKind::InvalidHeuristic,
            AStarError::MissingGCost { .. } => AlgorithmErrorKind::InvariantViolation,
            AStarError::NoPathFound { .. } => AlgorithmErrorKind::NoPath,
            AStarError::InvalidSearchResult { .. } => AlgorithmErrorKind::InvalidResult,
            AStarError::PathReconstruction { .. } => AlgorithmErrorKind::InvariantViolation,
        }
    }
}

impl fmt::Display for AStarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AStarError::UnweightedGraph => {
                write!(f, "Algorithm error (AStar): graph must be weighted")
            }
            AStarError::MissingStartNode { id } => {
                write!(f, "Algorithm error (AStar): start node '{}' not found", id)
            }
            AStarError::MissingEndNode { id } => {
                write!(f, "Algorithm error (AStar): end node '{}' not found", id)
            }
            AStarError::InvalidEdgeWeight {
                from,
                to,
                weight,
                reason,
            } => write!(
                f,
                "Algorithm error (AStar): invalid edge weight {} on {} -> {} ({})",
                weight, from, to, reason
            ),
            AStarError::InvalidHeuristic {
                start,
                goal,
                current,
                value,
            } => write!(
                f,
                "Algorithm error (AStar): heuristic produced non-finite value {} for start '{}', goal '{}', current '{}'",
                value, start, goal, current
            ),
            AStarError::MissingGCost { node_id } => write!(
                f,
                "Algorithm error (AStar): missing g-cost entry for node '{}'",
                node_id
            ),
            AStarError::NoPathFound { start, end } => write!(
                f,
                "Algorithm error (AStar): no path found from '{}' to '{}'",
                start, end
            ),
            AStarError::InvalidSearchResult { reason } => write!(
                f,
                "Algorithm error (AStar): invalid search result ({})",
                reason
            ),
            AStarError::PathReconstruction { source } => write!(
                f,
                "Algorithm error (AStar): path reconstruction failed: {}",
                source
            ),
        }
    }
}

impl Error for AStarError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AStarError::PathReconstruction { source } => Some(source),
            _ => None,
        }
    }
}

impl From<PathReconstructionError> for AStarError {
    fn from(source: PathReconstructionError) -> Self {
        AStarError::PathReconstruction { source }
    }
}
