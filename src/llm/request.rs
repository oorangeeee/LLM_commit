use crate::error::AppError;

/// 发送给 LLM 的请求。
pub struct LlmRequest {
    pub system_prompt: String,
    pub user_prompt: String,
    pub diff_content: String,
    pub max_tokens: Option<usize>,
}

impl LlmRequest {
    pub fn builder() -> LlmRequestBuilder {
        LlmRequestBuilder::default()
    }
}

/// LlmRequest 建造者。
#[derive(Default)]
pub struct LlmRequestBuilder {
    system_prompt: Option<String>,
    user_prompt: Option<String>,
    diff_content: Option<String>,
    max_tokens: Option<usize>,
}

impl LlmRequestBuilder {
    pub fn system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = Some(prompt.to_string());
        self
    }

    pub fn user_prompt(mut self, prompt: &str) -> Self {
        self.user_prompt = Some(prompt.to_string());
        self
    }

    pub fn diff_content(mut self, diff: &str) -> Self {
        self.diff_content = Some(diff.to_string());
        self
    }

    pub fn max_tokens(mut self, n: usize) -> Self {
        self.max_tokens = Some(n);
        self
    }

    pub fn build(self) -> Result<LlmRequest, AppError> {
        let system_prompt = self
            .system_prompt
            .ok_or_else(|| AppError::Llm("system_prompt is required".into()))?;
        let diff_content = self
            .diff_content
            .ok_or_else(|| AppError::Llm("diff_content is required".into()))?;
        let user_prompt = self.user_prompt.unwrap_or_else(|| {
            format!(
                "Generate a commit message for the following git diff:\n\n{}",
                diff_content
            )
        });
        Ok(LlmRequest {
            system_prompt,
            user_prompt,
            diff_content,
            max_tokens: self.max_tokens,
        })
    }
}
