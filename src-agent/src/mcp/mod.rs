/// MCP (Model Context Protocol) 集成
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPToolResult {
    pub content: Vec<MCPContent>,
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPContent {
    pub type: String,
    pub text: String,
}

pub struct MCPManager {
    servers: HashMap<String, MCPServerConfig>,
    tools: HashMap<String, MCPTool>,
}

impl MCPManager {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
            tools: HashMap::new(),
        }
    }
    
    pub fn add_server(&mut self, config: MCPServerConfig) {
        self.servers.insert(config.name.clone(), config);
    }
    
    pub fn remove_server(&mut self, name: &str) {
        self.servers.remove(name);
    }
    
    pub fn list_servers(&self) -> Vec<&MCPServerConfig> {
        self.servers.values().collect()
    }
    
    pub fn list_tools(&self) -> Vec<&MCPTool> {
        self.tools.values().collect()
    }
    
    pub fn get_tool(&self, name: &str) -> Option<&MCPTool> {
        self.tools.get(name)
    }
    
    pub async fn discover_tools(&mut self, server_name: &str) -> Result<(), MCPError> {
        let config = self.servers.get(server_name)
            .ok_or_else(|| MCPError::ServerNotFound(server_name.to_string()))?;
        
        // 启动 MCP server 进程并获取工具列表
        // 简化版：预定义工具
        // 实际应通过 JSON-RPC 协议与 server 通信
        
        // 示例：Tavily MCP server
        if config.name == "tavily" {
            self.tools.insert("tavily-search".to_string(), MCPTool {
                name: "tavily-search".to_string(),
                description: "使用 Tavily API 进行网络搜索".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string" },
                        "max_results": { "type": "integer", "default": 5 }
                    },
                    "required": ["query"]
                }),
            });
        }
        
        Ok(())
    }
    
    pub async fn call_tool(&self, tool_name: &str, arguments: serde_json::Value) -> Result<MCPToolResult, MCPError> {
        let tool = self.tools.get(tool_name)
            .ok_or_else(|| MCPError::ToolNotFound(tool_name.to_string()))?;
        
        // 根据工具类型执行调用
        match tool_name {
            "tavily-search" => {
                let query = arguments["query"].as_str()
                    .ok_or_else(|| MCPError::InvalidArguments)?;
                
                // 调用 Tavily API
                let result = self.call_tavily(query).await?;
                
                Ok(MCPToolResult {
                    content: vec![MCPContent {
                        type: "text".to_string(),
                        text: result,
                    }],
                    is_error: false,
                })
            }
            _ => Err(MCPError::ToolNotImplemented(tool_name.to_string())),
        }
    }
    
    async fn call_tavily(&self, query: &str) -> Result<String, MCPError> {
        // Tavily API 调用（需要配置 API key）
        Ok(format!("搜索 '{}' 的结果...", query))
    }
}

#[derive(Debug, Clone)]
pub enum MCPError {
    ServerNotFound(String),
    ToolNotFound(String),
    ToolNotImplemented(String),
    InvalidArguments,
    ConnectionError(String),
}

impl std::fmt::Display for MCPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServerNotFound(name) => write!(f, "Server not found: {}", name),
            Self::ToolNotFound(name) => write!(f, "Tool not found: {}", name),
            Self::ToolNotImplemented(name) => write!(f, "Tool not implemented: {}", name),
            Self::InvalidArguments => write!(f, "Invalid arguments"),
            Self::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}

impl std::error::Error for MCPError {}