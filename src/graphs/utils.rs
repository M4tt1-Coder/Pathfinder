//! Shared graph utility helpers.
//!
//! This module contains small, reusable helpers used by graph implementations
//! and algorithms. The functions here are intentionally generic so they can be
//! reused across graph variants.

use crate::nodes::trait_decl::{
    coordinate_datatype::CoordinateDatatype, coordinates_node::CoordinatesNode,
};

/// Calculates edge weight using Euclidean distance between endpoints.
///
/// # Formula
///
/// For endpoint coordinates $(x_1, y_1)$ and $(x_2, y_2)$, this function
/// computes:
///
/// $$
/// \sqrt{(x_1 - x_2)^2 + (y_1 - y_2)^2}
/// $$
///
/// # Returns
///
/// Non-negative floating-point weight used by shortest-path algorithms.
pub fn calculate_weight<CN: CoordinatesNode>(node_one: &CN, node_two: &CN) -> f32 {
    // Convert coordinates to f32 to perform geometric calculations.
    let dx = node_one.get_x().to_f32() - node_two.get_x().to_f32();
    let dy = node_one.get_y().to_f32() - node_two.get_y().to_f32();

    // Euclidean norm in 2D.
    (dx * dx + dy * dy).sqrt()
}
