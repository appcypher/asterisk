use std::collections::HashMap;

use crate::{
    models::TextModel,
    tools::{message_box::MessageBox, Tool},
};

use super::{Dreamer, Memories, Thread, DREAMER_SYSTEM_INSTRUCTION};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The builder for a `Dreamer`.
pub struct DreamerBuilder<M> {
    /// The tools for the dreamer.
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,

    /// The model for the dreamer.
    model: M,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<M> DreamerBuilder<M> {
    /// Sets the tools for the dreamer.
    pub fn tools(
        self,
        tools: impl IntoIterator<Item = (String, Box<dyn Tool + Send + Sync>)>,
    ) -> Self {
        DreamerBuilder {
            tools: tools.into_iter().collect(),
            ..self
        }
    }

    /// Sets the model for the dreamer.
    pub fn model<N: TextModel>(self, model: N) -> DreamerBuilder<N> {
        DreamerBuilder {
            tools: self.tools,
            model,
        }
    }
}

impl<M: TextModel> DreamerBuilder<M> {
    /// Builds the dreamer.
    pub fn build(self) -> Dreamer<M> {
        Dreamer {
            provided_tools: self.tools,
            model: self.model,
            memories: Memories::new(),
            thread: Thread::new(DREAMER_SYSTEM_INSTRUCTION),
            message_box: MessageBox::default(),
            idle: true,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for DreamerBuilder<()> {
    fn default() -> Self {
        DreamerBuilder {
            tools: HashMap::new(),
            model: (),
        }
    }
}
