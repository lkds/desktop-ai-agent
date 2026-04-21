/// 浏览器自动化工具
/// 使用 thirtyfour (Selenium WebDriver) 进行真实的网页操作

use async_trait::async_trait;
use thirtyfour::prelude::*;
use thirtyfour::{WebDriver, By, DesiredCapabilities, WebElement};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;
use scraper::{Html, Selector};
use serde::Serialize;

use super::tool_trait::{Tool, ToolError, StepResult, RiskLevel};

/// 全局 WebDriver 管理（懒加载）
static DRIVER: once_cell::sync::Lazy<Arc<RwLock<Option<WebDriver>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(None)));

/// 浏览器打开工具
pub struct BrowserOpenTool;

#[async_trait]
impl Tool for BrowserOpenTool {
    fn name(&self) -> &str { "browser_open" }
    fn description(&self) -> &str { "打开网页并获取内容（真实浏览器）" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "网址" },
                "wait_for": { "type": "string", "description": "等待元素 CSS 选择器（可选）" }
            },
            "required": ["url"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let url = params["url"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("url required".into()))?;
        let wait_for = params["wait_for"].as_str();
        
        let driver = get_or_create_driver().await?;
        
        driver.goto(url).await
            .map_err(|e| ToolError::ExecutionFailed(format!("导航失败: {}", e)))?;
        
        if let Some(selector) = wait_for {
            let _ = driver.query(By::Css(selector))
                .wait(Duration::from_secs(5), Duration::from_millis(100))
                .first()
                .await;
        } else {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        
        let source = driver.source().await
            .map_err(|e| ToolError::ExecutionFailed(format!("获取源码失败: {}", e)))?;
        
        let content = extract_main_content(&source);
        
        Ok(StepResult {
            output: format!("已打开 {}，内容:\n{}", url, content),
            files: None,
            data: Some(HashMap::from([
                ("url".to_string(), serde_json::Value::String(url.to_string())),
                ("title".to_string(), serde_json::Value::String(extract_title(&source))),
                ("content".to_string(), serde_json::Value::String(content)),
            ])),
        })
    }
}

/// 网页搜索工具
pub struct BrowserSearchTool;

#[derive(Serialize)]
struct SearchResult {
    title: String,
    url: String,
}

#[async_trait]
impl Tool for BrowserSearchTool {
    fn name(&self) -> &str { "browser_search" }
    fn description(&self) -> &str { "使用搜索引擎搜索（真实浏览器）" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "搜索关键词" },
                "engine": { "type": "string", "description": "搜索引擎: google/bing/baidu", "default": "google" },
                "limit": { "type": "integer", "description": "结果数量限制", "default": 10 }
            },
            "required": ["query"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let query = params["query"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("query required".into()))?;
        let engine = params["engine"].as_str().unwrap_or("google");
        let limit = params["limit"].as_i64().unwrap_or(10) as usize;
        
        let driver = get_or_create_driver().await?;
        
        let url = match engine {
            "google" => format!("https://www.google.com/search?q={}", urlencoding::encode(query)),
            "bing" => format!("https://www.bing.com/search?q={}", urlencoding::encode(query)),
            "baidu" => format!("https://www.baidu.com/s?wd={}", urlencoding::encode(query)),
            _ => format!("https://www.google.com/search?q={}", urlencoding::encode(query)),
        };
        
        driver.goto(&url).await
            .map_err(|e| ToolError::ExecutionFailed(format!("导航失败: {}", e)))?;
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        let source = driver.source().await
            .map_err(|e| ToolError::ExecutionFailed(format!("获取源码失败: {}", e)))?;
        
        let results = parse_search_results(&source, engine, limit);
        
        let output = results.iter()
            .enumerate()
            .map(|(i, r)| format!("{}. {}\n   {}", i+1, r.title, r.url))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        Ok(StepResult {
            output: format!("搜索 '{}' 结果:\n\n{}", query, output),
            files: None,
            data: Some(HashMap::from([
                ("query".to_string(), serde_json::Value::String(query.to_string())),
                ("engine".to_string(), serde_json::Value::String(engine.to_string())),
                ("results".to_string(), serde_json::to_value(&results).unwrap_or_default()),
            ])),
        })
    }
}

/// 网页抓取工具
pub struct BrowserScrapeTool;

