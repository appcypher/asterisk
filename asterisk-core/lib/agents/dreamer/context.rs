use std::str::FromStr;

use crate::models::PromptMessage;

use super::DreamerError;

//--------------------------------------------------------------------------------------------------
// Constant
//--------------------------------------------------------------------------------------------------

/// The tag for context messages.
pub const CONTEXT_TAG: &str = "[context]";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A message containing context.
#[derive(Debug, Clone)]
pub struct ContextMessage {
    content: String,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ContextMessage {
    /// Creates a new context message and tags it.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: CONTEXT_TAG.to_string() + "\n" + &content.into(),
        }
    }

    /// Returns the full content of the context.
    pub fn get_full_content(&self) -> &str {
        &self.content
    }

    /// Returns the main content of the context.
    pub fn get_main_content(&self) -> &str {
        self.content[CONTEXT_TAG.len()..].trim_start()
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl From<ContextMessage> for PromptMessage {
    fn from(message: ContextMessage) -> Self {
        PromptMessage::assistant(message.content)
    }
}

impl FromStr for ContextMessage {
    type Err = DreamerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start();

        if !s.starts_with(CONTEXT_TAG) {
            return Err(DreamerError::InvalidContextMessage(s.to_string()));
        }

        Ok(ContextMessage {
            content: s.to_string(),
        })
    }
}
