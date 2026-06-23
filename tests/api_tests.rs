use std::time::Duration;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use serde_json::{Value, json};
use task_api::{
    db::{pool::create_pool, tasks::TaskRepository},
    handlers::tasks::AppState,
    routes::tasks::task_routes,
};
use tower::ServiceExt;
use tower_http::trace::TraceLayer;
use tracing::Span;

async fn setup_test_app() -> Router {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tracing::info!("Connecting to database");
    let pool = create_pool(&database_url).await.unwrap();
    tracing::info!("Database connection established");

    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    tracing::info!("Migrations complete");

    let state = AppState {
        task_repo: TaskRepository::new(pool),
    };

    let app = Router::new().merge(task_routes()).with_state(state);

    app
}

#[tokio::test]
async fn test_create_task() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "title": "Test Task",
                        "description": "A test task"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let task: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(task["title"], "Test Task");
    assert_eq!(task["status"], "pending");
}

#[tokio::test]
async fn test_create_task_empty_title() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "title": "",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
