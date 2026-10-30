use std::sync::{Arc, Mutex};

use crate::models::Task;

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_idx: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            tasks: Arc::new(Mutex::new(vec![
                Task::new(1, "Go shopping".to_string()),
                Task::new(2, "Exercise before shopping".to_string()),
                Task::new(3, "Write a research article for publication".to_string()),
            ])),
            next_idx: Arc::new(Mutex::new(4)),
        }
    }
}
