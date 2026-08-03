//! Terminal-based graph input placeholder.
//!
//! # Overview
//!
//! This module is the future home of interactive graph construction from the
//! terminal. The runtime currently accepts [`InputOrigin::CommandLine`]
//! as configuration input, but the binary still reports that origin as
//! unsupported because the implementation has not been wired up yet.
//!
//! # Current Status
//!
//! - No public API is exposed from this module yet.
//! - File input remains the supported production path.
//! - The module exists so the crate layout already reflects the future shape of
//!   the runtime boundary.
//!
//! # Related Usage
//!
//! ```rust
//! use shortest_path_finder::data_input::file::cli_config::InputOrigin;
//!
//! let origin = InputOrigin::CommandLine;
//! assert!(matches!(origin, InputOrigin::CommandLine));
//! ```

// TODO: Implement CL-based graph input parsing and construction. + Add 'CommandLineInputError' and
// map to DataINputError::CommandLine variant. + Add CLI input parsing errors to AppError
