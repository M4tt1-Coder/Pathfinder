# Task : Build a small, well-tested Rust crate that provides:

- A generic graph abstraction
- Implementation of pathfinding algorithms over that abstraction (for example Dijkstra and A\*).
- A CLI to run the algorithms on file-based graph inputs.
- Solid engineering (docs, tests, benchmarks, error handling).

## Expected artifact

- A single crate named `shortest_path_finder` that includes a public library (lib) + CLI binary (`pathfinder`).
- The solution can be provided as a zip file containing the workspace.

## Input given

### Graph abstraction to use

```
pub trait Graph {
    type Node: Eq + std::hash::Hash + Clone;
    type Weight: Copy + PartialOrd + std::ops::Add<Output = Self::Weight>;

    // Iterate the neighbors of the given node
    fn neighbors<'a>(&'a self, u: &Self::Node) -> Box<dyn Iterator<Item = (&'a Self::Node, Self::Weight)> + 'a>;
    // Indicate if this graph is a directed one
    fn is_directed(&self) -> bool;
}
```

you can modify the trait to your needs (e.g. use super-traits), but it is important to keep the abstration.

### Graph file format

The current parser expects a one-line header followed by edge lines.

For a directed graph:

```text
D
A->B:7
B->C:3
A->C:15
B->D:2
C->D:4
```

For an undirected graph:

```text
UN
A-B:7
B-C:3
A-C:15
B-D:2
C-D:4
```

For a two-dimensional coordinate graph:

```text
TD
A:0,0=>B:4,2
B:4,2=>C:8,3
```

### Expected result

```text
$ pathfinder --graph-file graph.txt --start A --end D

Path: A -> B -> D
Distance: 9
```
