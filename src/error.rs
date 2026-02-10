/// 统一错误类型，使用 thiserror 派生。
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Git error: {0}")]
    Git(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Token limit exceeded: {current} tokens > {limit} tokens")]
    TokenLimitExceeded { current: usize, limit: usize },

    #[error("Model not found: {0}")]
    ModelNotFound(String),
}
