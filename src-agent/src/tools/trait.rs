/// 工具抽象层
/// 所有工具必须实现 Tool trait

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 工具 trait
#[async_trait]
pub trait Tool: Send + Sync {
    /// 工具名称
    fn name(&self) -> &str;
    
    /// 工具描述
    fn description(&self) -> &str;
    
    /// 参数 Schema（JSON Schema 格式）
    fn parameters_schema(&self) -> serde_json::Value;
    
    /// 执行工具
    async fn execute(
        &self,
        parameters: serde_json::Value,
        allowed_paths: &[String],
    ) -> Result<StepResult, ToolError>;
    
    /// 是否需要用户确认
    fn requires_confirmation(&self) -> bool {
        false
    }
    
    /// 风险等级
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
}

/// 工具风险等级
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,      // 低风险：只读操作
    Medium,   // 中风险：修改文件
    High,     // 高风险：删除、执行命令
}

/// 工具执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub output: String,
    pub files: Option<Vec<String>>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

/// 工具错误
#[derive(Debug, Clone)]
pub enum ToolError {
    InvalidParameters(String),
    PermissionDenied(String),
    ExecutionFailed(String),
    FileNotFoundError(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            Self::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            Self::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            Self::FileNotFoundError(path) => write!(f, "File not found: {}", path),
        }
    }
}

impl std::error::Error for ToolError {}

use std::collections::HashMap;