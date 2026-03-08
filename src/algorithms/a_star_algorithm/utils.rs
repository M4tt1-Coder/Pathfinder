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
    graphs::graph::{Graph, GraphNode},
    nodes::trait_decl::numeric_datatype::NumericDatatype,
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
