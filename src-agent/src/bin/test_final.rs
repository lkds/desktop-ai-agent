/// Agent 完整执行测试（使用百炼通用 API）
use qoderwork_agent::{
    AgentExecutor,
    ProviderConfig,
    ProviderKind,
    init_default_tools,
    providers::DashScopeProvider,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🧪 Agent 完整执行测试\n");
    
    let config = ProviderConfig {
        kind: ProviderKind::DashScope,
        api_key: Some("sk-bd8644024768482ba49a85cf53c5c2c4".to_string()),
        base_url: None, // 使用默认百炼 URL
        model: "qwen-plus".to_string(),
        extra: std::collections::HashMap::new(),
    };
    
    let provider = Arc::new(DashScopeProvider::new(config.clone()).unwrap());
    let tool_registry = Arc::new(init_default_tools());
    let executor = Arc::new(AgentExecutor::new(provider, tool_registry));
    executor.set_allowed_paths(vec!["/tmp".to_string()]).await;
    
    println!("Provider: DashScope");
    println!("Model: {}", config.model);
    println!("工具: 19 个\n");
    
    // 任务：创建文件
    println!("任务: 在 /tmp 目录创建 hello.txt，内容为 'Hello from AI Agent'");
    println!();
    
    let result = executor.execute(
        "在 /tmp 目录创建一个 hello.txt 文件，内容为 'Hello from AI Agent'".to_string()
    ).await;
    
    match result {
        Ok(task) => {
            println!("状态: {:?}", task.status);
            println!("\n步骤:");
            for (i, step) in task.steps.iter().enumerate() {
                println!("  {}. {:?} - {}", i+1, step.status, step.description);
            }
            if let Some(r) = &task.result {
                println!("\n✅ {}", r.summary);
            }
        }
        Err(e) => println!("❌ {}", e),
    }
}