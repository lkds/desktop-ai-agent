/// Agent 任务执行核心结构
/// Task = 任务，Step = 步骤，Tool = 工具

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,      // 等待执行
    Planning,     // 正在规划
    Running,      // 正在执行
    Paused,       // 已暂停
    Completed,    // 已完成
    Failed,       // 失败
    Cancelled,    // 已取消
}

/// 步骤状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Pending,      // 等待执行
    Running,      // 正在执行
    Success,      // 成功
    Failed,       // 失败
    Skipped,      // 跳过
}

/// 任务定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// 任务 ID
    pub id: String,
    /// 用户输入的原始任务描述
    pub description: String,
    /// 任务状态
    pub status: TaskStatus,
    /// 执行步骤列表
    pub steps: Vec<Step>,
    /// 当前执行到第几步
    pub current_step: usize,
    /// 任务结果
    pub result: Option<TaskResult>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 元数据（可存储额外信息）
    pub metadata: HashMap<String, String>,
}

/// 执行步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    /// 步骤 ID
    pub id: String,
    /// 步骤描述（来自 LLM 规划）
    pub description: String,
    /// 步骤状态
    pub status: StepStatus,
    /// 要调用的工具名称
    pub tool: String,
    /// 工具参数（JSON）
    pub parameters: serde_json::Value,
    /// 执行结果
    pub result: Option<StepResult>,
    /// 错误信息
    pub error: Option<String>,
    /// 执行开始时间
    pub started_at: Option<DateTime<Utc>>,
    /// 执行结束时间
    pub finished_at: Option<DateTime<Utc>>,
}

/// 步骤执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// 输出内容
    pub output: String,
    /// 是否有文件产出
    pub files: Option<Vec<String>>,
    /// 额外数据
    pub data: Option<HashMap<String, serde_json::Value>>,
}

/// 任务最终结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// 总结性输出
    pub summary: String,
    /// 产出的文件列表
    pub files: Vec<String>,
    /// 详细数据
    pub data: HashMap<String, serde_json::Value>,
}

/// 任务规划请求（发给 LLM）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanRequest {
    /// 用户原始任务
    pub task: String,
    /// 可用工具列表
    pub available_tools: Vec<ToolInfo>,
    /// 文件权限范围（允许访问的路径）
    pub allowed_paths: Vec<String>,
    /// 上下文信息（如已有文件）
    pub context: String,
}

/// 工具信息（用于告诉 LLM 可用工具）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
}

/// LLM 返回的规划结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanResponse {
    /// 规划出的步骤列表
    pub steps: Vec<PlannedStep>,
    /// 是否需要更多信息
    pub need_more_info: Option<String>,
    /// LLM 的思考过程（可选）
    pub reasoning: Option<String>,
}

/// LLM 规划出的单个步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedStep {
    pub description: String,
    pub tool: String,
    pub parameters: serde_json::Value,
    pub dependencies: Option<Vec<String>>, // 依赖的前置步骤 ID
}