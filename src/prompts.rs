//! # MCP Rust Starter - Prompts
//!
//! Prompt template definitions for the MCP server using rmcp SDK.
//! Prompts are pre-configured message templates the client can use.

use std::collections::HashMap;

use rmcp::{
    ErrorData as McpError,
    model::{
        GetPromptResult, ListPromptsResult, Prompt, PromptArgument, PromptMessage,
        PromptMessageContent, PromptMessageRole,
    },
};

/// Returns the list of available prompts.
pub fn list_prompts() -> Result<ListPromptsResult, McpError> {
    let prompts = vec![
        Prompt {
            name: "greet".into(),
            title: Some("Greeting Generator".into()),
            description: Some("Generate a greeting in a specific style".into()),
            arguments: Some(vec![
                PromptArgument {
                    name: "name".into(),
                    title: Some("Name".into()),
                    description: Some("Name of the person to greet".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "style".into(),
                    title: Some("Style".into()),
                    description: Some("The greeting style (formal, casual, enthusiastic)".into()),
                    required: Some(false),
                },
            ]),
            icons: None,
            meta: None,
        },
        Prompt {
            name: "code_review".into(),
            title: Some("Code Review".into()),
            description: Some("Request a code review with specific focus areas".into()),
            arguments: Some(vec![
                PromptArgument {
                    name: "code".into(),
                    title: Some("Code".into()),
                    description: Some("The code to review".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "language".into(),
                    title: Some("Language".into()),
                    description: Some("Programming language".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "focus".into(),
                    title: Some("Focus Area".into()),
                    description: Some(
                        "What to focus on (security, performance, readability, all)".into(),
                    ),
                    required: Some(false),
                },
            ]),
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
pub async fn get_prompt(
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

    let style = args.get("style").map(String::as_str).unwrap_or("casual");

    let text = match style {
        "formal" => format!("Please compose a formal, professional greeting for {name}."),
        "casual" => format!("Write a casual, friendly hello to {name}."),
        "enthusiastic" => format!("Create an excited, enthusiastic greeting for {name}!"),
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

    let language = args.get("language").ok_or_else(|| {
        McpError::invalid_params("Missing required 'language' argument".to_string(), None)
    })?;

    let focus = args.get("focus").map(String::as_str).unwrap_or("all");

    let focus_instruction = match focus {
        "security" => "Focus on security vulnerabilities and potential exploits.",
        "performance" => "Focus on performance optimizations and efficiency issues.",
        "readability" => "Focus on code clarity, naming, and maintainability.",
        _ => "Provide a comprehensive review covering security, performance, and readability.",
    };

    let text = format!(
        "Please review the following {language} code. {focus_instruction}\n\n```{language}\n{code}\n```"
    );

    Ok(GetPromptResult {
        description: Some("Code review request".into()),
        messages: vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::Text { text },
        }],
    })
}
