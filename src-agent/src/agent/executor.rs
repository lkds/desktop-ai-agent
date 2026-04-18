/// Agent 执行器
/// 核心逻辑：接收任务 -> 规划 -> 执行 -> 返回结果

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;

use super::task::*;
use crate::providers::{Provider, GenerateRequest, Message, Role, ToolDefinition};
use crate::tools::{Tool, ToolRegistry, ToolError};

/// Agent 执行器
pub struct AgentExecutor {
    /// LLM Provider
    provider: Arc<dyn Provider>,
    /// 工具注册表
    tool_registry: Arc<ToolRegistry>,
    /// 当前任务（支持暂停恢复）
    current_task: RwLock<Option<Task>>,
    /// 允许访问的路径
    allowed_paths: RwLock<Vec<String>>,
}

impl AgentExecutor {
    pub fn new(
        provider: Arc<dyn Provider>,
        tool_registry: Arc<ToolRegistry>,
    ) -> Self {
        Self {
            provider,
            tool_registry,
            current_task: RwLock::new(None),
            allowed_paths: RwLock::new(Vec::new()),
        }
    }
    
    /// 设置允许访问的路径
    pub async fn set_allowed_paths(&self, paths: Vec<String>) {
        *self.allowed_paths.write().await = paths;
    }
    
    /// 执行任务
    pub async fn execute(&self, description: String) -> Result<Task, AgentError> {
        // 创建任务
        let task_id = Uuid::new_v4().to_string();
        let task = Task {
            id: task_id.clone(),
            description: description.clone(),
            status: TaskStatus::Pending,
            steps: Vec::new(),
            current_step: 0,
            result: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };
        
        // 保存任务
        *self.current_task.write().await = Some(task.clone());
        
        // 1. 规划阶段
        let plan = self.plan(&description).await?;
        
        // 更新任务状态
        {
            let mut task_guard = self.current_task.write().await;
            if let Some(t) = task_guard.as_mut() {
                t.status = TaskStatus::Planning;
                t.steps = plan.steps.iter().enumerate().map(|(i, step)| {
                    Step {
                        id: format!("step-{}", i),
                        description: step.description.clone(),
                        status: StepStatus::Pending,
                        tool: step.tool.clone(),
                        parameters: step.parameters.clone(),
                        result: None,
                        error: None,
                        started_at: None,
                        finished_at: None,
                    }
                }).collect();
                t.updated_at = Utc::now();
            }
        }
        
        // 2. 执行阶段
        {
            let mut task_guard = self.current_task.write().await;
            if let Some(t) = task_guard.as_mut() {
                t.status = TaskStatus::Running;
            }
        }
        
        // 按步骤执行
        let results = Vec::new();
        let allowed_paths = self.allowed_paths.read().await.clone();
        
        {
            let task_guard = self.current_task.read().await;
            let task = task_guard.as_ref().ok_or(AgentError::NoTask)?;
            
            for (i, step) in task.steps.iter().enumerate() {
                // 执行单步
                let step_result = self.execute_step(step, &allowed_paths).await;
                
                // 更新步骤状态
                {
                    let mut task_guard = self.current_task.write().await;
                    if let Some(t) = task_guard.as_mut() {
                        match step_result {
                            Ok(result) => {
                                t.steps[i].status = StepStatus::Success;
                                t.steps[i].result = Some(result);
                                t.steps[i].finished_at = Some(Utc::now());
                            }
                            Err(e) => {
                                t.steps[i].status = StepStatus::Failed;
                                t.steps[i].error = Some(e.to_string());
                                t.steps[i].finished_at = Some(Utc::now());
                                // 失败后停止执行
                                t.status = TaskStatus::Failed;
                                break;
                            }
                        }
                        t.current_step = i + 1;
                        t.updated_at = Utc::now();
                    }
                }
            }
        }
        
        // 3. 完成阶段
        {
            let mut task_guard = self.current_task.write().await;
            if let Some(t) = task_guard.as_mut() {
                if t.status != TaskStatus::Failed {
                    t.status = TaskStatus::Completed;
                    
                    // 生成任务总结
                    let summary = self.summarize(&t).await?;
                    t.result = Some(TaskResult {
                        summary,
                        files: t.steps.iter()
                            .filter_map(|s| s.result.as_ref())
                            .filter_map(|r| r.files.as_ref())
                            .flatten()
                            .cloned()
                            .collect(),
                        data: HashMap::new(),
                    });
                }
                t.updated_at = Utc::now();
            }
        }
        
        // 返回最终任务
        let task_guard = self.current_task.read().await;
        Ok(task_guard.clone().unwrap())
    }
    
