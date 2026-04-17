//! Coordinate-bearing node contract.
//!
//! # Overview
//!
//! [`CoordinatesNode`] extends [`crate::graphs::graph::GraphNode`] with x/y
//! coordinate accessors. It is used by coordinate-aware graph models and by
//! A* implementations that require geometric information.
//!
//! # Coordinate Semantics
//!
//! The trait does not enforce a specific coordinate system (cartesian, grid,
//! map projection, etc.). It only guarantees that callers can retrieve two
//! scalar values via [`CoordinatesNode::get_x`] and [`CoordinatesNode::get_y`].
//!
//! # Usage
//!
//! ```rust
//! use std::fmt::{Display, Formatter};
//! use shortest_path_finder::graphs::graph::GraphNode;
//! use shortest_path_finder::nodes::trait_decl::coordinates_node::CoordinatesNode;
//!
//! #[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
//! struct MapNode {
//!     id: String,
//!     x: i32,
//!     y: i32,
//! }
//!
//! impl Display for MapNode {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "{}", self.id)
//!     }
//! }
//!
//! impl GraphNode for MapNode {
//!     fn get_id(&self) -> &str {
//!         &self.id
//!     }
//! }
//!
//! impl CoordinatesNode for MapNode {
//!     type CoordinateType = i32;
//!
//!     fn get_x(&self) -> Self::CoordinateType {
//!         self.x
//!     }
//!
//!     fn get_y(&self) -> Self::CoordinateType {
//!         self.y
//!     }
//! }
//!
//! fn manhattan_distance<N: CoordinatesNode<CoordinateType = i32>>(a: &N, b: &N) -> i32 {
//!     (a.get_x() - b.get_x()).abs() + (a.get_y() - b.get_y()).abs()
//! }
//!
//! let a = MapNode { id: "A".to_string(), x: 0, y: 0 };
//! let b = MapNode { id: "B".to_string(), x: 3, y: 4 };
//! assert_eq!(manhattan_distance(&a, &b), 7);
//! ```

use crate::{graphs::graph::GraphNode, weight_types::numeric_datatype::NumericDatatype};

/// Trait for nodes that expose two coordinates in addition to an identifier.
///
/// # Associated Types
///
/// - [`CoordinatesNode::CoordinateType`]: scalar type used for x/y values.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::nodes::{
///     trait_decl::coordinates_node::CoordinatesNode,
///     two_dimensional_node::TwoDimensionalNode,
/// };
///
/// let node = TwoDimensionalNode::new(5, 8, "P".to_string()).unwrap();
/// assert_eq!(node.get_x(), 5);
/// assert_eq!(node.get_y(), 8);
/// ```
pub trait CoordinatesNode: GraphNode {
    /// The type used for the node's coordinates.
    type CoordinateType: NumericDatatype;

    /// Returns the x-coordinate of the node.
    ///
    /// # Returns
    /// The x-coordinate of type `Self::CoordinateType`.
    fn get_x(&self) -> Self::CoordinateType;

    /// Returns the y-coordinate of the node.
    ///
    /// # Returns
    /// The y-coordinate of type `Self::CoordinateType`.
    fn get_y(&self) -> Self::CoordinateType;
}
