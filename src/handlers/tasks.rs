use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    db::tasks::TaskRepository,
    error::{AppError, AppResult},
    models::task::{CreateTask, Task, UpdateTask},
};

#[derive(Clone)]
pub struct AppState {
    pub task_repo: TaskRepository,
}

pub async fn list_tasks(State(state): State<AppState>) -> AppResult<Json<Vec<Task>>> {
    let tasks = state.task_repo.get_all().await?;
    Ok(Json(tasks))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Task>> {
    let task = state.task_repo.get_by_id(id).await?;

    Ok(Json(task))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(input): Json<CreateTask>,
) -> AppResult<(StatusCode, Json<Task>)> {
    if input.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }

    let task = state.task_repo.create(input).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTask>,
) -> AppResult<Json<Task>> {
    if let Some(ref title) = input.title {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }

    let task = state.task_repo.update(id, input).await?;

    Ok(Json(task))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = state.task_repo.delete(id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("Task with id {} not found", id)))
    }
}
