/// Agent CLI - 命令行测试工具
use qoderwork_agent::{
    AppConfig,
    AgentExecutor,
    ProviderConfig,
    ProviderKind,
    ToolRegistry,
    SkillsManager,
    providers::{OpenAIProvider, ClaudeProvider, OllamaProvider, DashScopeProvider},
};
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🤖 Desktop AI Agent CLI v0.1.0");
    println!("================================\n");
    
    // 加载配置
    let config_path = AppConfig::default_path();
    let config = AppConfig::load(&config_path).unwrap_or_else(|_| {
        println!("⚠️  未找到配置文件，使用默认配置");
        println!("请编辑 ~/.desktop-agent/config.json 配置模型\n");
        AppConfig::default()
    });
    
    println!("当前配置:");
    println!("  Provider: {:?}", config.provider.kind);
    println!("  Model: {}", config.provider.model);
    println!("  Allowed paths: {:?}", config.allowed_paths);
    println!();
    
    // 初始化工具注册表
    let tool_registry = Arc::new(ToolRegistry::new());
    
    // 初始化 Skills Manager
    let skills_dir = PathBuf::from(&config.skills_dir);
    let mut skills_manager = SkillsManager::new(skills_dir);
    skills_manager.load_all().await.unwrap_or_else(|e| {
        println!("⚠️  Skills 加载失败: {}", e);
    });
    
    let skills = skills_manager.list_skills();
    if skills.is_empty() {
        println!("暂无 Skills");
    } else {
        println!("已加载 Skills:");
        for skill in skills {
            println!("  - {}", skill.name);
        }
    }
    println!();
    
    // 初始化 Provider
    let provider: Arc<dyn qoderwork_agent::Provider> = match config.provider.kind {
        ProviderKind::OpenAI => {
            Arc::new(OpenAIProvider::new(config.provider.clone())
                .expect("❌ OpenAI Provider 创建失败"))
        }
        ProviderKind::Claude => {
            Arc::new(ClaudeProvider::new(config.provider.clone())
                .expect("❌ Claude Provider 创建失败"))
        }
        ProviderKind::Ollama => {
            Arc::new(OllamaProvider::new(config.provider.clone())
                .expect("❌ Ollama Provider 创建失败"))
        }
        ProviderKind::DashScope => {
            Arc::new(DashScopeProvider::new(config.provider.clone())
                .expect("DashScope Provider 创建失败"))
        }
        ProviderKind::Custom => {
            Arc::new(OpenAIProvider::new(config.provider.clone())
                .expect("❌ Custom Provider 创建失败"))
        }
    };
    
    // 初始化 Agent Executor
    let executor = Arc::new(AgentExecutor::new(provider, tool_registry));
    executor.set_allowed_paths(config.allowed_paths.clone()).await;
    
    println!("✅ Agent 初始化完成");
    println!();
    println!("输入任务描述开始执行（输入 'quit' 退出）");
    println!("特殊命令: 'status' - 查看状态, 'help' - 帮助");
    println!();
    
    // 交互循环
    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "quit" || input == "exit" {
            println!("再见！");
            break;
        }
        
        if input == "help" {
            println!("命令:");
            println!("  <任务描述> - 执行任务");
            println!("  status    - 查看当前状态");
            println!("  config    - 查看配置");
            println!("  quit      - 退出");
            continue;
        }
        
        if input == "status" {
            if let Some(task) = executor.get_status().await {
                println!("当前任务: {}", task.description);
                println!("状态: {:?}", task.status);
                println!("步骤: {}", task.steps.len());
            } else {
                println!("无当前任务");
            }
            continue;
        }
        
        if input == "config" {
            println!("Provider: {:?}", config.provider.kind);
            println!("Model: {}", config.provider.model);
            println!("API Key: {}", if config.provider.api_key.is_some() { "已设置" } else { "未设置" });
            println!("Allowed paths: {:?}", config.allowed_paths);
            continue;
        }
        
        if input.is_empty() {
            continue;
        }
        
        println!("\n执行任务: {}", input);
        println!();
        
        match executor.execute(input.to_string()).await {
            Ok(task) => {
                println!("状态: {:?}", task.status);
                for (i, step) in task.steps.iter().enumerate() {
                    println!("  [{}/{}] {:?} - {}", 
                        i + 1, task.steps.len(), step.status, step.description);
                    if let Some(result) = &step.result {
                        println!("      输出: {}", 
                            if result.output.len() > 100 { 
                                &result.output[..100] 
                            } else { 
                                &result.output 
                            });
                    }
                }
                if let Some(result) = &task.result {
                    println!();
                    println!("✅ 结果:");
                    println!("{}", result.summary);
                }
            }
            Err(e) => {
                println!("❌ 执行失败: {}", e);
            }
        }
        println!();
    }
}