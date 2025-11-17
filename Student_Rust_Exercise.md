# Task : Build a small, well-tested Rust crate that provides:

- A generic graph abstraction
- Implemenation of pathfinding algorithms over that abstraction (e.g. Dijkstra, A\*). One is enough, two would be better.
- A CLI to run the algorithms on edge-list inputs (e.g. from file)
- Solid engineering (docs, tests, benchmarks, error handling).

## Expected artifact

- A single crate named `pathfinder` that includes a public library (lib) + CLI binary (bin/graphfind).
- The solution can be provided as zip file containing the workspace

## Input given

### Graph abstration to use

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

graph.txt

```
A-B:7
B-C:3
A-C:15
B-D:2
C-D:4
```

for a graph like:

```
             (w:2)
           -------- D
          /         |
  (w:7)  /   (w:3)  | (w:4)
 A ----- B -------- C
  \________________/
     (w:15)
```

for directed graph the notation is slightly different:

```
A->B:7
```

### Expected result

```
$ pathfinder --graph graph.txt -start A --end D

Path: A,B,D
Distance: 9
```
