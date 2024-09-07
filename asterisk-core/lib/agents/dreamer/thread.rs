use std::str::FromStr;

use crate::models::{Prompt, PromptMessage, SystemMessage};

use super::{ContextMessage, DreamerError};

//--------------------------------------------------------------------------------------------------
// Constant
//--------------------------------------------------------------------------------------------------

/// The tag for thought messages.
pub const THOUGHT_TAG: &str = "[thought]";

/// The tag for action messages.
pub const ACTION_TAG: &str = "[action]";

/// The tag for observation messages.
pub const OBSERVATION_TAG: &str = "[observation]";

/// The tag for notification messages.
pub const NOTIFICATION_TAG: &str = "[notification]";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A history of agent interactions.
#[derive(Debug, Clone)]
pub struct Thread {
    system: SystemMessage,
    history: Vec<ThreadMessage>,
    context: Option<ContextMessage>,
}

/// A message in the thread.
#[derive(Debug, Clone)]
pub enum ThreadMessage {
    /// A thought message.
    Thought(ThoughtMessage),

    /// An action message.
    Action(ActionMessage),

    /// An observation message.
    Observation(ObservationMessage),

    /// A notification message.
    Notification(NotificationMessage),
}

/// `ThoughtMessage` is produced by the agent showing its thought process.
///
/// This message type is prefixed with `[thought]` tag.
/// An unfinished thought process ends with `...`.
///
/// We encourage the agent not to produce too many concepts in a
/// single thought message.
#[derive(Debug, Clone)]
pub struct ThoughtMessage {
    /// The content of the thought.
    content: String,
}

/// `ActionMessage` is produced by the agent showing its intended action.
///
/// This message type is prefixed with `[action]` tag.
/// An action message is always a direct response to previous thought
/// messages.
///
/// The agent should not produce too many action messages in a row.
#[derive(Debug, Clone)]
pub struct ActionMessage {
    /// The content of the action.
    content: String,
}

/// `ObservationMessage` is produced by the system to notify the agent
/// about the result of its intended action.
///
/// This message type is prefixed with `[observation]` tag.
/// An observation message is always a direct response to previous action
/// messages.
///
/// An observation message can be incomplete if it exceeds a specified
/// user-defined length. However, right now, that has not been implemented.
#[derive(Debug, Clone)]
pub struct ObservationMessage {
    /// The content of the observation.
    content: String,
}

/// `NotificationMessage` is produced by the system to notify the agent
/// about an event.
///
/// This message type is prefixed with `[notification]` tag.
#[derive(Debug, Clone)]
pub struct NotificationMessage {
    /// The content of the notification.
    content: String,
}

//--------------------------------------------------------------------------------------------------
// Method
//--------------------------------------------------------------------------------------------------

impl Thread {
    /// Creates a new thread.
    pub fn new(system_instruction: impl Into<String>) -> Self {
        Self {
            system: SystemMessage::new(system_instruction),
            history: Vec::new(),
            context: None,
        }
    }

    /// Updates the context.
    pub fn update_context(&mut self, message: impl Into<ContextMessage>) {
        self.context = Some(message.into());
    }

    /// Pushes a message to the thread.
    pub fn push_message(&mut self, message: ThreadMessage) {
        self.history.push(message);
    }
}

impl ThreadMessage {
    /// Creates a new thought message and tags it.
    pub fn thought(content: impl Into<String>) -> Self {
        Self::Thought(ThoughtMessage::new(content))
    }

    /// Creates a new action message and tags it.
    pub fn action(content: impl Into<String>) -> Self {
        Self::Action(ActionMessage::new(content))
    }

    /// Creates a new observation message and tags it.
    pub fn observation(content: impl Into<String>) -> Self {
        Self::Observation(ObservationMessage::new(content))
    }

    /// Creates a new notification message and tags it.
    pub fn notification(content: impl Into<String>) -> Self {
        Self::Notification(NotificationMessage::new(content))
    }
}

impl ThoughtMessage {
    /// Creates a new thought message and tags it.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: THOUGHT_TAG.to_string() + "\n" + &content.into(),
        }
    }

    /// Returns the full content of the thought.
    pub fn get_full_content(&self) -> &str {
        &self.content
    }

    /// Returns the main content of the thought.
    pub fn get_main_content(&self) -> &str {
        self.content[THOUGHT_TAG.len()..].trim_start_matches("\n")
    }

    /// Returns true if the thought is incomplete.
    pub fn is_incomplete(&self) -> bool {
        self.get_main_content().ends_with("...")
    }
}

impl ActionMessage {
    /// Creates a new action message and tags it.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: ACTION_TAG.to_string() + "\n" + &content.into(),
        }
    }

    /// Returns the full content of the action.
    pub fn get_full_content(&self) -> &str {
        &self.content
    }

    /// Returns the main content of the action.
    pub fn get_main_content(&self) -> &str {
        self.content[ACTION_TAG.len()..].trim_start_matches("\n")
    }
}

