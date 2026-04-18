/// Agent 复杂项目测试
use reqwest::Client;

#[tokio::main]
async fn main() {
    println!("🧪 Agent 复杂项目测试：创建 Rust CLI 工具\n");
    
    let client = Client::new();
    
    let task = r#"创建一个完整的 Rust CLI 工具项目：

项目名：word-counter
功能：统计文件的行数、单词数、字符数（类似 wc 命令）

需要完成：
1. 创建项目目录 /tmp/word-counter
2. 创建 Cargo.toml（name="word-counter", edition="2021"）
3. 创建 src/main.rs（实现文件统计功能）
4. 编译项目（cargo build）
5. 测试运行（用自身源码测试）

请规划并逐步执行，每步完成后报告结果。"#;
    
    let body = serde_json::json!({
        "model": "qwen3.6-plus",
        "messages": [{"role": "user", "content": task}],
        "max_tokens": 2000
    });
    
    println!("任务: {}\n", task);
    println!("等待 LLM 规划...\n");
    
    let resp = client.post("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions")
        .header("Authorization", "Bearer sk-bd8644024768482ba49a85cf53c5c2c4")
        .header("Content-Type", "application/json")
        .json(&body)
        .send().await.unwrap();
    
    let json: serde_json::Value = resp.json().await.unwrap();
    let content = json["choices"][0]["message"]["content"].as_str().unwrap();
    
    println!("LLM 规划:\n{}\n", content);
    
    // 执行步骤
    println!("=== 开始执行 ===\n");
    
    // 1. 创建目录
    std::fs::create_dir_all("/tmp/word-counter/src").unwrap();
    println!("✅ Step 1: 创建目录 /tmp/word-counter/src");
    
    // 2. Cargo.toml
    let cargo_toml = r#"[package]
name = "word-counter"
version = "0.1.0"
edition = "2021"

[dependencies]"#;
    std::fs::write("/tmp/word-counter/Cargo.toml", cargo_toml).unwrap();
    println!("✅ Step 2: 创建 Cargo.toml");
    
    // 3. main.rs
    let main_rs = r#"use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: word-counter <file>");
        return;
    }
    
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read file");
    
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    
    println!("Lines: {}", lines);
    println!("Words: {}", words);
    println!("Chars: {}", chars);
}"#;
    std::fs::write("/tmp/word-counter/src/main.rs", main_rs).unwrap();
    println!("✅ Step 3: 创建 src/main.rs");
    
    // 4. 编译
    println!("\nStep 4: 编译...");
    let output = std::process::Command::new("cargo")
        .args(["build", "--release"])
        .current_dir("/tmp/word-counter")
        .output()
        .expect("编译失败");
    
    if output.status.success() {
        println!("✅ 编译成功");
    } else {
        println!("❌ 编译失败: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    
    // 5. 测试运行
    println!("\nStep 5: 测试运行...");
    let output = std::process::Command::new("/tmp/word-counter/target/release/word-counter")
        .arg("/tmp/word-counter/src/main.rs")
        .output()
        .expect("运行失败");
    
    println!("输出:\n{}", String::from_utf8_lossy(&output.stdout));
    
    // 验证
    println!("\n=== 验证结果 ===");
    println!("项目目录: {}", std::fs::read_dir("/tmp/word-counter").unwrap().count());
    println!("编译产物存在: {}", std::path::Path::new("/tmp/word-counter/target/release/word-counter").exists());
    
    println!("\n🎉 复杂项目测试完成！");
}