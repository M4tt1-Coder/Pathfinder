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
//! Functions in this module are public for testability and composability, but
//! they are designed as algorithm-internal building blocks rather than stable
//! high-level APIs.
//!
//! # Usage Example
//!
//! ```rust
//! use shortest_path_finder::algorithms::a_star_algorithm::utils::prepare_g_cost_map;
//! use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
//! use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
//!
//! let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
//! let b = TwoDimensionalNode::new(1, 1, "B".to_string()).unwrap();
//! let graph = TwoDimensionalCoordinateGraph::new(vec![a.clone(), b]);
//! let costs = prepare_g_cost_map(&graph, "A");
//!
//! assert_eq!(costs["A"], 0.0_f32);
//! assert_eq!(costs["B"], f32::MAX);
//! ```

use std::collections::HashMap;

use crate::{
    algorithms::a_star_algorithm::a_star::{AStarExecutionError, AStarQueueElement},
    graphs::graph::{Graph, GraphNode},
    nodes::trait_decl::coordinates_node::CoordinatesNode,
    weight_types::numeric_datatype::NumericDatatype,
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
/// # Examples
///
/// ```rust
/// use shortest_path_finder::algorithms::a_star_algorithm::utils::prepare_g_cost_map;
/// use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
/// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
///
/// let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
/// let b = TwoDimensionalNode::new(2, 0, "B".to_string()).unwrap();
/// let graph = TwoDimensionalCoordinateGraph::new(vec![a.clone(), b]);
///
/// let g_costs = prepare_g_cost_map(&graph, "A");
/// assert_eq!(g_costs["A"], 0.0_f32);
/// assert_eq!(g_costs["B"], f32::MAX);
/// ```
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
/// start node is reached.
///
/// # Parameters
///
/// - `visited_nodes`: expanded queue elements, typically the closed set in
///   visit order.
///
/// # Returns
///
/// - `Ok((path, total_cost))` with nodes ordered start -> destination.
/// - `Err(AStarExecutionError)` if predecessor links are inconsistent.
///
/// # Examples
///
/// ```rust
/// use shortest_path_finder::algorithms::a_star_algorithm::{
///     a_star::AStarQueueElement,
///     utils::determine_path_cost,
/// };
/// use shortest_path_finder::graphs::graph::GraphNode;
/// use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;
///
/// let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
/// let b = TwoDimensionalNode::new(1, 0, "B".to_string()).unwrap();
///
/// let visited = vec![
///     AStarQueueElement::new(&a, 0_i32, 0_i32, None),
///     AStarQueueElement::new(&b, 5_i32, 0_i32, Some(&a)),
/// ];
///
/// let (path, cost) = determine_path_cost(visited).unwrap();
/// assert_eq!(cost, 5_i32);
/// assert_eq!(path.len(), 2);
/// assert_eq!(path[0].get_id(), "A");
/// assert_eq!(path[1].get_id(), "B");
/// ```
pub fn determine_path_cost<WD: NumericDatatype, N: CoordinatesNode>(
    visited_nodes: Vec<AStarQueueElement<WD, N>>,
) -> Result<(Vec<N>, WD), AStarExecutionError> {
    let mut path: Vec<N> = Vec::new();
    let mut distance = WD::zero();
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
                    return Err(AStarExecutionError::new(
                        "Predecessor not found in closed queue during path reconstruction."
                            .to_string(),
                    ));
                }
            }
        }

        // Reconstruction collected nodes from goal -> start, so reverse it.
        path.reverse();
    }

    Ok((path, distance))
}
