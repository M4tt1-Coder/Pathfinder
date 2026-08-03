//! Planned command-line input boundary for graph construction.
//!
//! # Overview
//!
//! The modularized data-input layout keeps this namespace reserved for future
//! interactive terminal input. The runtime does not yet expose a stable public
//! API here, but the module documents the intended boundary so the crate layout
//! remains explicit.
//!
//! # Current Status
//!
//! - No public runtime API is exposed yet.
//! - File-based input remains the supported production path.
//! - [`crate::data_input::file::cli_config::InputOrigin::CommandLine`] is the
//!   configuration value that will eventually route into this boundary.
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::data_input::file::cli_config::InputOrigin;
//!
//! assert_eq!(InputOrigin::CommandLine.as_str(), "cmd-line");
//! ```

mod cmd_line_input;
// Keep the implementation module private until the runtime is wired up.
