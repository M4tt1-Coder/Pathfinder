//! Utility functions for graph algorithms and data structures.
//!
//! This module provides various helper functions to facilitate operations on graphs,
//! such as initializing cost maps, heuristic calculations, path reconstructions, and more.
//!
//! Functions included are designed to support algorithms like A*, Dijkstra, and other graph traversals.
//!
//! # Dependencies
//! - Rust standard library: `std::collections::HashMap`
//! - Custom traits and types from your crate (`Graph`, `GraphNode`, `NumericDatatype`, etc.)
//!
//! # Usage
//! Import the functions as needed and call them with your graph structures and parameters.
//! These utilities help streamline common graph operations and improve code modularity.

use std::collections::HashMap;

use crate::{
    algorithms::a_star_algorithm::a_star::{AStarExecutionError, AStarQueueElement},
    graphs::graph::{Graph, GraphNode},
    nodes::trait_decl::{coordinates_node::CoordinatesNode, numeric_datatype::NumericDatatype},
};

/// Prepares the initial G-cost map for all nodes in the graph.
///
/// Sets the start node's G-cost to zero and all others to the maximum value.
///
/// # Arguments
/// - `graph`: Reference to the graph containing nodes.
/// - `start_node_id`: The ID of the start node as a string slice.
///
/// # Returns
/// - A `HashMap` mapping node IDs (`String`) to their G-cost (`ND`).
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

/// Determines the total path and cost from a sequence of visited nodes in an A* search.
///
/// This function reconstructs the path from the goal node back to the start node by
/// following the chain of predecessors stored in each node. It also calculates the
/// total cost (distance) of this path.
///
/// # Arguments
/// - `visited_nodes`: A vector of `AStarQueueElement` representing the nodes visited during the search,
///   typically the contents of the closed set or the sequence of expanded nodes.
///
/// # Returns
/// - `Ok((path, total_cost))`: A tuple containing the vector of nodes representing the path from start to goal,
///   and the total cost of this path.
/// - `Err(AStarExecutionError)`: If a predecessor is not found in the visited nodes during reconstruction,
///   indicating an inconsistency in the search data.
///
/// # Type Parameters
/// - `ND`: A type that implements `NumericDatatype`, representing the cost type (e.g., `f64`, `i32`).
/// - `N`: A type that implements `CoordinatesNode<CoordinateType = ND>`, representing nodes in the graph.
///
/// # Example
/// ```
/// let (path, cost) = determine_path_cost(visited_nodes).unwrap();
/// ```
pub fn determine_path_cost<ND: NumericDatatype, N: CoordinatesNode<CoordinateType = ND>>(
    visited_nodes: Vec<AStarQueueElement<ND, N>>,
) -> Result<(Vec<N>, ND), AStarExecutionError> {
    let mut path: Vec<N> = Vec::new();
    let mut distance = ND::zero();
    if let Some(visited_node) = visited_nodes.last() {
        let mut current_node = visited_node;
        distance = current_node.get_g_cost();
        path.push(current_node.get_node().clone());
        while let Some(predecessor) = current_node.get_predecessor() {
            path.push(predecessor.clone());
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
        path.reverse();
    }

    Ok((path, distance))
}
