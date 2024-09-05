//! First attempt at creating a reliable agent.

use tokio::{sync::mpsc, task::JoinHandle};
use tracing::info;

use crate::models::{openai::OpenAIModel, TextModel};

use super::{
    ActionMessage, AgentSideChannels, DreamerBuilder, DreamerError, DreamerResult, Memories,
    Metrics, ThoughtMessage, Thread, ThreadMessage, Tool,
};

//--------------------------------------------------------------------------------------------------
// Constant
//--------------------------------------------------------------------------------------------------

/// The system instruction for the dreamer agent.
const DREAMER_SYSTEM_INSTRUCTION: &str = include_str!("instruction.txt");

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
///    with the manager or tools.
///
/// 3. **Problem Solving Methodology**: This defines the rules of how the assistant is to solve
///    problems and things it must consider when solving them.
///
/// ## The Manager
///
/// The manager (a user) is the one that interacts with the agent, giving it instructions and
/// providing feedback.
///
/// ## What is Dreamer really?
///
/// You can think of dreamer as an entity dreaming up a story with an external force (the manager)
/// adding context to the dream to influence the direction of the dream. `Dreamer` dreams lucidly.
///
/// The system instruction is the initial influence on how the dream should progress, defining an
/// AI assistant that knows what it is, how it works, and what it is supposed to do.
#[allow(dead_code)] // TODO: remove this
pub struct Dreamer {
    /// The model used to generate responses.
    pub(crate) model: OpenAIModel,

    /// The memories of the assistant.
    pub(crate) memories: Memories,

    /// The thread of conversation.
    pub(crate) thread: Thread,

    /// The internal tools the assistant has access to.
    pub(crate) internal_tools: Vec<Box<dyn Tool + Send + Sync>>,

    /// The provided tools the assistant has access to.
    pub(crate) provided_tools: Vec<Box<dyn Tool + Send + Sync>>,

    /// Whether the agent is idle.
    pub(crate) idle: bool,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Dreamer {
    /// Creates a new `Dreamer` with the given system instruction.
    pub fn new() -> Self {
        Self {
            model: OpenAIModel::default(),
            memories: Memories::new(),
            thread: Thread::new(DREAMER_SYSTEM_INSTRUCTION),
            internal_tools: Vec::new(),
            provided_tools: Vec::new(),
            idle: true,
        }
    }

    /// Creates a new `Dreamer` builder.
    pub fn builder() -> DreamerBuilder {
        DreamerBuilder::default()
    }

    /// Runs the agent.
    pub fn run(mut self, mut channels: AgentSideChannels) -> JoinHandle<DreamerResult<()>> {
        tokio::spawn(async move {
            loop {
                if self.idle {
                    if let Some(message) = channels.message_rx.recv().await {
                        self.handle_incoming_message(message)?;
                    }

                    continue;
                }

                tokio::select! {
                    response = self.call() => {
                        self.handle_model_response(response?, &channels.metrics_tx)?;
                    }
                    message = channels.message_rx.recv() => if let Some(message) = message {
                        self.handle_incoming_message(message)?;
                    }
                }
            }
        })
    }
}

impl Dreamer {
    /// Handles the model response.
    fn handle_model_response(
        &mut self,
        response: String,
        metrics_tx: &mpsc::UnboundedSender<Metrics>,
    ) -> DreamerResult<()> {
        // Parse the response into a ThreadMessage.
        let message: ThreadMessage = response.parse()?;

        // Handle the message based on its type.
        match &message {
            ThreadMessage::Thought(thought) => self.handle_thought(thought)?,
            ThreadMessage::Action(action) => self.handle_action(action)?,
            _ => return Err(DreamerError::InvalidResponseMessage(message)),
        }

        // Send metrics to the metrics channel.
        metrics_tx.send(Metrics::ThreadMessage(message.clone()))?;

        // Add message to the thread.
        self.thread.push_message(message);

        Ok(())
    }

    /// Handles the incoming message.
    fn handle_incoming_message(&mut self, _message: String) -> DreamerResult<()> {
        println!("message: {}", _message);
        Ok(())
    }

    /// Handles the thought message.
    fn handle_thought(&mut self, thought: &ThoughtMessage) -> DreamerResult<()> {
        if !thought.is_incomplete() {
            self.make_idle();
        } else {
            self.make_busy();
        }

        Ok(())
    }

    /// Handles the action message.
    fn handle_action(&mut self, action: &ActionMessage) -> DreamerResult<()> {
        info!("TODO: action: {}", action.get_full_content());
        // TODO: parse and send the action to the outside world
        Ok(())
    }

    /// Calls the model by sending the thread to the model and receiving a response.
    async fn call(&self) -> DreamerResult<String> {
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
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for Dreamer {
    fn default() -> Self {
        Self::new()
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
