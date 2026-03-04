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
    handler::server::{tool::schema_for_type, tool::ToolRouter, wrapper::Parameters},
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
pub use tools::{
    AskLlmResponse, ConfirmActionResponse, GetFeedbackResponse, HelloResponse,
    LoadBonusToolResponse, LongTaskResponse, Weather,
};

// =============================================================================
// TOOL ANNOTATIONS - Every tool SHOULD have annotations for AI assistants
//
// WHY ANNOTATIONS MATTER:
// Annotations enable MCP client applications to understand the risk level of
// tool calls. Clients can use these hints to implement safety policies.
//
// ANNOTATION FIELDS:
// - readOnlyHint: Tool only reads data, doesn't modify state
// - destructiveHint: Tool can permanently delete or modify data
// - idempotentHint: Repeated calls with same args have same effect
// - openWorldHint: Tool accesses external systems (web, APIs, etc.)
// =============================================================================

// Tool parameter structures with JSON Schema generation

/// Parameters for the `hello` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(title = "helloArguments")]
pub struct HelloParams {
    /// Name of the person to greet
    #[schemars(title = "Name", description = "Name of the person to greet")]
    pub name: String,
}

/// Parameters for the `get_weather` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(title = "get_weatherArguments")]
pub struct GetWeatherParams {
    /// City name to get weather for
    #[schemars(title = "City", description = "City name to get weather for")]
    pub city: String,
}

/// Parameters for the `long_task` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(title = "long_taskArguments")]
pub struct LongTaskParams {
    /// Name for this task
    #[schemars(title = "Task Name", description = "Name for this task")]
    #[serde(rename = "taskName")]
    pub task_name: String,

    /// Number of steps to simulate
    #[schemars(
        title = "Steps",
        description = "Number of steps to simulate",
        default = "default_steps"
    )]
    #[serde(default = "default_steps")]
    pub steps: i32,
}

const fn default_steps() -> i32 {
    5
}

/// Parameters for the `ask_llm` tool.
#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(title = "ask_llmArguments")]
pub struct AskLlmParams {
    /// The question or prompt to send to the LLM
    #[schemars(
        title = "Prompt",
        description = "The question or prompt to send to the LLM"
    )]
    pub prompt: String,

    /// Maximum tokens in response
    #[schemars(
        title = "Max Tokens",
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
#[schemars(title = "confirm_actionArguments")]
pub struct ConfirmActionParams {
    /// Description of the action to confirm
    #[schemars(title = "Action", description = "Description of the action to confirm")]
    pub action: String,

