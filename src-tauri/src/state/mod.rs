use std::sync::{Arc, Mutex};
use crate::models::Data;

pub type AppState = Arc<Mutex<AppStateInner>>;

pub struct AppStateInner {
    pub data_store: Vec<Data>,
}

pub fn create_state() -> AppState {
    Arc::new(Mutex::new(AppStateInner {
        data_store: Vec::new(),
    }))
}
