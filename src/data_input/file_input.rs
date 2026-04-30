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
//! graph-type marker and must be exactly one of:
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
//! 2D edge line: <from>:x,y=><to>:x,y          (example: A:0,0=>B:4,2)
//! ```
//!
//! # Validation and consistency rules
//!
//! - The file must contain at least one line.
//! - The first line must identify a supported graph type using an exact header (`D`, `UN`, `TD`).
//! - Every remaining parsed line must match the syntax expected by the detected graph type.
//! - Whitespace-only lines are ignored.
//! - A file can produce exactly one graph variant.
//! - Duplicate edges are ignored during insertion.
//! - The first line is consumed for type detection and is not inserted as an edge.
//! - Two-dimensional file input is parsed and inserted into
//!   [`TwoDimensionalCoordinateGraph`] in [`generate_graph_from_file`].
//! - Two-dimensional parsing currently uses
//!   [`crate::nodes::two_dimensional_node::TwoDimensionalNode<i32>`] and
//!   therefore produces
//!   [`crate::graphs::two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph<i32>`].
//!
//! # Usage Examples
//!
//! Runnable doctest with a temporary graph file:
//!
//! ```rust
//! use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;
//! use std::{
//!     fs,
//!     time::{SystemTime, UNIX_EPOCH},
//! };
//!
//! let unique_id = SystemTime::now()
//!     .duration_since(UNIX_EPOCH)
//!     .expect("system clock should be after UNIX epoch")
//!     .as_nanos();
//! let file_path =
//!     std::env::temp_dir().join(format!("pathfinder-file-input-doc-{}.txt", unique_id));
//! fs::write(&file_path, "D\nA->B:7\nB->C:2\n").expect("temporary graph file should be writable");
//!
//! let file_path_owned = file_path.to_string_lossy().into_owned();
//! let parsed = retrieve_graph_data_from_file(&file_path_owned).expect("graph file should parse");
//! assert!(parsed.directed_graph.is_some());
//! assert!(parsed.undirected_graph.is_none());
//! assert!(parsed.two_dimensional_graph.is_none());
//!
//! let _ = fs::remove_file(&file_path);
//! ```
//!
//! Repository fixture example:
//!
//! ```no_run
//! use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;
//!
//! let parsed = retrieve_graph_data_from_file("test_files/directed_graph.txt");
//! assert!(parsed.is_ok());
//! ```
//!
//! The fixture example is marked as `no_run` because it depends on repository-local files.

use std::{
    error::Error,
    fmt, fs, io,
    path::Path,
    str::{FromStr, Lines},
};

use regex::Regex;
use strum_macros::EnumString;

use crate::{
    error::parse_error::ParseError,
    graphs::{
        directed::DirectedGraph, graph::Graph,
        two_dimensional_coordinate_graph::TwoDimensionalCoordinateGraph,
        undirected::UndirectedGraph,
    },
    nodes::{
        default_node::DefaultNode, node_types::NodeType, two_dimensional_node::TwoDimensionalNode,
    },
    weight_types::impl_weights::WeightType,
};

// TODO: Add feature that users can choose different coordinate types for the
// two-dimensional graph input. This would require changes in the parsing logic and the expected
// syntax for two-dimensional edges. For example, we could allow users to specify the coordinate
// type in the header line (e.g., `TD<i32>`) and then parse the coordinates accordingly in
// `convert_line_to_graph_data`. This would make the file input more flexible and compatible with
// different use cases.

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

/// Precompiled regex matchers for supported line syntaxes.
///
/// Compiling once per parse run avoids repeated work and guarantees that any
/// regex setup failure is surfaced as a parser error instead of a panic.
struct LineSyntaxRegexes {
    /// Regex for directed lines (`A->B:7`).
    directed: Regex,
    /// Regex for undirected lines (`A-B:7`).
    undirected: Regex,
    /// Regex for two-dimensional lines (`A:0,0=>B:4,2`).
    two_dimensional: Regex,
}

/// Result container for graph data loaded from file input.
///
/// # Invariants
///
/// Exactly one graph variant is expected to be present for valid parsed input. Construction is
/// therefore guarded by [`FileInputGraphResult::new`], which returns `None` if this invariant is
/// violated.
#[derive(Debug)]
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

