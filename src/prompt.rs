use anyhow::bail;
use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs,
        CreateCompletionRequestArgs, Role,
    },
    Client,
};

use futures::StreamExt;
use tiktoken_rs::tiktoken::{cl100k_base, p50k_base};

use crate::cli::CompletionArgs;

/// Calculate the maximum number of tokens possible to generate for a model
fn model_name_to_context_size(model_name: &str) -> u16 {
    match model_name {
        "text-davinci-003" => 4000,
        "text-davinci-002" => 4000,
        "text-curie-001" => 2048,
        "text-babbage-001" => 2048,
        "text-ada-001" => 2048,
        "code-davinci-002" => 4000,
        "code-cushman-001" => 2048,
        _ => 4096,
    }
}

fn count_tokens(model: &str, prompt: &str) -> anyhow::Result<u16> {
    let bpe = match should_use_chat_completion(model) {
        true => cl100k_base(),
        false => p50k_base(),
    }
    .unwrap();
    let tokens = bpe.encode_with_special_tokens(prompt);
    Ok(tokens.len() as u16)
}

pub(crate) fn should_use_chat_completion(model: &str) -> bool {
    model.to_lowercase().starts_with("gpt-3.5-turbo")
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
    let request = request.messages(messages);

    // let max_tokens = cli.max_tokens.unwrap_or_else(|| {
    //     model_name_to_context_size(model)
    //     - count_tokens(
    //         model,
    //         &cli.system_message.to_owned().unwrap_or("".to_owned()),
    //     ).unwrap_or(0)
    //     - count_tokens(model, prompt).unwrap_or(0)
    //     // Chat completions use extra tokens for the prompt
    //     - 10
    // });

    let request = if cli.max_tokens.is_some() {
        request.max_tokens(cli.max_tokens.unwrap())
    } else {
        request
    };
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

    let max_tokens = cli.max_tokens.unwrap_or_else(|| {
        model_name_to_context_size(model) - count_tokens(model, &prompt).unwrap_or(0)
    });
    let request = request.max_tokens(max_tokens);

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
