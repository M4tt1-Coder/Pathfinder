# 044 — Address PR review feedback

**Date**: 2026-04-17
**Tool**: GitHub Copilot (Coding Agent)
**Model**: claude-sonnet-4-5
**Iterations**: 1

## Prompt

**2026-04-17 01:24**

Apply changes based on PR review feedback:

1. `src/main.rs` – update module-level docs to mention A* for TD graphs (algorithm selection note was still saying "Dijkstra only").
2. `src/graphs/two_dimensional_coordinate_graph.rs` – remove incorrect "XOR-based" comment; the weight calculation uses Euclidean distance (`.pow(2)` / `sqrt`), not XOR.
3. `src/numeric_datatypes/impl_numeric_datatypes.rs` – fix `adjust_for_heuristic()` for `i32`: the old implementation cast `HEURISTIC_ADJUSTMENT_FACTOR as i32` which truncated to `0`, so it always returned `0`; now uses an `f32` intermediate with rounding.
4. `src/weight_types/mod.rs` – replace deprecated `u16::max_value()` call in doctest with fully-qualified `<u16 as GraphWeight>::max_value()`.
5. `src/weight_types/impl_weights.rs` – same deprecated `u16::max_value()` fix in doctest.
6. `Cargo.toml` – bump version from `0.2.1` to `0.3.0` to reflect breaking public API changes.
7. `src/data_input/file_input.rs` – widen 2D line-syntax regex to allow optional leading `-` for negative coordinates, consistent with `TwoDimensionalNode::from_str` which parses `i32` coordinates.
