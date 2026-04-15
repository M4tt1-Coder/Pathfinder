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

/// Constant factor used by `adjust_for_heuristic()` implementations.
static HEURISTIC_ADJUSTMENT_FACTOR: f32 = 0.001;

impl NumericDatatype for f32 {
    fn abs(&self) -> Self {
        f32::abs(*self)
    }

    fn adjust_for_heuristic(&self) -> Self {
        *self * HEURISTIC_ADJUSTMENT_FACTOR
    }
}

impl NumericDatatype for i32 {
    fn abs(&self) -> Self {
        i32::abs(*self)
    }

    fn adjust_for_heuristic(&self) -> Self {
        *self * HEURISTIC_ADJUSTMENT_FACTOR as i32
    }
}
