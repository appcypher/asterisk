//! First attempt at creating a reliable agent.
use crate::{agents::AgentResult, models::openai::OpenAIModel};

use super::{AgentSideChannels, DreamerBuilder, Memories, Thread, Tool};

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
#[allow(dead_code)] // TODO: Remove this
pub struct Dreamer {
    /// The model used to generate responses.
    pub(crate) model: OpenAIModel,

    /// The memories of the assistant.
    pub(crate) memories: Memories,

    /// The thread of conversation.
    pub(crate) thread: Thread,

    /// The internal tools the assistant has access to.
    pub(crate) internal_tools: Vec<Box<dyn Tool>>,

    /// The provided tools the assistant has access to.
    pub(crate) provided_tools: Vec<Box<dyn Tool>>,

    /// Asynchronous channels that the agent uses to interact with the outside world.
    pub(crate) channels: AgentSideChannels,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Dreamer {
    /// Creates a new `Dreamer` with the given system instruction.
    pub fn new(channels: AgentSideChannels) -> Self {
        Self {
            model: OpenAIModel::default(),
            memories: Memories::new(),
            thread: Thread::new(),
            internal_tools: Vec::new(),
            provided_tools: Vec::new(),
            channels,
        }
    }

    /// Starts the agent.
    pub async fn start(self) -> AgentResult<()> {
        Ok(())
    }

    /// Creates a new `Dreamer` builder.
    pub fn builder() -> DreamerBuilder<()> {
        DreamerBuilder::default()
    }
}

//--------------------------------------------------------------------------------------------------
// Constant
//--------------------------------------------------------------------------------------------------

/// The system instruction for the dreamer agent.
#[allow(dead_code)] // TODO: Remove this
const DREAMER_SYSTEM_INSTRUCTION: &str = include_str!("instruction.txt");

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_agent_dreamer() {
    //     // let agent = Dreamer::new();
    // }
}
