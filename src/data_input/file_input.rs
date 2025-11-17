// ```
// A-B:7
// B-C:3
// A-C:15
// B-D:2
// C-D:4
// ```
// for a graph like:
// ```
//              (w:2)
//           -------- D
//           /         |
//   (w:7)  /   (w:3)  | (w:4)
//  A ----- B -------- C
//   \________________/
//      (w:15)
// ```
// for directed graph the notation is slightly different:
// ```
// A->B:7
// ```

use std::{error::Error, fmt::Display, fs, path::Path};

use regex::Regex;

use crate::graphs::{
    directed::{DirectedEdge, DirectedGraph},
    graph::{Graph, Node},
    undirected::{UndirectedEdge, UndirectedGraph},
};

// ----- Implementation of the 'GraphDeterminationResult' struct -----

/// That's a result of the reading process and generation process to get a graph from a pre-defined
/// file.
///
/// Only one graph is allowed!
///
/// # Fields
///
/// -> 'directed_graph' -> Optional directed graph
/// -> 'undirected_graph' -> Optional undirected_graph
#[derive(Debug)]
pub struct FileInputGraphResult {
    pub directed_graph: Option<DirectedGraph>,
    pub undirected_graph: Option<UndirectedGraph>,
}

impl FileInputGraphResult {
    /// Prepares a result of the conversion process containing only ONE graph.
    ///
    /// # Arguments
    ///
    /// - 'directed_graph' -> A directed graph with one-way edges.
    /// - 'undirected_graph' -> Thats a new graph will only weighted edges.
    ///
    /// # Results
    ///
    /// => A 'FileInputGraphResult' with only ONE graph allowed
    pub fn new(
        directed_graph: Option<DirectedGraph>,
        undirected_graph: Option<UndirectedGraph>,
    ) -> Option<Self> {
        // cant be both -> only one graph is allowed
        if directed_graph.is_some() && undirected_graph.is_some() {
            return None;
        }
        Some(Self {
            directed_graph,
            undirected_graph,
        })
    }
}

// ----- Implementation of the 'InvalidDataInputError' struct -----

/// Custom error for the file input reading process to create a graph.
///
/// # Fields
///
/// - 'message' -> The actual error message.
#[derive(Clone, Debug)]
pub struct InvalidDataInputError {
    /// Descriptive message on what happend in the I/O process
    message: String,
}

impl InvalidDataInputError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for InvalidDataInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InvalidDataInputError {}

/// Reads a file and converts its content to a graph.
///
/// # Arguments
///
/// - 'file_path' -> Relative path to the file which the user specified.
///
/// # Returns
///
/// => Ok((Option<DirectedGraph>, Option<UndirectedGraph>)) if all process to generate the graph
/// ended without any issues.
pub fn retrieve_graph_data_from_file(
    file_path: &str,
) -> Result<FileInputGraphResult, Box<dyn Error>> {
    // create relative file path like "../example.txt"
    let rel_path = Path::new(file_path);

    let file_content = match fs::read_to_string(rel_path) {
        Ok(contents) => contents,
        Err(err) => return Err(Box::new(err)),
    };

    let res = match generate_graph_from_file(file_content) {
        Ok(graphs) => graphs,
        Err(err) => return Err(Box::new(err)),
    };

    Ok(res)
}

/// Validates if a line in file from which a graph should be generated has the right syntax.
///
/// # Arguments
///
/// - 'line' -> A single line in a file.
///
/// # Returns
///
/// => TRUE, if there is '-', ':' in correct order & the two nodes have names.
fn validate_line_syntax(line: &str) -> bool {
    // extra validation for the directed graph
    let reg_exps = vec![
        r"[A-Za-z0-9]+->[A-Za-z0-9]+:[0-9]+",
        r"[A-Za-z0-9]+-[A-Za-z0-9]+:[0-9]+",
    ];
    for exp in reg_exps {
        let reg = Regex::new(exp).unwrap();
        if reg.is_match(line) {
            return true;
        }
    }
    false
}

