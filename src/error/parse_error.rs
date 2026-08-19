//! Error types for parsing graph-related input lines and nodes.
//!
//! # Scope
//!
//! This module exposes the parser-facing errors used while reading graph
//! input lines, constructing nodes, and decoding numeric values.
//! [`ParseError`] captures the broad categories of failures, while
//! [`InvalidWeightError`] keeps the exact weight-parsing cause intact so the
//! caller can distinguish non-numeric input from integer overflow and other
//! numeric failures.
//!
//! # Overview
//!
//! The parser uses these errors in three places:
//!
//! - graph header validation,
//! - line syntax and node-id validation,
//! - weight parsing and node construction.
//!
//! The API is intentionally structured so CLI users see readable messages and
//! library consumers can still pattern-match on precise variants.
//!
//! # Examples
//!
//! Basic line validation:
//!
//! ```rust
//! use shortest_path_finder::error::ParseError;
//!
//! fn parse_node_line(line: &str) -> Result<(), ParseError> {
//!     if !line.contains(':') {
//!         return Err(ParseError::MissingColon);
//!     }
//!
//!     Ok(())
//! }
//!
//! assert!(matches!(parse_node_line("A:1,2"), Ok(())));
//! assert!(matches!(parse_node_line("A-1,2"), Err(ParseError::MissingColon)));
//! ```
//!
//! Weight classification:
//!
//! ```rust
//! use shortest_path_finder::error::parse_error::InvalidWeightError;
//! use shortest_path_finder::error::ParseError;
//!
//! let err = ParseError::InvalidWeight(InvalidWeightError::NonNumeric(
//!     "u16 weight expected".to_string(),
//! ));
//!
//! assert!(err.to_string().contains("not numeric"));
//! ```
//!
//! # Error Cases
//!
//! - [`ParseError::InvalidHeader`]: Graph header is invalid or unrecognized
//!   (expected `D`, `UN`, or `TD`).
//! - [`ParseError::MissingColon`]: Input line does not contain exactly one
//!   colon separating node id and coordinates.
//! - [`ParseError::InvalidCoordinates`]: Coordinates are not two comma-separated
//!   values.
//! - [`ParseError::InvalidInteger`]: Coordinates are not valid numeric values
//!   for the selected coordinate datatype.
//! - [`ParseError::InvalidWeight`]: Edge weight token is not a valid numeric
//!   value for the graph's weight type, with additional context in
//!   [`InvalidWeightError`].
//! - [`ParseError::EmptyId`]: Node id is empty.
//! - [`ParseError::NodeConstructionFailed`]: Node construction failed due to
//!   internal validation.
//! - [`ParseError::InvalidGraphType`]: Graph type could not be inferred from
//!   line content.
//! - [`ParseError::InvalidLineSyntax`]: Line does not match expected graph
//!   input syntax.
//! - [`ParseError::RegexCompilationFailed`]: Internal regex compilation failed
//!   during parser setup.
//! - [`ParseError::InvalidDataInput`]: Data input line or parser state is
//!   invalid with a detailed message.
//! - [`ParseError::GraphInsertionFailed`]: Graph insertion failed due to an internal error (for
//!   example, duplicate node id).

use std::error::Error;
use std::fmt::{self, Display};
use std::num::{IntErrorKind, ParseIntError};

use crate::graph::GraphInsertionError;

// ----- Implementation of the 'ParseError' enum -----

/// Errors that can occur while parsing graph-related node input.
///
/// This enum represents all error conditions encountered while parsing graph
/// input lines, node identifiers, and coordinate values.
///
/// # Responsibilities
///
/// - classify malformed graph headers,
/// - classify line-syntax and coordinate parsing problems,
/// - preserve structured weight parsing diagnostics,
/// - surface parser-internal setup failures with readable messages.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::error::ParseError;
///
/// fn classify_line(line: &str) -> Result<(), ParseError> {
///     if line.trim().is_empty() {
///         return Err(ParseError::InvalidDataInput("empty input line".to_string()));
///     }
///
///     Ok(())
/// }
///
/// assert!(classify_line("A:1,2").is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The graph header is invalid or unrecognized.
    ///
    /// Expected values are 'D' for directed, 'UN' for undirected, and 'TD' for two-dimensional
    /// graphs. The provided header string is included in the error for diagnostic purposes.
    InvalidHeader(String),
    /// The input string does not contain exactly one colon.
    ///
    /// Expected format: `<id>:<coordinates>` (e.g., `A:1,2`)
    MissingColon,
    /// The coordinates part does not contain exactly two comma-separated values.
    ///
    /// Expected format: `<x>,<y>` (e.g., `1,2`)
    InvalidCoordinates,
    /// The x or y coordinate could not be parsed as a numeric value.
    ///
    /// This occurs if either coordinate is not a valid value for the selected
    /// coordinate datatype of the node being parsed.
    InvalidInteger,
    /// The edge weight could not be parsed to a valid [`crate::graphs::graph::GraphWeight`] value.
    ///
    /// This variant preserves the structured weight-parsing cause in
    /// [`InvalidWeightError`]. That makes it possible to distinguish:
    ///
    /// - non-numeric input,
    /// - integer overflow or out-of-range values,
    /// - other parser-specific weight failures.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::error::parse_error::InvalidWeightError;
    /// use shortest_path_finder::error::ParseError;
    ///
    /// let err = ParseError::InvalidWeight(InvalidWeightError::OutOfRange(
    ///     "expected a u16-compatible value".to_string(),
    /// ));
    ///
    /// assert!(err.to_string().contains("out of range"));
    /// ```
    InvalidWeight(InvalidWeightError),
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
    /// Graph insertion failed while building a graph from parsed input.
    ///
    /// This variant wraps the concrete graph-layer insertion error so callers
    /// can inspect the exact failure while still handling parse errors through
    /// a single type.
    GraphInsertionFailed(GraphInsertionError),
    /// File/data input is invalid and includes a descriptive error message.
    ///
    /// This variant is used when parsing logic can provide additional context
    /// that is more specific than the standard enum variants.
    InvalidDataInput(String),
}

