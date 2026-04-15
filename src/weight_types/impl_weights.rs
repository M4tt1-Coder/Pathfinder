//! Graph-weight runtime enum and primitive `GraphWeight` implementations.
//!
//! # Overview
//!
//! This module contributes two things:
//! - [`WeightType`], an enum used by parsing code when weight types differ by
//!   graph format.
//! - Implementations of [`GraphWeight`](crate::graphs::graph::GraphWeight)
//!   for `u16`, `f32`, and `i32`.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::graph::GraphWeight;
//! use shortest_path_finder::weight_types::impl_weights::WeightType;
//!
//! let weight = WeightType::U16(7);
//! assert!(matches!(weight, WeightType::U16(7)));
//!
//! assert_eq!(u16::zero(), 0);
//! assert!(u16::max_value() > 1_000);
//! ```

use crate::graphs::graph::GraphWeight;

/// An enum representing possible types for edge weights in a graph.
///
/// This allows for dynamic handling of different numeric types as weights,
/// enabling algorithms to work with graphs whose edge weights may be of
/// varying types at runtime.
///
/// # Variants
///
/// - `U16(u16)`: Represents an unsigned 16-bit integer weight.
/// - `F32(f32)`: Represents a 32-bit floating point weight.
/// - `I32(i32)`: Represents a signed 32-bit integer weight.
pub enum WeightType {
    /// Represents an unsigned 16-bit integer weight.
    U16(u16),
    /// Represents a 32-bit floating point weight.
    F32(f32),
    /// Represents a signed 32-bit integer weight.
    I32(i32),
    /// Indicates that no weight is necessary for the graph (e.g., unweighted graphs).
    NotNecessary,
}

/// Implements the `GraphWeight` trait for `u16`.
impl GraphWeight for u16 {
    /// Returns the additive identity for `u16` weights (0).
    fn zero() -> Self {
        0
    }

    /// Returns the maximum possible value for `u16` weights.
    fn max_value() -> Self {
        u16::MAX
    }
}

/// Implements the `GraphWeight` trait for `f32`.
impl GraphWeight for f32 {
    /// Returns the additive identity for `f32` weights (0.0).
    fn zero() -> Self {
        0.
    }

    /// Returns the maximum possible value for `f32` weights.
    fn max_value() -> Self {
        f32::MAX
    }
}

/// Implements the `GraphWeight` trait for `i32`.
impl GraphWeight for i32 {
    /// Returns the additive identity for `i32` weights (0).
    fn zero() -> Self {
        0
    }

    /// Returns the maximum possible value for `i32` weights.
    fn max_value() -> Self {
        i32::MAX
    }
}
