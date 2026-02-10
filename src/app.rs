use std::env;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::git::GitOperations;
use crate::llm::{LlmProvider, LlmRequest};
use crate::prompt::UserInteraction;

/// 应用门面，持有所有 Service 的 trait object，编排主流程。
pub struct App {
    config: AppConfig,
    git: Box<dyn GitOperations>,
    llm: Box<dyn LlmProvider>,
    ui: Box<dyn UserInteraction>,
}

impl App {
    pub fn new(
        config: AppConfig,
        git: Box<dyn GitOperations>,
        llm: Box<dyn LlmProvider>,
        ui: Box<dyn UserInteraction>,
    ) -> Self {
        Self {
            config,
            git,
            llm,
            ui,
        }
    }

    /// 主流程入口：检测仓库 → 获取 diff → 调用 LLM → 用户确认 → 提交
    pub async fn run(&self) -> Result<(), AppError> {
        let current_dir = env::current_dir()?;

        // 1. 检测 Git 仓库
        let repo_path = self.git.discover_repo(&current_dir)?;
        println!("检测到 Git 仓库: {}", repo_path.display());

        // 2. 获取暂存区 diff
        let diff = self.git.staged_diff(&repo_path)?;
        if diff.raw.is_empty() {
            return Err(AppError::Git(
                "暂存区没有变更，请先使用 git add 添加变更".into(),
            ));
        }
        println!(
            "暂存区变更: {} 个文件, 预估 {} tokens",
            diff.files_changed, diff.estimated_tokens
        );

        // 3. 检查 token 限制
        if diff.estimated_tokens > self.config.token_limit {
            self.ui.warn(&format!(
                "diff 预估 {} tokens，超过限制 {} tokens，可能导致截断",
                diff.estimated_tokens, self.config.token_limit
            ));
        }

        // 4. 构建 LLM 请求并调用
        println!("正在调用 LLM 生成 commit message...");
        let user_prompt = self.config.prompt.user.replace("{diff}", &diff.raw);
        let request = LlmRequest::builder()
            .system_prompt(&self.config.prompt.system)
            .user_prompt(&user_prompt)
            .diff_content(&diff.raw)
            .build()?;

        let response = self.llm.generate(&request).await?;

        if let Some(tokens) = response.usage_tokens {
            println!("LLM 消耗 tokens: {}", tokens);
        }

        // 5. 用户确认
        let confirmed = self.ui.confirm_commit(&response.commit_message)?;
        if !confirmed {
            println!("已取消提交。");
            return Ok(());
        }

        // 6. 提交
        self.git.commit(&repo_path, &response.commit_message)?;
        println!("提交成功！");

        Ok(())
    }

    /// 列出所有可用模型
    pub fn list_models(&self) {
        self.ui.display_model_list(&self.config.models);
    }
}
