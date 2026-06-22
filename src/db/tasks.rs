use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::task::{CreateTask, Task, TaskStatus, UpdateTask},
};

#[derive(Clone)]
pub struct TaskRepository {
    pool: PgPool,
}

impl TaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> AppResult<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, created_at, updated_at
            FROM tasks
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    pub async fn get_by_id(&self, id: Uuid) -> AppResult<Task> {
        sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, created_at, updated_at
            FROM tasks
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found")))
    }

    pub async fn create(&self, input: CreateTask) -> AppResult<Task> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let task = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (id, title, description, status, created_at, updated_at)
            VALUES($1, $2, $3, $4, $5, $6)
            RETURNING id, title, description, status, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&input.title)
        .bind(&input.description)
        .bind(TaskStatus::Pending)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    pub async fn update(&self, id: Uuid, input: UpdateTask) -> AppResult<Task> {
        let now = Utc::now();

        let task = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                status = COALESCE($4, status),
                updated_at = $5
            WHERE id = $1
            RETURNING id, title, description, status, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&input.title)
        .bind(&input.description)
        .bind(&input.status)
        .bind(now)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)));

        task
    }

    pub async fn delete(&self, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
