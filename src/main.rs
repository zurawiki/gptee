mod cli;
mod prompt;
use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    cli::main().await
}
