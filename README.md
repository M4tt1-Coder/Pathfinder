<div align="center">
	<h1>PathFinder</h1>
	<p><strong>Rust shortest-path library and CLI for weighted graphs.</strong></p>
	<p>
		<a href="https://github.com/M4tt1-Coder/Pathfinder/actions/workflows/rust.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/M4tt1-Coder/Pathfinder/rust.yml?branch=main&label=CI&style=flat-square"></a>
		<a href="https://crates.io/crates/shortest_path_finder"><img alt="Crates.io" src="https://img.shields.io/crates/v/shortest_path_finder?style=flat-square"></a>
		<a href="https://docs.rs/shortest_path_finder"><img alt="Docs.rs" src="https://img.shields.io/docsrs/shortest_path_finder?style=flat-square"></a>
		<a href="LICENSE"><img alt="License" src="https://img.shields.io/github/license/M4tt1-Coder/Pathfinder?style=flat-square"></a>
		<a href="https://doc.rust-lang.org/edition-guide/rust-2024/"><img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.85%2B-blue?style=flat-square"></a>
	</p>
	<p>
		<strong>
			<span style="color:#1565C0">Library</span> |
			<span style="color:#2E7D32">CLI</span> |
			<span style="color:#EF6C00">Dijkstra + A*</span> |
			<span style="color:#6A1B9A">Directed / Undirected / 2D</span>
		</strong>
	</p>
	<p>
		<a href="https://docs.rs/shortest_path_finder">Docs</a> |
		<a href="https://crates.io/crates/shortest_path_finder">Crate</a> |
		<a href="#cli-usage">CLI usage</a> |
		<a href="#benchmarking">Benchmarks</a> |
		<a href="#challenges-and-roadmap">Roadmap</a>
	</p>
</div>

## At a glance

| <span style="color:#EF6C00"><strong>Algorithms</strong></span> | <span style="color:#1565C0"><strong>Graphs</strong></span> | <span style="color:#2E7D32"><strong>Input</strong></span> | <span style="color:#6A1B9A"><strong>Runtime</strong></span> |
| -------------------------------------------------------------- | ---------------------------------------------------------- | --------------------------------------------------------- | ----------------------------------------------------------- |
| Dijkstra, A\*                                                  | Directed, Undirected, 2D coordinate                        | File (header + edges)                                     | CLI + library                                               |

## Contents

