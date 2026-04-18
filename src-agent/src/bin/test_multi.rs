/// 多智能体并行测试
use qoderwork_agent::{
    AgentExecutor,
    ProviderConfig,
    ProviderKind,
    init_default_tools,
    providers::DashScopeProvider,
    SubagentConfig,
    MultiAgentCoordinator,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🧪 多智能体并行执行测试\n");
    
    let config = ProviderConfig {
        kind: ProviderKind::DashScope,
        api_key: Some("sk-bd8644024768482ba49a85cf53c5c2c4".into()),
        base_url: None,
        model: "qwen3.6-plus".into(),
        extra: std::collections::HashMap::new(),
    };
    
    let provider = Arc::new(DashScopeProvider::new(config).unwrap());
    let tools = Arc::new(init_default_tools());
    let executor = Arc::new(AgentExecutor::new(provider, tools));
    executor.set_allowed_paths(vec!["/tmp".into()]).await;
    
    let coordinator = MultiAgentCoordinator::new(executor);
    
    // 定义 3 个并行任务
    let tasks = vec![
        SubagentConfig {
            id: "task-1".into(),
            name: "创建文件A".into(),
            task: "在 /tmp/multi-test 目录创建 file-a.txt，内容为 'Agent A'".into(),
        },
        SubagentConfig {
            id: "task-2".into(),
            name: "创建文件B".into(),
            task: "在 /tmp/multi-test 目录创建 file-b.txt，内容为 'Agent B'".into(),
        },
        SubagentConfig {
            id: "task-3".into(),
            name: "创建文件C".into(),
            task: "在 /tmp/multi-test 目录创建 file-c.txt，内容为 'Agent C'".into(),
        },
    ];
    
    std::fs::create_dir_all("/tmp/multi-test").ok();
    
    println!("启动 {} 个子代理并行执行...\n", tasks.len());
    
    let results = coordinator.run_parallel(tasks).await;
    
    println!("执行结果:");
    for (id, result) in &results {
        println!("  {} [{}]: {}", result.status, id, 
            if result.output.len() > 50 { &result.output[..50] } else { &result.output });
    }
    
    // 验证文件
    println!("\n验证产物:");
    let files = ["file-a.txt", "file-b.txt", "file-c.txt"];
    for f in &files {
        let path = format!("/tmp/multi-test/{}", f);
        if std::path::Path::new(&path).exists() {
            let content = std::fs::read_to_string(&path).unwrap();
            println!("  ✅ {}: {}", f, content);
        } else {
            println!("  ❌ {} 不存在", f);
        }
    }
    
    println!("\n🎉 多智能体测试完成");
}