/// Top-level error type for file-input graph loading.
///
/// This enum preserves source errors from both file I/O and parser execution,
/// making it easier for callers to distinguish filesystem failures from input
/// format/validation issues.
///
/// # Variant semantics
///
/// - [`FileInputError::Io`]: Reading raw file text failed.
/// - [`FileInputError::Parse`]: Reading succeeded, but parser validation failed.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::data_input::file_input::FileInputError;
/// use shortest_path_finder::error::parse_error::ParseError;
///
/// let parse_error = FileInputError::Parse(ParseError::InvalidDataInput("bad input".to_string()));
/// assert!(matches!(parse_error, FileInputError::Parse(_)));
///
/// let io_error = FileInputError::Io {
///     path: "graph.txt".to_string(),
///     source: std::io::Error::other("read failed"),
/// };
/// assert!(matches!(io_error, FileInputError::Io { .. }));
/// ```
#[derive(Debug)]
pub enum FileInputError {
    /// File reading failed.
    Io { path: String, source: io::Error },
    /// File content was read but could not be parsed into a graph.
    Parse(ParseError),
}

impl fmt::Display for FileInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileInputError::Io { path, source } => {
                write!(f, "Failed to read graph file '{}': {}", path, source)
            }
            FileInputError::Parse(source) => {
                write!(f, "Failed to parse graph file content: {}", source)
            }
        }
    }
}

impl Error for FileInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileInputError::Io { source, .. } => Some(source),
            FileInputError::Parse(source) => Some(source),
        }
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
/// - `Err(FileInputError)` if the file cannot be read or if parsing/validation fails.
///
/// # Errors
///
/// This function can fail when:
/// - the provided file does not exist,
/// - the file content cannot be decoded as UTF-8 text,
/// - the input lines do not follow the expected syntax,
/// - or graph generation fails due to semantic validation errors.
///
/// Error variants:
/// - [`FileInputError::Io`] wraps filesystem failures with path context.
/// - [`FileInputError::Parse`] wraps parser failures from [`ParseError`].
///
/// # Examples
///
/// Successful parsing with a temporary file:
///
/// ```rust
/// use shortest_path_finder::data_input::file_input::retrieve_graph_data_from_file;
/// use std::{
///     fs,
///     time::{SystemTime, UNIX_EPOCH},
/// };
///
/// let unique_id = SystemTime::now()
///     .duration_since(UNIX_EPOCH)
///     .expect("system clock should be after UNIX epoch")
///     .as_nanos();
/// let path = std::env::temp_dir().join(format!("pathfinder-doc-directed-{}.txt", unique_id));
/// fs::write(&path, "D\nA->B:4\nB->C:3\n").expect("temporary graph file should be writable");
///
/// let path_owned = path.to_string_lossy().into_owned();
/// let result = retrieve_graph_data_from_file(&path_owned).expect("temporary graph should parse");
/// assert!(result.directed_graph.is_some());
/// assert!(result.undirected_graph.is_none());
/// assert!(result.two_dimensional_graph.is_none());
///
/// let _ = fs::remove_file(path);
/// ```
///
/// I/O failure classification:
///
/// ```rust
/// use shortest_path_finder::data_input::file_input::{
///     retrieve_graph_data_from_file,
///     FileInputError,
/// };
///
/// let err = retrieve_graph_data_from_file(".")
///     .expect_err("a directory path cannot be read as graph file text");
///
/// assert!(matches!(err, FileInputError::Io { .. }));
/// ```
///
/// Parse failure classification:
///
/// ```rust
/// use shortest_path_finder::data_input::file_input::{
///     retrieve_graph_data_from_file,
///     FileInputError,
/// };
/// use std::{
///     fs,
///     time::{SystemTime, UNIX_EPOCH},
/// };
///
/// let unique_id = SystemTime::now()
///     .duration_since(UNIX_EPOCH)
///     .expect("system clock should be after UNIX epoch")
///     .as_nanos();
/// let path = std::env::temp_dir().join(format!("pathfinder-doc-invalid-{}.txt", unique_id));
/// // Header announces directed format but edge uses undirected syntax.
/// fs::write(&path, "D\nA-B:4\n").expect("temporary graph file should be writable");
///
/// let path_owned = path.to_string_lossy().into_owned();
/// let err = retrieve_graph_data_from_file(&path_owned)
///     .expect_err("invalid directed line should return parse error");
/// assert!(matches!(err, FileInputError::Parse(_)));
///
/// let _ = fs::remove_file(path);
/// ```
pub fn retrieve_graph_data_from_file(
    file_path: &str,
) -> Result<FileInputGraphResult, FileInputError> {
    // Normalize user input into a path handle used by std::fs.
    let rel_path = Path::new(file_path);

    // Read full file text eagerly and preserve path context on I/O errors.
    let file_content = fs::read_to_string(rel_path).map_err(|source| FileInputError::Io {
        path: file_path.to_string(),
        source,
    })?;

    let res = generate_graph_from_file(file_content).map_err(FileInputError::Parse)?;

    Ok(res)
}

