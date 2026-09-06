//! Module defining errors related to data input operations.
//!
//! # Overview
//!
//! [`DataInputError`] is the unified boundary error for graph-loading failures.
//! It wraps origin-specific errors (currently file input only) so callers such as
//! [`crate::error::AppError`] can handle every input failure through one enum.
//!
//! # Variant Map
//!
//! | Variant | Source type | When it occurs |
//! |---------|-------------|----------------|
//! | [`DataInputError::File`] | [`FileInputError`] | Graph file I/O or parse failure |
//!
//! # Usage
//!
//! Match on [`DataInputError`] to branch on the input origin, and rely on
//! [`std::fmt::Display`] for human-readable messages in logs or CLI output.
//!
//! # Examples
//!
//! Creating a parse failure from a file-input error:
//!
//! ```rust
//! use shortest_path_finder::data_input::file::FileInputError;
//! use shortest_path_finder::error::DataInputError;
//! use shortest_path_finder::error::ParseError;
//!
//! let file_err = FileInputError::Parse {
//!     file_path: "graph.txt".to_string(),
//!     source: ParseError::MissingColon,
//! };
//! let err = DataInputError::File(file_err);
//! assert!(err.to_string().contains("File input error"));
//! ```
//!
//! Converting via [`From<FileInputError>`]:
//!
//! ```rust
//! use shortest_path_finder::data_input::file::FileInputError;
//! use shortest_path_finder::error::DataInputError;
//!
//! let io_err = FileInputError::Io {
//!     path: "graph.txt".to_string(),
//!     source: std::io::Error::other("permission denied"),
//! };
//! let err: DataInputError = io_err.into();
//! assert!(matches!(err, DataInputError::File(_)));
//! ```

use std::error::Error;

use crate::data_input::file::FileInputError;

/// Represents errors that can occur during data input operations.
///
/// This enum encapsulates origin-specific input failures. New origins (for
/// example interactive CLI input) can add variants here without changing the
/// public [`crate::error::AppError`] surface.
///
/// # Example
///
/// ```rust
/// use shortest_path_finder::data_input::file::FileInputError;
/// use shortest_path_finder::error::DataInputError;
/// use shortest_path_finder::error::ParseError;
///
/// let err = DataInputError::File(FileInputError::Parse {
///     file_path: "graph.txt".to_string(),
///     source: ParseError::InvalidLineSyntax,
/// });
/// assert!(err.to_string().contains("Invalid syntax"));
/// ```
#[derive(Debug)]
pub enum DataInputError {
    /// Error variant for issues encountered during file input operations.
    ///
    /// Contains a [`FileInputError`] with filesystem or parse details.
    File(FileInputError),
}

impl std::fmt::Display for DataInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataInputError::File(err) => write!(f, "File input error: {}", err),
        }
    }
}

impl From<FileInputError> for DataInputError {
    fn from(err: FileInputError) -> Self {
        DataInputError::File(err)
    }
}

impl Error for DataInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DataInputError::File(err) => Some(err),
        }
    }
}
