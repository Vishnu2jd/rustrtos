mod scheduler;
mod task;
mod timing;

use scheduler::Scheduler;
use task::{Task, TaskPriority};
use std::time::Duration;

fn main() {
    let mut scheduler = Scheduler::new();

    // Define tasks
    let task1 = Task::new(1, TaskPriority::High, || {
        println!("Executing high priority task");
        std::thread::sleep(Duration::from_millis(100));
    });

    let task2 = Task::new(2, TaskPriority::Medium, || {
        println!("Executing medium priority task");
        std::thread::sleep(Duration::from_millis(200));
    });

    let task3 = Task::new(3, TaskPriority::Low, || {
        println!("Executing low priority task");
        std::thread::sleep(Duration::from_millis(300));
    });

    // Add tasks to the scheduler
    scheduler.add_task(task1);
    scheduler.add_task(task2);
    scheduler.add_task(task3);

    // Run the scheduler
    scheduler.run();
}
