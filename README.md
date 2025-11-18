<!-- TODO: Udjust the README.md for GitHub -->

# PathFinder

Contains implementations of various shortest path algorithms like Dijkstra, etc.! You can calculate the best route from one point 'A' to point 'B'.

## Description

Basically, the executable later generates a graph based on data provided by the user through a file or manual in the terminal (not implemented yet). Then using a specified algorithm the shortest path is being determined.

Some graphs only work with specific algorithms and visa verca!

### Technologies

To implement the data structures needed for the execution, I primarily used types from the **std** module. For the implementation of the **_Dijkstra_** algorithm I used the `std::collections::{BinaryHeap, HashMap}` structs for example.

In some cases I need to identify objects, where I made use of the **uuid** crate.

```rust
    // for example in edges
    pub struct DirectedEdge {
        // ...
        id: uuid::Uuid,
    }
```

### Challenges & Feature

During the process, I faced a few challenges, which is normal.

- Creating suited **_generic data structures_** (e.g. for multiple graphs implementing the same behaviour) BUT it's never perfect ;D
- ...

In the future, I will add the following features to the crate ...

- [ ] **A\*** algorithm
- [ ] data input through the **command line**

## How to use it?

You need to be in the _root-directory_: `~/your/path/to/pathfinder`!

To actual run the binary executable, you need to compile the code in release mode!

Make sure you have **Rust** installed! To set it up please go [here ...](https://rust-lang.org/tools/install/)

Check if you have **Rust** installed with:

```sh
    cargo -V
    # expected output: "cargo 1.19.1 (ea2d97820 2025-10-10)"

```

Now, run this command in a terminal in the _root-directory_:

```sh
    cargo build --release
```

... this will create a `target` folder with the _executable_ in the `target/release` folder.

For running the `binary`, type this into the terminal!

```sh
    ./target/release/pathfinder --start A --end B
```

The syntax for the usage of the executable is:

```
    pathfinder [ --origin <file / cmd-line> --graph-file <path_to_file> --algo <algorithm_name>] --start <node> --end <node>
```

You can expect this output if you provided a valid query and data!

```sh
    ./target/release/pathfind --start A --end B

    Path: -> A -> B,
    Distance: 5

```

### Default Settings

For running the app you have to know some default settings which are applied if the user doesn't mutate them!

- the **origin of the data input** is the a file with the name `graph.txt` in the _root-directory_
- ... corresponing with that default name of the file mentioned is the same
- if no algorith was specified then the **_Dijkstra_** will be used
