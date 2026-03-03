use crate::models::Task;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct AppState {
    pub tasks: HashMap<Uuid, Task>,
}

pub type SharedState = Arc<Mutex<AppState>>;