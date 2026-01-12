//! # MCP Rust Starter - Tools
//!
//! Tool definitions for the MCP server using rmcp SDK.
//! Tools are functions that the client can invoke to perform actions.
//!
//! ## Tool Annotations
//!
//! Each tool includes annotations that describe its behavior:
//! - `title`: Human-readable display name
//! - `read_only_hint`: Tool only reads data, doesn't modify state
//! - `destructive_hint`: Tool can permanently delete or modify data
//! - `idempotent_hint`: Repeated calls with same args have same effect
//! - `open_world_hint`: Tool accesses external systems (web, APIs, etc.)

use serde::{Deserialize, Serialize};

/// Weather data returned by the get_weather tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
    pub location: String,
    pub temperature: i32,
    pub unit: String,
    pub conditions: String,
    pub humidity: i32,
}

/// Calculator operations - demonstrates using enums in MCP tool schemas.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    /// Add two numbers
    Add,
    /// Subtract second from first  
    Subtract,
    /// Multiply two numbers
    Multiply,
    /// Divide first by second
    Divide,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "add"),
            Operation::Subtract => write!(f, "subtract"),
            Operation::Multiply => write!(f, "multiply"),
            Operation::Divide => write!(f, "divide"),
        }
    }
}

// Note: Tool implementations are in lib.rs using the #[tool_router] macro.
// The rmcp SDK handles tool parameter parsing automatically.
// See the tool methods in McpServer impl block in lib.rs.
