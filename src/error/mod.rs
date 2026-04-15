//! Error types used across parsing and algorithm setup.
//!
//! # Overview
//!
//! The crate currently exposes parsing-focused error types through
//! [`parse_error`].
//!
//! # Usage
//!
//! ```rust
//! use shortest_path_finder::error::parse_error::ParseError;
//!
//! let err = ParseError::InvalidLineSyntax;
//! assert!(err.to_string().contains("Invalid syntax"));
//! ```

pub mod parse_error;
