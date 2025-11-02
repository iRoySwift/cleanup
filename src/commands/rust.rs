use colored::Colorize;
use dialoguer::{MultiSelect, theme::ColorfulTheme};
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::commands::Utils;
pub struct Rust;

#[derive(Debug)]
struct RustVersion {
    name: String,
    path: PathBuf,
    size: u64,
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
        let output = std::process::Command::new("rustup")
            .arg("show")
            .arg("active-toolchain")
            .output()
            .ok()?;
        let stdout = String::from_utf8(output.stdout).ok()?;
        stdout.split_whitespace().next().map(|s| s.to_string())
    }
    /// 获取所有 Rust 工具链
    fn get_rust_versions() -> Vec<RustVersion> {
        let home = std::env::var("HOME").unwrap();
        let rustup_path = Path::new(&home).join(".rustup/toolchains");
        if !rustup_path.exists() {
            return Vec::new();
        }

        let active_toolchain = Self::get_active_rust_version();
        let mut versions = Vec::new();
        for entry in std::fs::read_dir(&rustup_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.exists() {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                let size = Utils::calculate_dir_size(&path);
                let is_active = active_toolchain.as_ref().is_some_and(|v| name.contains(v));
                let version = Self::get_rust_version_info(&path);
                versions.push(RustVersion {
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
            let status = version
                .is_active
                .then(|| "✓ active".green())
                .unwrap_or("  inactive".red());
            let version_info = version
                .version
                .as_ref()
                .map(|v| format!(" ({})", v))
                .unwrap_or_default();

            println!(
                "{} {} {}",
                format!("{}{}", version.name, version_info),
                Utils::format_size(version.size).yellow(),
                status
            );
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

        let versions = Self::get_rust_versions();
        if versions.is_empty() {
            println!("No Rust versions found.\n");
            return;
        }

        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .items(
                versions
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
            let output = Command::new("rustup")
                .arg("uninstall")
                .arg(&versions[index].name)
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    println!("✓ Removed {}", versions[index].name.green());
                }
                _ => println!("✗ Failed to remove {}", versions[index].name.red()),
            }
        }
    }
}
