use std::{error::Error, fmt::Display};

use thiserror::Error;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// The result type for the dreamer agent operations.
pub type ToolResult<T> = Result<T, ToolError>;

/// Error type for the dreamer agent operations.
#[derive(Debug, Error)]
pub enum ToolError {
    /// The tool failed to execute.
    #[error("The tool failed to execute: {0}")]
    ExecutionFailed(String),

    /// The tool failed to parse.
    #[error("The tool failed to parse: {0}")]
    ParseFailed(#[from] serde_json::Error),

    /// Custom error.
    #[error(transparent)]
    Custom(#[from] AnyError),
}

/// An error that can represent any error.
#[derive(Debug)]
pub struct AnyError {
    error: anyhow::Error,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ToolError {
    /// Creates a new custom `Err` result.
    pub fn custom(error: impl Into<anyhow::Error>) -> ToolError {
        ToolError::Custom(AnyError {
            error: error.into(),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `ToolResult`.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> ToolResult<T> {
    Result::Ok(value)
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl PartialEq for AnyError {
    fn eq(&self, other: &Self) -> bool {
        self.error.to_string() == other.error.to_string()
    }
}

impl Display for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for AnyError {}
