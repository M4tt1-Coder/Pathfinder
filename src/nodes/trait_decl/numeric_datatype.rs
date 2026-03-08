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
//! use crate::graphs::graph::GraphWeight;
//! use crate::your_module::NumericDatatype;
//!
//! fn scale_weight<W: NumericDatatype>(weight: W, factor: W) -> W {
//!     weight * factor
//! }
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
/// This trait doesn't add new methods but serves as a convenient bound for generic functions requiring numeric types with these operations.
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
}
