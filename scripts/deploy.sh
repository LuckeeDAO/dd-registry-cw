#!/bin/bash

# DD Registry CW 合约部署脚本
# 基于文档要求实现自动化部署

set -e

# 配置变量
CONTRACT_NAME="dd-registry-cw"
CONTRACT_VERSION="1.0.0"
NETWORK="testnet"
CHAIN_ID="luckee-testnet-1"
NODE_URL="https://rpc.luckee-testnet.com"
GAS_PRICES="0.025uluckee"
GAS_ADJUSTMENT="1.5"
ADMIN_ADDRESS="luckee1admin..."  # 替换为实际管理员地址

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
    log_info "检查部署依赖..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo 未安装"
        exit 1
    fi
    
    if ! command -v cosmwasm-opt &> /dev/null; then
        log_warn "cosmwasm-opt 未安装，将使用未优化的版本"
    fi
    
    if ! command -v wasmd &> /dev/null; then
        log_error "wasmd 未安装"
        exit 1
    fi
    
    log_info "依赖检查完成"
}

# 构建合约
build_contract() {
    log_info "构建 DD Registry CW 合约..."
    
    cd /home/lc/luckee_dao/dd-registry-cw
    
    # 清理之前的构建
    cargo clean
    
    # 构建合约
    cargo build --release --target wasm32-unknown-unknown
    
    if [ $? -ne 0 ]; then
        log_error "合约构建失败"
        exit 1
    fi
    
    # 优化WASM文件
    if command -v cosmwasm-opt &> /dev/null; then
        log_info "优化 WASM 文件..."
        cosmwasm-opt target/wasm32-unknown-unknown/release/dd_registry_cw.wasm \
            -o target/wasm32-unknown-unknown/release/dd_registry_cw_optimized.wasm
        WASM_FILE="target/wasm32-unknown-unknown/release/dd_registry_cw_optimized.wasm"
    else
        WASM_FILE="target/wasm32-unknown-unknown/release/dd_registry_cw.wasm"
    fi
    
    log_info "合约构建完成: $WASM_FILE"
}

