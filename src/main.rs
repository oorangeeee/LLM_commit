use clap::Parser;

use llmc::app::App;
use llmc::cli::CliArgs;
use llmc::config::AppConfig;
use llmc::error::AppError;
use llmc::git::GitRepository;
use llmc::llm::LlmProviderFactory;
use llmc::prompt::TerminalPrompt;

/// 查找 config.toml：优先使用可执行文件所在目录，其次当前目录
fn find_config_path() -> std::path::PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        let exe_dir_config = exe_path.parent().unwrap().join("config.toml");
        if exe_dir_config.exists() {
            return exe_dir_config;
        }
    }
    std::path::PathBuf::from("config.toml")
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), AppError> {
    let args = CliArgs::parse();

    // 加载配置
    let config_path = find_config_path();
    let mut config = AppConfig::load(&config_path)?;

    // 如果指定了 --limit，覆盖配置中的 token_limit
    if let Some(limit) = args.limit {
        config.token_limit = limit;
    }

    // 确定使用的模型
    let model_name = args.model.as_deref().unwrap_or(&config.default_model);

    // 如果是 --model_list，展示后退出
    if args.model_list {
        let ui = TerminalPrompt::new();
        let git = GitRepository::new();
        let model_config = config.find_model(model_name)?;
        let llm = LlmProviderFactory::create(model_config)?;
        let app = App::new(config, Box::new(git), llm, Box::new(ui));
        app.list_models();
        return Ok(());
    }

    // 正常流程：查找模型配置 → 创建 provider → 运行
    let model_config = config.find_model(model_name)?;
    let llm = LlmProviderFactory::create(model_config)?;
    let git = GitRepository::new();
    let ui = TerminalPrompt::new();

    let app = App::new(config, Box::new(git), llm, Box::new(ui));
    app.run().await
}
