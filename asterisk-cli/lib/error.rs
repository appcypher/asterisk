use asterisk_core::{agents::dreamer::DreamerError, models::ModelError};
use thiserror::Error;
use tokio::sync::mpsc;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

/// The result type for the cli operations.
pub type CliResult<T> = Result<T, CliError>;

/// Error type for cli operations.
#[derive(Debug, Error)]
pub enum CliError {
    /// An I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Message send error.
    #[error("message send error: {0}")]
    MessageSend(#[from] mpsc::error::SendError<String>),

    /// Dreamer agent error.
    #[error("agent error: {0}")]
    DreamerError(#[from] DreamerError),

    /// Join error.
    #[error("join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    /// Model error.
    #[error("model error: {0}")]
    ModelError(#[from] ModelError),

    /// Invalid model error.
    #[error("invalid model: {0}")]
    InvalidModel(String),
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `CliResult`.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> CliResult<T> {
    Result::Ok(value)
}
