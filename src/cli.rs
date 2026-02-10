use clap::Parser;

/// LLM-powered Git Commit assistant
#[derive(Parser)]
#[command(name = "llmc", about = "LLM-powered Git Commit assistant")]
pub struct CliArgs {
    /// 切换使用的模型
    #[arg(long)]
    pub model: Option<String>,

    /// 列出所有可用模型
    #[arg(long = "model_list")]
    pub model_list: bool,

    /// 设置 diff 最大 token 数
    #[arg(long)]
    pub limit: Option<usize>,
}
