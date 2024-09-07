//! First attempt at creating a reliable agent.

use std::collections::HashMap;

use serde_json::Map;
use tokio::{sync::mpsc, task::JoinHandle};

use crate::{
    models::{openai::OpenAIModel, TextModel},
    tools::{self, message_box::MessageBox, Tool},
};

use super::{
    ActionMessage, AgentSideChannels, DreamerBuilder, DreamerError, DreamerResult, Memories,
    Metrics, ThoughtMessage, Thread, ThreadMessage,
};

//-------------------------------------------------------------------------------------------------
// Constant
//--------------------------------------------------------------------------------------------------

/// The system instruction for the dreamer agent.
pub const DREAMER_SYSTEM_INSTRUCTION: &str = include_str!("instructions/dreamer-0.1.1.md");

/// The notification message from the user.
pub const NOTIFICATION_USER_MESSAGE: &str = "Message from the user!";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `Dreamer` is a general-purpose agent that can shape its dreams in a way that helps it solve any
/// task.
///
/// ## Instructions
///
/// Dreamer is set up with three (3) types of instructions:
/// 1. **Identity Statement**: This is a statement that defines the factual information about what
///    the assistant is, what it does, and how it works. This allows the assistant to know who it is
///    and what its capabilities are.
///
/// 2. **Operating Procedure**: This defines the rules of how the assistant is to operate and interact
///    with the user or tools.
///
/// 3. **Problem Solving Methodology**: This defines the rules of how the assistant is to solve
///    problems and things it must consider when solving them.
///
/// ## The User
///
/// The user is the one that interacts with the agent, giving it instructions and
/// providing feedback.
///
/// ## What is Dreamer really?
///
/// You can think of dreamer as an entity dreaming up a story with an external force (the user)
/// adding context to the dream to influence the direction of the dream. `Dreamer` dreams lucidly.
///
/// The system instruction is the initial influence on how the dream should progress, defining an
/// AI assistant that knows what it is, how it works, and what it is supposed to do.
///
// TODO: Relying on context and knowledge base only
// TODO: General coherence for thoughts and actions.
#[allow(dead_code)] // TODO: remove this
pub struct Dreamer<M = OpenAIModel> {
    /// The model used to generate responses.
    pub(crate) model: M,

    /// The memories of the assistant.
    pub(crate) memories: Memories,

    /// The thread of conversation.
    pub(crate) thread: Thread,

    /// The tool for reading the user message.
    pub(crate) message_box: MessageBox,

    /// The provided tools the assistant has access to.
    pub(crate) provided_tools: HashMap<String, Box<dyn Tool + Send + Sync>>,

    /// Whether the agent is idle.
    pub(crate) idle: bool,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Dreamer {
    /// Creates a new `Dreamer` builder.
    pub fn builder() -> DreamerBuilder<()> {
        DreamerBuilder::default()
    }
}

impl<M> Dreamer<M> {
    /// Creates a new `Dreamer` with specified model.
    pub fn new(model: M) -> Self {
        Self {
            model,
            memories: Memories::new(),
            thread: Thread::new(DREAMER_SYSTEM_INSTRUCTION),
            message_box: MessageBox::default(),
            provided_tools: HashMap::new(),
            idle: true,
        }
    }

    /// Runs the agent.
    pub fn run(mut self, mut channels: AgentSideChannels) -> JoinHandle<DreamerResult<()>>
    where
        M: TextModel + Send + Sync + 'static,
    {
        tokio::spawn(async move {
            loop {
                if self.idle {
                    // Check if there is an incoming message from the outside world
                    if let Some(message) = channels.message_rx.recv().await {
                        self.handle_incoming_message(message, &channels.metrics_tx)?;
                    }

                    continue;
                }

                tokio::select! {
                    // API call to the LLM
                    response = self.call() => {
                        self.handle_model_response(response?, &channels.metrics_tx)?;
                    }
                    // Incoming message from the outside world
                    message = channels.message_rx.recv() => if let Some(message) = message {
                        self.handle_incoming_message(message, &channels.metrics_tx)?;
                    }
                }
            }
        })
    }
}

impl<M> Dreamer<M> {
    /// Handles the model response.
    fn handle_model_response(
        &mut self,
        response: String,
        metrics_tx: &mpsc::UnboundedSender<Metrics>,
    ) -> DreamerResult<()> {
        // Parse the response into a ThreadMessage.
        let message: ThreadMessage = response.parse()?;

        // Handle the message based on its type.
        match message.clone() {
            ThreadMessage::Thought(thought) => self.handle_thought(thought, metrics_tx)?,
            ThreadMessage::Action(action) => self.handle_action(action, metrics_tx)?,
            _ => return Err(DreamerError::InvalidResponseMessage(message)),
        }

        // Add message to the thread.
        self.thread.push_message(message);

        Ok(())
    }

    /// Handles the incoming message.
    fn handle_incoming_message(
        &mut self,
        message: String,
        metrics_tx: &mpsc::UnboundedSender<Metrics>,
    ) -> DreamerResult<()> {
        // Update the message box.
        self.message_box.update_message(message);

        // Extend the thread with the message.
        let message = ThreadMessage::notification(NOTIFICATION_USER_MESSAGE);

        // Send metrics to the metrics channel.
        metrics_tx.send(Metrics::ThreadMessage(message.clone()))?;

        // Add message to the thread.
        self.thread.push_message(message);

        // Make the agent busy.
        self.make_busy();

        Ok(())
    }

    /// Handles the thought message.
    fn handle_thought(
        &mut self,
        thought: ThoughtMessage,
        metrics_tx: &mpsc::UnboundedSender<Metrics>,
    ) -> DreamerResult<()> {
        // Send metrics to the metrics channel.
        metrics_tx.send(Metrics::ThreadMessage(ThreadMessage::Thought(
            thought.clone(),
        )))?;

        if thought.is_incomplete() {
            self.make_busy();
        } else {
            self.make_idle();
        }

        Ok(())
    }

    /// Handles the action message.
    fn handle_action(
        &mut self,
        action: ActionMessage,
        metrics_tx: &mpsc::UnboundedSender<Metrics>,
    ) -> DreamerResult<()> {
        // Add the action to the thread.
        self.thread
            .push_message(ThreadMessage::Action(action.clone()));

        let (name, _args) = tools::parse_tool(action.get_main_content())?;

        if name == "message_box" {
            // Execute the tool and get the observation.
            let observation = self.message_box.execute(Map::new())?;

            // Create the observation message.
            let message = ThreadMessage::observation(observation);

            // Send metrics to the metrics channel.
            metrics_tx.send(Metrics::ThreadMessage(message.clone()))?;

            // Add observation to the thread.
            self.thread.push_message(message);

            // Make the agent busy.
            self.make_busy();

            return Ok(());
        }

        self.make_idle();

        Ok(())
    }

    /// Calls the model by sending the thread to the model and receiving a response.
    async fn call(&self) -> DreamerResult<String>
    where
        M: TextModel + Send + Sync + 'static,
    {
        self.model
            .prompt(self.thread.clone())
            .await
            .map_err(Into::into)
    }

    /// Makes the agent idle.
    fn make_idle(&mut self) {
        self.idle = true;
    }

    /// Makes the agent busy.
    fn make_busy(&mut self) {
        self.idle = false;
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_agent_dreamer() { }
}
