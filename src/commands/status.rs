use crate::commands::{Rust, Simulator, Solana, Utils};
use colored::Colorize;

pub struct Status;

impl Status {
    pub fn show_status() {
        println!("{}", "📊 Storage Usage Summary:".bold().cyan());
        println!();

        let rusts = Rust::get_rust_versions();
        let solanas = Solana::get_solana_versions();
        let simulators = Simulator::get_simulators();

        let rust_total: u64 = rusts.iter().map(|r| r.size).sum();
        let solana_total: u64 = solanas.iter().map(|s| s.size).sum();
        let simulator_total: u64 = simulators.iter().map(|s| s.size).sum();

        println!(
            "🦀 Rust Toolchains: {} toolchains, {}",
            rusts.len(),
            Utils::format_size(rust_total).yellow()
        );

        println!(
            "⚡ Solana Versions: {} versions, {}",
            solanas.len(),
            Utils::format_size(solana_total).yellow()
        );

        let unavailable_simulators = simulators.iter().filter(|s| !s.is_available).count();
        println!(
            "📱 iOS Simulators: {} simulators ({} unavailable), {}",
            simulators.len(),
            unavailable_simulators,
            Utils::format_size(simulator_total).yellow()
        );

        println!();
        println!(
            "{} Total: {}",
            "💾".bold(),
            Utils::format_size(rust_total + solana_total + simulator_total)
                .bold()
                .green()
        );
    }
}
