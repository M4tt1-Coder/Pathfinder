//! Traits describing specialized node capabilities.
//!
//! # Overview
//!
//! This namespace currently exposes [`coordinates_node`], a trait for graph
//! nodes that carry x/y coordinates.
//!
//! # Usage
//!
//! ```rust
//! use std::fmt::{Display, Formatter};
//! use shortest_path_finder::graphs::graph::GraphNode;
//! use shortest_path_finder::nodes::trait_decl::coordinates_node::CoordinatesNode;
//!
//! #[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
//! struct N { id: String, x: i32, y: i32 }
//!
//! impl Display for N {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "{}", self.id)
//!     }
//! }
//!
//! impl GraphNode for N {
//!     fn get_id(&self) -> &str { &self.id }
//! }
//!
//! impl CoordinatesNode for N {
//!     type CoordinateType = i32;
//!     fn get_x(&self) -> i32 { self.x }
//!     fn get_y(&self) -> i32 { self.y }
//! }
//!
//! let node = N { id: "P1".to_string(), x: 1, y: 2 };
//! assert_eq!(node.get_x(), 1);
//! ```

pub mod coordinates_node;
