/// 定时任务测试
use qoderwork_agent::scheduler::{Scheduler, ScheduledTask, Schedule};
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() {
    println!("🧪 定时任务系统测试\n");
    
    let mut scheduler = Scheduler::new();
    
    // 添加定时任务
    let task1 = ScheduledTask {
        id: "daily-report".into(),
        name: "每日报告生成".into(),
        description: "每天早上 9 点生成日报".into(),
        schedule: Schedule::Daily { hour: 9, minute: 0 },
        task_definition: "生成日报".into(),
        enabled: true,
        last_run: None,
        next_run: Utc::now() + Duration::hours(1),
    };
    
    let task2 = ScheduledTask {
        id: "hourly-check".into(),
        name: "每小时检查".into(),
        description: "每小时检查一次状态".into(),
        schedule: Schedule::Interval { seconds: 3600 },
        task_definition: "检查状态".into(),
        enabled: true,
        last_run: None,
        next_run: Utc::now() + Duration::hours(1),
    };
    
    let task3 = ScheduledTask {
        id: "one-time".into(),
        name: "一次性任务".into(),
        description: "执行一次".into(),
        schedule: Schedule::Once(Utc::now() + Duration::seconds(5)),
        task_definition: "测试任务".into(),
        enabled: true,
        last_run: None,
        next_run: Utc::now() + Duration::seconds(5),
    };
    
    scheduler.add_task(task1);
    scheduler.add_task(task2);
    scheduler.add_task(task3);
    
    println!("已添加 {} 个定时任务\n", scheduler.list_tasks().len());
    
    for task in scheduler.list_tasks() {
        println!("  - {} [{}]: 下次执行 {}", 
            task.name, task.id, task.next_run.format("%H:%M:%S"));
    }
    
    // 检查到期任务
    println!("\n等待 5 秒检查到期任务...\n");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
    
    let due_tasks = scheduler.get_due_tasks();
    println!("到期任务数: {}", due_tasks.len());
    
    let task_ids: Vec<String> = due_tasks.iter().map(|t| t.id.clone()).collect();
    
    for task in due_tasks {
        println!("  ⏰ {} - {}", task.name, task.description);
    }
    
    for id in task_ids {
        scheduler.update_next_run(&id);
    }
    
    // 验证任务已更新
    println!("\n任务更新后:");
    for task in scheduler.list_tasks() {
        println!("  - {} last_run: {:?}", 
            task.name, task.last_run.map(|t| t.format("%H:%M:%S").to_string()));
    }
    
    println!("\n🎉 定时任务测试完成");
}