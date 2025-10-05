#!/bin/bash

# 生成 DD Registry CW 合约的 JSON Schema

set -e

echo "生成 DD Registry CW 合约的 JSON Schema..."

# 创建 schema 目录
mkdir -p schema

# 生成 schema
cargo run --bin schema

echo "Schema 生成完成！"
