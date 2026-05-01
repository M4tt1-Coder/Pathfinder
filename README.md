# PathFinder

PathFinder is a Rust library and CLI application for shortest-path computation on weighted graphs.
I build it with an eye for clean APIs, predictable behavior, and performance that scales as your graphs grow.
The runtime currently supports Dijkstra for directed/undirected graphs and A* for two-dimensional coordinate graphs.

## Description

PathFinder turns structured input into graph models and computes shortest paths between node IDs.
The library exposes directed and undirected weighted graphs plus coordinate-aware nodes and graphs used by A*.

In this repo you will find:

- The reusable crate: shortest_path_finder
- The CLI binary: pathfinder
- Benchmarks for the core modules
- CI workflows and optional pre-commit hooks

### Current Runtime Scope

- File-based input is implemented and wired into the CLI
- Command-line graph input mode exists in configuration but is not wired into the runtime flow yet
- Input origin is parsed from `--origin`, with backward-compatible fallback to legacy `--algo` origin values (`file`, `cmd-line`)
- Dijkstra is fully wired in the executable
- A* is wired for two-dimensional (`TD`) graph execution in the CLI path
- A* supports mixed numeric types where coordinates and edge/path weights differ (for example `i32` coordinates with `f32` edge weights)
- `TwoDimensionalNode` and `TwoDimensionalCoordinateGraph` support generic coordinate datatypes in library usage (for example `i32`, `f32`, `u8`); the file-input parser still uses `i32` coordinates for `TD` graph parsing
- Graph implementations maintain index-based adjacency lists to reduce duplication and improve neighbor lookup efficiency

### Technologies

Core stack and dependencies:

- Rust edition 2024
- std collections for algorithm internals (for example BinaryHeap and HashMap)
- uuid for edge identifiers
- regex for line-format validation during graph parsing
- strum and strum_macros for graph-type parsing helpers
- env_logger and log for runtime logging

Quality and automation:

- Three GitHub Actions workflows:
	- Rust CI checks (fmt, clippy, tests, docs)
	- Rust baseline verification on pushes and PRs to main
	- Automated release publishing on merged PRs into main
- Local pre-commit hooks for formatting, linting, tests, and optional cargo audit

### Project Structure

- src/main.rs: CLI entrypoint and runtime wiring
- src/cmd_line/app_config.rs: argument parsing and defaults
- src/data_input/file_input.rs: graph-file parsing and validation
- src/algorithms/: algorithm traits and implementations
- src/graphs/: graph trait and concrete graph types
- benches/: benchmark targets, including direct Dijkstra vs A* comparisons

### Library Usage (Rust)

If you use the crate directly, the flow is simple: build a graph, pick an algorithm, and read the `SearchResult`.
The snippets below are intentionally compact but mirror how I use the library in real code.

#### Dijkstra on a directed graph

```rust
use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
use shortest_path_finder::graphs::directed::DirectedGraph;
use shortest_path_finder::graphs::graph::Graph;
use shortest_path_finder::nodes::default_node::DefaultNode;

let mut graph = DirectedGraph::default();
let a = DefaultNode::new("A".to_string());
let b = DefaultNode::new("B".to_string());
let c = DefaultNode::new("C".to_string());

graph.insert_node(a.clone());
graph.insert_node(b.clone());
graph.insert_node(c.clone());

graph.insert_edge(&a, &b, Some(4));
graph.insert_edge(&b, &c, Some(2));
graph.insert_edge(&a, &c, Some(10));

let dijkstra = DijkstraAlgorithm::new(graph);
let result = dijkstra.shortest_path("A", "C").expect("path should exist");

assert_eq!(result.get_total_distance(), 6);
assert_eq!(result.get_path().len(), 3);
```

Swap `DirectedGraph` for `UndirectedGraph` when you want a non-directional graph with the same API.

#### A* on a coordinate graph

```rust
use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStar;
use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
use shortest_path_finder::graphs::graph::Graph;
use shortest_path_finder::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph;
use shortest_path_finder::nodes::two_dimensional_node::TwoDimensionalNode;

let a = TwoDimensionalNode::new(0, 0, "A".to_string()).unwrap();
let b = TwoDimensionalNode::new(2, 1, "B".to_string()).unwrap();
let c = TwoDimensionalNode::new(4, 1, "C".to_string()).unwrap();

let mut graph = TwoDimensionalCoordinateGraph::new(vec![a.clone(), b.clone(), c.clone()]);
graph.insert_edge(&a, &b, None);
graph.insert_edge(&b, &c, None);

let a_star = AStar::new(graph);
let result = a_star.shortest_path("A", "C").expect("path should exist");

println!("distance: {}", result.get_total_distance());
```

#### Parse a graph from a file and run Dijkstra

