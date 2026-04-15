//! File-based graph parsing for the Pathfinder application.
//!
//! # Overview
//!
//! This module reads text files and converts them into one concrete graph representation:
//! - [`DirectedGraph`] for directed, weighted edges,
//! - [`UndirectedGraph`] for undirected, weighted edges,
//! - [`TwoDimensionalCoordinateGraph`] for two-dimensional coordinate edges.
//!
//! The public entrypoint is [`retrieve_graph_data_from_file`].
//!
//! # Input Format
//!
//! The parser infers graph type from the first line and expects all following non-empty lines to
//! use the same graph encoding. In the current implementation and test files, the first line is a
//! graph-type marker:
//! - `D` for directed graph input,
//! - `UN` for undirected graph input,
//! - `TD` for two-dimensional graph input.
//!
//! Important current behavior:
//! - The first line is used only for graph-type detection.
//! - Graph edges are built from lines after the first line.
//! - Therefore, the first line is a header marker and is not parsed as an edge.
//!
//! ## Supported edge patterns
//!
//! ```text
//! Header line:  D | UN | TD
//! Directed:     <from>-><to>:<weight>         (example: A->B:7)
//! Undirected:   <from>-<to>:<weight>          (example: A-B:7)
//! 2D edge line: <from>:x,y-<to>:x,y           (example: A:0,0-B:4,2)
//! ```
//!
//! # Validation and consistency rules
//!
//! - The file must contain at least one line.
//! - The first line must identify a supported graph type.
//! - Every remaining parsed line must match one of the supported edge syntaxes.
//! - A file can produce exactly one graph variant.
//! - Duplicate edges are ignored during insertion.
//! - The first line is consumed for type detection and is not inserted as an edge.
//! - Two-dimensional file input is parsed and inserted into
//!   [`TwoDimensionalCoordinateGraph`] in [`generate_graph_from_file`].
//!
//! # Usage Example
//!
//! ```no_run
//! use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;
//!
//! let parsed = retrieve_graph_data_from_file("test_files/directed_graph.txt");
//! assert!(parsed.is_ok());
//! ```
//!
//! The example is marked as `no_run` because it depends on repository-local files at runtime.

use std::{error::Error, fs, path::Path, str::FromStr};

use regex::Regex;
use strum_macros::EnumString;

use crate::{
    error::parse_error::ParseError,
    graphs::{
        directed::{DirectedEdge, DirectedGraph},
        graph::Graph,
        two_dimensional_coordinate_graph::{TwoDimensionalCoordinateGraph, TwoDimensionalEdge},
        undirected::{UndirectedEdge, UndirectedGraph},
    },
    nodes::{
        default_node::DefaultNode, node_types::NodeType, two_dimensional_node::TwoDimensionalNode,
    },
    weight_types::impl_weights::WeightType,
};

// ----- Implementation of the 'FoundGraphType' enum -----

/// Enumerates graph kinds that can be detected while parsing file input.
///
/// # Purpose
///
/// [`FoundGraphType`] acts as a normalized discriminator after format detection. It allows parser
/// functions to branch into graph-type specific parsing without relying on raw string checks.
///
/// # Derive behavior
///
/// The enum derives `EnumString` from `strum`, enabling case-insensitive string parsing for the
/// configured aliases on each variant.
#[derive(EnumString, PartialEq)]
enum FoundGraphType {
    /// Undirected graph input.
    ///
    /// Accepts aliases `Undirected` and `UN` (case-insensitive).
    #[strum(serialize = "Undirected", serialize = "UN", ascii_case_insensitive)]
    UN,
    /// Directed graph input.
    ///
    /// Accepts aliases `Directed` and `D` (case-insensitive).
    #[strum(serialize = "Directed", serialize = "D", ascii_case_insensitive)]
    D,
    /// Two-dimensional coordinate graph input.
    ///
    /// Accepts aliases `TwoDimensional` and `TD` (case-insensitive).
    #[strum(serialize = "TwoDimensional", serialize = "TD", ascii_case_insensitive)]
    TD,
}

