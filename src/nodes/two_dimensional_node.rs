//! Two-dimensional node implementation used in coordinate-based graphs.
//!
//! # Overview
//!
//! [`TwoDimensionalNode`] represents a graph node with:
//! - a unique textual identifier,
//! - a typed x-coordinate,
//! - a typed y-coordinate.
//!
//! It implements both [`GraphNode`](crate::graphs::graph::GraphNode) and
//! [`CoordinatesNode`](crate::nodes::trait_decl::coordinates_node::CoordinatesNode),
//! enabling use in generic graph and pathfinding algorithms.
//!
//! # Coordinate Type
//!
//! `TwoDimensionalNode` is generic over coordinate type `C`:
//! - default: `C = i32`,
//! - supported by default in this crate: `i32`, `f32`, and `u8`,
//! - custom coordinate types can be used when they implement
//!   [`CoordinateDatatype`](crate::nodes::trait_decl::coordinate_datatype::CoordinateDatatype).
//!
//! # Identity and Ordering Semantics
//!
//! Node identity and ordering are based on the node ID only.
//! This keeps compatibility with graph constraints that require nodes to implement
//! `Eq`, `Hash`, and `Ord`, while allowing coordinate datatypes like `f32` that do
//! not provide full total ordering semantics.
//!
//! # Parsing Format
//!
//! `FromStr` supports parsing from `<id>:<x>,<y>` strings.
//! Parsing is generic as long as `C: FromStr`.
//!
//! # Usage
//!
//! ```rust
//! use std::str::FromStr;
//! use shortest_path_finder::graphs::graph::GraphNode;
//! use shortest_path_finder::nodes::{
//!     trait_decl::coordinates_node::CoordinatesNode,
//!     two_dimensional_node::TwoDimensionalNode,
//! };
//!
//! let node = TwoDimensionalNode::<i32>::from_str("Hub:3,5").unwrap();
//! assert_eq!(node.get_id(), "Hub");
//! assert_eq!(node.get_x(), 3);
//! assert_eq!(node.get_y(), 5);
//!
//! let precise = TwoDimensionalNode::<f32>::new(1.5, 2.75, "F32Node".to_string()).unwrap();
//! assert_eq!(precise.get_x(), 1.5);
//!
//! let parsed = "P:1.25,2.5".parse::<TwoDimensionalNode<f32>>().unwrap();
//! assert_eq!(parsed.get_y(), 2.5);
//! ```

// ----- Implementation of the 'TwoDimensionalNode' struct -----

use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::{
    error::parse_error::ParseError,
    graphs::graph::GraphNode,
    nodes::trait_decl::{
        coordinate_datatype::CoordinateDatatype, coordinates_node::CoordinatesNode,
    },
};

/// Coordinate-aware node type used by two-dimensional graph models.
///
/// # Invariants
///
/// - `id` must not be empty.
/// - Coordinates are stored as `C`, where `C: CoordinateDatatype`.
/// - Fields are private and immutable from outside the type after creation.
/// - Equality, hash, and ordering are based on `id` only.
///
/// # Type Parameter
///
/// - `C`: coordinate scalar type, defaulting to `i32`.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::graphs::graph::GraphNode;
/// use shortest_path_finder::nodes::{
///     trait_decl::coordinates_node::CoordinatesNode,
///     two_dimensional_node::TwoDimensionalNode,
/// };
///
/// let node = TwoDimensionalNode::new(2, -1, "Depot".to_string()).unwrap();
/// assert_eq!(node.get_id(), "Depot");
/// assert_eq!(node.get_x(), 2);
/// assert_eq!(node.get_y(), -1);
///
/// let floating = TwoDimensionalNode::<f32>::new(2.5, -1.0, "FloatDepot".to_string()).unwrap();
/// assert_eq!(floating.get_x(), 2.5);
/// ```
#[derive(Debug, Clone)]
pub struct TwoDimensionalNode<C: CoordinateDatatype = i32> {
    /// -- Private Field --
    ///
    /// The unique identifier for the node. It can be seen as its name too, but is used as an
    /// IDsince it the name needs to be unique in a graph.
    id: String,

    /// -- Private Field --
    ///
    /// X - ordinate of the individual 'TwoDimensionalNode' struct instance.
    x: C,

