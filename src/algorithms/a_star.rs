//! # Theory for the A* algorithm
//!
//! ## Sources
//!
//! - https://www.datacamp.com/tutorial/a-star-algorithm
//! - https://www.geeksforgeeks.org/dsa/a-search-algorithm/
//!

// TODO: Add the implementation of the A* algorithm

use std::fmt::Display;

use crate::{algorithms::algorithm::Algorithm, graphs::graph::Graph};

///
#[derive(Debug)]
pub struct AStar<G: Graph + Display> {
    ///
    graph: G,
}

// ----- Implementation of the 'A_Star' struct -----

impl<G: Graph + Display> Algorithm for AStar<G> {
    fn shortest_path(
        &self,
        start: crate::graphs::graph::Node,
        end: crate::graphs::graph::Node,
    ) -> Result<super::algorithm::SearchResult, Self::ExecutionError> {
        unimplemented!()
    }
    fn execute_step() -> Option<Self::StepExecutionResult> {
        unimplemented!()
    }
    type ExecutionError = unimplemented!();
    type StepExecutionResult = unimplemented!();
}
