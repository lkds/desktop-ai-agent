/// Tool Registry
use std::collections::HashMap;
use std::sync::Arc;
use super::tool_trait::Tool;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }
    
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }
    
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }
    
    pub fn list_tools(&self) -> Vec<Arc<dyn Tool>> {
        self.tools.values().cloned().collect()
    }
    
    pub fn list_tools_info(&self) -> Vec<ToolInfo> {
        self.tools.values().map(|tool| ToolInfo {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            parameters: tool.parameters_schema(),
        }).collect()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

pub fn init_default_tools() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(crate::tools::fileops::FileReadTool));
    registry.register(Arc::new(crate::tools::fileops::FileWriteTool));
    registry.register(Arc::new(crate::tools::fileops::DirListTool));
    registry.register(Arc::new(crate::tools::fileops::FileMoveTool));
    registry.register(Arc::new(crate::tools::fileops::FileDeleteTool));
    registry.register(Arc::new(crate::tools::browser::BrowserOpenTool));
    registry.register(Arc::new(crate::tools::browser::BrowserSearchTool));
    registry.register(Arc::new(crate::tools::shell::ShellExecuteTool));
    registry.register(Arc::new(crate::tools::shell::ShellScriptTool));
    registry
}