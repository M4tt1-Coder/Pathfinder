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
3. cargo build --workspace --all-targets --locked
4. cargo test --workspace --all-targets --locked
5. cargo test --workspace --doc --locked

## Coverage with `cargo llvm-cov`

Install the cargo subcommand once if it is not already available:

```sh
cargo install cargo-llvm-cov --locked
```

`cargo llvm-cov` instruments the Rust code, runs the selected Cargo tests, and
uses LLVM's source-based coverage tools to produce a report. The commands below
are intended to be run from the repository root.

### All tests and the CI threshold

Generate a summary for every package and target, enable all features, and fail
if total line coverage is below the project's 85% threshold:

```sh
cargo llvm-cov --workspace --all-features --all-targets \
  --summary-only --fail-under-lines 85
```

For a detailed text report, omit `--summary-only` and optionally show uncovered
lines:

```sh
cargo llvm-cov --workspace --all-features --all-targets \
  --text --show-missing-lines
```

The HTML report is written to `target/llvm-cov/html` by default. Add `--open`
to open it in a browser, or use `--output-dir` to choose another directory:

```sh
cargo llvm-cov --workspace --all-features --all-targets --html --open
cargo llvm-cov --workspace --all-features --all-targets \
  --html --output-dir target/coverage-html
```

### Selecting tests, packages, and targets

Coverage follows Cargo's target-selection flags, so a run can be narrowed
without changing the test code:

```sh
# This package's library unit tests.
cargo llvm-cov --lib

# One integration-test target (the file is tests/app_config_integration.rs).
cargo llvm-cov --test app_config_integration

# All integration-test targets, without examples or benchmarks.
cargo llvm-cov --tests

# A single package in a workspace, or all packages explicitly.
cargo llvm-cov --package shortest_path_finder
cargo llvm-cov --workspace
```

To run only selected test cases within a target, pass the normal test filter
after `--`:

```sh
cargo llvm-cov --test dijkstra_integration -- shortest_path
```

### Inspecting or filtering files

The Cargo subcommand selects test targets and packages; the coverage report
contains the source files exercised by those tests. Save a text report and
search it when investigating one source file:

```sh
cargo llvm-cov --text --output-path target/coverage.txt
rg -n -A 8 'src/algorithms/dijkstra_algorithm.rs' target/coverage.txt
```

To omit files matching a regular expression from the report, use
`--ignore-filename-regex`:

```sh
cargo llvm-cov --workspace --all-targets \
  --ignore-filename-regex '(^|/)(tests|examples|benches)/' \
  --text
```

This option excludes matching report paths; it does not prevent the selected
tests from running. For a focused source-file assessment, prefer selecting the
smallest relevant test target with `--test` and then inspect that file in the
text or HTML report.

### Exporting coverage for tooling

Use an output format and path suitable for CI or external coverage services:

```sh
cargo llvm-cov --workspace --all-targets --lcov \
  --output-path target/coverage.lcov
cargo llvm-cov --workspace --all-targets --json \
  --output-path target/coverage.json
```

Use `cargo llvm-cov --help` for the installed version's complete option list.
The upstream [`cargo-llvm-cov` README](https://github.com/taiki-e/cargo-llvm-cov/blob/main/README.md)
and [LLVM's `llvm-cov` command guide](https://llvm.org/docs/CommandGuide/llvm-cov.html)
describe the report formats and filtering behavior in more detail.

## Recommended test organization

Organize tests according to the boundary they exercise:

- Keep unit tests next to the implementation in `src/**`; larger modules can
  place their tests in dedicated child modules.
- Use `tests/*.rs` for integration tests, grouping each file around a feature,
  public API area, or user-facing workflow.
- Put reusable setup and helpers in `tests/common/`, and shared input data in
  `tests/fixtures/` when runtime-generated data is not sufficient.
- Use documentation examples as executable tests for API usage and behavior.
- Reserve dedicated harnesses or directories for specialized test types that
  need their own execution model.

Test location and test purpose are separate concerns. The former distinguishes
unit tests from integration tests; the latter identifies feature, regression,
or workflow coverage. Keeping those decisions independent makes the suite
easier to extend without adding unnecessary structure.

## Test design principles used

- Integration-first approach: tests call public APIs to reduce coupling to private implementation details.
- Realistic failure paths: malformed input files and missing-node scenarios are covered.
- Explicit comments and assertion intent: each test explains why a scenario matters.
- Deterministic inputs: temporary files are created at runtime to avoid global fixture mutation.
