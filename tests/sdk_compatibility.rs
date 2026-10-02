use std::collections::HashMap;

use mcp_rust_starter::{prompts, resources, McpServer};
use rmcp::ServerHandler;
use serde_json::json;

#[test]
fn resource_metadata_preserves_wire_format() {
    let result = serde_json::to_value(resources::list_resources().unwrap()).unwrap();
    assert_eq!(
        result,
        json!({
            "resources": [
                {
                    "uri": "about://server",
                    "name": "About",
                    "title": "About This Server",
                    "description": "Information about this MCP server",
                    "mimeType": "text/plain"
                },
                {
                    "uri": "doc://example",
                    "name": "Example Document",
                    "title": "Example Document",
                    "description": "An example document resource",
                    "mimeType": "text/plain"
                }
            ]
        })
    );
    let templates = serde_json::to_value(resources::list_resource_templates().unwrap()).unwrap();
    assert_eq!(
        templates,
        json!({
            "resourceTemplates": [
                {
                    "uriTemplate": "greeting://{name}",
                    "name": "Personalized Greeting",
                    "title": "Personalized Greeting",
                    "description": "A personalized greeting for a specific person",
                    "mimeType": "text/plain"
                },
                {
                    "uriTemplate": "item://{id}",
                    "name": "Item Data",
                    "title": "Item Data",
                    "description": "Data for a specific item by ID",
                    "mimeType": "application/json"
                }
            ]
        })
    );
    assert!(resources::read_resource("unknown://resource").is_err());
}

#[test]
fn prompt_messages_preserve_wire_format_and_argument_validation() {
    let args = HashMap::from([("name".to_string(), "Alice".to_string())]);
    let result = serde_json::to_value(prompts::get_prompt("greet", Some(args)).unwrap()).unwrap();
    assert_eq!(
        result,
        json!({
            "description": "Generate a personalized greeting",
            "messages": [{
                "role": "user",
                "content": {
                    "type": "text",
                    "text": "Write a casual, friendly hello to Alice."
                }
            }]
        })
    );
    assert!(prompts::get_prompt("greet", None).is_err());
    assert!(prompts::get_prompt("code_review", None).is_err());
    assert!(prompts::get_prompt("unknown", None).is_err());
}

#[test]
fn server_keeps_legacy_protocol_and_capabilities() {
    let info = serde_json::to_value(McpServer::new().get_info()).unwrap();
    assert_eq!(info["protocolVersion"], "2025-11-25");
    assert_eq!(info["serverInfo"]["name"], "mcp-rust-starter");
    assert_eq!(info["capabilities"]["tools"]["listChanged"], true);
    assert!(info["capabilities"]["resources"].is_object());
    assert!(info["capabilities"]["prompts"].is_object());
}
