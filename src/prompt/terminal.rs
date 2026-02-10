use std::io::{self, Write};

use super::UserInteraction;
use crate::config::ModelConfig;
use crate::error::AppError;

/// 基于终端 stdin/stdout 的用户交互实现。
#[derive(Default)]
pub struct TerminalPrompt;

impl TerminalPrompt {
    pub fn new() -> Self {
        Self
    }
}

impl UserInteraction for TerminalPrompt {
    fn confirm_commit(&self, message: &str) -> Result<bool, AppError> {
        println!("\n===== 生成的 Commit Message =====");
        println!("{}", message);
        println!("=================================\n");
        print!("是否使用此 commit message 提交？(y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let answer = input.trim().to_lowercase();
        Ok(answer == "y" || answer == "yes")
    }

    fn warn(&self, message: &str) {
        eprintln!("[警告] {}", message);
    }

    fn display_model_list(&self, models: &[ModelConfig]) {
        let header = format!(
            "{:<15} {:<10} {:<20} {}",
            "名称", "Provider", "模型 ID", "API 地址"
        );
        println!("\n{header}");
        println!("{}", "-".repeat(70));
        for m in models {
            let line = format!(
                "{:<15} {:<10} {:<20} {}",
                m.name, m.provider, m.model_id, m.api_base
            );
            println!("{line}");
        }
        println!();
    }
}