    /// 规划任务（调用 LLM）
    async fn plan(&self, description: &str) -> Result<PlanResponse, AgentError> {
        // 构建可用工具信息
        let tools_info: Vec<ToolInfo> = self.tool_registry.list_tools()
            .into_iter()
            .map(|tool| ToolInfo {
                name: tool.name(),
                description: tool.description(),
                parameters: tool.parameters_schema(),
            })
            .collect();
        
        // 构建 LLM 请求
        let system_prompt = build_planning_prompt(&tools_info);
        
        let request = GenerateRequest {
            messages: vec![
                Message {
                    role: Role::System,
                    content: system_prompt,
                    tool_calls: None,
                },
                Message {
                    role: Role::User,
                    content: description.to_string(),
                    tool_calls: None,
                },
            ],
            tools: None, // 规划阶段不需要工具调用
            temperature: Some(0.3),
            max_tokens: Some(2000),
            stream: false,
        };
        
        let response = self.provider.generate(request).await
            .map_err(|e| AgentError::ProviderError(e.to_string()))?;
        
        // 解析 LLM 返回的规划
        let plan_json = response.message.content;
        let plan: PlanResponse = serde_json::from_str(&plan_json)
            .map_err(|e| AgentError::ParseError(e.to_string()))?;
        
        Ok(plan)
    }
    
    /// 执行单个步骤
    async fn execute_step(
        &self,
        step: &Step,
        allowed_paths: &[String],
    ) -> Result<StepResult, AgentError> {
        // 获取工具
        let tool = self.tool_registry.get_tool(&step.tool)
            .ok_or_else(|| AgentError::ToolNotFound(step.tool.clone()))?;
        
        // 执行工具
        let result = tool.execute(step.parameters.clone(), allowed_paths)
            .await
            .map_err(|e| AgentError::ToolError(e.to_string()))?;
        
        Ok(result)
    }
    
    /// 生成任务总结（调用 LLM）
    async fn summarize(&self, task: &Task) -> Result<String, AgentError> {
        let steps_summary = task.steps.iter()
            .map(|s| format!("- {}: {}", s.description, 
                s.result.as_ref().map(|r| r.output.clone()).unwrap_or_else(|| 
                    s.error.clone().unwrap_or_default())
            ))
            .join("\n");
        
        let request = GenerateRequest {
            messages: vec![
                Message {
                    role: Role::System,
                    content: "你是一个任务总结助手。根据任务执行步骤，生成简洁的结果总结。",
                    tool_calls: None,
                },
                Message {
                    role: Role::User,
                    content: format!("任务：{}\n执行步骤：\n{}\n请生成简洁的结果总结。", 
                        task.description, steps_summary),
                    tool_calls: None,
                },
            ],
            tools: None,
            temperature: Some(0.5),
            max_tokens: Some(500),
            stream: false,
        };
        
        let response = self.provider.generate(request).await
            .map_err(|e| AgentError::ProviderError(e.to_string()))?;
        
        Ok(response.message.content)
    }
    
    /// 暂停任务
    pub async fn pause(&self) -> Result<(), AgentError> {
        let mut task_guard = self.current_task.write().await;
        if let Some(t) = task_guard.as_mut() {
            t.status = TaskStatus::Paused;
            t.updated_at = Utc::now();
        }
        Ok(())
    }
    
    /// 恢复任务
    pub async fn resume(&self) -> Result<Task, AgentError> {
        // TODO: 从暂停点继续执行
        let task_guard = self.current_task.read().await;
        Ok(task_guard.clone().unwrap())
    }
    
    /// 取消任务
    pub async fn cancel(&self) -> Result<(), AgentError> {
        let mut task_guard = self.current_task.write().await;
        if let Some(t) = task_guard.as_mut() {
            t.status = TaskStatus::Cancelled;
            t.updated_at = Utc::now();
        }
        Ok(())
    }
    
    /// 获取当前任务状态
    pub async fn get_status(&self) -> Option<Task> {
        self.current_task.read().await.clone()
    }
}

/// Agent 错误类型
#[derive(Debug, Clone)]
pub enum AgentError {
    NoTask,
    ProviderError(String),
    ParseError(String),
    ToolNotFound(String),
    ToolError(String),
    InvalidState,
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTask => write!(f, "No task in progress"),
            Self::ProviderError(msg) => write!(f, "Provider error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::ToolNotFound(name) => write!(f, "Tool not found: {}", name),
            Self::ToolError(msg) => write!(f, "Tool error: {}", msg),
            Self::InvalidState => write!(f, "Invalid state"),
        }
    }
}

impl std::error::Error for AgentError {}

/// 构建规划提示词
fn build_planning_prompt(tools: &[ToolInfo]) -> String {
    let tools_desc = tools.iter()
        .map(|t| format!("- {}: {} - 参数: {}", t.name, t.description, 
            serde_json::to_string(&t.parameters).unwrap_or_default()))
        .join("\n");
    
    format!(
        r#"你是一个任务规划助手。用户会给你一个任务描述，你需要：
1. 分析任务需求
2. 将任务拆解为可执行的步骤
3. 为每个步骤选择合适的工具

可用工具：
{}

请返回 JSON 格式的规划结果：
{
  "steps": [
    {
      "description": "步骤描述",
      "tool": "工具名称",
      "parameters": { 工具参数 }
    }
  ],
  "reasoning": "你的思考过程"
}

注意：
- 步骤应该具体、可执行
- 参数必须是有效的 JSON
- 如果需要更多信息，返回 { "need_more_info": "需要什么信息" }
"#,
        tools_desc
    )
}