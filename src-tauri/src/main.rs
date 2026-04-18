#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Tauri 应用入口
//! 集成 Rust Agent 后端

use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;

use qoderwork_agent::{
    AppConfig,
    AgentExecutor,
    ProviderKind,
    ToolRegistry,
    SkillsManager,
    Task,
    Skill,
    providers::{OpenAIProvider, ClaudeProvider, OllamaProvider},
};

/// 应用状态
pub struct AppState {
    pub executor: Arc<Mutex<Option<AgentExecutor>>>,
    pub config: Arc<Mutex<AppConfig>>,
    pub skills_manager: Arc<Mutex<SkillsManager>>,
    pub task_history: Arc<Mutex<Vec<Task>>>,
}

/// 执行任务
#[tauri::command]
async fn execute_task(
    description: String,
    state: State<'_, AppState>,
) -> Result<Task, String> {
    let executor_guard = state.executor.lock().await;
    let executor = executor_guard.as_ref()
        .ok_or_else(|| "Agent executor not initialized".to_string())?;
    
    let task = executor.execute(description).await
        .map_err(|e| e.to_string())?;
    
    let mut history = state.task_history.lock().await;
    history.push(task.clone());
    
    Ok(task)
}

/// 暂停任务
#[tauri::command]
async fn pause_task(state: State<'_, AppState>) -> Result<(), String> {
    let executor_guard = state.executor.lock().await;
    let executor = executor_guard.as_ref()
        .ok_or_else(|| "Agent executor not initialized".to_string())?;
    executor.pause().await.map_err(|e| e.to_string())
}

/// 取消任务
#[tauri::command]
async fn cancel_task(state: State<'_, AppState>) -> Result<(), String> {
    let executor_guard = state.executor.lock().await;
    let executor = executor_guard.as_ref()
        .ok_or_else(|| "Agent executor not initialized".to_string())?;
    executor.cancel().await.map_err(|e| e.to_string())
}

/// 获取任务状态
#[tauri::command]
async fn get_task_status(state: State<'_, AppState>) -> Result<Option<Task>, String> {
    let executor_guard = state.executor.lock().await;
    match executor_guard.as_ref() {
        Some(e) => Ok(e.get_status().await),
        None => Ok(None),
    }
}

/// 获取配置
#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().await;
    Ok(config.clone())
}

/// 保存配置
#[tauri::command]
async fn save_config(
    config: AppConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    config.validate().map_err(|e| e.to_string())?;
    let path = AppConfig::default_path();
    config.save(&path).map_err(|e| e.to_string())?;
    let mut current_config = state.config.lock().await;
    *current_config = config;
    Ok(())
}

/// 列出 Skills
#[tauri::command]
async fn list_skills(state: State<'_, AppState>) -> Result<Vec<Skill>, String> {
    let manager = state.skills_manager.lock().await;
    Ok(manager.list_skills().into_iter().cloned().collect())
}

/// 安装 Skill
#[tauri::command]
async fn install_skill(
    source_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut manager = state.skills_manager.lock().await;
    let path = PathBuf::from(source_path);
    manager.install_skill(&path).await.map_err(|e| e.to_string())
}

/// 卸载 Skill
#[tauri::command]
async fn uninstall_skill(
    skill_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut manager = state.skills_manager.lock().await;
    manager.uninstall_skill(&skill_id).await.map_err(|e| e.to_string())
}

/// 获取任务历史
#[tauri::command]
async fn get_task_history(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let history = state.task_history.lock().await;
    Ok(history.clone())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let config_path = AppConfig::default_path();
            let config = AppConfig::load(&config_path).unwrap_or_default();
            
            let tool_registry = Arc::new(ToolRegistry::new());
            
            let skills_dir = PathBuf::from(&config.skills_dir);
            let skills_manager = Arc::new(Mutex::new(SkillsManager::new(skills_dir)));
            
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
            
            let executor = Arc::new(Mutex::new(Some(AgentExecutor::new(provider, tool_registry))));
            
            let state = AppState {
                executor,
                config: Arc::new(Mutex::new(config)),
                skills_manager,
                task_history: Arc::new(Mutex::new(Vec::new())),
            };
            
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