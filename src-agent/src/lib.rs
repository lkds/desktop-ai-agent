/// QoderWork Agent Library Entry
pub mod providers;
pub mod agent;
pub mod tools;
pub mod skills;
pub mod config;
pub mod scheduler;

#[cfg(feature = "tauri")]
pub mod ipc;

// Re-export main types
pub use agent::{Task, AgentExecutor, AgentError, SubagentConfig, SubagentResult, MultiAgentCoordinator};
pub use providers::{Provider, ProviderConfig, ProviderKind, ProviderError, OpenAIProvider, DashScopeProvider};
pub use tools::{Tool, ToolRegistry, ToolError, init_default_tools};
pub use scheduler::{ScheduledTask, Schedule, Scheduler};
pub use config::AppConfig;