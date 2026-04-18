/// Tauri IPC handlers
/// 前端调用后端的接口

use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;

use crate::agent::{AgentExecutor, Task};
use crate::config::AppConfig;
use crate::skills::{SkillsManager, Skill};

/// 应用状态
pub struct AppState {
    pub executor: Arc<Mutex<Option<AgentExecutor>>>,
    pub config: Arc<Mutex<AppConfig>>,
    pub skills_manager: Arc<Mutex<SkillsManager>>,
    pub task_history: Arc<Mutex<Vec<Task>>>,
}

/// 执行任务
#[tauri::command]
pub async fn execute_task(
    description: String,
    state: State<'_, AppState>,
) -> Result<Task, String> {
    let executor_guard = state.executor.lock().await;
    let executor = executor_guard.as_ref()
        .ok_or_else(|| "Agent executor not initialized".to_string())?;
    
    let task = executor.execute(description).await
        .map_err(|e| e.to_string())?;
    
    // 保存到历史记录
    let mut history = state.task_history.lock().await;
    history.push(task.clone());
    
    Ok(task)
}

/// 暂停任务
#[tauri::command]
pub async fn pause_task(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let executor_guard = state.executor.lock().await;
    let executor = executor_guard.as_ref()
        .ok_or_else(|| "Agent executor not initialized".to_string())?;
    
    executor.pause().await.map_err(|e| e.to_string())
}

/// 取消任务
#[tauri::command]
pub async fn cancel_task(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let executor_guard = state.executor.lock().await;
    let executor = executor_guard.as_ref()
        .ok_or_else(|| "Agent executor not initialized".to_string())?;
    
    executor.cancel().await.map_err(|e| e.to_string())
}

/// 获取当前任务状态
#[tauri::command]
pub async fn get_task_status(
    state: State<'_, AppState>,
) -> Result<Option<Task>, String> {
    let executor_guard = state.executor.lock().await;
    let executor = executor_guard.as_ref();
    
    match executor {
        Some(e) => Ok(e.get_status().await),
        None => Ok(None),
    }
}

/// 获取配置
#[tauri::command]
pub async fn get_config(
    state: State<'_, AppState>,
) -> Result<AppConfig, String> {
    let config = state.config.lock().await;
    Ok(config.clone())
}

/// 保存配置
#[tauri::command]
pub async fn save_config(
    config: AppConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 验证配置
    config.validate().map_err(|e| e.to_string())?;
    
    // 保存到文件
    let path = AppConfig::default_path();
    config.save(&path).map_err(|e| e.to_string())?;
    
    // 更新状态
    let mut current_config = state.config.lock().await;
    *current_config = config;
    
    Ok(())
}

/// 列出所有 Skills
#[tauri::command]
pub async fn list_skills(
    state: State<'_, AppState>,
) -> Result<Vec<Skill>, String> {
    let manager = state.skills_manager.lock().await;
    Ok(manager.list_skills().into_iter().cloned().collect())
}

/// 安装 Skill
#[tauri::command]
pub async fn install_skill(
    skill_id: String,
    source_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut manager = state.skills_manager.lock().await;
    let path = PathBuf::from(source_path);
    manager.install_skill(&path).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// 卸载 Skill
#[tauri::command]
pub async fn uninstall_skill(
    skill_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut manager = state.skills_manager.lock().await;
    manager.uninstall_skill(&skill_id).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取任务历史
#[tauri::command]
pub async fn get_task_history(
    state: State<'_, AppState>,
) -> Result<Vec<Task>, String> {
    let history = state.task_history.lock().await;
    Ok(history.clone())
}