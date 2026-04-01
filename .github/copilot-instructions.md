# Copilot Coding Agent Instructions for `M4tt1-Coder/Pathfinder`

## Repository snapshot
- Language: Rust (`edition = "2024"`).
- Crate type: library + binary (`src/lib.rs`, `src/main.rs`).
- Core purpose: parse weighted graph input and run shortest-path algorithms (currently Dijkstra is wired in runtime flow).

## High-value file map
- `src/main.rs`: CLI entrypoint; parses args into `AppConfig`, loads graph from file, runs algorithm.
- `src/cmd_line/app_config.rs`: argument parsing and defaults (`--graph-file`, `--start`, `--end`, `--algo`, `--origin`).
- `src/data_input/file_input.rs`: graph file parsing/validation (`A-B:7` or `A->B:7` formats).
- `src/graphs/*`: graph traits and directed/undirected graph implementations.
- `src/algorithms/dijkstra.rs`: shortest path implementation.
- `.github/workflows/rust.yml`: CI uses `cargo build --verbose` and `cargo test --verbose`.

## How to work efficiently in this repo
1. Start with compile/test validation:
   - `cargo build --verbose`
   - `cargo test --verbose`
2. Keep changes minimal and module-local:
   - CLI/config changes: touch `src/main.rs` + `src/cmd_line/app_config.rs`.
   - Parsing/input changes: touch `src/data_input/file_input.rs`.
   - Algorithm changes: touch `src/algorithms/*` and only required graph trait/impl pieces.
3. Validate with the same CI commands before finishing.

## Important behavior and caveats
- Input file syntax is strict:
  - Directed edge line: `A->B:7`
  - Undirected edge line: `A-B:7`
- Graph type is inferred from the **first line** of the file; subsequent lines must stay consistent.
- Runtime currently supports file input path in `main`; `InputOrigin::CommandLine` is still `unimplemented!()`.
- In `AppConfig::retrieve_data_input`, the parser currently checks `--algo` instead of `--origin`; treat this as existing behavior unless your task explicitly targets CLI parsing fixes.

## Tests/quality expectations
- Existing CI checks only build and test via Cargo.
- There are benchmark targets under `benches/`, but they are not part of standard CI gating.
- No dedicated lint workflow is configured in GitHub Actions.

## Errors encountered during onboarding and workarounds used
- While running `cargo test --verbose` in this environment, tool output exceeded inline display limits and was redirected to a temp log file (`/tmp/copilot-tool-output-...txt`).
- Workaround: rely on process exit code and, when needed, inspect the saved temp file in chunks.

