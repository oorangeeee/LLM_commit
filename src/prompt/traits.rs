use crate::config::ModelConfig;
use crate::error::AppError;

/// 用户交互的抽象接口。
/// 将 stdin/stdout 交互解耦，便于测试和未来替换为 TUI。
pub trait UserInteraction {
    /// 展示 commit message 并请求用户确认
    fn confirm_commit(&self, message: &str) -> Result<bool, AppError>;

    /// 展示警告信息
    fn warn(&self, message: &str);

    /// 展示模型列表
    fn display_model_list(&self, models: &[ModelConfig]);
}
