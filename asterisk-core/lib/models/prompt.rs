//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

use std::vec::IntoIter;

/// A prompt is the input that the model will use to generate a response.
pub struct Prompt {
    messages: Vec<Message>,
}

/// A message is a single message in the prompt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// A message that sets the context for the conversation.
    System(String),

    /// A message that assumes the role of the user.
    User(String),

    /// A message that assumes the role of the assistant.
    Assistant(String),
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
    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Remove the last message from the prompt.
    pub fn pop(&mut self) -> Option<Message> {
        self.messages.pop()
    }

    /// Adds a user message to the prompt.
    pub fn add_user(&mut self, message: String) {
        self.messages.push(Message::User(message));
    }

    /// Adds an assistant message to the prompt.
    pub fn add_assistant(&mut self, message: String) {
        self.messages.push(Message::Assistant(message));
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

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl IntoIterator for Prompt {
    type Item = Message;
    type IntoIter = IntoIter<Message>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self::new()
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
        $crate::models::Message::System(prompt!(@content $str))
    };
    (@message user : $str:tt) => {
        $crate::models::Message::User(prompt!(@content $str))
    };
    (@message assistant : $str:tt) => {
        $crate::models::Message::Assistant(prompt!(@content $str))
    };
    (@content $str:literal) => { $str.to_string() };
    (@content [ ]) => { String::new() };
    (@content [ $($str:literal),+ $(,)? ]) => {
        [$($str),*].join(" ")
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
                "You are a helpful Japanese assistant.",
                "You should always answer in Japanese.",
            ],
            user: "What is the weather in Tokyo?",
            assistant: "The weather in Tokyo is sunny.",
        };

        assert_eq!(prompt.len(), 3);
        assert_eq!(
            prompt.messages[0],
            Message::System(
                "You are a helpful Japanese assistant. You should always answer in Japanese."
                    .to_string()
            )
        );
        assert_eq!(
            prompt.messages[1],
            Message::User("What is the weather in Tokyo?".to_string())
        );
        assert_eq!(
            prompt.messages[2],
            Message::Assistant("The weather in Tokyo is sunny.".to_string())
        );
    }
}
