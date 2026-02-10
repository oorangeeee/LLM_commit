use serde::Deserialize;

/// 单个模型的配置。
#[derive(Debug, Deserialize, Clone)]
pub struct ModelConfig {
    /// 模型标识名
    pub name: String,
    /// 后端类型，如 "openai"
    pub provider: String,
    /// API 地址
    pub api_base: String,
    /// 存放 API Key 的环境变量名
    pub api_key_env: String,
    /// 实际模型 ID
    pub model_id: String,
    /// 可选：最大生成 token 数
    pub max_tokens: Option<usize>,
}
