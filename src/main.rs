mod commands;
use crate::commands::{Interactive, Rust, Simulator, Solana, Status};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cleanup")]
#[command(about = "A Rust-base tool for mac")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show storage usage summary
    Status,
    /// Interactive cleanup wizard
    Interactive,
    /// Manage Solana installations
    Solana {
        #[arg(short, long, help = "List all installed Solana versions")]
        list: bool,
        #[arg(short, long, help = "Clean up Solana versions")]
        clean: bool,
    },
    /// Manage Rust installations
    Rust {
        #[arg(short, long, help = "List all installed Rust versions")]
        list: bool,
        #[arg(short, long, help = "Clean up Rust versions")]
        clean: bool,
    },
    /// Manage Simulator installations
    Simulator {
        #[arg(short, long, help = "List all installed Simulator versions")]
        list: bool,
        #[arg(short, long, help = "Clean up Simulator versions")]
        clean: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            Status::show_status();
        }
        Commands::Interactive => {
            Interactive::run_wizard();
        }
        Commands::Solana { list, clean } => {
            if list {
                Solana::show_solana_versions();
            }
            if clean {
                Solana::clean_solana_versions();
            }
        }
        Commands::Rust { list, clean } => {
            if list {
                Rust::show_rust_versions();
            }
            if clean {
                Rust::clean_rust_versions();
            }
        }
        Commands::Simulator { list, clean } => {
            if list {
                Simulator::show_simulator_versions();
            }
            if clean {
                Simulator::clean_simulators();
            }
        }
    };
}
