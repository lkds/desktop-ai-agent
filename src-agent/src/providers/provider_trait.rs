/// LLM Provider 抽象层
/// 统一接口支持 OpenAI、Claude、Ollama、自定义 endpoint

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider 类型
    pub kind: ProviderKind,
    /// API Key（可选，本地模型不需要）
    pub api_key: Option<String>,
    /// Base URL（可选，默认使用官方 API）
    pub base_url: Option<String>,
    /// 模型名称
    pub model: String,
    /// 额外参数
    pub extra: HashMap<String, String>,
}

/// 支持的 Provider 类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    OpenAI,
    Claude,
    Ollama,
    Custom,
}

/// 消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    /// 工具调用（可选）
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::Tool => write!(f, "tool"),
        }
    }
}

/// 工具调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String, // JSON string
}

/// 工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
}

/// 生成请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub messages: Vec<Message>,
    pub tools: Option<Vec<ToolDefinition>>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

/// 生成响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub message: Message,
    pub finish_reason: FinishReason,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FinishReason {
    Stop,
    ToolCalls,
    Length,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// 流式响应 chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub delta: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub finish_reason: Option<FinishReason>,
}

/// Provider trait - 所有 LLM provider 必须实现
#[async_trait]
pub trait Provider: Send + Sync {
    /// Provider 名称
    fn name(&self) -> &str;
    
    /// 发送生成请求（非流式）
    async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, ProviderError>;
    
    /// 发送生成请求（流式）
    async fn generate_stream(
        &self,
        request: GenerateRequest,
    ) -> Result<Box<dyn futures::Stream<Item = StreamChunk> + Unpin + Send>, ProviderError>;
    
    /// 检查 Provider 是否可用
    async fn health_check(&self) -> Result<bool, ProviderError>;
    
    /// 获取可用模型列表
    async fn list_models(&self) -> Result<Vec<String>, ProviderError>;
}

/// Provider 错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderError {
    ApiKeyMissing,
    ConnectionFailed(String),
    RateLimited,
    InvalidResponse(String),
    ModelNotFound(String),
    InternalError(String),
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiKeyMissing => write!(f, "API key is missing"),
            Self::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            Self::RateLimited => write!(f, "Rate limited"),
            Self::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            Self::ModelNotFound(model) => write!(f, "Model not found: {}", model),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ProviderError {}