```rust
use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
use shortest_path_finder::algorithms::dijkstra::DijkstraAlgorithm;
use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;

let parsed = retrieve_graph_data_from_file("test_files/directed_graph.txt")
    .expect("graph file should parse");
let graph = parsed.directed_graph.expect("directed graph expected");

let dijkstra = DijkstraAlgorithm::new(graph);
let result = dijkstra.shortest_path("A", "L").expect("path should exist");

println!("distance: {}", result.get_total_distance());
```

### Challenges and roadmap

Main engineering challenges addressed so far:

- Designing graph abstractions that support multiple graph models
- Keeping algorithm interfaces generic while preserving practical runtime ergonomics
- Validating strict, typed parsing from textual graph definitions

Planned and in-progress features:

- [X] Finalize full A* runtime integration
- [ ] Enable command-line graph input origin in executable flow
- [ ] Extend usage examples and integration tests for all graph variants

## Getting started

### Prerequisites

Install Rust from https://rust-lang.org/tools/install and verify your setup.

Example command:

```sh
cargo -V
```

Expected style of output example:

```text
cargo 1.xx.x (........ 2026-..-..)
```

### Build

From the repository root, build the release binary.

Example command:

```sh
cargo build --release
```

### Run the binary

Example command:

```sh
./target/release/pathfinder --graph-file graph.txt --start A --end B
```

### CLI syntax

```text
pathfinder [--origin <file|cmd-line>] [--graph-file <path_to_file>] [--algo <algorithm_name>] --start <node> --end <node>
```

Compatibility note:

- Input origin now reads from `--origin` when present.
- For backward compatibility, `--algo file` and `--algo cmd-line` are still accepted as origin markers when `--origin` is absent.
- The CLI parser now rejects unknown flags, duplicate flags, missing flag values, and unexpected non-flag tokens with explicit errors.

### CLI argument examples

Minimal example using defaults for origin and algorithm:

```sh
./target/release/pathfinder --start A --end B
```

Explicit file and algorithm example:

```sh
./target/release/pathfinder --graph-file graph.txt --algo Dijkstra --start A --end B
```

### Input file format

The current parser format (used by the provided test files) is header plus edge lines:

- Line 1 is a graph-type header and must be exactly one of: `D`, `UN`, or `TD`.
- Only lines after line 1 are converted into edges.
- Line 1 is not inserted as an edge.
- Whitespace-only lines after the header are ignored.
- Parse errors include file-line context and graph-type-specific expected syntax.

Directed example:

```text
D
A->B:7
B->C:3
C->D:5
```

Undirected example:

```text
UN
A-B:7
B-C:3
C-D:5
```

Two-dimensional format currently recognized by parser:

```text
TD
A:0,0=>B:2,1
B:2,1=>C:4,1
```

### Development workflow

Run checks locally before pushing:

CI-parity commands:

```sh
cargo fmt --all -- --check
```

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

```sh
cargo build --workspace --all-targets --locked --verbose
```

```sh
cargo test --workspace --all-targets --locked --verbose
```

```sh
cargo test --workspace --doc --locked --verbose
```

### Benchmarking

Run the algorithm benchmark target to compare all currently implemented runtime
algorithms (Dijkstra and A*) on shared benchmark scenarios:

```sh
cargo bench --bench pathfinder
```

The benchmark includes:

- Shared coordinate-graph construction cost
- Dijkstra vs A* instance creation cost on the same graph model
- Dijkstra vs A* shortest-path runtime on sparse grids
- Dijkstra vs A* shortest-path runtime on denser grids with diagonal shortcuts

### Automated releases

When a pull request is merged into `main`, the release workflow (`.github/workflows/release.yml`) runs and:

- Reads `package.version` from `Cargo.toml`
- Fails with an explicit error if the corresponding release tag already exists
- Fails with an explicit error if `package.version` is not greater than the latest `v*` release tag
- Publishes the crate to crates.io
- Creates a GitHub release using tag `v<package.version>`

Release authentication requirement:

- Configure crates.io trusted publishing for this repository so GitHub Actions can mint a short-lived publish token via OIDC

Important release rule:

- Always bump `version` in `Cargo.toml` before merging a release-worthy PR into `main`

Pre-commit hook setup (optional):

Example command for clippy:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

Example command for tests:

```sh
cargo test --all-features
```

If you use pre-commit in your environment, install and run hooks.

Example command to install hooks:

```sh
pre-commit install
```

Example command to run hooks manually:

```sh
pre-commit run --all-files
```

### Default Settings

Current defaults from CLI configuration:

- Input origin defaults to file
- Graph file defaults to graph.txt
- Algorithm defaults to Dijkstra

### Example output

Output shape example (values depend on input graph):

```text
Path: A -> ... -> B
Distance: <value>
```

## License

This repository is licensed under the terms defined in the license file: [LICENSE](LICENSE).
Please review [LICENSE](LICENSE) for full usage, distribution, and contribution terms.
