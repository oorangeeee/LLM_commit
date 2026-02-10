use std::path::{Path, PathBuf};

use super::DiffResult;
use crate::error::AppError;

/// Git 操作的抽象接口。
/// 方便单元测试时 mock，也支持未来替换底层实现。
pub trait GitOperations {
    /// 检测当前目录是否为 Git 仓库，返回仓库根路径
    fn discover_repo(&self, path: &Path) -> Result<PathBuf, AppError>;

    /// 获取暂存区的 diff 内容
    fn staged_diff(&self, repo_path: &Path) -> Result<DiffResult, AppError>;

    /// 使用指定的 commit message 提交暂存区的变更
    fn commit(&self, repo_path: &Path, message: &str) -> Result<(), AppError>;
}
