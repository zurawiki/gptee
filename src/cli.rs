use crate::prompt;
use async_openai::Client;
use std::io;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub(crate) async fn main() -> anyhow::Result<()> {
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
    let output = prompt::prompt(&client, &input).await?;
    println!("{output}");

    Ok(())
}
