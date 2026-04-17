//! Benchmarks for shortest-path algorithm execution.
//!
//! # Overview
//!
//! This benchmark target focuses on algorithm-level, scenario-based comparisons.
//! It benchmarks Dijkstra and A* on the same graph model and the same
//! start/end query pairs.
//!
//! Benchmark groups:
//! - construction of a shared coordinate benchmark graph,
//! - algorithm construction cost,
//! - shortest-path comparison on a sparse grid,
//! - shortest-path comparison on denser grids with diagonal shortcuts.
//!
//! # Run
//!
//! ```text
//! cargo bench --bench pathfinder
//! ```

use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    hint::black_box,
    ops::{Add, Div, Mul, Sub},
};

use divan::{Bencher, bench};
use shortest_path_finder::{
    algorithms::{
        a_star_algorithm::a_star::AStar,
        algorithm::{Algorithm, SearchResult},
        dijkstra::DijkstraAlgorithm,
    },
    graphs::graph::{Graph, GraphEdge, GraphNode, GraphWeight},
    nodes::two_dimensional_node::TwoDimensionalNode,
    weight_types::numeric_datatype::NumericDatatype,
};
use uuid::Uuid;

fn main() {
    divan::main();
}

const START_NODE_ID: &str = "0_0";
const STRAIGHT_EDGE_WEIGHT: BenchWeight = BenchWeight(1.0);
const DIAGONAL_EDGE_WEIGHT: BenchWeight = BenchWeight(1.4);

#[derive(Clone, Copy, Debug)]
enum ComparedAlgorithm {
    Dijkstra,
    AStar,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BenchWeight(f32);

impl Eq for BenchWeight {}

impl Ord for BenchWeight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for BenchWeight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for BenchWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}

impl Add for BenchWeight {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for BenchWeight {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for BenchWeight {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for BenchWeight {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl GraphWeight for BenchWeight {
    fn max_value() -> Self {
        Self(f32::MAX)
    }

    fn zero() -> Self {
        Self(0.0)
    }
}

impl NumericDatatype for BenchWeight {
    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    fn adjust_for_heuristic(&self) -> Self {
        Self(self.0 * 0.001)
    }

    fn to_f32(&self) -> f32 {
        self.0
    }

    fn from_f32(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct BenchmarkEdge {
    id: Uuid,
    from_id: String,
    to_id: String,
    weight: BenchWeight,
}

impl BenchmarkEdge {
    fn new(from_id: String, to_id: String, weight: BenchWeight) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_id,
            to_id,
            weight,
        }
    }
}

impl GraphEdge for BenchmarkEdge {
    type ID = Uuid;

    fn get_id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Debug, Clone)]
struct BenchmarkGraphInsertionError {
    message: String,
}

impl BenchmarkGraphInsertionError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for BenchmarkGraphInsertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BenchmarkGraphInsertionError {}

#[derive(Clone, Debug, Default)]
struct BenchmarkCoordinateGraph {
    nodes: Vec<TwoDimensionalNode>,
    edges: Vec<BenchmarkEdge>,
    adjacency: HashMap<String, Vec<(usize, BenchWeight)>>,
}

impl BenchmarkCoordinateGraph {
    fn node_index_by_id(&self, node_id: &str) -> Option<usize> {
        self.nodes.iter().position(|node| node.get_id() == node_id)
    }
}

impl Graph for BenchmarkCoordinateGraph {
    type Node = TwoDimensionalNode;
    type Weight = BenchWeight;
    type Edge = BenchmarkEdge;
    type InsertionError = BenchmarkGraphInsertionError;

    fn neighbors<'a>(
        &'a self,
        u: &Self::Node,
    ) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a> {
        let Some(entries) = self.adjacency.get(u.get_id()) else {
            return Box::new(std::iter::empty());
        };

        let neighbors: Vec<(&Self::Node, Self::Weight)> = entries
            .iter()
            .map(|(target_idx, weight)| (&self.nodes[*target_idx], *weight))
            .collect();

        Box::new(neighbors.into_iter())
    }

    fn is_directed(&self) -> bool {
        false
    }

    fn insert_node(&mut self, new_node: Self::Node) {
        if self.does_node_already_exist(&new_node) {
            return;
        }

        self.adjacency
            .entry(new_node.get_id().to_string())
            .or_default();
        self.nodes.push(new_node);
    }

    fn insert_edge(&mut self, edge: Self::Edge) -> Option<Self::InsertionError> {
        if self.does_edge_already_exist(&edge) {
            return Some(BenchmarkGraphInsertionError::new(format!(
                "Edge from '{}' to '{}' already exists.",
                edge.from_id, edge.to_id
            )));
        }

        let from_idx = match self.node_index_by_id(&edge.from_id) {
            Some(index) => index,
            None => {
                return Some(BenchmarkGraphInsertionError::new(format!(
                    "Node '{}' is missing in benchmark graph.",
                    edge.from_id
                )));
            }
        };
        let to_idx = match self.node_index_by_id(&edge.to_id) {
            Some(index) => index,
            None => {
                return Some(BenchmarkGraphInsertionError::new(format!(
                    "Node '{}' is missing in benchmark graph.",
                    edge.to_id
                )));
            }
        };

        self.adjacency
            .entry(edge.from_id.clone())
            .or_default()
            .push((to_idx, edge.weight));
        self.adjacency
            .entry(edge.to_id.clone())
            .or_default()
            .push((from_idx, edge.weight));

        self.edges.push(edge);
        None
    }

    fn does_edge_already_exist(&self, edge: &Self::Edge) -> bool {
        self.edges.iter().any(|existing| {
            (existing.from_id == edge.from_id && existing.to_id == edge.to_id)
                || (existing.from_id == edge.to_id && existing.to_id == edge.from_id)
        })
    }

