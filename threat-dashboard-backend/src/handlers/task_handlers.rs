use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::state::SharedState;
use uuid::Uuid;

pub async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    let id = Uuid::new_v4();
    let task = Task {
        id,
        title: payload.title,
        completed: false,
    };
    state.tasks.insert(id, task.clone());
    (StatusCode::CREATED, Json(task))
}

pub async fn list_tasks(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.lock().unwrap();
    let tasks: Vec<Task> = state.tasks.values().cloned().collect();
    Json(tasks)
}

pub async fn get_task(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, StatusCode> {
    let state = state.lock().unwrap();
    if let Some(task) = state.tasks.get(&id) {
        Ok(Json(task.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn update_task(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, StatusCode> {
    let mut state = state.lock().unwrap();
    if let Some(task) = state.tasks.get_mut(&id) {
        if let Some(title) = payload.title {
            task.title = title;
        }
        if let Some(completed) = payload.completed {
            task.completed = completed;
        }
        Ok(Json(task.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_task(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut state = state.lock().unwrap();
    if state.tasks.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}