// Two-dimensional file input uses the `=>` separator between coordinate nodes to avoid
// ambiguity with negative coordinate values.

/// Compiles all regexes required for line-syntax validation.
///
/// # Node ID constraint
///
/// All three formats restrict node IDs to the character class `[A-Za-z0-9]+`.
/// IDs containing other characters (e.g. `Station-42`, `node_1`) are **not**
/// supported in file input and will fail validation. Keep node names to letters
/// and digits only when using file-based graph input.
///
/// # Regex patterns
///
/// - Directed: `^[A-Za-z0-9]+->[A-Za-z0-9]+:[0-9]+$`
/// - Undirected: `^[A-Za-z0-9]+-[A-Za-z0-9]+:[0-9]+$`
/// - Two-dimensional: `^[A-Za-z0-9]+:-?[0-9]+,-?[0-9]+=>[A-Za-z0-9]+:-?[0-9]+,-?[0-9]+$`
///
/// # Errors
///
/// Returns [`ParseError::RegexCompilationFailed`] when any static regex pattern
/// cannot be compiled.
fn compile_line_syntax_regexes() -> Result<LineSyntaxRegexes, ParseError> {
    let directed = Regex::new(r"^[A-Za-z0-9]+->[A-Za-z0-9]+:[0-9]+$")
        .map_err(|err| ParseError::RegexCompilationFailed(err.to_string()))?;
    let undirected = Regex::new(r"^[A-Za-z0-9]+-[A-Za-z0-9]+:[0-9]+$")
        .map_err(|err| ParseError::RegexCompilationFailed(err.to_string()))?;
    let two_dimensional =
        Regex::new(r"^[A-Za-z0-9]+:-?[0-9]+,-?[0-9]+=>[A-Za-z0-9]+:-?[0-9]+,-?[0-9]+$")
            .map_err(|err| ParseError::RegexCompilationFailed(err.to_string()))?;

    Ok(LineSyntaxRegexes {
        directed,
        undirected,
        two_dimensional,
    })
}

/// Validates whether a single line matches one supported graph-line syntax.
///
/// # Parameters
///
/// - `line`: Raw text line to validate.
/// - `graph_type`: Detected graph kind for strict, graph-specific validation.
/// - `regexes`: Precompiled regexes used for matching.
///
/// # Returns
///
/// - `true` if the line fully matches the expected syntax for the current graph type.
/// - `false` otherwise.
///
/// # Notes
///
/// This function only checks lexical format for the currently selected graph
/// type. Semantic validation is handled in higher-level parsing functions.
fn validate_line_syntax(
    line: &str,
    graph_type: &FoundGraphType,
    regexes: &LineSyntaxRegexes,
) -> bool {
    let line = line.trim();
    match graph_type {
        FoundGraphType::D => regexes.directed.is_match(line),
        FoundGraphType::UN => regexes.undirected.is_match(line),
        FoundGraphType::TD => regexes.two_dimensional.is_match(line),
    }
}

