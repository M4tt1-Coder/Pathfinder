//! Error types used across parsing and algorithm setup.
//!
//! # Overview
//!
//! The crate currently exposes parsing-focused error types through
//! [`parse_error`], CLI setup parsing errors through [`config_error`], and
//! algorithm execution errors through [`algorithm_error`].
//!
//! # Module Map
//!
//! - [`parse_error`]: parse-time failures for graph input.
//! - [`config_error`]: CLI argument and configuration parsing failures.
//! - [`algorithm_error`]: algorithm execution and path reconstruction failures.
//!
//! # Examples
//!
//! ```rust
//! use shortest_path_finder::error::algorithm_error::AlgorithmErrorKind;
//!
//! assert_eq!(AlgorithmErrorKind::NoPath.exit_code(), 6);
//! ```
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