/// Result container for graph data loaded from file input.
///
/// # Invariants
///
/// Exactly one graph variant is expected to be present for valid parsed input. Construction is
/// therefore guarded by [`FileInputGraphResult::new`], which returns `None` if this invariant is
/// violated.
pub struct FileInputGraphResult {
    /// Parsed directed graph, if directed input was detected.
    pub directed_graph: Option<DirectedGraph>,
    /// Parsed undirected graph, if undirected input was detected.
    pub undirected_graph: Option<UndirectedGraph>,
    /// Parsed two-dimensional graph, if two-dimensional input was detected.
    pub two_dimensional_graph: Option<TwoDimensionalCoordinateGraph>,
}

impl FileInputGraphResult {
    /// Creates a validated [`FileInputGraphResult`].
    ///
    /// # Parameters
    ///
    /// - `directed_graph`: Optional directed graph payload.
    /// - `undirected_graph`: Optional undirected graph payload.
    /// - `two_dimensional_graph`: Optional two-dimensional graph payload.
    ///
    /// # Returns
    ///
    /// - `Some(Self)` when exactly one graph option is set.
    /// - `None` when zero or multiple graph options are set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use shortest_path_finder::graphs::directed::DirectedGraph;
    /// use shortest_path_finder::data_input::file_input::FileInputGraphResult;
    ///
    /// let valid = FileInputGraphResult::new(Some(DirectedGraph::default()), None, None);
    /// assert!(valid.is_some());
    /// ```
    ///
    /// ```rust
    /// use shortest_path_finder::data_input::file_input::FileInputGraphResult;
    ///
    /// let invalid = FileInputGraphResult::new(None, None, None);
    /// assert!(invalid.is_none());
    /// ```
    pub fn new(
        directed_graph: Option<DirectedGraph>,
        undirected_graph: Option<UndirectedGraph>,
        two_dimensional_graph: Option<TwoDimensionalCoordinateGraph>,
    ) -> Option<Self> {
        let mut count = 0;
        if directed_graph.is_some() {
            count += 1;
        }
        if undirected_graph.is_some() {
            count += 1;
        }
        if two_dimensional_graph.is_some() {
            count += 1;
        }

        if count != 1 {
            return None;
        }

        Some(Self {
            directed_graph,
            undirected_graph,
            two_dimensional_graph,
        })
    }
}

// _____ Public endpoint of the file input module _____

