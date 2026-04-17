//! Error types for parsing graph-related input lines and nodes.
//!
//! This module defines [`ParseError`], which enumerates all possible errors that can occur
//! during the parsing of graph input lines, node identifiers, and coordinate values. These errors
//! are surfaced by the graph file parser and node construction routines, and are intended to provide
//! precise diagnostics for both CLI users and library consumers.
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::error::parse_error::ParseError;
//!
//! fn parse_node_line(line: &str) -> Result<(), ParseError> {
//!     // Example usage
//!     if !line.contains(':') {
//!         return Err(ParseError::MissingColon);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Error Cases
//!
//! - [`ParseError::MissingColon`]: Input line does not contain exactly one colon separating node id and coordinates.
//! - [`ParseError::InvalidCoordinates`]: Coordinates are not two comma-separated values.
//! - [`ParseError::InvalidInteger`]: Coordinates are not valid integers.
//! - [`ParseError::InvalidWeightInteger`]: Edge weight token is not a valid integer.
//! - [`ParseError::EmptyId`]: Node id is empty.
//! - [`ParseError::NodeConstructionFailed`]: Node construction failed due to internal validation.
//! - [`ParseError::InvalidGraphType`]: Graph type could not be inferred from line content.
//! - [`ParseError::InvalidLineSyntax`]: Line does not match expected graph input syntax.
//! - [`ParseError::RegexCompilationFailed`]: Internal regex compilation failed during parser setup.
//! - [`ParseError::InvalidDataInput`]: Data input line or parser state is invalid with a detailed message.

use std::error::Error;
use std::fmt;

/// Errors that can occur while parsing graph-related node input.
///
/// This enum represents all possible error conditions encountered when parsing
/// graph input lines, node identifiers, and coordinate values. Each variant
/// corresponds to a specific validation or parsing failure, and is used to
/// provide detailed error reporting to the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The input string does not contain exactly one colon.
    ///
    /// Expected format: `<id>:<coordinates>` (e.g., `A:1,2`)
    MissingColon,
    /// The coordinates part does not contain exactly two comma-separated values.
    ///
    /// Expected format: `<x>,<y>` (e.g., `1,2`)
    InvalidCoordinates,
    /// The x or y coordinate could not be parsed as an integer.
    ///
    /// This occurs if either coordinate is not a valid integer value.
    InvalidInteger,
    /// The edge weight token could not be parsed as an integer.
    ///
    /// This variant is used for one-dimensional graph edges where a numeric
    /// weight is required (e.g., `A->B:7`).
    InvalidWeightInteger,
    /// The node id is empty.
    ///
    /// Node identifiers must not be empty strings.
    EmptyId,
    /// Construction of the node failed (e.g., due to internal validation).
    ///
    /// This error is returned if the node could not be constructed even though
    /// the input was syntactically valid.
    NodeConstructionFailed,
    /// Graph type was not recognized while parsing line content.
    ///
    /// This occurs if the parser cannot infer whether the line describes a directed
    /// or undirected edge, or if the syntax is inconsistent.
    InvalidGraphType,
    /// Generic syntax validation for graph line parsing failed.
    ///
    /// This is a catch-all for lines that do not match any expected graph input format.
    InvalidLineSyntax,
    /// Internal parser regex configuration could not be compiled.
    ///
    /// This indicates an internal setup issue rather than malformed user data.
    RegexCompilationFailed(String),
    /// File/data input is invalid and includes a descriptive error message.
    ///
    /// This variant is used when parsing logic can provide additional context
    /// that is more specific than the standard enum variants.
    InvalidDataInput(String),
}

impl fmt::Display for ParseError {
    /// Formats the error as a human-readable string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingColon => write!(
                f,
                "Input must contain exactly one colon separating id and coordinates"
            ),
            ParseError::InvalidCoordinates => {
                write!(f, "Coordinates must be two comma-separated integers")
            }
            ParseError::InvalidInteger => write!(f, "Coordinates must be valid integers"),
            ParseError::InvalidWeightInteger => {
                write!(f, "Edge weight must be a valid integer")
            }
            ParseError::EmptyId => write!(f, "Node id must not be empty"),
            ParseError::NodeConstructionFailed => {
                write!(f, "Failed to construct TwoDimensionalNode")
            }
            ParseError::InvalidGraphType => {
                write!(f, "Invalid graph type for line conversion")
            }
            ParseError::InvalidLineSyntax => {
                write!(f, "Invalid syntax for graph input line")
            }
            ParseError::RegexCompilationFailed(message) => {
                write!(f, "Failed to initialize parser regex: {}", message)
            }
            ParseError::InvalidDataInput(message) => write!(f, "{}", message),
        }
    }
}

impl Error for ParseError {}
