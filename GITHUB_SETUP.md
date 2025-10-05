# GitHub 仓库设置指南

## 🚀 快速上传到 GitHub

### 1. 创建 GitHub 仓库

1. 访问 [GitHub](https://github.com)
2. 点击 "New repository"
3. 填写仓库信息：
   - **Repository name**: `dd-registry-cw`
   - **Description**: `推荐积分系统智能合约 - CosmWasm 2.2.2`
   - **Visibility**: Public (或 Private)
   - **Initialize**: 不要勾选任何选项（我们已经有了代码）

### 2. 添加远程仓库

```bash
# 在项目根目录执行
cd /home/lc/luckee_dao/dd-registry-cw

# 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/dd-registry-cw.git

# 验证远程仓库
git remote -v
```

### 3. 使用自动上传脚本

```bash
# 运行上传脚本
./scripts/upload_to_github.sh
```

### 4. 手动上传（可选）

如果自动脚本失败，可以手动执行：

```bash
# 推送代码到 GitHub
git push -u origin main

# 如果遇到冲突，先拉取远程更改
git pull origin main --rebase
git push -u origin main
```

## 📋 仓库结构

上传后的仓库将包含：

```
dd-registry-cw/
├── .github/
│   └── workflows/
│       └── ci.yml              # CI/CD 工作流
├── docs/                       # 项目文档
├── scripts/                    # 脚本文件
│   ├── deploy.sh              # 部署脚本
│   └── upload_to_github.sh    # GitHub 上传脚本
├── src/                        # 源代码
├── tests/                      # 测试文件
├── .gitignore                  # Git 忽略文件
├── Cargo.toml                  # 项目配置
├── LICENSE                     # MIT 许可证
├── README.md                   # 项目说明
└── project.config             # 项目配置
```

## 🔧 CI/CD 功能

GitHub Actions 将自动执行：

- ✅ **代码格式检查** - 确保代码风格一致
- ✅ **代码质量检查** - 使用 clippy 进行静态分析
- ✅ **构建和测试** - 自动构建和运行测试
- ✅ **WASM 优化** - 自动优化 WASM 文件
- ✅ **安全扫描** - 进行安全审计
- ✅ **文档生成** - 自动生成文档

## 📊 项目统计

- **总文件数**: 33+ 文件
- **代码行数**: 7000+ 行
- **测试文件**: 2 个测试文件
- **文档文件**: 7+ 个文档文件
- **脚本文件**: 4+ 个脚本文件

## 🎯 后续步骤

1. **配置 GitHub Pages**（可选）
   - 在仓库设置中启用 GitHub Pages
   - 选择 `gh-pages` 分支作为源

2. **设置分支保护**（推荐）
   - 在仓库设置中配置分支保护规则
   - 要求 PR 审查
   - 要求状态检查通过

3. **配置 Secrets**（用于部署）
   - 添加部署密钥
   - 添加 API 令牌

4. **创建 Release**
   - 使用 GitHub 的 Release 功能
   - 上传优化后的 WASM 文件

## 🔗 有用的链接

- **GitHub 仓库**: `https://github.com/YOUR_USERNAME/dd-registry-cw`
- **GitHub Actions**: `https://github.com/YOUR_USERNAME/dd-registry-cw/actions`
- **Issues**: `https://github.com/YOUR_USERNAME/dd-registry-cw/issues`
- **Releases**: `https://github.com/YOUR_USERNAME/dd-registry-cw/releases`

## 📞 支持

如果您在设置过程中遇到问题：

1. 检查 Git 配置：`git config --list`
2. 检查远程仓库：`git remote -v`
3. 查看 Git 状态：`git status`
4. 查看提交历史：`git log --oneline`

---

**恭喜！** 🎉 您的 DD Registry CW 项目现在已经准备好上传到 GitHub 了！
