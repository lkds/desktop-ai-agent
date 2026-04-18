/// Tool Registry - 工具注册中心
/// 管理所有可用工具

use std::collections::HashMap;
use std::sync::Arc;

use super::trait::Tool;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
    
    /// 注册工具
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name(), tool);
    }
    
    /// 获取工具
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }
    
    /// 列出所有工具
    pub fn list_tools(&self) -> Vec<Arc<dyn Tool>> {
        self.tools.values().cloned().collect()
    }
    
    /// 列出工具信息（用于 LLM 规划）
    pub fn list_tools_info(&self) -> Vec<ToolInfo> {
        self.tools.values().map(|tool| ToolInfo {
            name: tool.name(),
            description: tool.description(),
            parameters: tool.parameters_schema(),
        }).collect()
    }
}

/// 工具信息（用于 LLM）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// 初始化默认工具集
pub fn init_default_tools() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    
    // 文件操作工具
    registry.register(Arc::new(FileReadTool));
    registry.register(Arc::new(FileWriteTool));
    registry.register(Arc::new(DirListTool));
    registry.register(Arc::new(FileMoveTool));
    registry.register(Arc::new(FileDeleteTool));
    
    // TODO: 添加更多工具
    // registry.register(Arc::new(BrowserTool));
    // registry.register(Arc::new(ShellTool));
    
    registry
}

use crate::tools::fileops::*;