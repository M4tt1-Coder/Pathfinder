// ----- Implementation of the 'TwoDimensionalNode' struct -----

use std::{fmt::Display, str::FromStr};

use crate::{
    error::parse_error::ParseError, graphs::graph::GraphNode,
    nodes::trait_decl::coordinates_node::CoordinatesNode,
};

// TODO: Introduce generic coordinate datatypes (f32, f64, i64, ...)

/// Node in a 'TwoDimensionalCoordinateGraph'.
///
/// In that context the node needs to hold information about where the node is placed on the 'map'.
///
/// All attributes are private and can't be mutated from outside after inizialization.
///
/// # Fields
///
/// - 'id' -> Identifier
/// - 'x' -> X - ordinate
/// - 'y' -> Y - ordinate
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TwoDimensionalNode {
    /// -- Private Field --
    ///
    /// The unique identifier for the node. It can be seen as its name too, but is used as an
    /// IDsince it the name needs to be unique in a graph.
    id: String,

    /// -- Private Field --
    ///
    /// X - ordinate of the individual 'TwoDimensionalNode' struct instance.
    x: i32,

    /// -- Private field --
    ///
    /// Y - ordinate of the individual 'TwoDimensionalNode' struct instance.
    y: i32,
}

impl TwoDimensionalNode {
    /// Creates a new instance of the 'TwoDimensionalNode' struct.
    ///
    /// When the identifier has a length of 0, then no new object is being created.
    ///
    /// # Arguments
    ///
    /// - 'x' -> X-ordinate of the node
    /// - 'y' -> Y-ordinate of the node
    /// - 'id' -> unique identifier of the node, which can't be null or a duplicate in the graph
    ///
    /// (external check)
    ///
    /// # Returns
    ///
    /// => Validated fresh 'TwoDimensionalNode'
    pub fn new(x: i32, y: i32, id: String) -> Option<Self> {
        // id must be longer then 0
        if id.is_empty() {
            return None;
        };
        Some(Self { x, y, id })
    }
}

impl CoordinatesNode for TwoDimensionalNode {
    type CoordinateType = i32;

    /// Returns the Y ordinate of the 'TwoDimensionalNode' in the graph.
    fn get_x(&self) -> i32 {
        self.x
    }

    /// Provides the Y ordinate of the node in the graph.
    fn get_y(&self) -> i32 {
        self.y
    }
}

impl GraphNode for TwoDimensionalNode {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl Display for TwoDimensionalNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}, X-ordinate: {}, Y-ordinate: {}",
            self.id, self.x, self.y
        )
    }
}

impl FromStr for TwoDimensionalNode {
    type Err = ParseError;

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

        // Parse x and y as integers
        let x: i32 = x_str
            .trim()
            .parse()
            .map_err(|_| ParseError::InvalidInteger)?;
        let y: i32 = y_str
            .trim()
            .parse()
            .map_err(|_| ParseError::InvalidInteger)?;

        // Construct the node, returning error if construction fails
        TwoDimensionalNode::new(x, y, id.to_string()).ok_or(ParseError::NodeConstructionFailed)
    }
}
