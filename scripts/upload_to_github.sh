#!/bin/bash

# DD Registry CW 项目自动上传到 GitHub 脚本
# 使用方法: ./scripts/upload_to_github.sh

set -e  # 遇到错误时退出

echo "🚀 开始上传 DD Registry CW 项目到 GitHub..."

# 检查是否在正确的目录
if [ ! -f "Cargo.toml" ]; then
    echo "❌ 错误: 请在项目根目录运行此脚本"
    exit 1
fi

# 检查 Git 状态
echo "📋 检查 Git 状态..."
git status

# 添加所有修改的文件
echo "📝 添加所有修改的文件..."
git add .

# 提交更改
echo "💾 提交更改..."
git commit -m "feat: 完成 DD Registry CW 推荐积分合约开发

- 实现完整的推荐积分系统
- 支持多级推荐奖励机制
- 实现用户等级系统 (Bronze/Silver/Gold/Platinum)
- 添加积分衰减和提取功能
- 实现安全机制 (防重入、访问控制、紧急暂停)
- 升级 cosmwasm-std 到 2.2.2 版本
- 升级 cw-storage-plus 到 2.x 版本
- 添加完整的测试覆盖
- 实现部署脚本和 CI/CD 配置
- 添加详细的文档和示例

核心功能:
- 用户注册和推荐关系管理
- 多级推荐积分分配
- 用户等级和积分倍数
- 积分提取和衰减机制
- 安全防护和访问控制
- 完整的查询接口

技术特性:
- CosmWasm 2.2.2 兼容
- 完整的错误处理
- 详细的测试覆盖
- 生产就绪的部署脚本"

# 确认远程仓库设置
echo "🔗 确认远程仓库设置..."
git remote -v

# 推送代码到 GitHub
echo "⬆️  推送代码到 GitHub..."
git push -u origin main

echo "✅ 项目已成功上传到 GitHub!"

# 显示项目信息
echo ""
echo "📊 项目统计:"
echo "   - 总文件数: $(find . -type f | wc -l)"
echo "   - 代码行数: $(find . -name "*.rs" -exec wc -l {} + | tail -1 | awk '{print $1}')"
echo "   - 测试文件: $(find . -name "*test*.rs" | wc -l)"
echo "   - 文档文件: $(find . -name "*.md" | wc -l)"
echo "   - 脚本文件: $(find . -name "*.sh" | wc -l)"

echo ""
echo "🎉 上传完成! 您现在可以访问 GitHub 仓库查看您的项目"
echo "📋 本次提交包含:"
echo "   - 完整的推荐积分合约实现"
echo "   - CosmWasm 2.2.2 兼容性"
echo "   - 完整的测试覆盖"
echo "   - 生产就绪的部署脚本"
echo "   - CI/CD 自动化配置"
echo "   - 详细的文档和示例"
