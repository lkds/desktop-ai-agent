/// Agent CLI - 命令行测试工具
/// 用于测试 Agent 核心功能

use qoderwork_agent::{
    AppConfig,
    AgentExecutor,
    ProviderConfig,
    ProviderKind,
    ToolRegistry,
    SkillsManager,
};
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🤖 Desktop AI Agent CLI");
    println!("========================\n");
    
    // 加载配置
    let config_path = AppConfig::default_path();
    let config = AppConfig::load(&config_path).unwrap_or_else(|_| {
        println!("使用默认配置");
        AppConfig::default()
    });
    
    println!("当前配置:");
    println!("  Provider: {:?}", config.provider.kind);
    println!("  Model: {}", config.provider.model);
    println!("  Allowed paths: {:?}", config.allowed_paths);
    println!();
    
    // 初始化工具注册表
    let mut tool_registry = ToolRegistry::new();
    // 注册默认工具（这里简化处理）
    println!("工具注册完成");
    
    // 初始化 Skills Manager
    let skills_dir = PathBuf::from(&config.skills_dir);
    let mut skills_manager = SkillsManager::new(skills_dir);
    skills_manager.load_all().await.unwrap_or_else(|e| {
        println!("Skills 加载失败: {}", e);
    });
    
    println!("已加载 Skills: {:?}", skills_manager.list_skills().iter().map(|s| s.name.clone()).collect::<Vec<_>>());
    println!();
    
    // 初始化 Provider
    let provider: Arc<dyn qoderwork_agent::Provider> = match config.provider.kind {
        ProviderKind::OpenAI => {
            Arc::new(qoderwork_agent::providers::OpenAIProvider::new(config.provider.clone())
                .expect("Failed to create OpenAI provider"))
        }
        ProviderKind::Claude => {
            Arc::new(qoderwork_agent::providers::ClaudeProvider::new(config.provider.clone())
                .expect("Failed to create Claude provider"))
        }
        ProviderKind::Ollama => {
            Arc::new(qoderwork_agent::providers::OllamaProvider::new(config.provider.clone())
                .expect("Failed to create Ollama provider"))
        }
        ProviderKind::Custom => {
            // Custom 使用 OpenAI-compatible 格式
            Arc::new(qoderwork_agent::providers::OpenAIProvider::new(config.provider.clone())
                .expect("Failed to create Custom provider"))
        }
    };
    
    // 初始化 Agent Executor
    let executor = AgentExecutor::new(provider, Arc::new(tool_registry));
    executor.set_allowed_paths(config.allowed_paths.clone()).await;
    
    println!("Agent 初始化完成");
    println!("输入任务描述开始执行（输入 'quit' 退出）");
    println!();
    
    // 交互循环
    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "quit" || input == "exit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        println!("执行任务: {}", input);
        println!();
        
        match executor.execute(input.to_string()).await {
            Ok(task) => {
                println!("任务状态: {:?}", task.status);
                for step in &task.steps {
                    println!("  步骤 {}: {:?} - {}", 
                        step.id, step.status, step.description);
                }
                if let Some(result) = &task.result {
                    println!();
                    println!("结果: {}", result.summary);
                }
            }
            Err(e) => {
                println!("执行失败: {}", e);
            }
        }
        println!();
    }
    
    println!("再见！");
}