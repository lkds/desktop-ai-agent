/// 配置管理模块
/// 负责读取、保存、验证配置

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

use crate::providers::provider_trait::ProviderConfig;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// LLM Provider 配置
    pub provider: ProviderConfig,
    /// 允许访问的路径列表
    pub allowed_paths: Vec<String>,
    /// Skills 目录
    pub skills_dir: String,
    /// 其他配置
    pub extra: HashMap<String, String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            provider: ProviderConfig {
                kind: crate::providers::ProviderKind::OpenAI,
                api_key: None,
                base_url: None,
                model: "gpt-4".to_string(),
                extra: HashMap::new(),
            },
            allowed_paths: vec![
                "~".to_string(),  // 默认允许访问用户目录
            ],
            skills_dir: "~/.desktop-agent/skills".to_string(),
            extra: HashMap::new(),
        }
    }
}

impl AppConfig {
    /// 从文件加载配置
    pub fn load(path: &PathBuf) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::ReadError(e.to_string()))?;
        
        let config: Self = serde_json::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        
        Ok(config)
    }
    
    /// 保存配置到文件
    pub fn save(&self, path: &PathBuf) -> Result<(), ConfigError> {
        // 确保目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::WriteError(e.to_string()))?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::SerializeError(e.to_string()))?;
        
        fs::write(path, content)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;
        
        Ok(())
    }
    
    /// 获取配置文件默认路径
    pub fn default_path() -> PathBuf {
        let home = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("~"));
        home.join(".desktop-agent").join("config.json")
    }
    
    /// 验证配置
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 检查 Provider 配置
        if self.provider.api_key.is_none() && 
           self.provider.kind != crate::providers::ProviderKind::Ollama {
            return Err(ConfigError::ValidationError("API key is required for non-local providers".to_string()));
        }
        
        // 检查模型名称
        if self.provider.model.is_empty() {
            return Err(ConfigError::ValidationError("Model name is required".to_string()));
        }
        
        // 检查允许路径
        if self.allowed_paths.is_empty() {
            return Err(ConfigError::ValidationError("At least one allowed path is required".to_string()));
        }
        
        Ok(())
    }
}

/// 配置错误
#[derive(Debug, Clone)]
pub enum ConfigError {
    ReadError(String),
    ParseError(String),
    WriteError(String),
    SerializeError(String),
    ValidationError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadError(msg) => write!(f, "Read error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::WriteError(msg) => write!(f, "Write error: {}", msg),
            Self::SerializeError(msg) => write!(f, "Serialize error: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}