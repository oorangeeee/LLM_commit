use async_trait::async_trait;

use super::request::LlmRequest;
use super::response::LlmResponse;
use crate::error::AppError;

/// LLM 后端的统一抽象。
/// 新增模型后端只需实现此 trait（策略模式）。
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// 返回该 Provider 的名称标识
    fn name(&self) -> &str;

    /// 发送请求并获取生成的 commit message
    async fn generate(&self, request: &LlmRequest) -> Result<LlmResponse, AppError>;
}
