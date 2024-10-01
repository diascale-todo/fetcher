use std::{env, time::Duration};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
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
        .route("/add-task/:name", post(add_task))
        .route("/delete-task/:id", post(delete_task))
        .with_state(pool);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
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

#[derive(Serialize, sqlx::FromRow)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

async fn get_tasks(State(pool): State<PgPool>) -> Json<Vec<Task>> {
    let rows: Vec<Task> = sqlx::query_as(
        r#"
            SELECT id, description, completed FROM todo_schema.todo_list ORDER BY id
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(
        rows.into_iter()
            .map(|task| Task {
                id: task.id,
                description: task.description,
                completed: task.completed,
            })
            .collect(),
    )
}

async fn mark_complete(State(pool): State<PgPool>, Path(id): Path<i32>) -> StatusCode {
    sqlx::query(
        r#"
            UPDATE todo_schema.todo_list SET completed = TRUE WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await
    .unwrap();
    StatusCode::OK
}

async fn mark_incomplete(State(pool): State<PgPool>, Path(id): Path<i32>) -> StatusCode {
    sqlx::query(
        r#"
            UPDATE todo_schema.todo_list SET completed = FALSE WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await
    .unwrap();
    StatusCode::OK
}

async fn add_task(State(pool): State<PgPool>, Path(name): Path<String>) -> StatusCode {
    sqlx::query(
        r#"
            INSERT INTO todo_schema.todo_list(description) VALUES ($1)
        "#,
    )
    .bind(name)
    .execute(&pool)
    .await
    .unwrap();
    StatusCode::OK
}

async fn delete_task(State(pool): State<PgPool>, Path(id): Path<i32>) -> StatusCode {
    sqlx::query!(r#"DELETE FROM todo_schema.todo_list WHERE id = $1"#, id)
        .execute(&pool)
        .await
        .unwrap();
    StatusCode::OK
}