impl ObservationMessage {
    /// Creates a new observation message and tags it.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: OBSERVATION_TAG.to_string() + "\n" + &content.into(),
        }
    }

    /// Returns the full content of the observation.
    pub fn get_full_content(&self) -> &str {
        &self.content
    }

    /// Returns the main content of the observation.
    pub fn get_main_content(&self) -> &str {
        self.content[OBSERVATION_TAG.len()..].trim_start_matches("\n")
    }

    /// Returns true if the observation is incomplete.
    pub fn is_incomplete(&self) -> bool {
        self.get_main_content().ends_with("...")
    }
}

impl NotificationMessage {
    /// Creates a new notification message and tags it.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: NOTIFICATION_TAG.to_string() + "\n" + &content.into(),
        }
    }

    /// Returns the full content of the notification.
    pub fn get_full_content(&self) -> &str {
        &self.content
    }

    /// Returns the main content of the notification.
    pub fn get_main_content(&self) -> &str {
        self.content[NOTIFICATION_TAG.len()..].trim_start_matches("\n")
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl From<ThreadMessage> for PromptMessage {
    fn from(message: ThreadMessage) -> Self {
        match message {
            ThreadMessage::Observation(message) => PromptMessage::assistant(message.content),
            ThreadMessage::Thought(message) => PromptMessage::assistant(message.content),
            ThreadMessage::Action(message) => PromptMessage::assistant(message.content),
            ThreadMessage::Notification(message) => PromptMessage::assistant(message.content),
        }
    }
}

impl From<Thread> for Prompt {
    fn from(thread: Thread) -> Self {
        let mut prompt = Prompt::new();
        prompt.push(PromptMessage::system(thread.system.content));

        for message in thread.history {
            prompt.push(message.into());
        }

        if let Some(context) = thread.context {
            prompt.push(context.into());
        }

        prompt
    }
}

impl FromStr for ThreadMessage {
    type Err = DreamerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(THOUGHT_TAG) {
            return Ok(ThreadMessage::Thought(ThoughtMessage {
                content: s.to_string(),
            }));
        }

        if s.starts_with(ACTION_TAG) {
            return Ok(ThreadMessage::Action(ActionMessage {
                content: s.to_string(),
            }));
        }

        if s.starts_with(OBSERVATION_TAG) {
            return Ok(ThreadMessage::Observation(ObservationMessage {
                content: s.to_string(),
            }));
        }

        if s.starts_with(NOTIFICATION_TAG) {
            return Ok(ThreadMessage::Notification(NotificationMessage {
                content: s.to_string(),
            }));
        }

        Err(DreamerError::InvalidThreadMessage(s.to_string()))
    }
}

impl FromStr for ObservationMessage {
    type Err = DreamerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(OBSERVATION_TAG) {
            return Err(DreamerError::InvalidObservationMessage(s.to_string()));
        }

        Ok(ObservationMessage {
            content: s.to_string(),
        })
    }
}

impl FromStr for ThoughtMessage {
    type Err = DreamerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(THOUGHT_TAG) {
            return Err(DreamerError::InvalidThoughtMessage(s.to_string()));
        }

        Ok(ThoughtMessage {
            content: s.to_string(),
        })
    }
}

impl FromStr for ActionMessage {
    type Err = DreamerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(ACTION_TAG) {
            return Err(DreamerError::InvalidActionMessage(s.to_string()));
        }

        Ok(ActionMessage {
            content: s.to_string(),
        })
    }
}

impl FromStr for NotificationMessage {
    type Err = DreamerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(NOTIFICATION_TAG) {
            return Err(DreamerError::InvalidNotificationMessage(s.to_string()));
        }

        Ok(NotificationMessage {
            content: s.to_string(),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_dreamer_thread_message_from_str() {
        let r = ThoughtMessage::from_str("[thought] This is a thought");
        assert!(r.is_ok());
        assert_eq!(r.unwrap().content, "[thought] This is a thought");

        let r = ActionMessage::from_str("[action] This is an action");
        assert!(r.is_ok());
        assert_eq!(r.unwrap().content, "[action] This is an action");

        let r = ObservationMessage::from_str("[observation] This is an observation");
        assert!(r.is_ok());
        assert_eq!(r.unwrap().content, "[observation] This is an observation");

        let r = NotificationMessage::from_str("[notification] This is a notification");
        assert!(r.is_ok());
        assert_eq!(r.unwrap().content, "[notification] This is a notification");

        let r = ThreadMessage::from_str("[thought] This is a thought");
        assert!(r.is_ok());

        let r = ThreadMessage::from_str("This is a thought");
        assert!(r.is_err());
    }
}
