# Test Suite Guide

This directory contains integration tests that validate the project as an end user would consume it through public APIs.

## Scope

- app_config_integration.rs: command-line argument parsing and defaults.
- file_input_integration.rs: file-based graph parsing for directed, undirected, and two-dimensional graph formats.
- graphs_integration.rs: directed and undirected graph insertion and traversal behavior.
- dijkstra_integration.rs: shortest path correctness and expected error scenarios.
- two_dimensional_node_integration.rs: coordinate node parsing and parse error behavior.

## Local execution

Run the same checks used by CI:

1. cargo fmt --all -- --check
2. cargo clippy --all-targets --all-features -- -D warnings
3. cargo test --workspace --all-targets --locked
4. cargo test --workspace --doc --locked

## Test design principles used

- Integration-first approach: tests call public APIs to reduce coupling to private implementation details.
- Realistic failure paths: malformed input files and missing-node scenarios are covered.
- Explicit comments and assertion intent: each test explains why a scenario matters.
- Deterministic inputs: temporary files are created at runtime to avoid global fixture mutation.