/// Returns user-facing syntax guidance for each graph type.
///
/// The returned text is designed to be appended to line-level parser errors so
/// users get both "what failed" and "what shape was expected" in one message.
fn expected_syntax_message(graph_type: &FoundGraphType) -> &'static str {
    match graph_type {
        FoundGraphType::D => "Expected directed syntax '<from>-><to>:<weight>' (example: A->B:5).",
        FoundGraphType::UN => "Expected undirected syntax '<from>-<to>:<weight>' (example: A-B:5).",
        FoundGraphType::TD => {
            "Expected two-dimensional syntax '<from>:x,y=><to>:x,y' (example: A:0,0=>B:4,2)."
        }
    }
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
/// - [`ParseError::InvalidWeightInteger`] when a 1D weight token cannot be parsed,
/// - [`ParseError::InvalidGraphType`] when no conversion branch is available.
///
/// # Examples
///
/// ```text
/// Directed edge line:   A->B:12
/// Undirected edge line: A-B:12
/// 2D edge line:         A:0,0=>B:4,2
/// ```
///
/// # Parsing strategy
///
/// - For one-dimensional graph types (`D`, `UN`): split line by edge separator,
///   then split the right side by `:` to obtain destination and integer weight.
/// - For two-dimensional graph type (`TD`): split the line into two serialized
///   coordinate nodes using `=>` and parse each node with [`TwoDimensionalNode::from_str`].
fn convert_line_to_graph_data(
    line: &str,
    detected_graph_type: &FoundGraphType,
) -> Result<(NodeType, NodeType, WeightType), ParseError> {
    match detected_graph_type {
        FoundGraphType::UN | FoundGraphType::D => {
            // One-dimensional formats differ only by separator; downstream extraction is shared.
            let separator = match detected_graph_type {
                FoundGraphType::UN => "-",
                FoundGraphType::D => "->",
                _ => return Err(ParseError::InvalidGraphType),
            };

            // Split into `<from>` and `<to>:<weight>`.
            // Validation runs before this conversion, so the separator split is deterministic.
            let first_split_results: Vec<&str> = line.trim().split(separator).collect();
            if first_split_results.len() != 2 {
                return Err(ParseError::InvalidLineSyntax);
            }

            // Split destination and weight (`<to>:<weight>`).
            let second_split_results: Vec<&str> =
                first_split_results[1].trim().split(':').collect();
            if second_split_results.len() != 2 {
                return Err(ParseError::InvalidLineSyntax);
            }

            // Build strongly typed node and weight values used by graph insertions.
            let first_node = DefaultNode::new(first_split_results[0].to_string());
            let second_node = DefaultNode::new(second_split_results[0].to_string());
            let weight: u16 = match second_split_results[1].parse() {
                Ok(w) => w,
                Err(_) => return Err(ParseError::InvalidWeightInteger),
            };
            Ok((
                NodeType::DefaultNode(first_node),
                NodeType::DefaultNode(second_node),
                WeightType::U16(weight),
            ))
        }
        FoundGraphType::TD => {
            // Split TD lines into exactly two serialized coordinate nodes.
            // `=>` avoids ambiguity with negative coordinate values.
            let initial_split_results: Vec<&str> = line.trim().split("=>").collect();
            if initial_split_results.len() != 2 {
                return Err(ParseError::InvalidLineSyntax);
            }

            // Each side must parse as `id:x,y` and can surface detailed ParseError variants.
            let first_node = TwoDimensionalNode::<i32>::from_str(initial_split_results[0].trim())?;
            let second_node = TwoDimensionalNode::<i32>::from_str(initial_split_results[1].trim())?;

            // TD edges derive their own geometric weight later, so no explicit file weight token.
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
/// The inferred [`FoundGraphType`] if the line is an exact supported graph header.
///
/// The first line is expected to be a graph-type marker such as `D`, `UN`, or `TD`.
///
/// # Case handling
///
/// Header matching is ASCII case-insensitive (`d`, `un`, `td` are accepted).
///
/// # Errors
///
/// Returns [`ParseError::InvalidDataInput`] if the line is not exactly `D`, `UN`, or `TD`.
fn determine_graph_from_first_line(first_line: &str) -> Result<FoundGraphType, ParseError> {
    let header = first_line.trim();

    if header.eq_ignore_ascii_case(&DirectedGraph::abbreviation()) {
        Ok(FoundGraphType::D)
    } else if header.eq_ignore_ascii_case(&UndirectedGraph::abbreviation()) {
        Ok(FoundGraphType::UN)
    } else if header.eq_ignore_ascii_case(&TwoDimensionalCoordinateGraph::<i32>::abbreviation()) {
        Ok(FoundGraphType::TD)
    } else {
        Err(ParseError::InvalidDataInput(format!(
            "Invalid graph header '{}'. Expected exactly one of: D, UN, TD.",
            header
        )))
    }
}

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
/// - Trims surrounding whitespace before per-line validation/parsing.
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

    // The first line is a mandatory graph-type header (`D`, `UN`, or `TD`).
    let first_line = match lines_iter.next() {
        Some(line) => line,
        None => {
            return Err(ParseError::InvalidDataInput(
                "The specified file is empty!".to_string(),
            ));
        }
    };

    // Parse the remaining lines with the graph-specific builder selected by the header.
    let detected_graph_type = determine_graph_from_first_line(first_line)?;

    match detected_graph_type {
        FoundGraphType::D => {
            let directed_graph = generate_directed_graph_from_file(lines_iter)?;

            FileInputGraphResult::new(Some(directed_graph), None, None).ok_or(
                ParseError::InvalidDataInput(
                    "Exactly one parsed graph variant must be present in FileInputGraphResult."
                        .to_string(),
                ),
            )
        }
        FoundGraphType::UN => {
            let undirected_graph = generate_undirected_graph_from_file(lines_iter)?;

            FileInputGraphResult::new(None, Some(undirected_graph), None).ok_or(
                ParseError::InvalidDataInput(
                    "Exactly one parsed graph variant must be present in FileInputGraphResult."
                        .to_string(),
                ),
            )
        }
        FoundGraphType::TD => {
            let two_dimensional_coordinate_graph =
                generate_two_dimensional_graph_from_file(lines_iter)?;

            FileInputGraphResult::new(None, None, Some(two_dimensional_coordinate_graph)).ok_or(
                ParseError::InvalidDataInput(
                    "Exactly one parsed graph variant must be present in FileInputGraphResult."
                        .to_string(),
                ),
            )
        }
    }
}

/// Builds a directed graph from file lines.
///
/// # Parameters    
///
/// - `lines_iter`: An iterator over the lines of the input file, excluding the first line.
///
/// # Returns
///
/// - `Ok(DirectedGraph)` if all lines are valid and the graph is successfully built.
/// - `Err(ParseError)` if any line has invalid syntax or if graph construction fails due to
///   semantic issues (e.g., incompatible node/weight types).
///
/// # Notes
///
/// This function is currently focused on directed graph parsing. Similar functions can be
/// implemented for undirected and two-dimensional graph parsing to improve modularity and
/// readability.
fn generate_directed_graph_from_file(lines_iter: Lines) -> Result<DirectedGraph, ParseError> {
    let mut graph = DirectedGraph::default();
    let graph_type = FoundGraphType::D;
    let syntax_regexes = compile_line_syntax_regexes()?;

    for (index, raw_line) in lines_iter.enumerate() {
        // `+2`: zero-based enumerate starts after the header line (file line 1).
        let line_number = index + 2;
        let line = raw_line.trim();

        // Empty lines are tolerated to keep hand-authored files readable.
        if line.is_empty() {
            continue;
        }

        if !validate_line_syntax(line, &graph_type, &syntax_regexes) {
            return Err(ParseError::InvalidDataInput(format!(
                "Invalid syntax at line {} ('{}'). {}",
                line_number,
                raw_line,
                expected_syntax_message(&graph_type)
            )));
        }

        let (from, to, weight) = convert_line_to_graph_data(line, &graph_type).map_err(|err| {
            ParseError::InvalidDataInput(format!(
                "Failed to parse line {} ('{}'): {}",
                line_number, raw_line, err
            ))
        })?;

        // Conversion returns generic enums; narrow them to directed-compatible payloads.
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
                    "Directed graph parsing produced an unexpected weight type!".to_string(),
                ));
            }
        };

        graph.insert_node(from.clone());
        graph.insert_node(to.clone());

        // Duplicate edges are ignored to keep insertion idempotent.
        if graph.does_edge_already_exist(&from, &to) {
            continue;
        }

        if let Some(err) = graph.insert_edge(&from, &to, Some(weight)) {
            return Err(ParseError::InvalidDataInput(err.message));
        }
    }

    Ok(graph)
}

