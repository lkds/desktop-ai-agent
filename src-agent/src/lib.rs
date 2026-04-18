/// QoderWork Agent Library Entry
/// 统一导出所有模块

pub mod providers;
pub mod agent;
pub mod tools;
pub mod skills;
pub mod config;

#[cfg(feature = "tauri")]
pub mod ipc;

// Re-export main types
pub use agent::{Task, AgentExecutor, AgentError};
pub use providers::{Provider, ProviderConfig, ProviderKind, ProviderError, OpenAIProvider};
pub use tools::{Tool, ToolRegistry, ToolError};
pub use skills::{Skill, SkillsManager, SkillError};
pub use config::AppConfig;