#[async_trait]
impl Tool for BrowserScrapeTool {
    fn name(&self) -> &str { "browser_scrape" }
    fn description(&self) -> &str { "抓取网页特定内容（真实浏览器 + CSS 选择器）" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "网址" },
                "selector": { "type": "string", "description": "CSS 选择器" },
                "extract": { "type": "string", "description": "提取内容: text/html", "default": "text" },
                "multiple": { "type": "boolean", "description": "是否提取多个元素", "default": false }
            },
            "required": ["url", "selector"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let url = params["url"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("url required".into()))?;
        let selector = params["selector"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("selector required".into()))?;
        let extract_type = params["extract"].as_str().unwrap_or("text");
        let multiple = params["multiple"].as_bool().unwrap_or(false);
        
        let driver = get_or_create_driver().await?;
        
        driver.goto(url).await
            .map_err(|e| ToolError::ExecutionFailed(format!("导航失败: {}", e)))?;
        
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        if multiple {
            let elements = driver.find_all(By::Css(selector)).await
                .map_err(|e| ToolError::ExecutionFailed(format!("查找元素失败: {}", e)))?;
            
            let mut results: Vec<String> = Vec::new();
            for elem in elements.iter() {
                let content = extract_element_content_async(elem, extract_type).await;
                results.push(content);
            }
            
            Ok(StepResult {
                output: format!("从 {} 抓取 {} 个元素:\n{}", url, results.len(), results.join("\n")),
                files: None,
                data: Some(HashMap::from([
                    ("url".to_string(), serde_json::Value::String(url.to_string())),
                    ("selector".to_string(), serde_json::Value::String(selector.to_string())),
                    ("results".to_string(), serde_json::to_value(&results).unwrap_or_default()),
                ])),
            })
        } else {
            let elem = driver.find(By::Css(selector)).await
                .map_err(|e| ToolError::ExecutionFailed(format!("查找元素失败: {}", e)))?;
            
            let content = extract_element_content_async(&elem, extract_type).await;
            
            Ok(StepResult {
                output: format!("从 {} 抓取 '{}':\n{}", url, selector, content),
                files: None,
                data: Some(HashMap::from([
                    ("url".to_string(), serde_json::Value::String(url.to_string())),
                    ("selector".to_string(), serde_json::Value::String(selector.to_string())),
                    ("content".to_string(), serde_json::Value::String(content)),
                ])),
            })
        }
    }
}

/// 浏览器交互工具
pub struct BrowserInteractTool;

#[async_trait]
impl Tool for BrowserInteractTool {
    fn name(&self) -> &str { "browser_interact" }
    fn description(&self) -> &str { "与网页交互：点击、输入文本、截图" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": { "type": "string", "description": "操作: click/input/screenshot/scroll" },
                "selector": { "type": "string", "description": "目标元素选择器" },
                "value": { "type": "string", "description": "输入值" }
            },
            "required": ["action"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    fn requires_confirmation(&self) -> bool { true }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let action = params["action"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("action required".into()))?;
        
        let driver = get_or_create_driver().await?;
        
        match action {
            "click" => {
                let selector = params["selector"].as_str()
                    .ok_or_else(|| ToolError::InvalidParameters("selector required".into()))?;
                let elem = driver.find(By::Css(selector)).await
                    .map_err(|e| ToolError::ExecutionFailed(format!("查找元素失败: {}", e)))?;
                elem.click().await
                    .map_err(|e| ToolError::ExecutionFailed(format!("点击失败: {}", e)))?;
                tokio::time::sleep(Duration::from_millis(300)).await;
                Ok(StepResult { output: format!("已点击 '{}'", selector), files: None, data: None })
            },
            "input" => {
                let selector = params["selector"].as_str()
                    .ok_or_else(|| ToolError::InvalidParameters("selector required".into()))?;
                let value = params["value"].as_str()
                    .ok_or_else(|| ToolError::InvalidParameters("value required".into()))?;
                let elem = driver.find(By::Css(selector)).await
                    .map_err(|e| ToolError::ExecutionFailed(format!("查找元素失败: {}", e)))?;
                elem.clear().await.ok();
                elem.send_keys(value).await
                    .map_err(|e| ToolError::ExecutionFailed(format!("输入失败: {}", e)))?;
                Ok(StepResult { output: format!("已在 '{}' 输入 '{}'", selector, value), files: None, data: None })
            },
            "screenshot" => {
                let screenshot = driver.screenshot_as_png().await
                    .map_err(|e| ToolError::ExecutionFailed(format!("截图失败: {}", e)))?;
                let filename = format!("screenshot_{}.png", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
                tokio::fs::write(&filename, &screenshot).await
                    .map_err(|e| ToolError::ExecutionFailed(format!("保存失败: {}", e)))?;
                Ok(StepResult { output: format!("已截图: {}", filename), files: Some(vec![filename]), data: None })
            },
            "scroll" => {
                let y = params["y"].as_i64().unwrap_or(500);
                driver.execute(&format!("window.scrollBy(0, {})", y), vec![]).await
                    .map_err(|e| ToolError::ExecutionFailed(format!("滚动失败: {}", e)))?;
                Ok(StepResult { output: format!("已滚动 {}px", y), files: None, data: None })
            },
            _ => Err(ToolError::InvalidParameters(format!("未知操作: {}", action)))
        }
    }
}

/// 关闭浏览器工具
pub struct BrowserCloseTool;

#[async_trait]
impl Tool for BrowserCloseTool {
    fn name(&self) -> &str { "browser_close" }
    fn description(&self) -> &str { "关闭浏览器会话" }
    fn parameters_schema(&self) -> serde_json::Value { serde_json::json!({ "type": "object", "properties": {} }) }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, _params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let mut driver_guard = DRIVER.write().await;
        if let Some(driver) = driver_guard.take() {
            driver.quit().await.ok();
        }
        Ok(StepResult { output: "浏览器已关闭".to_string(), files: None, data: None })
    }
}

