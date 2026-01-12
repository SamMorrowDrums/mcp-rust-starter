//! # MCP Rust Starter - stdio Transport
//!
//! This entrypoint runs the MCP server using stdio transport,
//! which is ideal for local development and CLI tool integration.
//!
//! ## Usage
//!
//! ```sh
//! cargo run --bin mcp-rust-starter-stdio
//! ```
//!
//! ## Documentation
//!
//! - [MCP Transports](https://modelcontextprotocol.io/docs/develop/transports#stdio)
//! - [rmcp SDK](https://github.com/anthropics/rust-mcp-sdk)

use mcp_rust_starter::McpServer;
use rmcp::ServiceExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize logging to stderr (don't interfere with stdio protocol)
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("MCP Rust Starter running on stdio");

    // Create the server and serve via stdio
    let server = McpServer::new();
    
    // The serve_stdio method handles all the stdio transport details
    let service = server.serve(rmcp::transport::stdio()).await;
    
    match service {
        Ok(running) => {
            tracing::info!("Server started successfully");
            // Wait for the server to complete
            if let Err(e) = running.waiting().await {
                tracing::error!("Server error: {:?}", e);
            }
        }
        Err(e) => {
            tracing::error!("Failed to start server: {:?}", e);
        }
    }

    tracing::info!("Server shutting down");
}
