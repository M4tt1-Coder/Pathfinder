//! This module provides implementations of the `NumericDatatype` trait for a range of fundamental
//! numeric types, including floating-point types (such as `f32`, `f64`) and integer types (such as `i32`, `u32`).
//! The primary goal of these implementations is to enable these types to be used interchangeably in
//! generic algorithms and data structures that require common numeric operations, such as absolute value,
//! addition, subtraction, multiplication, and division.
//!
//! By adhering to the `NumericDatatype` trait, these data types can be seamlessly integrated into
//! mathematical computations, numerical analysis, and other contexts where a uniform interface for
//! numeric types is beneficial. This design promotes code abstraction, reusability, and flexibility,
//! allowing algorithms to operate over any supported numeric type without concern for their specific
//! underlying representation.
//!
//! This module, therefore, serves as a foundational component for building generic, type-agnostic
//! numerical functionalities within the broader codebase.use crate::nodes::trait_decl::numeric_datatype::NumericDatatype;

use crate::weight_types::numeric_datatype::NumericDatatype;

/// A small constant factor used to adjust values for heuristic calculations, if necessary.
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