    /// -- Private field --
    ///
    /// Y - ordinate of the individual 'TwoDimensionalNode' struct instance.
    y: C,
}

impl<C: CoordinateDatatype> TwoDimensionalNode<C> {
    /// Creates a validated [`TwoDimensionalNode`].
    ///
    /// # Arguments
    ///
    /// - `x`: x-coordinate.
    /// - `y`: y-coordinate.
    /// - `id`: unique non-empty identifier.
    ///
    /// # Returns
    ///
    /// - `Some(Self)` if `id` is non-empty.
    /// - `None` if `id` is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let node = TwoDimensionalNode::new(2, 7, "N1".to_string());
    /// assert!(node.is_some());
    /// ```
    ///
    /// ```rust
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let node = TwoDimensionalNode::new(2, 7, "".to_string());
    /// assert!(node.is_none());
    /// ```
    pub fn new(x: C, y: C, id: String) -> Option<Self> {
        // id must be longer then 0
        if id.is_empty() {
            return None;
        };
        Some(Self { x, y, id })
    }
}

impl<C: CoordinateDatatype> CoordinatesNode for TwoDimensionalNode<C> {
    type CoordinateType = C;

    /// Returns the x-coordinate of this node.
    fn get_x(&self) -> C {
        self.x
    }

    /// Returns the y-coordinate of this node.
    fn get_y(&self) -> C {
        self.y
    }
}

impl<C: CoordinateDatatype> GraphNode for TwoDimensionalNode<C> {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl<C: CoordinateDatatype> Display for TwoDimensionalNode<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}, X-ordinate: {}, Y-ordinate: {}",
            self.id, self.x, self.y
        )
    }
}

impl<C: CoordinateDatatype> PartialEq for TwoDimensionalNode<C> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<C: CoordinateDatatype> Eq for TwoDimensionalNode<C> {}

impl<C: CoordinateDatatype> Hash for TwoDimensionalNode<C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<C: CoordinateDatatype> PartialOrd for TwoDimensionalNode<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: CoordinateDatatype> Ord for TwoDimensionalNode<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<C> FromStr for TwoDimensionalNode<C>
where
    C: CoordinateDatatype + FromStr,
{
    type Err = ParseError;

    /// Parses a node from `<id>:<x>,<y>` input.
    ///
    /// # Parsing Rules
    ///
    /// - Exactly one `:` must separate ID and coordinate payload.
    /// - Exactly one `,` must separate `x` and `y`.
    /// - Both coordinates must parse as coordinate type `C`.
    /// - ID must not be empty.
    ///
    /// # Returns
    ///
    /// - `Ok(TwoDimensionalNode)` on valid parse.
    /// - `Err(ParseError)` on malformed identifiers or coordinates.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use shortest_path_finder::graphs::graph::GraphNode;
    /// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
    ///
    /// let node = TwoDimensionalNode::<i32>::from_str("P:10,12").unwrap();
    /// assert_eq!(node.get_id(), "P");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Trim whitespace and split into two parts: id and coordinates
        let mut parts = s.trim().splitn(2, ':');
        let id_part = parts.next().unwrap_or("");
        let coord_part = parts.next();

        // Input must have exactly one colon
        let coord_part = match coord_part {
            Some(c) => c,
            None => return Err(ParseError::MissingColon),
        };

        let id = id_part.trim();
        if id.is_empty() {
            return Err(ParseError::EmptyId);
        }

        // Split coordinates by comma
        let mut coordinates = coord_part.trim().splitn(2, ',');
        let x_str = coordinates.next().unwrap_or("");
        let y_str = coordinates.next();

        // Must have exactly two coordinates
        let y_str = match y_str {
            Some(y) if !y.is_empty() => y,
            _ => return Err(ParseError::InvalidCoordinates),
        };

        // Parse x and y as the configured coordinate scalar type.
        let x: C = x_str
            .trim()
            .parse()
            .map_err(|_| ParseError::InvalidInteger)?;
        let y: C = y_str
            .trim()
            .parse()
            .map_err(|_| ParseError::InvalidInteger)?;

        // Construct the node, returning error if construction fails
        TwoDimensionalNode::new(x, y, id.to_string()).ok_or(ParseError::NodeConstructionFailed)
    }
}
