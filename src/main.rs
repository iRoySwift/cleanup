mod commands;
use clap::{Arg, ArgAction, Command};
use commands::Solana;

use crate::commands::Rust;

fn main() {
    let cmd = Command::new("cleanup")
        .version("1.0")
        .about("A file cleanup utility")
        .subcommand(
            Command::new("solana").about("Clean Solana files").args([
                Arg::new("list")
                    .short('l')
                    .long("list")
                    .help("List Solana files to be cleaned")
                    .action(ArgAction::SetTrue),
                Arg::new("clean")
                    .short('c')
                    .long("clean")
                    .help("Clean Solana files")
                    .action(ArgAction::SetTrue),
            ]),
        )
        .subcommand(
            Command::new("rust")
                .about("Clean Rust build artifacts and caches")
                .args([
                    Arg::new("list")
                        .long("list")
                        .short('l')
                        .help("List Rust files to be cleaned")
                        .action(ArgAction::SetTrue),
                    Arg::new("clean")
                        .long("clean")
                        .short('c')
                        .help("Run the interactive Rust cleanup workflow")
                        .action(ArgAction::SetTrue),
                ]),
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("solana", sub_m)) => {
            if sub_m.get_flag("list") {
                Solana::list_solana_versions();
            }
            if sub_m.get_flag("clean") {
                Solana::clean_solana_versions();
            }
        }
        Some(("rust", sub_m)) => {
            if sub_m.get_flag("list") {
                Rust::list_rust_versions();
            }
            if sub_m.get_flag("clean") {
                Rust::clean_rust_versions();
            }
        }
        _ => println!("No valid subcommand was used"),
    }
}
