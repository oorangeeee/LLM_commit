use serde::{Deserialize, Serialize};

/// Prompt 配置，控制发送给 LLM 的 system prompt 和 user prompt 模板。
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PromptConfig {
    /// system prompt，指导 LLM 生成 Conventional Commits 格式的 commit message
    pub system: String,
    /// user prompt 模板，{diff} 占位符会被替换为实际 diff 内容
    pub user: String,
}
