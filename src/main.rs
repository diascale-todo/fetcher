use std::{env, time::Duration};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    serve, Router,
};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/get-tasks", get(get_tasks))
        .route("/mark-complete/:id", get(mark_complete))
        .route("/mark-incomplete/:id", get(mark_incomplete))
        .with_state(pool);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Running on http://localhost:3000");
    serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Serialize)]
struct Ping {
    msg: String,
}

async fn ping() -> Json<Ping> {
    println!("hello");
    Json(Ping {
        msg: "Hello there!".to_string(),
    })
}

#[derive(Serialize)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

async fn get_tasks(State(pool): State<PgPool>) -> Json<Vec<Task>> {
    let rows = sqlx::query!(
        r#"
            SELECT id, description, completed FROM todo_schema.todo_list ORDER BY id
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(
        rows.into_iter()
            .map(|task| Task {
                id: task.id,
                description: task.description,
                completed: task.completed.unwrap(),
            })
            .collect(),
    )
}

async fn mark_complete(State(pool): State<PgPool>, Path(id): Path<i32>) -> StatusCode {
    sqlx::query!(
        r#"
            UPDATE todo_schema.todo_list SET completed = TRUE WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await
    .unwrap();
    StatusCode::OK
}

async fn mark_incomplete(State(pool): State<PgPool>, Path(id): Path<i32>) -> StatusCode {
    sqlx::query!(
        r#"
            UPDATE todo_schema.todo_list SET completed = FALSE WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await
    .unwrap();
    StatusCode::OK
}
