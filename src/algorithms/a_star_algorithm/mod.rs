//! A* algorithm module and its helper utilities.
//!
//! # Overview
//!
//! This namespace contains:
//! - [`a_star`]: public A* implementation and related types.
//! - `utils`: internal helper functions used by A* path reconstruction and
//!   cost bookkeeping.
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStarSearchResult;
//! use shortest_path_finder::algorithms::algorithm::SearchResult;
//! use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
//!
//! let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
//! let b = TwoDimensionalNode::new(1, 1, "B".to_string()).unwrap();
//! let result = AStarSearchResult::new(2_i32, vec![a, b]).unwrap();
//! assert_eq!(result.get_path().len(), 2);
//! ```

pub mod a_star;
mod utils;
