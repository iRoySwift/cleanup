use crate::commands::{Rust, Simulator, Solana, Status};
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};

pub struct Interactive;

/// 清理 Rust、Solana 和 Simulator 的交互式向导
#[allow(dead_code)]
impl Interactive {
    fn cleanup_rust() {
        let rust_cleanup = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Clean up Rust toolchains?")
            .interact()
            .unwrap();

        if rust_cleanup {
            Rust::clean_rust_versions();
            println!();
        }
    }
    fn cleanup_solana() {
        let solana_cleanup = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Clean up Solana installations?")
            .interact()
            .unwrap();

        if solana_cleanup {
            Solana::clean_solana_versions();
            println!();
        }
    }
    fn cleanup_simulator() {
        let simulator_cleanup = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Clean up Simulator installations?")
            .interact()
            .unwrap();

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
