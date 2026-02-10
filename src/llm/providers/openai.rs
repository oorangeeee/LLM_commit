use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::config::ModelConfig;
use crate::error::AppError;
use crate::llm::{LlmProvider, LlmRequest, LlmResponse};

/// OpenAI 兼容的 LLM Provider 实现。
/// 支持所有兼容 OpenAI Chat Completions API 的后端（OpenAI、DeepSeek 等）。
pub struct OpenAiProvider {
    client: Client,
    api_base: String,
    api_key: String,
    model_id: String,
    max_tokens: Option<usize>,
}

impl OpenAiProvider {
    pub fn new(config: &ModelConfig) -> Result<Self, AppError> {
        let api_key = std::env::var(&config.api_key_env).map_err(|_| {
            AppError::Config(format!(
                "环境变量 {} 未设置，请设置对应的 API Key",
                config.api_key_env
            ))
        })?;

        Ok(Self {
            client: Client::new(),
            api_base: config.api_base.clone(),
            api_key,
            model_id: config.model_id.clone(),
            max_tokens: config.max_tokens,
        })
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn generate(&self, request: &LlmRequest) -> Result<LlmResponse, AppError> {
        let url = format!("{}/chat/completions", self.api_base.trim_end_matches('/'));

        let max_tokens = request.max_tokens.or(self.max_tokens).unwrap_or(1024);

        let body = json!({
            "model": self.model_id,
            "messages": [
                {
                    "role": "system",
                    "content": request.system_prompt
                },
                {
                    "role": "user",
                    "content": request.user_prompt
                }
            ],
            "max_tokens": max_tokens,
            "stream": false
        });

        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Llm(format!("请求发送失败: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "无法读取响应体".into());
            return Err(AppError::Llm(format!(
                "API 返回错误 ({}): {}",
                status, text
            )));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Llm(format!("响应解析失败: {}", e)))?;

        let commit_message = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::Llm("无法从响应中提取 commit message".into()))?
            .trim()
            .to_string();

        let usage_tokens = json["usage"]["total_tokens"].as_u64().map(|n| n as usize);

        Ok(LlmResponse {
            commit_message,
            usage_tokens,
        })
    }
}
