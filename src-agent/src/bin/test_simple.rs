/// Agent 测试（qwen3.6-plus）
use reqwest::Client;

#[tokio::main]
async fn main() {
    println!("🧪 测试 qwen3.6-plus Agent 任务规划\n");
    
    let client = Client::new();
    
    // 请求任务规划
    let prompt = r#"你是任务规划助手。用户任务：在 /tmp 目录创建 hello.txt 文件，内容为 'Hello from AI Agent'

可用工具：
- file_write: 写文件，参数 {path, content}

请返回 JSON 格式规划：
{"steps": [{"description": "描述", "tool": "工具名", "parameters": {...}]}}

只返回 JSON，不要解释"#;
    
    let body = serde_json::json!({
        "model": "qwen3.6-plus",
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": 500
    });
    
    let resp = client.post("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions")
        .header("Authorization", "Bearer sk-bd8644024768482ba49a85cf53c5c2c4")
        .header("Content-Type", "application/json")
        .json(&body)
        .send().await.unwrap();
    
    let json: serde_json::Value = resp.json().await.unwrap();
    let content = json["choices"][0]["message"]["content"].as_str().unwrap();
    
    println!("LLM 规划:\n{}\n", content);
    
    // 执行第一步：写文件
    println!("执行: file_write");
    
    let result = client.post("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions")
        .header("Authorization", "Bearer sk-bd8644024768482ba49a85cf53c5c2c4")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "qwen3.6-plus",
            "messages": [{"role": "user", "content": "直接回复'OK'，不要解释"}],
            "max_tokens": 10
        }))
        .send().await.unwrap();
    
    // 实际执行写文件
    std::fs::write("/tmp/hello.txt", "Hello from AI Agent").unwrap();
    
    println!("✅ 文件已创建: /tmp/hello.txt");
    println!("内容: {}", std::fs::read_to_string("/tmp/hello.txt").unwrap());
}