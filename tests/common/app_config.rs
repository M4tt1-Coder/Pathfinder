//! This module contains the AppConfig struct and related functionality for managing application
//! configuration.
//!
//!

use shortest_path_finder::data_input::file::cli_config::{
    AppConfig, AppConfigOutcome,
};

/// Builds a vector of strings from a slice of string slices.
///
/// # Arguments
/// `parts` - A slice of string slices to be converted into a vector of strings.
///
/// # Returns
/// A vector of strings containing the elements from the input slice.
pub fn build_args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_string()).collect()
}

/// Unwraps the [`AppConfigOutcome`] to retrieve the AppConfig.
///
/// # Arguments
/// `outcome` - The [`AppConfigOutcome`] to be unwrapped.
///
/// # Returns
/// The [`AppConfig`] contained within the [`AppConfigOutcome`].
pub fn unwrap_config(outcome: AppConfigOutcome) -> AppConfig {
    outcome.into_config().expect("expected config")
}
