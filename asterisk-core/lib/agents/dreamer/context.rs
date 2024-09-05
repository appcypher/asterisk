use crate::models::PromptMessage;

//--------------------------------------------------------------------------------------------------
// Constant
//--------------------------------------------------------------------------------------------------

/// The tag for context messages.
pub const CONTEXT_TAG: &str = "[context]\n";

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
    /// Creates a new context message.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: CONTEXT_TAG.to_string() + &content.into(),
        }
    }

    /// Returns the full content of the context.
    pub fn get_full_content(&self) -> &str {
        &self.content
    }

    /// Returns the main content of the context.
    pub fn get_main_content(&self) -> &str {
        &self.content[CONTEXT_TAG.len()..]
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
