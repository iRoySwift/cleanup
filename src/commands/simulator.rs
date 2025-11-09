use colored::Colorize;
use dialoguer::{MultiSelect, theme::ColorfulTheme};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, process::Command};

use crate::commands::Utils;

#[derive(Deserialize)]
struct RuntimeList {
    runtimes: Vec<RuntimeEntry>,
}

#[derive(Debug, Deserialize)]
struct RuntimeEntry {
    name: String,
    identifier: String,
    #[serde(default, rename = "bundlePath")]
    path: String,
    #[serde(default, rename = "isAvailable")]
    is_available: bool,
    #[serde(default)]
    version: Option<String>,
}

#[derive(Deserialize)]
struct DeviceList {
    devices: HashMap<String, Vec<DeviceEntry>>,
}

#[derive(Debug, Clone, Deserialize)]
struct DeviceEntry {
    name: String,
    #[serde(default, rename = "udid")]
    identifier: String,
    #[serde(default, rename = "dataPath")]
    path: String,
    #[serde(default, rename = "isAvailable")]
    is_available: bool,
    #[serde(default, rename = "dataPathSize")]
    size: u64,
}

#[derive(Debug)]
pub struct SimulatorInfo {
    name: String,
    identifier: String,
    path: String,
    version: Option<String>,
    pub size: u64,
    pub is_available: bool,
    simulator_type: String,
}

pub struct Simulator;

impl Simulator {
    /// 获取所有 Simulator 设备
    fn get_simulator_devices() -> HashMap<String, Vec<DeviceEntry>> {
        let output = match Command::new("xcrun")
            .args(["simctl", "list", "devices", "-j"])
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Failed to execute xcrun simctl list devices -j: {}", err);
                return HashMap::new();
            }
        };

        match serde_json::from_slice::<DeviceList>(&output.stdout) {
            Ok(parsed) => parsed.devices,
            Err(err) => {
                eprintln!("Failed to parse devices JSON: {}", err);
                HashMap::new()
            }
        }
    }

    /// 获取所有 Simulator 运行时和设备信息
    fn get_simulator_runtimes() -> Vec<RuntimeEntry> {
        let output = match Command::new("xcrun")
            .args(["simctl", "list", "runtimes", "-j"])
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Failed to execute xcrun simctl list runtimes -j: {}", err);
                return Vec::new();
            }
        };

        match serde_json::from_slice::<RuntimeList>(&output.stdout) {
            Ok(parsed) => parsed.runtimes,
            Err(err) => {
                eprintln!("Failed to parse runtimes JSON: {}", err);
                Vec::new()
            }
        }
    }

    pub fn get_simulators() -> Vec<SimulatorInfo> {
        let runtimes = Self::get_simulator_runtimes();
        let devices = Self::get_simulator_devices();
        let mut simulators: Vec<SimulatorInfo> = Vec::new();

        for runtime in runtimes.into_iter() {
            let runtime_size = Utils::calculate_dir_size(PathBuf::from(&runtime.path).as_path());
            let simulator = SimulatorInfo {
                name: runtime.name,
                identifier: runtime.identifier.clone(),
                path: runtime.path,
                version: runtime.version,
                size: runtime_size,
                is_available: runtime.is_available,
                simulator_type: "runtime".to_string(),
            };
            simulators.push(simulator);

            let runtime_devices: Vec<DeviceEntry> = devices
                .get(&runtime.identifier)
                .cloned()
                .unwrap_or_default();
            for runtime_device in runtime_devices.into_iter() {
                let simulator = SimulatorInfo {
                    name: runtime_device.name,
                    identifier: runtime_device.identifier,
                    path: runtime_device.path,
                    version: Some("".to_string()),
                    size: runtime_device.size,
                    is_available: runtime_device.is_available,
                    simulator_type: "device".to_string(),
                };
                simulators.push(simulator);
            }
        }
        simulators
    }

    pub fn list_simulators() {
        println!("{}", "📱 iOS Simulators:".bold().cyan());
        println!();
        let simulators = Self::get_simulators();
        if simulators.is_empty() {
            println!("No iOS simulators found.");
            return;
        }
        let mut total_size = 0;
        for simulator in &simulators {
            total_size += simulator.size;
            let status = if simulator.is_available {
                "  available".green()
            } else {
                "  unavailable".red()
            };

            let simulator_info = simulator
                .version
                .as_ref()
                .map(|v| {
                    if v.is_empty() {
                        "".to_string()
                    } else {
                        format!(" ({})", v)
                    }
                })
                .unwrap_or_default();

            let name_block = format!("{}{}", simulator.name, simulator_info);
            let size_raw = Utils::format_size(simulator.size);
            let size_colored = format!("{:>10}", size_raw).yellow();
            if simulator.simulator_type == "runtime" {
                println!("{} {:<52} {:>10} {}", "", name_block, size_colored, status);
            } else {
                println!(
                    "{} {:<50} {:>10} {}",
                    " -", name_block, size_colored, status
                );
            };
        }
        println!();
        println!(
            "Total: {:?} simulators, {}",
            simulators.len(),
            Utils::format_size(total_size).bold()
        );
    }
    pub fn clean_simulators() {
        println!("{}", "🧹 Cleaning Simulator:".bold().cyan());
        println!();
        let list = Self::get_simulators();
        if list.is_empty() {
            println!("No iOS simulators found.");
            return;
        }

        println!("{}", "🧹 Cleaning Simulators:".bold().cyan());

        let selections = match MultiSelect::with_theme(&ColorfulTheme::default())
            .items(
                list.iter()
                    .map(|simulator| {
                        let status = if simulator.is_available {
                            "  available".green()
                        } else {
                            "  unavailable".red()
                        };
                        let simulator_info = simulator
                            .version
                            .as_ref()
                            .map(|v| {
                                if v.is_empty() {
                                    "".to_string()
                                } else {
                                    format!(" ({})", v)
                                }
                            })
                            .unwrap_or_default();
                        let name_block = format!("{}{}", simulator.name, simulator_info);
                        let size_raw = Utils::format_size(simulator.size);
                        let size_colored = format!("{:>10}", size_raw).yellow();
                        if simulator.simulator_type == "runtime" {
                            format!("{} {:<52} {:>10} {}", "", name_block, size_colored, status)
                        } else {
                            format!(
                                "{} {:<50} {:>10} {}",
                                " -", name_block, size_colored, status
                            )
                        }
                    })
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
            println!("No simulator selected.");
            return;
        }
        for &index in &selections {
            let select = &list[index];

            let id = if select.simulator_type == "runtime" {
                &select.name
            } else {
                &select.identifier
            };
            let output = Command::new("xcrun")
                .args(["simctl", "delete"])
                .arg(id)
                .output();

            println!("Removing {}...", select.name);
            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("✓ Removed {}", select.name.green());
                    } else {
                        println!(
                            "✗ Failed to remove {}: {}",
                            select.name.red(),
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(err) => println!("✗ Failed to remove {}: {}", select.name.red(), err),
            }
        }
    }
}
