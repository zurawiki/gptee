use std::io::Write;
use std::{error::Error, io};

use async_openai::{types::CreateCompletionRequestArgs, Client};

use rust_tokenizers::tokenizer::{Gpt2Tokenizer, Tokenizer};
use rust_tokenizers::vocab::{BpePairVocab, Gpt2Vocab, Vocab};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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
    let mut vocab_file = tempfile::NamedTempFile::new()?;
    vocab_file.write_all(include_bytes!("../resources/encoder.json"))?;
    let vocab = Gpt2Vocab::from_file(vocab_file.into_temp_path()).unwrap();

    let mut merges_file = tempfile::NamedTempFile::new()?;
    merges_file.write_all(include_bytes!("../resources/vocab.bpe"))?;
    let merges = BpePairVocab::from_file(merges_file.into_temp_path()).unwrap();

    let lower_case = false;
    let tokenizer = Gpt2Tokenizer::from_existing_vocab_and_merges(vocab, merges, lower_case);
    Ok(tokenizer.tokenize(prompt).len() as u16)
}

async fn prompt(client: &Client, prompt: &str) -> Result<String, Box<dyn Error>> {
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "text-ada-001".to_string());

    dbg!(&model);
    let request = CreateCompletionRequestArgs::default()
        .model(&model)
        .prompt(prompt)
        .max_tokens(dbg!(
            model_name_to_context_size(&model) - count_tokens(prompt)?
        ))
        .build()?;

    let response = client.completions().create(request).await?;

    Ok(response.choices[0].text.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    let output = prompt(&client, &input).await?;
    println!("{output}");

    Ok(())
}
