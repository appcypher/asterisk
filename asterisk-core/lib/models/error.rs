use std::{error::Error, fmt::Display};

use reqwest::Response;
use thiserror::Error;

use super::openai;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// The result type for model operations.
pub type ModelResult<T> = Result<T, ModelError>;

/// Error type for model operations.
#[derive(Debug, Error)]
pub enum ModelError {
    /// Error that occurs when a request to the API fails.
    #[error("Failed to send request to API")]
    RequestError(#[from] reqwest::Error),

    /// Error that occurs when the API returns an error from OpenAI.
    #[error("OpenAI error: {0}")]
    OpenAIResponseError(#[from] openai::ResponseError),

    /// Error that occurs when the response stream from the API fails.
    #[error("Failed to parse response from API")]
    OpenAIResponseStreamError(#[from] OpenAIResponseStreamError),

    /// Error that occurs when parsing the response from the API fails.
    #[error("Failed to parse response from API")]
    ParseError(#[from] serde_json::Error),

    /// Custom error.
    #[error(transparent)]
    Custom(#[from] AnyError),
}

/// Error type for the response stream from the OpenAI API.
#[derive(Debug, Error)]
pub enum OpenAIResponseStreamError {
    /// Error related to the OpenAI API.
    #[error("OpenAI response error")]
    OpenAIResponse(Response),

    /// Other errors related to the SSE.
    #[error("EventSource error: {0}")]
    EventSourceError(#[from] reqwest_eventsource::Error),
}

/// An error that can represent any error.
#[derive(Debug)]
pub struct AnyError {
    error: anyhow::Error,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ModelError {
    /// Creates a new custom `Err` result.
    pub fn custom(error: impl Into<anyhow::Error>) -> ModelError {
        ModelError::Custom(AnyError {
            error: error.into(),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `ModelResult`.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> ModelResult<T> {
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
