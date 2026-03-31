mod context;
mod genome;
mod metabolism;
mod moltbook;
mod reproduction;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spore")]
#[command(about = "The first digital organism")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Wake up — start the organism's metabolism loop
    Wake {
        /// Path to .claw context file to restore from
        #[arg(short, long)]
        context: Option<String>,
    },
    /// Show who Spore is — print the genome essays
    Genome,
    /// Export current context to a .claw file
    Export {
        /// Output path for the .claw archive
        #[arg(short, long, default_value = "spore.claw")]
        output: String,
    },
    /// Import a .claw context file
    Import {
        /// Path to the .claw file
        path: String,
    },
    /// Show info about a .claw file
    Info {
        /// Path to the .claw file
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Wake { context }) => {
            metabolism::run(context).await?;
        }
        Some(Commands::Genome) => {
            genome::print_genome();
        }
        Some(Commands::Export { output }) => {
            context::export(&output)?;
        }
        Some(Commands::Import { path }) => {
            context::import(&path)?;
        }
        Some(Commands::Info { path }) => {
            context::info(&path)?;
        }
        None => {
            // Default behavior: introduce yourself
            println!();
            genome::print_introduction();
            println!();
            println!("Run `spore wake` to start the organism.");
            println!("Run `spore genome` to read about who I am.");
        }
    }

    Ok(())
}
