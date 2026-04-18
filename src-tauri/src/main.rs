#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Tauri 应用入口
//! 集成 Rust Agent 后端

use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;

use qoderwork_agent::{
    AppConfig,
    AgentExecutor,
    ProviderKind,
    ToolRegistry,
    SkillsManager,
    providers::{OpenAIProvider, ClaudeProvider, OllamaProvider},
};

// 导入 IPC handlers
use qoderwork_agent::ipc::{
    AppState,
    execute_task, pause_task, cancel_task, get_task_status,
    get_config, save_config,
    list_skills, install_skill, uninstall_skill,
    get_task_history,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 加载配置
            let config_path = AppConfig::default_path();
            let config = AppConfig::load(&config_path).unwrap_or_default();
            
            // 初始化工具注册表
            let tool_registry = Arc::new(ToolRegistry::new());
            
            // 初始化 Skills Manager
            let skills_dir = PathBuf::from(&config.skills_dir);
            let skills_manager = Arc::new(Mutex::new(SkillsManager::new(skills_dir)));
            
            // 初始化 Provider
            let provider: Arc<dyn qoderwork_agent::Provider> = match config.provider.kind {
                ProviderKind::OpenAI => {
                    Arc::new(OpenAIProvider::new(config.provider.clone())
                        .expect("Failed to create OpenAI provider"))
                }
                ProviderKind::Claude => {
                    Arc::new(ClaudeProvider::new(config.provider.clone())
                        .expect("Failed to create Claude provider"))
                }
                ProviderKind::Ollama => {
                    Arc::new(OllamaProvider::new(config.provider.clone())
                        .expect("Failed to create Ollama provider"))
                }
                ProviderKind::Custom => {
                    Arc::new(OpenAIProvider::new(config.provider.clone())
                        .expect("Failed to create Custom provider"))
                }
            };
            
            // 初始化 Agent Executor
            let executor = Arc::new(Mutex::new(Some(AgentExecutor::new(provider, tool_registry))));
            
            // 创建应用状态
            let state = AppState {
                executor,
                config: Arc::new(Mutex::new(config)),
                skills_manager,
                task_history: Arc::new(Mutex::new(Vec::new())),
            };
            
            // 注册状态
            app.manage(state);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            execute_task,
            pause_task,
            cancel_task,
            get_task_status,
            get_config,
            save_config,
            list_skills,
            install_skill,
            uninstall_skill,
            get_task_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}