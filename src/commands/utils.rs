use std::path::Path;

use walkdir::WalkDir;

pub struct Utils;

impl Utils {
    /// 计算文件大小的辅助函数
    pub fn calculate_dir_size(dir: &Path) -> u64 {
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.metadata().ok())
            .filter(|metadata| metadata.is_file())
            .map(|metadata| metadata.len())
            .sum()
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
}