- [Quickstart](#quickstart)
- [Overview](#overview)
- [Runtime status](#runtime-status)
- [Library usage (Rust)](#library-usage-rust)
- [Input format](#input-format)
- [Challenges and roadmap](#challenges-and-roadmap)
- [Development workflow](#development-workflow)
- [Advanced details](#advanced-details)

---

## Quickstart

> <strong><span style="color:#1B5E20">Start here</span></strong>: Build the release binary and run the CLI with a graph file.

**Quick checklist**

- <span style="color:#1565C0"><strong>Install</strong></span>: Rust toolchain from https://rust-lang.org/tools/install
- <span style="color:#2E7D32"><strong>Build</strong></span>: `cargo build --release`
- <span style="color:#EF6C00"><strong>Run</strong></span>: `./target/release/pathfinder --graph-file graph.txt --start A --end B`

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

### CLI usage

#### Syntax

```text
pathfinder [--help] [--version] [--origin <file|cmd-line>] [--graph-file <path_to_file>] [--algo <algorithm_name>] --start <node> --end <node>
```

> <strong><span style="color:#1565C0">Compatibility</span></strong>: Input origin now reads from `--origin` when present. For backward compatibility, `--algo file` and `--algo cmd-line` are still accepted as origin markers when `--origin` is absent.

> <strong><span style="color:#2E7D32">Value escape</span></strong>: Use `--` between a flag and its value to allow values that start with `--`.

The CLI parser rejects unknown flags, duplicate flags, missing flag values, invalid flag values, conflicting flags, and unexpected non-flag tokens with explicit errors.

Graph-file parsing also preserves detailed weight diagnostics for one-dimensional edges. If a weight token is malformed, the library returns `ParseError::InvalidWeight` with an inner `InvalidWeightError` so callers can tell whether the failure came from non-numeric input, overflow, or another numeric parsing problem.

```rust
use shortest_path_finder::error::parse_error::InvalidWeightError;
use shortest_path_finder::error::ParseError;

fn classify_weight_error(err: &ParseError) -> &'static str {
	match err {
		ParseError::InvalidWeight(InvalidWeightError::NonNumeric(_)) => "weight token was not numeric",
		ParseError::InvalidWeight(InvalidWeightError::OutOfRange(_)) => "weight token was out of range",
		ParseError::InvalidWeight(InvalidWeightError::CannotBeNegative(_)) => "negative weights are unsupported",
		ParseError::InvalidWeight(InvalidWeightError::UnknownReason(_)) => "weight parser reported an unknown failure",
		_ => "different parse error",
	}
}
```

### CLI argument examples

Minimal example using defaults for origin and algorithm:

```sh
./target/release/pathfinder --start A --end B
```

Explicit file and algorithm example:

```sh
./target/release/pathfinder --graph-file graph.txt --algo Dijkstra --start A --end B
```

Show CLI usage:

```sh
./target/release/pathfinder --help
```

### Example output

Output shape example (values depend on input graph):

```text
Path: A -> ... -> B
Distance: <value>
```

### Default settings

| <span style="color:#1565C0"><strong>Setting</strong></span> | <span style="color:#2E7D32"><strong>Default</strong></span> |
| ----------------------------------------------------------- | ----------------------------------------------------------- |
| Input origin                                                | file                                                        |
| Graph file                                                  | graph.txt                                                   |
| Algorithm                                                   | Dijkstra                                                    |

---

## Overview

PathFinder turns structured input into graph models and computes shortest paths between node IDs.
It focuses on clean APIs, predictable behavior, and performance that scales as graphs grow.
The runtime currently supports Dijkstra for directed/undirected graphs and A\* for two-dimensional coordinate graphs.

In this repo you will find:

- The reusable crate: shortest_path_finder
- The CLI binary: pathfinder
- Benchmarks for the core modules
- CI workflows and optional pre-commit hooks

> **Naming**: PathFinder is the project, `shortest_path_finder` is the crate, and `pathfinder` is the CLI binary.

### Why PathFinder

- Strict, typed parsing with clear file-line validation errors
- Typed algorithm errors with stable classification for exit codes and telemetry
- Mixed numeric types for A\* coordinate graphs (for example `i32` coordinates with `f32` weights)
- Index-based adjacency lists for efficient neighbor lookup

## Runtime status

- <span style="color:#2E7D32"><strong>Ready</strong></span>: File-based input is implemented and wired into the CLI
- <span style="color:#EF6C00"><strong>Partial</strong></span>: Command-line graph input mode is parsed but the CLI returns a structured error (runtime support is pending)
- <span style="color:#1565C0"><strong>Parsing</strong></span>: Input origin is parsed from `--origin`, with backward-compatible fallback to legacy `--algo` origin values (`file`, `cmd-line`)
- <span style="color:#2E7D32"><strong>Ready</strong></span>: Dijkstra is fully wired in the executable
- <span style="color:#2E7D32"><strong>Ready</strong></span>: A\* is wired for two-dimensional (`TD`) graph execution in the CLI path
- <span style="color:#1565C0"><strong>Mixed types</strong></span>: A\* supports mixed numeric types where coordinates and edge/path weights differ (for example `i32` coordinates with `f32` edge weights)
- <span style="color:#1565C0"><strong>Library</strong></span>: `TwoDimensionalNode` and `TwoDimensionalCoordinateGraph` support generic coordinate datatypes in library usage (for example `i32`, `f32`, `u8`); the file-input parser still uses `i32` coordinates for `TD` graph parsing
- <span style="color:#1565C0"><strong>Performance</strong></span>: Graph implementations maintain index-based adjacency lists to reduce duplication and improve neighbor lookup efficiency

## Technologies

Core stack and dependencies:

- Rust edition 2024
- std collections for algorithm internals (for example BinaryHeap and HashMap)
- uuid for edge identifiers
- regex for line-format validation during graph parsing
- strum and strum_macros for graph-type parsing helpers
- env_logger and log for runtime logging

Quality and automation:

- Four GitHub Actions workflows:
  - `rust.yml`: formatting, clippy, build, tests, and doctests
  - `rust-ci.yml`: baseline verification on pushes and PRs to main
  - `codeql.yml`: static analysis for security scanning
  - `release.yml`: automated publishing on merged PRs into main
- Local pre-commit hooks for formatting, linting, tests, and optional cargo audit

## Project structure

- src/main.rs: CLI entrypoint and runtime wiring
- src/data_input/mod.rs: public input boundary for file and command-line origins
- src/data_input/file/mod.rs: file-input namespace re-export
- src/data_input/file/file_input.rs: graph-file parsing and validation
- src/data_input/file/cli_config/: CLI argument parsing, validation, and default values
- src/graphs/: graph trait and concrete graph types
- src/algorithms/: algorithm trait and implementations
- src/error/: layered errors for parsing, configuration, and algorithm execution
- src/lib.rs: crate-level docs and public re-exports
- benches/: benchmark targets, including direct Dijkstra vs A\* comparisons

## Library usage (Rust)

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

#### A\* on a coordinate graph

```rust
use shortest_path_finder::algorithms::a_star_algorithm::a_star::AStar;
use shortest_path_finder::algorithms::algorithm::{Algorithm, SearchResult};
use shortest_path_finder::graph::Graph;
use shortest_path_finder::graph::TwoDimensionalCoordinateGraph;
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
use shortest_path_finder::data_input::file::{
    retrieve_graph_data_from_file, FileInputGraphResult,
};

let parsed = retrieve_graph_data_from_file("test_files/directed_graph.txt")
    .expect("graph file should parse");
let FileInputGraphResult::DirectedGraph(graph) = parsed else {
    panic!("directed graph expected");
};

let dijkstra = DijkstraAlgorithm::new(graph);
let result = dijkstra.shortest_path("A", "L").expect("path should exist");

println!("distance: {}", result.get_total_distance());
```

## Input format

The current parser format (used by the provided test files) is header plus edge lines:

> <strong><span style="color:#C62828">Strict format</span></strong>: The parser is line-validated and surfaces file-line context for invalid input.

- Blank lines and whitespace-only lines are ignored.
- Line 1 is a graph-type header and must be exactly one of: `D`, `UN`, or `TD`.
- Only lines after line 1 are converted into edges.
- Line 1 is not inserted as an edge.
- Whitespace-only lines after the header are ignored.
- Parse errors include file-line context, graph-type-specific expected syntax, and weight-specific diagnostics when an edge weight cannot be parsed.

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

## Challenges and roadmap

Main engineering challenges addressed so far:

- Designing graph abstractions that support multiple graph models
- Keeping algorithm interfaces generic while preserving practical runtime ergonomics
- Validating strict, typed parsing from textual graph definitions

Planned and in-progress features:

- [x] Finalize full A\* runtime integration
- [ ] Enable command-line graph input origin in executable flow
- [x] Extend usage examples and integration tests for all graph variants

---

## Dev workflow

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

## Benchmarking

Run the algorithm benchmark target to compare all currently implemented runtime
algorithms (Dijkstra and A\*) on shared benchmark scenarios:

```sh
cargo bench --bench pathfinder
```

The benchmark includes:

- Shared coordinate-graph construction cost
- Dijkstra vs A\* instance creation cost on the same graph model
- Dijkstra vs A\* shortest-path runtime on sparse grids
- Dijkstra vs A\* shortest-path runtime on denser grids with diagonal shortcuts

---

## Advanced details

<details>
  <summary><strong>Exit codes</strong></summary>

- 0: success
- 1: setup, parsing, or graph-loading failure
- 2: invalid graph for the selected algorithm (for example unweighted)
- 3: required node is missing from the graph
- 4: invalid edge weight encountered
- 5: heuristic produced an invalid value
- 6: no path exists between the requested nodes
- 7: algorithm bookkeeping invariant failed
- 8: algorithm returned an invalid result

Exit codes for algorithm failures are derived from `AlgorithmErrorKind` in
the library error module. The CLI wraps algorithm-specific errors into
`AlgorithmError`, calls `kind()`, and exits with `kind().exit_code()`.

Example: mapping a Dijkstra error to a CLI exit code:

```rust
use shortest_path_finder::algorithms::dijkstra::DijkstraError;
use shortest_path_finder::error::algorithm_error::{AlgorithmError, AlgorithmErrorKind};

let err = AlgorithmError::from(DijkstraError::NoPathFound {
	start: "A".to_string(),
	end: "B".to_string(),
});
assert_eq!(err.kind(), AlgorithmErrorKind::NoPath);
assert_eq!(err.kind().exit_code(), 6);
```

Example CLI error output:

```text
[ERROR] Algorithm error (Dijkstra): no path found from 'A' to 'B'
```

</details>

<details>
  <summary><strong>Error handling (library)</strong></summary>

Errors are layered by boundary:

| Layer     | Type             | Responsibility                                      |
| --------- | ---------------- | --------------------------------------------------- |
| Parse     | `ParseError`     | Line-level graph syntax validation                  |
| Input     | `DataInputError` | File I/O and graph loading (`FileInputError` today) |
| Config    | `CLIParseError`  | CLI flag parsing and validation                     |
| Algorithm | `AlgorithmError` | Shortest-path execution failures                    |
| CLI       | `AppError`       | Binary wrapper with `exit_code()` mapping           |

File-input failures are wrapped at the loading boundary. [`FileInputError::Parse`]
carries the source file path alongside the underlying [`ParseError`]:

```rust
use shortest_path_finder::data_input::file::FileInputError;
use shortest_path_finder::error::{DataInputError, ParseError};

let err = DataInputError::from(FileInputError::Parse {
	file_path: "graph.txt".to_string(),
	source: ParseError::InvalidLineSyntax,
});
assert!(err.to_string().contains("File input error"));
```

Algorithms return typed errors. For stable classification in telemetry, exit
codes, or user messaging, wrap failures in `AlgorithmError` and query
`AlgorithmErrorKind`:

```rust
use shortest_path_finder::algorithms::dijkstra::DijkstraError;
use shortest_path_finder::error::algorithm_error::{AlgorithmError, AlgorithmErrorKind};

let err = AlgorithmError::from(DijkstraError::MissingStartNode {
	id: "A".to_string(),
	graph: "DirectedGraph".to_string(),
});

match err.kind() {
	AlgorithmErrorKind::MissingNode => println!("node is missing"),
	_ => println!("other error"),
}
```

The CLI collapses configuration, input, and algorithm failures into `AppError`:

```rust
use shortest_path_finder::error::{AppError, CLIParseError};

let err = AppError::from(CLIParseError::MissingRequiredFlag { flag: "--start" });
assert_eq!(err.exit_code(), 1);
```

</details>

<details>
  <summary><strong>Automated releases</strong></summary>

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

</details>

<details>
  <summary><strong>Pre-commit hook setup (optional)</strong></summary>

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

</details>

## License

This repository is licensed under the terms defined in the license file: [LICENSE](LICENSE).
Please review [LICENSE](LICENSE) for full usage, distribution, and contribution terms.
