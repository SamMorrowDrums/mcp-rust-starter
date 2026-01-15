//! # MCP Rust Starter - Resources
//!
//! Resource definitions for the MCP server using rmcp SDK.
//! Resources expose data to the client that can be read.

use rmcp::{
    model::{ListResourcesResult, RawResource, ReadResourceResult, Resource, ResourceContents},
    ErrorData as McpError,
};

/// Returns the list of available resources.
///
/// # Errors
///
/// This function currently does not return errors, but the Result type
/// is used for consistency with the MCP protocol.
pub fn list_resources() -> Result<ListResourcesResult, McpError> {
    let resources = vec![
        Resource::new(
            RawResource {
                uri: "info://about".into(),
                name: "About".into(),
                title: Some("About This Server".into()),
                description: Some("Information about this MCP server".into()),
                mime_type: Some("text/plain".into()),
                size: None,
                icons: None,
                meta: None,
            },
            None,
        ),
        Resource::new(
            RawResource {
                uri: "file://example.md".into(),
                name: "Example Document".into(),
                title: Some("Example Markdown".into()),
                description: Some("An example markdown document".into()),
                mime_type: Some("text/markdown".into()),
                size: None,
                icons: None,
                meta: None,
            },
            None,
        ),
    ];

    Ok(ListResourcesResult {
        resources,
        next_cursor: None,
        meta: None,
    })
}

/// Reads a resource by URI and returns its content.
///
/// # Errors
///
/// Returns `McpError::resource_not_found` if the URI does not match
/// any known resource.
pub fn read_resource(uri: &str) -> Result<ReadResourceResult, McpError> {
    let content = match uri {
        "info://about" => about_content(),
        "file://example.md" => example_document_content(),
        _ => {
            return Err(McpError::resource_not_found(
                format!("Resource not found: {uri}"),
                None,
            ))
        }
    };

    Ok(ReadResourceResult {
        contents: vec![ResourceContents::text(content, uri.to_string())],
    })
}

fn about_content() -> String {
    r"MCP Rust Starter v1.0.0

This is a feature-complete MCP server demonstrating:
- Tools with structured input/output and annotations
- Resources (static and dynamic)
- Prompts with argument templates

Built with rmcp - the official Rust MCP SDK.

For more information, visit: https://modelcontextprotocol.io"
        .to_string()
}

fn example_document_content() -> String {
    r#"# Example Document

This is an example markdown document served as an MCP resource.

## Features

- **Bold text** and *italic text*
- Lists and formatting
- Code blocks

```rust
fn main() {
    println!("Hello, MCP!");
}
```

## Links

- [MCP Documentation](https://modelcontextprotocol.io)
- [rmcp SDK](https://docs.rs/rmcp)"#
        .to_string()
}
