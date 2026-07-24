//! URL and command entity path mismatch error.

use thiserror::Error;

/// Signals that an entity path from a URL differs from the path in a command.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("URL entity ID path '{url_path}' doesn't match command entity ID path '{command_path}'")]
pub struct UrlParamEntityIdPathNotEqualsCmdError {
    url_path: String,
    command_path: String,
}

impl UrlParamEntityIdPathNotEqualsCmdError {
    /// Creates a path mismatch error.
    pub fn new(url_path: impl Into<String>, command_path: impl Into<String>) -> Self {
        Self {
            url_path: url_path.into(),
            command_path: command_path.into(),
        }
    }

    /// Returns the URL entity path.
    pub fn url_path(&self) -> &str {
        &self.url_path
    }

    /// Returns the command entity path.
    pub fn command_path(&self) -> &str {
        &self.command_path
    }
}
