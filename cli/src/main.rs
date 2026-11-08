use std::env;

use http_handler::HttpHandler;
use llm_abstract::{LlmProvider, LlmResonse, provider::ollama::OllamaLlm};

mod cli;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_run_ollama().await?;

    //     let args = env::args().collect::<Vec<_>>();
    //     let raw_config = cli::RawArgs::from_args(args)?;
    //
    //     println!("Cli started with conf {:?}", raw_config);

    // println!("Hello, world! {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

async fn test_run_ollama() -> Result<(), Box<dyn std::error::Error>> {
    let client = http_handler::get_client()?;
    let model = String::from("qwen2.5-coder:7b");
    let handler = HttpHandler::new(client);

    let mut ollama = OllamaLlm::new(handler, model);

    let msg = OllamaLlm::new_user_msg(String::from(
        "Write a Rust function that caculates the factorial of the input.",
    ));

    println!("Sending chat to LLM");
    let res = ollama.chat(msg).await?;

    println!("{}", res.get_message_content());

    Ok(())
}
