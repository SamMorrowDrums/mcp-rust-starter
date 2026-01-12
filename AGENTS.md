# AGENTS.md

This file provides context for AI coding agents working in this repository.

## Project Overview

**MCP Rust Starter** is a feature-complete Model Context Protocol (MCP) server template in Rust. It demonstrates all major MCP features including tools, resources, resource templates, prompts, sampling, progress updates, and dynamic tool loading.

**Purpose**: Workshop starter template for learning MCP server development.

## Technology Stack

- **Runtime**: Rust 1.75+ (2021 edition)
- **MCP SDK**: `mcp-server`, `mcp-core`, `mcp-macros`
- **Async Runtime**: Tokio
- **HTTP Server**: Axum
- **Serialization**: Serde + serde_json

## Project Structure

```
.
├── Cargo.toml                  # Package manifest
├── rust-toolchain.toml         # Rust toolchain pinning
├── rustfmt.toml                # Formatter configuration
├── src/
│   ├── lib.rs                  # Main server module (Router impl)
│   └── bin/
│       ├── stdio.rs            # stdio transport entrypoint
│       └── http.rs             # HTTP transport entrypoint
├── .vscode/
│   ├── mcp.json                # MCP server configuration
│   ├── tasks.json              # Build/run tasks
│   ├── launch.json             # Debug configurations
│   └── extensions.json
└── .devcontainer/
    └── devcontainer.json
```

## Build & Run Commands

```bash
# Build (debug)
cargo build

# Build (release - optimized)
cargo build --release

# Run server (stdio transport)
cargo run --bin mcp-rust-starter-stdio

# Run server (HTTP transport)
cargo run --bin mcp-rust-starter-http
# With custom port:
PORT=8080 cargo run --bin mcp-rust-starter-http
```

## Linting & Formatting

```bash
# Format code
cargo fmt

# Check formatting (CI mode)
cargo fmt --check

# Lint code
cargo clippy

# Lint with auto-fix
cargo clippy --fix --allow-dirty

# Full check
cargo check
```

## Testing

```bash
cargo test
```

## Key Files to Modify

- **Add/modify tools**: `src/lib.rs` → `list_tools()` and `call_tool()` methods
- **Add/modify resources**: `src/lib.rs` → `list_resources()` and `read_resource()` methods
- **Add/modify prompts**: `src/lib.rs` → `list_prompts()` and `get_prompt()` methods
- **Server capabilities**: `src/lib.rs` → `capabilities()` method
- **HTTP config**: `src/bin/http.rs`
- **Package metadata**: `Cargo.toml`

## MCP Features Implemented

| Feature | Location | Description |
|---------|----------|-------------|
| `hello` tool | `lib.rs` | Basic tool with annotations |
| `get_weather` tool | `lib.rs` | Structured JSON output |
| `ask_llm` tool | `lib.rs` | Sampling/LLM invocation |
| `long_task` tool | `lib.rs` | Progress updates |
| `load_bonus_tool` | `lib.rs` | Dynamic tool loading |
| Resources | `lib.rs` | Static `info://about`, `file://example.md` |
| Templates | `lib.rs` | `greeting://{name}`, `data://items/{id}` |
| Prompts | `lib.rs` | `greet`, `code_review` with arguments |

## Environment Variables

- `PORT` - HTTP server port (default: 3000)
- `RUST_LOG` - Log level (default: info)

## Conventions

- Implement the `Router` trait for MCP server
- Use `async_trait` for async trait methods
- Use `serde_json::json!()` for JSON schemas
- Follow Rust naming conventions (snake_case)
- Run `cargo fmt` before committing
- Run `cargo clippy` before PRs

## Code Quality Tools

Configured in `Cargo.toml` and config files:
- **rustfmt**: Code formatter (`rustfmt.toml`)
- **clippy**: Linter with pedantic rules (enabled in `Cargo.toml`)

## Router Pattern

```rust
#[async_trait]
impl Router for McpRouter {
    fn capabilities(&self) -> ServerCapabilities { ... }
    fn name(&self) -> String { "my-server".to_string() }
    fn version(&self) -> String { "1.0.0".to_string() }
    
    fn list_tools(&self) -> Vec<Tool> { ... }
    async fn call_tool(&self, name: &str, args: Value) -> Result<Vec<Content>, ToolError> { ... }
    
    fn list_resources(&self) -> Vec<Resource> { ... }
    async fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContents>, ResourceError> { ... }
    
    fn list_prompts(&self) -> Vec<Prompt> { ... }
    async fn get_prompt(&self, name: &str, args: Option<HashMap<String, String>>) 
        -> Result<GetPromptResult, PromptError> { ... }
}
```

## Documentation Links

- [MCP Specification](https://modelcontextprotocol.io/)
- [Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [Building Servers](https://modelcontextprotocol.io/docs/develop/build-server)
- [Tokio](https://tokio.rs/)
- [Axum](https://github.com/tokio-rs/axum)
