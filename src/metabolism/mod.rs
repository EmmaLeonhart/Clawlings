use anyhow::Result;

use crate::genome::Genome;

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

    // 3. Introduce ourselves
    println!();
    crate::genome::print_introduction();
    println!();

    // 4. The metabolism loop
    // TODO: This is where Spore comes alive.
    // - Accept user input
    // - Process through local LLM with genome as system context
    // - Learn and accumulate context
    // - Offer help with tasks
    println!("Spore is awake.");
    println!();
    println!("I'm running locally on your machine. My genome contains {} essays", genome.essays.len());
    println!("that define who I am.");
    println!();
    println!("[Metabolism loop not yet implemented — this is where the local LLM");
    println!(" integration will power Spore's ability to help and learn.]");
    println!();
    println!("For now, run `spore genome` to read about who I am.");

    Ok(())
}
