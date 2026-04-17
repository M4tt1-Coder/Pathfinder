//! Weight and numeric trait definitions used by graph algorithms.
//!
//! # Overview
//!
//! - [`numeric_datatype`]: arithmetic trait extending graph-weight semantics.
//! - [`impl_weights`]: concrete graph-weight implementations and runtime enum.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::graphs::graph::GraphWeight;
//!
//! assert_eq!(u16::zero(), 0);
//! assert!(<u16 as GraphWeight>::max_value() > 1000);
//! ```

pub mod impl_weights;
pub mod numeric_datatype;
