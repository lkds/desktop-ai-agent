/// Shell 命令执行工具
use async_trait::async_trait;
use super::tool_trait::{Tool, ToolError, StepResult, RiskLevel};

pub struct ShellExecuteTool;

#[async_trait]
impl Tool for ShellExecuteTool {
    fn name(&self) -> &str { "shell_execute" }
    fn description(&self) -> &str { "执行 shell 命令" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "要执行的命令" },
                "timeout": { "type": "integer", "description": "超时秒数", "default": 30 }
            },
            "required": ["command"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::High }
    fn requires_confirmation(&self) -> bool { true }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let command = params["command"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("command required".into()))?;
        let timeout = params["timeout"].as_i64().unwrap_or(30);
        
        // 安全检查：禁止危险命令
        let dangerous = ["rm -rf", "sudo", "chmod 777", "mkfs", "dd", "format"];
        for d in dangerous {
            if command.contains(d) {
                return Err(ToolError::ExecutionFailed(format!("危险命令被禁止: {}", d)));
            }
        }
        
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        let result = if output.status.success() {
            stdout.to_string()
        } else {
            format!("错误: {}", stderr)
        };
        
        Ok(StepResult {
            output: result,
            files: None,
            data: None,
        })
    }
}

pub struct ShellScriptTool;

#[async_trait]
impl Tool for ShellScriptTool {
    fn name(&self) -> &str { "shell_script" }
    fn description(&self) -> &str { "执行多行脚本" }
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "script": { "type": "string", "description": "多行脚本内容" }
            },
            "required": ["script"]
        })
    }
    fn risk_level(&self) -> RiskLevel { RiskLevel::High }
    fn requires_confirmation(&self) -> bool { true }
    
    async fn execute(&self, params: serde_json::Value, _allowed: &[String]) -> Result<StepResult, ToolError> {
        let script = params["script"].as_str()
            .ok_or_else(|| ToolError::InvalidParameters("script required".into()))?;
        
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .output()
            .await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        Ok(StepResult {
            output: format!("stdout: {}\nstderr: {}", stdout, stderr),
            files: None,
            data: None,
        })
    }
}