//! This module contains the tools for the dreamer agent.

use serde_json::{Map, Value};

use super::{Tool, ToolError, ToolResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tool for scanning the user message.
#[derive(Default)]
pub struct MessageBox {
    message: Option<String>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl MessageBox {
    /// Updates the message.
    pub fn update_message(&mut self, message: String) {
        self.message = Some(message);
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Tool for MessageBox {
    fn name(&self) -> String {
        "message_box".to_string()
    }

    fn description(&self) -> String {
        "This tool is used to read the user message".to_string()
    }

    fn execute(&self, _: Map<String, Value>) -> ToolResult<String> {
        match &self.message {
            Some(message) => Ok(message.clone()),
            None => Err(ToolError::custom(anyhow::anyhow!("Message is not set"))),
        }
    }
}