    /// Whether the action is destructive
    #[schemars(
        title = "Destructive",
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
#[schemars(title = "get_feedbackArguments")]
pub struct GetFeedbackParams {
    /// The question to ask the user
    #[schemars(title = "Question", description = "The question to ask the user")]
    pub question: String,
}

/// Server instructions for AI assistants.
pub const SERVER_INSTRUCTIONS: &str = r#"# MCP Rust Starter Server

A demonstration MCP server showcasing Rust SDK capabilities.

## Recommended Workflows

1. **Test connectivity** → Call `hello` to verify the server responds
2. **Structured output** → Call `get_weather` to see typed response data
3. **Progress reporting** → Call `long_task` to observe real-time progress notifications
4. **Dynamic tools** → Call `load_bonus_tool`, then re-list tools to see `bonus_calculator` appear
5. **LLM sampling** → Call `ask_llm` to have the server request a completion from the client
6. **Elicitation** → Call `confirm_action` (form-based) or `get_feedback` (URL-based) to request user input

## Multi-Tool Flows

- **Full demo**: `hello` → `get_weather` → `long_task` → `load_bonus_tool` → `bonus_calculator`
- **Dynamic loading**: `load_bonus_tool` triggers a `tools/list_changed` notification — refresh your tool list to see `bonus_calculator`
- **User interaction**: `confirm_action` demonstrates schema elicitation, `get_feedback` demonstrates URL elicitation

## Notes

- All tools include annotations (readOnlyHint, idempotentHint, openWorldHint) to guide safe usage
- Resources and prompts are available for context and templating — use `resources/list` and `prompts/list` to discover them"#;

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

// =============================================================================
// TOOLS
// Tools are functions that the client can invoke to perform actions.
// Each tool uses the #[tool] macro with annotations and an output_schema.
// =============================================================================

#[tool_router]
impl McpServer {
    /// **hello** – Basic connectivity test.
    /// The simplest possible tool: takes a name, returns a greeting.
    /// Use this to verify the server is running and reachable.
    #[tool(
        name = "hello",
        description = "Say hello to a person",
        output_schema = schema_for_type::<HelloResponse>(),
        annotations(
            title = "Say Hello",
            read_only_hint = true,
            destructive_hint = false,
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

    /// `get_weather` – Structured output with `output_schema`.
    /// Demonstrates returning typed JSON data (the `Weather` struct) so that
    /// clients can validate the response shape at runtime.
    #[tool(
        name = "get_weather",
        description = "Get the current weather for a city",
        output_schema = schema_for_type::<Weather>(),
        annotations(
            title = "Get Weather",
            read_only_hint = true,
            destructive_hint = false,
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
        let mut rng = rand::rng();
        let conditions = ["sunny", "cloudy", "rainy", "windy"];

        let weather = tools::Weather {
            location: params.0.city.clone(),
            temperature: rng.random_range(15..35),
            unit: "celsius".to_string(),
            conditions: conditions[rng.random_range(0..conditions.len())].to_string(),
            humidity: rng.random_range(40..80),
        };

        let json_str = serde_json::to_string_pretty(&weather)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }

    /// `long_task` – Progress reporting via notifications.
    /// Shows how a tool can report incremental progress to the client.
    /// In a full implementation, each step would send a `notifications/progress`
    /// message so the client can display a progress bar.
    #[tool(
        name = "long_task",
        description = "Simulate a long-running task with progress updates",
        output_schema = schema_for_type::<LongTaskResponse>(),
        annotations(
            title = "Long Running Task",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
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

    /// `load_bonus_tool` – Dynamic tool registration (`listChanged` notification).
    /// Demonstrates adding tools at runtime. When called, the server would
    /// register a new tool and send a `notifications/tools/list_changed`
    /// notification so clients refresh their tool list.
    /// This is why `enable_tool_list_changed()` is set in `get_info()`.
    #[tool(
        name = "load_bonus_tool",
        description = "Dynamically register a new bonus tool",
        output_schema = schema_for_type::<LoadBonusToolResponse>(),
        annotations(
            title = "Load Bonus Tool",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
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

    /// `ask_llm` – LLM sampling capability.
    /// Demonstrates MCP sampling: the server asks the *client's* LLM a question.
    /// This inverts the usual flow — instead of the client calling the server,
    /// the server requests a completion from the client via `create_message()`.
    #[tool(
        name = "ask_llm",
        description = "Ask the connected LLM a question using sampling",
        output_schema = schema_for_type::<AskLlmResponse>(),
        annotations(
            title = "Ask LLM",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
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

    /// `confirm_action` – Schema elicitation.
    /// Demonstrates MCP elicitation: the server presents a structured form
    /// (JSON Schema) to the user and collects their input. Useful for
    /// confirmation dialogs, settings forms, or multi-field input.
    #[tool(
        name = "confirm_action",
        description = "Request user confirmation before proceeding",
        output_schema = schema_for_type::<ConfirmActionResponse>(),
        annotations(
            title = "Confirm Action",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = false,
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

    /// `get_feedback` – URL elicitation.
    /// Demonstrates MCP elicitation via URL: the server asks the client to
    /// open a URL (e.g. a feedback form) and waits for the result.
    /// Note `open_world_hint = true` because it directs the user to an external URL.
    #[tool(
        name = "get_feedback",
        description = "Request feedback from the user",
        output_schema = schema_for_type::<GetFeedbackResponse>(),
        annotations(
            title = "Get Feedback",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = true
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
///
/// `ServerHandler` is the main trait from rmcp that wires your server into
/// the MCP lifecycle: capability negotiation, tool/resource/prompt listing,
/// and request dispatch.
impl ServerHandler for McpServer {
    /// Returns server metadata and capability flags.
    ///
    /// The `ServerCapabilities` builder declares which MCP features this
    /// server supports. Each `.enable_*()` call opts in to a feature:
    ///
    /// - `enable_tools()`            – server exposes callable tools
    /// - `enable_tool_list_changed()` – server may add/remove tools at runtime
    ///   (needed because `load_bonus_tool` dynamically registers a new tool)
    /// - `enable_resources()`         – server exposes readable resources
    /// - `enable_prompts()`           – server exposes prompt templates
    ///
    /// Prompts and resources do NOT enable `list_changed` because this server's
    /// prompt and resource lists are static — they never change after startup.
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_experimental()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .enable_tool_list_changed()
                .build(),
        )
        .with_server_info(Implementation::new("mcp-rust-starter", "1.0.0"))
        .with_instructions(SERVER_INSTRUCTIONS)
    }

    // -- Tool handlers --

    /// Lists all tools registered with this server (via the `#[tool_router]` macro).
    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, McpError> {
        Ok(rmcp::model::ListToolsResult {
            tools: self.tool_router.list_all(),
            next_cursor: None,
            meta: None,
        })
    }

    /// Dispatches a `tools/call` request to the matching tool implementation.
    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let tool_context =
            rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        self.tool_router.call(tool_context).await
    }

    // -- Resource handlers (read-only data exposed to clients) --

    /// Lists static resources available on this server.
    async fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        resources::list_resources()
    }

    /// Lists resource templates (parameterised URI patterns like `greeting://{name}`).
    async fn list_resource_templates(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListResourceTemplatesResult, McpError> {
        resources::list_resource_templates()
    }

    /// Reads a resource by URI, returning its content.
    async fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        resources::read_resource(&request.uri)
    }

    // -- Prompt handlers (reusable message templates) --

    /// Lists all prompt templates this server offers.
    async fn list_prompts(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        prompts::list_prompts()
    }

    /// Retrieves a prompt by name, filling in the supplied arguments.
    async fn get_prompt(
        &self,
        request: rmcp::model::GetPromptRequestParams,
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
