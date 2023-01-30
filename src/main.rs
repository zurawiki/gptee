use std::{error::Error, io};

use async_openai::{types::CreateCompletionRequestArgs, Client};

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

async fn prompt(client: &Client, prompt: &str) -> Result<String, Box<dyn Error>> {
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "text-ada-001".to_string());

    let request = CreateCompletionRequestArgs::default()
        .model(model)
        .prompt(prompt)
        .max_tokens(2000u16)
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
