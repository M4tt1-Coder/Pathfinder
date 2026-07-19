# Copilot Coding Agent Instructions for `M4tt1-Coder/Pathfinder`

## Repository snapshot

- Language: Rust (`edition = "2024"`).
- Crate type: library + binary (`src/lib.rs`, `src/main.rs`).
- Core purpose: parse weighted graph input and run shortest-path algorithms (Dijkstra for directed/undirected graphs, A* for two-dimensional graphs).

## High-value file map

- `src/main.rs`: CLI entrypoint; parses args into `AppConfig`, loads graph from file, runs algorithm.
- `src/cmd_line/app_config.rs`: argument parsing and defaults (`--graph-file`, `--start`, `--end`, `--algo`, `--origin`).
- `src/data_input/file_input.rs`: graph file parsing/validation (`A-B:7` or `A->B:7` formats).
- `src/graphs/*`: graph traits and directed/undirected graph implementations.
- `src/algorithms/dijkstra.rs`: shortest path implementation.
- `.github/workflows/rust.yml`: CI checks formatting, clippy, build, tests, and doctests.
- `.github/workflows/rust-ci.yml`: baseline CI verification on pushes and PRs.
- `.github/workflows/codeql.yml`: security scanning workflow.
- `.github/workflows/release.yml`: release publishing workflow.

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
   - First line header: `D`, `UN`, or `TD`
   - Directed edge line: `A->B:7`
   - Undirected edge line: `A-B:7`
   - Two-dimensional edge line: `A:0,0=>B:4,2`
- Graph type is inferred from the **first line** of the file; subsequent lines must stay consistent.
- Runtime currently supports file input in `main`; `InputOrigin::CommandLine` is parsed but returns a structured unsupported-origin error in the CLI runtime.
- In `AppConfig::retrieve_data_input`, `--origin` takes precedence and legacy `--algo file|cmd-line` values remain supported as fallback.

## Tests/quality expectations

- Existing CI checks only build and test via Cargo.
- There are benchmark targets under `benches/`, but they are not part of standard CI gating.
- No dedicated lint workflow is configured in GitHub Actions.
- Always run all test suites before committing and before finishing a task:
   - `cargo test --workspace --all-targets --locked`
   - `cargo test --workspace --doc --locked`

## Rust documentation requirements

- Whenever Rust code is modified, update the corresponding documentation in the same change.
- If modified items (functions, structs, enums, traits, modules, or behavior) have missing or outdated docs, add or correct Rust doc comments (`///` and `//!`) so they match the implementation.
- Prefer crate-appropriate Rust docs that work for `cargo doc` and communicate intent, parameters, return values, errors, and examples where relevant.
- For any newly created Rust source file, add file-level documentation suitable for crates.io rendering (typically inner module docs via `//!` at the top of the file).
- Treat documentation updates as required deliverables, not optional polish.

## Code documentation synchronization requirement

- For any prompt that changes code, update the corresponding documentation in the same change so implementation and docs stay synchronized.
- Consider both code-local and user-facing documentation as applicable to the change scope (for example: Rust doc comments, module docs, README usage notes, and behavior descriptions).
- If a code change does not require a documentation update, explicitly verify that existing documentation already matches the new behavior before finalizing.

## README synchronization requirement

- For every prompt that results in repository changes, explicitly check whether those changes affect `README.md`.
- If behavior, CLI usage, configuration, testing workflow, project structure, or documented examples have changed, update `README.md` in the same change.
- Treat this check as mandatory before finalizing work, even when changes are small.

## Errors encountered during onboarding and workarounds used

- While running `cargo test --verbose` in this environment, tool output exceeded inline display limits and was redirected to a temp log file (`/tmp/copilot-tool-output-...txt`).
- Workaround: rely on process exit code and, when needed, inspect the saved temp file in chunks.

## AI Prompt Diary requirement

- Always create a new diary entry for every user prompt handled by Copilot.
- Store entries under `diary/<branch_name>/` (for example: `diary/feature/a_star/`).
- Follow `diary/README.md` as the source of truth for diary structure and content.
- Use one file per entry named as `NNN-short-title.md` (three-digit incrementing id prefix).
- Use the exact entry template from `diary/README.md`:
   - `# NNN — Short Title`
   - `**Date**`, `**Tool**`, `**Model**`, `**Iterations**`
   - `## Prompt` with timestamped prompt text
- If a prompt has follow-up prompts, include each follow-up under `## Prompt` with its own timestamp, as specified in `diary/README.md`.

## Commit History requirement

- After each user prompt that results in repository modifications, create one or more commits containing the changes made for that prompt.
- Always separate distinct modifications into different commits to keep the history clean and reviewable.
- Keep commits granular and topic-focused; do not batch unrelated changes into one commit.
- Every commit must have a concise, descriptive title and a body description that briefly summarizes the commit contents.
