// ----- Implementation of the 'TwoDimensionalNode' struct -----

use std::fmt::Display;

use crate::{graphs::graph::GraphNode, nodes::trait_decl::coordinates_node::CoordinatesNode};

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
    x: u16,
    /// -- Private field --
    ///
    /// Y - ordinate of the individual 'TwoDimensionalNode' struct instance.
    y: u16,
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
    pub fn new(x: u16, y: u16, id: String) -> Option<Self> {
        // id must be longer then 0
        if id.is_empty() {
            return None;
        };
        Some(Self { x, y, id })
    }
}

impl CoordinatesNode for TwoDimensionalNode {
    type CoordinateType = u16;

    /// Returns the Y ordinate of the 'TwoDimensionalNode' in the graph.
    fn get_x(&self) -> u16 {
        self.x
    }

    /// Provides the Y ordinate of the node in the graph.
    fn get_y(&self) -> u16 {
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