# 上传合约
upload_contract() {
    log_info "上传合约到链上..."
    
    # 检查WASM文件大小
    WASM_SIZE=$(stat -c%s "$WASM_FILE")
    MAX_SIZE=$((1024 * 1024))  # 1MB限制
    
    if [ $WASM_SIZE -gt $MAX_SIZE ]; then
        log_error "WASM文件过大: ${WASM_SIZE} bytes > ${MAX_SIZE} bytes"
        exit 1
    fi
    
    log_info "WASM文件大小: ${WASM_SIZE} bytes"
    
    # 上传合约
    UPLOAD_RESULT=$(wasmd tx wasm store "$WASM_FILE" \
        --from "$ADMIN_ADDRESS" \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --gas-prices "$GAS_PRICES" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --yes \
        --output json)
    
    if [ $? -ne 0 ]; then
        log_error "合约上传失败"
        exit 1
    fi
    
    # 提取代码ID
    CODE_ID=$(echo "$UPLOAD_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
    
    if [ -z "$CODE_ID" ]; then
        log_error "无法获取代码ID"
        exit 1
    fi
    
    log_info "合约上传成功，代码ID: $CODE_ID"
    echo "$CODE_ID" > /tmp/dd_registry_cw_code_id.txt
}

# 实例化合约
instantiate_contract() {
    log_info "实例化 DD Registry CW 合约..."
    
    CODE_ID=$(cat /tmp/dd_registry_cw_code_id.txt)
    
    # 实例化消息
    INSTANTIATE_MSG='{
        "admin": "'$ADMIN_ADDRESS'",
        "config": {
            "max_referral_depth": 3,
            "referral_cooldown": 3600,
            "max_daily_referrals": 10,
            "points_decay_rate": "0.01",
            "min_points_for_withdrawal": "1000"
        },
        "points_rules": {
            "base_points_per_referral": "100",
            "level_multipliers": {
                "bronze": "1.0",
                "silver": "1.2",
                "gold": "1.5",
                "platinum": "2.0"
            },
            "referral_bonus_rates": {
                "direct": "0.5",
                "level_2": "0.2",
                "level_3": "0.1"
            }
        }
    }'
    
    # 实例化合约
    INSTANTIATE_RESULT=$(wasmd tx wasm instantiate "$CODE_ID" "$INSTANTIATE_MSG" \
        --from "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS" \
        --label "DD Registry CW v$CONTRACT_VERSION" \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --gas-prices "$GAS_PRICES" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --yes \
        --output json)
    
    if [ $? -ne 0 ]; then
        log_error "合约实例化失败"
        exit 1
    fi
    
    # 提取合约地址
    CONTRACT_ADDRESS=$(echo "$INSTANTIATE_RESULT" | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
    
    if [ -z "$CONTRACT_ADDRESS" ]; then
        log_error "无法获取合约地址"
        exit 1
    fi
    
    log_info "合约实例化成功，地址: $CONTRACT_ADDRESS"
    echo "$CONTRACT_ADDRESS" > /tmp/dd_registry_cw_contract_address.txt
}

# 验证部署
verify_deployment() {
    log_info "验证合约部署..."
    
    CONTRACT_ADDRESS=$(cat /tmp/dd_registry_cw_contract_address.txt)
    
    # 查询配置信息
    CONFIG_INFO=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" \
        '{"config":{}}' \
        --node "$NODE_URL" \
        --output json)
    
    if [ $? -ne 0 ]; then
        log_error "合约验证失败"
        exit 1
    fi
    
    MAX_DEPTH=$(echo "$CONFIG_INFO" | jq -r '.data.max_referral_depth')
    COOLDOWN=$(echo "$CONFIG_INFO" | jq -r '.data.referral_cooldown')
    
    log_info "配置信息验证成功:"
    log_info "  最大推荐深度: $MAX_DEPTH"
    log_info "  推荐冷却时间: $COOLDOWN 秒"
    
    # 查询积分规则
    POINTS_RULES=$(wasmd query wasm contract-state smart "$CONTRACT_ADDRESS" \
        '{"points_rules":{}}' \
        --node "$NODE_URL" \
        --output json)
    
    BASE_POINTS=$(echo "$POINTS_RULES" | jq -r '.data.base_points_per_referral')
    log_info "  基础推荐积分: $BASE_POINTS"
}

# 生成部署报告
generate_report() {
    log_info "生成部署报告..."
    
    CODE_ID=$(cat /tmp/dd_registry_cw_code_id.txt)
    CONTRACT_ADDRESS=$(cat /tmp/dd_registry_cw_contract_address.txt)
    
    REPORT_FILE="/tmp/dd_registry_cw_deployment_report.md"
    
    cat > "$REPORT_FILE" << EOF
# DD Registry CW 合约部署报告

## 部署信息
- **合约名称**: $CONTRACT_NAME
- **版本**: $CONTRACT_VERSION
- **网络**: $NETWORK
- **链ID**: $CHAIN_ID
- **部署时间**: $(date)

## 合约详情
- **代码ID**: $CODE_ID
- **合约地址**: $CONTRACT_ADDRESS
- **管理员**: $ADMIN_ADDRESS

## 部署状态
✅ 构建成功
✅ 上传成功
✅ 实例化成功
✅ 验证成功

## 后续步骤
1. 更新前端配置中的合约地址
2. 配置监控和告警
3. 进行功能测试
4. 更新文档

## 相关命令
\`\`\`bash
# 查询配置
wasmd query wasm contract-state smart $CONTRACT_ADDRESS '{"config":{}}' --node $NODE_URL

# 查询积分规则
wasmd query wasm contract-state smart $CONTRACT_ADDRESS '{"points_rules":{}}' --node $NODE_URL

# 注册用户
wasmd tx wasm execute $CONTRACT_ADDRESS '{"register":{"referrer":"'$ADMIN_ADDRESS'"}}' --from $ADMIN_ADDRESS --node $NODE_URL

# 查询用户信息
wasmd query wasm contract-state smart $CONTRACT_ADDRESS '{"user_info":{"user":"'$ADMIN_ADDRESS'"}}' --node $NODE_URL
\`\`\`
EOF
    
    log_info "部署报告已生成: $REPORT_FILE"
}

# 主函数
main() {
    log_info "开始部署 DD Registry CW 合约..."
    
    check_dependencies
    build_contract
    upload_contract
    instantiate_contract
    verify_deployment
    generate_report
    
    log_info "DD Registry CW 合约部署完成！"
    
    # 清理临时文件
    rm -f /tmp/dd_registry_cw_code_id.txt /tmp/dd_registry_cw_contract_address.txt
}

# 执行主函数
main "$@"
