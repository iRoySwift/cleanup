# Cleanup

Cleanup 是一个面向 macOS 的命令行工具，用于快速检查和整理本地的开发环境资源，包括 Rust 工具链、Solana 安装以及 iOS 模拟器。通过简单的命令或交互式向导，你可以了解磁盘占用情况、清理不再需要的版本，保持开发环境整洁有序。

## ✨ 特性

- 查看 Rust、Solana 与 iOS 模拟器的安装占用情况
- 交互式清理向导，逐步完成资源回收
- 支持单独列出或删除特定组件
- 输出带颜色的终端信息，便于阅读

## 📦 环境要求

- macOS
- Rust 1.81 或更新版本（建议使用 `rustup`）
- 已安装的 Rust、Solana 开发环境或 Xcode（以便检测和清理）

## 🛠️ 构建与运行

```bash
# 构建可执行文件
cargo build --release

# 运行帮助
cargo run -- --help
```

## 🚀 使用示例

### 查看磁盘占用

```bash
cargo run -- status
```

### 启动交互式清理向导

```bash
cargo run -- interactive
```

### 管理 Rust 工具链

```bash
# 列出已安装的 Rust 工具链
cargo run -- rust --list

# 清理选定的 Rust 工具链
cargo run -- rust --clean
```

### 管理 Solana 版本

```bash
cargo run -- solana --list
cargo run -- solana --clean
```

### 管理 iOS 模拟器

```bash
cargo run -- simulator --list
cargo run -- simulator --clean
```

## 📁 项目结构

```
src/
  main.rs              # CLI 入口，定义子命令
  commands/
    interactive.rs     # 交互式向导
    rust.rs            # Rust 工具链管理
    solana.rs          # Solana 版本管理
    simulator.rs       # iOS 模拟器管理
    status.rs          # 磁盘占用汇总
    utils.rs           # 公共工具函数
```

## 🧪 测试

```bash
# 运行所有单元测试
cargo test

# 只运行 utils 模块中的 format_size_scales_units 测试并输出原始日志
cargo test -p cleanup format_size_scales_units -- --nocapture

# 只运行 calculate_dir_size_sums_nested_files 测试
cargo test -p cleanup calculate_dir_size_sums_nested_files

# 验证 command_exists 检测脚本
cargo test -p cleanup command_exists_detects_binaries

# 手工执行 CI 中的三个步骤：格式检查、Clippy、测试
cargo fmt -- --check
cargo clippy -- -D warnings
```

功能验证建议：

1. 在沙盒环境中运行 `cargo run -- status`，确认信息统计是否准确。
2. 为 Rust/Solana 准备多个版本，执行 `--clean` 子命令验证筛选逻辑。
3. 在删除 iOS 模拟器前先确认目标设备确实可以移除。

## 🤝 贡献

欢迎提交 Issue 或 PR 来帮助改进项目。如果你有新的清理目标或优化建议，也可以直接提出讨论。

## 📄 许可证

本项目基于 MIT License 发布，详情见 [LICENSE](LICENSE)。
