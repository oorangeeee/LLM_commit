use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::ModelConfig;
use super::PromptConfig;
use crate::error::AppError;

/// 内嵌的默认配置内容
const DEFAULT_CONFIG: &str = include_str!("../../config.toml");

/// 全局配置，从 config.toml 反序列化。
#[derive(Debug, Deserialize, Serialize)]
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

    /// 解析配置文件路径，按优先级查找：
    /// 1. 用户通过 --config 指定的路径
    /// 2. ~/.config/llmc/config.toml（XDG 规范）
    /// 3. 当前目录的 config.toml（开发时方便）
    ///
    /// 如果 ~/.config/llmc/ 目录不存在，自动创建并写入默认配置。
    pub fn resolve_config_path(user_path: Option<&Path>) -> Result<PathBuf, AppError> {
        // 1. 用户显式指定
        if let Some(p) = user_path {
            if p.exists() {
                return Ok(p.to_path_buf());
            }
            return Err(AppError::Config(format!(
                "指定的配置文件不存在: {}",
                p.display()
            )));
        }

        // 2. XDG: ~/.config/llmc/config.toml
        if let Some(config_dir) = dirs::config_dir() {
            let llmc_dir = config_dir.join("llmc");
            let xdg_path = llmc_dir.join("config.toml");
            if xdg_path.exists() {
                return Ok(xdg_path);
            }

            // 3. 当前目录 config.toml（开发用）
            let cwd_path = PathBuf::from("config.toml");
            if cwd_path.exists() {
                return Ok(cwd_path);
            }

            // 都不存在，在 XDG 目录自动创建默认配置
            std::fs::create_dir_all(&llmc_dir).map_err(|e| {
                AppError::Config(format!(
                    "无法创建配置目录 {}: {}",
                    llmc_dir.display(),
                    e
                ))
            })?;
            std::fs::write(&xdg_path, DEFAULT_CONFIG).map_err(|e| {
                AppError::Config(format!(
                    "无法写入默认配置 {}: {}",
                    xdg_path.display(),
                    e
                ))
            })?;
            eprintln!("已创建默认配置文件: {}", xdg_path.display());
            eprintln!("请根据需要修改配置后重新运行。");
            return Ok(xdg_path);
        }

        // 无 XDG 目录的回退
        let cwd_path = PathBuf::from("config.toml");
        if cwd_path.exists() {
            return Ok(cwd_path);
        }

        Err(AppError::Config(
            "未找到配置文件。请使用 --config 指定路径，或创建 ~/.config/llmc/config.toml".into(),
        ))
    }

    /// 根据名称查找模型配置
    pub fn find_model(&self, name: &str) -> Result<&ModelConfig, AppError> {
        self.models
            .iter()
            .find(|m| m.name == name)
            .ok_or_else(|| AppError::ModelNotFound(name.to_string()))
    }
}
