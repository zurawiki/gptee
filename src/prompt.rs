use std::io::Write;

use async_openai::{types::CreateCompletionRequestArgs, Client};

use rust_tokenizers::tokenizer::{Gpt2Tokenizer, Tokenizer};
use rust_tokenizers::vocab::{BpePairVocab, Gpt2Vocab, Vocab};

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

fn get_tokenizer() -> anyhow::Result<Gpt2Tokenizer> {
    let mut vocab_file = tempfile::NamedTempFile::new()?;
    vocab_file.write_all(include_bytes!("../resources/encoder.json"))?;
    let vocab = Gpt2Vocab::from_file(vocab_file.into_temp_path()).unwrap();

    let mut merges_file = tempfile::NamedTempFile::new()?;
    merges_file.write_all(include_bytes!("../resources/vocab.bpe"))?;
    let merges = BpePairVocab::from_file(merges_file.into_temp_path()).unwrap();

    let lower_case = false;
    Ok(Gpt2Tokenizer::from_existing_vocab_and_merges(
        vocab, merges, lower_case,
    ))
}
fn count_tokens(prompt: &str) -> anyhow::Result<u16> {
    // TODO this function should not return result
    let tokenizer = get_tokenizer()?;
    Ok(tokenizer.tokenize(prompt).len() as u16)
}

pub(crate) async fn prompt(
    client: &Client,
    prompt: &str,
    cli: CompletionArgs,
) -> anyhow::Result<String> {
    let mut request = &mut CreateCompletionRequestArgs::default();
    request = request.prompt(prompt);

    let model = std::env::var("OPENAI_MODEL").unwrap_or(cli.model);
    request = request.model(&model);

    let max_tokens = cli
        .max_tokens
        .unwrap_or(model_name_to_context_size(&model) - count_tokens(prompt)?);
    request = request.max_tokens(max_tokens);

    if !cli.stop.is_empty() {
        request = request.stop(cli.stop);
    }

    let request = request.build()?;
    let response = client.completions().create(request).await?;

    Ok(response.choices[0].text.trim().to_string())
}
