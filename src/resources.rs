//! # MCP Rust Starter - Resources
//!
//! Resource definitions for the MCP server using rmcp SDK.
//! Resources expose data to the client that can be read.

use rmcp::{
    model::{
        ListResourceTemplatesResult, ListResourcesResult, RawResource, RawResourceTemplate,
        ReadResourceResult, Resource, ResourceContents, ResourceTemplate,
    },
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
                uri: "about://server".into(),
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
                uri: "doc://example".into(),
                name: "Example Document".into(),
                title: Some("Example Document".into()),
                description: Some("An example document resource".into()),
                mime_type: Some("text/plain".into()),
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

/// Returns the list of available resource templates.
///
/// # Errors
///
/// This function currently does not return errors, but the Result type
/// is used for consistency with the MCP protocol.
pub fn list_resource_templates() -> Result<ListResourceTemplatesResult, McpError> {
    let templates = vec![
        ResourceTemplate::new(
            RawResourceTemplate {
                uri_template: "greeting://{name}".into(),
                name: "Personalized Greeting".into(),
                title: Some("Personalized Greeting".into()),
                description: Some("A personalized greeting for a specific person".into()),
                mime_type: Some("text/plain".into()),
                icons: None,
            },
            None,
        ),
        ResourceTemplate::new(
            RawResourceTemplate {
                uri_template: "item://{id}".into(),
                name: "Item Data".into(),
                title: Some("Item Data".into()),
                description: Some("Data for a specific item by ID".into()),
                mime_type: Some("application/json".into()),
                icons: None,
            },
            None,
        ),
    ];

    Ok(ListResourceTemplatesResult {
        resource_templates: templates,
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
        "about://server" => about_content(),
        "doc://example" => example_document_content(),
        _ if uri.starts_with("greeting://") => {
            // Extract name from greeting://{name}
            let name = uri.strip_prefix("greeting://").unwrap_or("World");
            greeting_content(name)
        }
        _ if uri.starts_with("item://") => {
            // Extract ID from item://{id}
            let id = uri.strip_prefix("item://").unwrap_or("0");
            item_content(id)
        }
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

fn greeting_content(name: &str) -> String {
    format!("Hello, {name}! Welcome to the MCP Rust Starter Server.")
}

fn item_content(id: &str) -> String {
    // Return a JSON object for the item
    serde_json::json!({
        "id": id,
        "name": format!("Item {}", id),
        "description": format!("This is item number {}", id),
        "created_at": "2024-01-01T00:00:00Z",
        "status": "active"
    })
    .to_string()
}
