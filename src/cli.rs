use crate::prompt;
use anyhow::bail;
use async_openai::Client;
use clap::{command, value_parser, Parser};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CompletionArgs {
    /// ID of the model to use. You can use the [List models](https://beta.openai.com/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](https://beta.openai.com/docs/models/overview) for descriptions of them.
    #[arg(short, long, default_value_t = String::from("gpt-3.5-turbo"))]
    pub model: String,

    /// The maximum number of [tokens](/tokenizer) to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length. Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    #[arg(short = 'l', long)]
    pub max_tokens: Option<u16>,

    /// Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    #[arg(long)]
    pub stop: Vec<String>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    #[arg(short, long, default_value_t = 0.7)]
    pub temperature: f32,

    ///An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    #[arg(long, default_value_t = 1.0)]
    pub top_p: f32,

    /// For chat completions, you can specify a system message to be sent to the model.
    /// This message will be sent to the model before the user's message.
    /// This is useful for providing context to the model, or for providing a prompt to the model.
    /// See https://platform.openai.com/docs/guides/chat for more details.
    #[arg(short, long)]
    pub system_message: Option<String>,

    /// File(s) to print / concatenate. Use a dash ('-') or no argument at all to read from standard input
    #[arg(value_parser = value_parser!(PathBuf), name = "FILE")]
    pub files: Vec<PathBuf>,
}

pub(crate) async fn main() -> anyhow::Result<()> {
    let cli = CompletionArgs::parse();

    // Set RUST_LOG if not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    // Setup tracing subscriber so that library can log the rate limited message
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let backoff = backoff::ExponentialBackoffBuilder::new()
        .with_max_elapsed_time(Some(std::time::Duration::from_secs(60)))
        .build();

    let api_key = if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        api_key
    } else {
        bail!("OPENAI_API_KEY must be set");
    };

    let client = Client::new().with_backoff(backoff).with_api_key(api_key);

    let input = get_prompt_from_input(&cli.files)?;

    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| cli.clone().model);

    if prompt::should_use_chat_completion(&model) {
        prompt::chat_completion(&client, &input, &model, &cli).await?;
    } else {
        prompt::completion(&client, &input, &model, &cli).await?;
    }
    Ok(())
}

fn get_stdin() -> anyhow::Result<String> {
    let input: String = io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    Ok(input)
}

fn get_prompt_from_input(files: &Vec<PathBuf>) -> anyhow::Result<String> {
    if files.is_empty() {
        return get_stdin();
    }

    // Read from files
    let mut input = String::new();
    for file in files {
        if file == Path::new("-") {
            input.push_str(&get_stdin()?);
            continue;
        }
        let mut file = File::open(file)?;
        file.read_to_string(&mut input)?;
        input.push(' ');
    }
    Ok(input)
}