/// Builds an undirected graph from file lines.
///
/// # Parameters
///
/// - `lines_iter`: Iterator over lines after the graph-type header.
///
/// # Returns
///
/// - `Ok(UndirectedGraph)` when all non-empty lines parse and insert successfully.
/// - `Err(ParseError)` when any line violates syntax or semantic expectations.
///
/// # Behavior details
///
/// - Validates each non-empty line against undirected syntax (`A-B:7`).
/// - Converts each line into two default nodes and one `u16` weight.
/// - Inserts missing nodes before inserting the edge.
/// - Silently skips duplicate edges.
fn generate_undirected_graph_from_file(lines_iter: Lines) -> Result<UndirectedGraph, ParseError> {
    let mut graph = UndirectedGraph::default();
    let graph_type = FoundGraphType::UN;
    let syntax_regexes = compile_line_syntax_regexes()?;

    for (index, raw_line) in lines_iter.enumerate() {
        // `+2`: one line offset for zero-based enumerate, one for header line.
        let line_number = index + 2;
        let line = raw_line.trim();

        // Ignore blank lines so files can contain visual separators.
        if line.is_empty() {
            continue;
        }

        if !validate_line_syntax(line, &graph_type, &syntax_regexes) {
            return Err(ParseError::InvalidDataInput(format!(
                "Invalid syntax at line {} ('{}'). {}",
                line_number,
                raw_line,
                expected_syntax_message(&graph_type)
            )));
        }

        let (from, to, weight) = convert_line_to_graph_data(line, &graph_type).map_err(|err| {
            ParseError::InvalidDataInput(format!(
                "Failed to parse line {} ('{}'): {}",
                line_number, raw_line, err
            ))
        })?;

        // Enforce that converter output matches undirected graph expectations.
        let from = match from {
            NodeType::DefaultNode(node) => node,
            _ => {
                return Err(ParseError::InvalidDataInput(
                    "Undirected graph parsing produced an unexpected node type!".to_string(),
                ));
            }
        };
        let to = match to {
            NodeType::DefaultNode(node) => node,
            _ => {
                return Err(ParseError::InvalidDataInput(
                    "Undirected graph parsing produced an unexpected node type!".to_string(),
                ));
            }
        };
        let weight = match weight {
            WeightType::U16(value) => value,
            _ => {
                return Err(ParseError::InvalidDataInput(
                    "Undirected graph parsing produced an unexpected weight type!".to_string(),
                ));
            }
        };

        graph.insert_node(from.clone());
        graph.insert_node(to.clone());

        // Skip duplicates to preserve deterministic graph content.
        if graph.does_edge_already_exist(&from, &to) {
            continue;
        }

        if let Some(err) = graph.insert_edge(&from, &to, Some(weight)) {
            return Err(ParseError::InvalidDataInput(err.message));
        }
    }

    Ok(graph)
}

