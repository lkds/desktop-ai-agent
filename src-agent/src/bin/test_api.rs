/// 直接测试 Coding Plan API
use reqwest::Client;

#[tokio::main]
async fn main() {
    println!("🧪 直接测试 Coding Plan API\n");
    
    let client = Client::new();
    let url = "https://coding.dashscope.aliyuncs.com/v1/chat/completions";
    
    let body = serde_json::json!({
        "model": "qwen3.5-plus",
        "messages": [{"role": "user", "content": "你好，请回复一句话"}]
    });
    
    let response = client.post(url)
        .header("Authorization", "Bearer sk-sp-196f167ed8d947f89581f1dcf4d6fb68")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("请求失败");
    
    println!("Status: {}", response.status());
    
    let text = response.text().await.expect("读取失败");
    
    // 只显示回复内容
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
        if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
            println!("回复: {}", content);
        } else {
            println!("响应: {}", text);
        }
    } else {
        println!("响应: {}", text);
    }
}