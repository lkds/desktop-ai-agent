/// 浏览器自动化工具
/// 使用 headless browser 进行网页操作

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::tool_trait::{Tool, ToolError, StepResult, RiskLevel};

/// 浏览器打开工具
pub struct BrowserOpenTool;

#[async_trait]
impl Tool for BrowserOpenTool {
    fn name(&self) -> &str { "browser_open" }
    fn description(&self) -> &str { "打开网页" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "网址" }
            },
            "required": ["url"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let url = params["url"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("url required".into()))?;
        
        // 使用 curl 获取网页内容（简化版，实际应使用 playwright）
        let output = tokio::process::Command::new("curl")
            .arg("-s")
            .arg("-L")
            .arg(url)
            .output()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        let content = String::from_utf8_lossy(&output.stdout);
        let content = if content.len() > 5000 {
            &content[..5000]
        } else {
            &content
        };
        
        Ok(StepResult {
            output: format!("已打开 {}，内容摘要:\n{}", url, content),
            files: None,
            data: Some(HashMap::from([
                ("url".to_string(), serde_json::Value::String(url.to_string())),
                ("content".to_string(), serde_json::Value::String(content.to_string())),
            ])),
        })
    }
}

/// 网页搜索工具
pub struct BrowserSearchTool;

#[async_trait]
impl Tool for BrowserSearchTool {
    fn name(&self) -> &str { "browser_search" }
    fn description(&self) -> &str { "搜索引擎搜索" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "搜索关键词" },
                "engine": { "type": "string", "description": "搜索引擎", "default": "google" }
            },
            "required": ["query"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let query = params["query"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("query required".into()))?;
        let engine = params["engine"].as_str().unwrap_or("google");
        
        let url = match engine {
            "google" => format!("https://www.google.com/search?q={}", 
                urlencoding::encode(query)),
            "bing" => format!("https://www.bing.com/search?q={}", 
                urlencoding::encode(query)),
            "baidu" => format!("https://www.baidu.com/s?wd={}", 
                urlencoding::encode(query)),
            _ => format!("https://www.google.com/search?q={}", 
                urlencoding::encode(query)),
        };
        
        // 简化版：使用 curl
        let output = tokio::process::Command::new("curl")
            .arg("-s")
            .arg("-L")
            .arg(&url)
            .output()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        let html = String::from_utf8_lossy(&output.stdout);
        
        // 简单解析搜索结果（实际应使用 HTML parser）
        let results = extract_search_results(&html);
        
        Ok(StepResult {
            output: format!("搜索 '{}' 结果:\n{}", query, results.join("\n")),
            files: None,
            data: Some(HashMap::from([
                ("query".to_string(), serde_json::Value::String(query.to_string())),
                ("results".to_string(), serde_json::Value::Array(
                    results.iter().map(|r| serde_json::Value::String(r.clone())).collect()
                )),
            ])),
        })
    }
}

/// 网页抓取工具
pub struct BrowserScrapeTool;

#[async_trait]
impl Tool for BrowserScrapeTool {
    fn name(&self) -> &str { "browser_scrape" }
    fn description(&self) -> &str { "抓取网页特定内容" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "网址" },
                "selector": { "type": "string", "description": "CSS 选择器（简化版支持 text/title/links）" }
            },
            "required": ["url"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let url = params["url"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("url required".into()))?;
        let selector = params["selector"].as_str().unwrap_or("text");
        
        let output = tokio::process::Command::new("curl")
            .arg("-s")
            .arg("-L")
            .arg(url)
            .output()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        let html = String::from_utf8_lossy(&output.stdout);
        let result = match selector {
            "title" => extract_title(&html),
            "links" => extract_links(&html).join("\n"),
            "text" => extract_text(&html),
            _ => extract_text(&html),
        };
        
        Ok(StepResult {
            output: format!("从 {} 抓取 {}:\n{}", url, selector, result),
            files: None,
            data: None,
        })
    }
}

// 简化的 HTML 解析函数

fn extract_search_results(html: &str) -> Vec<String> {
    // 简化版：提取所有链接文本
    let mut results = Vec::new();
    for line in html.lines() {
        if line.contains("<a") && line.contains("href") {
            if let Some(start) = line.find(">") {
                if let Some(end) = line.find("</a>") {
                    let text = &line[start+1..end];
                    let text = text.trim();
                    if text.len() > 5 && text.len() < 200 {
                        results.push(text.to_string());
                    }
                }
            }
        }
    }
    if results.len() > 10 {
        results.truncate(10);
    }
    results
}

fn extract_title(html: &str) -> String {
    if let Some(start) = html.find("<title>") {
        if let Some(end) = html.find("</title>") {
            return html[start+7..end].trim().to_string();
        }
    }
    "No title".to_string()
}

fn extract_links(html: &str) -> Vec<String> {
    let mut links = Vec::new();
    for line in html.lines() {
        if line.contains("href=\"http") {
            if let Some(start) = line.find("href=\"") {
                if let Some(end) = line[start+6..].find("\"") {
                    let link = &line[start+6..start+6+end];
                    if link.starts_with("http") {
                        links.push(link.to_string());
                    }
                }
            }
        }
    }
    if links.len() > 20 {
        links.truncate(20);
    }
    links
}

fn extract_text(html: &str) -> String {
    // 简化版：去除标签
    let mut text = html.to_string();
    text = text.replace("<script", "").replace("</script>", "");
    text = text.replace("<style", "").replace("</style>", "");
    
    // 去除 HTML 标签
    let mut result = String::new();
    let mut in_tag = false;
    for c in text.chars() {
        if c == '<' { in_tag = true; }
        else if c == '>' { in_tag = false; }
        else if !in_tag { result.push(c); }
    }
    
    // 清理
    result = result.split_whitespace().collect::<Vec<_>>().join(" ");
    if result.len() > 2000 {
        result = result[..2000].to_string();
    }
    result
}