/// Builds a two-dimensional coordinate graph from file lines.
///
/// # Parameters
///
/// - `lines_iter`: Iterator over lines after the graph-type header.
///
/// # Returns
///
/// - `Ok(TwoDimensionalCoordinateGraph)` when all non-empty lines parse and insert successfully.
/// - `Err(ParseError)` when syntax validation or node conversion fails.
///
/// # Behavior details
///
/// - Validates each non-empty line against TD syntax (`A:0,0=>B:4,2`).
/// - Parses both endpoints as `TwoDimensionalNode<i32>`.
/// - Inserts both endpoint nodes before edge insertion.
/// - Silently skips duplicate edges.
fn generate_two_dimensional_graph_from_file(
    lines_iter: Lines,
) -> Result<TwoDimensionalCoordinateGraph, ParseError> {
    let mut graph = TwoDimensionalCoordinateGraph::default();
    let graph_type = FoundGraphType::TD;
    let syntax_regexes = compile_line_syntax_regexes()?;

    for (index, raw_line) in lines_iter.enumerate() {
        // `+2`: parser reports original file line numbers (header occupies line 1).
        let line_number = index + 2;
        let line = raw_line.trim();

        // Ignore blank lines to support grouped TD edge blocks.
        if line.is_empty() {
            continue;
        }

        if !validate_line_syntax(line, &graph_type, &syntax_regexes) {
            return Err(ParseError::InvalidDataInput(format!(
                "Invalid syntax at line {} ('{}'). {}",
                line_number,
                raw_line,
                expected_syntax_message(&graph_type)
            )));
        }

        let (node_a, node_b, _) = convert_line_to_graph_data(line, &graph_type).map_err(|err| {
            ParseError::InvalidDataInput(format!(
                "Failed to parse line {} ('{}'): {}",
                line_number, raw_line, err
            ))
        })?;

        // TD conversion must return TD node variants only.
        let node_a = match node_a {
            NodeType::TwoDimensionalNode(node) => node,
            _ => {
                return Err(ParseError::InvalidDataInput(
                    "Two-dimensional graph parsing produced an unexpected node type!".to_string(),
                ));
            }
        };
        let node_b = match node_b {
            NodeType::TwoDimensionalNode(node) => node,
            _ => {
                return Err(ParseError::InvalidDataInput(
                    "Two-dimensional graph parsing produced an unexpected node type!".to_string(),
                ));
            }
        };

        graph.insert_node(node_a.clone());
        graph.insert_node(node_b.clone());

        // Skip duplicate edges to keep parser idempotent for repeated lines.
        if graph.does_edge_already_exist(&node_a, &node_b) {
            continue;
        }

        if let Some(err) = graph.insert_edge(&node_a, &node_b, None) {
            return Err(ParseError::InvalidDataInput(err.message));
        }
    }
    Ok(graph)
}
