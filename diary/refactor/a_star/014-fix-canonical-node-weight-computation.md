# 014 — Fix Canonical Node Weight Computation

**Date**: 2026-05-03
**Tool**: GitHub Copilot
**Model**: Claude Sonnet 4
**Iterations**: 1

## Prompt

**2026-05-03 00:25**

Apply changes based on PR review feedback (comment_id 3177410736):
`TwoDimensionalCoordinateGraph::insert_edge` computed edge weight from the caller-supplied
`from`/`to` arguments rather than from the canonical nodes stored in `self.nodes`. A caller
could pass nodes with matching IDs but different coordinates, silently creating an edge whose
weight disagreed with the stored endpoints. Fix: after resolving indices, look up the canonical
nodes via `self.nodes[node_one_index]` and `self.nodes[node_two_index]` and pass those to
`calculate_weight`.
