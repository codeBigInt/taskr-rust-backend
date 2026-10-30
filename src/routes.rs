use crate::{
    handlers::task::{create, delete, get_one, health, list, update},
    state::AppState,
};
use axum::routing::{Router, get};

pub fn app(state: AppState) -> Router {
    return Router::new()
        .route("/health", get(health))
        .route("/tasks", get(list).post(create))
        .route("/tasks/{id}", get(get_one).delete(delete).patch(update))
        .with_state(state);
}
