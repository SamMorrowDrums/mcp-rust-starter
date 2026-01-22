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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Weather data returned by the `get_weather` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(title = "Weather")]
pub struct Weather {
    #[schemars(title = "Location")]
    pub location: String,
    #[schemars(title = "Temperature")]
    pub temperature: i32,
    #[schemars(title = "Unit")]
    pub unit: String,
    #[schemars(title = "Conditions")]
    pub conditions: String,
    #[schemars(title = "Humidity")]
    pub humidity: i32,
}

/// Response from the hello tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(title = "HelloResponse")]
pub struct HelloResponse {
    #[schemars(title = "Message")]
    pub message: String,
}

/// Response from the `long_task` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(title = "LongTaskResponse")]
pub struct LongTaskResponse {
    #[schemars(title = "Result")]
    pub result: String,
}

/// Response from the `load_bonus_tool` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(title = "LoadBonusToolResponse")]
pub struct LoadBonusToolResponse {
    #[schemars(title = "Note")]
    pub note: String,
    #[schemars(title = "Description")]
    pub description: String,
    #[schemars(title = "Usage")]
    pub usage: String,
    #[schemars(title = "Limitation")]
    pub limitation: String,
}

/// Response from the `ask_llm` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(title = "AskLlmResponse")]
pub struct AskLlmResponse {
    #[schemars(title = "Note")]
    pub note: String,
    #[schemars(title = "Prompt")]
    pub prompt: String,
    #[schemars(title = "Max Tokens")]
    pub max_tokens: i32,
    #[schemars(title = "Description")]
    pub description: String,
    #[schemars(title = "Usage")]
    pub usage: String,
    #[schemars(title = "Sampling Support")]
    pub sampling_support: String,
}

/// Response from the `confirm_action` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(title = "ConfirmActionResponse")]
pub struct ConfirmActionResponse {
    #[schemars(title = "Note")]
    pub note: String,
    #[schemars(title = "Action")]
    pub action: String,
    #[schemars(title = "Destructive")]
    pub destructive: bool,
    #[schemars(title = "Description")]
    pub description: String,
    #[schemars(title = "Usage")]
    pub usage: String,
    #[schemars(title = "Elicitation Support")]
    pub elicitation_support: String,
}

/// Response from the `get_feedback` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(title = "GetFeedbackResponse")]
pub struct GetFeedbackResponse {
    #[schemars(title = "Note")]
    pub note: String,
    #[schemars(title = "Question")]
    pub question: String,
    #[schemars(title = "Description")]
    pub description: String,
    #[schemars(title = "Usage")]
    pub usage: String,
    #[schemars(title = "Elicitation Support")]
    pub elicitation_support: String,
}

// Note: Tool implementations are in lib.rs using the #[tool_router] macro.
// The rmcp SDK handles tool parameter parsing automatically.
// See the tool methods in McpServer impl block in lib.rs.
