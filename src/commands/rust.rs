use colored::Colorize;
use dialoguer::{MultiSelect, theme::ColorfulTheme};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::commands::Utils;
pub struct Rust;

#[derive(Debug)]
pub struct RustInfo {
    name: String,
    pub size: u64,
    is_active: bool,
    version: Option<String>,
}

impl Rust {
    /// 获取单个 Rust 工具链的信息
    fn get_rust_version_info(_toolchain_path: &PathBuf) -> Option<String> {
        let rust_bin = _toolchain_path.join("bin/rustc");
        if !rust_bin.exists() {
            return None;
        }
        let output = Command::new(rust_bin).arg("--version").output().ok()?;

        String::from_utf8(output.stdout)
            .ok()
            .and_then(|s| s.split_whitespace().nth(1).map(|v| v.to_string()))
    }
    /// 获取当前激活的 Rust 工具链
    fn get_active_rust_version() -> Option<String> {
        if !Utils::command_exists("rustup") {
            eprintln!("rustup not found in PATH.");
            return None;
        }
        let output = std::process::Command::new("rustup")
            .args(["show", "active-toolchain"])
            .output()
            .ok()?;
        let stdout = String::from_utf8(output.stdout).ok()?;
        stdout.split_whitespace().next().map(|s| s.to_string())
    }
    /// 获取所有 Rust 工具链
    pub fn get_rust_versions() -> Vec<RustInfo> {
        if !Utils::command_exists("rustup") {
            eprintln!("rustup command not available; skipping Rust toolchain inspection.");
            return Vec::new();
        }
        let home = match std::env::var("HOME") {
            Ok(path) => path,
            Err(err) => {
                eprintln!("HOME environment variable is not set: {}", err);
                return Vec::new();
            }
        };
        let rustup_path = Path::new(&home).join(".rustup/toolchains");
        if !rustup_path.exists() {
            return Vec::new();
        }

        let active_toolchain = Self::get_active_rust_version();
        let mut versions = Vec::new();
        let entries = match std::fs::read_dir(&rustup_path) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("Failed to read Rust toolchain directory: {}", err);
                return Vec::new();
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("Failed to read toolchain entry: {}", err);
                    continue;
                }
            };
            let path = entry.path();
            if !path.exists() {
                continue;
            }
            let Some(name) = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
            else {
                continue;
            };
            let size = Utils::calculate_dir_size(&path);
            let is_active = active_toolchain.as_ref().is_some_and(|v| name.contains(v));
            let version = Self::get_rust_version_info(&path);
            versions.push(RustInfo {
                name,
                size,
                is_active,
                version,
            });
        }
        versions
    }

    /// 列出所有 Rust 工具链
    pub fn list_rust_versions() {
        println!("{}", "🦀 Rust versions:".bold().cyan());
        println!();

        let versions = Self::get_rust_versions();
        if versions.is_empty() {
            println!("No Rust versions found.\n");
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

            let name = format!("{}{}", version.name, version_info);
            let size_color = Utils::format_size(version.size).yellow();
            println!("{} {} {}", name, size_color, status);
        }

        println!();
        println!(
            "Total: {} version, {}",
            versions.len(),
            Utils::format_size(total_size).bold()
        );
    }

    pub fn clean_rust_versions() {
        println!("{}", "🦀 Rust clean:".bold().cyan());
        println!();

        if !Utils::command_exists("rustup") {
            eprintln!("rustup command not available; cannot clean toolchains.");
            return;
        }

        let list = Self::get_rust_versions();
        if list.is_empty() {
            println!("No Rust versions found.\n");
            return;
        }

        println!("{}", "🧹 Cleaning Rust Versions:".bold().cyan());

        let selections = match MultiSelect::with_theme(&ColorfulTheme::default())
            .items(
                list.iter()
                    .map(|v| format!("{} ({})", v.name, Utils::format_size(v.size)))
                    .collect::<Vec<String>>(),
            )
            .interact()
        {
            Ok(selection) => selection,
            Err(err) => {
                eprintln!("Failed to read selection: {}", err);
                return;
            }
        };

        if selections.is_empty() {
            println!("No Rust versions selected.");
            return;
        }

        for &index in &selections {
            let select = &list[index];

            let output = Command::new("rustup")
                .arg("uninstall")
                .arg(&select.name)
                .output();

            println!("Removing {}...", select.name);
            match output {
                Ok(output) if output.status.success() => {
                    println!("✓ Removed {}", select.name.green());
                }
                _ => println!("✗ Failed to remove {}", select.name.red()),
            }
        }
    }
}
