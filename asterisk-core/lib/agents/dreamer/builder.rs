use std::collections::HashMap;

use crate::{
    models::TextModel,
    tools::{inbox::Inbox, Tool},
};

use super::{Dreamer, Thread, DREAMER_SYSTEM_INSTRUCTION};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The builder for a `Dreamer`.
pub struct DreamerBuilder<M> {
    /// The tools for the dreamer.
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,

    /// The model for the dreamer.
    model: M,

    /// The system instruction for the dreamer.
    system_instruction: Option<String>,
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
            model,
            tools: self.tools,
            system_instruction: self.system_instruction,
        }
    }

    /// Sets the system instruction for the dreamer.
    pub fn system_instruction(self, system_instruction: String) -> Self {
        DreamerBuilder {
            system_instruction: Some(system_instruction),
            ..self
        }
    }
}

impl<M: TextModel> DreamerBuilder<M> {
    /// Builds the dreamer.
    pub fn build(self) -> Dreamer<M> {
        Dreamer {
            provided_tools: self.tools,
            model: self.model,
            thread: Thread::new(
                self.system_instruction
                    .unwrap_or(DREAMER_SYSTEM_INSTRUCTION.to_string()),
            ),
            inbox: Inbox::default(),
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
            system_instruction: None,
        }
    }
}
