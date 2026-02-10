use std::path::{Path, PathBuf};

use git2::{DiffOptions, Repository};

use super::{DiffResult, GitOperations};
use crate::error::AppError;

/// 基于 git2 的 GitOperations 实现。
#[derive(Default)]
pub struct GitRepository;

impl GitRepository {
    pub fn new() -> Self {
        Self
    }
}

impl GitOperations for GitRepository {
    fn discover_repo(&self, path: &Path) -> Result<PathBuf, AppError> {
        let repo = Repository::discover(path)
            .map_err(|e| AppError::Git(format!("未找到 Git 仓库: {}", e)))?;
        let workdir = repo
            .workdir()
            .ok_or_else(|| AppError::Git("无法获取仓库工作目录（可能是 bare 仓库）".into()))?;
        Ok(workdir.to_path_buf())
    }

    fn staged_diff(&self, repo_path: &Path) -> Result<DiffResult, AppError> {
        let repo = Repository::open(repo_path)
            .map_err(|e| AppError::Git(format!("无法打开仓库: {}", e)))?;

        // 获取 HEAD tree（如果是空仓库则为 None）
        let head_tree = repo.head().ok().and_then(|h| h.peel_to_tree().ok());

        let mut opts = DiffOptions::new();
        let diff = repo
            .diff_tree_to_index(head_tree.as_ref(), None, Some(&mut opts))
            .map_err(|e| AppError::Git(format!("无法获取 staged diff: {}", e)))?;

        let stats = diff
            .stats()
            .map_err(|e| AppError::Git(format!("无法获取 diff 统计: {}", e)))?;
        let files_changed = stats.files_changed();

        // 收集 diff 文本
        let mut raw = String::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            let origin = line.origin();
            if origin == '+' || origin == '-' || origin == ' ' {
                raw.push(origin);
            }
            if let Ok(content) = std::str::from_utf8(line.content()) {
                raw.push_str(content);
            }
            true
        })
        .map_err(|e| AppError::Git(format!("无法输出 diff: {}", e)))?;

        Ok(DiffResult::new(raw, files_changed))
    }

    fn commit(&self, repo_path: &Path, message: &str) -> Result<(), AppError> {
        let repo = Repository::open(repo_path)
            .map_err(|e| AppError::Git(format!("无法打开仓库: {}", e)))?;

        let sig = repo
            .signature()
            .map_err(|e| AppError::Git(format!("无法获取签名信息: {}", e)))?;

        let tree_id = repo
            .index()
            .map_err(|e| AppError::Git(format!("无法获取 index: {}", e)))?
            .write_tree()
            .map_err(|e| AppError::Git(format!("无法写入 tree: {}", e)))?;

        let tree = repo
            .find_tree(tree_id)
            .map_err(|e| AppError::Git(format!("无法查找 tree: {}", e)))?;

        // 获取 parent commit（如果存在）
        let parent = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let parents: Vec<&git2::Commit> = parent.iter().collect();

        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
            .map_err(|e| AppError::Git(format!("提交失败: {}", e)))?;

        Ok(())
    }
}
