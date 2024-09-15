//! This module contains the tools for the dreamer agent.

use std::path::Path;

use rusqlite::Connection;
use serde_json::{Map, Value};

use super::{Tool, ToolResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The tool for managing memories.
#[allow(dead_code)] // TODO: remove this
pub struct Memories {
    /// The database connection.
    database: Connection,
}

/// A memory entry.
#[allow(dead_code)] // TODO: remove this
pub struct MemoryEntry {
    /// The name of the memory.
    name: String,

    /// The value of the memory.
    value: String,

    /// The importance of the memory.
    importance: u64,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Memories {
    /// Creates a new memory tool with an in-memory database.
    pub fn new() -> ToolResult<Self> {
        Ok(Self {
            database: Connection::open_in_memory()?,
        })
    }

    /// Creates a new memory tool with a database at the given path.
    pub fn with_path(path: impl AsRef<Path>) -> ToolResult<Self> {
        Ok(Self {
            database: Connection::open(path)?,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Tool for Memories {
    fn name(&self) -> String {
        "knowledge_base".to_string()
    }

    fn description(&self) -> String {
        "This tool is used to manage the knowledge base".to_string()
    }

    fn execute(&self, _: Map<String, Value>) -> ToolResult<String> {
        Ok("".to_string())
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_memory() -> anyhow::Result<()> {
        let _memory = Memories::new()?;

        // let result = memory.execute(Map::new())?;
        // assert_eq!(result, "".to_string());

        Ok(())
    }
}
