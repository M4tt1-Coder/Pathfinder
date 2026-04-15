//! # NumericDatatype Trait
//!
//! The `NumericDatatype` trait extends `GraphWeight` with common arithmetic operations needed for numeric computations.
//! It is designed for types that support subtraction, multiplication, and division, making it suitable for numerical data
//! used in graph algorithms, weights, or coordinate calculations.
//!
//! # Bounds
//! - Implements `GraphWeight` (assumed to be a trait for types usable as weights).
//! - Supports subtraction (`Sub`) with output of the same type.
//! - Supports multiplication (`Mul`) with output of the same type.
//! - Supports division (`Div`) with output of the same type.
//! - `Sized` for compile-time size information, typical for primitive numeric types.
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
//! ```
//!

use std::ops::{Div, Mul, Sub};

use crate::graphs::graph::GraphWeight;

/// `NumericDatatype` extends `GraphWeight` with essential arithmetic operations for numerical data.
///
/// It is intended for types that support subtraction, multiplication, and division,
/// such as `f32`, `f64`, `i32`, `u32`, etc.
///
/// # Bounds
/// - Implements `GraphWeight`.
/// - Supports `Sub` with output `Self`.
/// - Supports `Mul` with output `Self`.
/// - Supports `Div` with output `Self`.
/// - `Sized` for compile-time size.
///
/// # Note
/// Besides arithmetic bounds, this trait defines helper methods used by heuristics
/// and mixed-type numeric flows (`adjust_for_heuristic`, `to_f32`, and
/// `from_f32`).
pub trait NumericDatatype:
    GraphWeight + Sub<Output = Self> + Sized + Mul<Output = Self> + Div<Output = Self>
{
    /// Returns the absolute value of the numeric type.
    ///
    /// # Returns
    ///
    /// => The absolute value of the implementing type.
    fn abs(&self) -> Self;

    /// Adjusts the value for heuristic calculations, if necessary.
    ///
    /// This method can be used to modify the value based on specific heuristic requirements, such
    /// as scaling or normalization.
    ///
    /// # Returns
    ///
    /// => The adjusted value for heuristic calculations.
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
