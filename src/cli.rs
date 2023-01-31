use crate::prompt;
use async_openai::Client;
use clap::{command, Parser};
use std::io;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CompletionArgs {
    /// ID of the model to use. You can use the [List models](https://beta.openai.com/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](https://beta.openai.com/docs/models/overview) for descriptions of them.
    #[arg(short, long, default_value_t = String::from("text-ada-001"))]
    pub model: String,

    /// The maximum number of [tokens](/tokenizer) to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length. Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    #[arg(short = 'l', long)]
    pub max_tokens: Option<u16>,

    /// Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    #[arg(short, long)]
    pub stop: Vec<String>,
}

pub(crate) async fn main() -> anyhow::Result<()> {
    let cli = CompletionArgs::parse();

    // This should come from env var outside the program
    std::env::set_var("RUST_LOG", "warn");

    // Setup tracing subscriber so that library can log the rate limited message
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let backoff = backoff::ExponentialBackoffBuilder::new()
        .with_max_elapsed_time(Some(std::time::Duration::from_secs(60)))
        .build();

    let client = Client::new().with_backoff(backoff);

    // get all of stdin into a string
    let input: String = io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let output = prompt::prompt(&client, &input, cli).await?;
    println!("{output}");

    Ok(())
}