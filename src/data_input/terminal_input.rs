//! Terminal-based graph input (planned module).
//!
//! # Overview
//!
//! Interactive graph input is planned but not implemented yet. Runtime support
//! for [`InputOrigin::CommandLine`](crate::cmd_line::app_config::InputOrigin::CommandLine)
//! currently resolves to `unimplemented!()` in the binary flow.
//!
//! # Current Status
//!
//! - No public API is exposed from this module yet.
//! - File input remains the supported production path.
//!
//! # Related Usage
//!
//! ```rust
//! use shortest_path_finder::cmd_line::app_config::InputOrigin;
//!
//! let origin = InputOrigin::CommandLine;
//! assert!(matches!(origin, InputOrigin::CommandLine));
//! ```

// TODO: Implement terminal-based graph input parsing and construction.
