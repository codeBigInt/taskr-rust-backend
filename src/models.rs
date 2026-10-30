use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Task {
            id,
            title,
            done: false,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTaskRequestBody {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequestBody {
    pub title: Option<String>,
    pub done: Option<bool>,
}
