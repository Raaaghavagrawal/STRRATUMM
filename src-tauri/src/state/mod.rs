use crate::models::Data;
use std::sync::{Arc, Mutex};

pub type AppState = Arc<Mutex<AppStateInner>>;

pub struct AppStateInner {
    pub data_store: Vec<Data>,
}

pub fn create_state() -> AppState {
    Arc::new(Mutex::new(AppStateInner {
        data_store: Vec::new(),
    }))
}
