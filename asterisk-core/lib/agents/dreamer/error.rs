use thiserror::Error;
use tokio::sync::mpsc;

use crate::{models, tools};

use super::{Metrics, ThreadMessage};

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// The result type for the dreamer agent operations.
pub type DreamerResult<T> = Result<T, DreamerError>;

/// Error type for the dreamer agent operations.
#[derive(Debug, Error)]
pub enum DreamerError {
    /// Invalid context message.
    #[error("invalid context message: {0}")]
    InvalidContextMessage(String),

    /// Invalid observation message.
    #[error("invalid observation message: {0}")]
    InvalidObservationMessage(String),

    /// Invalid thought message.
    #[error("invalid thought message: {0}")]
    InvalidThoughtMessage(String),

    /// Invalid notification message.
    #[error("invalid notification message: {0}")]
    InvalidNotificationMessage(String),

    /// Invalid action message.
    #[error("invalid action message: {0}")]
    InvalidActionMessage(String),

    /// Invalid thread message.
    #[error("invalid thread message: {0}")]
    InvalidThreadMessage(String),

    /// Invalid response message from the model.
    #[error("invalid response message from the model: {0:?}")]
    InvalidResponseMessage(ThreadMessage),

    /// Model error.
    #[error("model error: {0}")]
    ModelError(#[from] models::ModelError),

    /// Metrics send error.
    #[error("metrics send error: {0}")]
    MetricsSendError(#[from] mpsc::error::SendError<Metrics>),

    /// JSON parsing error.
    #[error("JSON parsing error: {0}")]
    JsonParsingError(#[from] serde_json::Error),

    /// Tool error.
    #[error("tool error: {0}")]
    ToolError(#[from] tools::ToolError),
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `DreamerResult`.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> DreamerResult<T> {
    Result::Ok(value)
}