/// Reads a graph definition file and parses it into one concrete graph result.
///
/// # Parameters
///
/// - `file_path`: Relative or absolute path to the graph input file.
///
/// # Returns
///
/// - `Ok(FileInputGraphResult)` if file reading and parsing succeeds.
/// - `Err(Box<dyn Error>)` if the file cannot be read or if parsing/validation fails.
///
/// # Errors
///
/// This function can fail when:
/// - the provided file does not exist,
/// - the file content cannot be decoded as UTF-8 text,
/// - the input lines do not follow the expected syntax,
/// - or graph generation fails due to semantic validation errors.
///
/// # Example
///
/// ```no_run
/// use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;
///
/// let result = retrieve_graph_data_from_file("test_files/undirected_graph.txt");
/// assert!(result.is_ok());
/// ```
pub fn retrieve_graph_data_from_file(
    file_path: &str,
) -> Result<FileInputGraphResult, Box<dyn Error>> {
    // create relative file path like "../example.txt"
    let rel_path = Path::new(file_path);

    // read contents from the file -> cover occuring errors
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

// ______________________________________

/// Validates whether a single line matches one supported graph-line syntax.
///
/// # Parameters
///
/// - `line`: Raw text line to validate.
///
/// # Returns
///
/// - `true` if the line matches any configured regex for directed, undirected, or two-dimensional
///   patterns.
/// - `false` otherwise.
///
/// # Notes
///
/// This function only checks lexical format. Semantic consistency (such as graph-type consistency
/// across lines) is validated in higher-level parsing functions.
fn validate_line_syntax(line: &str) -> bool {
    // MAKE SURE THIS CONTAINS ALL REGEXs OF ALL GRAPHS
    let reg_exps = vec![
        r"[A-Za-z0-9]+->[A-Za-z0-9]+:[0-9]+",
        r"[A-Za-z0-9]+-[A-Za-z0-9]+:[0-9]+",
        r"[A-Za-z0-9]+:[0-9]+,[0-9]+-[A-Za-z0-9]+:[0-9]+,[0-9]+", // A:2,6 - B:5,5
    ];
    for exp in reg_exps {
        let reg = Regex::new(exp).unwrap();
        if reg.is_match(line) {
            return true;
        }
    }
    false
}

/// Converts one validated edge line into typed node/weight data.
///
/// # Parameters
///
/// - `line`: The edge line to parse.
/// - `detected_graph_type`: The graph type that determines tokenization rules.
///
/// # Returns
///
/// `Ok((from, to, weight))` where:
/// - `from` and `to` are wrapped in [`NodeType`],
/// - `weight` is wrapped in [`WeightType`],
/// - and `weight` is [`WeightType::NotNecessary`] for two-dimensional parsing.
///
/// # Errors
///
/// Returns:
/// - [`ParseError::InvalidLineSyntax`] when separators or token counts are invalid,
/// - [`ParseError::InvalidInteger`] when a 1D weight token cannot be parsed,
/// - [`ParseError::InvalidGraphType`] when no conversion branch is available.
///
/// # Examples
///
/// ```text
/// Directed edge line:   A->B:12
/// Undirected edge line: A-B:12
/// 2D edge line:         A:0,0-B:4,2
/// ```
fn convert_line_to_graph_data(
    line: &str,
    detected_graph_type: &FoundGraphType,
) -> Result<(NodeType, NodeType, WeightType), ParseError> {
    match detected_graph_type {
        FoundGraphType::UN | FoundGraphType::D => {
            let separator = match detected_graph_type {
                FoundGraphType::UN => "-",
                FoundGraphType::D => "->",
                _ => return Err(ParseError::InvalidGraphType),
            };
            let first_split_results: Vec<&str> = line.trim().split(separator).collect();
            if first_split_results.len() != 2 {
                return Err(ParseError::InvalidLineSyntax);
            }
            let second_split_results: Vec<&str> =
                first_split_results[1].trim().split(':').collect();
            if second_split_results.len() != 2 {
                return Err(ParseError::InvalidLineSyntax);
            }
            // create nodes and weight values and return them
            let first_node = DefaultNode::new(first_split_results[0].to_string());
            let second_node = DefaultNode::new(second_split_results[0].to_string());
            let weight: u16 = match second_split_results[1].parse() {
                Ok(w) => w,
                Err(_) => return Err(ParseError::InvalidInteger),
            };
            Ok((
                NodeType::DefaultNode(first_node),
                NodeType::DefaultNode(second_node),
                WeightType::U16(weight),
            ))
        }
        FoundGraphType::TD => {
            let initial_split_results: Vec<&str> = line.trim().split('-').collect();
            if initial_split_results.len() != 2 {
                return Err(ParseError::InvalidLineSyntax);
            }
            let first_node = TwoDimensionalNode::from_str(initial_split_results[0].trim())?;
            // ___________________________
            // split the second node and weight
            // create nodes and weight values and return them
            let second_node = TwoDimensionalNode::from_str(initial_split_results[1].trim())?;
            // return the nodes and weight values
            Ok((
                NodeType::TwoDimensionalNode(first_node),
                NodeType::TwoDimensionalNode(second_node),
                WeightType::NotNecessary,
            ))
        }
    }
}

/// Detects the graph kind from the first input line.
///
/// # Parameters
///
/// - `first_line`: First non-empty line from the input file.
///
/// # Returns
///
/// The inferred [`FoundGraphType`] if the line syntax is valid and a known graph abbreviation
/// prefix is detected.
///
/// The first line is expected to be a graph-type marker such as `D`, `UN`, or `TD`.
///
/// # Errors
///
/// Returns [`ParseError::InvalidDataInput`] if the line does not match any supported syntax or if
/// no graph prefix can be resolved.
fn determine_graph_from_first_line(first_line: &str) -> Result<FoundGraphType, ParseError> {
    if first_line.starts_with(&DirectedGraph::abbreviation()) {
        Ok(FoundGraphType::D)
    } else if first_line.starts_with(&UndirectedGraph::abbreviation()) {
        Ok(FoundGraphType::UN)
    } else if first_line.starts_with(&TwoDimensionalCoordinateGraph::abbreviation()) {
        Ok(FoundGraphType::TD)
    } else {
        Err(ParseError::InvalidDataInput(
            "Couldn't convert the first line to a valid edge of a graph because of an unknown reason!".to_string(),
        ))
    }
}

// TODO: Refactor this function into smaller, graph-type specific parsing functions to improve
// readability and maintainability.

/// Builds a graph result from raw file text.
///
/// # Parameters
///
/// - `lines`: Full file content as one string.
///
/// # Behavior
///
/// - Detects graph type from the first line.
/// - Parses all remaining non-empty lines as edges of that same graph type.
/// - Inserts missing nodes before edge insertion.
/// - Skips duplicate edges.
/// - Returns an error for invalid syntax or incompatible parsed node/weight variants.
///
/// Note: the first line is not inserted as an edge in the resulting graph.
///
/// # Returns
///
/// - `Ok(FileInputGraphResult)` with exactly one graph variant populated.
/// - `Err(ParseError)` when input is empty, malformed, or unsupported.
///
/// # Important
///
/// Two-dimensional graph parsing is supported in this function.
fn generate_graph_from_file(lines: String) -> Result<FileInputGraphResult, ParseError> {
    let mut lines_iter = lines.lines();

    // there must be atleast one line to create a graph
    let first_line = match lines_iter.next() {
        Some(line) => line,
        None => {
            return Err(ParseError::InvalidDataInput(
                "The specified file is empty!".to_string(),
            ));
        }
    };

    let detected_graph_type = determine_graph_from_first_line(first_line)?;

    match detected_graph_type {
        FoundGraphType::D => {
            let mut graph = DirectedGraph::default();
            let graph_type = FoundGraphType::D;
            for line in lines_iter {
                if line.is_empty() {
                    continue;
                }

                if !validate_line_syntax(line) {
                    return Err(ParseError::InvalidDataInput(format!(
                        "Invalid line syntax on the line {}! Please use only 'A->B:2' or 'A-B:5' to stay consistent!",
                        line
                    )));
                }

                let (from, to, weight) = convert_line_to_graph_data(line, &graph_type)?;
                let from = match from {
                    NodeType::DefaultNode(node) => node,
                    _ => {
                        return Err(ParseError::InvalidDataInput(
                            "Directed graph parsing produced an unexpected node type!".to_string(),
                        ));
                    }
                };
                let to = match to {
                    NodeType::DefaultNode(node) => node,
                    _ => {
                        return Err(ParseError::InvalidDataInput(
                            "Directed graph parsing produced an unexpected node type!".to_string(),
                        ));
                    }
                };
                let weight = match weight {
                    WeightType::U16(value) => value,
                    _ => {
                        return Err(ParseError::InvalidDataInput(
                            "Directed graph parsing produced an unexpected weight type!"
                                .to_string(),
                        ));
                    }
                };

                let edge = DirectedEdge::new(from.clone(), to.clone(), weight);
                if graph.does_edge_already_exist(&edge) {
                    continue;
                }

                graph.insert_node(from);
                graph.insert_node(to);

                if let Some(err) = graph.insert_edge(edge) {
                    return Err(ParseError::InvalidDataInput(err.message));
                }
            }

            FileInputGraphResult::new(Some(graph), None, None).ok_or(ParseError::InvalidDataInput(
                "There can only be two graphs at the same time!".to_string(),
            ))
        }
        FoundGraphType::UN => {
            let mut graph = UndirectedGraph::default();
            let graph_type = FoundGraphType::UN;
            for line in lines_iter {
                if line.is_empty() {
                    continue;
                }

                if !validate_line_syntax(line) {
                    return Err(ParseError::InvalidDataInput(format!(
                        "Invalid line syntax on the line {}! Please use only 'A->B:2' or 'A-B:5' to stay consistent!",
                        line
                    )));
                }

                let (from, to, weight) = convert_line_to_graph_data(line, &graph_type)?;
                let from = match from {
                    NodeType::DefaultNode(node) => node,
                    _ => {
                        return Err(ParseError::InvalidDataInput(
                            "Undirected graph parsing produced an unexpected node type!"
                                .to_string(),
                        ));
                    }
                };
                let to = match to {
                    NodeType::DefaultNode(node) => node,
                    _ => {
                        return Err(ParseError::InvalidDataInput(
                            "Undirected graph parsing produced an unexpected node type!"
                                .to_string(),
                        ));
                    }
                };
                let weight = match weight {
                    WeightType::U16(value) => value,
                    _ => {
                        return Err(ParseError::InvalidDataInput(
                            "Undirected graph parsing produced an unexpected weight type!"
                                .to_string(),
                        ));
                    }
                };

                let edge = UndirectedEdge::new(from.clone(), to.clone(), weight);
                if graph.does_edge_already_exist(&edge) {
                    continue;
                }

                graph.insert_node(from);
                graph.insert_node(to);

                if let Some(err) = graph.insert_edge(edge) {
                    return Err(ParseError::InvalidDataInput(err.message));
                }
            }

            FileInputGraphResult::new(None, Some(graph), None).ok_or(ParseError::InvalidDataInput(
                "There can only be two graphs at the same time!".to_string(),
            ))
        }
        FoundGraphType::TD => {
            let mut graph = TwoDimensionalCoordinateGraph::default();
            let graph_type = FoundGraphType::TD;
            for line in lines_iter {
                if line.is_empty() {
                    continue;
                }

                if !validate_line_syntax(line) {
                    return Err(ParseError::InvalidDataInput(format!(
                        "Invalid line syntax on the line {}! Please use only 'A->B:2' or 'A-B:5' to stay consistent!",
                        line
                    )));
                }

                let (node_a, node_b, _) = convert_line_to_graph_data(line, &graph_type)?;
                let node_a = match node_a {
                    NodeType::TwoDimensionalNode(node) => node,
                    _ => {
                        return Err(ParseError::InvalidDataInput(
                            "Two-dimensional graph parsing produced an unexpected node type!"
                                .to_string(),
                        ));
                    }
                };
                let node_b = match node_b {
                    NodeType::TwoDimensionalNode(node) => node,
                    _ => {
                        return Err(ParseError::InvalidDataInput(
                            "Two-dimensional graph parsing produced an unexpected node type!"
                                .to_string(),
                        ));
                    }
                };

                let two_dimensional_edge = TwoDimensionalEdge::new(node_a, node_b);
                if graph.does_edge_already_exist(&two_dimensional_edge) {
                    continue;
                }

                graph.insert_node(two_dimensional_edge.node_one.clone());
                graph.insert_node(two_dimensional_edge.node_two.clone());

                if let Some(err) = graph.insert_edge(two_dimensional_edge) {
                    return Err(ParseError::InvalidDataInput(err.message));
                }
            }
            FileInputGraphResult::new(None, None, Some(graph)).ok_or(ParseError::InvalidDataInput(
                "There can only be two graphs at the same time!".to_string(),
            ))
        }
    }
}
