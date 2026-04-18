/// 工具执行测试
use qoderwork_agent::tools::init_default_tools;

#[tokio::main]
async fn main() {
    println!("🧪 工具执行测试\n");
    
    let registry = init_default_tools();
    let allowed = vec!["/tmp/test-rust".to_string()];
    
    std::fs::create_dir_all("/tmp/test-rust").ok();
    
    // 测试 file_write
    println!("1. 测试 file_write");
    let write_tool = registry.get_tool("file_write").unwrap();
    let result = write_tool.execute(
        serde_json::json!({"path": "/tmp/test-rust/hello.txt", "content": "Hello from Rust Agent!"}),
        &allowed
    ).await;
    match result {
        Ok(r) => println!("   ✅ {}", r.output),
        Err(e) => println!("   ❌ {}", e),
    }
    
    // 测试 file_read
    println!("\n2. 测试 file_read");
    let read_tool = registry.get_tool("file_read").unwrap();
    let result = read_tool.execute(
        serde_json::json!({"path": "/tmp/test-rust/hello.txt"}),
        &allowed
    ).await;
    match result {
        Ok(r) => println!("   ✅ 内容: {}", r.output),
        Err(e) => println!("   ❌ {}", e),
    }
    
    // 测试 dir_list
    println!("\n3. 测试 dir_list");
    let list_tool = registry.get_tool("dir_list").unwrap();
    let result = list_tool.execute(
        serde_json::json!({"path": "/tmp/test-rust"}),
        &allowed
    ).await;
    match result {
        Ok(r) => println!("   ✅ {}", r.output),
        Err(e) => println!("   ❌ {}", e),
    }
    
    // 测试 browser_open
    println!("\n4. 测试 browser_open");
    let browser_tool = registry.get_tool("browser_open").unwrap();
    let result = browser_tool.execute(
        serde_json::json!({"url": "https://example.com"}),
        &allowed
    ).await;
    match result {
        Ok(r) => {
            let s = if r.output.len() > 200 { &r.output[..200] } else { &r.output };
            println!("   ✅ {}", s);
        }
        Err(e) => println!("   ❌ {}", e),
    }
    
    // 测试 shell_execute
    println!("\n5. 测试 shell_execute");
    let shell_tool = registry.get_tool("shell_execute").unwrap();
    let result = shell_tool.execute(
        serde_json::json!({"command": "echo 'Shell test OK'"}),
        &allowed
    ).await;
    match result {
        Ok(r) => println!("   ✅ {}", r.output),
        Err(e) => println!("   ❌ {}", e),
    }
    
    println!("\n=== 所有测试完成 ===");
}