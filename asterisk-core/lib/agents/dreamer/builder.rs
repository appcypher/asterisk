use super::{Dreamer, Tool};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The builder for a `Dreamer`.
#[derive(Default)]
pub struct DreamerBuilder {
    /// The tools for the dreamer.
    tools: Vec<Box<dyn Tool + Send + Sync>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl DreamerBuilder {
    /// Sets the tools for the dreamer.
    pub fn tools(self, tools: impl IntoIterator<Item = Box<dyn Tool + Send + Sync>>) -> Self {
        DreamerBuilder {
            tools: tools.into_iter().collect(),
        }
    }

    /// Builds the dreamer.
    pub fn build(self) -> Dreamer {
        Dreamer {
            internal_tools: self.tools,
            ..Default::default()
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------
