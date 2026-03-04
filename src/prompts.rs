//! # MCP Rust Starter - Prompts
//!
//! Prompts are **reusable message templates** that clients can retrieve and
//! fill with arguments. Think of them as "saved prompts" — the server defines
//! the template structure, and the client supplies the values.
//!
//! This module demonstrates two prompt templates:
//! - **greet** – generates a personalised greeting in different styles
//! - **code_review** – creates a structured code review request

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
        Prompt::new(
            "greet",
            Some("Generate a greeting message"),
            Some(vec![
                PromptArgument::new("name")
                    .with_description("Name of the person to greet")
                    .with_required(true),
                PromptArgument::new("style")
                    .with_description("Greeting style (formal/casual)")
                    .with_required(false),
            ]),
        )
        .with_title("Greeting Prompt"),
        Prompt::new(
            "code_review",
            Some("Review code for potential improvements"),
            Some(vec![PromptArgument::new("code")
                .with_description("The code to review")
                .with_required(true)]),
        )
        .with_title("Code Review"),
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

    Ok(GetPromptResult::new(vec![PromptMessage::new(
        PromptMessageRole::User,
        PromptMessageContent::text(text),
    )])
    .with_description("Generate a personalized greeting"))
}

fn code_review_prompt(args: &HashMap<String, String>) -> Result<GetPromptResult, McpError> {
    let code = args.get("code").ok_or_else(|| {
        McpError::invalid_params("Missing required 'code' argument".to_string(), None)
    })?;

    let text =
        format!("Please review the following code and provide feedback:\n\n```\n{code}\n```");

    Ok(GetPromptResult::new(vec![PromptMessage::new(
        PromptMessageRole::User,
        PromptMessageContent::text(text),
    )])
    .with_description("Code review request"))
}
