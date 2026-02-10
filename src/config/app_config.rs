use serde::Deserialize;
use std::path::Path;

use super::ModelConfig;
use super::PromptConfig;
use crate::error::AppError;

/// 全局配置，从 config.toml 反序列化。
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub default_model: String,
    pub token_limit: usize,
    pub prompt: PromptConfig,
    pub models: Vec<ModelConfig>,
}

impl AppConfig {
    /// 从指定路径加载配置文件
    pub fn load(path: &Path) -> Result<Self, AppError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AppError::Config(format!("无法读取配置文件 {}: {}", path.display(), e)))?;
        let config: AppConfig = toml::from_str(&content)
            .map_err(|e| AppError::Config(format!("配置文件解析失败: {}", e)))?;
        Ok(config)
    }

    /// 根据名称查找模型配置
    pub fn find_model(&self, name: &str) -> Result<&ModelConfig, AppError> {
        self.models
            .iter()
            .find(|m| m.name == name)
            .ok_or_else(|| AppError::ModelNotFound(name.to_string()))
    }
}
