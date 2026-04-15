//! Extended numeric trait used by weighted graph algorithms.
//!
//! # Overview
//!
//! [`NumericDatatype`] builds on [`crate::graphs::graph::GraphWeight`] and adds
//! arithmetic functionality needed by heuristic-based algorithms such as A*.
//!
//! # Responsibilities
//!
//! Implementors must provide:
//! - arithmetic operators (`Sub`, `Mul`, `Div`),
//! - absolute-value handling via [`NumericDatatype::abs`],
//! - heuristic adjustment via [`NumericDatatype::adjust_for_heuristic`],
//! - conversion helpers to and from `f32`.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::weight_types::numeric_datatype::NumericDatatype;
//!
//! fn scale_weight<W: NumericDatatype>(weight: W, factor: W) -> W {
//!     weight * factor
//! }
//!
//! assert_eq!(scale_weight(3i32, 2i32), 6i32);
//! assert_eq!((-3_i32).abs(), 3_i32);
//! ```

use std::ops::{Div, Mul, Sub};

use crate::graphs::graph::GraphWeight;

/// Numeric contract for algorithm weight/coordinate datatypes.
///
/// # Design intent
///
/// This trait allows algorithms to stay generic over integer and floating-point
/// types while still performing heuristic math in a predictable way.
pub trait NumericDatatype:
    GraphWeight + Sub<Output = Self> + Sized + Mul<Output = Self> + Div<Output = Self>
{
    /// Returns the absolute value of the numeric value.
    ///
    /// # Returns
    ///
    /// Absolute value of the implementing type.
    fn abs(&self) -> Self;

    /// Adjusts the value for heuristic calculations.
    ///
    /// This method can be used to modify the value based on specific heuristic requirements, such
    /// as scaling or normalization.
    ///
    /// # Returns
    ///
    /// Adjusted value used in heuristic calculations.
    fn adjust_for_heuristic(&self) -> Self;

    /// Converts the numeric value into an `f32` representation.
    ///
    /// This is primarily used by mixed-type heuristic calculations where
    /// coordinate and edge-weight datatypes differ.
    fn to_f32(&self) -> f32;

    /// Creates a value of `Self` from an `f32` representation.
    ///
    /// Implementations may apply a lossy conversion depending on target type.
    fn from_f32(value: f32) -> Self;
}
