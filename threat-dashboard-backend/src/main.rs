mod handlers;
mod models;
mod state;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use handlers::{
    create_task, delete_task, get_task, list_tasks, update_task,
};
use state::SharedState;

#[tokio::main]
async fn main() {
    let state = SharedState::default();

  
let app = Router::new()
    .route("/tasks", post(create_task).get(list_tasks))
    .route("/tasks/:id", get(get_task).patch(update_task).delete(delete_task))
    .with_state(state);


    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}