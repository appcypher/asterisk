//! Models

mod error;
mod prompt;
mod traits;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub mod ollama;
pub mod openai;

pub use error::*;
pub use prompt::*;
pub use traits::*;