impl Display for ParseError {
    /// Formats the error as a human-readable string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidHeader(invalid_header) => write!(
                f,
                "Invalid graph header: '{}'! Expected exactly one of: 'D', 'UN', 'TD'.",
                invalid_header
            ),
            ParseError::MissingColon => write!(
                f,
                "Input must contain exactly one colon separating id and coordinates"
            ),
            ParseError::InvalidCoordinates => {
                write!(f, "Coordinates must be two comma-separated values")
            }
            ParseError::InvalidInteger => {
                write!(f, "Coordinates must be valid numeric values")
            }
            ParseError::InvalidWeight(cause) => {
                write!(f, "Invalid weight value: {}", cause)
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
            ParseError::GraphInsertionFailed(err) => {
                write!(f, "Graph insertion failed: {}", err)
            }
        }
    }
}

impl Error for ParseError {}

// ----- Implementation of the 'InvalidWeightError' enum -----

/// Represents the cause of an invalid weight parsing error.
///
/// This enum keeps the original weight-parsing cause intact so callers can
/// react differently to bad input, overflow, and parser-specific failure
/// messages.
///
/// # Variants
///
/// - [`InvalidWeightError::NonNumeric`]: The token could not be parsed as a
///   number at all.
/// - [`InvalidWeightError::OutOfRange`]: The parsed numeric value exceeded the
///   supported integer range.
/// - [`InvalidWeightError::CannotBeNegative`]: The graph type rejects negative
///   weights.
/// - [`InvalidWeightError::UnknownReason`]: A fallback message for unexpected
///   parsing failures.
///
/// # Examples
///
/// Converting a non-numeric integer token:
///
/// ```rust
/// use shortest_path_finder::error::parse_error::InvalidWeightError;
///
/// let parse_error = "abc".parse::<u16>().expect_err("non-numeric input should fail");
/// let err = InvalidWeightError::from(parse_error);
/// assert!(matches!(err, InvalidWeightError::NonNumeric(_)));
/// ```
///
/// Converting an out-of-range token:
///
/// ```rust
/// use shortest_path_finder::error::parse_error::InvalidWeightError;
///
/// let parse_error = "999999".parse::<u8>().expect_err("value is too large for u8");
/// let err = InvalidWeightError::from(parse_error);
/// assert!(matches!(err, InvalidWeightError::OutOfRange(_)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidWeightError {
    /// The weight value is not numeric and cannot be parsed to the expected type.
    NonNumeric(String),
    /// The weight value is outside the expected range for the graph's weight type.
    OutOfRange(String),
    /// The weight value is negative, which is not allowed for the graph's weight type.
    CannotBeNegative(String),
    /// An unknown reason for the invalid weight, with a descriptive message.
    UnknownReason(String),
}

impl Display for InvalidWeightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidWeightError::NonNumeric(expected_type) => {
                write!(
                    f,
                    "Weight value is not numeric. Expected: {}",
                    expected_type
                )
            }
            InvalidWeightError::OutOfRange(expected_range) => {
                write!(
                    f,
                    "Weight value is out of range. Expected: {}",
                    expected_range
                )
            }
            InvalidWeightError::CannotBeNegative(expected_type) => {
                write!(
                    f,
                    "Weight value cannot be negative. Expected: {}",
                    expected_type
                )
            }
            InvalidWeightError::UnknownReason(message) => {
                write!(f, "Invalid weight value: {}", message)
            }
        }
    }
}

impl Error for InvalidWeightError {}

impl From<ParseIntError> for InvalidWeightError {
    fn from(err: ParseIntError) -> Self {
        match err.kind() {
            IntErrorKind::Zero => InvalidWeightError::NonNumeric(String::from(
                "non-zero integer expected, but got 'ZERO' (zero value)",
            )),
            IntErrorKind::Empty => InvalidWeightError::NonNumeric(String::from(
                "non-empty integer expected, but got empty string ''",
            )),
            IntErrorKind::InvalidDigit => InvalidWeightError::NonNumeric(String::from(
                "non-numeric integer expected, but got invalid digit(s) (e.g., letters or symbols)",
            )),
            IntErrorKind::PosOverflow => {
                InvalidWeightError::OutOfRange(int_out_of_range_error_message())
            }
            IntErrorKind::NegOverflow => {
                InvalidWeightError::OutOfRange(int_out_of_range_error_message())
            }
            _ => {
                InvalidWeightError::UnknownReason(format!("Unknown integer parsing error: {}", err))
            }
        }
    }
}

/// Returns a detailed error message for integer values that are out of range for the graph's weight
/// type.
///
/// This message lists the supported integer types and their respective ranges, providing guidance
/// for users to correct their input.
fn int_out_of_range_error_message() -> String {
    "Integer value is out of range for the graph's weight type\nSupported integer types:\ni8: -128 to 127\ni16: -32,768 to 
32,767\ni32: -2,147,483,648 to 2,147,483,647\ni64: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807\nu8: 0 to 255\nu16: 0 to 
65,535\nu32: 0 to 4,294,967,295\nu64: 0 to 18,446,744,073,709,551,615".to_string()
}
