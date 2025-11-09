use std::{env, fs, path::Path};

use walkdir::WalkDir;

pub struct Utils;

impl Utils {
    /// 计算文件大小的辅助函数
    pub fn calculate_dir_size(dir: &Path) -> u64 {
        if dir.exists() {
            WalkDir::new(dir)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.metadata().ok())
                .filter(|metadata| metadata.is_file())
                .map(|metadata| metadata.len())
                .sum()
        } else {
            0
        }
    }

    /// kb, mb, gb 格式化大小的辅助函数
    pub fn format_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;

        while size > 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        // const KB: u64 = 1u64 << 10; // 1024
        // const MB: u64 = 1u64 << 20; // 1_048_576
        // const GB: u64 = 1u64 << 30; // 1_073_741_824
        // let (unit, value) = if size >= GB {
        //     ("GB", size as f64 / GB as f64)
        // } else if size >= MB {
        //     ("MB", size as f64 / MB as f64)
        // } else if size >= KB {
        //     ("KB", size as f64 / KB as f64)
        // } else {
        //     ("B", size as f64)
        // };
        format!("{:.2} {}", size, UNITS[unit_index])
    }

    /// 检查命令是否在当前 PATH 中可用
    pub fn command_exists(command: &str) -> bool {
        fn is_executable(candidate: &Path) -> bool {
            if !candidate.is_file() {
                return false;
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = fs::metadata(candidate) {
                    return metadata.permissions().mode() & 0o111 != 0;
                }
                false
            }

            #[cfg(not(unix))]
            {
                true
            }
        }

        if command.contains(std::path::MAIN_SEPARATOR) {
            return is_executable(Path::new(command));
        }

        env::var_os("PATH")
            .map(|paths| {
                env::split_paths(&paths).any(|dir| {
                    let candidate = dir.join(command);
                    is_executable(&candidate)
                })
            })
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// 创建一个临时目录用于测试
    fn create_temp_dir() -> std::path::PathBuf {
        // Build an isolated sandbox so file-system tests never touch real data.
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        dir.push(format!("cleanup_test_{}", nanos));
        fs::create_dir(&dir).expect("Failed to create temp directory");
        dir
    }

    /// 测试格式化大小函数
    #[test]
    fn format_size_scales_units() {
        assert_eq!(Utils::format_size(0), "0.00 B");
        assert_eq!(Utils::format_size(512), "512.00 B");
        assert_eq!(Utils::format_size(2048), "2.00 KB");
        assert_eq!(Utils::format_size(5_242_880), "5.00 MB");
    }

    /// 测试计算目录大小函数
    #[test]
    fn calculate_dir_size_sums_nested_files() {
        // temp_dir acts as the root of the ephemeral directory tree used in this test.
        let temp_dir = create_temp_dir();

        let top_file = temp_dir.join("top.bin");
        let mut file = File::create(&top_file).expect("Failed to create top file");
        file.write_all(&vec![0u8; 2048])
            .expect("Failed to write top file");

        let nested = temp_dir.join("nested");
        fs::create_dir(&nested).expect("Failed to create nested directory");
        let nested_file = nested.join("nested.bin");
        let mut file = File::create(&nested_file).expect("Failed to create nested file");
        file.write_all(&vec![0u8; 1024])
            .expect("Failed to write nested file");

        let measured = Utils::calculate_dir_size(&temp_dir);
        assert_eq!(measured, 2048 + 1024);

        fs::remove_dir_all(&temp_dir).expect("Failed to remove temp directory");
    }

    #[test]
    fn command_exists_detects_binaries() {
        // macOS typically ships with /usr/bin/env; treat it as a stable probe.
        assert!(Utils::command_exists("env"));
        assert!(!Utils::command_exists(
            "cleanup-command-that-should-not-exist"
        ));
    }
}
