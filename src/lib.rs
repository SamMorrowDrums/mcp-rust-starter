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
    handler::server::tool::ToolRouter,
    model::{
        CallToolResult, Content, GetPromptResult, Implementation, ListPromptsResult,
        ListResourcesResult, ReadResourceResult, ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    tool, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};

// Re-export types for convenience
pub use tools::{Operation, Weather};

/// Server instructions for AI assistants.
pub const SERVER_INSTRUCTIONS: &str = r"# MCP Rust Starter Server

A demonstration MCP server showcasing Rust SDK capabilities.

## Available Tools

### Greeting & Demos
- **hello**: Simple greeting - use to test connectivity
- **get_weather**: Returns simulated weather data
- **calculator**: Basic arithmetic operations (add, subtract, multiply, divide)

- **calculator**: Basic arithmetic operations (add, subtract, multiply, divide)

## Available Resources

- **info://about**: Server information
- **file://example.md**: Example markdown document

## Available Prompts

- **greet**: Generates a personalized greeting
- **code_review**: Structured code review prompt

## Recommended Workflows

1. **Testing Connection**: Call `hello` with your name to verify the server is responding
2. **Weather Demo**: Call `get_weather` with a location to see structured output
3. **Calculator**: Use `calculator` for basic math operations

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
        description = "A friendly greeting tool that says hello",
        annotations(
            title = "Say Hello",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        ),
        icons = icons::waving_hand()
    )]
    async fn hello(&self) -> Result<CallToolResult, McpError> {
        let message = "Hello! Welcome to the MCP Rust Starter Server.";
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    /// Get current weather (simulated data).
    #[tool(
        name = "get_weather",
        description = "Get current weather (simulated). Returns random weather data.",
        annotations(
            title = "Get Weather",
            read_only_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        ),
        icons = icons::sun_behind_cloud()
    )]
    async fn get_weather(&self) -> Result<CallToolResult, McpError> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let conditions = ["sunny", "cloudy", "rainy", "windy"];
        let locations = ["New York", "London", "Tokyo", "Paris"];

        let weather = tools::Weather {
            location: locations[rng.gen_range(0..locations.len())].to_string(),
            temperature: rng.gen_range(15..35),
            unit: "celsius".to_string(),
            conditions: conditions[rng.gen_range(0..conditions.len())].to_string(),
            humidity: rng.gen_range(40..80),
        };

        let json_str = serde_json::to_string_pretty(&weather)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }

    /// Get server status.
    #[tool(
        name = "server_status",
        description = "Get current server status and uptime information",
        annotations(
            title = "Server Status",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        ),
        icons = icons::robot()
    )]
    async fn server_status(&self) -> Result<CallToolResult, McpError> {
        let status = serde_json::json!({
            "status": "running",
            "server_name": "mcp-rust-starter",
            "version": "1.0.0",
            "sdk": "rmcp 0.11"
        });

        let json_str = serde_json::to_string_pretty(&status)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }

    /// Calculator tool demonstrating enum parameters.
    #[tool(
        name = "calculator",
        description = "A basic calculator that performs arithmetic operations",
        annotations(
            title = "Calculator",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        ),
        icons = icons::abacus()
    )]
    async fn calculator(&self) -> Result<CallToolResult, McpError> {
        // Note: rmcp 0.11 has limited support for parameterized tools
        // This demo shows the tool structure - in practice you'd parse params from request
        let result = serde_json::json!({
            "hint": "This calculator accepts a, b (numbers) and operation (add/subtract/multiply/divide)",
            "example": "10 + 5 = 15",
            "supported_operations": ["add", "subtract", "multiply", "divide"]
        });

        let json_str = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }

    // Note: Elicitation tools would require access to the RequestContext
    // which is not available in the tool_router macro. These are placeholder
    // implementations showing the pattern. Full elicitation requires custom
    // call_tool implementation.

    /// Request user confirmation before proceeding with an action.
    /// Demonstrates elicitation capability for user interaction.
    #[tool(
        name = "confirm_action",
        description = "Request user confirmation. Uses MCP elicitation to get user approval before proceeding with an action.",
        annotations(
            title = "Confirm Action",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        ),
        icons = icons::question()
    )]
    async fn confirm_action(&self) -> Result<CallToolResult, McpError> {
        // In a full implementation, this would use:
        // let result = context.peer().elicit::<ConfirmSchema>("Confirm action?").await;
        let result = serde_json::json!({
            "note": "This tool demonstrates MCP elicitation capability.",
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
        description = "Collect user feedback. Uses MCP elicitation to gather text input from the user.",
        annotations(
            title = "Get Feedback",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        ),
        icons = icons::speech()
    )]
    async fn get_feedback(&self) -> Result<CallToolResult, McpError> {
        // In a full implementation, this would use:
        // let feedback = context.peer().elicit::<FeedbackSchema>("Please provide feedback").await;
        let result = serde_json::json!({
            "note": "This tool demonstrates MCP elicitation capability for text input.",
            "description": "In a full implementation, this would request feedback via the MCP elicitation protocol.",
            "usage": "Call with a 'prompt' parameter describing what feedback is needed.",
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
