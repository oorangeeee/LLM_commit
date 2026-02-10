# llmc

`llmc` 是一个纯 Rust 实现的 Git Commit 助手。它自动分析暂存区变更，通过 LLM 生成严格符合 [Conventional Commits](https://www.conventionalcommits.org/) 规范的提交信息。

## 快速开始

### 1. 配置环境变量

API Key 通过环境变量提供，`config.toml` 中只记录变量名，不存储任何密钥：

```bash
export DEEPSEEK_API_KEY="your-api-key-here"
```

### 2. 使用

```bash
# 默认流程：检测仓库 → 读取 staged diff → 调用 LLM → 确认 → 提交
llmc

# 指定模型
llmc --model deepseek

# 列出所有可用模型
llmc --model_list

# 设置 diff 最大 token 数（默认 1000）
llmc --limit 2000
```

## 配置

所有配置位于项目根目录的 `config.toml`，该文件**不包含任何敏感信息**，可以安全提交到版本控制。

```toml
default_model = "deepseek"
token_limit = 1000

[prompt]
system = """..."""   # 控制 LLM 输出格式的 system prompt
user = "...{diff}"   # user prompt 模板，{diff} 会被替换为实际 diff

[[models]]
name = "deepseek"
provider = "openai"
api_base = "https://api.deepseek.com"
api_key_env = "DEEPSEEK_API_KEY"   # 环境变量名，非密钥本身
model_id = "deepseek-chat"
max_tokens = 1024
```

### 安全设计

- `api_key_env` 字段存储的是**环境变量名称**（如 `"DEEPSEEK_API_KEY"`），程序运行时从环境变量读取实际密钥
- `config.toml` 中不存储任何 API Key，可安全纳入版本管理
- 新增模型只需在 `[[models]]` 中追加配置，并设置对应的环境变量

### Prompt 配置

`[prompt]` 段控制发送给 LLM 的提示词，可自由调整生成风格：

- `system`：system prompt，引导 LLM 生成 Conventional Commits 格式
- `user`：user prompt 模板，`{diff}` 占位符在运行时被替换为实际 diff 内容

## Conventional Commits

生成的 commit message 严格遵循 Conventional Commits 规范：

```
<type>(<scope>): <description>

[optional body]
```

支持的 type：`feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

## 技术栈

纯 Rust 实现，主要依赖：

| 功能 | Crate |
|---|---|
| CLI 解析 | clap |
| 配置解析 | toml + serde |
| Git 操作 | git2 |
| HTTP 请求 | reqwest |
| 异步运行时 | tokio |
| 错误处理 | thiserror |
