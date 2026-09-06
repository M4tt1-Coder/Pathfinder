//! Internal helper utilities used by the A* implementation.
//!
//! # Overview
//!
//! This module currently provides two core helpers:
//! - [`prepare_g_cost_map`]: initializes the per-node $g(n)$ map.
//! - [`determine_path_cost`]: reconstructs path and total cost from visited
//!   queue elements after search completion.
//!
//! # Intended Scope
//!
//! These helpers are public for testability, but they are intended for internal use.
//! They support path reconstruction and cost initialization and are not considered
//! part of the crate's stable high-level API surface.
//!
//! # Notes
//!
//! - `prepare_g_cost_map` uses `NumericDatatype::max_value()` as the initial
//!   "infinite" sentinel for all nodes except the start.
//! - `determine_path_cost` expects the destination to be the final entry in the
//!   visited list.

use std::collections::HashMap;

use crate::{
    algorithms::{NumericDatatype, a_star_algorithm::a_star::AStarQueueElement},
    error::algorithm_error::a_star_error::PathReconstructionError,
    graph::{Graph, GraphNode},
    nodes::trait_decl::CoordinatesNode,
};

/// Prepares the initial `g(n)` map for A* processing.
///
/// # Behavior
///
/// - The node with ID `start_node_id` receives `ND::zero()`.
/// - Every other node receives `ND::max_value()`.
///
/// # Parameters
///
/// - `graph`: graph whose nodes should be initialized.
/// - `start_node_id`: start-node identifier.
///
/// # Returns
///
/// Map from node ID to initialized g-cost.
///
/// # Notes
///
/// `ND::max_value()` is treated as an "infinite" placeholder. Ensure the
/// numeric datatype uses a large finite value so later comparisons behave as
/// expected.
pub fn prepare_g_cost_map<ND: NumericDatatype, G: Graph<Weight = ND>>(
    graph: &G,
    start_node_id: &str,
) -> HashMap<String, ND> {
    let mut g_cost_map: HashMap<String, ND> = HashMap::new();

    // Iterate over all nodes in the graph
    for node in graph.get_all_nodes() {
        let node_id = node.get_id().to_string();

        if node_id == start_node_id {
            // Set G-cost of start node to zero
            g_cost_map.insert(node_id, ND::zero());
        } else {
            // Set G-cost of other nodes to maximum value
            g_cost_map.insert(node_id, ND::max_value());
        }
    }

    g_cost_map
}

/// Reconstructs path and total cost from visited A* queue elements.
///
/// # Reconstruction Model
///
/// The function expects the destination node to be the last entry in
/// `visited_nodes`. It then follows predecessor references backwards until the
/// current entry has no predecessor.
///
/// # Parameters
///
/// - `visited_nodes`: expanded queue elements, typically the closed set in
///   visit order.
///
/// # Returns
///
/// - `Ok((path, total_cost))` with nodes ordered start -> destination.
///
/// # Errors
///
/// - `Err(PathReconstructionError::EmptyClosedSet)` if no nodes were visited.
/// - `Err(PathReconstructionError::MissingClosedEntry)` if a predecessor chain
///   cannot be resolved from the visited set.
///
/// # Notes
///
/// The predecessor chain is followed until no predecessor is found, so the
/// visited list must include every node referenced by `predecessor` fields.
pub fn determine_path_cost<WD: NumericDatatype, N: CoordinatesNode>(
    visited_nodes: Vec<AStarQueueElement<WD, N>>,
) -> Result<(Vec<N>, WD), PathReconstructionError> {
    let mut path: Vec<N> = Vec::new();
    let mut distance = WD::zero();
    if visited_nodes.is_empty() {
        return Err(PathReconstructionError::EmptyClosedSet);
    }

    if let Some(visited_node) = visited_nodes.last() {
        // The final closed-set entry is expected to be the destination node.
        let mut current_node = visited_node;
        distance = current_node.get_g_cost();
        path.push(current_node.get_node().clone());

        // Walk predecessor links backwards until the start node is reached.
        while let Some(predecessor) = current_node.get_predecessor() {
            path.push(predecessor.clone());

            // Resolve predecessor metadata from the closed queue so the next
            // predecessor hop can be followed.
            current_node = match visited_nodes.iter().find(|e| e.get_node() == predecessor) {
                Some(element) => element,
                None => {
                    return Err(PathReconstructionError::MissingClosedEntry {
                        node_id: predecessor.get_id().to_string(),
                    });
                }
            }
        }

        // Reconstruction collected nodes from goal -> start, so reverse it.
        path.reverse();
    }

    Ok((path, distance))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        algorithms::a_star_algorithm::a_star::AStarQueueElement,
        graph::TwoDimensionalCoordinateGraph, nodes::TwoDimensionalNode,
    };

    #[test]
    fn prepare_g_cost_map_assigns_zero_to_start_and_max_to_other_nodes() {
        let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
        let b = TwoDimensionalNode::new(2, 0, "B".to_string()).unwrap();
        let graph = TwoDimensionalCoordinateGraph::new(vec![a.clone(), b]);

        let g_costs = prepare_g_cost_map(&graph, "A");

        assert_eq!(g_costs["A"], 0.0_f32);
        assert_eq!(g_costs["B"], f32::MAX);
    }

    #[test]
    fn determine_path_cost_reconstructs_the_full_path() {
        let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
        let b = TwoDimensionalNode::new(1, 0, "B".to_string()).unwrap();

        let visited = vec![
            AStarQueueElement::new(&a, 0_i32, 0_i32, None),
            AStarQueueElement::new(&b, 5_i32, 0_i32, Some(&a)),
        ];

        let (path, cost) = determine_path_cost(visited).unwrap();

        assert_eq!(cost, 5_i32);
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].get_id(), "A");
        assert_eq!(path[1].get_id(), "B");
    }
}