async fn get_or_create_driver() -> Result<WebDriver, ToolError> {
    let driver_guard = DRIVER.read().await;
    if driver_guard.is_some() {
        return Ok(driver_guard.as_ref().unwrap().clone());
    }
    
    let mut caps = DesiredCapabilities::chrome();
    let _ = caps.add_chrome_arg("--headless");
    let _ = caps.add_chrome_arg("--no-sandbox");
    let _ = caps.add_chrome_arg("--disable-dev-shm-usage");
    let _ = caps.add_chrome_arg("--disable-gpu");
    let _ = caps.add_chrome_arg("--window-size=1920,1080");
    
    let driver = WebDriver::new("http://localhost:4444", caps).await
        .map_err(|e| ToolError::ExecutionFailed(format!("连接 WebDriver 失败: {}. 请确保 chromedriver 在 localhost:4444 运行", e)))?;
    
    drop(driver_guard);
    let mut driver_guard = DRIVER.write().await;
    *driver_guard = Some(driver.clone());
    
    Ok(driver)
}

async fn extract_element_content_async(elem: &WebElement, extract_type: &str) -> String {
    match extract_type {
        "html" => elem.inner_html().await.unwrap_or_default(),
        _ => elem.text().await.unwrap_or_default()
    }
}

fn parse_search_results(html: &str, engine: &str, limit: usize) -> Vec<SearchResult> {
    let document = Html::parse_document(html);
    
    let selector = match engine {
        "google" => Selector::parse("div.g, div#search div.g").ok(),
        "bing" => Selector::parse("li.b_algo").ok(),
        "baidu" => Selector::parse("div.result, div.c-container").ok(),
        _ => Selector::parse("a").ok(),
    };
    
    let link_selector = Selector::parse("a").ok();
    let title_selector = Selector::parse("h3, h2").ok();
    
    let mut results = Vec::new();
    
    if let Some(sel) = selector {
        for elem in document.select(&sel).take(limit) {
            let title = title_selector.as_ref()
                .and_then(|ts| elem.select(ts).next())
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            
            let url = link_selector.as_ref()
                .and_then(|ls| elem.select(ls).next())
                .and_then(|e| e.value().attr("href"))
                .map(|s| s.to_string())
                .unwrap_or_default();
            
            if title.len() > 5 && url.starts_with("http") {
                results.push(SearchResult { title, url });
            }
        }
    }
    
    results
}

fn extract_title(html: &str) -> String {
    let document = Html::parse_document(html);
    Selector::parse("title")
        .ok()
        .and_then(|sel| document.select(&sel).next())
        .map(|e| e.text().collect::<String>())
        .unwrap_or_else(|| "No title".to_string())
}

fn extract_main_content(html: &str) -> String {
    let document = Html::parse_document(html);
    
    let content_selectors = ["article", "main", "div.content", "div.post", "div.article", "div#content", "div#main", "body"];
    
    for sel_str in content_selectors {
        if let Ok(sel) = Selector::parse(sel_str) {
            if let Some(elem) = document.select(&sel).next() {
                let text = elem.text().collect::<Vec<_>>().join(" ");
                let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
                if cleaned.len() > 100 {
                    return if cleaned.len() > 3000 { cleaned[..3000].to_string() + "..." } else { cleaned };
                }
            }
        }
    }
    
    let body_sel = Selector::parse("body").ok();
    let text = body_sel.as_ref()
        .and_then(|sel| document.select(sel).next())
        .map(|e| e.text().collect::<Vec<_>>().join(" "))
        .unwrap_or_default();
    
    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if cleaned.len() > 2000 { cleaned[..2000].to_string() + "..." } else { cleaned }
}