/// Generates both nodes and weight to create an edge.
///
/// # Arguments
///
/// - 'line' -> The string line to use to create the data objects.
/// - 'directed' -> TRUE, when the syntax is that from a directed graph.
///
/// A pre-condition is that the line has alraedy been validated.
///
/// # Returns
///
/// => Tuble which holds both nodes and the edge weight.
fn convert_line_to_graph_data(line: &str, directed: bool) -> Option<(Node, Node, u16)> {
    // first split by '-' OR '->'
    let first_split_results: Vec<&str> = if directed {
        line.split("->").collect()
    } else {
        line.split('-').collect()
    };

    let first_node = Node::new(first_split_results[0].to_string());

    let second_split_results: Vec<&str> = first_split_results[1].split(':').collect();

    let second_node = Node::new(second_split_results[0].to_string());

    let weight: u16 = match second_split_results[1].parse() {
        Ok(w) => w,
        Err(_) => return None,
    };

    Some((first_node, second_node, weight))
}

/// Makes sure the first line had the right syntax and convertes the provided data depending on
/// the syntax ('->' OR '-') to a directed or undirected graph.
///
/// # Arguments
///
/// - 'first_line' -> First line of the provided file.
///
/// # Returns
///
/// => Ok((Option<DirectedGraph>, Option<UndirectedGraph>)) when a graph could be generated.
fn determine_graph_from_first_line(
    first_line: &str,
) -> Result<FileInputGraphResult, InvalidDataInputError> {
    // validate that the line has a valid format
    if !validate_line_syntax(first_line) {
        return Err(
            InvalidDataInputError::new("The first line of the input file is in a wrong format! Please use these formats: (directed) 'A->B:4' OR (undirected) A-B:46".to_string()),
        );
    }
    if first_line.contains("->") {
        let mut directed_graph = DirectedGraph::default();

        let (from, to, weight) =
            match convert_line_to_graph_data(first_line, directed_graph.is_directed()) {
                Some(edge_data) => edge_data,
                None => {
                    return Err(InvalidDataInputError::new(format!(
                        "Couldn't convert the first line {} to valid edge data!",
                        first_line
                    )));
                }
            };

        // create all nodes and edges
        let edge = DirectedEdge::new(from.clone(), to.clone(), weight);

        // insert the first node
        directed_graph.insert_node(from);

        // insert the second node
        directed_graph.insert_node(to);

        // add the edge
        if let Some(err) = directed_graph.insert_edge(edge) {
            return Err(InvalidDataInputError::new(err.message));
        }

        Ok(
            match FileInputGraphResult::new(Some(directed_graph), None) {
                Some(result) => result,
                None => {
                    return Err(InvalidDataInputError::new(
                        "There can't be TWO graphs be generated at the same time!".to_string(),
                    ));
                }
            },
        )
    } else if first_line.contains('-') {
        let mut undirected_graph = UndirectedGraph::default();

        let (from, to, weight) =
            match convert_line_to_graph_data(first_line, undirected_graph.is_directed()) {
                Some(edge_data) => edge_data,
                None => {
                    return Err(InvalidDataInputError::new(format!(
                        "Couldn't convert the first line {} to valid edge data!",
                        first_line
                    )));
                }
            };

        // create all nodes and edges
        let edge = UndirectedEdge::new(from.clone(), to.clone(), weight);

        // insert the first node
        undirected_graph.insert_node(from);

        // insert the second node
        undirected_graph.insert_node(to);

        if let Some(err) = undirected_graph.insert_edge(edge) {
            return Err(InvalidDataInputError::new(err.message));
        }

        Ok(
            match FileInputGraphResult::new(None, Some(undirected_graph)) {
                Some(result) => result,
                None => {
                    return Err(InvalidDataInputError::new(
                        "There can't be TWO graphs be generated at the same time!".to_string(),
                    ));
                }
            },
        )
    } else {
        Err(InvalidDataInputError::new(
            "Couldn't convert the first line to a valid edge of a graph because of an unknown reason!".to_string(),
        ))
    }
}

