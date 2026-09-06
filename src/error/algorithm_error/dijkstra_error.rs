//! Dijkstra execution errors and path-reconstruction failures.
//!
//! This module documents the error states that can occur while relaxing a
//! weighted graph in Dijkstra's algorithm. Typical failures include missing
//! start or end nodes, invalid edge weights, overflow while summing
//! distances, and predecessor-chain problems when the final path is rebuilt.
//!
//! Consumers usually work with [`DijkstraError`] directly, while the CLI
//! wrapper [`crate::error::algorithm_error::AlgorithmError`] exposes a stable
//! classification through
//! [`crate::error::algorithm_error::AlgorithmErrorKind`].
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::error::algorithm_error::{AlgorithmErrorKind, DijkstraError};
//!
//! let err = DijkstraError::NoPathFound {
//!     start: "A".to_string(),
//!     end: "B".to_string(),
//! };
//! assert_eq!(err.kind(), AlgorithmErrorKind::NoPath);
//! assert!(err.to_string().contains("no path"));
//! ```

use crate::error::algorithm_error::AlgorithmErrorKind;
use std::{error::Error, fmt};

/// Reason for rejecting an edge weight during Dijkstra execution.
///
/// # Meaning
///
/// - `Negative` means the weight is less than zero.
/// - `NonFinite` means the weight is NaN or +/- infinity.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::dijkstra_error::EdgeWeightViolation;
///
/// assert_eq!(EdgeWeightViolation::Negative.to_string(), "negative");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeWeightViolation {
    /// Weight is negative.
    Negative,
    /// Weight is non-finite (NaN or +/- infinity).
    NonFinite,
}

impl fmt::Display for EdgeWeightViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeWeightViolation::Negative => write!(f, "negative"),
            EdgeWeightViolation::NonFinite => write!(f, "non-finite"),
        }
    }
}

/// Context describing where a node went missing during Dijkstra processing.
///
/// # Meaning
///
/// This is surfaced when the distance map built from `get_all_nodes` does not
/// align with the adjacency lists traversed during relaxation. It usually
/// indicates the graph was mutated or built inconsistently.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::dijkstra_error::MissingNodeContext;
///
/// assert_eq!(MissingNodeContext::CurrentNode.to_string(), "current node");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingNodeContext {
    /// The current queue item was not found in the distance map.
    CurrentNode,
    /// A neighbor referenced by an edge was not found in the distance map.
    NeighborNode,
}

impl fmt::Display for MissingNodeContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MissingNodeContext::CurrentNode => write!(f, "current node"),
            MissingNodeContext::NeighborNode => write!(f, "neighbor node"),
        }
    }
}

/// Error returned while reconstructing a Dijkstra path.
///
/// # Causes
///
/// These errors indicate inconsistent predecessor links or missing distance
/// data after the relaxation phase.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::algorithm_error::dijkstra_error::PathReconstructionError;
///
/// let err = PathReconstructionError::MissingPredecessor {
///     node_id: "X".to_string(),
/// };
/// assert!(err.to_string().contains("missing predecessor"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathReconstructionError {
    /// No distance entry exists for the requested node.
    MissingDistanceEntry {
        /// Node identifier with no recorded distance.
        node_id: String,
    },
    /// Predecessor entry is missing while walking the chain.
    MissingPredecessor {
        /// Node identifier whose predecessor could not be resolved.
        node_id: String,
    },
    /// Predecessor traversal exceeded the number of known nodes.
    PredecessorLoop {
        /// Start node identifier for the attempted reconstruction.
        start: String,
        /// End node identifier for the attempted reconstruction.
        end: String,
        /// Node identifier where the predecessor chain stalled.
        current: String,
    },
}

impl fmt::Display for PathReconstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathReconstructionError::MissingDistanceEntry { node_id } => {
                write!(f, "missing distance entry for node '{}'", node_id)
            }
            PathReconstructionError::MissingPredecessor { node_id } => write!(
                f,
                "missing predecessor while reconstructing node '{}'",
                node_id
            ),
            PathReconstructionError::PredecessorLoop {
                start,
                end,
                current,
            } => write!(
                f,
                "predecessor chain looped while reconstructing '{}' -> '{}' (stuck at '{}')",
                start, end, current
            ),
        }
    }
}

impl Error for PathReconstructionError {}

/// Dijkstra execution errors.
///
/// # Categories
///
/// Use [`DijkstraError::kind`] to map failures to [`AlgorithmErrorKind`].
///
/// # Variants
///
/// Errors cover invalid graph setup, missing nodes, invalid weights, overflow,
/// path reconstruction failures, and invalid result invariants.
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
    MissingStartNode {
        /// Identifier of the missing start node.
        id: String,
        /// Graph type name included for diagnostics.
        graph: String,
    },
    /// End node does not exist in the graph.
    MissingEndNode {
        /// Identifier of the missing end node.
        id: String,
        /// Graph type name included for diagnostics.
        graph: String,
    },
    /// A node is missing while computing distances.
    ///
    /// The `context` field indicates whether the missing entry was the current
    /// node or one of its neighbors.
    MissingNodeDuringProcessing {
        /// Identifier of the node that could not be found in the distance map.
        id: String,
        /// Whether the missing entry was the current node or a neighbor.
        context: MissingNodeContext,
    },
    /// Edge weight violates algorithm constraints.
    InvalidEdgeWeight {
        /// Source node identifier of the offending edge.
        from: String,
        /// Destination node identifier of the offending edge.
        to: String,
        /// String rendering of the rejected weight value.
        weight: String,
        /// Classification of the weight violation.
        reason: EdgeWeightViolation,
    },
    /// Edge relaxation would overflow the distance datatype.
    DistanceOverflow {
        /// Source node identifier during relaxation.
        from: String,
        /// Destination node identifier during relaxation.
        to: String,
        /// String rendering of the accumulated distance before overflow.
        current_distance: String,
        /// String rendering of the edge weight that caused overflow.
        edge_weight: String,
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
    /// Path reconstruction failed using predecessor links.
    PathReconstruction {
        /// Underlying predecessor-chain reconstruction failure.
        source: PathReconstructionError,
    },
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
            DijkstraError::DistanceOverflow { .. } => AlgorithmErrorKind::InvalidWeight,
            DijkstraError::NoPathFound { .. } => AlgorithmErrorKind::NoPath,
            DijkstraError::InvalidSearchResult { .. } => AlgorithmErrorKind::InvalidResult,
            DijkstraError::PathReconstruction { .. } => AlgorithmErrorKind::InvariantViolation,
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
            DijkstraError::MissingNodeDuringProcessing { id, context } => write!(
                f,
                "Algorithm error (Dijkstra): node '{}' missing during distance processing ({})",
                id, context
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
            DijkstraError::DistanceOverflow {
                from,
                to,
                current_distance,
                edge_weight,
            } => write!(
                f,
                "Algorithm error (Dijkstra): distance overflow while relaxing edge {} -> {} (current: {}, edge: {})",
                from, to, current_distance, edge_weight
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
            DijkstraError::PathReconstruction { source } => write!(
                f,
                "Algorithm error (Dijkstra): path reconstruction failed: {}",
                source
            ),
        }
    }
}

impl Error for DijkstraError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DijkstraError::PathReconstruction { source } => Some(source),
            _ => None,
        }
    }
}

impl From<PathReconstructionError> for DijkstraError {
    fn from(source: PathReconstructionError) -> Self {
        DijkstraError::PathReconstruction { source }
    }
}
