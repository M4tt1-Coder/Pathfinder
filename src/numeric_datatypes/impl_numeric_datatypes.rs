//! `NumericDatatype` implementations for primitive numeric types.
//!
//! # Overview
//!
//! This module provides concrete implementations of
//! [`NumericDatatype`](crate::weight_types::numeric_datatype::NumericDatatype)
//! for selected primitive types used by graph algorithms and heuristics.
//!
//! Currently implemented:
//! - `f32`
//! - `i32`
//!
//! # Heuristic Adjustment
//!
//! `adjust_for_heuristic()` applies a lightweight scaling factor intended for
//! heuristic score tuning in coordinate-based pathfinding.
//!
//! Current implementation details:
//! - `f32`: multiplies by `0.001`.
//! - `i32`: converts to `f32`, multiplies by `0.001`, rounds, then converts
//!   back to `i32`.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::weight_types::numeric_datatype::NumericDatatype;
//!
//! let value = -4_i32;
//! assert_eq!(value.abs(), 4);
//!
//! let heuristic = 10.0_f32.adjust_for_heuristic();
//! assert!(heuristic > 0.0);
//! ```

use crate::weight_types::numeric_datatype::NumericDatatype;

/// Constant factor used by heuristic scaling implementations.
///
/// # Rationale
///
/// Cross-product magnitudes in A* heuristic calculations can grow quickly for
/// larger coordinate differences. A small factor keeps heuristic values in a
/// practical range relative to edge weights.
static HEURISTIC_ADJUSTMENT_FACTOR: f32 = 0.001;

/// [`NumericDatatype`] implementation for `f32`.
///
/// This variant preserves fractional precision when used as a path-cost type.
impl NumericDatatype for f32 {
    fn abs(&self) -> Self {
        f32::abs(*self)
    }

    fn adjust_for_heuristic(&self) -> Self {
        // Keep fractional precision when scaling heuristic magnitudes.
        *self * HEURISTIC_ADJUSTMENT_FACTOR
    }

    fn to_f32(&self) -> f32 {
        *self
    }

    fn from_f32(value: f32) -> Self {
        value
    }
}

/// [`NumericDatatype`] implementation for `i32`.
///
/// Heuristic scaling is performed in floating-point space and then rounded back
/// to integer values to preserve compatibility with integer-weighted graphs.
impl NumericDatatype for i32 {
    fn abs(&self) -> Self {
        i32::abs(*self)
    }

    fn adjust_for_heuristic(&self) -> Self {
        // Scale in f32 space first, then round to the nearest integer score.
        (*self as f32 * HEURISTIC_ADJUSTMENT_FACTOR).round() as i32
    }

    fn to_f32(&self) -> f32 {
        *self as f32
    }

    fn from_f32(value: f32) -> Self {
        value as i32
    }
}
