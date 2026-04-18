/// Agent 完整执行测试
use qoderwork_agent::{
    AgentExecutor,
    ProviderConfig,
    ProviderKind,
    init_default_tools,
    providers::OpenAIProvider,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🧪 Agent 完整执行测试\n");
    
    // 配置 Provider（使用 Coding Plan API）
    let config = ProviderConfig {
        kind: ProviderKind::Custom,
        api_key: Some("sk-sp-196f167ed8d947f89581f1dcf4d6fb68".to_string()),
        base_url: Some("https://coding.dashscope.aliyuncs.com/v1".to_string()),
        model: "qwen3.5-plus".to_string(),
        extra: std::collections::HashMap::new(),
    };
    
    let provider = Arc::new(OpenAIProvider::new(config.clone())
        .expect("Provider 创建失败"));
    
    // 初始化工具和 Agent
    let tool_registry = Arc::new(init_default_tools());
    let executor = Arc::new(AgentExecutor::new(provider, tool_registry));
    executor.set_allowed_paths(vec!["/tmp".to_string()]).await;
    
    println!("Provider: {:?}", config.kind);
    println!("Base URL: {}", config.base_url.as_ref().unwrap());
    println!("Model: {}", config.model);
    println!("工具数量: 19 个\n");
    
    // 执行任务
    println!("执行任务: 在 /tmp/test-rust 目录创建一个 hello.txt 文件，内容为 'Hello from AI Agent'");
    println!();
    
    let result = executor.execute(
        "在 /tmp/test-rust 目录创建一个 hello.txt 文件，内容为 'Hello from AI Agent'".to_string()
    ).await;
    
    match result {
        Ok(task) => {
            println!("任务状态: {:?}", task.status);
            println!("\n执行步骤:");
            for (i, step) in task.steps.iter().enumerate() {
                println!("  [{}/{}] {:?} - {}", i+1, task.steps.len(), step.status, step.description);
            }
            
            if let Some(result) = &task.result {
                println!("\n✅ 结果:");
                println!("{}", result.summary);
            }
        }
        Err(e) => {
            println!("❌ 执行失败: {}", e);
        }
    }
}