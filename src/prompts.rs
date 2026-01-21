//! # MCP Rust Starter - Prompts
//!
//! Prompt template definitions for the MCP server using rmcp SDK.
//! Prompts are pre-configured message templates the client can use.

use std::collections::HashMap;

use rmcp::{
    model::{
        GetPromptResult, ListPromptsResult, Prompt, PromptArgument, PromptMessage,
        PromptMessageContent, PromptMessageRole,
    },
    ErrorData as McpError,
};

/// Returns the list of available prompts.
///
/// # Errors
///
/// This function currently does not return errors, but the Result type
/// is used for consistency with the MCP protocol.
pub fn list_prompts() -> Result<ListPromptsResult, McpError> {
    let prompts = vec![
        Prompt {
            name: "greet".into(),
            title: Some("Greeting Prompt".into()),
            description: Some("Generate a greeting message".into()),
            arguments: Some(vec![
                PromptArgument {
                    name: "name".into(),
                    title: None,
                    description: Some("Name of the person to greet".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "style".into(),
                    title: None,
                    description: Some("Greeting style (formal/casual)".into()),
                    required: Some(false),
                },
            ]),
            icons: None,
            meta: None,
        },
        Prompt {
            name: "code_review".into(),
            title: Some("Code Review".into()),
            description: Some("Review code for potential improvements".into()),
            arguments: Some(vec![PromptArgument {
                name: "code".into(),
                title: None,
                description: Some("The code to review".into()),
                required: Some(true),
            }]),
            icons: None,
            meta: None,
        },
    ];

    Ok(ListPromptsResult {
        prompts,
        next_cursor: None,
        meta: None,
    })
}

/// Gets a prompt by name with the given arguments.
///
/// # Errors
///
/// Returns `McpError::invalid_params` if the prompt name is not found
/// or if required arguments are missing.
#[allow(clippy::implicit_hasher)]
pub fn get_prompt(
    prompt_name: &str,
    arguments: Option<HashMap<String, String>>,
) -> Result<GetPromptResult, McpError> {
    let args = arguments.unwrap_or_default();

    match prompt_name {
        "greet" => greet_prompt(&args),
        "code_review" => code_review_prompt(&args),
        _ => Err(McpError::invalid_params(
            format!("Prompt not found: {prompt_name}"),
            None,
        )),
    }
}

fn greet_prompt(args: &HashMap<String, String>) -> Result<GetPromptResult, McpError> {
    let name = args.get("name").ok_or_else(|| {
        McpError::invalid_params("Missing required 'name' argument".to_string(), None)
    })?;

    let style = args.get("style").map_or("casual", String::as_str);

    let text = match style {
        "formal" => format!("Please compose a formal, professional greeting for {name}."),
        "enthusiastic" => format!("Create an excited, enthusiastic greeting for {name}!"),
        // Default to casual style for "casual" and any other value
        _ => format!("Write a casual, friendly hello to {name}."),
    };

    Ok(GetPromptResult {
        description: Some("Generate a personalized greeting".into()),
        messages: vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::Text { text },
        }],
    })
}

fn code_review_prompt(args: &HashMap<String, String>) -> Result<GetPromptResult, McpError> {
    let code = args.get("code").ok_or_else(|| {
        McpError::invalid_params("Missing required 'code' argument".to_string(), None)
    })?;

    let text =
        format!("Please review the following code and provide feedback:\n\n```\n{code}\n```");

    Ok(GetPromptResult {
        description: Some("Code review request".into()),
        messages: vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::Text { text },
        }],
    })
}
