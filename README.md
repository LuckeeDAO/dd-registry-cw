# DD Registry CW - 推荐积分合约

基于 CosmWasm 的推荐积分系统智能合约，支持多级推荐奖励、用户等级管理和积分提取功能。

## 🚀 功能特性

### 核心功能
- **用户注册系统**：支持用户注册和推荐关系建立
- **多级推荐奖励**：支持最多3级推荐关系，每级都有不同的奖励比例
- **用户等级系统**：基于推荐数量的等级系统（Bronze/Silver/Gold/Platinum）
- **积分管理**：支持积分分配、提取和衰减机制
- **安全防护**：防重入攻击、访问控制、紧急暂停功能

### 技术特性
- **CosmWasm 2.2.2**：使用最新的 CosmWasm 框架
- **cw-storage-plus 2.x**：高效的存储管理
- **完整测试覆盖**：单元测试和集成测试
- **生产就绪**：包含部署脚本和 CI/CD 配置

## 📁 项目结构

```
dd-registry-cw/
├── src/                    # 源代码
│   ├── contract.rs         # 合约入口点
│   ├── execute.rs          # 执行消息处理
│   ├── query.rs            # 查询消息处理
│   ├── msg.rs              # 消息定义
│   ├── state.rs            # 状态管理
│   ├── error.rs            # 错误定义
│   ├── user.rs             # 用户管理
│   ├── points.rs           # 积分管理
│   ├── referral.rs         # 推荐关系管理
│   ├── security.rs         # 安全功能
│   └── lib.rs              # 库入口
├── tests/                  # 测试文件
│   ├── integration.rs      # 集成测试
│   └── unit.rs            # 单元测试
├── scripts/                # 脚本文件
│   ├── deploy.sh          # 部署脚本
│   └── upload_to_github.sh # GitHub 上传脚本
├── .github/                # GitHub 配置
│   └── workflows/          # CI/CD 工作流
├── docs/                   # 项目文档
├── Cargo.toml             # 项目配置
└── README.md              # 项目说明
```

## 🛠️ 快速开始

### 1. 环境要求

- Rust 1.70+
- CosmWasm CLI (wasmd)
- cosmwasm-opt (可选，用于优化)

### 2. 构建合约

```bash
# 克隆项目
git clone <repository-url>
cd dd-registry-cw

# 构建合约
cargo build --release --target wasm32-unknown-unknown

# 优化 WASM (可选)
cosmwasm-opt target/wasm32-unknown-unknown/release/dd_registry_cw.wasm \
  -o target/wasm32-unknown-unknown/release/dd_registry_cw_optimized.wasm
```

### 3. 运行测试

```bash
# 运行所有测试
cargo test

# 运行单元测试
cargo test --lib

# 运行集成测试
cargo test --test integration
```

### 4. 部署合约

```bash
# 使用部署脚本
./scripts/deploy.sh

# 或手动部署
wasmd tx wasm store target/wasm32-unknown-unknown/release/dd_registry_cw.wasm \
  --from <your-key> \
  --gas auto \
  --gas-adjustment 1.3 \
  --chain-id <chain-id> \
  --node <rpc-url> \
  --yes
```

## 📋 合约接口

### 实例化消息

```json
{
  "admin": "cosmwasm1...",
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
}
```

### 执行消息

```json
{
  "register": {
    "referrer": "cosmwasm1..."
  }
}
```

```json
{
  "allocate_points": {
    "user": "cosmwasm1...",
    "points": "1000",
    "reason": "activity_reward"
  }
}
```

```json
{
  "withdraw_points": {
    "amount": "500"
  }
}
```

### 查询消息

```json
{
  "user_info": {
    "user": "cosmwasm1..."
  }
}
```

```json
{
  "points_leaderboard": {
    "limit": 10,
    "start_after": null
  }
}
```

## 🔒 安全特性

- **防重入保护**：防止重入攻击
- **访问控制**：只有授权用户可以执行特定操作
- **紧急暂停**：管理员可以暂停合约功能
- **输入验证**：所有输入都经过严格验证
- **溢出保护**：使用 SafeMath 防止整数溢出

## 📊 用户等级系统

| 等级 | 推荐数量 | 积分倍数 | 特权 |
|------|----------|----------|------|
| Bronze | 0-4 | 1.0x | 基础功能 |
| Silver | 5-9 | 1.2x | 提高积分倍数 |
| Gold | 10-19 | 1.5x | 更高积分倍数 |
| Platinum | 20+ | 2.0x | 最高积分倍数 |

## 🧪 测试

项目包含完整的测试覆盖：

- **单元测试**：测试各个模块的功能
- **集成测试**：测试完整的业务流程
- **安全测试**：测试安全机制
- **边界测试**：测试边界条件

运行测试：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_user_registration
cargo test test_referral_relation
cargo test test_points_allocation
```

## 🚀 部署

### 自动部署

使用提供的部署脚本：

```bash
./scripts/deploy.sh
```

### 手动部署

1. **上传合约代码**：
```bash
wasmd tx wasm store target/wasm32-unknown-unknown/release/dd_registry_cw.wasm \
  --from <your-key> \
  --gas auto \
  --gas-adjustment 1.3 \
  --chain-id <chain-id> \
  --node <rpc-url> \
  --yes
```

2. **实例化合约**：
```bash
wasmd tx wasm instantiate <code-id> '{"admin":"<admin-address>",...}' \
  --from <your-key> \
  --admin <admin-address> \
  --label "DD Registry CW" \
  --chain-id <chain-id> \
  --node <rpc-url> \
  --yes
```

## 📈 CI/CD

项目配置了完整的 CI/CD 流程：

- **代码格式检查**：确保代码风格一致
- **代码质量检查**：使用 clippy 进行静态分析
- **构建和测试**：自动构建和运行测试
- **WASM 优化**：自动优化 WASM 文件
- **安全扫描**：进行安全审计
- **文档生成**：自动生成文档

## 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 📞 支持

如果您遇到问题或有任何问题，请：

1. 查看 [文档](docs/)
2. 搜索 [Issues](https://github.com/your-org/dd-registry-cw/issues)
3. 创建新的 Issue

## 🎯 路线图

- [ ] 添加更多积分奖励类型
- [ ] 实现积分交易功能
- [ ] 添加用户行为分析
- [ ] 支持多链部署
- [ ] 添加前端界面

---

**DD Registry CW** - 构建去中心化的推荐积分生态系统 🚀