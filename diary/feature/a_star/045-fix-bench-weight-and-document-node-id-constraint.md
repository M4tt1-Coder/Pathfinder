# 045 — Fix BenchWeight PartialEq and document node ID constraint

**Date**: 2026-04-17
**Tool**: GitHub Copilot (Coding Agent)
**Model**: claude-sonnet-4-5
**Iterations**: 1

## Prompt

**2026-04-17 01:31**

Apply changes based on PR review feedback:

1. `benches/pathfinder.rs` – `BenchWeight` wraps `f32` but derived `PartialEq` (NaN != NaN semantics) while `Ord`/`Eq` are implemented via `total_cmp`, breaking the `Eq`/`Ord` contract for NaN values. Replace the derived `PartialEq` with a manual implementation comparing `to_bits()` so that `PartialEq`, `Eq`, and `Ord` are all consistent with `total_cmp`.

2. `src/data_input/file_input.rs` – The file-input regexes restrict node IDs to `[A-Za-z0-9]+` but this constraint is not documented. Users may assume IDs like `Station-42` or `node_1` work in files and hit confusing parse errors. Add a `# Node ID constraint` section to the `compile_line_syntax_regexes` doc comment explaining that node IDs must consist of letters and digits only.
