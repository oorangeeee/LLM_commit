/// 暂存区 diff 的结构化表示。
pub struct DiffResult {
    /// 原始 diff 文本
    pub raw: String,
    /// 变更文件数
    pub files_changed: usize,
    /// 预估 token 数
    pub estimated_tokens: usize,
}

impl DiffResult {
    /// 创建 DiffResult，自动估算 token 数（按字符数 / 4）
    pub fn new(raw: String, files_changed: usize) -> Self {
        let estimated_tokens = raw.len() / 4;
        Self {
            raw,
            files_changed,
            estimated_tokens,
        }
    }
}
