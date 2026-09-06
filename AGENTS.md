# Agent Instructions — Pathfinder

Read this file **before starting any task** in this repository. It applies to all AI coding agents (Cursor, GitHub Copilot, Claude Code, etc.).

Source: adapted from [`.github/copilot-instructions.md`](.github/copilot-instructions.md), kept in sync with the current codebase.

---

## Repository snapshot

|                 |                                                        |
| --------------- | ------------------------------------------------------ |
| **Language**    | Rust (`edition = "2024"`, MSRV 1.85+)                  |
| **Crate**       | `shortest_path_finder` — library + `pathfinder` binary |
| **Purpose**     | Parse weighted graphs and run shortest-path algorithms |
| **Algorithms**  | Dijkstra, A* (coordinate-based)                        |
| **Graph types** | Directed, undirected, 2D coordinate                    |

---

## High-value file map

| Area                           | Path                                                                                                                           |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| CLI entrypoint                 | `src/main.rs`                                                                                                                  |
| Argument parsing / defaults    | `src/data_input/file/cli_config/config.rs`                                                                                     |
| Graph file parsing             | `src/data_input/file/file_input.rs`                                                                                            |
| Graph traits & implementations | `src/graph/*`                                                                                                                  |
| Algorithm trait & selection    | `src/algorithms/algorithm.rs`                                                                                                  |
| Dijkstra                       | `src/algorithms/dijkstra_algorithm.rs`                                                                                         |
| A*                             | `src/algorithms/a_star_algorithm/`                                                                                             |
| Error types                    | `src/error/*`                                                                                                                  |
| Node models                    | `src/nodes/*`                                                                                                                  |
| Benchmarks                     | `benches/` (not CI-gated)                                                                                                      |
| CI workflows                   | `.github/workflows/rust.yml`, `.github/workflows/rust-ci.yml`, `.github/workflows/codeql.yml`, `.github/workflows/release.yml` |
| User docs                      | `README.md`                                                                                                                    |
| AI diary                       | `diary/` — see [`diary/README.md`](diary/README.md)                                                                            |

---

## How to work efficiently

1. **Start narrow** — identify which module owns the change (CLI, parsing, graphs, algorithms, errors) and touch only what is required.
2. **Validate before finishing** — run the same checks as CI (see below).
3. **Keep changes minimal** — match existing naming, types, and patterns in the surrounding code.
4. **Sync docs in the same change** — Rust doc comments, `README.md`, and diary entries are part of the deliverable, not optional polish.

### Module-local change guide

| Change type     | Primary files                                              |
| --------------- | ---------------------------------------------------------- |
| CLI / config    | `src/main.rs`, `src/data_input/file/cli_config/config.rs` |
| Input / parsing | `src/data_input/file/file_input.rs`                        |
| Algorithms      | `src/algorithms/*` + only required graph trait/impl pieces |
| Graph structure | `src/graph/*`, `src/nodes/*`                               |
| Errors          | `src/error/*`                                              |

---

## Validation (match CI)

Run from the repository root before finishing:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --workspace --all-targets --locked --verbose
cargo test --workspace --all-targets --locked --verbose
cargo test --workspace --doc --locked --verbose
```

Benchmarks exist under `benches/` but are not part of standard CI gating.

---

## Important behavior and caveats

### Graph input format (strict)

- **Directed edge:** `A->B:7`
- **Undirected edge:** `A-B:7`
- Graph type is inferred from the **first line** of the file; subsequent lines must stay consistent.

### Runtime limitations

- File input is the supported runtime path in `main`.
- `InputOrigin::CommandLine` is parsed, but the CLI runtime returns a structured unsupported-origin error for it.
- In `AppConfig::retrieve_data_input`, `--origin` takes precedence and legacy `--algo file|cmd-line` values are still accepted as fallback.

---

## Documentation requirements

### Rust doc comments

Whenever Rust code is modified:

- Update `///` and `//!` doc comments so they match the implementation.
- Cover intent, parameters, return values, errors, and examples where relevant.
- New source files need file-level `//!` module docs suitable for `cargo doc` / docs.rs.
- If a change does not require a doc update, **verify** existing docs already match the new behavior.

### README synchronization

Before finalizing any change, check whether `README.md` needs updating:

- CLI usage, flags, or defaults
- Input format or examples
- Supported algorithms or graph types
- Project structure, dev workflow, or testing instructions

Update `README.md` in the same change when behavior or documented usage changes.

---

## AI diary (required for AI-assisted work)

This repository tracks AI-assisted contributions transparently. Follow [`diary/README.md`](diary/README.md).

For every user prompt that produces or modifies code:

1. Create a diary entry under `diary/<branch_name>/` (e.g. `diary/feature/a_star/`).
2. Name files `NNN-short-title.md` (three-digit incrementing id).
3. Use the entry template from `diary/README.md`:
   - `# NNN — Short Title`
   - `**Date**`, `**Tool**`, `**Model**`, `**Iterations**`
   - `## Prompt` with timestamped prompt text (include follow-ups with their own timestamps).

Include the diary entry in one of the commits for that prompt (preferably the final commit).

---

## Commit guidelines

- **One prompt → one or more focused commits** for the resulting changes.
- **Separate unrelated changes** into distinct commits; do not batch unrelated work.
- Every commit needs a concise title and a brief body summarizing the contents.
- Diary commits use the `[diary] NNN — Short description` convention (see `diary/README.md`).
- **Only create commits when the user asks**, unless the user or project workflow explicitly requests autonomous commits.

---

## Code style principles

1. **Minimize scope** — smallest correct diff; avoid unrelated changes.
2. **Avoid over-engineering** — no premature abstractions or excessive error handling for impossible edges.
3. **Follow existing conventions** — read surrounding code before writing; match naming, types, and patterns.
4. **Comments sparingly** — code should be self-explanatory; comment only non-obvious logic.
5. **Tests when meaningful** — add tests when they cover real behavior; skip trivial assertions.

---

## Quick reference

```sh
# Build release binary
cargo build --release

# Run CLI
./target/release/pathfinder --graph-file graph.txt --start A --end B

# Full validation (CI-equivalent)
cargo fmt --all -- --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --workspace --all-targets --locked && \
cargo test --workspace --doc --locked
```
