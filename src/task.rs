use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

impl Ord for TaskPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (TaskPriority::High, TaskPriority::High) => Ordering::Equal,
            (TaskPriority::High, _) => Ordering::Greater,
            (TaskPriority::Medium, TaskPriority::High) => Ordering::Less,
            (TaskPriority::Medium, TaskPriority::Medium) => Ordering::Equal,
            (TaskPriority::Medium, TaskPriority::Low) => Ordering::Greater,
            (TaskPriority::Low, TaskPriority::Low) => Ordering::Equal,
            (TaskPriority::Low, _) => Ordering::Less,
        }
    }
}

impl PartialOrd for TaskPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Task {
    pub id: u64,
    pub priority: TaskPriority,
    pub handler: Box<dyn Fn() -> ()>,
}

impl Task {
    pub fn new<F>(id: u64, priority: TaskPriority, handler: F) -> Self
    where
        F: Fn() -> () + 'static,
    {
        Task {
            id,
            priority,
            handler: Box::new(handler),
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.id == other.id
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).then_with(|| self.id.cmp(&other.id))
    }
}
