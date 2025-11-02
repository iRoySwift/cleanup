mod commands;
use clap::{Parser, Subcommand};
use commands::Solana;

use crate::commands::Rust;

#[derive(Parser)]
#[command(name = "cleanup")]
#[command(about = "A Rust-base tool for mac")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Solana {
        #[arg(short, long)]
        list: bool,
        #[arg(short, long)]
        clean: bool,
    },
    Rust {
        #[arg(short, long)]
        list: bool,
        #[arg(short, long)]
        clean: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Solana { list, clean } => {
            if list {
                Solana::list_solana_versions();
            }
            if clean {
                Solana::clean_solana_versions();
            }
        }
        Commands::Rust { list, clean } => {
            if list {
                Rust::list_rust_versions();
            }
            if clean {
                Rust::clean_rust_versions();
            }
        }
    };
}
