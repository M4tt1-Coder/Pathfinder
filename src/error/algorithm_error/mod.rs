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
//! - [`AlgorithmErrorKind`]: groups error categories used by the CLI.
//! - [`AlgorithmError`]: wraps algorithm-specific error payloads.
//! - [`a_star_error::AStarError`]: A* execution failures.
//! - [`dijkstra_error::DijkstraError`]: Dijkstra execution failures.
//! - [`a_star_error::PathReconstructionError`]: A* path reconstruction failures.
//! - [`dijkstra_error::PathReconstructionError`]: Dijkstra path reconstruction failures.
//!
//! # CLI Integration
//!
//! `AlgorithmErrorKind::exit_code()` maps error categories to stable, numeric
//! exit codes for the CLI runtime. The [`AlgorithmError`] wrapper makes it easy
//! to convert algorithm-specific errors into a single CLI-facing payload.
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
/// # Meaning
///
/// These variants classify errors across algorithms so the CLI can provide
/// consistent exit codes and messaging.
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
/// # Purpose
///
/// The CLI wants one error type that exposes both a human-readable display and
/// a categorized [`AlgorithmErrorKind`]. This enum provides that wrapper.
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
    AStar(AStarError),
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

impl From<AStarError> for AlgorithmError {
    fn from(err: AStarError) -> Self {
        Self::AStar(err)
    }
}

impl From<DijkstraError> for AlgorithmError {
    fn from(err: DijkstraError) -> Self {
        Self::Dijkstra(err)
    }
}

// ========== Modulization ==========

// ~ public modules ~

pub mod a_star_error;
pub mod dijkstra_error;

// ~ flatten module paths ~

pub use a_star_error::AStarError;
pub use dijkstra_error::DijkstraError;
