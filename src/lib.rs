//! # MCP Rust Starter
//!
//! A feature-complete MCP server demonstrating tools, resources, and prompts
//! using the official [rmcp](https://crates.io/crates/rmcp) SDK.
//!
//! ## Features
//!
//! - **Tools**: Callable functions with structured input/output and annotations
//! - **Resources**: Static and dynamic data exposed to clients
//! - **Prompts**: Pre-configured message templates
//!
//! ## Documentation
//!
//! - [MCP Specification](https://modelcontextprotocol.io/)
//! - [rmcp SDK](https://docs.rs/rmcp)

pub mod icons;
pub mod prompts;
pub mod resources;
pub mod tools;

use std::collections::HashMap;

use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, GetPromptResult, Implementation, ListPromptsResult,
        ListResourcesResult, ReadResourceResult, ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    tool, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Re-export types for convenience
pub use tools::Weather;

// Tool parameter structures with JSON Schema generation

/// Parameters for the `hello` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct HelloParams {
    /// Name of the person to greet
    #[schemars(description = "Name of the person to greet")]
    pub name: String,
}

/// Parameters for the `get_weather` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetWeatherParams {
    /// City name to get weather for
    #[schemars(description = "City name to get weather for")]
    pub city: String,
}

/// Parameters for the `long_task` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LongTaskParams {
    /// Name for this task
    #[schemars(description = "Name for this task")]
    #[serde(rename = "taskName")]
    pub task_name: String,

    /// Number of steps to simulate
    #[schemars(description = "Number of steps to simulate", default = "default_steps")]
    #[serde(default = "default_steps")]
    pub steps: i32,
}

const fn default_steps() -> i32 {
    5
}

/// Parameters for the `ask_llm` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AskLlmParams {
    /// The question or prompt to send to the LLM
    #[schemars(description = "The question or prompt to send to the LLM")]
    pub prompt: String,

    /// Maximum tokens in response
    #[schemars(
        description = "Maximum tokens in response",
        default = "default_max_tokens"
    )]
    #[serde(rename = "maxTokens", default = "default_max_tokens")]
    pub max_tokens: i32,
}

const fn default_max_tokens() -> i32 {
    100
}

/// Parameters for the `confirm_action` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ConfirmActionParams {
    /// Description of the action to confirm
    #[schemars(description = "Description of the action to confirm")]
    pub action: String,

    /// Whether the action is destructive
    #[schemars(
        description = "Whether the action is destructive",
        default = "default_destructive"
    )]
    #[serde(default = "default_destructive")]
    pub destructive: bool,
}

const fn default_destructive() -> bool {
    false
}

/// Parameters for the `get_feedback` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetFeedbackParams {
    /// The question to ask the user
    #[schemars(description = "The question to ask the user")]
    pub question: String,
}

/// Server instructions for AI assistants.
pub const SERVER_INSTRUCTIONS: &str = r"# MCP Rust Starter Server

A demonstration MCP server showcasing Rust SDK capabilities.

## Available Tools

### Core Tools
- **hello**: Greet a person by name
- **get_weather**: Get weather information for a city
- **long_task**: Simulate a long-running task with progress updates

### Advanced Tools
- **load_bonus_tool**: Dynamically register a new bonus tool
- **ask_llm**: Ask the connected LLM a question using sampling
- **confirm_action**: Request user confirmation before proceeding
- **get_feedback**: Request feedback from the user

## Available Resources

- **about://server**: Server information
- **doc://example**: Example markdown document

## Available Prompts

- **greet**: Generates a personalized greeting
- **code_review**: Structured code review prompt

## Recommended Workflows

1. **Testing Connection**: Call `hello` with a name to verify the server is responding
2. **Weather Demo**: Call `get_weather` with a location to see structured output
3. **Long Task**: Use `long_task` to see progress updates

## Tool Annotations

All tools include annotations indicating:
- title: Human-readable name for display
- read_only_hint: Whether they modify state
- idempotent_hint: If they're safe to retry
- open_world_hint: Whether they access external systems

Use these hints to make informed decisions about tool usage.";

