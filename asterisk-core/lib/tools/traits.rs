use serde_json::{Map, Value};

use super::ToolResult;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// A tool that an agent can use.
pub trait Tool {
    /// Returns the name of the tool.
    fn name(&self) -> String;

    /// Returns the description of the tool.
    fn description(&self) -> String;

    /// Executes the tool.
    fn execute(&self, input: Map<String, Value>) -> ToolResult<String>;
}
