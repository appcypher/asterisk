//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

use std::vec::IntoIter;

use super::openai::{RequestMessage, RequestMessages};

/// A prompt is collection of messages that serves as input to the model.
pub struct Prompt {
    messages: Vec<PromptMessage>,
}

/// A message is a single message in the prompt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptMessage {
    /// A message that sets the context for the conversation.
    System(SystemMessage),

    /// A message that assumes the role of the user.
    User(UserMessage),

    /// A message that assumes the role of the assistant.
    Assistant(AssistantMessage),
}

/// A system message is a message that sets the context for the conversation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemMessage {
    /// The content of the message.
    pub content: String,
}

/// A user message is a message that the user sends to the assistant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserMessage {
    /// The content of the message.
    pub content: String,
}

/// A assistant message is a message that the assistant sends to the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantMessage {
    /// The content of the message.
    pub content: String,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Prompt {
    /// Create an empty prompt.
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    /// Add a message to the prompt.
    pub fn push(&mut self, message: PromptMessage) {
        self.messages.push(message);
    }

    /// Remove the last message from the prompt.
    pub fn pop(&mut self) -> Option<PromptMessage> {
        self.messages.pop()
    }

    /// Add a message to the prompt.
    pub fn add_message(&mut self, message: PromptMessage) {
        self.messages.push(message);
    }

    /// Get the number of messages in the prompt.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if the prompt is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl SystemMessage {
    /// Create a new system message.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl UserMessage {
    /// Create a new user message.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl AssistantMessage {
    /// Create a new assistant message.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl PromptMessage {
    /// Create a new system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self::System(SystemMessage::new(content.into()))
    }

    /// Create a new user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self::User(UserMessage::new(content.into()))
    }

    /// Create a new assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::Assistant(AssistantMessage::new(content.into()))
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl IntoIterator for Prompt {
    type Item = PromptMessage;
    type IntoIter = IntoIter<PromptMessage>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Prompt> for RequestMessages {
    fn from(prompt: Prompt) -> Self {
        let request_messages = prompt
            .into_iter()
            .map(|m| match m {
                PromptMessage::System(SystemMessage { content }) => RequestMessage::System {
                    content,
                    name: None,
                },
                PromptMessage::User(UserMessage { content }) => RequestMessage::User {
                    content,
                    name: None,
                },
                PromptMessage::Assistant(AssistantMessage { content }) => {
                    RequestMessage::Assistant {
                        content,
                        name: None,
                        refusal: None,
                        tool_calls: None,
                    }
                }
            })
            .collect();

        Self(request_messages)
    }
}

//--------------------------------------------------------------------------------------------------
// Macros
//--------------------------------------------------------------------------------------------------

/// Create a prompt from a list of messages.
#[macro_export(local_inner_macros)]
macro_rules! prompt {
    ($($ident:ident : $str:tt),* $(,)?) => {{
        let mut prompt = $crate::models::Prompt::new();
        $(
            prompt.push(prompt!(@message $ident :  $str));
        )*
        prompt
    }};
    (@message system : $str:tt) => {
        $crate::models::PromptMessage::System($crate::models::SystemMessage::new(prompt!(@content $str)))
    };
    (@message user : $str:tt) => {
        $crate::models::PromptMessage::User($crate::models::UserMessage::new(prompt!(@content $str)))
    };
    (@message assistant : $str:tt) => {
        $crate::models::PromptMessage::Assistant($crate::models::AssistantMessage::new(prompt!(@content $str)))
    };
    (@content $str:literal) => { $str.to_string() };
    (@content [ ]) => { String::new() };
    (@content [ $($str:literal)+ ]) => {
        [$($str),*].join("\n")
    };
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_prompt() {
        let prompt = prompt! {
            system: [
                "You are a helpful Japanese assistant."
                "You should always answer in Japanese."
            ],
            user: "What is the weather in Tokyo?",
            assistant: "The weather in Tokyo is sunny.",
        };

        assert_eq!(prompt.len(), 3);
        assert_eq!(
            prompt.messages[0],
            PromptMessage::system(
                "You are a helpful Japanese assistant.\nYou should always answer in Japanese."
            )
        );
        assert_eq!(
            prompt.messages[1],
            PromptMessage::user("What is the weather in Tokyo?")
        );
        assert_eq!(
            prompt.messages[2],
            PromptMessage::assistant("The weather in Tokyo is sunny.")
        );
    }
}
