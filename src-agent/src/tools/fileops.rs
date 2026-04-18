/// 文件操作工具
/// 支持：读、写、列目录、移动、删除、搜索

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use super::trait::{Tool, ToolError, StepResult, RiskLevel};
use crate::agent::task::Step;

/// 文件读取工具
pub struct FileReadTool;

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }
    
    fn description(&self) -> &str {
        "读取文件内容"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "文件路径"
                },
                "encoding": {
                    "type": "string",
                    "description": "文件编码，默认 utf-8",
                    "default": "utf-8"
                }
            },
            "required": ["path"]
        })
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    async fn execute(
        &self,
        parameters: serde_json::Value,
        allowed_paths: &[String],
    ) -> Result<StepResult, ToolError> {
        let path = parameters["path"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required"))?;
        
        // 检查路径权限
        check_path_permission(path, allowed_paths)?;
        
        // 读取文件
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        Ok(StepResult {
            output: content,
            files: Some(vec![path.to_string()]),
            data: None,
        })
    }
}

/// 文件写入工具
pub struct FileWriteTool;

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &str {
        "file_write"
    }
    
    fn description(&self) -> &str {
        "写入文件内容（覆盖或追加）"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "文件路径"
                },
                "content": {
                    "type": "string",
                    "description": "要写入的内容"
                },
                "mode": {
                    "type": "string",
                    "description": "写入模式：overwrite（覆盖）或 append（追加）",
                    "default": "overwrite"
                }
            },
            "required": ["path", "content"]
        })
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    fn requires_confirmation(&self) -> bool {
        true
    }
    
    async fn execute(
        &self,
        parameters: serde_json::Value,
        allowed_paths: &[String],
    ) -> Result<StepResult, ToolError> {
        let path = parameters["path"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required"))?;
        let content = parameters["content"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("content is required"))?;
        let mode = parameters["mode"].as_str().unwrap_or("overwrite");
        
        // 检查路径权限
        check_path_permission(path, allowed_paths)?;
        
        // 写入文件
        if mode == "append" {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(path)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
            file.write_all(content.as_bytes())
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        } else {
            fs::write(path, content)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        }
        
        Ok(StepResult {
            output: format!("Successfully written to {}", path),
            files: Some(vec![path.to_string()]),
            data: None,
        })
    }
}

/// 目录列表工具
pub struct DirListTool;

#[async_trait]
impl Tool for DirListTool {
    fn name(&self) -> &str {
        "dir_list"
    }
    
    fn description(&self) -> &str {
        "列出目录下的文件和子目录"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "目录路径"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "是否递归列出",
                    "default": false
                },
                "filter": {
                    "type": "string",
                    "description": "文件类型过滤器，如 *.txt"
                }
            },
            "required": ["path"]
        })
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    async fn execute(
        &self,
        parameters: serde_json::Value,
        allowed_paths: &[String],
    ) -> Result<StepResult, ToolError> {
        let path = parameters["path"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required"))?;
        let recursive = parameters["recursive"].as_bool().unwrap_or(false);
        
        check_path_permission(path, allowed_paths)?;
        
        let mut entries = Vec::new();
        let mut dir_queue = vec![PathBuf::from(path)];
        
        while let Some(dir) = dir_queue.pop() {
            let mut read_dir = fs::read_dir(&dir)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
            
            while let Some(entry) = read_dir.next_entry().await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))? {
                let entry_path = entry.path();
                let entry_type = entry.file_type().await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
                
                entries.push(format!("{} {}", 
                    if entry_type.is_dir() { "[DIR]" } else { "[FILE]" },
                    entry_path.display()
                ));
                
                if recursive && entry_type.is_dir() {
                    dir_queue.push(entry_path);
                }
            }
        }
        
        Ok(StepResult {
            output: entries.join("\n"),
            files: None,
            data: None,
        })
    }
}

/// 文件移动工具
pub struct FileMoveTool;

#[async_trait]
impl Tool for FileMoveTool {
    fn name(&self) -> &str {
        "file_move"
    }
    
    fn description(&self) -> &str {
        "移动文件或目录"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "source": {
                    "type": "string",
                    "description": "源路径"
                },
                "destination": {
                    "type": "string",
                    "description": "目标路径"
                }
            },
            "required": ["source", "destination"]
        })
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    fn requires_confirmation(&self) -> bool {
        true
    }
    
    async fn execute(
        &self,
        parameters: serde_json::Value,
        allowed_paths: &[String],
    ) -> Result<StepResult, ToolError> {
        let source = parameters["source"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("source is required"))?;
        let dest = parameters["destination"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("destination is required"))?;
        
        check_path_permission(source, allowed_paths)?;
        check_path_permission(dest, allowed_paths)?;
        
        fs::rename(source, dest)
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        Ok(StepResult {
            output: format!("Moved {} to {}", source, dest),
            files: Some(vec![dest.to_string()]),
            data: None,
        })
    }
}

/// 文件删除工具
pub struct FileDeleteTool;

#[async_trait]
impl Tool for FileDeleteTool {
    fn name(&self) -> &str {
        "file_delete"
    }
    
    fn description(&self) -> &str {
        "删除文件或目录"
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "要删除的路径"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "是否递归删除目录",
                    "default": false
                }
            },
            "required": ["path"]
        })
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::High
    }
    
    fn requires_confirmation(&self) -> bool {
        true
    }
    
    async fn execute(
        &self,
        parameters: serde_json::Value,
        allowed_paths: &[String],
    ) -> Result<StepResult, ToolError> {
        let path = parameters["path"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required"))?;
        let recursive = parameters["recursive"].as_bool().unwrap_or(false);
        
        check_path_permission(path, allowed_paths)?;
        
        if recursive {
            fs::remove_dir_all(path)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        } else {
            fs::remove_file(path)
                .await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        }
        
        Ok(StepResult {
            output: format!("Deleted {}", path),
            files: None,
            data: None,
        })
    }
}

/// 检查路径权限
fn check_path_permission(path: &str, allowed_paths: &[String]) -> Result<(), ToolError> {
    let path_buf = PathBuf::from(path);
    
    // 检查路径是否在允许范围内
    let is_allowed = allowed_paths.iter().any(|allowed| {
        let allowed_buf = PathBuf::from(allowed);
        path_buf.starts_with(&allowed_buf) || path_buf == allowed_buf
    });
    
    if !is_allowed {
        return Err(ToolError::PermissionDenied(path.to_string()));
    }
    
    // 检查是否访问敏感路径
    let forbidden_paths = [
        "/etc/passwd",
        "/etc/shadow",
        "~/.ssh",
        "~/.gnupg",
    ];
    
    for forbidden in forbidden_paths {
        if path.contains(forbidden) {
            return Err(ToolError::PermissionDenied(path.to_string()));
        }
    }
    
    Ok(())
}