/// The graph is determined according to the syntax used in the input file. Atleast one line
/// needs to be provided.
///
/// # Arguments
///
/// - 'lines' -> All lines provided by the file the user specified.
///
/// # Returns
///  
/// => Ok((Option<DirectedGraph>, Option<UndirectedGraph>)) when the graph could successfully be
/// created.
fn generate_graph_from_file(lines: String) -> Result<FileInputGraphResult, InvalidDataInputError> {
    let mut lines_iter = lines.lines();

    // there must be atleast one line to create a graph
    let first_line = match lines_iter.next() {
        Some(line) => line,
        None => {
            return Err(InvalidDataInputError::new(
                "The specified file is empty!".to_string(),
            ));
        }
    };

    let graph_result = determine_graph_from_first_line(first_line)?;
    if let Some(mut graph) = graph_result.directed_graph {
        for line in lines_iter {
            if line.is_empty() {
                continue;
            }

            if !validate_line_syntax(line) {
                return Err(InvalidDataInputError::new(format!(
                    "Invalid line syntax on the line {}! Please use only 'A->B:2' or 'A-B:5' to stay consistent!",
                    line
                )));
            }

            // generate edge data and add it to the graph
            let (from, to, weight) = match convert_line_to_graph_data(line, graph.is_directed()) {
                Some(data) => data,
                None => {
                    return Err(InvalidDataInputError::new(format!(
                        "Couldn't convert line '{}' to valid graph data!",
                        line
                    )));
                }
            };

            // create all nodes and edges
            let edge = DirectedEdge::new(from.clone(), to.clone(), weight);

            // in the case the edge already exists -> both nodes also need to exists already
            // then dont add the edge
            if graph.does_edge_already_exist(&edge) {
                continue;
            }

            // insert the first node
            graph.insert_node(from);

            // insert the second node
            graph.insert_node(to);

            if let Some(err) = graph.insert_edge(edge) {
                return Err(InvalidDataInputError::new(err.message));
            }
        }
        Ok(match FileInputGraphResult::new(Some(graph), None) {
            Some(result) => result,
            None => {
                return Err(InvalidDataInputError::new(
                    "There can only be two graphs at the same time!".to_string(),
                ));
            }
        })
    } else if let Some(mut graph) = graph_result.undirected_graph {
        for line in lines_iter {
            if line.is_empty() {
                continue;
            }
            if !validate_line_syntax(line) {
                return Err(InvalidDataInputError::new(format!(
                    "Invalid line syntax on the line {}! Please use only 'A->B:2' or 'A-B:5' to stay consistent!",
                    line
                )));
            }

            let (from, to, weight) = match convert_line_to_graph_data(line, graph.is_directed()) {
                Some(edge_data) => edge_data,
                None => {
                    return Err(InvalidDataInputError::new(format!(
                        "Couldn't convert the first line {} to valid edge data!",
                        first_line
                    )));
                }
            };

            // create all nodes and edges
            let edge = UndirectedEdge::new(from.clone(), to.clone(), weight);

            if graph.does_edge_already_exist(&edge) {
                continue;
            }

            // insert the first node
            graph.insert_node(from);

            // insert the second node
            graph.insert_node(to);

            if let Some(err) = graph.insert_edge(edge) {
                return Err(InvalidDataInputError::new(err.message));
            }
        }

        Ok(match FileInputGraphResult::new(None, Some(graph)) {
            Some(result) => result,
            None => {
                return Err(InvalidDataInputError::new(
                    "There can only be two graphs at the same time!".to_string(),
                ));
            }
        })
    } else {
        Err(InvalidDataInputError::new(
            "Unexpected error whilem attempting to generate the graph!".to_string(),
        ))
    }
}
