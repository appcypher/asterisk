use std::collections::HashMap;

use crate::{
    models::openai::{ModelType, OpenAIModel},
    tools::Tool,
};

use super::Dreamer;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The builder for a `Dreamer`.
#[derive(Default)]
pub struct DreamerBuilder {
    /// The tools for the dreamer.
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,

    /// The type of model to use.
    model: Option<ModelType>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl DreamerBuilder {
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
    pub fn model(self, model: ModelType) -> Self {
        DreamerBuilder {
            model: Some(model),
            ..self
        }
    }

    /// Builds the dreamer.
    pub fn build(self) -> Dreamer {
        Dreamer {
            provided_tools: self.tools,
            model: OpenAIModel::builder()
                .model(self.model.unwrap_or_default())
                .build(),
            ..Default::default()
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------