/// The main MCP server implementing all handlers.
#[derive(Clone)]
pub struct McpServer {
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl McpServer {
    /// Create a new MCP server instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

/// Tool implementations using rmcp macros.
/// Each tool has proper annotations including a human-readable title.
#[tool_router]
impl McpServer {
    /// A friendly greeting tool that says hello.
    #[tool(
        name = "hello",
        description = "Say hello to a person",
        annotations(
            title = "Say Hello",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        ),
        icons = icons::waving_hand()
    )]
    async fn hello(&self, params: Parameters<HelloParams>) -> Result<CallToolResult, McpError> {
        let message = format!(
            "Hello, {}! Welcome to the MCP Rust Starter Server.",
            params.0.name
        );
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    /// Get current weather (simulated data).
    #[tool(
        name = "get_weather",
        description = "Get the current weather for a city",
        annotations(
            title = "Get Weather",
            read_only_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        ),
        icons = icons::sun_behind_cloud()
    )]
    async fn get_weather(
        &self,
        params: Parameters<GetWeatherParams>,
    ) -> Result<CallToolResult, McpError> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let conditions = ["sunny", "cloudy", "rainy", "windy"];

        let weather = tools::Weather {
            location: params.0.city.clone(),
            temperature: rng.gen_range(15..35),
            unit: "celsius".to_string(),
            conditions: conditions[rng.gen_range(0..conditions.len())].to_string(),
            humidity: rng.gen_range(40..80),
        };

        let json_str = serde_json::to_string_pretty(&weather)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }

    /// Simulate a long-running task with progress updates.
    #[tool(
        name = "long_task",
        description = "Simulate a long-running task with progress updates",
        annotations(
            title = "Long Task",
            read_only_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        ),
        icons = icons::hourglass()
    )]
    async fn long_task(
        &self,
        params: Parameters<LongTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        use std::fmt::Write;

        // Simulate a long task with progress updates
        let steps = params.0.steps;
        let task_name = &params.0.task_name;

        let mut result = format!("Starting task '{task_name}' with {steps} steps:\n");

        for i in 1..=steps {
            // In a real implementation, this would send progress notifications
            writeln!(&mut result, "Step {i}/{steps} completed").unwrap();
            // Simulate work
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        write!(&mut result, "Task '{task_name}' completed successfully!").unwrap();

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    /// Dynamically register a new bonus tool.
    #[tool(
        name = "load_bonus_tool",
        description = "Dynamically register a new bonus tool",
        annotations(
            title = "Load Bonus Tool",
            read_only_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        ),
        icons = icons::package()
    )]
    async fn load_bonus_tool(&self) -> Result<CallToolResult, McpError> {
        // In a full implementation, this would dynamically register a new tool
        let result = serde_json::json!({
            "note": "This tool demonstrates dynamic tool loading capability.",
            "description": "In a full implementation, this would register a new 'bonus_tool' that clients can discover and call.",
            "usage": "Call this tool to trigger dynamic tool registration.",
            "limitation": "rmcp SDK does not currently support runtime tool registration, so this is a placeholder."
        });

        let json_str = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }

    /// Ask the connected LLM a question using sampling.
    #[tool(
        name = "ask_llm",
        description = "Ask the connected LLM a question using sampling",
        annotations(
            title = "Ask LLM",
            read_only_hint = true,
            idempotent_hint = false,
            open_world_hint = true
        ),
        icons = icons::thought_balloon()
    )]
    async fn ask_llm(&self, params: Parameters<AskLlmParams>) -> Result<CallToolResult, McpError> {
        // In a full implementation, this would use the MCP sampling feature to ask the LLM
        let result = serde_json::json!({
            "note": "This tool demonstrates MCP sampling capability.",
            "prompt": params.0.prompt,
            "max_tokens": params.0.max_tokens,
            "description": "In a full implementation, this would use context.peer().create_message() to request sampling from the connected LLM.",
            "usage": "Call with a 'prompt' parameter to ask a question.",
            "sampling_support": "Requires rmcp 'sampling' feature and client support."
        });

        let json_str = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }

    /// Request user confirmation before proceeding with an action.
    /// Demonstrates elicitation capability for user interaction.
    #[tool(
        name = "confirm_action",
        description = "Request user confirmation before proceeding",
        annotations(
            title = "Confirm Action",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        ),
        icons = icons::question()
    )]
    async fn confirm_action(
        &self,
        params: Parameters<ConfirmActionParams>,
    ) -> Result<CallToolResult, McpError> {
        // In a full implementation, this would use:
        // let result = context.peer().elicit::<ConfirmSchema>("Confirm action?").await;
        let result = serde_json::json!({
            "note": "This tool demonstrates MCP elicitation capability.",
            "action": params.0.action,
            "destructive": params.0.destructive,
            "description": "In a full implementation, this would request user confirmation via the MCP elicitation protocol.",
            "usage": "Call with an 'action' parameter describing what needs confirmation.",
            "elicitation_support": "Requires rmcp 'elicitation' feature and client support."
        });

        let json_str = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }

    /// Collect feedback from the user.
    /// Demonstrates elicitation with text input schema.
    #[tool(
        name = "get_feedback",
        description = "Request feedback from the user",
        annotations(
            title = "Get Feedback",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        ),
        icons = icons::speech()
    )]
    async fn get_feedback(
        &self,
        params: Parameters<GetFeedbackParams>,
    ) -> Result<CallToolResult, McpError> {
        // In a full implementation, this would use:
        // let feedback = context.peer().elicit::<FeedbackSchema>("Please provide feedback").await;
        let result = serde_json::json!({
            "note": "This tool demonstrates MCP elicitation capability for text input.",
            "question": params.0.question,
            "description": "In a full implementation, this would request feedback via the MCP elicitation protocol.",
            "usage": "Call with a 'question' parameter describing what feedback is needed.",
            "elicitation_support": "Requires rmcp 'elicitation' feature and client support."
        });

        let json_str = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }
}

/// Server handler implementation for MCP protocol.
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "mcp-rust-starter".into(),
                version: "1.0.0".into(),
                title: Some("MCP Rust Starter".into()),
                icons: None,
                website_url: None,
            },
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            instructions: Some(SERVER_INSTRUCTIONS.into()),
            ..Default::default()
        }
    }

    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, McpError> {
        Ok(rmcp::model::ListToolsResult {
            tools: self.tool_router.list_all(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let tool_context =
            rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        self.tool_router.call(tool_context).await
    }

    async fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        resources::list_resources()
    }

    async fn list_resource_templates(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListResourceTemplatesResult, McpError> {
        resources::list_resource_templates()
    }

    async fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        resources::read_resource(&request.uri)
    }

    async fn list_prompts(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        prompts::list_prompts()
    }

    async fn get_prompt(
        &self,
        request: rmcp::model::GetPromptRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        // Convert serde_json::Map to HashMap<String, String>
        let arguments = request.arguments.map(|map| {
            map.into_iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k, s.to_string())))
                .collect::<HashMap<String, String>>()
        });
        prompts::get_prompt(&request.name, arguments)
    }
}
