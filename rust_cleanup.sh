#!/bin/bash

echo "=== Rust Toolchain Cleanup Script ==="
echo

# 显示当前工具链
echo "Current toolchains:"
rustup toolchain list
echo

# 显示存储使用情况
echo "Storage usage:"
du -sh ~/.rustup/toolchains/*
echo

# 询问是否删除旧版本
echo "Do you want to remove old versioned toolchains? (y/N)"
read -r response

if [[ "$response" =~ ^[Yy]$ ]]; then
    echo "Removing old versioned toolchains..."

    # 获取所有版本号工具链（不包括stable/nightly）
    versioned_toolchains=$(rustup toolchain list | grep -E '^[0-9]+\.[0-9]+\.[0-9]+-' | cut -d' ' -f1)

    for toolchain in $versioned_toolchains; do
        echo "Removing $toolchain..."
        rustup toolchain uninstall "$toolchain"
    done

    # 获取特定日期的nightly版本
    dated_nightlies=$(rustup toolchain list | grep -E '^nightly-[0-9]{4}-[0-9]{2}-[0-9]{2}-' | cut -d' ' -f1)

    for toolchain in $dated_nightlies; do
        echo "Removing $toolchain..."
        rustup toolchain uninstall "$toolchain"
    done

    echo "Cleanup completed!"
    echo
    echo "Remaining toolchains:"
    rustup toolchain list
    echo
    echo "Storage after cleanup:"
    du -sh ~/.rustup/toolchains/*
else
    echo "Cleanup cancelled."
fi
