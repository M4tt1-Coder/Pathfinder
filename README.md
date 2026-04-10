# PathFinder

PathFinder is a Rust library and CLI application for shortest-path computation on weighted graphs.
The runtime currently uses Dijkstra, with A* present in the codebase and under active integration.

## Description

PathFinder builds a graph from input data and computes the shortest path from a start node to an end node.
The project supports directed and undirected weighted graphs, plus two-dimensional node types used by A* internals.

The repository provides:

- A reusable crate: shortest_path_finder
- A CLI binary: pathfinder
- Benchmarks for core modules
- CI and pre-commit quality checks

### Current Runtime Scope

- File-based input is implemented and used by the CLI
- Command-line graph input mode is defined but not implemented in runtime flow
- Dijkstra is fully wired in the executable
- A* is available in modules but not yet enabled in the final CLI execution path

### Technologies

Core stack and dependencies:

- Rust edition 2024
- std collections for algorithm internals (for example BinaryHeap and HashMap)
- uuid for edge identifiers
- regex for line-format validation during graph parsing
- strum and strum_macros for graph-type parsing helpers
- env_logger and log for runtime logging

Quality and automation:

- GitHub Actions workflow for fmt, clippy, and tests
- Local pre-commit hooks for formatting, linting, tests, and optional cargo audit

### Project Structure

- src/main.rs: CLI entrypoint and runtime wiring
- src/cmd_line/app_config.rs: argument parsing and defaults
- src/data_input/file_input.rs: graph-file parsing and validation
- src/algorithms/: algorithm traits and implementations
- src/graphs/: graph trait and concrete graph types
- benches/: benchmark targets

### Challenges & Feature

Main engineering challenges addressed so far:

- Designing graph abstractions that support multiple graph models
- Keeping algorithm interfaces generic while preserving practical runtime ergonomics
- Validating strict, typed parsing from textual graph definitions

Planned and in-progress features:

- [ ] Finalize full A* runtime integration
- [ ] Enable command-line graph input origin in executable flow
- [ ] Extend usage examples and integration tests for all graph variants

## How to use it?

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

### CLI argument examples

Minimal example using defaults for origin and algorithm:

```sh
./target/release/pathfinder --start A --end B
```

Explicit file and algorithm example:

```sh
./target/release/pathfinder --origin file --graph-file graph.txt --algo Dijkstra --start A --end B
```

### Input file format

The current parser expects graph-type prefixes in each line and applies a strict flow:

- Line 1 is used to detect graph type (`D`, `UN`, `TD`) and validate syntax.
- Only lines after line 1 are converted into edges.
- This means line 1 must be a valid prefixed edge line (for example `DA->B:7`), not only `D` or
	`UN`, and it is not inserted as an edge.

Directed example:

```text
DA->B:7
DA->B:7
DB->C:3
DC->D:5
```

In this example, the first line detects type and is ignored for insertion, while the second line
adds the first actual edge.

Undirected example:

```text
UNA-B:7
UNA-B:7
UNB-C:3
UNC-D:5
```

Two-dimensional format currently recognized by parser:

```text
TDA:0,0-B:2,1
TDA:0,0-B:2,1
TDB:2,1-C:4,1
```

### Development workflow

Run checks locally before pushing:

Example command for formatting:

```sh
cargo fmt --all -- --check
```

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
