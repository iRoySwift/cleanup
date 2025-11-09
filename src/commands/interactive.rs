use crate::commands::{Rust, Simulator, Solana, Status};
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};

pub struct Interactive;

/// 清理 Rust、Solana 和 Simulator 的交互式向导
#[allow(dead_code)]
impl Interactive {
    fn prompt(prompt: &str) -> bool {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
        {
            Ok(choice) => choice,
            Err(err) => {
                eprintln!("Failed to read input: {}", err);
                false
            }
        }
    }

    fn cleanup_rust() {
        let rust_cleanup = Self::prompt("Clean up Rust toolchains?");

        if rust_cleanup {
            Rust::clean_rust_versions();
            println!();
        }
    }
    fn cleanup_solana() {
        let solana_cleanup = Self::prompt("Clean up Solana installations?");

        if solana_cleanup {
            Solana::clean_solana_versions();
            println!();
        }
    }
    fn cleanup_simulator() {
        let simulator_cleanup = Self::prompt("Clean up Simulator installations?");

        if simulator_cleanup {
            Simulator::clean_simulators();
            println!();
        }
    }
    pub fn run_wizard() {
        println!("{}", "🎯 Interactive Cleanup Wizard".bold().cyan());
        println!();

        Status::show_status();
        println!();

        Self::cleanup_rust();
        Self::cleanup_solana();
        Self::cleanup_simulator();

        println!("{}", "🎉 Cleanup completed!".bold().green());
        Status::show_status();
    }
}
