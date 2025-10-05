#!/bin/bash

# DD Registry CW 部署脚本
# 用于部署推荐积分合约到 Luckee 网络

set -e

# 配置变量
CONTRACT_NAME="dd-registry-cw"
CONTRACT_LABEL="DD Registry CW - 推荐积分合约"
ADMIN_ADDRESS="luckee1..."  # 请替换为实际的管理员地址
NODE_URL="https://rpc.luckee.network"  # 请替换为实际的节点URL
CHAIN_ID="luckee-1"  # 请替换为实际的链ID

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查依赖
check_dependencies() {
    log_info "检查依赖..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo 未安装，请先安装 Rust"
        exit 1
    fi
    
    if ! command -v wasmd &> /dev/null; then
        log_error "wasmd 未安装，请先安装 CosmWasm CLI"
        exit 1
    fi
    
    if ! command -v cosmwasm-opt &> /dev/null; then
        log_error "cosmwasm-opt 未安装，请先安装 CosmWasm 优化工具"
        exit 1
    fi
    
    log_info "依赖检查完成"
}

# 编译合约
build_contract() {
    log_info "编译合约..."
    
    # 编译 WASM
    cargo wasm
    
    if [ $? -ne 0 ]; then
        log_error "合约编译失败"
        exit 1
    fi
    
    log_info "合约编译完成"
}

# 优化合约
optimize_contract() {
    log_info "优化合约..."
    
    # 优化 WASM
    cosmwasm-opt target/wasm32-unknown-unknown/release/${CONTRACT_NAME}.wasm
    
    if [ $? -ne 0 ]; then
        log_error "合约优化失败"
        exit 1
    fi
    
    log_info "合约优化完成"
}

# 上传合约
upload_contract() {
    log_info "上传合约..."
    
    # 上传合约代码
    UPLOAD_RESULT=$(wasmd tx wasm store target/wasm32-unknown-unknown/release/${CONTRACT_NAME}.wasm \
        --from admin \
        --gas auto \
        --gas-adjustment 1.3 \
        --node ${NODE_URL} \
        --chain-id ${CHAIN_ID} \
        --output json \
        --yes)
    
    if [ $? -ne 0 ]; then
        log_error "合约上传失败"
        exit 1
    fi
    
    # 提取合约代码ID
    CODE_ID=$(echo $UPLOAD_RESULT | jq -r '.logs[0].events[0].attributes[0].value')
    
    if [ "$CODE_ID" = "null" ] || [ -z "$CODE_ID" ]; then
        log_error "无法提取合约代码ID"
        exit 1
    fi
    
    log_info "合约上传成功，代码ID: ${CODE_ID}"
    echo "CODE_ID=${CODE_ID}" > .env
}

# 实例化合约
instantiate_contract() {
    log_info "实例化合约..."
    
    # 读取代码ID
    if [ ! -f .env ]; then
        log_error ".env 文件不存在，请先上传合约"
        exit 1
    fi
    
    source .env
    
    # 实例化合约
    INSTANTIATE_RESULT=$(wasmd tx wasm instantiate ${CODE_ID} \
        '{
            "admin": "'${ADMIN_ADDRESS}'",
            "config": {
                "enabled": true,
                "max_referral_depth": 3,
                "referral_cooldown": 3600,
                "max_daily_referrals": 10,
                "points_decay_period": 30,
                "points_decay_rate": "0.01",
                "min_withdrawal_amount": "1000",
                "emergency_paused": false
            },
            "points_rules": {
                "direct_referral_rate": "0.5",
                "level_2_rate": "0.2",
                "level_3_rate": "0.1",
                "base_points": "100",
                "level_multipliers": {},
                "activity_rules": []
            }
        }' \
        --from admin \
        --label "${CONTRACT_LABEL}" \
        --gas auto \
        --gas-adjustment 1.3 \
        --node ${NODE_URL} \
        --chain-id ${CHAIN_ID} \
        --output json \
        --yes)
    
    if [ $? -ne 0 ]; then
        log_error "合约实例化失败"
        exit 1
    fi
    
    # 提取合约地址
    CONTRACT_ADDRESS=$(echo $INSTANTIATE_RESULT | jq -r '.logs[0].events[0].attributes[0].value')
    
    if [ "$CONTRACT_ADDRESS" = "null" ] || [ -z "$CONTRACT_ADDRESS" ]; then
        log_error "无法提取合约地址"
        exit 1
    fi
    
    log_info "合约实例化成功，地址: ${CONTRACT_ADDRESS}"
    echo "CONTRACT_ADDRESS=${CONTRACT_ADDRESS}" >> .env
}

# 验证部署
verify_deployment() {
    log_info "验证部署..."
    
    source .env
    
    # 查询合约信息
    CONTRACT_INFO=$(wasmd query wasm contract ${CONTRACT_ADDRESS} \
        --node ${NODE_URL} \
        --output json)
    
    if [ $? -ne 0 ]; then
        log_error "合约验证失败"
        exit 1
    fi
    
    log_info "合约验证成功"
    log_info "合约地址: ${CONTRACT_ADDRESS}"
    log_info "合约代码ID: ${CODE_ID}"
}

# 运行测试
run_tests() {
    log_info "运行测试..."
    
    # 运行单元测试
    cargo test
    
    if [ $? -ne 0 ]; then
        log_error "单元测试失败"
        exit 1
    fi
    
    # 运行集成测试
    cargo test --test integration
    
    if [ $? -ne 0 ]; then
        log_error "集成测试失败"
        exit 1
    fi
    
    log_info "测试完成"
}

# 主函数
main() {
    log_info "开始部署 DD Registry CW 合约..."
    
    # 检查依赖
    check_dependencies
    
    # 运行测试
    run_tests
    
    # 编译合约
    build_contract
    
    # 优化合约
    optimize_contract
    
    # 上传合约
    upload_contract
    
    # 实例化合约
    instantiate_contract
    
    # 验证部署
    verify_deployment
    
    log_info "部署完成！"
    log_info "请保存以下信息："
    source .env
    echo "合约地址: ${CONTRACT_ADDRESS}"
    echo "合约代码ID: ${CODE_ID}"
    echo "管理员地址: ${ADMIN_ADDRESS}"
}

# 清理函数
cleanup() {
    log_info "清理临时文件..."
    rm -f .env
}

# 错误处理
trap cleanup EXIT

# 运行主函数
main "$@"
