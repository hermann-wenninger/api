use std::sync::Mutex;
pub struct AppState {
    pub name: String,
}

pub struct AppStateCounter {
    pub counter: Mutex<i32>,
}
