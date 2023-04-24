use anyhow::bail;
use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs,
        CreateCompletionRequestArgs, Role,
    },
    Client,
};

use futures::StreamExt;
use tiktoken_rs::async_openai::get_chat_completion_max_tokens;

use crate::cli::CompletionArgs;

pub(crate) fn should_use_chat_completion(model: &str) -> bool {
    model.to_lowercase().starts_with("gpt-4") || model.to_lowercase().starts_with("gpt-3.5-turbo")
}

pub(crate) async fn chat_completion(
    client: &Client,
    prompt: &str,
    model: &str,
    cli: &CompletionArgs,
) -> anyhow::Result<()> {
    let request = &mut CreateChatCompletionRequestArgs::default();
    let request = request.model(model);
    let request = request.top_p(cli.top_p);
    let request = request.temperature(cli.temperature);

    let mut messages = vec![ChatCompletionRequestMessageArgs::default()
        .content(prompt)
        .role(Role::User)
        .build()?];
    if let Some(system_message) = &cli.system_message {
        messages.insert(
            0,
            ChatCompletionRequestMessageArgs::default()
                .content(system_message)
                .role(Role::System)
                .build()?,
        );
    }
    let request = request.messages(messages.to_owned());
    let max_tokens = cli
        .max_tokens
        .unwrap_or_else(|| get_chat_completion_max_tokens(model, &messages).unwrap() as u16);

    let request = request.max_tokens(max_tokens);
    let request = if !cli.stop.is_empty() {
        request.stop(&cli.stop)
    } else {
        request
    };

    let request = request.build()?;
    let mut stream = client.chat().create_stream(request).await?;

    // For reasons not documented in OpenAI docs / OpenAPI spec, the response of streaming call is different and doesn't include all the same fields.
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        print!("{content}");
                    }
                });
            }
            Err(e) => {
                bail!("{e}");
            }
        }
    }
    println!();

    Ok(())
}
pub(crate) async fn completion(
    client: &Client,
    prompt: &str,
    model: &str,
    cli: &CompletionArgs,
) -> anyhow::Result<()> {
    let request = &mut CreateCompletionRequestArgs::default();
    let request = request.temperature(cli.temperature).top_p(cli.top_p);

    let mut prompt = prompt.to_string();
    if let Some(system_message) = &cli.system_message {
        prompt = format!("{system_message} {prompt}");
    }
    let request = request.prompt(&prompt);

    let request = request.model(model);

    let request = if let Some(max_tokens) = cli.max_tokens {
        request.max_tokens(max_tokens)
    } else {
        let max_tokens = tiktoken_rs::get_completion_max_tokens(model, &prompt)? as u16;
        request.max_tokens(max_tokens)
    };

    let request = if !cli.stop.is_empty() {
        request.stop(&cli.stop)
    } else {
        request
    };

    let request = request.stream(true);
    let request = request.build()?;

    let mut stream = client.completions().create_stream(request).await?;

    while let Some(response) = stream.next().await {
        match response {
            Ok(ccr) => ccr.choices.iter().for_each(|c| {
                print!("{}", c.text);
            }),
            Err(e) => bail!("{e}"),
        }
    }
    println!();
    Ok(())
}
