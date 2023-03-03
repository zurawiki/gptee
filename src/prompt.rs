use async_openai::{types::CreateCompletionRequestArgs, Client};

use futures::StreamExt;
use tiktoken_rs::tiktoken::p50k_base;

use crate::cli::CompletionArgs;

/// Calculate the maximum number of tokens possible to generate for a model
fn model_name_to_context_size(model_name: &str) -> u16 {
    match model_name {
        "text-davinci-003" => 4097,
        "text-curie-001" => 2048,
        "text-babbage-001" => 2048,
        "text-ada-001" => 2048,
        "code-davinci-002" => 8000,
        "code-cushman-001" => 2048,
        _ => 4097,
    }
}

fn count_tokens(prompt: &str) -> anyhow::Result<u16> {
    let bpe = p50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(prompt);
    Ok(tokens.len() as u16)
}

pub(crate) async fn prompt(
    client: &Client,
    prompt: &str,
    cli: CompletionArgs,
) -> anyhow::Result<()> {
    let mut request = &mut CreateCompletionRequestArgs::default();
    request = request.prompt(prompt);

    let model = std::env::var("OPENAI_MODEL").unwrap_or(cli.model);
    request = request.model(&model);

    let max_tokens = model_name_to_context_size(&model) - count_tokens(prompt)?;
    let max_tokens = cli.max_tokens.unwrap_or(max_tokens);
    request = request.max_tokens(max_tokens);

    if !cli.stop.is_empty() {
        request = request.stop(cli.stop);
    }

    let request = request.stream(true);
    let request = request.build()?;

    let mut stream = client.completions().create_stream(request).await?;

    while let Some(response) = stream.next().await {
        match response {
            Ok(ccr) => ccr.choices.iter().for_each(|c| {
                print!("{}", c.text);
            }),
            Err(e) => eprintln!("{e}"),
        }
    }
    println!("");
    Ok(())
}
