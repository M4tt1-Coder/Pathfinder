//! Implementations for numeric helper traits used by algorithms.
//!
//! # Overview
//!
//! This namespace contains concrete trait implementations that extend primitive
//! numeric types with functionality required by coordinate heuristics.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::weight_types::numeric_datatype::NumericDatatype;
//!
//! let value = 10_i32;
//! assert_eq!(value.abs(), 10);
//! ```

pub mod impl_numeric_datatypes;
