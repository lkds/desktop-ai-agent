/// 文件操作工具
use async_trait::async_trait;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use super::tool_trait::{Tool, ToolError, StepResult, RiskLevel};

pub struct FileReadTool;

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str { "file_read" }
    fn description(&self) -> &str { "读取文件内容" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]})
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, allowed: &[String]) -> Result<StepResult, ToolError> {
        let path = params["path"].as_str().ok_or_else(|| ToolError::InvalidParameters("path required".into()))?;
        check_path(path, allowed)?;
        let content = fs::read_to_string(path).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(StepResult { output: content, files: Some(vec![path.into()]), data: None })
    }
}

pub struct FileWriteTool;

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &str { "file_write" }
    fn description(&self) -> &str { "写入文件内容" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}, "mode": {"type": "string", "default": "overwrite"}}, "required": ["path", "content"]})
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    fn requires_confirmation(&self) -> bool { true }
    
    async fn execute(&self, params: serde_json::Value, allowed: &[String]) -> Result<StepResult, ToolError> {
        let path = params["path"].as_str().ok_or_else(|| ToolError::InvalidParameters("path required".into()))?;
        let content = params["content"].as_str().ok_or_else(|| ToolError::InvalidParameters("content required".into()))?;
        check_path(path, allowed)?;
        fs::write(path, content).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(StepResult { output: format!("Written to {}", path), files: Some(vec![path.into()]), data: None })
    }
}

pub struct DirListTool;

#[async_trait]
impl Tool for DirListTool {
    fn name(&self) -> &str { "dir_list" }
    fn description(&self) -> &str { "列出目录内容" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type": "object", "properties": {"path": {"type": "string"}, "recursive": {"type": "boolean", "default": false}}, "required": ["path"]})
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    async fn execute(&self, params: serde_json::Value, allowed: &[String]) -> Result<StepResult, ToolError> {
        let path = params["path"].as_str().ok_or_else(|| ToolError::InvalidParameters("path required".into()))?;
        let recursive = params["recursive"].as_bool().unwrap_or(false);
        check_path(path, allowed)?;
        let mut entries = Vec::new();
        let mut queue = vec![PathBuf::from(path)];
        while let Some(dir) = queue.pop() {
            let mut read_dir = fs::read_dir(&dir).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
            while let Some(entry) = read_dir.next_entry().await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))? {
                let p = entry.path();
                let t = entry.file_type().await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
                entries.push(format!("{} {}", if t.is_dir() { "[DIR]" } else { "[FILE]" }, p.display()));
                if recursive && t.is_dir() { queue.push(p); }
            }
        }
        Ok(StepResult { output: entries.join("\n"), files: None, data: None })
    }
}

pub struct FileMoveTool;

#[async_trait]
impl Tool for FileMoveTool {
    fn name(&self) -> &str { "file_move" }
    fn description(&self) -> &str { "移动文件或目录" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type": "object", "properties": {"source": {"type": "string"}, "destination": {"type": "string"}}, "required": ["source", "destination"]})
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    fn requires_confirmation(&self) -> bool { true }
    
    async fn execute(&self, params: serde_json::Value, allowed: &[String]) -> Result<StepResult, ToolError> {
        let src = params["source"].as_str().ok_or_else(|| ToolError::InvalidParameters("source required".into()))?;
        let dst = params["destination"].as_str().ok_or_else(|| ToolError::InvalidParameters("destination required".into()))?;
        check_path(src, allowed)?;
        check_path(dst, allowed)?;
        fs::rename(src, dst).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(StepResult { output: format!("Moved {} to {}", src, dst), files: Some(vec![dst.into()]), data: None })
    }
}

pub struct FileDeleteTool;

#[async_trait]
impl Tool for FileDeleteTool {
    fn name(&self) -> &str { "file_delete" }
    fn description(&self) -> &str { "删除文件或目录" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({"type": "object", "properties": {"path": {"type": "string"}, "recursive": {"type": "boolean", "default": false}}, "required": ["path"]})
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::High }
    fn requires_confirmation(&self) -> bool { true }
    
    async fn execute(&self, params: serde_json::Value, allowed: &[String]) -> Result<StepResult, ToolError> {
        let path = params["path"].as_str().ok_or_else(|| ToolError::InvalidParameters("path required".into()))?;
        let recursive = params["recursive"].as_bool().unwrap_or(false);
        check_path(path, allowed)?;
        if recursive {
            fs::remove_dir_all(path).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        } else {
            fs::remove_file(path).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        }
        Ok(StepResult { output: format!("Deleted {}", path), files: None, data: None })
    }
}

fn check_path(path: &str, allowed: &[String]) -> Result<(), ToolError> {
    let pb = PathBuf::from(path);
    let ok = allowed.iter().any(|a| pb.starts_with(PathBuf::from(a)));
    if !ok { return Err(ToolError::PermissionDenied(path.into())); }
    Ok(())
}