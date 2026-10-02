# MCP Rust Starter

[![CI](https://github.com/SamMorrowDrums/mcp-rust-starter/actions/workflows/ci.yml/badge.svg)](https://github.com/SamMorrowDrums/mcp-rust-starter/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Clippy](https://img.shields.io/badge/Clippy-pedantic-green?logo=rust&logoColor=white)](https://rust-lang.github.io/rust-clippy/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MCP](https://img.shields.io/badge/MCP-Model%20Context%20Protocol-purple)](https://modelcontextprotocol.io/)

A feature-complete Model Context Protocol (MCP) server template in Rust. This starter demonstrates all major MCP features with clean, idiomatic Rust code.

## 📚 Documentation

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [Building MCP Servers](https://modelcontextprotocol.io/docs/develop/build-server)

## ✨ Features

| Category | Feature | Description |
|----------|---------|-------------|
| **Tools** | `hello` | Basic greeting tool |
| | `get_weather` | Tool returning structured JSON |
| | `calculator` | Basic arithmetic operations |
| **Resources** | `info://about` | Static informational resource |
| | `file://example.md` | File-based markdown resource |
| **Templates** | `greeting://{name}` | Personalized greeting |
| | `data://items/{id}` | Data lookup by ID |
| **Prompts** | `greet` | Greeting in various styles |
| | `code_review` | Code review with focus areas |

> **Note:** Tool annotations (readOnlyHint, etc.) are not yet available in mcp-spec v0.1.0.
> Tool behavior hints are documented in the description field until the crate is updated.

## 🚀 Quick Start

### Prerequisites

- [Current stable Rust](https://www.rust-lang.org/tools/install) (2021 edition; selected by `rust-toolchain.toml`)
- [Cargo](https://doc.rust-lang.org/cargo/) (included with Rust)
- (Optional) [cargo-watch](https://crates.io/crates/cargo-watch) for live reload

### Installation

```bash
# Clone the repository
git clone https://github.com/SamMorrowDrums/mcp-rust-starter.git
cd mcp-rust-starter

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

### Running the Server

**stdio transport** (for local development):
```bash
cargo run --bin mcp-rust-starter-stdio
```

**HTTP transport** (for remote/web deployment):
```bash
cargo run --bin mcp-rust-starter-http
# Or with custom port:
PORT=8080 cargo run --bin mcp-rust-starter-http
# Server runs on http://localhost:3000 by default
```

The patched SDK rejects HTTP `Host` headers outside its default loopback
allowlist. For a public deployment, configure `StreamableHttpServerConfig` in
`src/bin/http.rs` with `.with_allowed_hosts(["your-server.example.com"])` for
the exact hostnames you serve. Do not disable host validation.

## 🔧 VS Code Integration

This project includes VS Code configuration for seamless development:

1. Open the project in VS Code
2. The MCP configuration is in `.vscode/mcp.json`
3. Build with `Ctrl+Shift+B` (or `Cmd+Shift+B` on Mac)
4. Debug with F5 (configurations for both transports)
5. Test the server using VS Code's MCP tools

### Using DevContainers

1. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
2. Open command palette: "Dev Containers: Reopen in Container"
3. Everything is pre-configured and ready to use!

## 📁 Project Structure

```
.
├── Cargo.toml                 # Package manifest with dependencies
├── rust-toolchain.toml        # Rust toolchain pinning
├── rustfmt.toml               # Formatter configuration
├── clippy.toml                # Linter configuration
├── src/
│   ├── lib.rs                 # Server orchestration (Router impl)
│   ├── tools.rs               # Tool definitions (hello, get_weather, etc.)
│   ├── resources.rs           # Resource and template definitions
│   ├── prompts.rs             # Prompt definitions
│   └── bin/
│       ├── stdio.rs           # stdio transport entrypoint
│       └── http.rs            # HTTP transport entrypoint
├── .vscode/
│   ├── mcp.json               # MCP server configuration
│   ├── tasks.json             # Build/run tasks
│   ├── launch.json            # Debug configurations
│   └── extensions.json
├── .devcontainer/
│   └── devcontainer.json
└── LICENSE
```

## 🛠️ Development

```bash
# Development with live reload
cargo watch -x 'run --bin mcp-rust-starter-stdio'
# Requires cargo-watch: cargo install cargo-watch

# Build
cargo build

# Run tests
cargo test

# Format code (auto-fix)
cargo fmt

# Lint code
cargo clippy

# Lint with auto-fix
cargo clippy --fix

# Check without building
cargo check

# Build release
cargo build --release
```

### Live Reload

Install [cargo-watch](https://crates.io/crates/cargo-watch) for automatic rebuilds:
```bash
cargo install cargo-watch
cargo watch -x 'run --bin mcp-rust-starter-stdio'
```
Changes to any source file will automatically rebuild and restart the server.

### Dependency maintenance

Dependabot checks Cargo dependencies and GitHub Actions weekly. Commit `Cargo.lock`
alongside dependency changes so both transports use the audited dependency graph.
To check it against RustSec advisories, including unsoundness warnings:

```bash
cargo install cargo-audit --locked
cargo audit --deny warnings
```

The server uses the patched rmcp 2.x SDK. Upgrading to rmcp 3.x is deferred until
its new MCP protocol lifecycle can be migrated without changing client compatibility.
The optional remote endpoint workflow uses `mcp-server-diff` to probe a configured
server; it is not a full protocol conformance suite.

## 🔍 MCP Inspector

The [MCP Inspector](https://modelcontextprotocol.io/docs/tools/inspector) is an essential development tool for testing and debugging MCP servers.

### Running Inspector

```bash
npx @modelcontextprotocol/inspector -- cargo run --bin mcp-rust-starter-stdio
```

### What Inspector Provides

- **Tools Tab**: List and invoke all registered tools with parameters
- **Resources Tab**: Browse and read resources and templates
- **Prompts Tab**: View and test prompt templates
- **Logs Tab**: See JSON-RPC messages between client and server
- **Schema Validation**: Verify tool input/output schemas

### Debugging Tips

1. Start Inspector before connecting your IDE/client
2. Use the "Logs" tab to see exact request/response payloads
3. Test tool annotations (ToolAnnotations) are exposed correctly
4. Verify the Router trait implementation works correctly
5. Check that all tools return proper Content types

## 📖 Feature Examples

### Tool with Annotations

```rust
Tool {
    name: "hello".to_string(),
    description: Some("A friendly greeting tool".to_string()),
    input_schema: json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "The name to greet"
            }
        },
        "required": ["name"]
    }),
    annotations: Some(ToolAnnotations {
        title: Some("Say Hello".to_string()),
        read_only_hint: Some(true),
        ..Default::default()
    }),
}
```

### Resource Implementation

```rust
async fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContents>, ResourceError> {
    match uri {
        "info://about" => Ok(vec![ResourceContents::text(
            uri,
            "MCP Rust Starter v1.0.0",
            Some("text/plain"),
        )]),
        _ => Err(ResourceError::NotFound(uri.to_string())),
    }
}
```

### Prompt Definition

```rust
Prompt {
    name: "greet".to_string(),
    description: Some("Generate a greeting".to_string()),
    arguments: Some(vec![
        PromptArgument {
            name: "name".to_string(),
            description: Some("Name to greet".to_string()),
            required: Some(true),
        },
    ]),
}
```

## 🔐 Configuration

Environment variables:
- `PORT` - HTTP server port (default: 3000)
- `RUST_LOG` - Log level (default: info)

## 🧹 Code Quality

This project uses Rust's standard tooling:
- **rustfmt** - Code formatting (`cargo fmt`)
- **clippy** - Linting with pedantic rules (`cargo clippy`)

Both run automatically in CI and are configured in their respective `.toml` files.

## 🤝 Contributing

Contributions welcome! Please ensure your changes:
1. Pass `cargo fmt --check`
2. Pass `cargo clippy`
3. Pass `cargo test`
4. Maintain feature parity with other language starters

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.
