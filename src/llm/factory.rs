use super::providers::OpenAiProvider;
use super::traits::LlmProvider;
use crate::config::ModelConfig;
use crate::error::AppError;

/// 根据 ModelConfig.provider 字段动态创建对应的 LlmProvider 实现。
pub struct LlmProviderFactory;

impl LlmProviderFactory {
    pub fn create(model_config: &ModelConfig) -> Result<Box<dyn LlmProvider>, AppError> {
        match model_config.provider.as_str() {
            "openai" => Ok(Box::new(OpenAiProvider::new(model_config)?)),
            // 扩展点：新增后端在此注册
            // "anthropic" => Ok(Box::new(AnthropicProvider::new(model_config)?)),
            other => Err(AppError::Config(format!("未知的 provider 类型: {}", other))),
        }
    }
}
