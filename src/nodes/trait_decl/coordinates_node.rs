//! # CoordinatesNode Trait
//!
//! The `CoordinatesNode` trait extends the `GraphNode` trait by adding spatial coordinate information.
//! It is designed for graph nodes that have associated coordinates, such as nodes in a spatial graph or map.
//!
//! # Usage
//!
//! ```rust
//! use pathfinder::graphs::graph::{GraphNode, GraphWeight, CoordinatesNode};
//!
//! struct MapNode {
//!     id: String,
//!     x: f64,
//!     y: f64,
//! }
//!
//! impl GraphNode for MapNode {
//!     fn get_id(&self) -> &str {
//!         &self.id
//!     }
//! }
//!
//! impl CoordinatesNode for MapNode {
//!     type CoordinateType = f64;
//!     
//!     fn get_x(&self) -> Self::CoordinateType {
//!         self.x
//!     }
//!     
//!     fn get_y(&self) -> Self::CoordinateType {
//!         self.y
//!     }
//! }
//! ```
//!

use crate::{graphs::graph::GraphNode, nodes::trait_decl::numeric_datatype::NumericDatatype};

/// `CoordinatesNode` is a trait for graph nodes that have spatial coordinate information, extending `GraphNode`.
///
/// It introduces an associated type `CoordinateType` which must implement `GraphWeight`, allowing flexibility
/// in the type used for coordinates (e.g., `f32`, `f64`, `i32`, etc.).
///
/// The trait provides methods to retrieve the x and y coordinates of the node.
///
/// # Type Parameters
/// - `CoordinateType`: The type used for the node's coordinates, must implement `GraphWeight`.
pub trait CoordinatesNode: GraphNode {
    /// The type used for the node's coordinates.
    type CoordinateType: NumericDatatype;

    /// Retrieves the x-coordinate of the node.
    ///
    /// # Returns
    /// The x-coordinate of type `Self::CoordinateType`.
    fn get_x(&self) -> Self::CoordinateType;

    /// Retrieves the y-coordinate of the node.
    ///
    /// # Returns
    /// The y-coordinate of type `Self::CoordinateType`.
    fn get_y(&self) -> Self::CoordinateType;
}
