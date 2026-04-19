//! Integration tests for two-dimensional node parsing and shared parser errors.
//!
//! This validates input boundary behavior for coordinate-based node values.

use std::collections::HashSet;
use std::str::FromStr;

use shortest_path_finder::{
    error::parse_error::ParseError,
    graphs::graph::GraphNode,
    nodes::{
        trait_decl::coordinates_node::CoordinatesNode, two_dimensional_node::TwoDimensionalNode,
    },
};

#[test]
fn two_dimensional_node_parses_valid_coordinate_string() {
    let node = TwoDimensionalNode::<i32>::from_str("CityA:12,34").expect("parsing should succeed");

    assert_eq!(node.get_id(), "CityA");
    assert_eq!(node.get_x(), 12);
    assert_eq!(node.get_y(), 34);
}

#[test]
fn two_dimensional_node_requires_colon_separator() {
    let err = TwoDimensionalNode::<i32>::from_str("CityA-12,34").expect_err("colon is required");

    assert_eq!(err, ParseError::MissingColon);
}

#[test]
fn two_dimensional_node_requires_two_coordinates() {
    let err =
        TwoDimensionalNode::<i32>::from_str("CityA:12").expect_err("two coordinates are required");

    assert_eq!(err, ParseError::InvalidCoordinates);
}

#[test]
fn two_dimensional_node_rejects_non_integer_coordinates() {
    let err = TwoDimensionalNode::<i32>::from_str("CityA:12,abc")
        .expect_err("coordinates must be integer");

    assert_eq!(err, ParseError::InvalidInteger);
}

#[test]
fn parse_error_display_includes_custom_invalid_data_message() {
    let err = ParseError::InvalidDataInput("custom message".to_string());

    assert_eq!(err.to_string(), "custom message");
}

#[test]
fn two_dimensional_node_supports_f32_coordinates() {
    let node = TwoDimensionalNode::<f32>::new(1.5, -2.25, "FloatNode".to_string())
        .expect("node construction should succeed");

    assert_eq!(node.get_id(), "FloatNode");
    assert_eq!(node.get_x(), 1.5);
    assert_eq!(node.get_y(), -2.25);
}

#[test]
fn two_dimensional_node_equality_is_id_based() {
    let a = TwoDimensionalNode::<f32>::new(1.0, 2.0, "SameId".to_string())
        .expect("node A construction should succeed");
    let b = TwoDimensionalNode::<f32>::new(100.0, -42.0, "SameId".to_string())
        .expect("node B construction should succeed");

    assert_eq!(a, b);

    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    assert_eq!(set.len(), 1);
}

#[test]
fn two_dimensional_node_parses_f32_coordinates_when_requested() {
    let node = "F:1.25,2.5"
        .parse::<TwoDimensionalNode<f32>>()
        .expect("f32 parsing should succeed");

    assert_eq!(node.get_x(), 1.25);
    assert_eq!(node.get_y(), 2.5);
}
