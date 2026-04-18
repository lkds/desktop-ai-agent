/// Agent 执行器
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

use super::task::*;
use crate::providers::provider_trait::{Provider, GenerateRequest, Message, Role};
use crate::tools::ToolRegistry;

pub struct AgentExecutor {
    provider: Arc<dyn Provider>,
    tool_registry: Arc<ToolRegistry>,
    current_task: RwLock<Option<Task>>,
    allowed_paths: RwLock<Vec<String>>,
}

impl AgentExecutor {
    pub fn new(provider: Arc<dyn Provider>, tool_registry: Arc<ToolRegistry>) -> Self {
        Self {
            provider,
            tool_registry,
            current_task: RwLock::new(None),
            allowed_paths: RwLock::new(Vec::new()),
        }
    }
    
    pub async fn set_allowed_paths(&self, paths: Vec<String>) {
        *self.allowed_paths.write().await = paths;
    }
    
    pub async fn execute(&self, description: String) -> Result<Task, AgentError> {
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
        
        *self.current_task.write().await = Some(task.clone());
        
        // 1. 规划阶段
        let plan = self.plan(&description).await?;
        
        {
            let mut task_guard = self.current_task.write().await;
            if let Some(t) = task_guard.as_mut() {
                t.status = TaskStatus::Planning;
                t.steps = plan.steps.iter().enumerate().map(|(i, step)| Step {
                    id: format!("step-{}", i),
                    description: step.description.clone(),
                    status: StepStatus::Pending,
                    tool: step.tool.clone(),
                    parameters: step.parameters.clone(),
                    result: None,
                    error: None,
                    started_at: None,
                    finished_at: None,
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
        
        let allowed_paths = self.allowed_paths.read().await.clone();
        
        let task_guard = self.current_task.read().await;
        let steps_count = task_guard.as_ref().map(|t| t.steps.len()).unwrap_or(0);
        
        for i in 0..steps_count {
            let step = {
                let guard = self.current_task.read().await;
                guard.as_ref().and_then(|t| t.steps.get(i).cloned())
            };
            
            if let Some(step) = step {
                let step_result = self.execute_step(&step, &allowed_paths).await;
                
                {
                    let mut task_guard = self.current_task.write().await;
                    if let Some(t) = task_guard.as_mut() {
                        match step_result {
                            Ok(result) => {
                                t.steps[i].status = StepStatus::Success;
                                t.steps[i].result = Some(StepResult {
                                    output: result.output,
                                    files: result.files,
                                    data: result.data,
                                });
                                t.steps[i].finished_at = Some(Utc::now());
                            }
                            Err(e) => {
                                t.steps[i].status = StepStatus::Failed;
                                t.steps[i].error = Some(e.to_string());
                                t.steps[i].finished_at = Some(Utc::now());
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
                    let summary = t.steps.iter()
                        .map(|s| format!("- {}: {}", s.description, 
                            s.result.as_ref().map(|r| r.output.clone()).unwrap_or_else(|| 
                                s.error.clone().unwrap_or_default())
                        ))
                        .collect::<Vec<_>>()
                        .join("\n");
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
        
        let task_guard = self.current_task.read().await;
        Ok(task_guard.clone().unwrap())
    }
    
    async fn plan(&self, description: &str) -> Result<PlanResponse, AgentError> {
        let tools_info: Vec<ToolInfo> = self.tool_registry.list_tools()
            .into_iter()
            .map(|tool| ToolInfo {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                parameters: tool.parameters_schema(),
            })
            .collect();
        
        let tools_desc = tools_info.iter()
            .map(|t| format!("- {}: {} - 参数: {}", t.name, t.description, 
                serde_json::to_string(&t.parameters).unwrap_or_default()))
            .collect::<Vec<_>>()
            .join("\n");
        
        let system_prompt = format!(
            "你是一个任务规划助手。用户会给你一个任务描述，你需要：\n\
            1. 分析任务需求\n\
            2. 将任务拆解为可执行的步骤\n\
            3. 为每个步骤选择合适的工具\n\n\
            可用工具：\n{}\n\n\
            请返回 JSON 格式的规划结果：\n\
            {{\"steps\": [{{\"description\": \"步骤描述\", \"tool\": \"工具名称\", \"parameters\": {{}}}}], \"reasoning\": \"思考过程\"}}\n\n\
            注意：参数必须是有效的 JSON",
            tools_desc
        );
        
        let request = GenerateRequest {
            messages: vec![
                Message { role: Role::System, content: system_prompt, tool_calls: None },
                Message { role: Role::User, content: description.to_string(), tool_calls: None },
            ],
            tools: None,
            temperature: Some(0.3),
            max_tokens: Some(2000),
            stream: false,
        };
        
        let response = self.provider.generate(request).await
            .map_err(|e| AgentError::ProviderError(e.to_string()))?;
        
        let plan: PlanResponse = serde_json::from_str(&response.message.content)
            .map_err(|e| AgentError::ParseError(e.to_string()))?;
        
        Ok(plan)
    }
    
    async fn execute_step(&self, step: &Step, allowed_paths: &[String]) -> Result<crate::tools::StepResult, AgentError> {
        let tool = self.tool_registry.get_tool(&step.tool)
            .ok_or_else(|| AgentError::ToolNotFound(step.tool.clone()))?;
        
        let result = tool.execute(step.parameters.clone(), allowed_paths).await
            .map_err(|e| AgentError::ToolError(e.to_string()))?;
        
        Ok(result)
    }
    
    pub async fn pause(&self) -> Result<(), AgentError> {
        let mut task_guard = self.current_task.write().await;
        if let Some(t) = task_guard.as_mut() {
            t.status = TaskStatus::Paused;
            t.updated_at = Utc::now();
        }
        Ok(())
    }
    
    pub async fn cancel(&self) -> Result<(), AgentError> {
        let mut task_guard = self.current_task.write().await;
        if let Some(t) = task_guard.as_mut() {
            t.status = TaskStatus::Cancelled;
            t.updated_at = Utc::now();
        }
        Ok(())
    }
    
    pub async fn get_status(&self) -> Option<Task> {
        self.current_task.read().await.clone()
    }
}

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