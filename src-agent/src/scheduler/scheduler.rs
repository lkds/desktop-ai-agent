/// 定时任务系统
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration, Timelike};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub schedule: Schedule,
    pub task_definition: String,
    pub enabled: bool,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Schedule {
    Once(DateTime<Utc>),
    Interval { seconds: u64 },
    Daily { hour: u32, minute: u32 },
    Weekly { day: u32, hour: u32, minute: u32 },
    Cron { expression: String },
}

pub struct Scheduler {
    tasks: HashMap<String, ScheduledTask>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { tasks: HashMap::new() }
    }
    
    pub fn add_task(&mut self, task: ScheduledTask) {
        self.tasks.insert(task.id.clone(), task);
    }
    
    pub fn remove_task(&mut self, id: &str) {
        self.tasks.remove(id);
    }
    
    pub fn list_tasks(&self) -> Vec<&ScheduledTask> {
        self.tasks.values().collect()
    }
    
    pub fn get_due_tasks(&self) -> Vec<&ScheduledTask> {
        let now = Utc::now();
        self.tasks.values()
            .filter(|t| t.enabled && t.next_run <= now)
            .collect()
    }
    
    pub fn update_next_run(&mut self, id: &str) {
        if let Some(task) = self.tasks.get_mut(id) {
            task.last_run = Some(Utc::now());
            task.next_run = Self::calculate_next(&task.schedule);
        }
    }
    
    fn calculate_next(schedule: &Schedule) -> DateTime<Utc> {
        let now = Utc::now();
        match schedule {
            Schedule::Once(t) => *t,
            Schedule::Interval { seconds } => now + Duration::seconds(*seconds as i64),
            Schedule::Daily { hour, minute } => {
                let next = now.with_hour(*hour).unwrap()
                    .with_minute(*minute).unwrap();
                if next <= now { next + Duration::days(1) } else { next }
            }
            _ => now + Duration::hours(1),
        }
    }
}