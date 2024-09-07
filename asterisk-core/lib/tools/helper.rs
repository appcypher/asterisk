use serde_json::{Map, Value};

use super::ToolResult;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

/// Parses the tool invocation.
pub fn parse_tool(tool: &str) -> ToolResult<(String, Map<String, Value>)> {
    let json: Value = serde_json::from_str(tool)?;
    let tool_name = json["name"].as_str().unwrap().to_string();
    let tool_args = json["args"].as_object().unwrap().clone();

    Ok((tool_name, tool_args))
}
