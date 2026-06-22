use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::handlers::tasks::{
    AppState, create_task, delete_task, get_task, list_tasks, update_task,
};

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_task))
        .route("/tasks/{id}", patch(update_task))
        .route("/tasks/{id}", delete(delete_task))
}
