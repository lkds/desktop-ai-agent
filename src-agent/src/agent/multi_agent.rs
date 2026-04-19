/// 多智能体并行执行系统
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

use crate::agent::AgentExecutor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentConfig {
    pub id: String,
    pub name: String,
    pub task: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentResult {
    pub id: String,
    pub status: String,
    pub output: String,
}

pub struct MultiAgentCoordinator {
    executor: Arc<AgentExecutor>,
}

impl MultiAgentCoordinator {
    pub fn new(executor: Arc<AgentExecutor>) -> Self {
        Self { executor }
    }
    
    pub async fn run_parallel(&self, configs: Vec<SubagentConfig>) -> HashMap<String, SubagentResult> {
        let mut tasks = Vec::new();
        
        for config in configs {
            let executor = self.executor.clone();
            let id = config.id.clone();
            let task_def = config.task.clone();
            
            tasks.push(tokio::spawn(async move {
                let result = executor.execute(task_def).await;
                match result {
                    Ok(t) => SubagentResult {
                        id,
                        status: format!("{:?}", t.status),
                        output: t.result.map(|r| r.summary).unwrap_or_default(),
                    },
                    Err(e) => SubagentResult {
                        id,
                        status: "error".to_string(),
                        output: e.to_string(),
                    },
                }
            }));
        }
        
        let mut all_results = HashMap::new();
        for task in tasks {
            if let Ok(result) = task.await {
                all_results.insert(result.id.clone(), result);
            }
        }
        
        all_results
    }
    
    pub async fn run_sequential(&self, configs: Vec<SubagentConfig>) -> Vec<SubagentResult> {
        let mut results = Vec::new();
        
        for config in configs {
            let result = self.executor.execute(config.task.clone()).await;
            results.push(match result {
                Ok(t) => SubagentResult {
                    id: config.id,
                    status: format!("{:?}", t.status),
                    output: t.result.map(|r| r.summary).unwrap_or_default(),
                },
                Err(e) => SubagentResult {
                    id: config.id,
                    status: "error".to_string(),
                    output: e.to_string(),
                },
            });
        }
        
        results
    }
}