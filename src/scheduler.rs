use crate::task::Task;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Scheduler {
    tasks: BinaryHeap<Reverse<Task>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: BinaryHeap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(Reverse(task));
    }

    pub fn run(&mut self) {
        while let Some(Reverse(task)) = self.tasks.pop() {
            println!("Running task {}", task.id);
            (task.handler)();
        }
    }
}