    fn does_node_already_exist(&self, node: &Self::Node) -> bool {
        self.nodes
            .iter()
            .any(|existing| existing.get_id() == node.get_id())
    }

    fn get_node_by_id(&self, id: &str) -> Option<&Self::Node> {
        self.nodes.iter().find(|node| node.get_id() == id)
    }

    fn get_edge_by_id(&self, id: &Uuid) -> Option<&Self::Edge> {
        self.edges.iter().find(|edge| edge.get_id() == *id)
    }

    fn get_all_nodes(&self) -> &Vec<Self::Node> {
        &self.nodes
    }

    fn is_weighted(&self) -> bool {
        true
    }

    fn abbreviation() -> String {
        "BENCH2D".to_string()
    }
}

impl Display for BenchmarkCoordinateGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BenchmarkCoordinateGraph(nodes: {}, edges: {})",
            self.nodes.len(),
            self.edges.len()
        )
    }
}

fn node_id(x: usize, y: usize) -> String {
    format!("{}_{}", x, y)
}

fn goal_node_id(grid_side: usize) -> String {
    node_id(grid_side - 1, grid_side - 1)
}

fn insert_grid_edge(
    graph: &mut BenchmarkCoordinateGraph,
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    weight: BenchWeight,
) {
    let edge = BenchmarkEdge::new(node_id(from_x, from_y), node_id(to_x, to_y), weight);

    if let Some(err) = graph.insert_edge(edge) {
        panic!("Benchmark grid edge insertion failed: {}", err);
    }
}

fn build_grid_graph(
    grid_side: usize,
    include_diagonal_shortcuts: bool,
) -> BenchmarkCoordinateGraph {
    let mut graph = BenchmarkCoordinateGraph::default();

    for y in 0..grid_side {
        for x in 0..grid_side {
            let node = TwoDimensionalNode::new(x as i32, y as i32, node_id(x, y))
                .expect("Benchmark node IDs must be non-empty.");
            graph.insert_node(node);
        }
    }

    for y in 0..grid_side {
        for x in 0..grid_side {
            if x + 1 < grid_side {
                insert_grid_edge(&mut graph, x, y, x + 1, y, STRAIGHT_EDGE_WEIGHT);
            }
            if y + 1 < grid_side {
                insert_grid_edge(&mut graph, x, y, x, y + 1, STRAIGHT_EDGE_WEIGHT);
            }
            if include_diagonal_shortcuts && x + 1 < grid_side && y + 1 < grid_side {
                insert_grid_edge(&mut graph, x, y, x + 1, y + 1, DIAGONAL_EDGE_WEIGHT);
            }
        }
    }

    graph
}

fn run_single_search(
    algorithm: ComparedAlgorithm,
    graph: &BenchmarkCoordinateGraph,
    start_node_id: &str,
    end_node_id: &str,
) -> (usize, f32) {
    match algorithm {
        ComparedAlgorithm::Dijkstra => {
            let result = DijkstraAlgorithm::new(graph.clone())
                .shortest_path(start_node_id, end_node_id)
                .expect("Dijkstra benchmark query failed unexpectedly.");
            (
                result.get_path().len(),
                result.get_total_distance().to_f32(),
            )
        }
        ComparedAlgorithm::AStar => {
            let result = AStar::new(graph.clone())
                .shortest_path(start_node_id, end_node_id)
                .expect("A* benchmark query failed unexpectedly.");
            (
                result.get_path().len(),
                result.get_total_distance().to_f32(),
            )
        }
    }
}

fn benchmark_algorithm_comparison(
    bencher: Bencher,
    algorithm: ComparedAlgorithm,
    grid_side: usize,
    include_diagonal_shortcuts: bool,
) {
    let goal_node_id = goal_node_id(grid_side);

    bencher
        .with_inputs(|| build_grid_graph(grid_side, include_diagonal_shortcuts))
        .bench_refs(|graph| {
            let output = run_single_search(algorithm, graph, START_NODE_ID, &goal_node_id);
            black_box(output);
        });
}

#[bench(args = [24, 48, 72])]
fn build_shared_coordinate_benchmark_graph(grid_side: usize) {
    let graph = build_grid_graph(grid_side, true);
    black_box(graph);
}

#[bench(args = [ComparedAlgorithm::Dijkstra, ComparedAlgorithm::AStar])]
fn create_algorithm_instance_for_shared_coordinate_graph(
    bencher: Bencher,
    algorithm: ComparedAlgorithm,
) {
    bencher
        .with_inputs(|| build_grid_graph(40, true))
        .bench_refs(|graph| match algorithm {
            ComparedAlgorithm::Dijkstra => {
                black_box(DijkstraAlgorithm::new(graph.clone()));
            }
            ComparedAlgorithm::AStar => {
                black_box(AStar::new(graph.clone()));
            }
        });
}

#[bench(args = [ComparedAlgorithm::Dijkstra, ComparedAlgorithm::AStar])]
fn compare_algorithms_on_sparse_grid(bencher: Bencher, algorithm: ComparedAlgorithm) {
    benchmark_algorithm_comparison(bencher, algorithm, 40, false);
}

#[bench(args = [ComparedAlgorithm::Dijkstra, ComparedAlgorithm::AStar])]
fn compare_algorithms_on_diagonal_grid(bencher: Bencher, algorithm: ComparedAlgorithm) {
    benchmark_algorithm_comparison(bencher, algorithm, 40, true);
}

#[bench(args = [ComparedAlgorithm::Dijkstra, ComparedAlgorithm::AStar])]
fn compare_algorithms_on_large_diagonal_grid(bencher: Bencher, algorithm: ComparedAlgorithm) {
    benchmark_algorithm_comparison(bencher, algorithm, 64, true);
}
