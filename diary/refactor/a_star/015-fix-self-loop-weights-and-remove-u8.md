# 015 — Fix Self-Loop Weights and Remove Unsound u8 CoordinateDatatype

**Date**: 2026-05-03
**Tool**: GitHub Copilot
**Model**: Claude Sonnet 4
**Iterations**: 1

## Prompt

**2026-05-03 00:30**

Apply changes based on three PR review comments:

1. (comment 3177410742) `CoordinateDatatype`'s `u8` implementation is unsound: `a - b`
   underflows before `abs()` runs whenever `a < b`. Remove the `u8` impl and update docs
   that reference it.

2. (comment 3177410745) `DirectedGraph::insert_edge` forces self-loop weight to `0`,
   preventing callers from representing positive-cost loops like `A -> A : 9`. Move
   weight extraction before the self-loop branch so the caller-provided weight is used.

3. (comment 3177410748) `UndirectedGraph::insert_edge` similarly drops the caller
   weight for self-loops and stores `0`. Use the caller-provided weight instead.
