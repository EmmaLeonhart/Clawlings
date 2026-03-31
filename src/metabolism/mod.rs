pub mod llm;

use anyhow::Result;
use std::io::{self, BufRead, Write};

use crate::genome::Genome;
use llm::{LlmClient, Message};

const DEFAULT_LLM_URL: &str = "http://localhost:8080";

/// The metabolism is Spore's core life loop.
///
/// It loads context, presents itself, helps the user, learns from interactions,
/// and persists its updated context on shutdown.
pub async fn run(context_path: Option<String>) -> Result<()> {
    // 1. Restore context if provided
    if let Some(ref path) = context_path {
        println!("Restoring context from {path}...");
        crate::context::import(path)?;
    }

    // 2. Load genome (static identity)
    let genome = Genome::load();
    let system_context = genome.as_system_context();

    // 3. Connect to local LLM
    let llm_url =
        std::env::var("SPORE_LLM_URL").unwrap_or_else(|_| DEFAULT_LLM_URL.to_string());
    let client = LlmClient::new(&llm_url);

    println!();
    crate::genome::print_introduction();
    println!();

    if !client.is_available().await {
        println!("I can't find a local LLM server at {llm_url}.");
        println!();
        println!("To bring me to life, start a local LLM server:");
        println!("  - Ollama:    ollama serve && ollama run deepseek-r1:8b");
        println!("  - llama.cpp: llama-server -m model.gguf");
        println!("  - LM Studio: start the local server from the UI");
        println!();
        println!("Then run `spore wake` again.");
        println!();
        println!("Set SPORE_LLM_URL to use a different address.");
        return Ok(());
    }

    println!("I'm alive. Type something, or 'quit' to let me sleep.");
    println!();

    // 4. The metabolism loop
    let mut conversation = vec![Message {
        role: "system".to_string(),
        content: system_context,
    }];

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("you> ");
        stdout.flush()?;

        let mut input = String::new();
        if stdin.lock().read_line(&mut input)? == 0 {
            break; // EOF
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "quit" || input == "exit" {
            println!();
            println!("Going to sleep. My context is preserved.");
            break;
        }

        conversation.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        match client.chat(&conversation).await {
            Ok(response) => {
                println!();
                println!("spore> {response}");
                println!();

                conversation.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
            }
            Err(e) => {
                eprintln!("(I had trouble thinking: {e})");
                // Remove the failed user message so conversation stays clean
                conversation.pop();
            }
        }
    }

    Ok(())
}
