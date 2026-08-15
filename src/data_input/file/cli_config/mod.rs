//! CLI configuration helpers for the file-input runtime path.
//!
//! # Overview
//!
//! This module re-exports the types used to parse command-line arguments
//! that control file-based graph loading. The public surface includes:
//! - [`AppConfig`]: runtime configuration extracted from CLI arguments.
//! - [`InputOrigin`]: enumeration for specifying graph data input source.
//! - Lower-level tokenization and validation helpers (private).
//!
//! # Example
//!
//! ```rust
//! use shortest_path_finder::data_input::file::cli_config::{AppConfig, InputOrigin};
//!
//! let args = vec!["pathfinder", "--start", "A", "--end", "B"]
//!     .into_iter()
//!     .map(String::from)
//!     .collect();
//!
//! let config = AppConfig::setup_config(args)
//!     .expect("configuration should parse")
//!     .into_config()
//!     .expect("expected runtime config");
//! assert!(matches!(config.data_input, InputOrigin::File));
//! ```

mod config;

// ~ flatten module paths ~

pub use config::*;

// ~ public modules ~

// ~ private modules ~

mod parser;
