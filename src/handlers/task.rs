use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::{CreateTaskRequestBody, Task, UpdateTaskRequestBody},
    state::AppState,
};

pub async fn health() -> &'static str {
    "OK"
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequestBody>,
) -> (StatusCode, Json<Task>) {
    let mut tasks = state.tasks.lock().unwrap();
    let mut next_index = state.next_idx.lock().unwrap();

    let new_task = Task::new(*next_index, payload.title);

    tasks.push(new_task.clone());
    *next_index += 1;

    (StatusCode::CREATED, Json(new_task))
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    let mut tasks = state.tasks.lock().unwrap();
    let next_index = state.next_idx.lock().unwrap();

    tasks.retain(|task| task.id != id);

    if tasks.len() == (*next_index - 1) as usize {
        return StatusCode::NO_CONTENT;
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTaskRequestBody>,
) -> Result<Json<Task>, StatusCode> {
    let mut tasks = state.tasks.lock().unwrap();

    let task = tasks
        .iter_mut()
        .find(|task| task.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        task.title = title;
    }

    if let Some(done) = payload.done {
        task.done = done;
    }

    Ok(Json(task.clone()))
}

pub async fn list(State(state): State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();

    Json(tasks.clone())
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Task>, StatusCode> {
    let tasks = state.tasks.lock().unwrap();
    tasks
        .iter()
        .find(|task| task.id == id)
        .map(|t| Json(t.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}
