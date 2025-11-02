use colored::Colorize;
use dialoguer::{MultiSelect, theme::ColorfulTheme};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::commands::Utils;
pub struct Solana;

#[derive(Debug)]
struct SolanaVersion {
    name: String,
    path: PathBuf,
    size: u64,
    is_active: bool,
    version: Option<String>,
}

impl Solana {
    /// 获取单个 Solana 版本的信息
    fn get_solana_version_info(version_path: &PathBuf) -> Option<String> {
        let solana_bin = version_path.join("solana-release/bin/solana");
        if !solana_bin.exists() {
            return None;
        }
        let output = Command::new(solana_bin).arg("--version").output().ok()?;
        String::from_utf8(output.stdout)
            .ok()
            .and_then(|s| s.split_whitespace().nth(1).map(|v| v.to_string()))
    }

    /// 获取所有 Solana 版本
    fn get_solana_versions() -> Vec<SolanaVersion> {
        let home = std::env::home_dir().unwrap();
        let solana_dir = home.join(".local/share/solana/install/releases");

        if !solana_dir.exists() {
            return Vec::new();
        }
        let mut versions = Vec::new();
        let active_version = Self::get_active_solana_version();

        for entry in std::fs::read_dir(&solana_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.exists() {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                let size = Utils::calculate_dir_size(&path);
                let is_active = active_version.as_ref().is_some_and(|v| name.contains(v));
                let version = Self::get_solana_version_info(&path);
                versions.push(SolanaVersion {
                    name,
                    path,
                    size,
                    is_active,
                    version,
                });
            }
        }

        versions
    }

    /// 获取当前激活的 Solana 版本
    fn get_active_solana_version() -> Option<String> {
        let home = std::env::var("HOME").unwrap();
        let active_link = Path::new(&home).join(".local/share/solana/install/active_release");

        if let Ok(target) = fs::read_link(&active_link) {
            target
                .to_string_lossy()
                .split('/')
                .find(|s| s.starts_with("stable-"))
                .map(|s| s.to_string())
        } else {
            None
        }
    }

    /// 列出 Solana 版本
    pub fn list_solana_versions() {
        println!("{}", "⚡ Solana Versions:".bold().cyan());
        println!();
        let versions = Self::get_solana_versions();
        if versions.is_empty() {
            println!("No Solana versions found.");
            return;
        }
        let mut total_size = 0;
        for version in &versions {
            total_size += version.size;
            let status = if version.is_active {
                "✓ active".green()
            } else {
                "  inactive".red()
            };

            let version_info = version
                .version
                .as_ref()
                .map(|v| format!(" ({})", v))
                .unwrap_or_default();
            println!(
                "{:<50} {:>10} {}",
                format!("{}{}", version.name, version_info),
                Utils::format_size(version.size).yellow(),
                status
            );
        }
        println!();
        println!(
            "Total: {:?} versions, {}",
            versions.len(),
            Utils::format_size(total_size).bold()
        );
    }

    /// 清理 Solana 版本
    pub fn clean_solana_versions() {
        let versions = Self::get_solana_versions();
        let inactive_versions: Vec<&SolanaVersion> =
            versions.iter().filter(|v| !v.is_active).collect();
        if inactive_versions.is_empty() {
            println!("{}", "No inactive Solana versions to clean.".bold().green());
            return;
        }

        println!("{}", "🧹 Cleaning Solana Versions:".bold().cyan());

        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select versions to remove:")
            .items(
                &inactive_versions
                    .iter()
                    .map(|v| format!("{} ({})", v.name, Utils::format_size(v.size)))
                    .collect::<Vec<String>>(),
            )
            .interact()
            .unwrap();

        if selections.is_empty() {
            println!("No versions selected.");
            return;
        }

        for &index in &selections {
            let version = &inactive_versions[index];
            println!("Removing {}...", version.name);
            if fs::remove_dir_all(&version.path).is_ok() {
                println!("{} removed.", version.name.green());
            } else {
                println!("Failed to remove {}.", version.name.red());
            }
        }
    }
}
