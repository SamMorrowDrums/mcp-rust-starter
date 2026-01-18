//! # MCP Rust Starter - HTTP Transport
//!
//! This entrypoint runs the MCP server using HTTP transport with
//! the Streamable HTTP protocol.
//!
//! ## Usage
//!
//! ```sh
//! cargo run --bin mcp-rust-starter-http
//! ```
//!
//! The server will listen on `http://localhost:3000/mcp`
//!
//! ## Documentation
//!
//! - [MCP Transports](https://modelcontextprotocol.io/docs/develop/transports#http)
//! - [rmcp SDK](https://github.com/anthropics/rust-mcp-sdk)

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use mcp_rust_starter::McpServer;
use rmcp::transport::{
    streamable_http_server::session::local::LocalSessionManager, StreamableHttpServerConfig,
    StreamableHttpService,
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("Invalid address");
    tracing::info!("MCP Rust Starter HTTP server starting on {}", addr);

    // Configure the MCP HTTP service
    let config = StreamableHttpServerConfig::default();
    let session_manager = Arc::new(LocalSessionManager::default());

    // Create the MCP service that spawns a new server instance per session
    let mcp_service =
        StreamableHttpService::new(move || Ok(McpServer::new()), session_manager, config);

    // Build the router with health check and MCP endpoint
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest_service("/mcp", mcp_service)
        .layer(cors);

    tracing::info!("Server ready at http://{}/mcp", addr);
    tracing::info!("Health check at http://{}/health", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");
    axum::serve(listener, app).await.expect("Server error");
}

async fn health_check() -> &'static str {
    "OK"
}
