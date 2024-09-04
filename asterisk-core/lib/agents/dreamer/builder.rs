use crate::models::openai::OpenAIModel;

use super::{AgentSideChannels, Dreamer, Memories, Thread, Tool};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The builder for a `Dreamer`.
pub struct DreamerBuilder<C> {
    /// The channels for the dreamer.
    channels: C,

    /// The tools for the dreamer.
    tools: Vec<Box<dyn Tool>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<C> DreamerBuilder<C> {
    /// Sets the channels for the dreamer.
    pub fn channels(self, channels: AgentSideChannels) -> DreamerBuilder<AgentSideChannels> {
        DreamerBuilder {
            channels,
            tools: self.tools,
        }
    }

    /// Sets the tools for the dreamer.
    pub fn tools(self, tools: impl IntoIterator<Item = Box<dyn Tool>>) -> DreamerBuilder<C> {
        DreamerBuilder {
            channels: self.channels,
            tools: tools.into_iter().collect(),
        }
    }
}

impl DreamerBuilder<AgentSideChannels> {
    /// Builds the dreamer.
    pub fn build(self) -> Dreamer {
        Dreamer {
            model: OpenAIModel::default(),
            memories: Memories::new(),
            thread: Thread::new(),
            internal_tools: Vec::new(), // TODO: Implement internal tools
            provided_tools: self.tools,
            channels: self.channels,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for DreamerBuilder<()> {
    fn default() -> Self {
        Self {
            channels: (),
            tools: Vec::new(),
        }
    }
}
