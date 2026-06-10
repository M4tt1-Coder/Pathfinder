//! Module defining errors related to data input operations.
//!
//! This module provides the [`DataInputError`] enum, which represents
//! various errors that can occur during data input processes, such as
//! reading from files or command-line inputs.
//!
//! # Usage
//!
//! You can match against [`DataInputError`] variants to handle specific
//! error cases, and utilize the [`Display`] implementation to produce
//! human-readable error messages.
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::error::{data_input_error::DataInputError, parse_error::ParseError};
//! use shortest_path_finder::data_input::file_input::FileInputError;
//!
//! // Example of creating a file input error
//! let file_err = FileInputError::Parse(ParseError::MissingColon);
//! let err = DataInputError::File(file_err);
//!
//! // Display the error
//! println!("{}", err);
//! ```

use std::error::Error;

use crate::data_input::file_input::FileInputError; // Import for file input error variants

/// Represents errors that can occur during data input operations.
///
/// This enum encapsulates different error types related to data input,
/// such as file reading issues or other input-related problems.
#[derive(Debug)]
pub enum DataInputError {
    /// Error variant for issues encountered during file input operations.
    ///
    /// Contains a [`FileInputError`] which provides detailed information
    /// about the specific file input error.
    File(FileInputError),
    // Future error variants related to command-line input can be added here.
    // For example:
    // CommandLine(),
}

impl std::fmt::Display for DataInputError {
    /// Formats the [`DataInputError`] into a human-readable string.
    ///
    /// The output message depends on the specific variant.
    ///
    /// # Variants
    /// - [`File`]: Displays the error message from the contained [`FileInputError`].
    /// - Future variants (e.g., command-line input errors) can have their own display messages when
    ///   implemented.
    ///
    /// # Example
    //// ```rust
    /// use shortest_path_finder::error::DataInputError;
    /// use shortest_path_finder::data_input::file_input::FileInputError;
    ///
    /// let file_err = FileInputError::ReadError("Failed to read file".to_string());
    /// let err = DataInputError::File(file_err);
    /// println!("{}", err); // Output: "File input error: Failed to read file"
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Display message for file input errors.
            DataInputError::File(err) => write!(f, "File input error: {}", err),
            // Add display for other variants when implemented.
            // DataInputError::CommandLine() => write!(f, "Command line input error: ..."),
        }
    }
}

impl From<FileInputError> for DataInputError {
    /// Converts a [`FileInputError`] into a [`DataInputError`].
    ///
    /// This allows for easy conversion of file input errors into the
    /// unified data input error type used by the CLI.
    fn from(err: FileInputError) -> Self {
        DataInputError::File(err)
    }
}

impl Error for DataInputError {
    /// Provides the underlying source of the error, if available.
    ///
    /// For example, for a [`File`] variant, it returns the underlying
    /// [`FileInputError`]. Returns `None` if there is no underlying error.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            //The source is the underlying [`FileInputError`].
            DataInputError::File(err) => Some(err),
            // Add source retrieval for other variants when implemented.
            // DataInputError::CommandLine() => None,
        }
    }
}
