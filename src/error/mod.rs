//! Error types used across parsing and algorithm setup.
//!
//! # Overview
//!
//! The crate currently exposes parsing-focused error types through
//! [`parse_error`], CLI setup parsing errors through [`config_error`], and
//! algorithm execution errors through [`algorithm_error`].
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::error::parse_error::ParseError;
//!
//! let err = ParseError::InvalidLineSyntax;
//! assert!(err.to_string().contains("Invalid syntax"));
//! ```

pub mod config_error;

pub mod parse_error;

pub mod algorithm_error;
