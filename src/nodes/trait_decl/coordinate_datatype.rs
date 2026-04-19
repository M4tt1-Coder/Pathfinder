//! Coordinate-specific numeric contract for node coordinate values.
//!
//! # Overview
//!
//! [`CoordinateDatatype`] defines the minimum capabilities required from
//! coordinate scalars used by coordinate-aware nodes and graphs.
//!
//! It is intentionally separated from weight-focused numeric traits so
//! coordinate semantics can evolve independently from edge-weight semantics.
//!
//! # Required Capabilities
//!
//! Implementors must support:
//! - copying by value,
//! - ordering/equality checks,
//! - subtraction,
//! - absolute value,
//! - conversion to and from `f32` for geometry helpers.
//!
//! # Examples
//!
//! ```rust
//! use shortest_path_finder::nodes::trait_decl::coordinate_datatype::CoordinateDatatype;
//!
//! fn manhattan_step<C: CoordinateDatatype>(a: C, b: C) -> C {
//!     (a - b).abs()
//! }
//!
//! assert_eq!(manhattan_step(7_i32, 2_i32), 5_i32);
//! assert_eq!(manhattan_step(7_u8, 2_u8), 5_u8);
//! assert!((manhattan_step(4.5_f32, 1.0_f32) - 3.5_f32).abs() < 1e-6);
//! ```

use std::{fmt::Display, ops::Sub};

/// Numeric contract for coordinate values used by [`crate::nodes::trait_decl::coordinates_node::CoordinatesNode`].
pub trait CoordinateDatatype:
    Copy + Display + std::fmt::Debug + PartialOrd + PartialEq + Sub<Output = Self>
{
    /// Returns the absolute value of this coordinate scalar.
    fn abs(&self) -> Self;

    /// Converts this value into an `f32` representation.
    fn to_f32(&self) -> f32;

    /// Creates a coordinate value from `f32`.
    fn from_f32(value: f32) -> Self;
}

impl CoordinateDatatype for f32 {
    fn abs(&self) -> Self {
        f32::abs(*self)
    }

    fn to_f32(&self) -> f32 {
        *self
    }

    fn from_f32(value: f32) -> Self {
        value
    }
}

impl CoordinateDatatype for i32 {
    fn abs(&self) -> Self {
        i32::abs(*self)
    }

    fn to_f32(&self) -> f32 {
        *self as f32
    }

    fn from_f32(value: f32) -> Self {
        value as i32
    }
}

impl CoordinateDatatype for u8 {
    fn abs(&self) -> Self {
        *self
    }

    fn to_f32(&self) -> f32 {
        *self as f32
    }

    fn from_f32(value: f32) -> Self {
        if value.is_sign_negative() {
            0
        } else {
            value.round() as u8
        }
    }
}
