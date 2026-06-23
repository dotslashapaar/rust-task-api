use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn setup_test_app() -> Router {
    todo!()
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
