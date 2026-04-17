//! Command-line parsing and runtime configuration types.
//!
//! # Overview
//!
//! This module provides application startup configuration parsing from CLI
//! arguments via [`app_config`].
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::cmd_line::app_config::AppConfig;
//!
//! let args = vec!["pathfinder", "--start", "A", "--end", "B"]
//!     .into_iter()
//!     .map(String::from)
//!     .collect();
//! let config = AppConfig::setup_config(args).unwrap();
//! assert_eq!(config.start_node_id, "A");
//! ```

pub mod app_config;
